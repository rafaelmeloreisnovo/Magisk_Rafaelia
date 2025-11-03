package com.topjohnwu.magisk.core.validation

import com.topjohnwu.magisk.core.logging.JSONLogger

/**
 * Comprehensive type validation utility for all variable types.
 * Ensures data integrity and proper type handling across the application.
 */
object TypeValidator {
    
    /**
     * Validate string is not null or empty.
     */
    fun validateString(
        value: String?,
        fieldName: String,
        component: String,
        allowEmpty: Boolean = false,
        maxLength: Int? = null,
        minLength: Int? = null,
        pattern: Regex? = null
    ): Result<String> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        if (!allowEmpty && value.isEmpty()) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be empty"))
        }
        
        maxLength?.let {
            if (value.length > it) {
                return Result.failure(IllegalArgumentException("$fieldName exceeds maximum length of $it"))
            }
        }
        
        minLength?.let {
            if (value.length < it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at least $it characters"))
            }
        }
        
        pattern?.let {
            if (!value.matches(it)) {
                return Result.failure(IllegalArgumentException("$fieldName does not match required pattern"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate integer is within range.
     */
    fun validateInt(
        value: Int?,
        fieldName: String,
        component: String,
        min: Int? = null,
        max: Int? = null
    ): Result<Int> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        min?.let {
            if (value < it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at least $it"))
            }
        }
        
        max?.let {
            if (value > it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at most $it"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate long is within range.
     */
    fun validateLong(
        value: Long?,
        fieldName: String,
        component: String,
        min: Long? = null,
        max: Long? = null
    ): Result<Long> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        min?.let {
            if (value < it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at least $it"))
            }
        }
        
        max?.let {
            if (value > it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at most $it"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate double is within range.
     */
    fun validateDouble(
        value: Double?,
        fieldName: String,
        component: String,
        min: Double? = null,
        max: Double? = null
    ): Result<Double> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        if (value.isNaN() || value.isInfinite()) {
            return Result.failure(IllegalArgumentException("$fieldName must be a valid number"))
        }
        
        min?.let {
            if (value < it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at least $it"))
            }
        }
        
        max?.let {
            if (value > it) {
                return Result.failure(IllegalArgumentException("$fieldName must be at most $it"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate boolean is not null.
     */
    fun validateBoolean(
        value: Boolean?,
        fieldName: String,
        component: String
    ): Result<Boolean> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        return Result.success(value)
    }

    /**
     * Validate collection is not null or empty.
     */
    fun <T> validateCollection(
        value: Collection<T>?,
        fieldName: String,
        component: String,
        allowEmpty: Boolean = false,
        minSize: Int? = null,
        maxSize: Int? = null
    ): Result<Collection<T>> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        if (!allowEmpty && value.isEmpty()) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be empty"))
        }
        
        minSize?.let {
            if (value.size < it) {
                return Result.failure(IllegalArgumentException("$fieldName must have at least $it items"))
            }
        }
        
        maxSize?.let {
            if (value.size > it) {
                return Result.failure(IllegalArgumentException("$fieldName must have at most $it items"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate map is not null or empty.
     */
    fun <K, V> validateMap(
        value: Map<K, V>?,
        fieldName: String,
        component: String,
        allowEmpty: Boolean = false,
        minSize: Int? = null,
        maxSize: Int? = null
    ): Result<Map<K, V>> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        if (!allowEmpty && value.isEmpty()) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be empty"))
        }
        
        minSize?.let {
            if (value.size < it) {
                return Result.failure(IllegalArgumentException("$fieldName must have at least $it entries"))
            }
        }
        
        maxSize?.let {
            if (value.size > it) {
                return Result.failure(IllegalArgumentException("$fieldName must have at most $it entries"))
            }
        }
        
        return Result.success(value)
    }

    /**
     * Validate enum value.
     */
    inline fun <reified T : Enum<T>> validateEnum(
        value: String?,
        fieldName: String,
        component: String
    ): Result<T> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        
        return try {
            val enumValue = enumValueOf<T>(value)
            Result.success(enumValue)
        } catch (e: IllegalArgumentException) {
            val validValues = enumValues<T>().joinToString(", ") { it.name }
            Result.failure(
                IllegalArgumentException("$fieldName must be one of: $validValues")
            )
        }
    }

    /**
     * Validate object is not null.
     */
    fun <T : Any> validateNotNull(
        value: T?,
        fieldName: String,
        component: String
    ): Result<T> {
        if (value == null) {
            return Result.failure(IllegalArgumentException("$fieldName cannot be null"))
        }
        return Result.success(value)
    }

    /**
     * Validate all conditions in a list.
     * @param enableLogging If true, logs validation failures (default: true)
     */
    fun validateAll(
        component: String,
        vararg validations: Pair<String, Result<*>>,
        enableLogging: Boolean = true
    ): Result<Unit> {
        val failures = mutableListOf<String>()
        
        validations.forEach { (name, result) ->
            result.onFailure { error ->
                failures.add("$name: ${error.message}")
                if (enableLogging) {
                    JSONLogger.warn(
                        component,
                        "validation_failed",
                        extra = mapOf(
                            "field" to name,
                            "error" to error.message
                        )
                    )
                }
            }
        }
        
        return if (failures.isEmpty()) {
            Result.success(Unit)
        } else {
            Result.failure(
                IllegalArgumentException("Validation failed: ${failures.joinToString("; ")}")
            )
        }
    }
}
