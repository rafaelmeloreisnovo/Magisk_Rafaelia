# Estratégia de Branches Simplificada
# Simplified Branching Strategy

**Version:** 2.0.0  
**Date:** 2025-11-18  
**Model:** Main + Dev + Feature

---

## 🎯 Filosofia / Philosophy

**PORTUGUÊS:**
Estratégia simples e prática de dois branches principais com features temporárias.
Foco em clareza, facilidade de uso e redução de complexidade cognitiva.

**ENGLISH:**
Simple and practical two-branch strategy with temporary features.
Focus on clarity, ease of use, and cognitive load reduction.

**Princípio RAFAELIA:** VAZIO → VERBO → CHEIO → RETRO

---

## 🌳 Estrutura de Branches / Branch Structure

```
┌─────────────────────────────────────────────┐
│  main (Produção Estável / Stable Production) │
└──────────────┬──────────────────────────────┘
               │
               │ ← Merge quando estável / Merge when stable
               │
┌──────────────▼──────────────────────────────┐
│  develop (Desenvolvimento / Development)     │
└──────────────┬──────────────────────────────┘
               │
               │ ← PRs de features
               │
         ┌─────┴─────┬─────────┬─────────┐
         │           │         │         │
    feature/A   feature/B  feature/C  hotfix/X
    (temporário)
```

---

## 📦 Branches Principais / Main Branches

### 1. `main` - Produção / Production

**Propósito / Purpose:**
- Código estável e pronto para uso
- Stable, production-ready code

**Regras / Rules:**
- ✅ Sempre deve compilar e funcionar / Always must build and work
- ✅ Todos os testes passando / All tests passing
- ✅ Documentação atualizada / Updated documentation
- ✅ Security checks passed
- ❌ **Nunca fazer push direto / Never push directly**
- ❌ **Só merge via Pull Request / Only merge via PR**

**Proteções / Protections:**
- Requer PR aprovado / Requires approved PR
- Requer CI passar / Requires CI to pass
- Requer review de código / Requires code review
- Push direto bloqueado / Direct push blocked

**Tags / Releases:**
- Toda release é tagueada / Every release is tagged
- Formato: `v27.X-rafaelia`
- Exemplo: `v27.0-rafaelia`, `v27.1-rafaelia`

---

### 2. `develop` - Desenvolvimento / Development

**Propósito / Purpose:**
- Integração de novas features
- Integration of new features
- Branch de trabalho diário / Daily work branch

**Regras / Rules:**
- ✅ Deve compilar (pode ter bugs menores) / Must build (minor bugs OK)
- ✅ CI deve passar / CI must pass
- ✅ Testes importantes passando / Important tests passing
- ✅ Aceita commits diretos de manutenção / Accepts direct maintenance commits
- ✅ Features vêm via PR / Features come via PR

**Características / Characteristics:**
- Branch mais ativo / Most active branch
- Iteração rápida / Fast iteration
- Experimentos permitidos / Experiments allowed
- Base para features / Base for features

---

## 🔧 Branches Temporárias / Temporary Branches

### 3. `feature/*` - Features

**Propósito / Purpose:**
- Desenvolvimento de nova funcionalidade
- Development of new functionality

**Convenção de Nome / Naming Convention:**
```
feature/nome-descritivo
feature/issue-123-adicionar-login
feature/melhorar-performance-boot
```

**Ciclo de Vida / Lifecycle:**
1. Criar a partir de `develop` / Create from `develop`
2. Desenvolver feature / Develop feature
3. Abrir PR para `develop` / Open PR to `develop`
4. Review e merge / Review and merge
5. **Deletar branch / Delete branch**

**Regras / Rules:**
- ✅ Vida curta (máx 2 semanas) / Short-lived (max 2 weeks)
- ✅ Foco em uma feature / Focus on one feature
- ✅ Testes incluídos / Tests included
- ✅ Documentação atualizada / Documentation updated

---

### 4. `hotfix/*` - Correções Urgentes

**Propósito / Purpose:**
- Correção urgente em produção
- Urgent production fix

**Convenção de Nome / Naming Convention:**
```
hotfix/nome-problema
hotfix/corrigir-crash-boot
hotfix/issue-456-seguranca
```

**Ciclo de Vida / Lifecycle:**
1. Criar a partir de `main` / Create from `main`
2. Correção mínima / Minimal fix
3. Testes focados / Focused tests
4. PR para `main` (urgente) / PR to `main` (urgent)
5. Cherry-pick para `develop` / Cherry-pick to `develop`
6. **Deletar branch / Delete branch**

**Quando usar / When to use:**
- ⚠️ Bug crítico em produção / Critical bug in production
- ⚠️ Vulnerabilidade de segurança / Security vulnerability
- ⚠️ Sistema não funcionando / System not working

---

## 🔄 Fluxos de Trabalho / Workflows

### Workflow 1: Nova Feature

