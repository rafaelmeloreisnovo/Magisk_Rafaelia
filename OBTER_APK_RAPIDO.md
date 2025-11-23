# 🚀 Início Rápido: Obter APK do Magisk_Rafaelia

## ⚡ Forma Mais Rápida (2 cliques)

1. Acesse [Actions](../../actions/workflows/build.yml?query=is%3Asuccess)
2. Clique no último build bem-sucedido → Baixe artifacts no final da página

✅ **Pronto!** Extraia o ZIP e você encontrará:
- `app-debug.apk` - Para testes (assinado com chave de debug)
- `app-release.apk` - Para distribuição (assinado)
- `app-unsigned.apk` - **SEM ASSINATURA** (assine você mesmo!)

---

## 🛠️ Compilar Localmente (3 comandos)

```bash
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia
python3 build.py ndk && python3 build.py -v all
```

Encontre o APK em `out/app-debug.apk`

---

## 📖 Guias Detalhados

- 🇧🇷 [Guia em Português](COMO_OBTER_APK.md) - Guia completo
- 🇬🇧 [English Guide](HOW_TO_GET_APK.md) - Complete guide in English

---

**Links Rápidos:**
- [📦 Baixar do Actions](../../actions)
- [📚 README Completo](README.MD)
- [🔧 Documentação de Build](docs/build.md)
