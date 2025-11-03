package com.topjohnwu.magisk.core.examples

import com.topjohnwu.magisk.core.error.ErrorCategory
import com.topjohnwu.magisk.core.error.ErrorContext
import com.topjohnwu.magisk.core.error.ErrorHandlerUtil
import com.topjohnwu.magisk.core.error.ErrorSeverity
import com.topjohnwu.magisk.core.error.RecoveryStrategy
import com.topjohnwu.magisk.core.logging.JSONLogger
import com.topjohnwu.magisk.core.validation.TypeValidator
import kotlinx.coroutines.runBlocking

/**
 * Comprehensive examples demonstrating the error handling and bug treatment system.
 * These examples show best practices for implementing robust error handling across
 * all methods and conditions in your code.
 */
object ErrorHandlingExample {

    /**
     * Example 1: Basic error handling with logging at different levels
     */
    fun basicLoggingExample() {
        JSONLogger.trace("Example", "function_entry", extra = mapOf("function" to "basicLoggingExample"))
        
        try {
            // Some operation
            JSONLogger.debug("Example", "processing", extra = mapOf("step" to 1))
            
            // Success case
            JSONLogger.info("Example", "operation_complete", extra = mapOf("result" to "success"))
            
        } catch (e: Exception) {
            JSONLogger.error("Example", "operation_failed", throwable = e)
        }
    }

