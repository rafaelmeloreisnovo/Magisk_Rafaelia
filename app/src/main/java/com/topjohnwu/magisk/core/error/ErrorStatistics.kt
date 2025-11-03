package com.topjohnwu.magisk.core.error

/**
 * Type-safe error statistics data class.
 */
data class ErrorStatistics(
    val totalErrors: Int,
    val byCategory: Map<ErrorCategory, Int>,
    val bySeverity: Map<ErrorSeverity, Int>,
    val byComponent: Map<String, Int>,
    val recentErrors: List<Map<String, Any?>>
)
