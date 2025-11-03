package com.topjohnwu.magisk.core.logging

import org.json.JSONObject
import java.time.Instant

object JSONLogger {
    private fun emit(obj: JSONObject) {
        // In Android use Log.i/w/e; here we print so CI/adb logcat captures it.
        println(obj.toString())
    }

    private fun log(level: String, component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap(), throwable: Throwable? = null) {
        val obj = JSONObject()
        obj.put("ts", Instant.now().toString())
        obj.put("level", level)
        obj.put("component", component)
        obj.put("event", event)
        sessionId?.let { obj.put("sessionId", it) }
        obj.put("extra", JSONObject(extra))
        throwable?.let {
            val errorDetails = JSONObject()
            errorDetails.put("message", it.message ?: "No message")
            errorDetails.put("type", it.javaClass.simpleName)
            errorDetails.put("stackTrace", it.stackTraceToString())
            obj.put("error", errorDetails)
        }
        emit(obj)
    }

    fun trace(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap()) {
        log("TRACE", component, event, sessionId, extra)
    }

    fun debug(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap()) {
        log("DEBUG", component, event, sessionId, extra)
    }

    fun info(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap()) {
        log("INFO", component, event, sessionId, extra)
    }

    fun warn(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap(), throwable: Throwable? = null) {
        log("WARN", component, event, sessionId, extra, throwable)
    }

    fun error(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap(), throwable: Throwable? = null) {
        log("ERROR", component, event, sessionId, extra, throwable)
    }

    fun fatal(component: String, event: String, sessionId: String? = null, extra: Map<String, Any?> = emptyMap(), throwable: Throwable? = null) {
        log("FATAL", component, event, sessionId, extra, throwable)
    }
}
