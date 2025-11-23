#!/usr/bin/env python3
"""
RAFAELIA Fullstack TT Suite - Setup Configuration

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved

See RAFAELIA_LICENSE.md for licensing terms.
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read the README
readme_path = Path(__file__).parent / "README.md"
long_description = readme_path.read_text(encoding="utf-8") if readme_path.exists() else ""

# Read version from __init__.py
version = "1.0.0"

setup(
    name="rafaelia",
    version=version,
    author="Rafael Melo Reis",
    author_email="202743211+rafaelmeloreisnovo@users.noreply.github.com",
    description="RAFAELIA Tensor Train Suite - Vector→Vector AI Architecture",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia",
    project_urls={
        "Bug Reports": "https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia/issues",
        "Source": "https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia",
        "Documentation": "https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia/tree/master/rafaelia",
    },
    packages=find_packages(exclude=["tests", "tests.*"]),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Science/Research",
        "Intended Audience :: Education",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Topic :: Scientific/Engineering :: Mathematics",
        "License :: Other/Proprietary License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
    install_requires=[
        "numpy>=1.20.0",
    ],
    extras_require={
        "gpu": ["cupy-cuda11x>=10.0.0"],
        "acceleration": ["numba>=0.55.0", "scipy>=1.7.0"],
        "hashing": ["blake3>=0.3.0"],
        "compression": ["zstandard>=0.18.0"],
        "web": ["flask>=2.0.0"],
        "all": [
            "cupy-cuda11x>=10.0.0",
            "numba>=0.55.0",
            "scipy>=1.7.0",
            "blake3>=0.3.0",
            "zstandard>=0.18.0",
            "flask>=2.0.0",
        ],
        "dev": [
            "pytest>=7.0.0",
            "pytest-cov>=3.0.0",
            "black>=22.0.0",
            "flake8>=4.0.0",
            "mypy>=0.950",
        ],
    },
    entry_points={
        "console_scripts": [
            "rafaelia-demo=rafaelia.integration.engine:main",
        ],
    },
    include_package_data=True,
    package_data={
        "rafaelia": [
            "*.md",
            "LICENSE",
        ],
    },
    zip_safe=False,
    keywords=[
        "tensor-train",
        "tensor-decomposition",
        "tt-cross",
        "machine-learning",
        "artificial-intelligence",
        "high-dimensional",
        "rafaelia",
        "vector-ai",
    ],
    license="Dual License - See RAFAELIA_LICENSE.md",
)
