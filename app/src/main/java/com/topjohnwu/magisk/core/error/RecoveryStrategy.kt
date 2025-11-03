package com.topjohnwu.magisk.core.error

import com.topjohnwu.magisk.core.logging.JSONLogger
import kotlinx.coroutines.delay

/**
 * Comprehensive error recovery strategies for different error scenarios.
 * Provides intelligent recovery mechanisms based on error type and context.
 */
object RecoveryStrategy {
    
    /**
     * Attempt to recover from an error using appropriate strategy.
     */
    suspend fun <T> recover(
        context: ErrorContext,
        fallbackValue: T? = null,
        retryAction: (suspend () -> T)? = null
    ): T? {
        JSONLogger.info(
            "RecoveryStrategy",
            "attempting_recovery",
            extra = mapOf(
                "category" to context.category.name,
                "component" to context.component,
                "operation" to context.operation
            )
        )
        
        return when (context.category) {
            ErrorCategory.NETWORK -> recoverFromNetworkError(context, fallbackValue, retryAction)
            ErrorCategory.IO -> recoverFromIOError(context, fallbackValue, retryAction)
            ErrorCategory.VALIDATION -> recoverFromValidationError(context, fallbackValue)
            ErrorCategory.PARSING -> recoverFromParsingError(context, fallbackValue)
            ErrorCategory.DATABASE -> recoverFromDatabaseError(context, fallbackValue, retryAction)
            else -> recoverGeneric(context, fallbackValue, retryAction)
        }
    }
    
    /**
     * Recover from network errors with exponential backoff retry.
     */
    private suspend fun <T> recoverFromNetworkError(
        context: ErrorContext,
        fallbackValue: T?,
        retryAction: (suspend () -> T)?
    ): T? {
        if (retryAction == null) {
            JSONLogger.warn(
                "RecoveryStrategy",
                "network_recovery_no_retry",
                extra = context.toMap()
            )
            return fallbackValue
        }
        
        val maxRetries = 3
        var delay = 1000L
        
        repeat(maxRetries) { attempt ->
            try {
                JSONLogger.debug(
                    "RecoveryStrategy",
                    "network_retry_attempt",
                    extra = mapOf(
                        "attempt" to (attempt + 1),
                        "delay" to delay
                    )
                )
                
                delay(delay)
                val result = retryAction()
                
                JSONLogger.info(
                    "RecoveryStrategy",
                    "network_recovery_success",
                    extra = mapOf("attempt" to (attempt + 1))
                )
                
                return result
            } catch (e: Throwable) {
                delay *= 2
                JSONLogger.warn(
                    "RecoveryStrategy",
                    "network_retry_failed",
                    extra = mapOf(
                        "attempt" to (attempt + 1),
                        "error" to e.message
                    ),
                    throwable = e
                )
            }
        }
        
        JSONLogger.error(
            "RecoveryStrategy",
            "network_recovery_exhausted",
            extra = context.toMap()
        )
        
        return fallbackValue
    }
    
    /**
     * Recover from I/O errors.
     */
    private suspend fun <T> recoverFromIOError(
        context: ErrorContext,
        fallbackValue: T?,
        retryAction: (suspend () -> T)?
    ): T? {
        if (retryAction != null) {
            try {
                delay(500)
                val result = retryAction()
                
                JSONLogger.info(
                    "RecoveryStrategy",
                    "io_recovery_success",
                    extra = context.toMap()
                )
                
                return result
            } catch (e: Throwable) {
                JSONLogger.warn(
                    "RecoveryStrategy",
                    "io_recovery_failed",
                    extra = context.toMap(),
                    throwable = e
                )
            }
        }
        
        return fallbackValue
    }
    
    /**
     * Recover from validation errors (usually not retryable).
     */
    private fun <T> recoverFromValidationError(
        context: ErrorContext,
        fallbackValue: T?
    ): T? {
        JSONLogger.warn(
            "RecoveryStrategy",
            "validation_recovery_fallback",
            extra = context.toMap()
        )
        return fallbackValue
    }
    
    /**
     * Recover from parsing errors.
     */
    private fun <T> recoverFromParsingError(
        context: ErrorContext,
        fallbackValue: T?
    ): T? {
        JSONLogger.warn(
            "RecoveryStrategy",
            "parsing_recovery_fallback",
            extra = context.toMap()
        )
        return fallbackValue
    }
    
    /**
     * Recover from database errors with retry.
     */
    private suspend fun <T> recoverFromDatabaseError(
        context: ErrorContext,
        fallbackValue: T?,
        retryAction: (suspend () -> T)?
    ): T? {
        if (retryAction != null) {
            val maxRetries = 2
            repeat(maxRetries) { attempt ->
                try {
                    delay(1000)
                    val result = retryAction()
                    
                    JSONLogger.info(
                        "RecoveryStrategy",
                        "database_recovery_success",
                        extra = mapOf("attempt" to (attempt + 1))
                    )
                    
                    return result
                } catch (e: Throwable) {
                    JSONLogger.warn(
                        "RecoveryStrategy",
                        "database_retry_failed",
                        extra = mapOf(
                            "attempt" to (attempt + 1),
                            "error" to e.message
                        ),
                        throwable = e
                    )
                }
            }
        }
        
        return fallbackValue
    }
    
    /**
     * Generic recovery for unclassified errors.
     */
    private suspend fun <T> recoverGeneric(
        context: ErrorContext,
        fallbackValue: T?,
        retryAction: (suspend () -> T)?
    ): T? {
        if (retryAction != null && context.recoverable) {
            try {
                delay(500)
                val result = retryAction()
                
                JSONLogger.info(
                    "RecoveryStrategy",
                    "generic_recovery_success",
                    extra = context.toMap()
                )
                
                return result
            } catch (e: Throwable) {
                JSONLogger.warn(
                    "RecoveryStrategy",
                    "generic_recovery_failed",
                    extra = context.toMap(),
                    throwable = e
                )
            }
        }
        
        return fallbackValue
    }
}