```bash
# 1. Criar branch de feature / Create feature branch
git checkout develop
git pull origin develop
git checkout -b feature/minha-feature

# 2. Desenvolver / Develop
git add .
git commit -m "feat: adiciona nova funcionalidade"
# ... mais commits

# 3. Manter atualizado / Keep updated
git fetch origin
git rebase origin/develop

# 4. Push e criar PR / Push and create PR
git push origin feature/minha-feature
# Abrir PR no GitHub: feature/minha-feature → develop

# 5. Após merge, limpar / After merge, cleanup
git checkout develop
git pull origin develop
git branch -d feature/minha-feature
git push origin --delete feature/minha-feature
```

---

### Workflow 2: Preparar Release

```bash
# 1. Garantir develop está estável / Ensure develop is stable
git checkout develop
git pull origin develop

# Executar todos os testes / Run all tests
./gradlew test
./gradlew build

# 2. Criar PR para main / Create PR to main
# No GitHub: develop → main
# Título: "Release v27.X-rafaelia"

# 3. Review rigoroso / Rigorous review
# - Verificar changelog / Check changelog
# - Verificar docs / Check docs
# - Executar testes manuais / Run manual tests
# - Security scan

# 4. Após merge, criar tag / After merge, create tag
git checkout main
git pull origin main
git tag -a v27.1-rafaelia -m "Release v27.1"
git push origin v27.1-rafaelia
```

---

### Workflow 3: Hotfix Urgente

```bash
# 1. Criar hotfix de main / Create hotfix from main
git checkout main
git pull origin main
git checkout -b hotfix/corrigir-crash

# 2. Fazer correção mínima / Make minimal fix
git add .
git commit -m "fix: corrige crash em boot"

# 3. Push e PR para main / Push and PR to main
git push origin hotfix/corrigir-crash
# Abrir PR: hotfix/corrigir-crash → main

# 4. Após merge em main / After merge to main
git checkout main
git pull origin main

# Tag de patch release
git tag -a v27.1.1-rafaelia -m "Hotfix: crash boot"
git push origin v27.1.1-rafaelia

# 5. Aplicar em develop / Apply to develop
git checkout develop
git cherry-pick <commit-hash>
git push origin develop

# 6. Limpar / Cleanup
git branch -d hotfix/corrigir-crash
git push origin --delete hotfix/corrigir-crash
```

---

## 📋 Como Contribuir / How to Contribute

### Passo a Passo / Step by Step

#### 1️⃣ Fork e Clone

```bash
# Fork no GitHub interface
# Depois / Then:
git clone https://github.com/SEU-USUARIO/Magisk_Rafaelia.git
cd Magisk_Rafaelia
git remote add upstream https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
```

#### 2️⃣ Criar Feature Branch

```bash
git checkout develop
git pull upstream develop
git checkout -b feature/minha-contribuicao
```

#### 3️⃣ Fazer Mudanças

```bash
# Fazer mudanças / Make changes
# Testar localmente / Test locally
./gradlew test

# Commit
git add .
git commit -m "feat: descrição clara da mudança"
```

#### 4️⃣ Manter Atualizado

```bash
# Sincronizar com upstream
git fetch upstream
git rebase upstream/develop

# Resolver conflitos se houver / Resolve conflicts if any
# Continuar rebase / Continue rebase
git rebase --continue
```

#### 5️⃣ Abrir Pull Request

```bash
# Push para seu fork / Push to your fork
git push origin feature/minha-contribuicao

# No GitHub:
# 1. Ir no seu fork
# 2. Clicar "Compare & pull request"
# 3. Base: rafaelmeloreisnovo/develop
# 4. Compare: SEU-USUARIO/feature/minha-contribuicao
# 5. Preencher descrição
# 6. Criar PR
```

#### 6️⃣ Review e Merge

```bash
# Aguardar review / Wait for review
# Fazer mudanças solicitadas / Make requested changes

# Após merge / After merge:
git checkout develop
git pull upstream develop
git branch -d feature/minha-contribuicao
```

---

## ✅ Checklist para Pull Request

### Antes de Abrir PR / Before Opening PR

- [ ] **Código**
  - [ ] Compila sem erros / Builds without errors
  - [ ] Testes passam / Tests pass
  - [ ] Sem warnings críticos / No critical warnings
  - [ ] Código limpo e comentado / Clean and commented code

- [ ] **Documentação**
  - [ ] README atualizado se necessário / README updated if needed
  - [ ] Docstrings adicionadas / Docstrings added
  - [ ] Changelog atualizado / Changelog updated

- [ ] **Testes**
  - [ ] Novos testes adicionados / New tests added
  - [ ] Cobertura adequada / Adequate coverage
  - [ ] Edge cases cobertos / Edge cases covered

- [ ] **Git**
  - [ ] Commits claros e atômicos / Clear and atomic commits
  - [ ] Branch atualizado com develop / Branch updated with develop
  - [ ] Sem conflitos / No conflicts

---

## 🎨 Convenções de Commit

### Formato / Format

```
tipo(escopo): descrição curta

Descrição detalhada opcional.

Footers opcionais.
```

### Tipos / Types

