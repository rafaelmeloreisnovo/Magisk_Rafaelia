# Como Obter o APK Compilado do Magisk_Rafaelia

## 📱 Opções para Obter o APK

Existem **3 formas principais** de obter o APK compilado do Magisk_Rafaelia:

---

## 🎯 Opção 1: Download dos Artifacts do GitHub Actions (Mais Fácil)

Esta é a forma **mais rápida e fácil** para usuários que não querem compilar localmente.

### Passos:

1. Acesse a página [Actions](../../actions) do repositório
2. Clique em um workflow de build bem-sucedido (com ✅)
3. Role até o final da página até a seção **"Artifacts"**
4. Baixe o arquivo com o hash do commit (exemplo: `abc123def`)
5. Extraia o arquivo ZIP baixado
6. Dentro você encontrará:
   - `app-release.apk` - APK release assinado
   - `app-debug.apk` - APK debug
   - Binários nativos compilados
   - Manifesto RAFAELIA

### Exemplo Visual:
```
GitHub → Actions → Build Workflow (✅ Success) → Artifacts → Download
```

---

## 🏗️ Opção 2: Compilar Localmente (Completo)

Para desenvolvedores que querem compilar o projeto completo.

### Requisitos:

- **Sistema Operacional**: Linux x64, macOS (Intel/ARM), ou Windows x64
- **Python**: 3.8 ou superior
- **Git**: Com suporte a links simbólicos (Windows: ativar durante instalação)
- **Android Studio**: Instalado e configurado
- **Java**: JDK 17 (pode usar o do Android Studio)
- **Espaço em Disco**: ~10 GB livres

### Passo a Passo:

1. **Clone o repositório com submódulos:**
```bash
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia
```

2. **Configure as variáveis de ambiente:**
```bash
# Linux/macOS
export ANDROID_HOME=/caminho/para/android/sdk
export ANDROID_STUDIO=/caminho/para/android/studio  # Opcional

# Windows (PowerShell)
$env:ANDROID_HOME="C:\Users\SeuUsuario\AppData\Local\Android\Sdk"
$env:ANDROID_STUDIO="C:\Program Files\Android\Android Studio"
```

3. **Instale o Magisk NDK:**
```bash
python3 build.py ndk
```

4. **Configure o build (opcional):**
```bash
# Copie o arquivo de configuração
cp config.prop.sample config.prop

# Edite config.prop se quiser customizar:
# - version: versão do Magisk
# - outdir: diretório de saída
# - abiList: arquiteturas para compilar
```

5. **Compile tudo:**
```bash
# Build completo (release)
python3 build.py -r all

# Ou build debug (mais rápido)
python3 build.py -v all

# Ou apenas o APK (sem binários nativos)
python3 build.py app
```

6. **Encontre os arquivos compilados:**
```
out/
├── app-release.apk          # APK release
├── app-debug.apk            # APK debug
├── magisk-v1.1.0.zip        # ZIP flashável
└── ...
```

### Tempo Estimado:
- **Primeira compilação**: 15-30 minutos
- **Compilações subsequentes**: 5-10 minutos

---

## ⚙️ Opção 3: Usar GitHub Actions para Compilar (Automático)

Para automatizar builds em seu próprio fork.

### Como Funciona:

O repositório já está configurado com GitHub Actions que **automaticamente compilam** o APK quando você:
- Faz push para a branch `master`
- Abre um Pull Request
- Dispara manualmente o workflow

### Workflow Configurado:

O arquivo `.github/workflows/build.yml` já contém:
- ✅ Build de release e debug
- ✅ Upload de artifacts
- ✅ Geração de manifesto RAFAELIA
- ✅ Testes em múltiplas versões do Android

### Para Disparar Manualmente:

1. Acesse [Actions](../../actions)
2. Clique em "Magisk Build"
3. Clique em "Run workflow"
4. Selecione a branch e clique em "Run workflow"
5. Aguarde a compilação (15-20 minutos)
6. Baixe os artifacts quando concluído

---

## 📦 O Que Você Recebe

