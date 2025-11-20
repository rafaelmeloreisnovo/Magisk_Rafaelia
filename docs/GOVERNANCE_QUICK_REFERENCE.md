# Governance Quick Reference Card

## 🚀 Quick Start

```bash
# Check compliance before committing
python3 tools/governance_compliance_checker.py

# Generate detailed report
python3 tools/governance_compliance_checker.py --report report.json

# Check specific module
python3 tools/governance_compliance_checker.py --root ./mymodule
```

## 📋 Required Standards

| Standard | What to Check |
|----------|---------------|
| **ISO 27001** | Security controls, access management |
| **ISO 9126** | Code quality, functionality, reliability |
| **IEEE 730** | Documentation, reviews, QA process |
| **IEEE 828** | Version control, change management |
| **NIST 800-53** | Input validation, error handling, logging |
| **W3C** | Documentation format, accessibility |

## ✅ Pre-Commit Checklist

- [ ] All files have proper headers (docstrings/comments)
- [ ] No hardcoded secrets or credentials
- [ ] Error handling in place (`set -euo pipefail` for bash)
- [ ] Input validation for all user inputs
- [ ] Tests added/updated (80%+ coverage)
- [ ] Documentation updated
- [ ] Compliance check passes (>75%)

## 🐍 Python Template

```python
#!/usr/bin/env python3
"""
Module description.

Standards: ISO 9126, IEEE 730, NIST SP 800-53
Author: Name
Date: YYYY-MM-DD
"""

def function_name(param: str) -> dict:
    """
    Function description.
    
    Args:
        param: Parameter description
        
    Returns:
        dict: Return value description
        
    Raises:
        ValueError: When invalid input
        
    Security:
        - Input validation (NIST)
        - Output sanitization
    """
    # Validate input
    if not param:
        raise ValueError("param required")
    
    # Process
    result = process(param)
    
    return result
```

## 🔧 Shell Script Template

```bash
#!/usr/bin/env bash
#
# Script: script_name.sh
# Description: What it does
# Author: Name
# Date: YYYY-MM-DD
#
# Standards: IEEE 730, NIST SP 800-53
#
# Usage: ./script_name.sh [options]
#
# Exit Codes:
#   0 - Success
#   1 - Error

# Strict error handling (REQUIRED)
set -euo pipefail

# Error trap
trap 'echo "Error on line $LINENO"; exit 1' ERR

# Main function
main() {
    # Validate inputs
    if [[ $# -lt 1 ]]; then
        echo "Usage: $0 <argument>" >&2
        exit 1
    fi
    
    # Process
    echo "Processing..."
}

main "$@"
```

## 🔒 Security Rules

### ✅ DO
- Validate ALL inputs
- Use parameterized queries
- Sanitize outputs
- Log security events
- Use environment variables for secrets
- Implement least privilege
- Handle errors gracefully

### ❌ DON'T
- Hardcode secrets/passwords
- Use `eval` without validation
- Trust user input
- Expose stack traces
- Use weak crypto (MD5, SHA1)
- Skip input validation
- Ignore security warnings

## 🧪 Testing Requirements

```python
# Minimum 80% coverage
import unittest

class TestExample(unittest.TestCase):
    """Tests for example module (IEEE 829)."""
    
    def test_valid_input(self):
        """Test with valid input."""
        result = process("valid")
        self.assertEqual(result, expected)
    
    def test_invalid_input(self):
        """Test error handling (NIST)."""
        with self.assertRaises(ValueError):
            process("")
```

## 📝 Documentation Musts

### Code Comments
```python
# ✅ GOOD: Explains why
# Use SHA3 for NIST compliance
hash = sha3_256(data)

# ❌ BAD: Obvious
# Hash the data
hash = sha3_256(data)
```

### Docstrings
- Required for all public functions/classes
- Include: description, args, returns, raises
- Note security considerations
- Provide examples

## 🎯 Compliance Thresholds

- **90%+**: Excellent ✨
- **75-89%**: Good ✅ (mergeable)
- **60-74%**: Acceptable ⚠️
- **<60%**: Poor ❌ (blocked)

## 🔄 PR Process

1. **Before PR**
   ```bash
   # Run checks
   python3 tools/governance_compliance_checker.py
   pytest tests/ --cov
   python3 build.py all
   ```

2. **Create PR**
   - Use PR template
   - Fill all compliance checkboxes
   - Link related issues

3. **After PR**
   - Automated compliance check runs
   - Address reviewer feedback
   - Must get 2+ approvals

## 🆘 Common Fixes

### Missing Docstring
```python
# Before
def foo():
    return "bar"

# After
def foo() -> str:
    """Return bar string."""
    return "bar"
```

### Missing Shebang
```bash
# Before
echo "hello"

# After
#!/usr/bin/env bash
echo "hello"
```

### No Error Handling
```bash
# Before
rm -rf "$dir"

# After
set -euo pipefail
if [[ -z "${dir:-}" ]]; then
    echo "Error: dir not set" >&2
    exit 1
fi
rm -rf "$dir"
```

### Hardcoded Secret
```python
# Before
key = "secret123"

# After
import os
key = os.environ.get('API_KEY')
if not key:
    raise ValueError("API_KEY not set")
```

## 📚 Key Documents

- [GOVERNANCE.md](../GOVERNANCE.md) - Full framework
- [GOVERNANCE_AUDIT.md](../GOVERNANCE_AUDIT.md) - Status
- [Developer Guide](GOVERNANCE_DEVELOPER_GUIDE.md) - Details
- [PR Template](../.github/PULL_REQUEST_TEMPLATE.md)

## 🏷️ Standards Badges

Add to your documentation:

```markdown
[![Governance](https://img.shields.io/badge/Governance-Active-brightgreen)](GOVERNANCE.md)
[![Standards](https://img.shields.io/badge/Standards-ISO%20|%20IEEE%20|%20NIST-blue)](GOVERNANCE.md)
```

## 💡 Pro Tips

1. Run compliance checker BEFORE committing
2. Fix violations incrementally
3. Document exceptions clearly
4. Ask for help when stuck
5. Keep standards in mind while coding (not after)

## 🔗 Quick Links

- Compliance Tool: `tools/governance_compliance_checker.py`
- CI Workflow: `.github/workflows/governance-compliance.yml`
- PR Template: `.github/PULL_REQUEST_TEMPLATE.md`

---

**Print this card and keep it handy while coding!**

**Signature:** RAFCODE-Φ-∆RafaelVerboΩ
