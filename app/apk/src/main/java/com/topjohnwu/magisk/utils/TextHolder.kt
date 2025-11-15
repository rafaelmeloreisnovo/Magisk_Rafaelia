package com.topjohnwu.magisk.utils

import android.content.res.Resources

/**
 * TextHolder - Abstract holder for text that can be either a direct string or a string resource ID
 *
 * This sealed-like class hierarchy provides a type-safe way to handle text that might come from
 * different sources (hardcoded strings vs. Android string resources). This is particularly useful
 * for UI components that need to display text from various sources while maintaining localization
 * support and lazy resource loading.
 *
 * Benefits:
 * - Defers resource loading until getText() is called
 * - Supports string formatting with parameters
 * - Type-safe representation of text sources
 * - Easy to pass around without worrying about Context/Resources availability
 */
abstract class TextHolder {

    /**
     * Returns true if this holder represents empty text
     */
    open val isEmpty: Boolean get() = false
    
    /**
     * Retrieves the actual text content using the provided Resources
     *
     * @param resources Android Resources object for resolving string resources
     * @return The resolved text as a CharSequence
     */
    abstract fun getText(resources: Resources): CharSequence

    // ---

    /**
     * Holds a direct string value
     *
     * Use this when you have a hardcoded string or dynamically generated text
     * that doesn't need localization.
     */
    class String(
        private val value: CharSequence
    ) : TextHolder() {
        override val isEmpty get() = value.isEmpty()
        override fun getText(resources: Resources) = value
    }

    /**
     * Holds a string resource ID
     *
     * Use this when you want to display localized text from strings.xml.
     * The actual string is loaded lazily when getText() is called.
     */
    open class Resource(
        protected val value: Int
    ) : TextHolder() {
        override val isEmpty get() = value == 0
        override fun getText(resources: Resources) = resources.getString(value)
    }

    /**
     * Holds a string resource ID with formatting parameters
     *
     * Use this for localized strings that need dynamic content (e.g., "Welcome, %s!")
     * Parameters can be either regular objects or other TextHolders, which will be
     * resolved recursively before formatting.
     */
    class ResourceArgs(
        value: Int,
        private vararg val params: Any
    ) : Resource(value) {
        override fun getText(resources: Resources): kotlin.String {
            // Replace TextHolder parameters with their resolved strings
            // This allows nested TextHolders for complex formatting scenarios
            val args = params.map { if (it is TextHolder) it.getText(resources) else it }
            return resources.getString(value, *args.toTypedArray())
        }
    }

    // ---

    companion object {
        /** Empty text holder singleton for convenience */
        val EMPTY = String("")
    }
}

// Extension functions for easy conversion to TextHolder

/** Convert a string resource ID to a TextHolder */
fun Int.asText(): TextHolder = TextHolder.Resource(this)

/** Convert a string resource ID with parameters to a TextHolder */
fun Int.asText(vararg params: Any): TextHolder = TextHolder.ResourceArgs(this, *params)

/** Convert a CharSequence to a TextHolder */
fun CharSequence.asText(): TextHolder = TextHolder.String(this)
