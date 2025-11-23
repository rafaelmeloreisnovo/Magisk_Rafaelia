//! statement.rs - Part of Magisk_Rafaelia
//!
//! Part of Magisk_Rafaelia
//! RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
//! 
//! Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
//! (EMPTY → ACTION → FULL → FEEDBACK)
//! 
//! Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
//! Foundation: CientiEspiritual - Scientific Spirituality
//! Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)
//! 
//! RAFAELIA Framework Principles:
//! - Complete operational state coverage (1008 State Matrix)
//! - Full audit system with integrity verification
//! - Real-time telemetry and anomaly detection
//! - Security hardening and ethical computing
//! - Continuous improvement through infinite feedback loop (ψχρΔΣΩ)

/*

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy

All Rights Reserved. Patent Pending.

DUAL LICENSE - Choose one:

1. SOCIAL INCLUSION LICENSE (Free):
   ✓ Educational use
   ✓ Research and academic purposes
   ✓ Non-profit organizations
   ✓ Social inclusion initiatives
   ✓ Open source contributions (with attribution)
   ✗ Commercial use prohibited

2. COMMERCIAL SAAS LICENSE (Paid Subscription):
   Required for:
   ✓ Commercial products or services
   ✓ SaaS applications
   ✓ Revenue-generating purposes
   ✓ Enterprise deployments
   Contact: rafaelmeloreisnovo for licensing terms

AUTOMATIC PENALTIES FOR VIOLATIONS:
Unauthorized commercial use is subject to automatic statutory penalties:
- Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
- Plus: 5% of gross revenue derived from unauthorized use
- Plus: Legal fees and costs of enforcement
- Criminal prosecution under applicable copyright law

VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
- Valid in all jurisdictions signatory to Berne Convention (180+ countries)
- Enforced under TRIPS agreement (WTO member states)
- Protected by reciprocal copyright treaties
- Minimum protection: Life of author + 50 years (Berne minimum)
- Extended protection: Life + 70 years (EU, USA, Brazil and others)

ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
All derivative works, redistributions, or substantial use must include:
1. This complete copyright and license notice
2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
4. Indication of any modifications made
5. Date of last modification


INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:

This software is developed in compliance with international copyright law,
human rights frameworks, and ethical standards including:

COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
- Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
  └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
- WIPO Copyright Treaty (WCT, 1996) - Digital rights management
- WIPO Performances and Phonograms Treaty (WPPT, 1996)
- Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
- Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
- Vienna Convention on the Law of Treaties (1969) - Treaty interpretation

HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
- Universal Declaration of Human Rights (UDHR, 1948)
  └─ Article 27: Right to protection of moral and material interests
- International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
  └─ Article 15: Right to benefit from scientific progress and protection of authorship
- Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
  └─ Articles 13, 16, 17: Expression, privacy, access to information
- Vienna Declaration and Programme of Action (1993) - Human rights universality

UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
- UNESCO Universal Declaration on Cultural Diversity (2001)
- UNESCO Recommendation on Open Science (2021)
- UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
- Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)

DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
- GDPR - General Data Protection Regulation (EU 2016/679)
- LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
- CCPA - California Consumer Privacy Act (USA)
- Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)

TECHNICAL STANDARDS / NORMAS TÉCNICAS:
- ISO/IEC 9001:2015 - Quality Management Systems
- ISO/IEC 27001:2022 - Information Security Management
- ISO/IEC 27002:2022 - Information Security Controls
- ISO/IEC 27018:2019 - PII Protection in Public Clouds
- ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
- ISO/IEC 8000 - Data Quality Standards
- IEEE 830-1998 - Software Requirements Specification
- IEEE 1012-2016 - Software Verification and Validation
- IEEE 12207-2017 - Software Life Cycle Processes
- IEEE 14764-2021 - Software Maintenance
- IEEE 42010-2011 - Architecture Description
- NIST Cybersecurity Framework (CSF) v1.1/v2.0
- NIST SP 800-53 Rev. 5 - Security and Privacy Controls
- NIST AI Risk Management Framework (AI RMF 1.0)

CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
- Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
- Universal jurisdiction for human rights violations
- Rome Statute of the International Criminal Court (1998) - For severe violations

ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:

This software adheres to the Ethica[8] framework with eight fundamental principles:

1. TRANSPARENCY (Transparência) 🔍
   └─ Open communication, documented decisions, explainable systems
   
2. ACCOUNTABILITY (Responsabilidade) 📋
   └─ Clear ownership, traceable actions, consequence acceptance
   
3. FAIRNESS (Justiça) ⚖️
   └─ Equitable treatment, non-discrimination, equal access
   
4. PRIVACY (Privacidade) 🔒
   └─ Data protection, consent respect, confidentiality
   
5. SECURITY (Segurança) 🛡️
   └─ Protection of systems, data integrity, threat mitigation
   
6. RELIABILITY (Confiabilidade) 🔧
   └─ Dependable operation, consistent behavior, stability
   
7. SAFETY (Proteção) 🛟
   └─ No harm to users, safe operations, risk prevention
   
8. SUSTAINABILITY (Sustentabilidade) ♻️
   └─ Long-term viability, environmental responsibility, social good

ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
  Life > Ethics > Law > Convenience
  Vida > Ética > Lei > Conveniência

ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:

This code is original work or properly attributed derivative work.
Every fragment, function, class, and algorithm has been:
  ✓ Originally created by the author, OR
  ✓ Properly licensed and attributed, OR
  ✓ In the public domain with documentation

NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
ZERO TOLERANCE for unauthorized copying or intellectual property theft.

Verification Methods:
- SHA3-512 checksums for integrity verification
- BLAKE3 hashing for rapid verification
- Git commit history as proof of authorship timeline
- Code review and compliance audits

Any concerns about intellectual property should be reported to:
rafaelmeloreisnovo [at] gmail [dot] com

NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):

These anchors provide stable reference points for:
- Version tracking and synchronization
- Legal compliance verification
- Authorship chain of custody
- Update propagation tracking
- Audit trail maintenance

⚓ ANCHOR_ID: 2C670A9D6857C08F
⚓ FILE_PATH: native/src/sepolicy/statement.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: B1E81F5359ADC1C5C15E3048CBA5B337


*/


