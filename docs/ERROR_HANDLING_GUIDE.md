# Comprehensive Error Handling and Bug Treatment Guide

## Overview

This guide describes the enterprise-level error handling and bug treatment infrastructure implemented in Magisk_Rafaelia. The system provides comprehensive error tracking, categorization, validation, and recovery mechanisms across all components.

## Core Components

### 1. JSONLogger (Enhanced)

The enhanced JSONLogger provides multiple logging levels for comprehensive application monitoring:

**Available Levels:**
- `TRACE` - Detailed execution flow tracking
- `DEBUG` - Development debugging information
- `INFO` - General informational messages
- `WARN` - Warning conditions that should be reviewed
- `ERROR` - Error conditions that affect functionality
- `FATAL` - Critical errors that may cause system instability

**Features:**
- Automatic timestamp tracking
- Component and event categorization
- Session ID tracking
- Structured metadata support
- Exception stack trace capture

**Usage Example:**
```kotlin
JSONLogger.info("MyComponent", "operation_started", sessionId = "session-123")
JSONLogger.error("MyComponent", "operation_failed", throwable = exception)
JSONLogger.warn("MyComponent", "validation_issue", extra = mapOf("field" to "email"))
```

### 2. Error Categorization System

#### ErrorCategory Enum
Classifies errors into specific categories for better tracking and handling:

- `NETWORK` - Network-related errors (timeouts, connection issues)
- `IO` - File I/O operations
- `SECURITY` - Security and permission issues
- `VALIDATION` - Data validation failures
- `PARSING` - Data parsing errors (JSON, XML, etc.)
- `DATABASE` - Database operation errors
- `CONFIGURATION` - Configuration errors
- `RUNTIME` - Runtime errors
- `UNKNOWN` - Unclassified errors

#### ErrorContext
Comprehensive error context that captures all relevant information:

```kotlin
data class ErrorContext(
    val timestamp: Instant,
    val category: ErrorCategory,
    val throwable: Throwable,
    val component: String,
    val operation: String,
    val metadata: Map<String, Any?>,
    val severity: ErrorSeverity,
    val recoverable: Boolean,
    val userMessage: String?
)
```

#### ErrorSeverity Enum
- `LOW` - Minor issues, system continues normally
- `MEDIUM` - Moderate issues, some functionality affected
- `HIGH` - Severe issues, major functionality affected
- `CRITICAL` - Critical issues, system stability at risk

### 3. ErrorHandlerUtil

The central error handling utility providing comprehensive error management.

#### Key Features:

**executeWithRetry** - Execute operations with automatic retry logic:
```kotlin
val result = ErrorHandlerUtil.executeWithRetry(
    component = "DataService",
    operation = "fetchData",
    maxAttempts = 3,
    delayMs = 1000,
    exponentialBackoff = true
) { attempt ->
    // Your operation code here
    dataSource.fetch()
}
```

**executeSafe** - Execute operations with error handling but no retries:
```kotlin
val result = ErrorHandlerUtil.executeSafe(
    component = "DataService",
    operation = "parseData",
    severity = ErrorSeverity.MEDIUM,
    defaultValue = emptyList()
) {
    // Your operation code here
    parser.parse(data)
}
```

**validate** - Validate data with comprehensive error tracking:
```kotlin
val result = ErrorHandlerUtil.validate(
    value = userInput,
    component = "UserForm",
    fieldName = "email",
    validator = { it.contains("@") },
    errorMessage = "Invalid email format"
)
```

**Error Statistics** - Monitor error patterns:
```kotlin
val stats = ErrorHandlerUtil.getErrorStats()
// Returns: totalErrors, byCategory, bySeverity, byComponent, recentErrors
```

### 4. TypeValidator

Comprehensive validation for all data types with detailed error messages.

#### String Validation:
```kotlin
val result = TypeValidator.validateString(
    value = input,
    fieldName = "username",
    component = "UserService",
    allowEmpty = false,
    minLength = 3,
    maxLength = 20,
    pattern = Regex("[a-zA-Z0-9]+")
)
```

#### Numeric Validation:
```kotlin
val intResult = TypeValidator.validateInt(
    value = age,
    fieldName = "age",
    component = "UserService",
    min = 0,
    max = 150
)

val doubleResult = TypeValidator.validateDouble(
    value = price,
    fieldName = "price",
    component = "PriceService",
    min = 0.0,
    max = 999999.99
)
```

#### Collection Validation:
```kotlin
val listResult = TypeValidator.validateCollection(
    value = items,
    fieldName = "items",
    component = "CartService",
    allowEmpty = false,
    minSize = 1,
    maxSize = 100
)
```

#### Composite Validation:
```kotlin
val result = TypeValidator.validateAll(
    component = "UserService",
    "username" to TypeValidator.validateString(username, "username", "UserService"),
    "age" to TypeValidator.validateInt(age, "age", "UserService", min = 0),
    "email" to TypeValidator.validateString(email, "email", "UserService", pattern = emailRegex)
)
```

### 5. RecoveryStrategy

Intelligent error recovery based on error category and context.

```kotlin
val recovered = RecoveryStrategy.recover(
    context = errorContext,
    fallbackValue = defaultValue,
    retryAction = { 
        // Retry logic
        service.retryOperation()
    }
)
```

