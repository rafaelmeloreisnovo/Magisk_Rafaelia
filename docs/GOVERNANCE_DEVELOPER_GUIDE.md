# Governance Compliance Guide for Developers

## Overview

This guide helps developers understand and comply with the Global Governance Framework implemented in Magisk_Rafaelia. All code contributions MUST meet the standards defined in [GOVERNANCE.md](../GOVERNANCE.md).

## Quick Reference

### Standards Summary

| Standard | Purpose | Key Requirements |
|----------|---------|------------------|
| **ISO/IEC 27001** | Information Security | Access control, encryption, security audits |
| **ISO/IEC 9126** | Software Quality | Functionality, reliability, usability, efficiency |
| **ISO/IEC 25010** | System Quality | Maintainability, portability, performance |
| **IEEE 730** | Quality Assurance | Reviews, audits, documentation |
| **IEEE 828** | Configuration Mgmt | Version control, change management |
| **IEEE 829** | Test Documentation | Test plans, cases, results |
| **IEEE 1012** | V&V | Verification and validation processes |
| **NIST SP 800-53** | Security Controls | Comprehensive security framework |
| **W3C** | Web Standards | Accessibility, internationalization |
| **ABNT NBR** | Brazilian Standards | Technical compliance for BR market |

## Before You Start Coding

### 1. Check Current Compliance

Run the compliance checker to understand current status:

```bash
python3 tools/governance_compliance_checker.py --root . --report report.json
```

### 2. Review Governance Documentation

- Read [GOVERNANCE.md](../GOVERNANCE.md) for complete standards
- Review [GOVERNANCE_AUDIT.md](../GOVERNANCE_AUDIT.md) for current status
- Check the [PR Template](.github/PULL_REQUEST_TEMPLATE.md) for requirements

## Writing Compliant Code

### Python Files

**Required Elements:**

1. **Module Docstring** (ISO 9126)
```python
#!/usr/bin/env python3
"""
Module Name - Brief Description

Longer description of what this module does, its purpose,
and how it fits into the larger system.

Standards Compliance:
- ISO/IEC 27001: Security controls implemented
- IEEE 730: Quality assurance applied
- NIST SP 800-53: Security best practices

Author: Your Name
Date: YYYY-MM-DD
Version: X.Y.Z
"""
```

2. **Function Documentation**
```python
def process_data(input_data: str, options: dict) -> dict:
    """
    Process input data according to specified options.
    
    Args:
        input_data: Raw input string to process
        options: Configuration dictionary with processing options
        
    Returns:
        dict: Processed results with status and data
        
    Raises:
        ValueError: If input_data is empty or invalid
        SecurityError: If input fails security validation
        
    Security:
        - Input validation performed (NIST SP 800-53)
        - Output sanitization applied
        
    Examples:
        >>> process_data("test", {"mode": "strict"})
        {'status': 'success', 'data': 'processed_test'}
    """
    # Implementation
```

3. **Security Best Practices** (NIST)
```python
# ✅ GOOD: Secure practices
def validate_input(user_input: str) -> str:
    """Validate and sanitize user input."""
    # Input validation
    if not user_input or len(user_input) > 1000:
        raise ValueError("Invalid input length")
    
    # Sanitization
    safe_input = re.sub(r'[^\w\s-]', '', user_input)
    
    return safe_input

# ❌ BAD: Security issues
def process_command(cmd: str):
    """Process shell command - UNSAFE!"""
    os.system(cmd)  # Command injection vulnerability!
    eval(cmd)       # Code execution vulnerability!
```

### Shell Scripts

**Required Elements:**

1. **Script Header** (IEEE 730)
```bash
#!/usr/bin/env bash
#
# Script Name: example_script.sh
# Description: Brief description of what this script does
# Author: Your Name
# Date: YYYY-MM-DD
# Version: X.Y.Z
#
# Standards Compliance:
# - IEEE 730: Quality assurance
# - NIST SP 800-53: Security controls
#
# Usage:
#   ./example_script.sh [options]
#
# Options:
#   -h, --help     Show this help message
#   -v, --verbose  Enable verbose output
#
# Exit Codes:
#   0 - Success
#   1 - General error
#   2 - Invalid arguments
```

2. **Error Handling** (NIST)
```bash
# ✅ REQUIRED: Strict error handling
set -euo pipefail

# Optional: Enable debugging
# set -x

# Error trap
trap 'echo "Error on line $LINENO"; exit 1' ERR
```

3. **Input Validation**
```bash
# ✅ GOOD: Validate inputs
validate_path() {
    local path="$1"
    
    # Check if path is absolute
    if [[ ! "$path" =~ ^/ ]]; then
        echo "Error: Path must be absolute" >&2
        return 1
    fi
    
    # Check for path traversal
    if [[ "$path" =~ \.\. ]]; then
        echo "Error: Path traversal detected" >&2
        return 1
    fi
    
    return 0
}

# ❌ BAD: No validation
dangerous_operation() {
    rm -rf "$1"  # No validation - could delete anything!
}
```

