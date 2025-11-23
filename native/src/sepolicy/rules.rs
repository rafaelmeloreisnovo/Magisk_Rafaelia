//! rules.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 2EEDC44D9388A7F5
⚓ FILE_PATH: native/src/sepolicy/rules.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 5462922436F603E9423B2B17BE8499CF


*/


use crate::SePolicy;
use crate::consts::{SEPOL_FILE_TYPE, SEPOL_LOG_TYPE, SEPOL_PROC_DOMAIN};
use crate::ffi::Xperm;
use base::{LogLevel, set_log_level_state};

macro_rules! rules {
    (@args all) => {
        vec![]
    };
    (@args xall) => {
        vec![Xperm { low: 0x0000, high: 0xFFFF, reset: false }]
    };
    (@args svcmgr) => {
        vec!["servicemanager", "vndservicemanager", "hwservicemanager"]
    };
    (@args [proc]) => {
        vec![SEPOL_PROC_DOMAIN]
    };
    (@args [file]) => {
        vec![SEPOL_FILE_TYPE]
    };
    (@args [log]) => {
        vec![SEPOL_LOG_TYPE]
    };
    (@args proc) => {
        SEPOL_PROC_DOMAIN
    };
    (@args file) => {
        SEPOL_FILE_TYPE
    };
    (@args log) => {
        SEPOL_LOG_TYPE
    };
    (@args [$($arg:tt)*]) => {
        vec![$($arg)*]
    };
    (@args $arg:expr) => {
        $arg
    };
    (@stmt $self:ident) => {};
    (@stmt $self:ident $action:ident($($args:tt),*); $($res:tt)*) => {
        $self.$action($(rules!(@args $args)),*);
        rules!{@stmt $self $($res)* }
    };
    (use $self:ident; $($res:tt)*) => {{
        rules!{@stmt $self $($res)* }
    }};
}

impl SePolicy {
    pub fn magisk_rules(&mut self) {
        // Temp suppress warnings
        set_log_level_state(LogLevel::Warn, false);
        rules! {
            use self;
            // Prevent anything to change sepolicy except ourselves
            deny(all, ["kernel"], ["security"], ["load_policy"]);
            type_(proc, ["domain"]);
            typeattribute([proc], ["mlstrustedsubject", "netdomain", "appdomain"]);
            type_(file, ["file_type"]);
            typeattribute([file], ["mlstrustedobject"]);
            type_(log, ["file_type"]);
            typeattribute([log], ["mlstrustedobject"]);

            // Create unconstrained file type
            allow(["domain"], [file],
                ["file", "dir", "fifo_file", "chr_file", "lnk_file", "sock_file"], all);

            // Only allow zygote to open log pipe
            allow(["zygote"], [log], ["fifo_file"], ["open", "read"]);
            // Allow all processes to output logs
            allow(["domain"], [log], ["fifo_file"], ["write"]);

            // Make our root domain unconstrained
            allow([proc], [
                "fs_type", "dev_type", "file_type", "domain",
                "service_manager_type", "hwservice_manager_type", "vndservice_manager_type",
                "port_type", "node_type", "property_type"
            ], all, all);

            // Just in case, make the domain permissive
            permissive([proc]);

            // Allow us to do any ioctl
            allowxperm([proc], ["fs_type", "dev_type", "file_type", "domain"],
                ["blk_file", "fifo_file", "chr_file"], xall);
            allowxperm([proc], [proc], ["tcp_socket", "udp_socket", "rawip_socket"], xall);

            // Let binder work with our processes
            allow(svcmgr, [proc], ["dir"], ["search"]);
            allow(svcmgr, [proc], ["file"], ["open", "read", "map"]);
            allow(svcmgr, [proc], ["process"], ["getattr"]);
            allow(["domain"], [proc], ["binder"], ["call", "transfer"]);

            // Other common IPC
            allow(["domain"], [proc], ["process"], ["sigchld"]);
            allow(["domain"], [proc], ["fd"], ["use"]);
            allow(["domain"], [proc], ["fifo_file"], ["write", "read", "open", "getattr"]);

            // Allow these processes to access MagiskSU and output logs
            allow(["zygote", "shell", "platform_app",
                "system_app", "priv_app", "untrusted_app", "untrusted_app_all"],
                [proc], ["unix_stream_socket"], ["connectto", "getopt"]);

            // Let selected domains access tmpfs files
            // For tmpfs overlay on 2SI, Zygisk on lower Android versions and AVD scripts
            allow(["init", "zygote", "shell"], ["tmpfs"], ["file"], all);

            // Allow magiskinit daemon to log to kmsg
            allow(["kernel"], ["rootfs", "tmpfs"], ["chr_file"], ["write"]);

            // Allow magiskinit daemon to handle mock selinuxfs
            allow(["kernel"], ["tmpfs"], ["fifo_file"], ["open", "read", "write"]);

            // For relabelling files
            allow(["rootfs"], ["labeledfs", "tmpfs"], ["filesystem"], ["associate"]);
            allow([file], ["pipefs", "devpts"], ["filesystem"], ["associate"]);
            allow(["kernel"], all, ["file"], ["relabelto"]);
            allow(["kernel"], ["tmpfs"], ["file"], ["relabelfrom"]);

            // Let init transit to SEPOL_PROC_DOMAIN
            allow(["kernel"], ["kernel"], ["process"], ["setcurrent"]);
            allow(["kernel"], [proc], ["process"], ["dyntransition"]);

            // Let init run stuffs
            allow(["init"], [proc], ["process"], all);

            // For mounting loop devices, mirrors, tmpfs
            allow(["kernel"], ["fs_type", "dev_type", "file_type"], ["file"], ["read", "write"]);

            // Zygisk rules
            allow(["zygote"], ["zygote"], ["process"], ["execmem"]);
            allow(["zygote"], ["fs_type"], ["filesystem"], ["unmount"]);
            allow(["system_server"], ["system_server"], ["process"], ["execmem"]);

            // Shut llkd up
            dontaudit(["llkd"], [proc], ["process"], ["ptrace"]);

            // Keep /data/adb/* context
            deny(["init"], ["adb_data_file"], ["dir"], ["search"]);
            deny(["vendor_init"], ["adb_data_file"], ["dir"], ["search"]);
        }

        #[cfg(any())]
        self.strip_dontaudit();

        set_log_level_state(LogLevel::Warn, true);
    }
}
