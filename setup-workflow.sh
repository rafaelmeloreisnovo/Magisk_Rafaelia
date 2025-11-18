#!/bin/bash
# Setup script for two-branch workflow
# This script helps set up the rascunho branch and provides instructions for branch protection

set -e

echo "=========================================="
echo "Magisk_Rafaelia Two-Branch Workflow Setup"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in a git repository
if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

echo -e "${BLUE}Step 1: Checking current branches${NC}"
echo ""

# Check if master branch exists
if ! git show-ref --verify --quiet refs/heads/master; then
    echo -e "${RED}Error: master branch doesn't exist${NC}"
    exit 1
fi

echo -e "${GREEN}✓ master branch exists${NC}"

# Check if rascunho branch exists locally
if git show-ref --verify --quiet refs/heads/rascunho; then
    echo -e "${GREEN}✓ rascunho branch exists locally${NC}"
else
    echo -e "${YELLOW}⚠ rascunho branch doesn't exist locally${NC}"
    
    # Check if it exists remotely
    if git ls-remote --heads origin rascunho | grep -q rascunho; then
        echo -e "${YELLOW}⚠ rascunho exists remotely, checking out...${NC}"
        git fetch origin rascunho
        git checkout -b rascunho origin/rascunho
        echo -e "${GREEN}✓ Checked out existing rascunho branch${NC}"
    else
        echo -e "${YELLOW}Creating rascunho branch from master...${NC}"
        git checkout master
        git pull origin master
        git checkout -b rascunho
        git push -u origin rascunho
        echo -e "${GREEN}✓ Created and pushed rascunho branch${NC}"
    fi
fi

echo ""
echo -e "${BLUE}Step 2: Verifying workflow files${NC}"
echo ""

# Check if workflow files exist
files_to_check=(
    "CONTRIBUTING.md"
    "WORKFLOW_GUIDE.md"
    "BRANCH_PROTECTION.md"
    "WORKFLOW_QUICK_REFERENCE.md"
    ".github/CODEOWNERS"
    ".github/pull_request_template.md"
    ".github/workflows/branch-workflow.yml"
)

all_exist=true
for file in "${files_to_check[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ $file${NC}"
    else
        echo -e "${RED}✗ $file (missing)${NC}"
        all_exist=false
    fi
done

if [ "$all_exist" = false ]; then
    echo ""
    echo -e "${RED}Some workflow files are missing. Please ensure all documentation is committed.${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Step 3: Branch Protection Setup Instructions${NC}"
echo ""
echo -e "${YELLOW}⚠ Branch protection must be configured manually on GitHub${NC}"
echo ""
echo "Please follow these steps:"
echo ""
echo "1. Go to: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:\/]\(.*\)\.git/\1/')/settings/branches"
echo ""
echo "2. Configure protection for 'master' branch:"
echo "   - Click 'Add rule'"
echo "   - Branch name pattern: master"
echo "   - ✅ Require a pull request before merging"
echo "   - ✅ Require approvals (at least 1)"
echo "   - ✅ Require status checks to pass"
echo "   - ✅ Require conversation resolution"
echo "   - ✅ Include administrators"
echo "   - ❌ Allow force pushes (DISABLED)"
echo "   - ❌ Allow deletions (DISABLED)"
echo ""
echo "3. Configure protection for 'rascunho' branch:"
echo "   - Click 'Add rule'"
echo "   - Branch name pattern: rascunho"
echo "   - ✅ Require a pull request before merging"
echo "   - ✅ Require approvals (at least 1)"
echo "   - ✅ Require status checks to pass"
echo "   - ✅ Include administrators"
echo "   - ❌ Allow force pushes (DISABLED)"
echo "   - ❌ Allow deletions (DISABLED)"
echo ""
echo "4. Set default branch to 'rascunho':"
echo "   - Go to Settings → General"
echo "   - Under 'Default branch', click the switch icon"
echo "   - Select 'rascunho'"
echo "   - Click 'Update' and confirm"
echo ""

echo -e "${BLUE}Step 4: Verification${NC}"
echo ""
echo "After setting up branch protection, verify with:"
echo ""
echo "  git checkout master"
echo "  echo 'test' > test.txt"
echo "  git add test.txt"
echo "  git commit -m 'test'"
echo "  git push origin master"
echo ""
echo "This should FAIL with a branch protection error."
echo ""

echo -e "${BLUE}Step 5: Documentation${NC}"
echo ""
echo "Share these guides with your team:"
echo ""
echo "  📖 CONTRIBUTING.md - How to contribute"
echo "  🔄 WORKFLOW_GUIDE.md - Detailed workflow"
echo "  🛡️ BRANCH_PROTECTION.md - Security settings"
echo "  ⚡ WORKFLOW_QUICK_REFERENCE.md - Quick reference"
echo ""

echo "=========================================="
echo -e "${GREEN}Setup Complete!${NC}"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  1. Configure branch protection on GitHub (see above)"
echo "  2. Set default branch to 'rascunho'"
echo "  3. Share documentation with contributors"
echo "  4. Start using the two-branch workflow!"
echo ""
echo "RAFCODE-Φ-∆RafaelVerboΩ"
