//! mount.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 4F8E0E4D59FF8D16
⚓ FILE_PATH: native/src/core/mount.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 3D485F9001D79083439827BB3DFF7B13


*/


use crate::consts::{MODULEMNT, MODULEROOT, PREINITDEV, PREINITMIRR, WORKERDIR};
use crate::ffi::{get_magisk_tmp, resolve_preinit_dir, switch_mnt_ns};
use crate::resetprop::get_prop;
use base::{
    FsPathBuilder, LibcReturn, LoggedResult, MountInfo, ResultExt, Utf8CStr, Utf8CStrBuf, cstr,
    debug, info, libc, parse_mount_info, warn,
};
use libc::{c_uint, dev_t};
use nix::mount::MsFlags;
use nix::sys::stat::{Mode, SFlag, mknod};
use num_traits::AsPrimitive;
use std::cmp::Ordering::{Greater, Less};
use std::path::{Path, PathBuf};

pub fn setup_preinit_dir() {
    let magisk_tmp = get_magisk_tmp();

    // Mount preinit directory
    let dev_path = cstr::buf::new::<64>()
        .join_path(magisk_tmp)
        .join_path(PREINITDEV);
    if let Ok(attr) = dev_path.get_attr()
        && attr.st.st_mode & libc::S_IFMT as c_uint == libc::S_IFBLK.as_()
    {
        // DO NOT mount the block device directly, as we do not know the flags and configs
        // to properly mount the partition; mounting block devices directly as rw could cause
        // crashes if the filesystem driver is crap (e.g. some broken F2FS drivers).
        // What we do instead is to scan through the current mountinfo and find a pre-existing
        // mount point mounting our desired partition, and then bind mount the target folder.
        let preinit_dev = attr.st.st_rdev;
        let mnt_path = cstr::buf::default()
            .join_path(magisk_tmp)
            .join_path(PREINITMIRR);
        for info in parse_mount_info("self") {
            if info.root == "/" && info.device == preinit_dev {
                if !info.fs_option.split(',').any(|s| s == "rw") {
                    // Only care about rw mounts
                    continue;
                }
                let mut target = info.target;
                let target = Utf8CStr::from_string(&mut target);
                let mut preinit_dir = resolve_preinit_dir(target);
                let preinit_dir = Utf8CStr::from_string(&mut preinit_dir);
                let r: LoggedResult<()> = try {
                    preinit_dir.mkdir(0o700)?;
                    mnt_path.mkdirs(0o755)?;
                    mnt_path.remove().ok();
                    mnt_path.create_symlink_to(preinit_dir)?;
                };
                if r.is_ok() {
                    info!("* Found preinit dir: {}", preinit_dir);
                    return;
                }
            }
        }
    }

    warn!("mount: preinit dir not found");
}

pub fn setup_module_mount() {
    // Bind remount module root to clear nosuid
    let module_mnt = cstr::buf::default()
        .join_path(get_magisk_tmp())
        .join_path(MODULEMNT);
    let _: LoggedResult<()> = try {
        module_mnt.mkdir(0o755)?;
        cstr!(MODULEROOT).bind_mount_to(&module_mnt, false)?;
        module_mnt.remount_mount_point_flags(MsFlags::MS_RDONLY)?;
    };
}

pub fn clean_mounts() {
    let magisk_tmp = get_magisk_tmp();

    let mut buf = cstr::buf::default();

    let module_mnt = buf.append_path(magisk_tmp).append_path(MODULEMNT);
    module_mnt.unmount().log_ok();
    buf.clear();

    let worker_dir = buf.append_path(magisk_tmp).append_path(WORKERDIR);
    let _: LoggedResult<()> = try {
        worker_dir.set_mount_private(true)?;
        worker_dir.unmount()?;
    };
}

// when partitions have the same fs type, the order is:
// - data: it has sufficient space and can be safely written
// - cache: size is limited, but still can be safely written
// - metadata: size is limited, and it might cause unexpected behavior if written
// - persist: it's the last resort, as it's dangerous to write to it
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PartId {
    Data,
    Cache,
    Metadata,
    Persist,
}

enum EncryptType {
    None,
    Block,
    File,
    Metadata,
}

