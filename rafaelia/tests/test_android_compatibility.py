# Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
# Instituto Rafael - CientiEspiritual Philosophy
#
# This file is part of Magisk_Rafaelia.
#
# Magisk_Rafaelia is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

"""Tests for Android compatibility module."""

import unittest
import sys
from pathlib import Path

# Add parent directory to path for direct module testing without package installation
# This allows testing the module in development mode
sys.path.insert(0, str(Path(__file__).parent.parent))

from core.android_compatibility import (
    AndroidVersion,
    KernelVersion,
    DeviceInfo,
    AndroidCompatibilityChecker,
    check_compatibility,
    is_android_15_compatible,
    is_rmx3834_compatible
)


class TestAndroidVersion(unittest.TestCase):
    """Test AndroidVersion enum."""
    
    def test_from_api_level(self):
        """Test getting version from API level."""
        version = AndroidVersion.from_api_level(33)
        self.assertIsNotNone(version)
        self.assertEqual(version.api_level, 33)
        self.assertEqual(version.codename, "Tiramisu")
    
    def test_from_api_level_invalid(self):
        """Test with invalid API level."""
        version = AndroidVersion.from_api_level(999)
        self.assertIsNone(version)
    
    def test_from_version(self):
        """Test getting version from version string."""
        version = AndroidVersion.from_version("15")
        self.assertIsNotNone(version)
        self.assertEqual(version.api_level, 35)


class TestKernelVersion(unittest.TestCase):
    """Test KernelVersion class."""
    
    def test_parse_simple(self):
        """Test parsing simple kernel version."""
        kernel = KernelVersion.parse("5.15.178")
        self.assertEqual(kernel.major, 5)
        self.assertEqual(kernel.minor, 15)
        self.assertEqual(kernel.patch, 178)
        self.assertEqual(kernel.suffix, "")
    
    def test_parse_with_suffix(self):
        """Test parsing kernel version with suffix."""
        kernel = KernelVersion.parse("5.15.178-android13-8-gabf75819a85e-ab569")
        self.assertEqual(kernel.major, 5)
        self.assertEqual(kernel.minor, 15)
        self.assertEqual(kernel.patch, 178)
        self.assertEqual(kernel.suffix, "android13-8-gabf75819a85e-ab569")
    
    def test_parse_invalid(self):
        """Test parsing invalid kernel version."""
        with self.assertRaises(ValueError):
            KernelVersion.parse("invalid")
    
    def test_comparison(self):
        """Test kernel version comparison."""
        v1 = KernelVersion.parse("5.15.178")
        v2 = KernelVersion.parse("5.15.179")
        v3 = KernelVersion.parse("5.16.0")
        
        self.assertTrue(v1 < v2)
        self.assertTrue(v2 < v3)
        self.assertTrue(v1 < v3)
        self.assertFalse(v2 < v1)
    
    def test_equality(self):
        """Test kernel version equality."""
        v1 = KernelVersion.parse("5.15.178")
        v2 = KernelVersion.parse("5.15.178-android13")
        
        # Should be equal regardless of suffix
        self.assertEqual(v1, v2)
    
    def test_str(self):
        """Test string representation."""
        kernel = KernelVersion.parse("5.15.178-android13-8")
        self.assertEqual(str(kernel), "5.15.178-android13-8")


class TestDeviceInfo(unittest.TestCase):
    """Test DeviceInfo class."""
    
    def test_is_realme(self):
        """Test Realme device detection."""
        device1 = DeviceInfo(
            model="RMX3834",
            manufacturer="Realme",
            device="RMX3834",
            build_id="test"
        )
        self.assertTrue(device1.is_realme())
        
        device2 = DeviceInfo(
            model="SM-G998B",
            manufacturer="Samsung",
            device="samsung",
            build_id="test"
        )
        self.assertFalse(device2.is_realme())
    
    def test_is_specific_model(self):
        """Test specific model detection."""
        device = DeviceInfo(
            model="RMX3834",
            manufacturer="Realme",
            device="RMX3834",
            build_id="test"
        )
        
        self.assertTrue(device.is_specific_model("RMX3834"))
        self.assertTrue(device.is_specific_model("RMX"))
        self.assertFalse(device.is_specific_model("SM-"))