Após compilar ou baixar, você terá:

### APKs:
- **app-release.apk**: Versão para distribuição (requer keystore próprio)
- **app-debug.apk**: Versão para desenvolvimento (não verificar assinatura)

### Binários Nativos:
- `magisk` - Binário principal do Magisk
- `magiskboot` - Ferramenta para manipular boot images
- `magiskinit` - Init substituto
- `magiskpolicy` - Manipulação de políticas SELinux

### Arquiteturas Suportadas:
- ARM 64-bit (arm64-v8a)
- ARM 32-bit (armeabi-v7a)
- Intel/AMD 64-bit (x86_64)
- Intel/AMD 32-bit (x86)

### Extras:
- Manifesto RAFAELIA com hash SHA256 e metadados
- Símbolos de debug (se build de release)

---

## 🔐 Assinando o APK (Opcional para Distribuição)

Se você quer distribuir seu próprio build:

1. **Gere uma keystore:**
```bash
keytool -genkey -v -keystore my-release-key.jks \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias my-alias
```

2. **Configure no config.prop:**
```properties
keyStore=my-release-key.jks
keyStorePass=sua_senha
keyAlias=my-alias
keyPass=senha_da_chave
```

3. **Compile com release:**
```bash
python3 build.py -r all
```

⚠️ **IMPORTANTE**: Builds oficiais do Magisk verificam assinaturas. Use builds debug para desenvolvimento.

---

## 🐛 Problemas Comuns

### "Could not GET dl.google.com"
- **Causa**: Restrições de rede bloqueando acesso ao Google Maven
- **Solução**: Use Opção 1 (GitHub Actions) ou compile em ambiente com internet completa

### "Unknown ABI: [string]"
- **Causa**: config.prop com valores placeholder
- **Solução**: Deixe `abiList=` vazio ou remova config.prop

### "NDK not found"
- **Causa**: Magisk NDK não instalado
- **Solução**: Execute `python3 build.py ndk`

### "Git submodules not initialized"
- **Causa**: Clone sem `--recurse-submodules`
- **Solução**: Execute `git submodule update --init --recursive`

### Build falha no Windows
- **Causa**: Links simbólicos não habilitados
- **Solução**: Ative modo desenvolvedor no Windows e reinstale Git com suporte a symlinks

---

## 📚 Documentação Adicional

- [Build.md](docs/build.md) - Documentação completa de build em inglês
- [README.MD](README.MD) - Visão geral do projeto
- [BUILD_SUCCESS.md](BUILD_SUCCESS.md) - Relatório do último build bem-sucedido
- [COMPILATION_SUMMARY.md](COMPILATION_SUMMARY.md) - Resumo da compilação anterior

---

## ✅ Verificação do APK

Após obter o APK, você pode verificar:

```bash
# Ver informações do APK
aapt dump badging app-release.apk | grep -E 'package|versionName'

# Verificar assinatura
jarsigner -verify -verbose -certs app-release.apk

# Extrair e inspecionar
unzip -l app-release.apk
```

---

## 🆘 Suporte

Se você ainda tiver problemas:

1. Verifique os [Issues](../../issues) existentes
2. Consulte a [documentação oficial do Magisk](https://topjohnwu.github.io/Magisk/)
3. Abra um novo issue com:
   - Sistema operacional
   - Versão do Python/Java
   - Log completo do erro
   - Passos para reproduzir

---

## 📝 Resumo Rápido

| Método | Dificuldade | Tempo | Requisitos | Melhor Para |
|--------|-------------|-------|------------|-------------|
| **GitHub Actions Download** | ⭐ Fácil | 5 min | Browser | Usuários finais |
| **Compilação Local** | ⭐⭐⭐ Médio | 15-30 min | Dev env completo | Desenvolvedores |
| **GitHub Actions Manual** | ⭐⭐ Fácil | 20 min | Conta GitHub | Contributors |

---

**Versão**: 1.1.0-rafaelia  
**Última Atualização**: 2025-11-04  
**Assinatura**: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
