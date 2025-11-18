# Quick Reference - Two-Branch Workflow

## 🇧🇷 Português

### Fluxo Básico

```bash
# 1. Começar nova feature
git checkout rascunho
git pull origin rascunho
git checkout -b feature/minha-funcionalidade

# 2. Desenvolver
git add .
git commit -m "feat: adiciona funcionalidade X"

# 3. Push e PR
git push origin feature/minha-funcionalidade
# Abrir PR para 'rascunho' no GitHub
```

### Estrutura de Branches

- **master** = Produção (🔒 protegido)
- **rascunho** = Staging (🔒 protegido)
- **feature/*** = Desenvolvimento (temporário)

### Regras Rápidas

✅ FAZER:
- Feature → rascunho (via PR)
- Rascunho → master (via PR)

❌ NÃO FAZER:
- Feature → master (pular rascunho)
- Commit direto em master
- Commit direto em rascunho

### Prefixos de Branch

- `feature/` - Nova funcionalidade
- `bugfix/` - Correção de bug
- `hotfix/` - Correção urgente
- `docs/` - Documentação
- `refactor/` - Refatoração
- `test/` - Testes

### Mensagens de Commit

- `feat:` - Nova funcionalidade
- `fix:` - Correção
- `docs:` - Documentação
- `refactor:` - Refatoração
- `test:` - Testes
- `chore:` - Manutenção

---

## 🇬🇧 English

### Basic Flow

```bash
# 1. Start new feature
git checkout rascunho
git pull origin rascunho
git checkout -b feature/my-feature

# 2. Develop
git add .
git commit -m "feat: add feature X"

# 3. Push and PR
git push origin feature/my-feature
# Open PR to 'rascunho' on GitHub
```

### Branch Structure

- **master** = Production (🔒 protected)
- **rascunho** = Staging (🔒 protected)
- **feature/*** = Development (temporary)

### Quick Rules

✅ DO:
- Feature → rascunho (via PR)
- Rascunho → master (via PR)

❌ DON'T:
- Feature → master (skip rascunho)
- Direct commit to master
- Direct commit to rascunho

### Branch Prefixes

- `feature/` - New feature
- `bugfix/` - Bug fix
- `hotfix/` - Urgent fix
- `docs/` - Documentation
- `refactor/` - Refactoring
- `test/` - Tests

### Commit Messages

- `feat:` - New feature
- `fix:` - Fix
- `docs:` - Documentation
- `refactor:` - Refactoring
- `test:` - Tests
- `chore:` - Maintenance

---

## Visual Diagram

```
┌─────────────────────────────────────────────┐
│                  MASTER                     │
│         (Production - Stable)               │
│    Only accepts PRs from rascunho          │
└─────────────────────────────────────────────┘
                    ▲
                    │
              PR Review & Merge
                    │
┌─────────────────────────────────────────────┐
│                RASCUNHO                     │
│        (Staging - Testing Queue)            │
│   Accepts PRs from feature branches         │
└─────────────────────────────────────────────┘
                    ▲
                    │
             PR Review & Merge
                    │
┌─────────────────────────────────────────────┐
│            FEATURE BRANCHES                 │
│                                             │
│  feature/*, bugfix/*, docs/*, etc.         │
│  (Temporary - Delete after merge)          │
└─────────────────────────────────────────────┘
```

---

## Documentation

📖 **Full Guides:**
- [CONTRIBUTING.md](CONTRIBUTING.md) - Complete guide
- [WORKFLOW_GUIDE.md](WORKFLOW_GUIDE.md) - Detailed workflow
- [BRANCH_PROTECTION.md](BRANCH_PROTECTION.md) - Protection settings

🔗 **Links:**
- [README.MD](README.MD) - Project overview
- [Build Guide](docs/build.md) - How to build

---

**RAFCODE-Φ-∆RafaelVerboΩ**
