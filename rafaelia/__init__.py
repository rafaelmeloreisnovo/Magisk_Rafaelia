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
RAFAELIA Fullstack TT Suite - Main Package

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved

This package provides a comprehensive Tensor Train (TT) decomposition suite
with cross-approximation, local updates, RAFAELIA manifest integration,
and ZIPRAF_OMEGA v999 governance framework.

Licensed under Dual License:
- Free for social inclusion (education, research, non-profit)
- Commercial use requires paid SaaS subscription

See RAFAELIA_LICENSE.md for complete terms.

Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
Philosophy: VAZIO → VERBO → CHEIO → RETRO
Motto: Haja Lux, Haja Etica
"""

from __future__ import annotations

import importlib
import importlib.util
from typing import Dict, Tuple

__version__ = "1.0.0"
__author__ = "Rafael Melo Reis (rafaelmeloreisnovo)"
__copyright__ = "Copyright (C) 2025 Rafael Melo Reis"
__license__ = "Dual License - See RAFAELIA_LICENSE.md"
__institution__ = "Instituto Rafael"
__framework__ = "ESTADO FRACTAL HAJA & ZIPRAF_OMEGA v999"
__philosophy__ = "CientiEspiritual"

_LAZY_EXPORTS: Dict[str, Tuple[str, str]] = {
    # Core algorithms
    "TTCrossApproximation": ("rafaelia.core.tt_cross", "TTCrossApproximation"),
    "TTLocalUpdate": ("rafaelia.core.tt_update", "TTLocalUpdate"),
    # Utilities
    "FibonacciSpiral": ("rafaelia.utils.spiral", "FibonacciSpiral"),
    "GoldenRatioSampler": ("rafaelia.utils.spiral", "GoldenRatioSampler"),
    "TTAccelerator": ("rafaelia.utils.acceleration", "TTAccelerator"),
    # Integration
    "RAFAELIAEngine": ("rafaelia.integration.engine", "RAFAELIAEngine"),
    # Governance
    "governance": ("rafaelia.governance", ""),
}

_OPTIONAL_IMPORT_HINTS: Dict[str, str] = {
    "numpy": "Install the RAFAELIA numerical dependencies before using TT algorithms.",
}


def _missing_optional_dependency(module_name: str) -> str:
    for dependency, hint in _OPTIONAL_IMPORT_HINTS.items():
        if importlib.util.find_spec(dependency) is None:
            return f"Optional dependency '{dependency}' is unavailable. {hint}"
    return f"Unable to import optional RAFAELIA module '{module_name}'."


def __getattr__(name: str):
    """Lazy-load heavy RAFAELIA exports without breaking lightweight imports."""

    if name not in _LAZY_EXPORTS:
        raise AttributeError(f"module 'rafaelia' has no attribute {name!r}")

    module_name, attribute_name = _LAZY_EXPORTS[name]
    if importlib.util.find_spec(module_name) is None:
        raise ImportError(_missing_optional_dependency(module_name))
    module = importlib.import_module(module_name)
    value = module if not attribute_name else getattr(module, attribute_name)
    globals()[name] = value
    return value


__governance_available__ = importlib.util.find_spec("rafaelia.governance") is not None

__all__ = [
    # Core algorithms
    "TTCrossApproximation",
    "TTLocalUpdate",
    # Utilities
    "FibonacciSpiral",
    "GoldenRatioSampler",
    "TTAccelerator",
    # Integration
    "RAFAELIAEngine",
    # Governance
    "governance",
    # Metadata
    "__version__",
    "__author__",
    "__copyright__",
    "__license__",
    "__institution__",
    "__framework__",
    "__philosophy__",
    "__governance_available__",
]