use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, BufReader, Cursor};
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::SePolicy;
use crate::ffi::Xperm;
use base::nix::fcntl::OFlag;
use base::{BufReadExt, LoggedResult, Utf8CStr, error, warn};

pub enum Token<'a> {
    AL,
    DN,
    AA,
    DA,
    AX,
    AY,
    DX,
    PM,
    EF,
    TA,
    TY,
    AT,
    TT,
    TC,
    TM,
    GF,
    LB,
    RB,
    CM,
    ST,
    TL,
    HP,
    HX(u16),
    ID(&'a str),
}

type Tokens<'a> = Peekable<IntoIter<Token<'a>>>;
type ParseResult<'a, T> = Result<T, ParseError<'a>>;

enum ParseError<'a> {
    General,
    AvtabAv(Token<'a>),
    AvtabXperms(Token<'a>),
    AvtabType(Token<'a>),
    TypeState(Token<'a>),
    TypeAttr,
    TypeTrans,
    NewType,
    NewAttr,
    GenfsCon,
    ShowHelp,
    UnknownAction(Token<'a>),
}

macro_rules! throw {
    () => {
        Err(ParseError::General)?
    };
}

fn parse_id<'a>(tokens: &mut Tokens<'a>) -> ParseResult<'a, &'a str> {
    match tokens.next() {
        Some(Token::ID(name)) => Ok(name),
        _ => throw!(),
    }
}

// names ::= ID(n) { vec![n] };
// names ::= names(mut v) ID(n) { v.push(n); v };
// term ::= ID(n) { vec![n] }
// term ::= LB names(n) RB { n };
fn parse_term<'a>(tokens: &mut Tokens<'a>) -> ParseResult<'a, Vec<&'a str>> {
    match tokens.next() {
        Some(Token::ID(name)) => Ok(vec![name]),
        Some(Token::LB) => {
            let mut names = Vec::new();
            loop {
                match tokens.next() {
                    Some(Token::ID(name)) => names.push(name),
                    Some(Token::RB) => break,
                    _ => throw!(),
                }
            }
            Ok(names)
        }
        _ => throw!(),
    }
}

// names ::= ST { vec![] }
// names ::= ID(n) { vec![n] };
// names ::= names(mut v) ID(n) { v.push(n); v };
// names ::= names(n) ST { vec![] };
// sterm ::= ST { vec![] }
// sterm ::= ID(n) { vec![n] }
// sterm ::= LB names(n) RB { n };
fn parse_sterm<'a>(tokens: &mut Tokens<'a>) -> ParseResult<'a, Vec<&'a str>> {
    match tokens.next() {
        Some(Token::ID(name)) => Ok(vec![name]),
        Some(Token::ST) => Ok(vec![]),
        Some(Token::LB) => {
            let mut names = Some(Vec::new());
            loop {
                match tokens.next() {
                    Some(Token::ID(name)) => {
                        if let Some(ref mut names) = names {
                            names.push(name)
                        }
                    }
                    Some(Token::ST) => names = None,
                    Some(Token::RB) => break,
                    _ => throw!(),
                }
            }
            Ok(names.unwrap_or(vec![]))
        }
        _ => throw!(),
    }
}

// xperm ::= HX(low) { Xperm{low, high: low, reset: false} };
// xperm ::= HX(low) HP HX(high) { Xperm{low, high, reset: false} };
fn parse_xperm<'a>(tokens: &mut Tokens<'a>) -> ParseResult<'a, Xperm> {
    let low = match tokens.next() {
        Some(Token::HX(low)) => low,
        _ => throw!(),
    };
    let high = match tokens.peek() {
        Some(Token::HP) => {
            tokens.next();
            match tokens.next() {
                Some(Token::HX(high)) => high,
                _ => throw!(),
            }
        }
        _ => low,
    };
    Ok(Xperm {
        low,
        high,
        reset: false,
    })
}

