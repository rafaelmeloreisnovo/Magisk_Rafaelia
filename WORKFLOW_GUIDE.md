# Guia de Fluxo de Trabalho / Workflow Guide

[🇧🇷 Português](#guia-em-português) | [🇬🇧 English](#english-guide)

---

## Guia em Português

### Estratégia de Branches: Apenas 2 Branches Principais

Este projeto implementa uma estratégia simplificada de **duas branches** para manter organização, rastreabilidade e melhores práticas.

### Estrutura de Branches

```
┌─────────────────────────────────────────────────────┐
│                    MASTER                           │
│  (Produção - Código Estável e Aprovado)            │
│  ✅ Testado  ✅ Revisado  ✅ Pronto                 │
└─────────────────────────────────────────────────────┘
                         ▲
                         │ PR (após aprovação)
                         │
┌─────────────────────────────────────────────────────┐
│                   RASCUNHO                          │
│  (Staging - Fila de Mudanças para Revisão)         │
│  🔄 Em teste  🔍 Em revisão  📋 Enfileirado         │
└─────────────────────────────────────────────────────┘
                         ▲
                         │ PR (de features)
                         │
┌─────────────────────────────────────────────────────┐
│              FEATURE BRANCHES                       │
│  (Temporárias - Desenvolvimento Individual)         │
│  feature/*, bugfix/*, docs/*                       │
└─────────────────────────────────────────────────────┘
```

### Fluxo Detalhado

#### 1. Desenvolvedor Cria Feature

```bash
# 1. Atualiza rascunho
git checkout rascunho
git pull origin rascunho

# 2. Cria branch de feature
git checkout -b feature/minha-funcionalidade

# 3. Desenvolve
# ... faz mudanças ...
git add .
git commit -m "feat: implementa funcionalidade X"

# 4. Push
git push origin feature/minha-funcionalidade
```

#### 2. Pull Request para Rascunho

- Abra PR de `feature/minha-funcionalidade` → `rascunho`
- Descreva o que foi feito
- Aguarde CI/CD passar
- Aguarde code review
- Merge após aprovação

#### 3. Mudanças Ficam em Rascunho

- Todas as features mergeadas ficam em `rascunho`
- Mantenedor pode:
  - Testar conjunto de mudanças
  - Verificar integração entre features
  - Preparar release notes
  - Decidir o que vai para produção

#### 4. Promoção para Master

Quando mudanças em `rascunho` estão prontas para produção:

```bash
# Mantenedor cria PR
git checkout master
git pull origin master
git checkout -b release/v1.x.x
git merge rascunho

# Resolve conflitos se necessário
# Atualiza versão
# Push e cria PR

git push origin release/v1.x.x
# PR: release/v1.x.x → master
```

### Regras e Políticas

#### ❌ NUNCA Faça:
- Commit direto em `master`
- Commit direto em `rascunho`
- PR de feature direto para `master`
- Force push em branches principais

#### ✅ SEMPRE Faça:
- Crie feature branch a partir de `rascunho`
- PR de feature para `rascunho`
- Aguarde aprovação antes de merge
- Delete feature branch após merge

### Benefícios

#### 🎯 Rastreabilidade
- Cada mudança passa por `rascunho` primeiro
- Histórico completo de quando e como algo entrou
- Fácil de reverter mudanças específicas

#### 🔍 Revisão em Camadas
1. **Primeira camada**: Feature → Rascunho
   - Code review individual
   - Testes unitários
   - CI/CD básico

2. **Segunda camada**: Rascunho → Master
   - Revisão de integração
   - Testes de sistema
   - Validação de release

#### 📋 Fila de Mudanças
- `rascunho` serve como fila
- Mantenedor vê todas as mudanças pendentes
- Decide o timing de cada release
- Não perde nenhuma contribuição

#### 🛡️ Proteção do Master
- `master` sempre estável
- Pode fazer release a qualquer momento
- Confiança para usuários finais

### Convenções de Nome de Branch

```
feature/nome-da-funcionalidade    # Nova funcionalidade
bugfix/descricao-do-bug           # Correção de bug
hotfix/descricao-critica          # Correção urgente
docs/descricao-da-doc             # Apenas documentação
refactor/descricao-refactor       # Refatoração
test/descricao-teste              # Adição de testes
release/v1.x.x                    # Branch de release
```

### Exemplo Prático

**Cenário**: Adicionar novo módulo RAFAELIA

```bash
# 1. Desenvolvedor A cria feature
git checkout rascunho
git pull origin rascunho
git checkout -b feature/rafaelia-module-x
# ... desenvolve ...
git push origin feature/rafaelia-module-x
# PR → rascunho (aprovado e merged)

# 2. Desenvolvedor B cria outra feature
git checkout rascunho
git pull origin rascunho  # pega mudanças do Dev A
git checkout -b feature/rafaelia-module-y
# ... desenvolve ...
git push origin feature/rafaelia-module-y
# PR → rascunho (aprovado e merged)

# 3. Mantenedor revisa rascunho
# - Testa modules X e Y juntos
# - Verifica integração
# - Prepara release notes
# - Cria PR rascunho → master

# 4. Release v1.2.0
# - Merge em master
# - Tag v1.2.0
# - APK gerado via CI/CD
# - Publicado para usuários
```

### Troubleshooting

#### Conflito ao Fazer Merge em Rascunho

```bash
git checkout feature/minha-feature
git fetch origin
git merge origin/rascunho
# Resolve conflitos
git add .
git commit -m "merge: resolve conflicts with rascunho"
git push origin feature/minha-feature
```

#### Feature Branch Desatualizada

```bash
git checkout feature/minha-feature
git fetch origin
git rebase origin/rascunho
# Ou se preferir merge:
git merge origin/rascunho
git push origin feature/minha-feature
```

---

## English Guide

### Branch Strategy: Only 2 Main Branches

This project implements a simplified **two-branch** strategy to maintain organization, traceability, and best practices.

### Branch Structure

```
┌─────────────────────────────────────────────────────┐
│                    MASTER                           │
│  (Production - Stable and Approved Code)           │
│  ✅ Tested  ✅ Reviewed  ✅ Ready                   │
└─────────────────────────────────────────────────────┘
                         ▲
                         │ PR (after approval)
                         │
┌─────────────────────────────────────────────────────┐
│                   RASCUNHO                          │
│  (Staging - Queue of Changes for Review)           │
│  🔄 Testing  🔍 Under review  📋 Queued             │
└─────────────────────────────────────────────────────┘
                         ▲
                         │ PR (from features)
                         │
┌─────────────────────────────────────────────────────┐
│              FEATURE BRANCHES                       │
│  (Temporary - Individual Development)               │
│  feature/*, bugfix/*, docs/*                       │
└─────────────────────────────────────────────────────┘
```

### Detailed Workflow

#### 1. Developer Creates Feature

```bash
# 1. Update rascunho
git checkout rascunho
git pull origin rascunho

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Develop
# ... make changes ...
git add .
git commit -m "feat: implement feature X"

# 4. Push
git push origin feature/my-feature
```

#### 2. Pull Request to Rascunho

- Open PR from `feature/my-feature` → `rascunho`
- Describe what was done
- Wait for CI/CD to pass
- Wait for code review
- Merge after approval

#### 3. Changes Stay in Rascunho

- All merged features stay in `rascunho`
- Maintainer can:
  - Test set of changes together
  - Verify integration between features
  - Prepare release notes
  - Decide what goes to production

#### 4. Promotion to Master

When changes in `rascunho` are ready for production:

```bash
# Maintainer creates PR
git checkout master
git pull origin master
git checkout -b release/v1.x.x
git merge rascunho

# Resolve conflicts if needed
# Update version
# Push and create PR

git push origin release/v1.x.x
# PR: release/v1.x.x → master
```

### Rules and Policies

#### ❌ NEVER Do:
- Direct commit to `master`
- Direct commit to `rascunho`
- PR from feature directly to `master`
- Force push to main branches

#### ✅ ALWAYS Do:
- Create feature branch from `rascunho`
- PR from feature to `rascunho`
- Wait for approval before merge
- Delete feature branch after merge

### Benefits

#### 🎯 Traceability
- Every change goes through `rascunho` first
- Complete history of when and how something entered
- Easy to revert specific changes

#### 🔍 Layered Review
1. **First layer**: Feature → Rascunho
   - Individual code review
   - Unit tests
   - Basic CI/CD

2. **Second layer**: Rascunho → Master
   - Integration review
   - System tests
   - Release validation

#### 📋 Change Queue
- `rascunho` serves as queue
- Maintainer sees all pending changes
- Decides timing of each release
- No contribution is lost

#### 🛡️ Master Protection
- `master` always stable
- Can release at any time
- Trust for end users

### Branch Naming Conventions

```
feature/feature-name              # New feature
bugfix/bug-description           # Bug fix
hotfix/critical-description      # Urgent fix
docs/doc-description             # Documentation only
refactor/refactor-description    # Refactoring
test/test-description            # Add tests
release/v1.x.x                   # Release branch
```

### Practical Example

**Scenario**: Add new RAFAELIA module

```bash
# 1. Developer A creates feature
git checkout rascunho
git pull origin rascunho
git checkout -b feature/rafaelia-module-x
# ... develop ...
git push origin feature/rafaelia-module-x
# PR → rascunho (approved and merged)

# 2. Developer B creates another feature
git checkout rascunho
git pull origin rascunho  # gets Dev A changes
git checkout -b feature/rafaelia-module-y
# ... develop ...
git push origin feature/rafaelia-module-y
# PR → rascunho (approved and merged)

# 3. Maintainer reviews rascunho
# - Tests modules X and Y together
# - Verifies integration
# - Prepares release notes
# - Creates PR rascunho → master

# 4. Release v1.2.0
# - Merge to master
# - Tag v1.2.0
# - APK generated via CI/CD
# - Published to users
```

### Troubleshooting

#### Conflict When Merging to Rascunho

```bash
git checkout feature/my-feature
git fetch origin
git merge origin/rascunho
# Resolve conflicts
git add .
git commit -m "merge: resolve conflicts with rascunho"
git push origin feature/my-feature
```

#### Outdated Feature Branch

```bash
git checkout feature/my-feature
git fetch origin
git rebase origin/rascunho
# Or if you prefer merge:
git merge origin/rascunho
git push origin feature/my-feature
```

---

## CI/CD Integration

The workflow includes automated checks:

- ✅ Build validation on all PRs
- ✅ Test execution
- ✅ Code quality checks
- ✅ Security scans (CodeQL)
- ✅ APK generation for approved changes

---

## Questions?

- 📖 Read [CONTRIBUTING.md](CONTRIBUTING.md)
- 📚 Check [README.MD](README.MD)
- 🐛 Report issues

**RAFCODE-Φ-∆RafaelVerboΩ**
