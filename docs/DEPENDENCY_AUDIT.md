# Dependency Baseline Audit

Este repositório possui uma auditoria automática para detectar dependências obsoletas na trilha de build/release:

- GitHub Actions com versões antigas de `checkout`, `setup-java`, `setup-python`, `setup-android`, `setup-gradle`, `upload-artifact`, `download-artifact` e `first-interaction`.
- Referências em workflows e também em **composite actions** (`.github/actions/**/action.yml`).
- Baseline de Gradle Wrapper (`app/gradle/wrapper/gradle-wrapper.properties`).
- Baseline de Android Gradle Plugin e Kotlin (`app/gradle/libs.versions.toml`).

## Execução local

```bash
python3 .github/scripts/dependency_audit.py
```

## Execução na CI

A auditoria roda no workflow `quality-gates.yml` no job **Dependency Baseline Audit** e falha o pipeline quando encontra baseline obsoleta.


## Cobertura de arquitetura

O workflow `Buildnew.yml` está normalizado com 5 ABIs: `armeabi-v7a`, `arm64-v8a`, `x86`, `x86_64` e `riscv64`.