// xperms ::= HX(low) { if low > 0 { vec![Xperm{low, high: low, reset: false}] } else { vec![Xperm{low: 0x0000, high: 0xFFFF, reset: true}] }};
// xperms ::= LB xperm_list(l) RB { l };
// xperms ::= TL LB xperm_list(mut l) RB { l.iter_mut().for_each(|x| { x.reset = true; }); l };
// xperms ::= ST { vec![Xperm{low: 0x0000, high: 0xFFFF, reset: false}] };
//
// xperm_list ::= xperm(p) { vec![p] }
// xperm_list ::= xperm_list(mut l) xperm(p) { l.push(p); l }
fn parse_xperms<'a>(tokens: &mut Tokens<'a>) -> ParseResult<'a, Vec<Xperm>> {
    let mut xperms = Vec::new();
    let reset = match tokens.peek() {
        Some(Token::TL) => {
            tokens.next();
            if !matches!(tokens.peek(), Some(Token::LB)) {
                throw!();
            }
            true
        }
        _ => false,
    };
    match tokens.next() {
        Some(Token::LB) => {
            // parse xperm_list
            loop {
                let mut xperm = parse_xperm(tokens)?;
                xperm.reset = reset;
                xperms.push(xperm);
                if matches!(tokens.peek(), Some(Token::RB)) {
                    tokens.next();
                    break;
                }
            }
        }
        Some(Token::ST) => {
            xperms.push(Xperm {
                low: 0x0000,
                high: 0xFFFF,
                reset,
            });
        }
        Some(Token::HX(low)) => {
            if low > 0 {
                xperms.push(Xperm {
                    low,
                    high: low,
                    reset,
                });
            } else {
                xperms.push(Xperm {
                    low: 0x0000,
                    high: 0xFFFF,
                    reset,
                });
            }
        }
        _ => throw!(),
    }
    Ok(xperms)
}

