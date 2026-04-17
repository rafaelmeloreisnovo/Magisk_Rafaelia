package com.topjohnwu.magisk.core

import android.content.Context
import android.util.Base64
import com.topjohnwu.magisk.core.logging.JSONLogger
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import java.io.InputStream
import java.nio.charset.StandardCharsets
import java.security.MessageDigest
import java.security.SecureRandom
import java.time.Instant
import javax.crypto.Mac
import javax.crypto.SecretKey
import javax.crypto.spec.SecretKeySpec

class BackupManager private constructor(private val ctx: Context) {

    companion object {
        private var instance: BackupManager? = null
        fun getInstance(context: Context): BackupManager {
            if (instance == null) instance = BackupManager(context.applicationContext)
            return instance!!
        }

        private const val MANIFEST_DIR = "rafaelia_backups"
        private const val MANIFEST_EXT = "-manifest.json"
        private const val BACKUP_EXT = ".img"
        private const val FALLBACK_KEY_FILE = ".fallback_hmac.key"
    }

    private fun getBackupDir(): File = File(ctx.filesDir, MANIFEST_DIR).apply { mkdirs() }

    private fun generateSessionId(): String {
        val b = ByteArray(12)
        SecureRandom().nextBytes(b)
        return b.joinToString("") { "%02x".format(it) }
    }

    private fun hexStringToByteArray(s: String): ByteArray {
        val data = ByteArray(s.length / 2)
        var i = 0
        while (i < s.length) {
            data[i / 2] = ((Character.digit(s[i], 16) shl 4) + Character.digit(s[i + 1], 16)).toByte()
            i += 2
        }
        return data
    }

    private fun getPersistedFallbackKey(): SecretKey {
        val keyFile = File(getBackupDir(), FALLBACK_KEY_FILE)
        if (keyFile.exists()) {
            val raw = Base64.decode(keyFile.readText(StandardCharsets.UTF_8), Base64.NO_WRAP)
            return SecretKeySpec(raw, "HmacSHA256")
        }
        val seed = ByteArray(32)
        SecureRandom().nextBytes(seed)
        keyFile.writeText(Base64.encodeToString(seed, Base64.NO_WRAP), StandardCharsets.UTF_8)
        return SecretKeySpec(seed, "HmacSHA256")
    }

    private fun resolveSigningKey(sessionId: String): Pair<SecretKey, String> {
        val ks = try {
            KeyStoreWrapper.getHmacKey(ctx)
        } catch (_: Exception) {
            null
        }
        if (ks != null) return ks to "keystore"
        JSONLogger.warn("BackupManager", "keystore_unavailable_fallback", sessionId, emptyMap())
        return getPersistedFallbackKey() to "persisted_fallback"
    }

    private fun hmacB64(key: SecretKey, shaHex: String): String {
        val mac = Mac.getInstance("HmacSHA256")
        mac.init(key)
        val computed = mac.doFinal(hexStringToByteArray(shaHex))
        return Base64.encodeToString(computed, Base64.NO_WRAP)
    }

    suspend fun backupBootImage(source: File, sessionIdIn: String? = null): String? = withContext(Dispatchers.IO) {
        if (!source.exists()) return@withContext null
        source.inputStream().use { backupFromStream(it, source.name, sessionIdIn) }
    }

    suspend fun backupFromStream(stream: InputStream, originalName: String, sessionIdIn: String? = null): String? = withContext(Dispatchers.IO) {
        val sessionId = sessionIdIn ?: generateSessionId()
        val dstDir = getBackupDir()
        val tmpFile = File(dstDir, "$sessionId${BACKUP_EXT}.tmp")
        val finalBackup = File(dstDir, "$sessionId$BACKUP_EXT")
        val manifestFile = File(dstDir, "$sessionId$MANIFEST_EXT")

        try {
            val (shaHex, size) = stream.use { ins ->
                FileOutputStream(tmpFile).use { out ->
                    val md = MessageDigest.getInstance("SHA-256")
                    val buf = ByteArray(32 * 1024)
                    var processed = 0L
                    var read: Int
                    while (ins.read(buf).also { read = it } != -1) {
                        out.write(buf, 0, read)
                        md.update(buf, 0, read)
                        processed += read
                    }
                    out.fd.sync()
                    md.digest().joinToString("") { "%02x".format(it) } to processed
                }
            }

            val (key, keySource) = resolveSigningKey(sessionId)
            val hmac = hmacB64(key, shaHex)

            val manifest = JSONObject().apply {
                put("sessionId", sessionId)
                put("originalName", originalName)
                put("backupPath", finalBackup.absolutePath)
                put("sha256", shaHex)
                put("hmac_b64", hmac)
                put("key_source", keySource)
                put("size", size)
                put("timestamp", Instant.now().toString())
            }

            if (!tmpFile.renameTo(finalBackup)) return@withContext null
            val tmpManifest = File(dstDir, "$sessionId${MANIFEST_EXT}.tmp")
            tmpManifest.writeText(manifest.toString(), StandardCharsets.UTF_8)
            if (!tmpManifest.renameTo(manifestFile)) return@withContext null

            JSONLogger.info("BackupManager", "backup_complete", sessionId, mapOf("manifest" to manifestFile.absolutePath))
            return@withContext manifestFile.absolutePath
        } catch (t: Throwable) {
            JSONLogger.error("BackupManager", "backup_exception", sessionId, mapOf("error" to t.toString()))
            tmpFile.delete()
            return@withContext null
        }
    }

    fun loadManifestFile(sessionId: String): File? {
        val f = File(getBackupDir(), "$sessionId$MANIFEST_EXT")
        return if (f.exists()) f else null
    }

    suspend fun validateBackup(sessionId: String, keyProvider: (() -> SecretKey?)? = null): Boolean = withContext(Dispatchers.IO) {
        val manifestFile = loadManifestFile(sessionId) ?: return@withContext false
        val json = JSONObject(manifestFile.readText(StandardCharsets.UTF_8))
        val backupPath = json.optString("backupPath", "")
        val expectedSha = json.optString("sha256", "")
        val expectedHmacB64 = json.optString("hmac_b64", "")
        if (backupPath.isEmpty() || expectedSha.isEmpty() || expectedHmacB64.isEmpty()) return@withContext false

        val backupFile = File(backupPath)
        if (!backupFile.exists()) return@withContext false

        val actualSha = backupFile.inputStream().use { ins ->
            val md = MessageDigest.getInstance("SHA-256")
            val buf = ByteArray(32 * 1024)
            var read: Int
            while (ins.read(buf).also { read = it } != -1) md.update(buf, 0, read)
            md.digest().joinToString("") { "%02x".format(it) }
        }
        if (!actualSha.equals(expectedSha, ignoreCase = true)) return@withContext false

        val key = try {
            keyProvider?.invoke() ?: KeyStoreWrapper.getHmacKey(ctx)
        } catch (_: Exception) {
            getPersistedFallbackKey()
        }

        return@withContext hmacB64(key, actualSha) == expectedHmacB64
    }

    fun exportManifestForExternalUse(sessionId: String): File? = loadManifestFile(sessionId)
}
