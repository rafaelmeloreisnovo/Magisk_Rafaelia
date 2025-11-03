package com.topjohnwu.magisk.core.error

import java.time.Instant

/**
 * Comprehensive error context for detailed bug tracking and diagnostics.
 * Captures all relevant information about an error occurrence.
 */
data class ErrorContext(
    val timestamp: Instant = Instant.now(),
    val category: ErrorCategory,
    val throwable: Throwable,
    val component: String,
    val operation: String,
    val metadata: Map<String, Any?> = emptyMap(),
    val severity: ErrorSeverity = ErrorSeverity.MEDIUM,
    val recoverable: Boolean = true,
    val userMessage: String? = null
) {
    fun toMap(): Map<String, Any?> {
        return mapOf(
            "timestamp" to timestamp.toString(),
            "category" to category.name,
            "errorType" to throwable.javaClass.simpleName,
            "errorMessage" to throwable.message,
            "component" to component,
            "operation" to operation,
            "severity" to severity.name,
            "recoverable" to recoverable,
            "userMessage" to userMessage,
            "metadata" to metadata
        )
    }
}

enum class ErrorSeverity {
    LOW,      // Minor issues, system can continue normally
    MEDIUM,   // Moderate issues, some functionality affected
    HIGH,     // Severe issues, major functionality affected
    CRITICAL  // Critical issues, system stability at risk
}