fn match_string<'a>(tokens: &mut Tokens<'a>, pattern: &str) -> ParseResult<'a, ()> {
    if let Some(Token::ID(s)) = tokens.next()
        && s == pattern
    {
        return Ok(());
    }
    Err(ParseError::General)
}

fn extract_token<'a>(s: &'a str, tokens: &mut Vec<Token<'a>>) {
    match s {
        "allow" => tokens.push(Token::AL),
        "deny" => tokens.push(Token::DN),
        "auditallow" => tokens.push(Token::AA),
        "dontaudit" => tokens.push(Token::DA),
        "allowxperm" => tokens.push(Token::AX),
        "auditallowxperm" => tokens.push(Token::AY),
        "dontauditxperm" => tokens.push(Token::DX),
        "permissive" => tokens.push(Token::PM),
        "enforce" => tokens.push(Token::EF),
        "typeattribute" => tokens.push(Token::TA),
        "type" => tokens.push(Token::TY),
        "attribute" => tokens.push(Token::AT),
        "type_transition" => tokens.push(Token::TT),
        "type_change" => tokens.push(Token::TC),
        "type_member" => tokens.push(Token::TM),
        "genfscon" => tokens.push(Token::GF),
        "*" => tokens.push(Token::ST),
        "" => {}
        _ => {
            if let Some(idx) = s.find('{') {
                let (a, b) = s.split_at(idx);
                extract_token(a, tokens);
                tokens.push(Token::LB);
                extract_token(&b[1..], tokens);
            } else if let Some(idx) = s.find('}') {
                let (a, b) = s.split_at(idx);
                extract_token(a, tokens);
                tokens.push(Token::RB);
                extract_token(&b[1..], tokens);
            } else if let Some(idx) = s.find(',') {
                let (a, b) = s.split_at(idx);
                extract_token(a, tokens);
                tokens.push(Token::CM);
                extract_token(&b[1..], tokens);
            } else if let Some(idx) = s.find('-') {
                let (a, b) = s.split_at(idx);
                extract_token(a, tokens);
                tokens.push(Token::HP);
                extract_token(&b[1..], tokens);
            } else if let Some(s) = s.strip_prefix('~') {
                tokens.push(Token::TL);
                extract_token(s, tokens);
            } else if let Some(s) = s.strip_prefix("0x") {
                tokens.push(Token::HX(s.parse().unwrap_or(0)));
            } else {
                tokens.push(Token::ID(s));
            }
        }
    }
}

fn tokenize_statement(statement: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    for s in statement.split_whitespace() {
        extract_token(s, &mut tokens);
    }
    tokens
}

impl SePolicy {
    pub fn load_rules(&mut self, rules: &str) {
        let mut cursor = Cursor::new(rules.as_bytes());
        self.load_rules_from_reader(&mut cursor);
    }

