# Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
# Instituto Rafael - CientiEspiritual Philosophy
#
# This file is part of Magisk_Rafaelia.
#
# Magisk_Rafaelia is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

"""
RAFAELIA Android Compatibility Module

This module provides comprehensive Android version and device compatibility
checking to ensure proper operation across different Android versions and devices.

Features:
- Android API level detection and validation
- Kernel version compatibility checking
- Device-specific compatibility rules
- Feature availability checking per Android version
- Compatibility warnings and recommendations

Supports:
- Android 13 (API 33, Tiramisu) with kernel 5.15.178
- Android 14 (API 34, UpsideDownCake)
- Android 15 (API 35, VanillaIceCream)
- Device-specific handling (RMX3834, etc.)

Part of Magisk_Rafaelia RAFAELIA Framework
Philosophy: VAZIO → VERBO → CHEIO → RETRO (Ensuring compatibility across all Android devices)

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy
All Rights Reserved.
"""

import re
import logging
import platform
import subprocess
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from enum import Enum


class AndroidVersion(Enum):
    """Android version enumeration with API levels."""
    MARSHMALLOW = (23, "6.0", "Marshmallow")
    NOUGAT = (24, "7.0", "Nougat")
    NOUGAT_1 = (25, "7.1", "Nougat")
    OREO = (26, "8.0", "Oreo")
    OREO_1 = (27, "8.1", "Oreo")
    PIE = (28, "9.0", "Pie")
    Q = (29, "10", "Q")
    R = (30, "11", "R")
    S = (31, "12", "S")
    S_V2 = (32, "12L", "S_V2")
    TIRAMISU = (33, "13", "Tiramisu")
    UPSIDE_DOWN_CAKE = (34, "14", "UpsideDownCake")
    VANILLA_ICE_CREAM = (35, "15", "VanillaIceCream")
    
    def __init__(self, api_level: int, version: str, codename: str):
        self.api_level = api_level
        self.version = version
        self.codename = codename
    
    @classmethod
    def from_api_level(cls, api_level: int) -> Optional['AndroidVersion']:
        """Get AndroidVersion from API level."""
        for version in cls:
            if version.api_level == api_level:
                return version
        return None
    
    @classmethod
    def from_version(cls, version: str) -> Optional['AndroidVersion']:
        """Get AndroidVersion from version string."""
        for ver in cls:
            if ver.version == version:
                return ver
        return None


@dataclass
class KernelVersion:
    """Represents a Linux kernel version."""
    major: int
    minor: int
    patch: int
    suffix: str = ""
    
    @classmethod
    def parse(cls, version_str: str) -> 'KernelVersion':
        """
        Parse kernel version string.
        
        Examples:
            - "5.15.178"
            - "5.15.178-android13-8-gabf75819a85e-ab569"
        """
        # Extract base version
        match = re.match(r'^(\d+)\.(\d+)\.(\d+)(.*)$', version_str.strip())
        if not match:
            raise ValueError(f"Invalid kernel version: {version_str}")
        
        major, minor, patch, suffix = match.groups()
        return cls(
            major=int(major),
            minor=int(minor),
            patch=int(patch),
            suffix=suffix.lstrip('-')
        )
    
    def __str__(self) -> str:
        """String representation."""
        version = f"{self.major}.{self.minor}.{self.patch}"
        if self.suffix:
            version += f"-{self.suffix}"
        return version
    
    def __lt__(self, other: 'KernelVersion') -> bool:
        """Compare kernel versions."""
        if self.major != other.major:
            return self.major < other.major
        if self.minor != other.minor:
            return self.minor < other.minor
        return self.patch < other.patch
    
    def __eq__(self, other: 'KernelVersion') -> bool:
        """Check equality."""
        return (self.major == other.major and 
                self.minor == other.minor and 
                self.patch == other.patch)
    
    def __le__(self, other: 'KernelVersion') -> bool:
        return self < other or self == other


@dataclass
class DeviceInfo:
    """Device information."""
    model: str
    manufacturer: str
    device: str
    build_id: str
    android_version: Optional[AndroidVersion] = None
    kernel_version: Optional[KernelVersion] = None
    
    def is_realme(self) -> bool:
        """Check if device is a Realme device."""
        return self.manufacturer.lower() == "realme" or self.model.upper().startswith("RMX")
    
    def is_specific_model(self, model_prefix: str) -> bool:
        """Check if device matches specific model prefix."""
        return self.model.upper().startswith(model_prefix.upper())


