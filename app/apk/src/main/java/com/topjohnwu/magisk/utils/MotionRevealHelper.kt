package com.topjohnwu.magisk.utils

import android.animation.Animator
import android.animation.AnimatorSet
import android.animation.ObjectAnimator
import android.view.View
import androidx.core.animation.addListener
import androidx.core.text.layoutDirection
import androidx.core.view.isInvisible
import androidx.core.view.isVisible
import androidx.core.view.marginBottom
import androidx.core.view.marginEnd
import androidx.interpolator.view.animation.FastOutSlowInInterpolator
import com.google.android.material.circularreveal.CircularRevealCompat
import com.google.android.material.circularreveal.CircularRevealWidget
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.topjohnwu.magisk.core.utils.LocaleSetting
import kotlin.math.hypot

/**
 * MotionRevealHelper - Provides coordinated animations for Material Design circular reveal effects
 *
 * This helper creates smooth, coordinated animations between a FloatingActionButton (FAB) and
 * a view that supports circular reveal. It's commonly used for expanding/collapsing UI elements
 * with Material Design motion principles.
 *
 * The animation consists of two parts:
 * 1. FAB movement animation (moves to/from the reveal center point)
 * 2. Circular reveal animation (expands/contracts the target view)
 *
 * These animations are sequenced appropriately:
 * - When expanding: FAB moves first, then view reveals
 * - When collapsing: View reveals (shrinks) first, then FAB returns
 */
object MotionRevealHelper {

    /**
     * Animates a circular reveal with coordinated FAB movement
     *
     * This function orchestrates the complete animation sequence, handling both the
     * circular reveal of the target view and the movement of the FAB.
     *
     * @param revealable The view to reveal/hide (must implement CircularRevealWidget and extend View)
     * @param fab The FloatingActionButton to animate alongside the reveal
     * @param expanded Whether to expand (true) or collapse (false) the revealable view
     *
     * @param CV Type that must be both a CircularRevealWidget and a View
     */
    fun <CV> withViews(
        revealable: CV,
        fab: FloatingActionButton,
        expanded: Boolean
    ) where CV : CircularRevealWidget, CV : View {
        // Set initial state (opposite of target state)
        revealable.revealInfo = revealable.createRevealInfo(!expanded)

        // Create target state info and animations
        val revealInfo = revealable.createRevealInfo(expanded)
        val revealAnim = revealable.createRevealAnim(revealInfo)
        val moveAnim = fab.createMoveAnim(revealInfo)

        // Sequence the animations appropriately
        AnimatorSet().also {
            if (expanded) {
                // When expanding: move FAB first, then reveal view
                it.play(revealAnim).after(moveAnim)
            } else {
                // When collapsing: hide view first, then return FAB
                it.play(moveAnim).after(revealAnim)
            }
        }.start()
    }

    /**
     * Creates a circular reveal animation for the target view
     *
     * The animation either expands from center point (radius 0 -> max) or
     * contracts to center point (max -> radius 0).
     *
     * @param revealInfo Target reveal state (center point and radius)
     * @return Animator that performs the circular reveal
     */
    private fun <CV> CV.createRevealAnim(
        revealInfo: CircularRevealWidget.RevealInfo
    ): Animator where CV : CircularRevealWidget, CV : View =
        CircularRevealCompat.createCircularReveal(
            this,
            revealInfo.centerX,
            revealInfo.centerY,
            revealInfo.radius
        ).apply {
            addListener(onStart = {
                // Make view visible when starting to reveal
                isVisible = true
            }, onEnd = {
                // Hide view when fully collapsed (radius = 0)
                if (revealInfo.radius == 0f) {
                    isInvisible = true
                }
            })
        }

    /**
     * Creates a coordinated movement animation for the FAB
     *
     * The FAB moves from its normal position to the reveal center point (or vice versa),
     * accounting for RTL (Right-to-Left) layout direction and view margins.
     *
     * @param revealInfo Target reveal state (determines movement destination)
     * @return Animator that moves the FAB in X and Y directions
     */
    private fun FloatingActionButton.createMoveAnim(
        revealInfo: CircularRevealWidget.RevealInfo
    ): Animator = AnimatorSet().also {
        // Use Material Design's fast-out-slow-in easing
        it.interpolator = FastOutSlowInInterpolator()
        
        // Show FAB when starting, hide when reveal is fully expanded
        it.addListener(onStart = { show() }, onEnd = { if (revealInfo.radius != 0f) hide() })

        // Calculate horizontal movement, respecting RTL layout
        val rtlMod =
            if (LocaleSetting.instance.currentLocale.layoutDirection == View.LAYOUT_DIRECTION_RTL)
                1f else -1f
        val maxX = revealInfo.centerX - marginEnd - measuredWidth / 2f
        val targetX = if (revealInfo.radius == 0f) 0f else maxX * rtlMod
        val moveX = ObjectAnimator.ofFloat(this, View.TRANSLATION_X, targetX)

        // Calculate vertical movement
        val maxY = revealInfo.centerY - marginBottom - measuredHeight / 2f
        val targetY = if (revealInfo.radius == 0f) 0f else -maxY
        val moveY = ObjectAnimator.ofFloat(this, View.TRANSLATION_Y, targetY)

        // Play both X and Y animations simultaneously
        it.playTogether(moveX, moveY)
    }

    /**
     * Creates reveal info for a view's center point with appropriate radius
     *
     * @param expanded If true, radius extends to cover entire view; if false, radius is 0
     * @return RevealInfo with center coordinates and calculated radius
     */
    private fun View.createRevealInfo(expanded: Boolean): CircularRevealWidget.RevealInfo {
        // Center point, adjusted for bottom padding
        val cX = measuredWidth / 2f
        val cY = measuredHeight / 2f - paddingBottom
        // Radius: use Pythagorean theorem to reach furthest corner, or 0 for collapsed
        return CircularRevealWidget.RevealInfo(cX, cY, if (expanded) hypot(cX, cY) else 0f)
    }

}
