package com.topjohnwu.magisk.tests

import com.topjohnwu.magisk.core.logging.JSONLogger
import com.topjohnwu.magisk.core.error.ErrorCategory
import com.topjohnwu.magisk.core.error.ErrorContext
import com.topjohnwu.magisk.core.error.ErrorSeverity
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking

object TestWrapper {
    /**
     * Run an action with retries and comprehensive error handling.
     * Enhanced with detailed error categorization and recovery mechanisms.
     */
    fun runWithRetries(
        maxAttempts: Int = 3,
        delayMs: Long = 2000,
        exponentialBackoff: Boolean = true,
        actionName: String = "testAction",
        action: suspend () -> Boolean
    ) {
        runBlocking {
            var attempt = 1
            var currentDelay = delayMs
            val errors = mutableListOf<ErrorContext>()
            
            while (attempt <= maxAttempts) {
                JSONLogger.info(
                    "TestWrapper",
                    "attempt_start",
                    null,
                    mapOf(
                        "attempt" to attempt,
                        "maxAttempts" to maxAttempts,
                        "action" to actionName,
                        "delay" to currentDelay
                    )
                )
                
                val ok = try {
                    action()
                } catch (t: Throwable) {
                    val errorContext = ErrorContext(
                        category = ErrorCategory.fromThrowable(t),
                        throwable = t,
                        component = "TestWrapper",
                        operation = actionName,
                        metadata = mapOf(
                            "attempt" to attempt,
                            "maxAttempts" to maxAttempts
                        ),
                        severity = if (attempt < maxAttempts) ErrorSeverity.MEDIUM else ErrorSeverity.HIGH,
                        recoverable = attempt < maxAttempts
                    )
                    errors.add(errorContext)
                    
                    JSONLogger.error(
                        "TestWrapper",
                        "exception",
                        null,
                        errorContext.toMap(),
                        t
                    )
                    false
                }
                
                if (ok) {
                    JSONLogger.info(
                        "TestWrapper",
                        "attempt_success",
                        null,
                        mapOf(
                            "attempt" to attempt,
                            "action" to actionName,
                            "totalErrors" to errors.size
                        )
                    )
                    return@runBlocking
                }
                
                JSONLogger.warn(
                    "TestWrapper",
                    "attempt_failed",
                    null,
                    mapOf(
                        "attempt" to attempt,
                        "action" to actionName,
                        "remainingAttempts" to (maxAttempts - attempt)
                    )
                )
                
                attempt++
                if (attempt <= maxAttempts) {
                    delay(currentDelay)
                    if (exponentialBackoff) {
                        currentDelay *= 2
                    }
                }
            }
            
            // All attempts failed - log comprehensive error report
            val errorReport = mapOf(
                "action" to actionName,
                "totalAttempts" to maxAttempts,
                "errors" to errors.map { it.toMap() },
                "errorCategories" to errors.groupingBy { it.category }.eachCount(),
                "errorTypes" to errors.map { it.throwable.javaClass.simpleName }.distinct()
            )
            
            JSONLogger.fatal(
                "TestWrapper",
                "all_attempts_failed",
                null,
                errorReport
            )
        }
    }
    
    /**
     * Run an action once with comprehensive error handling.
     */
    fun <T> runSafe(
        actionName: String = "testAction",
        defaultValue: T? = null,
        action: () -> T
    ): T? {
        return try {
            JSONLogger.debug("TestWrapper", "action_start", extra = mapOf("action" to actionName))
            val result = action()
            JSONLogger.debug("TestWrapper", "action_success", extra = mapOf("action" to actionName))
            result
        } catch (t: Throwable) {
            val errorContext = ErrorContext(
                category = ErrorCategory.fromThrowable(t),
                throwable = t,
                component = "TestWrapper",
                operation = actionName,
                severity = if (defaultValue != null) ErrorSeverity.MEDIUM else ErrorSeverity.HIGH,
                recoverable = defaultValue != null
            )
            
            JSONLogger.error(
                "TestWrapper",
                "action_failed",
                null,
                errorContext.toMap(),
                t
            )
            
            defaultValue
        }
    }
}