@dataclass
class CompatibilityResult:
    """Result of compatibility check."""
    is_compatible: bool
    android_compatible: bool
    kernel_compatible: bool
    device_compatible: bool
    warnings: List[str]
    errors: List[str]
    recommendations: List[str]
    
    def to_dict(self) -> Dict:
        """Convert to dictionary."""
        return {
            'is_compatible': self.is_compatible,
            'android_compatible': self.android_compatible,
            'kernel_compatible': self.kernel_compatible,
            'device_compatible': self.device_compatible,
            'warnings': self.warnings,
            'errors': self.errors,
            'recommendations': self.recommendations
        }


class AndroidCompatibilityChecker:
    """
    Check Android version and device compatibility.
    
    Ensures that RAFAELIA components are compatible with the target
    Android version, kernel version, and device hardware.
    """
    
    # Minimum supported versions
    MIN_ANDROID_API = 23  # Android 6.0 Marshmallow
    MIN_KERNEL_VERSION = KernelVersion(4, 4, 0)
    
    # Known compatible devices
    KNOWN_DEVICES = {
        'RMX3834': {
            'name': 'Realme 12 Pro',
            'recommended_android': [33, 34, 35],
            'recommended_kernel': '5.15.178',
            'notes': 'Full RAFAELIA support'
        }
    }
    
    # Android version specific features and requirements
    ANDROID_FEATURES = {
        33: {  # Android 13
            'features': ['per_app_language', 'themed_icons', 'notification_runtime_permission'],
            'selinux_required': True,
            'kernel_min': '5.10',
        },
        34: {  # Android 14
            'features': ['ultra_hdr', 'regional_preferences', 'grammatical_inflection'],
            'selinux_required': True,
            'kernel_min': '5.15',
        },
        35: {  # Android 15
            'features': ['partial_screen_sharing', 'private_space', 'satellite_connectivity'],
            'selinux_required': True,
            'kernel_min': '5.15',
        }
    }
    
    def __init__(self):
        """Initialize the compatibility checker."""
        self.logger = logging.getLogger("RAFAELIA.AndroidCompatibility")
    
    def check_android_version(self, api_level: int) -> Tuple[bool, List[str], List[str]]:
        """
        Check if Android version is compatible.
        
        Returns:
            (is_compatible, warnings, errors, recommendations)
        """
        warnings = []
        errors = []
        
        if api_level < self.MIN_ANDROID_API:
            errors.append(f"Android API {api_level} is below minimum supported version {self.MIN_ANDROID_API}")
            return False, warnings, errors
        
        android_ver = AndroidVersion.from_api_level(api_level)
        if android_ver:
            self.logger.info(f"Detected Android {android_ver.version} ({android_ver.codename})")
            
            # Check for known issues with specific versions
            if api_level == 29:  # Android 10
                warnings.append("Android 10 has known SELinux policy issues, ensure proper configuration")
            elif api_level >= 35:  # Android 15+
                warnings.append("Android 15+ requires updated security policies")
        else:
            warnings.append(f"Unknown Android API level {api_level}, compatibility uncertain")
        
        return True, warnings, errors, recommendations
    
    def check_kernel_version(self, kernel_str: str, api_level: int) -> Tuple[bool, List[str], List[str]]:
        """
        Check if kernel version is compatible.
        
        Returns:
            (is_compatible, warnings, errors, recommendations)
        """
        warnings = []
        errors = []
        
        try:
            kernel = KernelVersion.parse(kernel_str)
            self.logger.info(f"Detected kernel {kernel}")
            
            # Check minimum kernel version
            if kernel < self.MIN_KERNEL_VERSION:
                errors.append(f"Kernel {kernel} is below minimum supported version {self.MIN_KERNEL_VERSION}")
                return False, warnings, errors
            
            # Check kernel version against Android API requirements
            features = self.ANDROID_FEATURES.get(api_level, {})
            min_kernel = features.get('kernel_min')
            if min_kernel:
                min_kernel_parsed = KernelVersion.parse(min_kernel + ".0")
                if kernel < min_kernel_parsed:
                    warnings.append(
                        f"Kernel {kernel} is below recommended {min_kernel} for Android API {api_level}"
                    )
            
            # Special handling for Android 13 kernel 5.15.178
            if api_level == 33 and kernel.major == 5 and kernel.minor == 15:
                if kernel.patch >= 178:
                    self.logger.info("Detected Android 13 with optimal kernel 5.15.178+")
                else:
                    warnings.append(f"Android 13 works best with kernel 5.15.178+ (current: {kernel})")
            
            # Check for GKI (Generic Kernel Image) support
            if 'android' in kernel.suffix.lower():
                self.logger.info("GKI kernel detected - enhanced compatibility")
            else:
                warnings.append("Non-GKI kernel - some features may have limited support")
            
            return True, warnings, errors
            
        except ValueError as e:
            errors.append(f"Could not parse kernel version '{kernel_str}': {e}")
            return False, warnings, errors
    
    def check_device_compatibility(self, device_info: DeviceInfo) -> Tuple[bool, List[str], List[str], List[str]]:
        """
        Check if device is compatible.
        
        Returns:
            (is_compatible, warnings, errors, recommendations)
        """
        warnings = []
        errors = []
        recommendations = []
        
        self.logger.info(f"Checking device: {device_info.manufacturer} {device_info.model}")
        
        # Check known devices
        model_key = device_info.model.upper()
        if model_key in self.KNOWN_DEVICES:
            device_data = self.KNOWN_DEVICES[model_key]
            self.logger.info(f"Known device: {device_data['name']}")
            
            # Check Android version
            if device_info.android_version:
                api_level = device_info.android_version.api_level
                if api_level in device_data['recommended_android']:
                    self.logger.info(f"Android version {api_level} is optimal for this device")
                else:
                    warnings.append(
                        f"Android API {api_level} is not in recommended list: "
                        f"{device_data['recommended_android']}"
                    )
            
            recommendations.append(device_data['notes'])
        
        # Realme-specific checks
        if device_info.is_realme():
            self.logger.info("Realme device detected")
            warnings.append("Realme devices may require ColorOS-specific adjustments")
            recommendations.append("Ensure ColorOS security features are properly configured")
        
        # Special handling for RMX3834
        if device_info.is_specific_model("RMX3834"):
            self.logger.info("RMX3834 detected - Realme 12 Pro")
            if device_info.android_version and device_info.android_version.api_level == 35:
                self.logger.info("RMX3834 on Android 15 - full support available")
            else:
                warnings.append("RMX3834 works best with Android 15")
        
        return True, warnings, errors, recommendations
    
    def check_full_compatibility(
        self,
        api_level: Optional[int] = None,
        kernel_version: Optional[str] = None,
        device_info: Optional[DeviceInfo] = None
    ) -> CompatibilityResult:
        """
        Perform comprehensive compatibility check.
        
        Args:
            api_level: Android API level
            kernel_version: Kernel version string
            device_info: Device information
            
        Returns:
            CompatibilityResult with detailed analysis
        """
        all_warnings = []
        all_errors = []
        all_recommendations = []
        
        android_compatible = True
        kernel_compatible = True
        device_compatible = True
        
        # Check Android version
        if api_level is not None:
            android_compatible, warnings, errors = self.check_android_version(api_level)
            all_warnings.extend(warnings)
            all_errors.extend(errors)
        else:
            all_warnings.append("Android API level not provided, skipping Android version check")
        
        # Check kernel version
        if kernel_version is not None and api_level is not None:
            kernel_compatible, warnings, errors = self.check_kernel_version(kernel_version, api_level)
            all_warnings.extend(warnings)
            all_errors.extend(errors)
        elif kernel_version is not None:
            all_warnings.append("Cannot validate kernel without Android API level")
        else:
            all_warnings.append("Kernel version not provided, skipping kernel check")
        
        # Check device compatibility
        if device_info is not None:
            device_compatible, warnings, errors, recommendations = self.check_device_compatibility(device_info)
            all_warnings.extend(warnings)
            all_errors.extend(errors)
            all_recommendations.extend(recommendations)
        else:
            all_warnings.append("Device info not provided, skipping device-specific checks")
        
        # Overall compatibility
        is_compatible = android_compatible and kernel_compatible and device_compatible
        
        # Add general recommendations
        if is_compatible:
            all_recommendations.append("System meets all compatibility requirements")
        else:
            all_recommendations.append("Address compatibility issues before proceeding")
        
        return CompatibilityResult(
            is_compatible=is_compatible,
            android_compatible=android_compatible,
            kernel_compatible=kernel_compatible,
            device_compatible=device_compatible,
            warnings=all_warnings,
            errors=all_errors,
            recommendations=all_recommendations
        )
    
    def get_system_info(self) -> Dict[str, str]:
        """
        Get current system information.
        
        Note: This will only work on Android devices. On other platforms,
        it returns placeholder data.
        """
        info = {
            'platform': platform.system(),
            'platform_release': platform.release(),
            'platform_version': platform.version(),
        }
        
        # Try to detect Android-specific info (when running on device)
        if platform.system() == 'Linux':
            # Try to read Android properties (if available)
            try:
                api_level = subprocess.check_output(
                    ['getprop', 'ro.build.version.sdk'],
                    stderr=subprocess.DEVNULL
                ).decode().strip()
                info['android_api_level'] = api_level
            except (subprocess.CalledProcessError, OSError, FileNotFoundError):
                pass
            
            try:
                model = subprocess.check_output(
                    ['getprop', 'ro.product.model'],
                    stderr=subprocess.DEVNULL
                ).decode().strip()
                info['device_model'] = model
            except (subprocess.CalledProcessError, OSError, FileNotFoundError):
                pass
            
            try:
                manufacturer = subprocess.check_output(
                    ['getprop', 'ro.product.manufacturer'],
                    stderr=subprocess.DEVNULL
                ).decode().strip()
                info['device_manufacturer'] = manufacturer
            except (subprocess.CalledProcessError, OSError, FileNotFoundError):
                pass
        
        return info