**Recovery Strategies by Category:**
- **NETWORK**: Exponential backoff retry (3 attempts)
- **IO**: Single retry with delay
- **DATABASE**: Two retry attempts with 1s delay
- **VALIDATION/PARSING**: Fallback to default value (not retryable)
- **Generic**: Single retry if recoverable

### 6. Enhanced TestWrapper

Comprehensive testing utilities with detailed error tracking.

```kotlin
TestWrapper.runWithRetries(
    maxAttempts = 5,
    delayMs = 2000,
    exponentialBackoff = true,
    actionName = "integration_test"
) {
    // Your test code here
    performTestOperation()
}

val result = TestWrapper.runSafe(
    actionName = "unit_test",
    defaultValue = false
) {
    // Your test code here
    performTestCheck()
}
```

## Best Practices

### 1. Always Log at Appropriate Levels
- Use `TRACE` for detailed flow tracking during development
- Use `DEBUG` for debugging information
- Use `INFO` for significant business events
- Use `WARN` for recoverable issues
- Use `ERROR` for functionality-affecting errors
- Use `FATAL` for critical system errors

### 2. Provide Context
Always include relevant metadata in logs:
```kotlin
JSONLogger.error(
    "PaymentService",
    "payment_failed",
    sessionId = paymentSession,
    extra = mapOf(
        "userId" to userId,
        "amount" to amount,
        "currency" to currency,
        "paymentMethod" to method
    ),
    throwable = exception
)
```

### 3. Use Type Validation
Validate all inputs before processing:
```kotlin
val validatedInput = TypeValidator.validateString(
    value = userInput,
    fieldName = "input",
    component = "InputProcessor",
    allowEmpty = false
).getOrElse { 
    return handleValidationError(it)
}
```

### 4. Implement Retry Logic Appropriately
Use retry logic for transient failures:
```kotlin
val result = ErrorHandlerUtil.executeWithRetry(
    component = "APIClient",
    operation = "fetchData",
    maxAttempts = 3,
    exponentialBackoff = true
) { attempt ->
    apiClient.fetch()
}
```

### 5. Handle Errors Gracefully
Provide meaningful user messages:
```kotlin
val context = ErrorContext(
    category = ErrorCategory.NETWORK,
    throwable = exception,
    component = "NetworkService",
    operation = "fetchData",
    severity = ErrorSeverity.HIGH,
    recoverable = true,
    userMessage = "Unable to connect to server. Please check your internet connection."
)
```

### 6. Monitor Error Patterns
Regularly review error statistics:
```kotlin
val stats = ErrorHandlerUtil.getErrorStats()
// Analyze patterns to identify systemic issues
```

### 7. Use ErrorCategory for Routing
Route errors to appropriate handlers based on category:
```kotlin
when (errorContext.category) {
    ErrorCategory.NETWORK -> handleNetworkError(errorContext)
    ErrorCategory.VALIDATION -> handleValidationError(errorContext)
    ErrorCategory.SECURITY -> handleSecurityError(errorContext)
    else -> handleGenericError(errorContext)
}
```

## Integration Example

Complete example integrating all components:

```kotlin
class DataService {
    suspend fun fetchAndProcessData(userId: String): Result<ProcessedData> {
        // Validate input
        val validatedUserId = TypeValidator.validateString(
            value = userId,
            fieldName = "userId",
            component = "DataService",
            pattern = Regex("[a-f0-9]{32}")
        ).getOrElse { 
            return Result.failure(it)
        }
        
        // Fetch with retry
        val fetchResult = ErrorHandlerUtil.executeWithRetry(
            component = "DataService",
            operation = "fetchUserData",
            maxAttempts = 3,
            exponentialBackoff = true
        ) { attempt ->
            JSONLogger.debug("DataService", "fetch_attempt", extra = mapOf("attempt" to attempt))
            dataSource.fetch(validatedUserId)
        }
        
        val rawData = fetchResult.getOrElse { error ->
            val errorContext = ErrorContext(
                category = ErrorCategory.fromThrowable(error),
                throwable = error,
                component = "DataService",
                operation = "fetchUserData",
                metadata = mapOf("userId" to validatedUserId),
                severity = ErrorSeverity.HIGH,
                recoverable = true
            )
            
            // Attempt recovery
            return RecoveryStrategy.recover(
                context = errorContext,
                fallbackValue = ProcessedData.empty(),
                retryAction = { dataSource.fetch(validatedUserId) }
            )?.let { Result.success(it as ProcessedData) } 
                ?: Result.failure(error)
        }
        
        // Process with error handling
        return ErrorHandlerUtil.executeSafe(
            component = "DataService",
            operation = "processData",
            severity = ErrorSeverity.MEDIUM,
            defaultValue = null
        ) {
            processor.process(rawData)
        }?.let { Result.success(it) } 
            ?: Result.failure(RuntimeException("Processing failed"))
    }
}
```

## Conclusion

This comprehensive error handling system ensures maximum reliability and maintainability across all application components. By following these patterns and best practices, you can achieve enterprise-level bug treatment and error recovery throughout the codebase.

For questions or improvements, refer to the implementation files in:
- `app/src/main/java/com/topjohnwu/magisk/core/logging/`
- `app/src/main/java/com/topjohnwu/magisk/core/error/`
- `app/src/main/java/com/topjohnwu/magisk/core/validation/`
