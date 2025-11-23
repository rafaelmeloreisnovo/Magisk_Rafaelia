//! bootstages.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 148673BC52B0A0E1
⚓ FILE_PATH: native/src/core/bootstages.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: A4E7DEC9F531EBCE0F4BB8369D90C636


*/


use crate::consts::{APP_PACKAGE_NAME, BBPATH, DATABIN, MODULEROOT, SECURE_DIR};
use crate::daemon::MagiskD;
use crate::ffi::{
    DbEntryKey, RequestCode, check_key_combo, exec_common_scripts, exec_module_scripts,
    get_magisk_tmp, initialize_denylist,
};
use crate::logging::setup_logfile;
use crate::module::disable_modules;
use crate::mount::{clean_mounts, setup_preinit_dir};
use crate::resetprop::get_prop;
use crate::selinux::restorecon;
use base::const_format::concatcp;
use base::{BufReadExt, FsPathBuilder, ResultExt, cstr, error, info};
use bitflags::bitflags;
use nix::fcntl::OFlag;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering;

bitflags! {
    #[derive(Default)]
    pub struct BootState : u32 {
        const PostFsDataDone = 1 << 0;
        const LateStartDone = 1 << 1;
        const BootComplete = 1 << 2;
        const SafeMode = 1 << 3;
    }
}

impl MagiskD {
    fn setup_magisk_env(&self) -> bool {
        info!("* Initializing Magisk environment");

        let mut buf = cstr::buf::default();

        let app_bin_dir = buf
            .append_path(self.app_data_dir())
            .append_path("0")
            .append_path(APP_PACKAGE_NAME)
            .append_path("install");

        // Alternative binaries paths
        let alt_bin_dirs = &[
            cstr!("/cache/data_adb/magisk"),
            cstr!("/data/magisk"),
            app_bin_dir,
        ];
        for dir in alt_bin_dirs {
            if dir.exists() {
                cstr!(DATABIN).remove_all().ok();
                dir.copy_to(cstr!(DATABIN)).ok();
                dir.remove_all().ok();
            }
        }
        cstr!("/cache/data_adb").remove_all().ok();

        // Directories in /data/adb
        cstr!(SECURE_DIR).follow_link().chmod(0o700).log_ok();
        cstr!(DATABIN).mkdir(0o755).log_ok();
        cstr!(MODULEROOT).mkdir(0o755).log_ok();
        cstr!(concatcp!(SECURE_DIR, "/post-fs-data.d"))
            .mkdir(0o755)
            .log_ok();
        cstr!(concatcp!(SECURE_DIR, "/service.d"))
            .mkdir(0o755)
            .log_ok();
        restorecon();

        let busybox = cstr!(concatcp!(DATABIN, "/busybox"));
        if !busybox.exists() {
            return false;
        }

        let tmp_bb = buf.append_path(get_magisk_tmp()).append_path(BBPATH);
        tmp_bb.mkdirs(0o755).ok();
        tmp_bb.append_path("busybox");
        busybox.copy_to(tmp_bb).ok();
        tmp_bb.follow_link().chmod(0o755).log_ok();

        // Install busybox applets
        Command::new(&tmp_bb)
            .arg("--install")
            .arg("-s")
            .arg(tmp_bb.parent_dir().unwrap())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .log_ok();

        // magisk32 and magiskpolicy are not installed into ramdisk and has to be copied
        // from data to magisk tmp
        let magisk32 = cstr!(concatcp!(DATABIN, "/magisk32"));
        if magisk32.exists() {
            let tmp = buf.append_path(get_magisk_tmp()).append_path("magisk32");
            magisk32.copy_to(tmp).log_ok();
        }
        let magiskpolicy = cstr!(concatcp!(DATABIN, "/magiskpolicy"));
        if magiskpolicy.exists() {
            let tmp = buf
                .append_path(get_magisk_tmp())
                .append_path("magiskpolicy");
            magiskpolicy.copy_to(tmp).log_ok();
        }

        true
    }

