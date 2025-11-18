# Configuração de Proteção de Branches / Branch Protection Settings

[🇧🇷 Português](#português) | [🇬🇧 English](#english)

---

## Português

### Visão Geral

Este documento descreve as configurações recomendadas de proteção de branches para manter a integridade do código e implementar o fluxo de trabalho de duas branches (master + rascunho).

### Configurações para Branch `master`

#### Configuração no GitHub

Navegue para: **Settings → Branches → Branch protection rules → Add rule**

**Branch name pattern:** `master`

#### Regras Obrigatórias (Required)

✅ **Require a pull request before merging**
- Require approvals: **1** (mínimo)
- Dismiss stale pull request approvals when new commits are pushed: **Enabled**
- Require review from Code Owners: **Enabled** (se houver CODEOWNERS)

✅ **Require status checks to pass before merging**
- Require branches to be up to date before merging: **Enabled**
- Status checks that are required:
  - `build` (se disponível)
  - `test` (se disponível)
  - `codeql` (análise de segurança)
  - `lint` (se disponível)

✅ **Require conversation resolution before merging**
- Enabled

✅ **Require signed commits**
- Opcional mas recomendado

✅ **Require linear history**
- Enabled (evita merge commits desnecessários)

✅ **Include administrators**
- Enabled (as regras se aplicam a todos, incluindo admins)

❌ **Allow force pushes**
- Disabled (NUNCA permitir force push em master)

❌ **Allow deletions**
- Disabled (não permitir deletar master)

### Configurações para Branch `rascunho`

**Branch name pattern:** `rascunho`

#### Regras Recomendadas

✅ **Require a pull request before merging**
- Require approvals: **1** (mínimo)
- Dismiss stale pull request approvals when new commits are pushed: **Enabled**

✅ **Require status checks to pass before merging**
- Require branches to be up to date before merging: **Enabled**
- Status checks that are required:
  - `build`
  - `test`

⚠️ **Require conversation resolution before merging**
- Opcional (pode ser mais flexível em rascunho)

⚠️ **Require linear history**
- Opcional (rascunho pode ter merge commits)

✅ **Include administrators**
- Enabled

❌ **Allow force pushes**
- Disabled (evitar perda de histórico)

❌ **Allow deletions**
- Disabled (não permitir deletar rascunho)

### Configuração via YAML (para referência)

Embora as configurações sejam feitas via interface do GitHub, aqui está uma representação em YAML para documentação:

```yaml
# .github/settings.yml (requer GitHub App "Settings" instalado)
branches:
  - name: master
    protection:
      required_pull_request_reviews:
        required_approving_review_count: 1
        dismiss_stale_reviews: true
        require_code_owner_reviews: true
      required_status_checks:
        strict: true
        contexts:
          - build
          - test
          - codeql
      enforce_admins: true
      required_linear_history: true
      allow_force_pushes: false
      allow_deletions: false
      required_conversation_resolution: true

  - name: rascunho
    protection:
      required_pull_request_reviews:
        required_approving_review_count: 1
        dismiss_stale_reviews: true
      required_status_checks:
        strict: true
        contexts:
          - build
          - test
      enforce_admins: true
      allow_force_pushes: false
      allow_deletions: false
```

### CODEOWNERS File

Crie um arquivo `.github/CODEOWNERS` para definir responsáveis por áreas do código:

```
# CODEOWNERS - Define code owners for automatic review requests

# Global owners (fallback)
* @rafaelmeloreisnovo

# RAFAELIA framework
/native/src/core/rafaelia_*.rs @rafaelmeloreisnovo
/tools/rafaelia/ @rafaelmeloreisnovo
/docs/RAFAELIA_*.md @rafaelmeloreisnovo

# Native code
/native/ @rafaelmeloreisnovo

# Build system
/build.py @rafaelmeloreisnovo
/.github/workflows/ @rafaelmeloreisnovo

# Documentation
/docs/ @rafaelmeloreisnovo
*.md @rafaelmeloreisnovo
```

### Passos para Configurar

#### 1. Criar Branch Rascunho

Se o branch `rascunho` não existe:

```bash
# No repositório local
git checkout master
git pull origin master
git checkout -b rascunho
git push origin rascunho
```

#### 2. Configurar Proteções no GitHub

1. Vá para **Settings** do repositório
2. Clique em **Branches** no menu lateral
3. Em **Branch protection rules**, clique **Add rule**
4. Configure `master` conforme acima
5. Clique **Create** ou **Save changes**
6. Repita para `rascunho`

#### 3. Definir Branch Padrão

1. Em **Settings → Branches**
2. Em **Default branch**, selecione `rascunho`
3. Clique **Update**
4. Confirme a mudança

**Motivo**: O branch padrão deve ser `rascunho` para que novos PRs sejam direcionados a ele automaticamente.

#### 4. Criar CODEOWNERS (opcional)

```bash
# Criar arquivo
mkdir -p .github
cat > .github/CODEOWNERS << 'EOF'
# CODEOWNERS
* @rafaelmeloreisnovo
EOF

git add .github/CODEOWNERS
git commit -m "docs: add CODEOWNERS file"
git push origin rascunho
```

### Verificação das Configurações

#### Testar Proteção do Master

```bash
# Isto DEVE falhar:
git checkout master
echo "test" > test.txt
git add test.txt
git commit -m "test"
git push origin master
# Erro esperado: "protected branch hook declined"
```

#### Testar Fluxo Correto

```bash
# Isto DEVE funcionar:
git checkout rascunho
git checkout -b feature/test
echo "test" > test.txt
git add test.txt
git commit -m "feat: add test file"
git push origin feature/test
# Agora abra PR para rascunho via interface do GitHub
```

### Monitoramento

#### Webhooks (opcional)

Configure webhooks para notificações:
- **Settings → Webhooks → Add webhook**
- Eventos: Pull requests, Pushes, Status checks

#### GitHub Actions

Workflows existentes já validam:
- ✅ Build em cada PR
- ✅ Testes em cada PR
- ✅ CodeQL security scan
- ✅ Geração de APK

### Exceções e Emergency Access

Em caso de emergência extrema (sistema comprometido, correção crítica):

1. **Desabilitar proteção temporariamente**
   - Settings → Branches → Edit rule
   - Desmarcar "Include administrators"
   - Fazer correção
   - Reabilitar imediatamente

2. **Documentar a exceção**
   - Criar issue explicando o motivo
   - Documentar mudanças feitas
   - Criar PR retroativo para rascunho

3. **Revisar processo**
   - Por que a emergência aconteceu?
   - Como prevenir no futuro?

---

## English

### Overview

This document describes recommended branch protection settings to maintain code integrity and implement the two-branch workflow (master + rascunho).

### Settings for `master` Branch

#### GitHub Configuration

Navigate to: **Settings → Branches → Branch protection rules → Add rule**

**Branch name pattern:** `master`

#### Required Rules

✅ **Require a pull request before merging**
- Require approvals: **1** (minimum)
- Dismiss stale pull request approvals when new commits are pushed: **Enabled**
- Require review from Code Owners: **Enabled** (if CODEOWNERS exists)

✅ **Require status checks to pass before merging**
- Require branches to be up to date before merging: **Enabled**
- Status checks that are required:
  - `build` (if available)
  - `test` (if available)
  - `codeql` (security analysis)
  - `lint` (if available)

✅ **Require conversation resolution before merging**
- Enabled

✅ **Require signed commits**
- Optional but recommended

✅ **Require linear history**
- Enabled (avoids unnecessary merge commits)

✅ **Include administrators**
- Enabled (rules apply to everyone, including admins)

❌ **Allow force pushes**
- Disabled (NEVER allow force push to master)

❌ **Allow deletions**
- Disabled (don't allow deleting master)

### Settings for `rascunho` Branch

**Branch name pattern:** `rascunho`

#### Recommended Rules

✅ **Require a pull request before merging**
- Require approvals: **1** (minimum)
- Dismiss stale pull request approvals when new commits are pushed: **Enabled**

✅ **Require status checks to pass before merging**
- Require branches to be up to date before merging: **Enabled**
- Status checks that are required:
  - `build`
  - `test`

⚠️ **Require conversation resolution before merging**
- Optional (can be more flexible in rascunho)

⚠️ **Require linear history**
- Optional (rascunho can have merge commits)

✅ **Include administrators**
- Enabled

❌ **Allow force pushes**
- Disabled (avoid losing history)

❌ **Allow deletions**
- Disabled (don't allow deleting rascunho)

### YAML Configuration (for reference)

While settings are done via GitHub interface, here's a YAML representation for documentation:

```yaml
# .github/settings.yml (requires "Settings" GitHub App installed)
branches:
  - name: master
    protection:
      required_pull_request_reviews:
        required_approving_review_count: 1
        dismiss_stale_reviews: true
        require_code_owner_reviews: true
      required_status_checks:
        strict: true
        contexts:
          - build
          - test
          - codeql
      enforce_admins: true
      required_linear_history: true
      allow_force_pushes: false
      allow_deletions: false
      required_conversation_resolution: true

  - name: rascunho
    protection:
      required_pull_request_reviews:
        required_approving_review_count: 1
        dismiss_stale_reviews: true
      required_status_checks:
        strict: true
        contexts:
          - build
          - test
      enforce_admins: true
      allow_force_pushes: false
      allow_deletions: false
```

### CODEOWNERS File

Create a `.github/CODEOWNERS` file to define code area owners:

```
# CODEOWNERS - Define code owners for automatic review requests

# Global owners (fallback)
* @rafaelmeloreisnovo

# RAFAELIA framework
/native/src/core/rafaelia_*.rs @rafaelmeloreisnovo
/tools/rafaelia/ @rafaelmeloreisnovo
/docs/RAFAELIA_*.md @rafaelmeloreisnovo

# Native code
/native/ @rafaelmeloreisnovo

# Build system
/build.py @rafaelmeloreisnovo
/.github/workflows/ @rafaelmeloreisnovo

# Documentation
/docs/ @rafaelmeloreisnovo
*.md @rafaelmeloreisnovo
```

### Setup Steps

#### 1. Create Rascunho Branch

If `rascunho` branch doesn't exist:

```bash
# In local repository
git checkout master
git pull origin master
git checkout -b rascunho
git push origin rascunho
```

#### 2. Configure Protections on GitHub

1. Go to repository **Settings**
2. Click **Branches** in sidebar
3. Under **Branch protection rules**, click **Add rule**
4. Configure `master` as above
5. Click **Create** or **Save changes**
6. Repeat for `rascunho`

#### 3. Set Default Branch

1. In **Settings → Branches**
2. Under **Default branch**, select `rascunho`
3. Click **Update**
4. Confirm the change

**Reason**: Default branch should be `rascunho` so new PRs are automatically targeted to it.

#### 4. Create CODEOWNERS (optional)

```bash
# Create file
mkdir -p .github
cat > .github/CODEOWNERS << 'EOF'
# CODEOWNERS
* @rafaelmeloreisnovo
EOF

git add .github/CODEOWNERS
git commit -m "docs: add CODEOWNERS file"
git push origin rascunho
```

### Verify Configuration

#### Test Master Protection

```bash
# This SHOULD fail:
git checkout master
echo "test" > test.txt
git add test.txt
git commit -m "test"
git push origin master
# Expected error: "protected branch hook declined"
```

#### Test Correct Flow

```bash
# This SHOULD work:
git checkout rascunho
git checkout -b feature/test
echo "test" > test.txt
git add test.txt
git commit -m "feat: add test file"
git push origin feature/test
# Now open PR to rascunho via GitHub interface
```

### Monitoring

#### Webhooks (optional)

Configure webhooks for notifications:
- **Settings → Webhooks → Add webhook**
- Events: Pull requests, Pushes, Status checks

#### GitHub Actions

Existing workflows already validate:
- ✅ Build on each PR
- ✅ Tests on each PR
- ✅ CodeQL security scan
- ✅ APK generation

### Exceptions and Emergency Access

In case of extreme emergency (compromised system, critical fix):

1. **Temporarily disable protection**
   - Settings → Branches → Edit rule
   - Uncheck "Include administrators"
   - Make correction
   - Re-enable immediately

2. **Document the exception**
   - Create issue explaining reason
   - Document changes made
   - Create retroactive PR to rascunho

3. **Review process**
   - Why did the emergency happen?
   - How to prevent in the future?

---

## Additional Resources

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [CONTRIBUTING.md](../CONTRIBUTING.md)
- [WORKFLOW_GUIDE.md](../WORKFLOW_GUIDE.md)

**RAFCODE-Φ-∆RafaelVerboΩ**