- **feat**: Nova funcionalidade / New feature
- **fix**: Correção de bug / Bug fix
- **docs**: Documentação / Documentation
- **style**: Formatação / Formatting
- **refactor**: Refatoração / Refactoring
- **test**: Testes / Tests
- **chore**: Manutenção / Maintenance
- **perf**: Performance / Performance
- **ci**: CI/CD / CI/CD

### Exemplos / Examples

```bash
feat(engine): adiciona suporte a GPU

Implementa aceleração GPU usando CuPy para operações tensoriais.

Closes #123
```

```bash
fix(boot): corrige crash em dispositivos Samsung

O patch de boot falhava em dispositivos com Knox. Ajustado para
detectar e tratar Knox adequadamente.

Fixes #456
```

```bash
docs(readme): atualiza guia de instalação

Adiciona seção sobre requisitos mínimos e troubleshooting.
```

---

## 🔍 Review de Código / Code Review

### O Que Verificar / What to Check

**Funcionalidade / Functionality:**
- [ ] Código faz o que diz fazer / Code does what it says
- [ ] Casos extremos tratados / Edge cases handled
- [ ] Sem regressões / No regressions

**Qualidade / Quality:**
- [ ] Código limpo e legível / Clean and readable code
- [ ] Nomes descritivos / Descriptive names
- [ ] Comentários onde necessário / Comments where needed
- [ ] Sem código duplicado / No duplicated code

**Testes / Tests:**
- [ ] Testes adequados incluídos / Adequate tests included
- [ ] Cobertura suficiente / Sufficient coverage
- [ ] Testes passam / Tests pass

**Segurança / Security:**
- [ ] Sem vulnerabilidades óbvias / No obvious vulnerabilities
- [ ] Input validation / Input validation
- [ ] Permissões corretas / Correct permissions

**Performance / Performance:**
- [ ] Sem gargalos óbvios / No obvious bottlenecks
- [ ] Uso eficiente de recursos / Efficient resource usage

---

## 🚀 Processo de Release

### Checklist de Release

#### Preparação / Preparation

- [ ] Todos os testes passam / All tests pass
- [ ] Documentação atualizada / Documentation updated
- [ ] Changelog completo / Complete changelog
- [ ] Security scan limpo / Clean security scan
- [ ] Performance verificada / Performance verified

#### Release / Release

1. **Criar PR develop → main**
   - Título: `Release v27.X-rafaelia`
   - Descrição com changelog

2. **Review rigoroso**
   - Pelo menos 1 aprovação
   - Todos os checks passando

3. **Merge e Tag**
   ```bash
   git checkout main
   git pull origin main
   git tag -a v27.X-rafaelia -m "Release v27.X"
   git push origin v27.X-rafaelia
   ```

4. **Criar GitHub Release**
   - Tag: `v27.X-rafaelia`
   - Release notes do changelog
   - Anexar APK

5. **Comunicar**
   - Postar em discussions
   - Atualizar README principal
   - Notificar usuários

---

## 📊 Resumo Visual / Visual Summary

```
┌─────────────────────────────────────────────────────┐
│                     main                             │
│  (Produção / Production - Sempre estável / Stable)  │
└──────────────┬──────────────────────────────────────┘
               │
               │ PRs rigorosos / Strict PRs
               │ Review obrigatório / Mandatory review
               │
┌──────────────▼──────────────────────────────────────┐
│                    develop                           │
│  (Desenvolvimento / Development - Ativo / Active)    │
└──────────────┬──────────────────────────────────────┘
               │
               │ PRs de features / Feature PRs
               │ Review recomendado / Review recommended
               │
    ┌──────────┴──────────┬───────────────┐
    │                     │               │
┌───▼────┐         ┌──────▼───┐    ┌─────▼────┐
│feature/│         │feature/  │    │ hotfix/  │
│   A    │         │    B     │    │    X     │
└────────┘         └──────────┘    └──────────┘
(temporário)       (temporário)    (de main)
```

---

## 💡 Boas Práticas / Best Practices

### DOs ✅

- ✅ Sempre trabalhar em branch separada
- ✅ Commits pequenos e frequentes
- ✅ Mensagens de commit claras
- ✅ Testar antes de abrir PR
- ✅ Manter branches atualizadas
- ✅ Deletar branches após merge
- ✅ Documentar mudanças significativas
- ✅ Fazer code review construtivo

### DON'Ts ❌

- ❌ Push direto em main
- ❌ Commits enormes
- ❌ Mensagens vagas ("fix", "update")
- ❌ PR sem testes
- ❌ Branches desatualizadas
- ❌ Branches abandonadas
- ❌ Quebrar main
- ❌ Review superficial

---

## 📞 Dúvidas? / Questions?

- 📖 Leia a documentação completa
- 💬 Abra discussion no GitHub
- 🐛 Reporte bugs via issues
- 📧 Contate mantenedores

---

**Filosofia:** VAZIO → VERBO → CHEIO → RETRO  
**Motto:** "Haja Lux, Haja Etica"  
**Assinatura:** RAFCODE-Φ-∆RafaelVerboΩ