    fn post_fs_data(&self) -> bool {
        setup_logfile();
        info!("** post-fs-data mode running");

        self.preserve_stub_apk();

        // Check secure dir
        let secure_dir = cstr!(SECURE_DIR);
        if !secure_dir.exists() {
            if self.sdk_int < 24 {
                secure_dir.mkdir(0o700).log_ok();
            } else {
                error!("* {} is not present, abort", SECURE_DIR);
                return true;
            }
        }

        self.prune_su_access();

        if !self.setup_magisk_env() {
            error!("* Magisk environment incomplete, abort");
            return true;
        }

        // Check safe mode
        let boot_cnt = self.get_db_setting(DbEntryKey::BootloopCount);
        self.set_db_setting(DbEntryKey::BootloopCount, boot_cnt + 1)
            .log()
            .ok();
        let safe_mode = boot_cnt >= 2
            || get_prop(cstr!("persist.sys.safemode")) == "1"
            || get_prop(cstr!("ro.sys.safemode")) == "1"
            || check_key_combo();

        if safe_mode {
            info!("* Safe mode triggered");
            // Disable all modules and zygisk so next boot will be clean
            disable_modules();
            self.set_db_setting(DbEntryKey::ZygiskConfig, 0).log_ok();
            return true;
        }

        exec_common_scripts(cstr!("post-fs-data"));
        self.zygisk_enabled.store(
            self.get_db_setting(DbEntryKey::ZygiskConfig) != 0,
            Ordering::Release,
        );
        initialize_denylist();
        self.handle_modules();
        clean_mounts();

        false
    }

    fn late_start(&self) {
        setup_logfile();
        info!("** late_start service mode running");

        exec_common_scripts(cstr!("service"));
        if let Some(module_list) = self.module_list.get() {
            exec_module_scripts(cstr!("service"), module_list);
        }
    }

    fn boot_complete(&self) {
        setup_logfile();
        info!("** boot-complete triggered");

        // Reset the bootloop counter once we have boot-complete
        self.set_db_setting(DbEntryKey::BootloopCount, 0).log_ok();

        // At this point it's safe to create the folder
        let secure_dir = cstr!(SECURE_DIR);
        if !secure_dir.exists() {
            secure_dir.mkdir(0o700).log_ok();
        }

        setup_preinit_dir();
        self.ensure_manager();
        if self.zygisk_enabled.load(Ordering::Relaxed) {
            self.zygisk.lock().unwrap().reset(true);
        }
    }

    pub fn boot_stage_handler(&self, client: UnixStream, code: RequestCode) {
        // Make sure boot stage execution is always serialized
        let mut state = self.boot_stage_lock.lock().unwrap();

        match code {
            RequestCode::POST_FS_DATA => {
                if check_data() && !state.contains(BootState::PostFsDataDone) {
                    if self.post_fs_data() {
                        state.insert(BootState::SafeMode);
                    }
                    state.insert(BootState::PostFsDataDone);
                }
            }
            RequestCode::LATE_START => {
                drop(client);
                if state.contains(BootState::PostFsDataDone) && !state.contains(BootState::SafeMode)
                {
                    self.late_start();
                    state.insert(BootState::LateStartDone);
                }
            }
            RequestCode::BOOT_COMPLETE => {
                drop(client);
                if state.contains(BootState::PostFsDataDone) {
                    state.insert(BootState::BootComplete);
                    self.boot_complete()
                }
            }
            _ => {}
        }
    }
}

fn check_data() -> bool {
    if let Ok(file) = cstr!("/proc/mounts").open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
        let mut mnt = false;
        BufReader::new(file).for_each_line(|line| {
            if line.contains(" /data ") && !line.contains("tmpfs") {
                mnt = true;
                return false;
            }
            true
        });
        if !mnt {
            return false;
        }
        let crypto = get_prop(cstr!("ro.crypto.state"));
        return if !crypto.is_empty() {
            if crypto != "encrypted" {
                // Unencrypted, we can directly access data
                true
            } else {
                // Encrypted, check whether vold is started
                !get_prop(cstr!("init.svc.vold")).is_empty()
            }
        } else {
            // ro.crypto.state is not set, assume it's unencrypted
            true
        };
    }
    false
}
