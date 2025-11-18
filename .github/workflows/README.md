# GitHub Workflows

This directory contains GitHub Actions workflows that automate various aspects of the Magisk_Rafaelia project.

## Branch Workflow Assistant

**File:** `branch-workflow.yml`

### Purpose

Enforces and assists with the two-branch workflow strategy (master + rascunho).

### What It Does

1. **Validates PR Target Branch**
   - Prevents feature branches from targeting `master` directly
   - Ensures features go to `rascunho` first
   - Allows `rascunho` → `master` promotions
   - Allows `release/*` → `master` for releases

2. **Adds Helpful Comments**
   - Welcomes first-time contributors
   - Explains the workflow if PR targets wrong branch
   - Provides links to documentation
   - Gives checklist for proper PRs

3. **Checks Branch Naming**
   - Validates branch names follow conventions
   - Suggests proper prefixes (feature/, bugfix/, etc.)
   - Warns but doesn't fail on non-conventional names

### Workflow Triggers

```yaml
on:
  pull_request:
    types: [opened, edited, reopened]
    branches:
      - master
      - rascunho
```

### Jobs

#### 1. check-target-branch
- Validates that PRs follow the workflow
- Fails if feature branch targets master directly
- Passes if PR follows correct flow

#### 2. add-workflow-comment
- Automatically comments on new PRs
- Guides contributors to correct workflow
- Only runs when PR is first opened

#### 3. check-branch-name
- Validates branch naming conventions
- Suggests improvements
- Doesn't fail (warning only)

### Expected Branch Patterns

✅ **Allowed:**
```
feature/my-feature    → rascunho
bugfix/fix-issue      → rascunho
docs/update-readme    → rascunho
rascunho              → master
release/v1.2.3        → master
```

❌ **Blocked:**
```
feature/my-feature    → master (should go to rascunho)
bugfix/fix-issue      → master (should go to rascunho)
```

### Error Messages

When a PR incorrectly targets `master`:

```
❌ ERROR: Feature branches should target 'rascunho', not 'master'

📖 Our workflow requires:
  1. Feature branches → rascunho (via PR)
  2. rascunho → master (via PR, after review)

Please change the target branch of this PR to 'rascunho'
```

### Comment Examples

**For incorrect workflow:**
```markdown
👋 Hello @username! Thank you for your contribution!

❌ **This PR targets `master` directly, which doesn't follow our workflow.**

## 🔄 Our Two-Branch Workflow

feature branch → rascunho → master
     (PR)          (PR)

### Please change this PR's target branch to `rascunho`
```

**For correct workflow:**
```markdown
👋 Hello @username! Thank you for your contribution!

✅ **This PR correctly targets `rascunho` - our staging branch.**

### Next Steps:
1. ✅ CI/CD checks will run automatically
2. 🔍 Code review will be performed
...
```

## Other Workflows

### build.yml
- Builds APK for all platforms
- Runs on push to master and rascunho
- Generates artifacts
- Activates RAFAELIA framework

### ci.yml
- Continuous integration checks
- Builds native code
- Runs Android builds
- Tests compilation

### codeql.yml
- Security scanning
- Analyzes code for vulnerabilities
- Runs on PRs and scheduled

## Integration with Two-Branch Workflow

All workflows respect the two-branch strategy:

1. **On Feature Branches:**
   - Basic CI runs (build, test)
   - No APK artifacts generated
   - Fast feedback for developers

2. **On Rascunho:**
   - Full CI/CD pipeline
   - APK artifacts generated
   - Security scanning
   - Ready for integration testing

3. **On Master:**
   - Full release pipeline
   - Production APK generation
   - Full test suite
   - Release artifacts

## Customization

To modify the workflow behavior, edit `branch-workflow.yml`:

```yaml
# Change required branch patterns
if [[ "${PR_BASE}" == "master" ]] && [[ "${PR_HEAD}" != "rascunho"* ]]

# Modify comment messages
body: `Your custom message here`

# Add additional checks
run: |
  echo "Your custom validation"
```

## Testing the Workflow

### Test locally:
```bash
# This should be blocked by workflow:
git checkout -b feature/test
git push origin feature/test
# Open PR to master → should get error message

# This should pass:
git checkout -b feature/test2
git push origin feature/test2
# Open PR to rascunho → should get welcome message
```

### Check workflow runs:
1. Go to repository Actions tab
2. Find "Branch Workflow Assistant"
3. Check run logs for validation results

## Documentation Links

- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Contribution guide
- [WORKFLOW_GUIDE.md](../../WORKFLOW_GUIDE.md) - Detailed workflow
- [BRANCH_PROTECTION.md](../../BRANCH_PROTECTION.md) - Protection settings

## Troubleshooting

### Workflow not running?
- Check that PR targets `master` or `rascunho`
- Verify workflow file is in `.github/workflows/`
- Check GitHub Actions are enabled for repo

### Comments not being added?
- Verify `pull-requests: write` permission is set
- Check if GitHub Actions has access to add comments
- Review workflow logs for errors

### Branch validation failing?
- Read the error message carefully
- Check your PR target branch
- Ensure you're following the two-branch workflow

---

**RAFCODE-Φ-∆RafaelVerboΩ**
