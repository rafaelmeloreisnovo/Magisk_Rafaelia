#!/usr/bin/env bash
# ==================================================
# ZIPRAF_OMEGA License Verification Script
# ==================================================
# Signature: RAFCODE-Φ-∆VerificationΩ
# Version: 1.0.0
# Purpose: Validate 3-layer autonomous fractal licensing
# ==================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Exit codes
EXIT_SUCCESS=0
EXIT_IDENTITY_FAIL=1
EXIT_RIGHTS_FAIL=2
EXIT_ETHICS_FAIL=3
EXIT_MULTIPLE_FAIL=4

# Counters
ERRORS=0
WARNINGS=0

# ==================================================
# Helper Functions
# ==================================================

log_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

log_success() {
    echo -e "${GREEN}✅${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

log_error() {
    echo -e "${RED}❌${NC} $1"
    ((ERRORS++))
}

print_header() {
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "$1"
    echo "═══════════════════════════════════════════════════════════════"
}

# ==================================================
# CAMADA 1 — IDENTIDADE
# ==================================================

verify_layer1_identity() {
    print_header "🔐 CAMADA 1 — IDENTIDADE"
    
    local layer1_pass=true
    
    # Check RAFCODE-Φ in METADATA.md
    log_info "Verificando RAFCODE-Φ..."
    if [ -f "METADATA.md" ]; then
        if grep -q "RAFCODE-Φ" METADATA.md; then
            log_success "RAFCODE-Φ encontrado em METADATA.md"
        else
            log_error "RAFCODE-Φ não encontrado em METADATA.md"
            layer1_pass=false
        fi
    else
        log_error "METADATA.md não encontrado"
        layer1_pass=false
    fi
    
    # Check BITRAF64
    log_info "Verificando BITRAF64..."
    if [ -f "METADATA.md" ]; then
        if grep -q "Bitraf64 identifier:" METADATA.md; then
            # Extract and validate format (should be 64+ characters with Greek letters)
            bitraf64=$(grep -A1 "Bitraf64 identifier:" METADATA.md | tail -n1)
            if [ ${#bitraf64} -ge 40 ]; then
                log_success "BITRAF64 encontrado e tem comprimento adequado"
            else
                log_warning "BITRAF64 parece curto demais"
            fi
        else
            log_error "BITRAF64 não encontrado em METADATA.md"
            layer1_pass=false
        fi
    fi
    
    # Check Seals (Selos)
    log_info "Verificando Selos ΣΩΔΦBITRAF..."
    if [ -f "METADATA.md" ] || [ -f "RAFAELIA_MANIFEST.json" ]; then
        if grep -q "Selos:" METADATA.md 2>/dev/null || grep -q "selos" RAFAELIA_MANIFEST.json 2>/dev/null; then
            log_success "Selos ΣΩΔΦBITRAF encontrados"
        else
            log_error "Selos ΣΩΔΦBITRAF não encontrados"
            layer1_pass=false
        fi
    fi
    
    # Check for hash placeholders (SHA3-512 and BLAKE3)
    log_info "Verificando hashes SHA3-512 e BLAKE3..."
    if [ -f "METADATA.md" ]; then
        if grep -q "sha3:" METADATA.md && grep -q "blake3:" METADATA.md; then
            log_success "Campos de hash SHA3-512 e BLAKE3 presentes"
        else
            log_warning "Campos de hash não encontrados (serão gerados no build)"
        fi
    fi
    
    if [ "$layer1_pass" = true ]; then
        log_success "CAMADA 1 — IDENTIDADE: ✅ PASSOU"
        return 0
    else
        log_error "CAMADA 1 — IDENTIDADE: ❌ FALHOU"
        return 1
    fi
}

# ==================================================
# CAMADA 2 — DIREITOS E DEVERES
# ==================================================

verify_layer2_rights() {
    print_header "⚖️ CAMADA 2 — DIREITOS E DEVERES"
    
    local layer2_pass=true
    
    # Check LICENSE file (GPL-3.0 with RAFAELIA extensions)
    log_info "Verificando LICENSE..."
    if [ -f "LICENSE" ]; then
        if grep -q "GNU GENERAL PUBLIC LICENSE" LICENSE; then
            log_success "GPL-3.0 presente no LICENSE"
        else
            log_error "GPL-3.0 não encontrado no LICENSE"
            layer2_pass=false
        fi
        
        if grep -q "RAFAELIA" LICENSE || grep -q "RAFCODE-Φ" LICENSE; then
            log_success "Extensões RAFAELIA presentes no LICENSE"
        else
            log_warning "Extensões RAFAELIA não encontradas no LICENSE"
        fi
    else
        log_error "LICENSE não encontrado"
        layer2_pass=false
    fi
    
    # Check for attribution (copyright notices)
    log_info "Verificando atribuições de autoria..."
    if grep -rq "Rafael" LICENSE README.MD 2>/dev/null; then
        log_success "Atribuições de autoria encontradas"
    else
        log_warning "Atribuições de autoria não encontradas em arquivos principais"
    fi
    
    # Check for LGPD/GDPR references
    log_info "Verificando conformidade LGPD/GDPR..."
    if grep -rq "LGPD\|GDPR" LICENSE 2>/dev/null; then
        log_success "Referências a LGPD/GDPR encontradas"
    else
        log_warning "Referências a LGPD/GDPR não encontradas (opcional)"
    fi
    
    if [ "$layer2_pass" = true ]; then
        log_success "CAMADA 2 — DIREITOS E DEVERES: ✅ PASSOU"
        return 0
    else
        log_error "CAMADA 2 — DIREITOS E DEVERES: ❌ FALHOU"
        return 1
    fi
}

# ==================================================
# CAMADA 3 — EXECUÇÃO
# ==================================================

verify_layer3_execution() {
    print_header "🤖 CAMADA 3 — EXECUÇÃO"
    
    local layer3_pass=true
    
    # Check for ZIPRAF_OMEGA_LICENSING_MODULE documentation
    log_info "Verificando módulo de licenciamento..."
    if [ -f "ZIPRAF_OMEGA_LICENSING_MODULE.md" ]; then
        log_success "ZIPRAF_OMEGA_LICENSING_MODULE.md encontrado"
        
        # Verify it contains Ethica[8]
        if grep -q "Éthica\[8\]" ZIPRAF_OMEGA_LICENSING_MODULE.md; then
            log_success "Éthica[8] documentada"
        else
            log_warning "Éthica[8] não encontrada na documentação"
        fi
    else
        log_error "ZIPRAF_OMEGA_LICENSING_MODULE.md não encontrado"
        layer3_pass=false
    fi
    
    # Check for ethical principles in LICENSE
    log_info "Verificando princípios éticos..."
    local ethics_found=0
    local ethics_principles=("Truth" "Non-maleficence" "Beneficence" "Justice" "Autonomy" "Responsibility" "Protection" "Integrity")
    
    for principle in "${ethics_principles[@]}"; do
        if grep -q "$principle" LICENSE 2>/dev/null || grep -q "$principle" ZIPRAF_OMEGA_LICENSING_MODULE.md 2>/dev/null; then
            ((ethics_found++))
        fi
    done
    
    if [ $ethics_found -ge 6 ]; then
        log_success "Princípios éticos Éthica[8] encontrados ($ethics_found/8)"
    else
        log_warning "Poucos princípios éticos documentados ($ethics_found/8)"
    fi
    
    # Check for activation file
    log_info "Verificando arquivo de ativação..."
    if [ -f "ZIPRAF_OMEGA_FULL DO it ativar.txt" ]; then
        log_success "Arquivo de ativação ZIPRAF_OMEGA presente"
    else
        log_warning "Arquivo de ativação não encontrado"
    fi
    
    # Integrity check (basic file structure)
    log_info "Verificando integridade básica do projeto..."
    local required_files=("LICENSE" "README.MD" "METADATA.md")
    local missing_files=0
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            log_error "Arquivo obrigatório ausente: $file"
            ((missing_files++))
            layer3_pass=false
        fi
    done
    
    if [ $missing_files -eq 0 ]; then
        log_success "Todos os arquivos obrigatórios presentes"
    fi
    
    if [ "$layer3_pass" = true ]; then
        log_success "CAMADA 3 — EXECUÇÃO: ✅ PASSOU"
        return 0
    else
        log_error "CAMADA 3 — EXECUÇÃO: ❌ FALHOU"
        return 1
    fi
}

# ==================================================
# Main Verification Function
# ==================================================

full_check() {
    print_header "🔐 ZIPRAF_OMEGA_LICENSING_MODULE v999"
    log_info "Iniciando verificação completa de licenciamento..."
    log_info "Timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo ""
    
    local layer1_result=0
    local layer2_result=0
    local layer3_result=0
    
    # Run all layer checks
    verify_layer1_identity || layer1_result=$?
    echo ""
    
    verify_layer2_rights || layer2_result=$?
    echo ""
    
    verify_layer3_execution || layer3_result=$?
    echo ""
    
    # Final summary
    print_header "📊 RESUMO DA VERIFICAÇÃO"
    
    if [ $layer1_result -eq 0 ] && [ $layer2_result -eq 0 ] && [ $layer3_result -eq 0 ]; then
        log_success "TODAS AS CAMADAS PASSARAM ✅"
        echo ""
        echo "╔═══════════════════════════════════════════════════════════════╗"
        echo "║                    LICENÇA VERIFICADA                         ║"
        echo "║              ZIPRAF_Ω_FUNCTION = AUTORIZADO                   ║"
        echo "╚═══════════════════════════════════════════════════════════════╝"
        echo ""
        log_info "Erros: $ERRORS | Avisos: $WARNINGS"
        return $EXIT_SUCCESS
    else
        log_error "FALHAS DETECTADAS ❌"
        echo ""
        echo "╔═══════════════════════════════════════════════════════════════╗"
        echo "║                  LICENÇA NÃO VERIFICADA                       ║"
        echo "║               ZIPRAF_Ω_FUNCTION = NEGADO                      ║"
        echo "╚═══════════════════════════════════════════════════════════════╝"
        echo ""
        
        # Determine specific exit code
        if [ $layer1_result -ne 0 ]; then
            log_error "Camada 1 (Identidade) falhou"
        fi
        if [ $layer2_result -ne 0 ]; then
            log_error "Camada 2 (Direitos e Deveres) falhou"
        fi
        if [ $layer3_result -ne 0 ]; then
            log_error "Camada 3 (Execução) falhou"
        fi
        
        log_info "Erros: $ERRORS | Avisos: $WARNINGS"
        
        # Return appropriate exit code
        local failed_layers=$((layer1_result + layer2_result + layer3_result))
        if [ $failed_layers -gt 1 ]; then
            return $EXIT_MULTIPLE_FAIL
        elif [ $layer1_result -ne 0 ]; then
            return $EXIT_IDENTITY_FAIL
        elif [ $layer2_result -ne 0 ]; then
            return $EXIT_RIGHTS_FAIL
        else
            return $EXIT_ETHICS_FAIL
        fi
    fi
}

check_identity_only() {
    verify_layer1_identity
}

check_rights_only() {
    verify_layer2_rights
}

check_ethics_only() {
    verify_layer3_execution
}

# ==================================================
# Main Script
# ==================================================

show_help() {
    echo "ZIPRAF_OMEGA License Verification Script"
    echo ""
    echo "Uso: $0 [OPÇÃO]"
    echo ""
    echo "Opções:"
    echo "  --full-check          Verificação completa (todas as 3 camadas)"
    echo "  --check-identity      Verificar apenas Camada 1 (Identidade)"
    echo "  --check-rights        Verificar apenas Camada 2 (Direitos e Deveres)"
    echo "  --check-ethics        Verificar apenas Camada 3 (Execução)"
    echo "  -h, --help            Mostrar esta ajuda"
    echo ""
    echo "Códigos de saída:"
    echo "  0 - Sucesso"
    echo "  1 - Falha na Camada 1 (Identidade)"
    echo "  2 - Falha na Camada 2 (Direitos e Deveres)"
    echo "  3 - Falha na Camada 3 (Execução)"
    echo "  4 - Múltiplas falhas"
    echo ""
}

# Parse command line arguments
case "${1:-}" in
    --full-check)
        full_check
        exit $?
        ;;
    --check-identity)
        check_identity_only
        exit $?
        ;;
    --check-rights)
        check_rights_only
        exit $?
        ;;
    --check-ethics)
        check_ethics_only
        exit $?
        ;;
    -h|--help)
        show_help
        exit 0
        ;;
    "")
        # Default to full check
        full_check
        exit $?
        ;;
    *)
        echo "Opção inválida: $1"
        echo "Use --help para ver opções disponíveis"
        exit 1
        ;;
esac
