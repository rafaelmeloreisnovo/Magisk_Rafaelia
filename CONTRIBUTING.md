# Contributing to Magisk_Rafaelia / Contribuindo para Magisk_Rafaelia

[🇧🇷 Português](#português) | [🇬🇧 English](#english)

---

## Português

### Estratégia de Branches - Duas Branches

O projeto Magisk_Rafaelia utiliza uma estratégia simplificada de **duas branches** para manter a rastreabilidade e aplicar melhores práticas de desenvolvimento:

#### 1. **`master`** - Branch de Produção
- Branch principal e estável
- Contém apenas código testado e aprovado
- Protegida contra commits diretos
- Apenas aceita merges via Pull Requests do branch `rascunho`
- Representa a versão de produção do projeto

#### 2. **`rascunho`** (Rascunho/Draft) - Branch de Desenvolvimento
- Branch de desenvolvimento e staging
- Onde novas funcionalidades são integradas primeiro
- Todas as feature branches devem fazer merge para `rascunho`
- Código é revisado e testado aqui antes de ir para `master`
- Permite manter as mudanças enfileiradas para revisão

### Fluxo de Trabalho

```
feature branch → rascunho → master
     (PR)          (PR)
```

#### Para Contribuidores:

1. **Criar uma Feature Branch**
   ```bash
   git checkout rascunho
   git pull origin rascunho
   git checkout -b feature/minha-funcionalidade
   ```

2. **Desenvolver e Commit**
   ```bash
   # Faça suas alterações
   git add .
   git commit -m "feat: adiciona nova funcionalidade"
   ```

3. **Push e Pull Request para `rascunho`**
   ```bash
   git push origin feature/minha-funcionalidade
   ```
   - Abra um Pull Request para o branch `rascunho`
   - Descreva suas mudanças claramente
   - Aguarde revisão e aprovação

4. **Após Merge em `rascunho`**
   - O mantenedor revisará as mudanças em `rascunho`
   - Se aprovado, um PR será criado de `rascunho` → `master`
   - Isso mantém a rastreabilidade completa

### Vantagens desta Estratégia

✅ **Rastreabilidade**: Todas as mudanças são registradas e revisadas  
✅ **Controle de Qualidade**: Duas camadas de revisão (rascunho → master)  
✅ **Organização**: Mudanças ficam enfileiradas em `rascunho`  
✅ **Segurança**: `master` permanece sempre estável  
✅ **Simplicidade**: Apenas 2 branches principais para gerenciar  

### Diretrizes de Commit

Use commits semânticos:
- `feat:` - Nova funcionalidade
- `fix:` - Correção de bug
- `docs:` - Apenas documentação
- `refactor:` - Refatoração de código
- `test:` - Adiciona ou corrige testes
- `chore:` - Manutenção, configuração

Exemplo:
```bash
git commit -m "feat: adiciona suporte a novos módulos RAFAELIA"
git commit -m "fix: corrige crash no boot loader"
git commit -m "docs: atualiza guia de instalação"
```

### Pull Request Template

Ao criar um PR, inclua:
- **Título**: Descrição clara e concisa
- **Descrição**: O que foi alterado e por quê
- **Tipo de Mudança**: Feature, Bug Fix, Documentação, etc.
- **Testes**: Como foi testado
- **Checklist**: Marque todos os itens aplicáveis

### Políticas de Branch Protection

#### Branch `master`:
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Include administrators (enforce for everyone)
- ❌ Não permite commits diretos

#### Branch `rascunho`:
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging
- ⚠️ Permite merge mais flexível para desenvolvimento

---

## English

### Branch Strategy - Two Branches

The Magisk_Rafaelia project uses a simplified **two-branch** strategy to maintain traceability and apply development best practices:

#### 1. **`master`** - Production Branch
- Main and stable branch
- Contains only tested and approved code
- Protected against direct commits
- Only accepts merges via Pull Requests from `rascunho`
- Represents the production version of the project

#### 2. **`rascunho`** (Draft) - Development Branch
- Development and staging branch
- Where new features are integrated first
- All feature branches should merge into `rascunho`
- Code is reviewed and tested here before going to `master`
- Allows changes to be queued for review

### Workflow

```
feature branch → rascunho → master
     (PR)          (PR)
```

#### For Contributors:

1. **Create a Feature Branch**
   ```bash
   git checkout rascunho
   git pull origin rascunho
   git checkout -b feature/my-feature
   ```

2. **Develop and Commit**
   ```bash
   # Make your changes
   git add .
   git commit -m "feat: add new feature"
   ```

3. **Push and Pull Request to `rascunho`**
   ```bash
   git push origin feature/my-feature
   ```
   - Open a Pull Request to the `rascunho` branch
   - Describe your changes clearly
   - Wait for review and approval

4. **After Merge into `rascunho`**
   - The maintainer will review changes in `rascunho`
   - If approved, a PR will be created from `rascunho` → `master`
   - This maintains complete traceability

### Advantages of This Strategy

✅ **Traceability**: All changes are recorded and reviewed  
✅ **Quality Control**: Two layers of review (rascunho → master)  
✅ **Organization**: Changes are queued in `rascunho`  
✅ **Safety**: `master` always remains stable  
✅ **Simplicity**: Only 2 main branches to manage  

### Commit Guidelines

Use semantic commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `refactor:` - Code refactoring
- `test:` - Add or fix tests
- `chore:` - Maintenance, configuration

Example:
```bash
git commit -m "feat: add support for new RAFAELIA modules"
git commit -m "fix: fix boot loader crash"
git commit -m "docs: update installation guide"
```

### Pull Request Template

When creating a PR, include:
- **Title**: Clear and concise description
- **Description**: What was changed and why
- **Type of Change**: Feature, Bug Fix, Documentation, etc.
- **Testing**: How it was tested
- **Checklist**: Check all applicable items

### Branch Protection Policies

#### Branch `master`:
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Include administrators (enforce for everyone)
- ❌ No direct commits allowed

#### Branch `rascunho`:
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging
- ⚠️ Allows more flexible merging for development

---

## Getting Started

### First Time Setup

```bash
# Clone the repository
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia

# Make sure you're on rascunho for development
git checkout rascunho
git pull origin rascunho

# Create your feature branch
git checkout -b feature/your-feature-name
```

### Before Submitting a PR

1. ✅ Run tests: `python3 build.py -v all`
2. ✅ Check code quality
3. ✅ Update documentation if needed
4. ✅ Write clear commit messages
5. ✅ Create PR to `rascunho` (not `master`)

---

## Questions?

- 📖 Read the [README.MD](README.MD)
- 🔧 Check the [Build Guide](docs/build.md)
- 🐛 Report bugs via [Issues](../../issues)

Thank you for contributing! / Obrigado por contribuir!

**RAFCODE-Φ-∆RafaelVerboΩ**