# Module-level convenience functions
def check_compatibility(
    api_level: Optional[int] = None,
    kernel_version: Optional[str] = None,
    device_model: Optional[str] = None,
    device_manufacturer: Optional[str] = None,
    build_id: Optional[str] = None
) -> CompatibilityResult:
    """
    Check Android compatibility (convenience function).
    
    Args:
        api_level: Android API level (e.g., 33 for Android 13)
        kernel_version: Kernel version string (e.g., "5.15.178-android13-8-gabf75819a85e-ab569")
        device_model: Device model (e.g., "RMX3834")
        device_manufacturer: Device manufacturer (e.g., "Realme")
        build_id: Build ID string
        
    Returns:
        CompatibilityResult with detailed analysis
    """
    checker = AndroidCompatibilityChecker()
    
    # Build device info if provided
    device_info = None
    if device_model or device_manufacturer:
        android_ver = AndroidVersion.from_api_level(api_level) if api_level else None
        kernel_ver = KernelVersion.parse(kernel_version) if kernel_version else None
        
        device_info = DeviceInfo(
            model=device_model or "Unknown",
            manufacturer=device_manufacturer or "Unknown",
            device=device_model or "Unknown",
            build_id=build_id or "Unknown",
            android_version=android_ver,
            kernel_version=kernel_ver
        )
    
    return checker.check_full_compatibility(api_level, kernel_version, device_info)


