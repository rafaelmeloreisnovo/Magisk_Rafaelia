package com.topjohnwu.magisk.core.error

import com.topjohnwu.magisk.core.logging.JSONLogger
import kotlinx.coroutines.delay
import java.util.concurrent.ConcurrentLinkedQueue

/**
 * Enterprise-level error handling utility with comprehensive bug treatment.
 * Provides robust error handling, recovery mechanisms, and detailed logging.
 */
object ErrorHandlerUtil {
    private val errorHistory = ConcurrentLinkedQueue<ErrorContext>()
    private const val MAX_HISTORY_SIZE = 1000

    /**
     * Execute an operation with comprehensive error handling and optional retries.
     */
    suspend fun <T> executeWithRetry(
        component: String,
        operation: String,
        maxAttempts: Int = 3,
        delayMs: Long = 1000,
        exponentialBackoff: Boolean = true,
        onError: ((ErrorContext) -> Unit)? = null,
        action: suspend (attempt: Int) -> T
    ): Result<T> {
        var currentDelay = delayMs
        repeat(maxAttempts) { attempt ->
            try {
                JSONLogger.debug(
                    component, 
                    "operation_attempt",
                    extra = mapOf(
                        "operation" to operation,
                        "attempt" to (attempt + 1),
                        "maxAttempts" to maxAttempts
                    )
                )
                
                val result = action(attempt + 1)
                
                JSONLogger.info(
                    component,
                    "operation_success",
                    extra = mapOf(
                        "operation" to operation,
                        "attempt" to (attempt + 1)
                    )
                )
                
                return Result.success(result)
            } catch (e: Throwable) {
                val context = ErrorContext(
                    category = ErrorCategory.fromThrowable(e),
                    throwable = e,
                    component = component,
                    operation = operation,
                    metadata = mapOf(
                        "attempt" to (attempt + 1),
                        "maxAttempts" to maxAttempts
                    ),
                    recoverable = attempt < maxAttempts - 1
                )
                
                recordError(context)
                onError?.invoke(context)
                
                JSONLogger.warn(
                    component,
                    "operation_failed",
                    extra = context.toMap(),
                    throwable = e
                )
                
                if (attempt < maxAttempts - 1) {
                    delay(currentDelay)
                    if (exponentialBackoff) {
                        currentDelay *= 2
                    }
                } else {
                    JSONLogger.error(
                        component,
                        "operation_exhausted",
                        extra = context.toMap(),
                        throwable = e
                    )
                    return Result.failure(e)
                }
            }
        }
        
        return Result.failure(RuntimeException("Unexpected execution path"))
    }

    /**
     * Execute an operation with error handling but no retries.
     */
    inline fun <T> executeSafe(
        component: String,
        operation: String,
        severity: ErrorSeverity = ErrorSeverity.MEDIUM,
        defaultValue: T? = null,
        onError: ((ErrorContext) -> Unit)? = null,
        action: () -> T
    ): T? {
        try {
            JSONLogger.trace(component, "operation_start", extra = mapOf("operation" to operation))
            val result = action()
            JSONLogger.trace(component, "operation_complete", extra = mapOf("operation" to operation))
            return result
        } catch (e: Throwable) {
            val context = ErrorContext(
                category = ErrorCategory.fromThrowable(e),
                throwable = e,
                component = component,
                operation = operation,
                severity = severity,
                recoverable = defaultValue != null
            )
            
            recordError(context)
            onError?.invoke(context)
            
            JSONLogger.error(
                component,
                "operation_error",
                extra = context.toMap(),
                throwable = e
            )
            
            return defaultValue
        }
    }

    /**
     * Validate data with comprehensive error handling.
     */
    fun <T> validate(
        value: T?,
        component: String,
        fieldName: String,
        validator: (T) -> Boolean,
        errorMessage: String? = null
    ): Result<T> {
        return try {
            when {
                value == null -> {
                    val error = IllegalArgumentException("$fieldName cannot be null")
                    val context = ErrorContext(
                        category = ErrorCategory.VALIDATION,
                        throwable = error,
                        component = component,
                        operation = "validation",
                        metadata = mapOf("field" to fieldName, "reason" to "null_value"),
                        severity = ErrorSeverity.MEDIUM,
                        userMessage = errorMessage ?: "$fieldName is required"
                    )
                    recordError(context)
                    JSONLogger.warn(component, "validation_failed", extra = context.toMap())
                    Result.failure(error)
                }
                !validator(value) -> {
                    val error = IllegalArgumentException("$fieldName validation failed")
                    val context = ErrorContext(
                        category = ErrorCategory.VALIDATION,
                        throwable = error,
                        component = component,
                        operation = "validation",
                        metadata = mapOf("field" to fieldName, "reason" to "validator_failed"),
                        severity = ErrorSeverity.MEDIUM,
                        userMessage = errorMessage ?: "$fieldName is invalid"
                    )
                    recordError(context)
                    JSONLogger.warn(component, "validation_failed", extra = context.toMap())
                    Result.failure(error)
                }
                else -> {
                    JSONLogger.trace(component, "validation_passed", extra = mapOf("field" to fieldName))
                    Result.success(value)
                }
            }
        } catch (e: Throwable) {
            val context = ErrorContext(
                category = ErrorCategory.RUNTIME,
                throwable = e,
                component = component,
                operation = "validation",
                metadata = mapOf("field" to fieldName),
                severity = ErrorSeverity.HIGH
            )
            recordError(context)
            JSONLogger.error(component, "validation_error", extra = context.toMap(), throwable = e)
            Result.failure(e)
        }
    }

    /**
     * Record error for analytics and tracking.
     */
    private fun recordError(context: ErrorContext) {
        errorHistory.add(context)
        while (errorHistory.size > MAX_HISTORY_SIZE) {
            errorHistory.poll()
        }
    }

    /**
     * Get error statistics for monitoring (type-safe).
     */
    fun getErrorStats(): ErrorStatistics {
        val errors = errorHistory.toList()
        val categoryCounts = errors.groupingBy { it.category }.eachCount()
        val severityCounts = errors.groupingBy { it.severity }.eachCount()
        val componentCounts = errors.groupingBy { it.component }.eachCount()
        
        return ErrorStatistics(
            totalErrors = errors.size,
            byCategory = categoryCounts,
            bySeverity = severityCounts,
            byComponent = componentCounts,
            recentErrors = errors.takeLast(10).map { it.toMap() }
        )
    }
    
    /**
     * Get error statistics as map (for backward compatibility).
     */
    fun getErrorStatsMap(): Map<String, Any> {
        val stats = getErrorStats()
        return mapOf(
            "totalErrors" to stats.totalErrors,
            "byCategory" to stats.byCategory,
            "bySeverity" to stats.bySeverity,
            "byComponent" to stats.byComponent,
            "recentErrors" to stats.recentErrors
        )
    }

    /**
     * Clear error history.
     */
    fun clearHistory() {
        errorHistory.clear()
        JSONLogger.info("ErrorHandlerUtil", "history_cleared")
    }
}