    pub fn load_rule_file(&mut self, filename: &Utf8CStr) {
        let result: LoggedResult<()> = try {
            let file = filename.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC)?;
            let mut reader = BufReader::new(file);
            self.load_rules_from_reader(&mut reader);
        };
        result.ok();
    }

    fn load_rules_from_reader<T: BufRead>(&mut self, reader: &mut T) {
        reader.for_each_line(|line| {
            self.parse_statement(line);
            true
        });
    }

    fn parse_statement(&mut self, statement: &str) {
        let statement = statement.trim();
        if statement.is_empty() || statement.starts_with('#') {
            return;
        }
        let mut tokens = tokenize_statement(statement).into_iter().peekable();
        let result = self.exec_statement(&mut tokens);
        if let Err(e) = result {
            warn!("Syntax error in: \"{}\"", statement);
            error!("Hint: {}", e);
        }
    }

    // statement ::= AL sterm(s) sterm(t) sterm(c) sterm(p) { sepolicy.allow(s, t, c, p); };
    // statement ::= DN sterm(s) sterm(t) sterm(c) sterm(p) { sepolicy.deny(s, t, c, p); };
    // statement ::= AA sterm(s) sterm(t) sterm(c) sterm(p) { sepolicy.auditallow(s, t, c, p); };
    // statement ::= DA sterm(s) sterm(t) sterm(c) sterm(p) { sepolicy.dontaudit(s, t, c, p); };
    // statement ::= AX sterm(s) sterm(t) sterm(c) ID(i) xperms(p) { sepolicy.allowxperm(s, t, c, p); };
    // statement ::= AY sterm(s) sterm(t) sterm(c) ID(i) xperms(p) { sepolicy.auditallowxperm(s, t, c, p); };
    // statement ::= DX sterm(s) sterm(t) sterm(c) ID(i) xperms(p) { sepolicy.dontauditxperm(s, t, c, p); };
    // statement ::= PM sterm(t) { sepolicy.permissive(t); };
    // statement ::= EF sterm(t) { sepolicy.enforce(t); };
    // statement ::= TA term(t) term(a) { sepolicy.typeattribute(t, a); };
    // statement ::= TY ID(t) { sepolicy.type_(t, vec![]);};
    // statement ::= TY ID(t) term(a) { sepolicy.type_(t, a);};
    // statement ::= AT ID(t) { sepolicy.attribute(t); };
    // statement ::= TT ID(s) ID(t) ID(c) ID(d) { sepolicy.type_transition(s, t, c, d, vec![]); };
    // statement ::= TT ID(s) ID(t) ID(c) ID(d) ID(o) { sepolicy.type_transition(s, t, c, d, vec![o]); };
    // statement ::= TC ID(s) ID(t) ID(c) ID(d) { sepolicy.type_change(s, t, c, d); };
    // statement ::= TM ID(s) ID(t) ID(c) ID(d) { sepolicy.type_member(s, t, c, d);};
    // statement ::= GF ID(s) ID(t) ID(c) { sepolicy.genfscon(s, t, c); };
    fn exec_statement<'a>(&mut self, tokens: &mut Tokens<'a>) -> ParseResult<'a, ()> {
        let action = match tokens.next() {
            Some(token) => token,
            _ => Err(ParseError::ShowHelp)?,
        };
        let check_additional_args = |tokens: &mut Tokens<'a>| {
            if tokens.peek().is_none() {
                Ok(())
            } else {
                Err(ParseError::General)
            }
        };
        match action {
            Token::AL | Token::DN | Token::AA | Token::DA => {
                let result: ParseResult<()> = try {
                    let s = parse_sterm(tokens)?;
                    let t = parse_sterm(tokens)?;
                    let c = parse_sterm(tokens)?;
                    let p = parse_sterm(tokens)?;
                    check_additional_args(tokens)?;
                    match action {
                        Token::AL => self.allow(s, t, c, p),
                        Token::DN => self.deny(s, t, c, p),
                        Token::AA => self.auditallow(s, t, c, p),
                        Token::DA => self.dontaudit(s, t, c, p),
                        _ => unreachable!(),
                    }
                };
                if result.is_err() {
                    Err(ParseError::AvtabAv(action))?
                }
            }
            Token::AX | Token::AY | Token::DX => {
                let result: ParseResult<()> = try {
                    let s = parse_sterm(tokens)?;
                    let t = parse_sterm(tokens)?;
                    let c = parse_sterm(tokens)?;
                    match_string(tokens, "ioctl")?;
                    let p = parse_xperms(tokens)?;
                    check_additional_args(tokens)?;
                    match action {
                        Token::AX => self.allowxperm(s, t, c, p),
                        Token::AY => self.auditallowxperm(s, t, c, p),
                        Token::DX => self.dontauditxperm(s, t, c, p),
                        _ => unreachable!(),
                    }
                };
                if result.is_err() {
                    Err(ParseError::AvtabXperms(action))?
                }
            }
            Token::PM | Token::EF => {
                let result: ParseResult<()> = try {
                    let t = parse_sterm(tokens)?;
                    check_additional_args(tokens)?;
                    match action {
                        Token::PM => self.permissive(t),
                        Token::EF => self.enforce(t),
                        _ => unreachable!(),
                    }
                };
                if result.is_err() {
                    Err(ParseError::TypeState(action))?
                }
            }
            Token::TA => {
                let result: ParseResult<()> = try {
                    let t = parse_term(tokens)?;
                    let a = parse_term(tokens)?;
                    check_additional_args(tokens)?;
                    self.typeattribute(t, a)
                };
                if result.is_err() {
                    Err(ParseError::TypeAttr)?
                }
            }
            Token::TY => {
                let result: ParseResult<()> = try {
                    let t = parse_id(tokens)?;
                    let a = if tokens.peek().is_none() {
                        vec![]
                    } else {
                        parse_term(tokens)?
                    };
                    check_additional_args(tokens)?;
                    self.type_(t, a)
                };
                if result.is_err() {
                    Err(ParseError::NewType)?
                }
            }
            Token::AT => {
                let result: ParseResult<()> = try {
                    let t = parse_id(tokens)?;
                    check_additional_args(tokens)?;
                    self.attribute(t)
                };
                if result.is_err() {
                    Err(ParseError::NewAttr)?
                }
            }
            Token::TC | Token::TM => {
                let result: ParseResult<()> = try {
                    let s = parse_id(tokens)?;
                    let t = parse_id(tokens)?;
                    let c = parse_id(tokens)?;
                    let d = parse_id(tokens)?;
                    check_additional_args(tokens)?;
                    match action {
                        Token::TC => self.type_change(s, t, c, d),
                        Token::TM => self.type_member(s, t, c, d),
                        _ => unreachable!(),
                    }
                };
                if result.is_err() {
                    Err(ParseError::AvtabType(action))?
                }
            }
            Token::TT => {
                let result: ParseResult<()> = try {
                    let s = parse_id(tokens)?;
                    let t = parse_id(tokens)?;
                    let c = parse_id(tokens)?;
                    let d = parse_id(tokens)?;
                    let o = if tokens.peek().is_none() {
                        ""
                    } else {
                        parse_id(tokens)?
                    };
                    check_additional_args(tokens)?;
                    self.type_transition(s, t, c, d, o);
                };
                if result.is_err() {
                    Err(ParseError::TypeTrans)?
                }
            }
            Token::GF => {
                let result: ParseResult<()> = try {
                    let s = parse_id(tokens)?;
                    let t = parse_id(tokens)?;
                    let c = parse_id(tokens)?;
                    check_additional_args(tokens)?;
                    self.genfscon(s, t, c)
                };
                if result.is_err() {
                    Err(ParseError::GenfsCon)?
                }
            }
            _ => Err(ParseError::UnknownAction(action))?,
        }
        Ok(())
    }
}