### GitHub Workflows

**Required Elements:**

1. **Permissions** (NIST SP 800-53)
```yaml
name: Example Workflow

on:
  push:
    branches: [ main, develop ]

# ✅ REQUIRED: Explicit permissions
permissions:
  contents: read
  pull-requests: write
  issues: write
```

2. **Timeouts** (IEEE 730)
```yaml
jobs:
  build:
    name: Build Project
    runs-on: ubuntu-latest
    timeout-minutes: 30  # ✅ REQUIRED: Prevent runaway jobs
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        timeout-minutes: 5  # ✅ GOOD: Step-level timeout
```

3. **Security** (NIST)
```yaml
# ✅ GOOD: Pin action versions
- uses: actions/checkout@v4  # Specific version

# ❌ BAD: Unpinned version
- uses: actions/checkout@main  # Security risk!

# ✅ GOOD: Use secrets properly
- name: Deploy
  env:
    API_KEY: ${{ secrets.API_KEY }}
    
# ❌ BAD: Exposed secret
- name: Deploy
  run: |
    echo "MY_SECRET_KEY=abc123" >> $GITHUB_ENV
```

## Testing Requirements

### Unit Tests (IEEE 829)

**Minimum Requirements:**
- 80% code coverage
- Test all public APIs
- Test error conditions
- Test edge cases

```python
import unittest

class TestDataProcessor(unittest.TestCase):
    """
    Test suite for data processor module.
    
    Standards: IEEE 829 - Software Test Documentation
    Coverage Target: 80%
    """
    
    def setUp(self):
        """Set up test fixtures."""
        self.processor = DataProcessor()
    
    def test_valid_input(self):
        """Test processing with valid input."""
        result = self.processor.process("valid data")
        self.assertEqual(result.status, "success")
    
    def test_invalid_input(self):
        """Test error handling with invalid input."""
        with self.assertRaises(ValueError):
            self.processor.process("")
    
    def test_security_validation(self):
        """Test security input validation (NIST)."""
        malicious = "<script>alert('xss')</script>"
        result = self.processor.process(malicious)
        self.assertNotIn("<script>", result.data)
```

### Integration Tests

```python
class TestIntegration(unittest.TestCase):
    """
    Integration tests for end-to-end workflows.
    
    Standards: IEEE 1012 - Verification and Validation
    """
    
    def test_complete_workflow(self):
        """Test complete processing workflow."""
        # Arrange
        input_data = create_test_data()
        
        # Act
        result = complete_workflow(input_data)
        
        # Assert
        self.assertTrue(result.success)
        self.assertIsNotNone(result.output)
```

## Documentation Requirements

### Code Comments (ISO 9126)

```python
# ✅ GOOD: Helpful comments
def calculate_checksum(data: bytes) -> str:
    """Calculate SHA3-256 checksum of data."""
    # Use SHA3 for NIST compliance
    hasher = hashlib.sha3_256()
    hasher.update(data)
    return hasher.hexdigest()

# ❌ BAD: Obvious or outdated comments
def add(a, b):
    # Add two numbers  # Obvious!
    return a + b

# ❌ BAD: Commented-out code
def process():
    # do_something()  # Remove instead of commenting!
    do_something_else()
```

### README Files (W3C)

Every module should have a README.md with:

```markdown
# Module Name

Brief description of the module.

## Features

- Feature 1
- Feature 2

## Installation

\`\`\`bash
# Installation steps
\`\`\`

## Usage

\`\`\`python
# Usage examples
\`\`\`

## Standards Compliance

- ISO/IEC 27001: Security controls
- IEEE 730: Quality assurance
- NIST SP 800-53: Security framework

## API Documentation

### function_name(param1, param2)

Description of function.

**Parameters:**
- `param1` (type): Description
- `param2` (type): Description

**Returns:**
- type: Description

**Raises:**
- Exception: When condition

## Testing

\`\`\`bash
# Run tests
python -m pytest tests/
\`\`\`

## License

See LICENSE file in root directory.
```

## Security Requirements (NIST SP 800-53)

### Required Security Controls

1. **Access Control (AC)**
   - Implement least privilege
   - Validate all inputs
   - Sanitize all outputs

2. **Audit and Accountability (AU)**
   - Log security-relevant events
   - Protect log integrity
   - Review logs regularly

3. **Identification and Authentication (IA)**
   - No hardcoded credentials
   - Use secure authentication methods
   - Implement session management

4. **System and Communications Protection (SC)**
   - Encrypt sensitive data
   - Use TLS 1.3+ for network communication
   - Validate certificates

