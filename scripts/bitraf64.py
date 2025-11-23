#!/usr/bin/env python3
"""
Bitraf64 Encoding and Verification Script
════════════════════════════════════════════════════════════════════════════════
RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
Selo: ΣΩΔΦBITRAF
════════════════════════════════════════════════════════════════════════════════

Bitraf64: Custom encoding system using 10-symbol alphabet for integrity verification.
Alphabet: {A, B, Δ, F, I, Ω, Φ, R, Σ, T}
Length: 64 characters
"""

import sys
import hashlib
import json
from typing import Optional, Tuple

# Bitraf64 alphabet (10 symbols)
BITRAF64_ALPHABET = ['A', 'B', 'Δ', 'F', 'I', 'Ω', 'Φ', 'R', 'Σ', 'T']
BITRAF64_LENGTH = 64


class Bitraf64:
    """Bitraf64 encoding and verification system."""
    
    def __init__(self):
        self.alphabet = BITRAF64_ALPHABET
        self.length = BITRAF64_LENGTH
    
    def encode_data(self, data: bytes) -> str:
        """
        Encode data into Bitraf64 format.
        
        Args:
            data: Binary data to encode
            
        Returns:
            64-character Bitraf64 string
        """
        # Generate hash of data
        hash_obj = hashlib.sha3_256(data)
        hash_bytes = hash_obj.digest()
        
        # Convert to Bitraf64 representation
        bitraf64 = []
        for i in range(self.length):
            # Use hash bytes cyclically
            byte_val = hash_bytes[i % len(hash_bytes)]
            # Map to alphabet using modulo
            symbol_idx = byte_val % len(self.alphabet)
            bitraf64.append(self.alphabet[symbol_idx])
        
        return ''.join(bitraf64)
    
    def encode_string(self, text: str) -> str:
        """
        Encode string into Bitraf64 format.
        
        Args:
            text: String to encode
            
        Returns:
            64-character Bitraf64 string
        """
        return self.encode_data(text.encode('utf-8'))
    
    def verify_format(self, bitraf64: str) -> Tuple[bool, str]:
        """
        Verify that a string is valid Bitraf64 format.
        
        Args:
            bitraf64: String to verify
            
        Returns:
            Tuple of (is_valid, message)
        """
        # Check length
        if len(bitraf64) != self.length:
            return False, f"Invalid length: {len(bitraf64)} (expected {self.length})"
        
        # Check all characters are in alphabet
        for i, char in enumerate(bitraf64):
            if char not in self.alphabet:
                return False, f"Invalid character '{char}' at position {i}"
        
        return True, "Valid Bitraf64 format"
    
    def generate_manifest_entry(self, data: bytes, description: str = "") -> dict:
        """
        Generate a manifest entry with Bitraf64 encoding.
        
        Args:
            data: Data to encode
            description: Optional description
            
        Returns:
            Dictionary with Bitraf64 and metadata
        """
        bitraf64 = self.encode_data(data)
        
        return {
            'bitraf64': bitraf64,
            'description': description,
            'sha3_256': hashlib.sha3_256(data).hexdigest(),
            'blake3': self._blake3_hash(data) if self._has_blake3() else None,
        }
    
    def _has_blake3(self) -> bool:
        """Check if blake3 is available."""
        try:
            import blake3
            return True
        except ImportError:
            return False
    
    def _blake3_hash(self, data: bytes) -> Optional[str]:
        """Generate blake3 hash if available."""
        try:
            import blake3
            return blake3.blake3(data).hexdigest()
        except ImportError:
            return None


def validate_manifest_bitraf64(manifest_path: str) -> bool:
    """
    Validate Bitraf64 in RAFAELIA_MANIFEST.json.
    
    Args:
        manifest_path: Path to manifest file
        
    Returns:
        True if valid, False otherwise
    """
    print("🔢 Validating Bitraf64 in manifest...\n")
    
    try:
        with open(manifest_path, 'r', encoding='utf-8') as f:
            manifest = json.load(f)
        
        if 'bitraf64' not in manifest:
            print("❌ No bitraf64 field in manifest")
            return False
        
        bitraf64_value = manifest['bitraf64']
        
        validator = Bitraf64()
        is_valid, message = validator.verify_format(bitraf64_value)
        
        if is_valid:
            print(f"✅ {message}")
            print(f"📋 Bitraf64: {bitraf64_value}")
            return True
        else:
            print(f"❌ {message}")
            return False
            
    except FileNotFoundError:
        print(f"❌ Manifest file not found: {manifest_path}")
        return False
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON in manifest: {e}")
        return False
    except Exception as e:
        print(f"❌ Error validating manifest: {e}")
        return False


def generate_bitraf64_example():
    """Generate example Bitraf64 encodings."""
    print("🔢 Generating Bitraf64 examples...\n")
    
    encoder = Bitraf64()
    
    examples = [
        "RAFAELIA Framework",
        "Rafael Melo Reis Novo",
        "RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ",
        "Magisk_Rafaelia Project",
    ]
    
    for text in examples:
        bitraf64 = encoder.encode_string(text)
        print(f"Text: {text}")
        print(f"Bitraf64: {bitraf64}")
        print()


def main():
    """Main entry point."""
    print("🚀 Bitraf64 Encoding & Verification System")
    print("RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ")
    print("Selo: ΣΩΔΦBITRAF\n")
    print("="*80 + "\n")
    
    if len(sys.argv) > 1:
        command = sys.argv[1]
        
        if command == 'validate':
            # Validate manifest
            manifest_path = sys.argv[2] if len(sys.argv) > 2 else 'RAFAELIA_MANIFEST.json'
            if validate_manifest_bitraf64(manifest_path):
                print("\n✅ Validation successful!")
                sys.exit(0)
            else:
                print("\n❌ Validation failed!")
                sys.exit(1)
        
        elif command == 'encode':
            # Encode a string
            if len(sys.argv) < 3:
                print("Usage: bitraf64.py encode <text>")
                sys.exit(1)
            
            text = ' '.join(sys.argv[2:])
            encoder = Bitraf64()
            bitraf64 = encoder.encode_string(text)
            
            print(f"Input: {text}")
            print(f"Bitraf64: {bitraf64}")
            
            is_valid, message = encoder.verify_format(bitraf64)
            print(f"Verification: {message}")
            sys.exit(0)
        
        elif command == 'verify':
            # Verify a Bitraf64 string
            if len(sys.argv) < 3:
                print("Usage: bitraf64.py verify <bitraf64_string>")
                sys.exit(1)
            
            bitraf64 = sys.argv[2]
            encoder = Bitraf64()
            is_valid, message = encoder.verify_format(bitraf64)
            
            print(f"Bitraf64: {bitraf64}")
            print(f"Result: {message}")
            
            if is_valid:
                sys.exit(0)
            else:
                sys.exit(1)
        
        elif command == 'examples':
            # Generate examples
            generate_bitraf64_example()
            sys.exit(0)
        
        else:
            print(f"Unknown command: {command}")
            print("\nAvailable commands:")
            print("  validate [manifest_path] - Validate Bitraf64 in manifest")
            print("  encode <text>            - Encode text to Bitraf64")
            print("  verify <bitraf64>        - Verify Bitraf64 format")
            print("  examples                 - Generate example encodings")
            sys.exit(1)
    
    else:
        # Default: show examples
        generate_bitraf64_example()
        
        # Try to validate manifest if it exists
        import os
        if os.path.exists('RAFAELIA_MANIFEST.json'):
            print("\n" + "="*80 + "\n")
            validate_manifest_bitraf64('RAFAELIA_MANIFEST.json')


if __name__ == '__main__':
    main()
