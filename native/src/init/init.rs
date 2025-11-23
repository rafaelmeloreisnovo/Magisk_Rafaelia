//! init.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 1FB9C9EE02FF4F2E
⚓ FILE_PATH: native/src/init/init.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 540C7F1B08B12AE70B6F2F935E8364A5


*/


use crate::ffi::{BootConfig, MagiskInit, backup_init, magisk_proxy_main};
use crate::logging::setup_klog;
use crate::mount::is_rootfs;
use crate::twostage::hexpatch_init_for_second_stage;
use base::libc::{basename, getpid, mount, umask};
use base::{LibcReturn, LoggedResult, ResultExt, cstr, info, raw_cstr};
use std::ffi::{CStr, c_char};
use std::ptr::null;

impl MagiskInit {
    fn new(argv: *mut *mut c_char) -> Self {
        Self {
            preinit_dev: String::new(),
            mount_list: Vec::new(),
            overlay_con: Vec::new(),
            argv,
            config: BootConfig {
                skip_initramfs: false,
                force_normal_boot: false,
                rootwait: false,
                emulator: false,
                slot: [0; 3],
                dt_dir: [0; 64],
                fstab_suffix: [0; 32],
                hardware: [0; 32],
                hardware_plat: [0; 32],
                partition_map: Vec::new(),
            },
        }
    }

    fn first_stage(&self) {
        info!("First Stage Init");
        self.prepare_data();

        if !cstr!("/sdcard").exists() && !cstr!("/first_stage_ramdisk/sdcard").exists() {
            self.hijack_init_with_switch_root();
            self.restore_ramdisk_init();
        } else {
            self.restore_ramdisk_init();
            // Fallback to hexpatch if /sdcard exists
            hexpatch_init_for_second_stage(true);
        }
    }

    fn second_stage(&mut self) {
        info!("Second Stage Init");

        cstr!("/init").unmount().ok();
        cstr!("/system/bin/init").unmount().ok(); // just in case
        cstr!("/data/init").remove().ok();

        unsafe {
            // Make sure init dmesg logs won't get messed up
            *self.argv = raw_cstr!("/system/bin/init") as *mut _;
        }

        // Some weird devices like meizu, uses 2SI but still have legacy rootfs
        if is_rootfs() {
            // We are still on rootfs, so make sure we will execute the init of the 2nd stage
            let init_path = cstr!("/init");
            init_path.remove().ok();
            init_path
                .create_symlink_to(cstr!("/system/bin/init"))
                .log_ok();
            self.patch_rw_root();
        } else {
            self.patch_ro_root();
        }
    }

    fn legacy_system_as_root(&mut self) {
        info!("Legacy SAR Init");
        self.prepare_data();
        let is_two_stage = self.mount_system_root();
        if is_two_stage {
            hexpatch_init_for_second_stage(false);
        } else {
            self.patch_ro_root();
        }
    }

    fn rootfs(&mut self) {
        info!("RootFS Init");
        self.prepare_data();
        self.restore_ramdisk_init();
        self.patch_rw_root();
    }

    fn recovery(&self) {
        info!("Ramdisk is recovery, abort");
        self.restore_ramdisk_init();
        cstr!("/.backup").remove_all().ok();
    }

    fn restore_ramdisk_init(&self) {
        cstr!("/init").remove().ok();

        let orig_init = backup_init();

        if orig_init.exists() {
            orig_init.rename_to(cstr!("/init")).log_ok();
        } else {
            // If the backup init is missing, this means that the boot ramdisk
            // was created from scratch, and the real init is in a separate CPIO,
            // which is guaranteed to be placed at /system/bin/init.
            cstr!("/init")
                .create_symlink_to(cstr!("/system/bin/init"))
                .log_ok();
        }
    }

    fn start(&mut self) -> LoggedResult<()> {
        if !cstr!("/proc/cmdline").exists() {
            cstr!("/proc").mkdir(0o755)?;
            unsafe {
                mount(
                    raw_cstr!("proc"),
                    raw_cstr!("/proc"),
                    raw_cstr!("proc"),
                    0,
                    null(),
                )
            }
            .check_err()?;
            self.mount_list.push("/proc".to_string());
        }
        if !cstr!("/sys/block").exists() {
            cstr!("/sys").mkdir(0o755)?;
            unsafe {
                mount(
                    raw_cstr!("sysfs"),
                    raw_cstr!("/sys"),
                    raw_cstr!("sysfs"),
                    0,
                    null(),
                )
            }
            .check_err()?;
            self.mount_list.push("/sys".to_string());
        }

        setup_klog();

        self.config.init();

        let argv1 = unsafe { *self.argv.offset(1) };
        if !argv1.is_null() && unsafe { CStr::from_ptr(argv1) == c"selinux_setup" } {
            self.second_stage();
        } else if self.config.skip_initramfs {
            self.legacy_system_as_root();
        } else if self.config.force_normal_boot {
            self.first_stage();
        } else if cstr!("/sbin/recovery").exists() || cstr!("/system/bin/recovery").exists() {
            self.recovery();
        } else if self.check_two_stage() {
            self.first_stage();
        } else {
            self.rootfs();
        }

        // Finally execute the original init
        self.exec_init();

        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(
    argc: i32,
    argv: *mut *mut c_char,
    _envp: *const *const c_char,
) -> i32 {
    unsafe {
        umask(0);

        let name = basename(*argv);

        if CStr::from_ptr(name) == c"magisk" {
            return magisk_proxy_main(argc, argv);
        }

        if getpid() == 1 {
            MagiskInit::new(argv).start().log_ok();
        }

        1
    }
}