5. **System and Information Integrity (SI)**
   - Input validation
   - Error handling
   - Security monitoring

### Security Checklist

- [ ] No hardcoded secrets or API keys
- [ ] All inputs validated and sanitized
- [ ] Proper error handling (no stack traces to users)
- [ ] Secure file operations (check paths, permissions)
- [ ] Network communication uses encryption
- [ ] Dependencies are up-to-date and scanned
- [ ] Security logging implemented
- [ ] Authentication/authorization implemented correctly

## Using the Compliance Checker

### Basic Usage

```bash
# Check compliance
python3 tools/governance_compliance_checker.py

# Generate detailed report
python3 tools/governance_compliance_checker.py --report report.json

# Check specific directory
python3 tools/governance_compliance_checker.py --root ./my-module
```

### Understanding Results

```
✓ Passed: Check meets standards
⚠ Warnings: Should be fixed but not blocking
✗ Violations: MUST be fixed before merge
```

### Compliance Rates

- **90%+**: Excellent ✅
- **75-89%**: Good ✅
- **60-74%**: Acceptable ⚠️
- **<60%**: Poor ❌ (blocks merge)

## Pull Request Process

### Before Submitting

1. **Run compliance checker**
   ```bash
   python3 tools/governance_compliance_checker.py --report report.json
   ```

2. **Fix violations**
   - Address all ❌ violations
   - Fix ⚠️ warnings if possible
   - Document any exceptions needed

3. **Run tests**
   ```bash
   python -m pytest tests/ --cov
   ```

4. **Build project**
   ```bash
   python3 build.py all
   ```

### PR Checklist

Use the [PR Template](.github/PULL_REQUEST_TEMPLATE.md) and ensure:

- [ ] All governance standards checkboxes completed
- [ ] Code quality requirements met
- [ ] Security validation passed
- [ ] Tests added/updated with 80%+ coverage
- [ ] Documentation updated
- [ ] Build succeeds
- [ ] Compliance check passes (>75%)

### During Review

1. **Automated checks**
   - Governance compliance workflow runs automatically
   - Report posted as PR comment
   - Must achieve 75%+ compliance

2. **Code review**
   - Minimum 2 reviewers required
   - Security review for security-related changes
   - Architecture review for major changes

3. **Address feedback**
   - Respond to all comments
   - Fix identified issues
   - Update documentation as needed

## Common Issues and Solutions

### Issue: Missing docstring

**Problem:**
```python
def my_function():
    return "hello"
```

**Solution:**
```python
def my_function() -> str:
    """
    Return a greeting message.
    
    Returns:
        str: Greeting message
    """
    return "hello"
```

### Issue: Missing shebang in script

**Problem:**
```bash
echo "Hello"
```

**Solution:**
```bash
#!/usr/bin/env bash
# Script description
echo "Hello"
```

### Issue: No error handling

**Problem:**
```bash
rm -rf "$directory"
```

**Solution:**
```bash
#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${directory:-}" ]]; then
    echo "Error: directory not set" >&2
    exit 1
fi

if [[ ! -d "$directory" ]]; then
    echo "Error: directory does not exist" >&2
    exit 1
fi

rm -rf "$directory"
```

### Issue: Hardcoded secret

**Problem:**
```python
api_key = "abc123secret"
```

**Solution:**
```python
import os

api_key = os.environ.get('API_KEY')
if not api_key:
    raise ValueError("API_KEY environment variable not set")
```

## Resources

### Documentation
- [GOVERNANCE.md](../GOVERNANCE.md) - Complete standards framework
- [GOVERNANCE_AUDIT.md](../GOVERNANCE_AUDIT.md) - Implementation status
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines

### Tools
- [Compliance Checker](../tools/governance_compliance_checker.py)
- [Governance Workflow](../.github/workflows/governance-compliance.yml)
- [PR Template](../.github/PULL_REQUEST_TEMPLATE.md)

### Standards References
- [ISO/IEC 27001](https://www.iso.org/standard/27001)
- [IEEE Software Engineering Standards](https://standards.ieee.org/)
- [NIST SP 800-53](https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final)
- [W3C Standards](https://www.w3.org/standards/)
- [ABNT Standards](https://www.abnt.org.br/)

## Getting Help

### Questions?

1. Check existing documentation
2. Review [GOVERNANCE.md](../GOVERNANCE.md)
3. Ask in pull request comments
4. Contact governance team

### Need an Exception?

If you believe a standard cannot be met:

1. Document the technical reason
2. Propose alternative mitigation
3. Request exception in PR description
4. Await governance team approval

---

**Remember:** Governance compliance ensures quality, security, and maintainability. These standards protect users and make the codebase better for everyone.

**Signature:** RAFCODE-Φ-∆RafaelVerboΩ
