package com.topjohnwu.magisk.utils

import android.content.ContentResolver
import android.provider.Settings

/**
 * AccessibilityUtils - Utilities for checking accessibility and animation settings
 *
 * Provides helper functions to query system settings related to accessibility features
 * and user interface preferences. This is particularly useful for adapting app behavior
 * to respect user preferences and accessibility needs.
 */
class AccessibilityUtils {
    companion object {
        /**
         * Checks if system animations are enabled
         *
         * This function queries three animation scale settings:
         * - ANIMATOR_DURATION_SCALE: Controls property animators and transitions
         * - TRANSITION_ANIMATION_SCALE: Controls activity/fragment transitions
         * - WINDOW_ANIMATION_SCALE: Controls window animations (minimize/maximize)
         *
         * Animations are considered disabled only if ALL three scales are set to 0.
         * This typically happens when:
         * - User disables animations in Developer Options
         * - Accessibility features require reduced motion
         * - Device is in battery saver mode (depending on settings)
         *
         * @param cr ContentResolver to query system settings
         * @return true if any animation scale is non-zero, false if all are disabled
         *
         * Usage example:
         * ```
         * if (AccessibilityUtils.isAnimationEnabled(contentResolver)) {
         *     // Show fancy animations
         * } else {
         *     // Use instant state changes
         * }
         * ```
         */
        fun isAnimationEnabled(cr: ContentResolver): Boolean {
            return !(Settings.Global.getFloat(cr, Settings.Global.ANIMATOR_DURATION_SCALE, 1.0f) == 0.0f
                && Settings.Global.getFloat(cr, Settings.Global.TRANSITION_ANIMATION_SCALE, 1.0f) == 0.0f
                && Settings.Global.getFloat(cr, Settings.Global.WINDOW_ANIMATION_SCALE, 1.0f) == 0.0f)
        }
    }
}