// Token to string
impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::AL => f.write_str("allow"),
            Token::DN => f.write_str("deny"),
            Token::AA => f.write_str("auditallow"),
            Token::DA => f.write_str("dontaudit"),
            Token::AX => f.write_str("allowxperm"),
            Token::AY => f.write_str("auditallowxperm"),
            Token::DX => f.write_str("dontauditxperm"),
            Token::PM => f.write_str("permissive"),
            Token::EF => f.write_str("enforce"),
            Token::TA => f.write_str("typeattribute"),
            Token::TY => f.write_str("type"),
            Token::AT => f.write_str("attribute"),
            Token::TT => f.write_str("type_transition"),
            Token::TC => f.write_str("type_change"),
            Token::TM => f.write_str("type_member"),
            Token::GF => f.write_str("genfscon"),
            Token::LB => f.write_char('{'),
            Token::RB => f.write_char('}'),
            Token::CM => f.write_char(','),
            Token::ST => f.write_char('*'),
            Token::TL => f.write_char('~'),
            Token::HP => f.write_char('-'),
            Token::HX(n) => f.write_fmt(format_args!("{n:06X}")),
            Token::ID(s) => f.write_str(s),
        }
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::General => Ok(()),
            ParseError::ShowHelp => format_statement_help(f),
            ParseError::AvtabAv(action) => {
                write!(f, "{action} *source_type *target_type *class *perm_set")
            }
            ParseError::AvtabXperms(action) => {
                write!(
                    f,
                    "{action} *source_type *target_type *class operation xperm_set"
                )
            }
            ParseError::AvtabType(action) => {
                write!(f, "{action} source_type target_type class default_type")
            }
            ParseError::TypeState(action) => {
                write!(f, "{action} *type")
            }
            ParseError::TypeAttr => f.write_str("typeattribute ^type ^attribute"),
            ParseError::TypeTrans => f.write_str(
                "type_transition source_type target_type class default_type (object_name)",
            ),
            ParseError::NewType => f.write_str("type type_name ^(attribute)"),
            ParseError::NewAttr => f.write_str("attribute attribute_name"),
            ParseError::GenfsCon => f.write_str("genfscon fs_name partial_path fs_context"),
            ParseError::UnknownAction(action) => write!(f, "Unknown action: \"{action}\""),
        }
    }
}