    /**
     * Example 2: Input validation with TypeValidator
     */
    fun inputValidationExample(
        username: String?,
        email: String?,
        age: Int?,
        tags: List<String>?
    ): Result<Unit> {
        // Validate all inputs together
        val validationResult = TypeValidator.validateAll(
            component = "UserRegistration",
            "username" to TypeValidator.validateString(
                username,
                "username",
                "UserRegistration",
                minLength = 3,
                maxLength = 20,
                pattern = Regex("^[a-zA-Z0-9_]+$")
            ),
            "email" to TypeValidator.validateString(
                email,
                "email",
                "UserRegistration",
                pattern = Regex("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
            ),
            "age" to TypeValidator.validateInt(
                age,
                "age",
                "UserRegistration",
                min = 13,
                max = 120
            ),
            "tags" to TypeValidator.validateCollection(
                tags,
                "tags",
                "UserRegistration",
                allowEmpty = true,
                maxSize = 10
            )
        )
        
        return validationResult.onSuccess {
            JSONLogger.info("UserRegistration", "validation_success")
        }.onFailure { error ->
            JSONLogger.error("UserRegistration", "validation_failed", throwable = error)
        }
    }

    /**
     * Example 3: Network operation with retry logic
     */
    fun networkOperationExample() = runBlocking {
        val result = ErrorHandlerUtil.executeWithRetry(
            component = "NetworkService",
            operation = "fetchUserData",
            maxAttempts = 3,
            delayMs = 1000,
            exponentialBackoff = true,
            onError = { errorContext ->
                // Custom error handling
                if (errorContext.severity == ErrorSeverity.HIGH) {
                    // Alert monitoring system
                    JSONLogger.fatal("NetworkService", "critical_network_error", extra = errorContext.toMap())
                }
            }
        ) { attempt ->
            JSONLogger.debug("NetworkService", "attempt", extra = mapOf("attempt" to attempt))
            
            // Simulate network call
            // In real code: apiClient.fetchUserData()
            if (attempt < 2) {
                throw java.net.SocketTimeoutException("Connection timeout")
            }
            "User data fetched successfully"
        }
        
        result.onSuccess { data ->
            JSONLogger.info("NetworkService", "fetch_success", extra = mapOf("data" to data))
        }.onFailure { error ->
            JSONLogger.error("NetworkService", "fetch_failed", throwable = error)
        }
    }

    /**
     * Example 4: Safe execution with default fallback
     */
    fun safeExecutionExample(configFile: String): Map<String, Any> {
        return ErrorHandlerUtil.executeSafe(
            component = "ConfigLoader",
            operation = "loadConfig",
            severity = ErrorSeverity.MEDIUM,
            defaultValue = mapOf("default" to true),
            onError = { errorContext ->
                JSONLogger.warn(
                    "ConfigLoader",
                    "using_default_config",
                    extra = errorContext.toMap()
                )
            }
        ) {
            // Simulate config loading
            // In real code: parseConfigFile(configFile)
            if (configFile.isEmpty()) {
                throw IllegalArgumentException("Config file path is empty")
            }
            mapOf("loaded" to true, "file" to configFile)
        } ?: mapOf("default" to true)
    }

    /**
     * Example 5: Error recovery with fallback strategy
     */
    fun errorRecoveryExample() = runBlocking {
        // Simulate an error scenario
        val errorContext = ErrorContext(
            category = ErrorCategory.NETWORK,
            throwable = java.net.ConnectException("Connection refused"),
            component = "DataService",
            operation = "syncData",
            metadata = mapOf("server" to "api.example.com"),
            severity = ErrorSeverity.HIGH,
            recoverable = true,
            userMessage = "Unable to connect to server. Retrying..."
        )
        
        // Attempt recovery
        val recovered = RecoveryStrategy.recover(
            context = errorContext,
            fallbackValue = emptyList<String>(),
            retryAction = {
                // Retry logic
                JSONLogger.info("DataService", "retry_sync")
                listOf("recovered", "data")
            }
        )
        
        JSONLogger.info(
            "DataService",
            "recovery_complete",
            extra = mapOf("recovered" to recovered)
        )
    }

    /**
     * Example 6: Complex workflow with comprehensive error handling
     */
    fun complexWorkflowExample(userId: String?, operation: String?) = runBlocking {
        JSONLogger.info("Workflow", "workflow_started", extra = mapOf("userId" to userId))
        
        // Step 1: Validate inputs
        val userIdResult = TypeValidator.validateString(
            userId,
            "userId",
            "Workflow",
            pattern = Regex("^[a-f0-9]{32}$")
        )
        
        val operationResult = TypeValidator.validateString(
            operation,
            "operation",
            "Workflow",
            allowEmpty = false
        )
        
        val validationResult = TypeValidator.validateAll(
            "Workflow",
            "userId" to userIdResult,
            "operation" to operationResult
        )
        
        if (validationResult.isFailure) {
            JSONLogger.error("Workflow", "validation_failed", throwable = validationResult.exceptionOrNull())
            return@runBlocking Result.failure<String>(validationResult.exceptionOrNull()!!)
        }
        
        // Step 2: Execute operation with retry
        val operationResult2 = ErrorHandlerUtil.executeWithRetry(
            component = "Workflow",
            operation = "executeOperation",
            maxAttempts = 3
        ) { attempt ->
            JSONLogger.debug("Workflow", "operation_attempt", extra = mapOf("attempt" to attempt))
            
            // Simulate operation execution
            if (attempt < 2) {
                throw RuntimeException("Temporary failure")
            }
            "Operation completed successfully"
        }
        
        // Step 3: Process results
        operationResult2.onSuccess { result ->
            JSONLogger.info("Workflow", "workflow_success", extra = mapOf("result" to result))
        }.onFailure { error ->
            val errorContext = ErrorContext(
                category = ErrorCategory.fromThrowable(error),
                throwable = error,
                component = "Workflow",
                operation = "executeOperation",
                metadata = mapOf("userId" to userId, "operation" to operation),
                severity = ErrorSeverity.HIGH,
                recoverable = false
            )
            
            JSONLogger.fatal("Workflow", "workflow_failed", extra = errorContext.toMap())
        }
        
        return@runBlocking operationResult2
    }

    /**
     * Example 7: Error statistics monitoring
     */
    fun monitoringExample() {
        // Get error statistics (type-safe)
        val stats = ErrorHandlerUtil.getErrorStats()
        
        JSONLogger.info(
            "Monitoring",
            "error_statistics",
            extra = mapOf(
                "totalErrors" to stats.totalErrors,
                "byCategory" to stats.byCategory,
                "bySeverity" to stats.bySeverity,
                "byComponent" to stats.byComponent
            )
        )
        
        // Analyze patterns (type-safe)
        val byCategory = stats.byCategory
        val bySeverity = stats.bySeverity
        
        // Log alerts for high severity errors
        val criticalCount = bySeverity[com.topjohnwu.magisk.core.error.ErrorSeverity.CRITICAL] ?: 0
        if (criticalCount > 0) {
            JSONLogger.fatal(
                "Monitoring",
                "critical_errors_detected",
                extra = mapOf("count" to criticalCount)
            )
        }
    }

    /**
     * Example 8: Database operation with error handling
     */
    fun databaseOperationExample(query: String?) = runBlocking {
        // Validate query
        val queryResult = TypeValidator.validateString(
            query,
            "query",
            "DatabaseService",
            allowEmpty = false,
            maxLength = 1000
        )
        
        if (queryResult.isFailure) {
            return@runBlocking Result.failure<List<Any>>(queryResult.exceptionOrNull()!!)
        }
        
        // Execute with error handling
        ErrorHandlerUtil.executeWithRetry(
            component = "DatabaseService",
            operation = "executeQuery",
            maxAttempts = 2,
            delayMs = 1000
        ) { attempt ->
            JSONLogger.debug(
                "DatabaseService",
                "query_attempt",
                extra = mapOf("attempt" to attempt, "query" to query)
            )
            
            // Simulate database operation
            try {
                // In real code: database.execute(query)
                listOf("result1", "result2")
            } catch (e: Exception) {
                val errorContext = ErrorContext(
                    category = ErrorCategory.DATABASE,
                    throwable = e,
                    component = "DatabaseService",
                    operation = "executeQuery",
                    metadata = mapOf("query" to query, "attempt" to attempt),
                    severity = ErrorSeverity.HIGH,
                    recoverable = true
                )
                
                // Attempt recovery
                RecoveryStrategy.recover(
                    context = errorContext,
                    fallbackValue = emptyList(),
                    retryAction = { 
                        // Retry with connection reset
                        listOf("recovered_result") 
                    }
                ) ?: throw e
            }
        }
    }
}
