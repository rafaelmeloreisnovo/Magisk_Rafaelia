package com.topjohnwu.magisk.core.error

/**
 * Comprehensive error categorization for better bug tracking and handling.
 * Each category represents a specific type of error that can occur in the system.
 */
enum class ErrorCategory {
    NETWORK,           // Network-related errors
    IO,                // File I/O errors
    SECURITY,          // Security and permission errors
    VALIDATION,        // Data validation errors
    PARSING,           // Data parsing errors
    DATABASE,          // Database operation errors
    CONFIGURATION,     // Configuration errors
    RUNTIME,           // Runtime errors
    UNKNOWN;           // Unclassified errors

    companion object {
        fun fromThrowable(throwable: Throwable): ErrorCategory {
            return when (throwable) {
                is java.io.IOException -> IO
                is java.net.UnknownHostException, 
                is java.net.SocketTimeoutException,
                is java.net.ConnectException,
                is java.net.SocketException,
                is java.net.NoRouteToHostException,
                is java.net.ProtocolException -> NETWORK
                is SecurityException -> SECURITY
                is IllegalArgumentException,
                is IllegalStateException -> VALIDATION
                is org.json.JSONException,
                is NumberFormatException -> PARSING
                else -> UNKNOWN
            }
        }
    }
}