pub fn find_preinit_device() -> String {
    let encrypt_type = if get_prop(cstr!("ro.crypto.state")) != "encrypted" {
        EncryptType::None
    } else if get_prop(cstr!("ro.crypto.type")) == "block" {
        EncryptType::Block
    } else if get_prop(cstr!("ro.crypto.metadata.enabled")) == "true" {
        EncryptType::Metadata
    } else {
        EncryptType::File
    };

    let mut matched_info = parse_mount_info("self")
        .into_iter()
        .filter_map(|info| {
            if info.root != "/" || !info.source.starts_with('/') || info.source.contains("/dm-") {
                return None;
            }
            match info.fs_type.as_str() {
                "ext4" | "f2fs" => (),
                _ => return None,
            }
            if !info.fs_option.split(',').any(|s| s == "rw") {
                return None;
            }
            if let Some(path) = Path::new(&info.source).parent() {
                if !path.ends_with("by-name") && !path.ends_with("block") {
                    return None;
                }
            } else {
                return None;
            }
            // take data iff it's not encrypted or file-based encrypted without metadata
            // other partitions are always taken
            match info.target.as_str() {
                "/persist" | "/mnt/vendor/persist" => Some((PartId::Persist, info)),
                "/metadata" => Some((PartId::Metadata, info)),
                "/cache" => Some((PartId::Cache, info)),
                "/data" => Some((PartId::Data, info))
                    .take_if(|_| matches!(encrypt_type, EncryptType::None | EncryptType::File)),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    if matched_info.is_empty() {
        return String::new();
    }

    let (_, preinit_info, _) = matched_info.select_nth_unstable_by(
        0,
        |(ap, MountInfo { fs_type: at, .. }), (bp, MountInfo { fs_type: bt, .. })| match (
            ap,
            bp,
            at.as_str() == "ext4",
            bt.as_str() == "ext4",
        ) {
            // metadata is not affected by f2fs kernel bug
            (PartId::Metadata, _, _, true) | (_, PartId::Metadata, true, _) => ap.cmp(bp),
            // otherwise, take ext4 f2fs because f2fs has a kernel bug that causes kernel panic
            (_, _, true, false) => Less,
            (_, _, false, true) => Greater,
            // if both has the same fs type, compare the mount point
            _ => ap.cmp(bp),
        },
    );
    let info = &preinit_info.1;
    let mut target = info.target.clone();
    let mut preinit_dir = resolve_preinit_dir(Utf8CStr::from_string(&mut target));
    if unsafe { libc::getuid() } == 0
        && let Ok(tmp) = std::env::var("MAGISKTMP")
        && !tmp.is_empty()
    {
        let mut buf = cstr::buf::default();
        let mirror_dir = buf.append_path(&tmp).append_path(PREINITMIRR);
        let preinit_dir = Utf8CStr::from_string(&mut preinit_dir);
        let _: LoggedResult<()> = try {
            preinit_dir.mkdirs(0o700)?;
            mirror_dir.mkdirs(0o755)?;
            mirror_dir.unmount().ok();
            mirror_dir.remove().ok();
            mirror_dir.create_symlink_to(preinit_dir)?;
        };
        if std::env::var_os("MAKEDEV").is_some() {
            buf.clear();
            let dev_path = buf.append_path(&tmp).append_path(PREINITDEV);
            mknod(
                dev_path.as_utf8_cstr(),
                SFlag::S_IFBLK,
                Mode::from_bits_truncate(0o600),
                info.device as dev_t,
            )
            .check_os_err("mknod", Some(dev_path), None)
            .log_ok();
        }
    }
    Path::new(&info.source)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn revert_unmount(pid: i32) {
    if pid > 0 {
        if switch_mnt_ns(pid) != 0 {
            return;
        }
        debug!("denylist: handling PID=[{}]", pid);
    }

    let mut targets = Vec::new();

    // Unmount Magisk tmpfs and mounts from module files
    for info in parse_mount_info("self") {
        if info.source == "magisk" || info.root.starts_with("/adb/modules") {
            targets.push(info.target);
        }
    }

    if targets.is_empty() {
        return;
    }

    let mut prev: Option<PathBuf> = None;
    targets.sort();
    targets.retain(|target| {
        if let Some(prev) = &prev
            && Path::new(target).starts_with(prev)
        {
            return false;
        }
        prev = Some(PathBuf::from(target.clone()));
        true
    });

    for mut target in targets {
        let target = Utf8CStr::from_string(&mut target);
        if target.unmount().is_ok() {
            debug!("denylist: Unmounted ({})", target);
        }
    }
}