def is_android_15_compatible() -> bool:
    """Quick check if system is Android 15 compatible."""
    checker = AndroidCompatibilityChecker()
    result = checker.check_full_compatibility(api_level=35)
    return result.is_compatible


def is_rmx3834_compatible(api_level: int = 35, kernel: str = "5.15.178") -> bool:
    """Quick check if configuration is compatible with RMX3834."""
    result = check_compatibility(
        api_level=api_level,
        kernel_version=kernel,
        device_model="RMX3834",
        device_manufacturer="Realme"
    )
    return result.is_compatible


if __name__ == "__main__":
    # Example usage
    logging.basicConfig(level=logging.INFO)
    
    print("="*70)
    print("RAFAELIA Android Compatibility Checker")
    print("="*70)
    
    # Test case 1: Android 13 with kernel 5.15.178
    print("\n[Test 1] Android 13 with kernel 5.15.178-android13-8-gabf75819a85e-ab569")
    print("-"*70)
    result = check_compatibility(
        api_level=33,
        kernel_version="5.15.178-android13-8-gabf75819a85e-ab569"
    )
    print(f"Compatible: {result.is_compatible}")
    if result.warnings:
        print("Warnings:")
        for w in result.warnings:
            print(f"  ⚠️  {w}")
    if result.errors:
        print("Errors:")
        for e in result.errors:
            print(f"  ❌ {e}")
    
    # Test case 2: RMX3834 on Android 15
    print("\n[Test 2] RMX3834 (Realme 12 Pro) on Android 15")
    print("-"*70)
    result = check_compatibility(
        api_level=35,
        kernel_version="5.15.178",
        device_model="RMX3834",
        device_manufacturer="Realme",
        build_id="RMX3834export_15_F.94"
    )
    print(f"Compatible: {result.is_compatible}")
    print(f"Android Compatible: {result.android_compatible}")
    print(f"Kernel Compatible: {result.kernel_compatible}")
    print(f"Device Compatible: {result.device_compatible}")
    if result.warnings:
        print("Warnings:")
        for w in result.warnings:
            print(f"  ⚠️  {w}")
    if result.recommendations:
        print("Recommendations:")
        for r in result.recommendations:
            print(f"  💡 {r}")
    
    # Test case 3: Check convenience functions
    print("\n[Test 3] Quick compatibility checks")
    print("-"*70)
    print(f"Android 15 compatible: {is_android_15_compatible()}")
    print(f"RMX3834 compatible: {is_rmx3834_compatible()}")