class TestAndroidCompatibilityChecker(unittest.TestCase):
    """Test AndroidCompatibilityChecker class."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.checker = AndroidCompatibilityChecker()
    
    def test_check_android_version_valid(self):
        """Test checking valid Android version."""
        is_compat, warnings, errors = self.checker.check_android_version(33)
        self.assertTrue(is_compat)
        self.assertEqual(len(errors), 0)
    
    def test_check_android_version_too_old(self):
        """Test checking too old Android version."""
        is_compat, warnings, errors = self.checker.check_android_version(21)
        self.assertFalse(is_compat)
        self.assertGreater(len(errors), 0)
    
    def test_check_android_version_unknown(self):
        """Test checking unknown Android version."""
        is_compat, warnings, errors = self.checker.check_android_version(999)
        self.assertTrue(is_compat)  # Should still be compatible, just with warning
        self.assertGreater(len(warnings), 0)
    
    def test_check_kernel_version_valid(self):
        """Test checking valid kernel version."""
        is_compat, warnings, errors = self.checker.check_kernel_version(
            "5.15.178-android13-8-gabf75819a85e-ab569",
            33
        )
        self.assertTrue(is_compat)
        self.assertEqual(len(errors), 0)
    
    def test_check_kernel_version_too_old(self):
        """Test checking too old kernel version."""
        is_compat, warnings, errors = self.checker.check_kernel_version("3.10.0", 33)
        self.assertFalse(is_compat)
        self.assertGreater(len(errors), 0)
    
    def test_check_kernel_version_invalid(self):
        """Test checking invalid kernel version."""
        is_compat, warnings, errors = self.checker.check_kernel_version("invalid", 33)
        self.assertFalse(is_compat)
        self.assertGreater(len(errors), 0)
    
    def test_check_device_compatibility_rmx3834(self):
        """Test checking RMX3834 device compatibility."""
        device = DeviceInfo(
            model="RMX3834",
            manufacturer="Realme",
            device="RMX3834",
            build_id="RMX3834export_15_F.94",
            android_version=AndroidVersion.VANILLA_ICE_CREAM,
            kernel_version=KernelVersion.parse("5.15.178")
        )
        
        is_compat, warnings, errors, recommendations = self.checker.check_device_compatibility(device)
        self.assertTrue(is_compat)
        self.assertEqual(len(errors), 0)
    
    def test_check_full_compatibility_android13(self):
        """Test full compatibility check for Android 13."""
        result = self.checker.check_full_compatibility(
            api_level=33,
            kernel_version="5.15.178-android13-8-gabf75819a85e-ab569"
        )
        
        self.assertTrue(result.is_compatible)
        self.assertTrue(result.android_compatible)
        self.assertTrue(result.kernel_compatible)
    
    def test_check_full_compatibility_rmx3834_android15(self):
        """Test full compatibility check for RMX3834 on Android 15."""
        device = DeviceInfo(
            model="RMX3834",
            manufacturer="Realme",
            device="RMX3834",
            build_id="RMX3834export_15_F.94",
            android_version=AndroidVersion.VANILLA_ICE_CREAM,
            kernel_version=KernelVersion.parse("5.15.178")
        )
        
        result = self.checker.check_full_compatibility(
            api_level=35,
            kernel_version="5.15.178",
            device_info=device
        )
        
        self.assertTrue(result.is_compatible)
        self.assertTrue(result.android_compatible)
        self.assertTrue(result.kernel_compatible)
        self.assertTrue(result.device_compatible)


class TestConvenienceFunctions(unittest.TestCase):
    """Test module-level convenience functions."""
    
    def test_check_compatibility_android13(self):
        """Test check_compatibility for Android 13."""
        result = check_compatibility(
            api_level=33,
            kernel_version="5.15.178-android13-8-gabf75819a85e-ab569"
        )
        
        self.assertTrue(result.is_compatible)
    
    def test_check_compatibility_rmx3834(self):
        """Test check_compatibility for RMX3834."""
        result = check_compatibility(
            api_level=35,
            kernel_version="5.15.178",
            device_model="RMX3834",
            device_manufacturer="Realme",
            build_id="RMX3834export_15_F.94"
        )
        
        self.assertTrue(result.is_compatible)
        self.assertTrue(result.device_compatible)
    
    def test_is_android_15_compatible(self):
        """Test is_android_15_compatible."""
        # This test will depend on environment, so just check it doesn't crash
        result = is_android_15_compatible()
        self.assertIsInstance(result, bool)
    
    def test_is_rmx3834_compatible(self):
        """Test is_rmx3834_compatible."""
        result = is_rmx3834_compatible()
        self.assertTrue(result)


if __name__ == '__main__':
    unittest.main()