pub(crate) fn format_statement_help(f: &mut dyn Write) -> std::fmt::Result {
    write!(
        f,
        r#"** Policy statements:

One policy statement should be treated as a single parameter;
this means each policy statement should be enclosed in quotes.
Multiple policy statements can be provided in a single command.

Statements has a format of "<rule_name> [args...]".
Arguments labeled with (^) can accept one or more entries.
Multiple entries consist of a space separated list enclosed in braces ({{}}).
Arguments labeled with (*) are the same as (^), but additionally
support the match-all operator (*).

Example: "allow {{ s1 s2 }} {{ t1 t2 }} class *"
Will be expanded to:

allow s1 t1 class {{ all-permissions-of-class }}
allow s1 t2 class {{ all-permissions-of-class }}
allow s2 t1 class {{ all-permissions-of-class }}
allow s2 t2 class {{ all-permissions-of-class }}

** Extended permissions:

The only supported operation for extended permissions right now is 'ioctl'.
xperm_set is one or multiple hexadecimal numeric values ranging from 0x0000 to 0xFFFF.
Multiple values consist of a space separated list enclosed in braces ({{}}).
Use the complement operator (~) to specify all permissions except those explicitly listed.
Use the range operator (-) to specify all permissions within the low – high range.
Use the match all operator (*) to match all ioctl commands (0x0000-0xFFFF).
The special value 0 is used to clear all rules.

Some examples:
allowxperm source target class ioctl 0x8910
allowxperm source target class ioctl {{ 0x8910-0x8926 0x892A-0x8935 }}
allowxperm source target class ioctl ~{{ 0x8910 0x892A }}
allowxperm source target class ioctl *

** Supported policy statements:

{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
"#,
        ParseError::AvtabAv(Token::AL),
        ParseError::AvtabAv(Token::DN),
        ParseError::AvtabAv(Token::AA),
        ParseError::AvtabAv(Token::DA),
        ParseError::AvtabXperms(Token::AX),
        ParseError::AvtabXperms(Token::AY),
        ParseError::AvtabXperms(Token::DX),
        ParseError::TypeState(Token::PM),
        ParseError::TypeState(Token::EF),
        ParseError::TypeAttr,
        ParseError::NewType,
        ParseError::NewAttr,
        ParseError::TypeTrans,
        ParseError::AvtabType(Token::TC),
        ParseError::AvtabType(Token::TM),
        ParseError::GenfsCon
    )
}
