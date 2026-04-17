package com.topjohnwu.magisk.ui

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ProgressBar
import android.widget.TextView
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.lifecycle.lifecycleScope
import com.topjohnwu.magisk.core.BackupManager
import com.topjohnwu.magisk.core.logging.JSONLogger
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileOutputStream

class PatchFlowFragment : Fragment() {

    private val REQUEST_CODE_PICK = 1234
    private lateinit var pickButton: Button
    private lateinit var statusText: TextView
    private lateinit var progressBar: ProgressBar

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?): View {
        val context = requireContext()
        val root = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 32, 32, 32)
        }

        pickButton = Button(context).apply { text = "Select boot image" }
        statusText = TextView(context)
        progressBar = ProgressBar(context, null, android.R.attr.progressBarStyleHorizontal).apply {
            visibility = View.GONE
        }

        root.addView(pickButton)
        root.addView(statusText)
        root.addView(progressBar)
        pickButton.setOnClickListener { openFilePicker() }
        return root
    }

    private fun openFilePicker() {
        JSONLogger.info("PatchFlowFragment", "open_picker", null, emptyMap())
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "*/*"
        }
        startActivityForResult(intent, REQUEST_CODE_PICK)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == REQUEST_CODE_PICK && resultCode == Activity.RESULT_OK) {
            val uri: Uri? = data?.data
            if (uri != null) {
                lifecycleScope.launch { handleUri(uri) }
            } else {
                context?.let { Toast.makeText(it, "No file selected", Toast.LENGTH_SHORT).show() }
            }
        }
    }

    private suspend fun handleUri(uri: Uri) {
        val fragmentContext = context ?: return
        val sessionId = generateSessionId()
        showProgress(true, "Preparing backup...")

        val ok = withContext(Dispatchers.IO) {
            try {
                val tmp = File(fragmentContext.cacheDir, "rafaelia-$sessionId.img")
                fragmentContext.contentResolver.openInputStream(uri)?.use { ins ->
                    FileOutputStream(tmp).use { out ->
                        val buf = ByteArray(32 * 1024)
                        var read: Int
                        while (ins.read(buf).also { read = it } != -1) out.write(buf, 0, read)
                        out.fd.sync()
                    }
                } ?: return@withContext false

                val bm = BackupManager.getInstance(fragmentContext)
                val manifestPath = bm.backupBootImage(source = tmp, sessionIdIn = sessionId)
                tmp.delete()
                manifestPath != null
            } catch (t: Throwable) {
                JSONLogger.error("PatchFlowFragment", "backup_unexpected", sessionId, mapOf("error" to t.toString()))
                false
            }
        }

        if (!isAdded) return
        showProgress(false)
        val currentContext = context ?: return
        if (ok) {
            Toast.makeText(currentContext, "Backup finished", Toast.LENGTH_SHORT).show()
            onBackupSuccess(sessionId)
        } else {
            Toast.makeText(currentContext, "Backup failed — check logs", Toast.LENGTH_LONG).show()
            onBackupFailure(sessionId)
        }
    }

    private fun showProgress(show: Boolean, message: String? = null) {
        progressBar.visibility = if (show) View.VISIBLE else View.GONE
        statusText.text = message ?: ""
    }

    private fun generateSessionId(): String {
        val sb = StringBuilder()
        repeat(12) { sb.append("%02x".format((0..255).random())) }
        return sb.toString()
    }

    protected open fun onBackupSuccess(sessionId: String) {}
    protected open fun onBackupFailure(sessionId: String) {}
}
