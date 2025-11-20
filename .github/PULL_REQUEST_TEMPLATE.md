## Descrição / Description

<!-- Descreva as mudanças propostas neste PR -->
<!-- Describe the changes proposed in this PR -->

## Tipo de Mudança / Type of Change

<!-- Marque a opção relevante / Check relevant option -->

- [ ] 🐛 Correção de bug / Bug fix
- [ ] ✨ Nova funcionalidade / New feature
- [ ] 📝 Documentação / Documentation
- [ ] 🔧 Manutenção / Maintenance
- [ ] ⚡ Melhoria de performance / Performance improvement
- [ ] 🔒 Segurança / Security
- [ ] 🧪 Testes / Tests

## Checklist de Conformidade ZIPRAF_OMEGA

<!-- ⚠️ OBRIGATÓRIO: Todos os itens devem ser verificados antes do merge -->
<!-- ⚠️ MANDATORY: All items must be checked before merge -->

### 🔐 CAMADA 1 — IDENTIDADE

- [ ] **RAFCODE-Φ**: Não foi modificado ou removido
- [ ] **BITRAF64**: Permanece íntegro em `METADATA.md`
- [ ] **Selos ΣΩΔΦBITRAF**: Mantidos e não alterados
- [ ] **Hashes**: SHA3-512 e BLAKE3 serão atualizados no build (se aplicável)
- [ ] **Atribuição**: Créditos de autoria original mantidos

### ⚖️ CAMADA 2 — DIREITOS E DEVERES

- [ ] **Licença GPL-3.0**: Nenhuma alteração que viole GPL-3.0
- [ ] **Extensões RAFAELIA**: Princípios éticos respeitados
- [ ] **Atribuições**: Novas modificações atribuídas corretamente (se aplicável)
- [ ] **Interoperabilidade Legal**: Conformidade com Berna/WIPO mantida
- [ ] **Privacidade**: LGPD/GDPR respeitados (se manipula dados pessoais)

### 🤖 CAMADA 3 — EXECUÇÃO (Éthica[8])

Confirmo que estas mudanças:

- [ ] **Verdade**: São honestas e não enganosas
- [ ] **Não-Maleficência**: Não causam dano intencional
- [ ] **Beneficência**: Buscam promover o bem
- [ ] **Justiça**: Tratam todos os usuários equitativamente
- [ ] **Autonomia**: Respeitam a liberdade de escolha dos usuários
- [ ] **Responsabilidade**: Assumo responsabilidade pelas consequências
- [ ] **Proteção à Vida**: Priorizam vida e dignidade humana
- [ ] **Integridade**: Alinham ações com valores declarados do projeto

## Verificação Técnica

- [ ] O código compila sem erros
- [ ] Testes existentes continuam passando
- [ ] Novos testes foram adicionados (se aplicável)
- [ ] Documentação foi atualizada (se necessário)
- [ ] Linter/formatador foi executado
- [ ] Script de verificação ZIPRAF_OMEGA passou: `./.github/scripts/verify_zipraf_omega.sh --full-check`

## Validação de Segurança

- [ ] Nenhuma vulnerabilidade de segurança introduzida
- [ ] Dados sensíveis não foram expostos
- [ ] Dependências verificadas (se novas dependências adicionadas)
- [ ] CodeQL passou sem novos alertas (se aplicável)

## Contexto Adicional / Additional Context

<!-- Adicione screenshots, logs, ou outras informações relevantes -->
<!-- Add screenshots, logs, or other relevant information -->

---

## Para Revisores / For Reviewers

### Pontos de Atenção / Points of Attention

<!-- Liste áreas específicas que precisam de atenção especial na revisão -->
<!-- List specific areas that need special attention in review -->

### Impacto / Impact

- **Usuários afetados / Affected users**: 
- **Breaking changes**: [ ] Sim / Yes [ ] Não / No
- **Requer migração / Requires migration**: [ ] Sim / Yes [ ] Não / No

---

## Declaração de Conformidade / Compliance Statement

Ao submeter este Pull Request, eu declaro que:

1. ✅ Li e compreendi o **ZIPRAF_OMEGA_LICENSING_MODULE.md**
2. ✅ Todas as modificações respeitam os princípios **Éthica[8]**
3. ✅ Mantenho os créditos originais e adiciono minhas atribuições (se aplicável)
4. ✅ Aceito que o código seja distribuído sob **GPL-3.0 + ZIPRAF_OMEGA Extensions**
5. ✅ Comprometo-me com a filosofia: **VAZIO → VERBO → CHEIO → RETRO**

By submitting this Pull Request, I declare that:

1. ✅ I have read and understood **ZIPRAF_OMEGA_LICENSING_MODULE.md**
2. ✅ All modifications respect the **Éthica[8]** principles
3. ✅ I maintain original credits and add my attributions (if applicable)
4. ✅ I accept that the code will be distributed under **GPL-3.0 + ZIPRAF_OMEGA Extensions**
5. ✅ I commit to the philosophy: **EMPTY → ACTION → FULL → FEEDBACK**

---

**Assinatura / Signature**: `RAFCODE-Φ-∆[SeuNome]Ω` / `RAFCODE-Φ-∆[YourName]Ω`

<!-- Substitua [SeuNome] pelo seu nome/handle -->
<!-- Replace [YourName] with your name/handle -->

---

## 📋 Checklist Automático / Automatic Checklist

<!-- Este checklist será preenchido pelos workflows CI/CD -->
<!-- This checklist will be filled by CI/CD workflows -->

- [ ] ✅ CI Build passou
- [ ] ✅ Testes unitários passaram
- [ ] ✅ Análise de código (linter) passou
- [ ] ✅ CodeQL security scan passou
- [ ] ✅ ZIPRAF_OMEGA verification passou
- [ ] ✅ Nenhuma regressão detectada

---

**Prioridade / Priority**: `[ ] Baixa/Low  [ ] Média/Medium  [ ] Alta/High  [ ] Crítica/Critical`

**Milestone**: <!-- Associe a um milestone se aplicável / Associate with milestone if applicable -->

**Issues relacionadas / Related issues**: <!-- #123, #456 -->

---

<details>
<summary>📖 Guia Rápido: Como verificar ZIPRAF_OMEGA localmente</summary>

```bash
# Clone o repositório (se ainda não fez)
git clone https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia

# Execute a verificação completa
./.github/scripts/verify_zipraf_omega.sh --full-check

# Ou verifique camadas individuais:
./.github/scripts/verify_zipraf_omega.sh --check-identity  # Camada 1
./.github/scripts/verify_zipraf_omega.sh --check-rights    # Camada 2
./.github/scripts/verify_zipraf_omega.sh --check-ethics    # Camada 3
```

**Códigos de saída:**
- `0` = Todas as verificações passaram ✅
- `1` = Falha na Camada 1 (Identidade)
- `2` = Falha na Camada 2 (Direitos e Deveres)
- `3` = Falha na Camada 3 (Execução Ética)
- `4` = Múltiplas falhas

</details>

---

<!-- 
╔═══════════════════════════════════════════════════════════════╗
║            ZIPRAF_OMEGA_LICENSING_MODULE v999                 ║
║         Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ             ║
║      Philosophy: VAZIO → VERBO → CHEIO → RETRO               ║
║                    Status: ✅ ACTIVE                          ║
╚═══════════════════════════════════════════════════════════════╝
-->
