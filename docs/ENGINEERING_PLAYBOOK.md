# Engineering Playbook (Build, Release, CI, Native)

## Objetivo

Este documento define a **fonte única de verdade** para build/release do projeto, alinhando Android, Gradle, CMake, NDK, JNI, SDK/JDK e GitHub Actions.

## Fonte de verdade

1. **Build Orquestrado**: `build.py`
2. **Build Android local**: `python3 build.py -v all`
3. **Build Android release**: `python3 build.py -vr all`
4. **Build nativo (C/C++)**: `native/CMakeLists.txt`
5. **Validação pré-CI**: `scripts/pre_ci_validate.py`
6. **CI principal**: `.github/workflows/ci.yml`
7. **Build de artefatos**: `.github/workflows/build.yml`

## Pipeline mínimo coerente

### 1) Bootstrap de toolchain

- JDK configurado (CI usa setup-java).
- Android SDK + Build Tools.
- NDK provisionado por `python3 build.py ndk`.

### 2) Build nativo

- Configuração por CMake (`native/CMakeLists.txt`).
- Compilação de bibliotecas consumidas pela aplicação Android.

### 3) Build Android

- Build controlado por `build.py`, evitando divergência entre CI e build local.
- Variantes debug/release geradas pelo mesmo caminho.

### 4) Testes e validações

- Validações rápidas: `scripts/pre_ci_validate.py --skip-slow`.
- Validações completas: `scripts/pre_ci_validate.py --strict`.

### 5) Artefatos

- Workflows principais exigem upload via `actions/upload-artifact`.
- Diretórios de saída permanecem rastreáveis em `out/` e `app/apk/build/outputs`.

## Contratos de engenharia

1. Não introduzir caminho alternativo que ignore `build.py` em CI oficial.
2. Não remover upload de artefatos dos workflows primários.
3. Não converter release oficial em unsigned por conveniência.
4. Se alterar contratos de build, atualizar este playbook + `docs/README.md`.

## Comandos operacionais

```bash
# validação rápida (pré-commit)
python3 scripts/pre_ci_validate.py --skip-slow

# validação completa (pré-PR)
python3 scripts/pre_ci_validate.py --strict

# bootstrap NDK
python3 build.py ndk

# build debug
python3 build.py -v all

# build release
python3 build.py -vr all
```

## Artefatos esperados

- APKs em `app/apk/build/outputs/...`
- Saídas de build em `out/`
- Artefatos de CI via Actions (jobs em `ci.yml` e `build.yml`)

## Política de mudanças estruturais

Ao modificar Gradle/CMake/NDK/JNI/workflows:

1. Confirmar build local mínimo.
2. Confirmar validação pré-CI.
3. Confirmar presença de upload de artefato.
4. Atualizar documentação desta seção.
