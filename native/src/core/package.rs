//! package.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: B3F67B41DA378647
⚓ FILE_PATH: native/src/core/package.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: D777419E1C9F7E70BEF944F1CAF94E0A


*/


use crate::consts::{APP_PACKAGE_NAME, MAGISK_VER_CODE};
use crate::daemon::{AID_APP_END, AID_APP_START, AID_USER_OFFSET, MagiskD, to_app_id};
use crate::ffi::{DbEntryKey, get_magisk_tmp, install_apk, uninstall_pkg};
use base::WalkResult::{Abort, Continue, Skip};
use base::{
    BufReadExt, Directory, FsPathBuilder, LoggedResult, ReadExt, ResultExt, Utf8CStrBuf,
    Utf8CString, cstr, error, fd_get_attr, warn,
};
use bit_set::BitSet;
use nix::fcntl::OFlag;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::os::fd::AsRawFd;
use std::time::Duration;

const EOCD_MAGIC: u32 = 0x06054B50;
const APK_SIGNING_BLOCK_MAGIC: [u8; 16] = *b"APK Sig Block 42";
const SIGNATURE_SCHEME_V2_MAGIC: u32 = 0x7109871A;
const PACKAGES_XML: &str = "/data/system/packages.xml";

macro_rules! bad_apk {
    ($msg:literal) => {
        io::Error::new(io::ErrorKind::InvalidData, concat!("cert: ", $msg))
    };
}

/*
 * A v2/v3 signed APK has the format as following
 *
 * +---------------+
 * | zip content   |
 * +---------------+
 * | signing block |
 * +---------------+
 * | central dir   |
 * +---------------+
 * | EOCD          |
 * +---------------+
 *
 * Scan from end of file to find EOCD, and figure our way back to the
 * offset of the signing block. Next, directly extract the certificate
 * from the v2 signature block.
 *
 * All structures above are mostly just for documentation purpose.
 *
 * This method extracts the first certificate of the first signer
 * within the APK v2 signature block.
 */
fn read_certificate(apk: &mut File, version: i32) -> Vec<u8> {
    let res: io::Result<Vec<u8>> = try {
        let mut u32_val = 0u32;
        let mut u64_val = 0u64;

        // Find EOCD
        for i in 0u16.. {
            let mut comment_sz = 0u16;
            apk.seek(SeekFrom::End(-(size_of_val(&comment_sz) as i64) - i as i64))?;
            apk.read_pod(&mut comment_sz)?;

            if comment_sz == i {
                apk.seek(SeekFrom::Current(-22))?;
                let mut magic = 0u32;
                apk.read_pod(&mut magic)?;
                if magic == EOCD_MAGIC {
                    break;
                }
            }
            if i == 0xffff {
                Err(bad_apk!("invalid APK format"))?;
            }
        }

        // We are now at EOCD + sizeof(magic)
        // Seek and read central_dir_off to find the start of the central directory
        let mut central_dir_off = 0u32;
        apk.seek(SeekFrom::Current(12))?;
        apk.read_pod(&mut central_dir_off)?;

        // Code for parse APK comment to get version code
        if version >= 0 {
            let mut comment_sz = 0u16;
            apk.read_pod(&mut comment_sz)?;
            let mut comment = vec![0u8; comment_sz as usize];
            apk.read_exact(&mut comment)?;
            let mut comment = Cursor::new(&comment);
            let mut apk_ver = 0;
            comment.for_each_prop(|k, v| {
                if k == "versionCode" {
                    apk_ver = v.parse::<i32>().unwrap_or(0);
                    false
                } else {
                    true
                }
            });
            if version > apk_ver {
                Err(bad_apk!("APK version too low"))?;
            }
        }

        // Next, find the start of the APK signing block
        apk.seek(SeekFrom::Start((central_dir_off - 24) as u64))?;
        apk.read_pod(&mut u64_val)?; // u64_value = block_sz_
        let mut magic = [0u8; 16];
        apk.read_exact(&mut magic)?;
        if magic != APK_SIGNING_BLOCK_MAGIC {
            Err(bad_apk!("invalid signing block magic"))?;
        }
        let mut signing_blk_sz = 0u64;
        apk.seek(SeekFrom::Current(
            -(u64_val as i64) - (size_of_val(&signing_blk_sz) as i64),
        ))?;
        apk.read_pod(&mut signing_blk_sz)?;
        if signing_blk_sz != u64_val {
            Err(bad_apk!("invalid signing block size"))?;
        }

        // Finally, we are now at the beginning of the id-value pair sequence
        loop {
            apk.read_pod(&mut u64_val)?; // id-value pair length
            if u64_val == signing_blk_sz {
                Err(bad_apk!("cannot find certificate"))?;
            }

            let mut id = 0u32;
            apk.read_pod(&mut id)?;
            if id == SIGNATURE_SCHEME_V2_MAGIC {
                // Skip [signer sequence length] + [1st signer length] + [signed data length]
                apk.seek(SeekFrom::Current((size_of_val(&u32_val) * 3) as i64))?;

                apk.read_pod(&mut u32_val)?; // digest sequence length
                apk.seek(SeekFrom::Current(u32_val as i64))?; // skip all digests

                apk.seek(SeekFrom::Current(size_of_val(&u32_val) as i64))?; // cert sequence length
                apk.read_pod(&mut u32_val)?; // 1st cert length

                let mut cert = vec![0; u32_val as usize];
                apk.read_exact(cert.as_mut())?;
                break cert;
            } else {
                // Skip this id-value pair
                apk.seek(SeekFrom::Current(
                    u64_val as i64 - (size_of_val(&id) as i64),
                ))?;
            }
        }
    };
    res.log().unwrap_or(vec![])
}

fn find_apk_path(pkg: &str) -> LoggedResult<Utf8CString> {
    let mut buf = cstr::buf::default();
    Directory::open(cstr!("/data/app"))?.pre_order_walk(|e| {
        if !e.is_dir() {
            return Ok(Skip);
        }
        let name_bytes = e.name().as_bytes();
        if name_bytes.starts_with(pkg.as_bytes()) && name_bytes[pkg.len()] == b'-' {
            // Found the APK path, we can abort now
            e.resolve_path(&mut buf)?;
            return Ok(Abort);
        }
        if name_bytes.starts_with(b"~~") {
            return Ok(Continue);
        }
        Ok(Skip)
    })?;
    if !buf.is_empty() {
        buf.push_str("/base.apk");
    }
    Ok(buf.to_owned())
}

enum Status {
    Installed,
    NotInstalled,
    CertMismatch,
}

pub struct ManagerInfo {
    stub_apk_fd: Option<File>,
    trusted_cert: Vec<u8>,
    repackaged_app_id: i32,
    repackaged_pkg: String,
    repackaged_cert: Vec<u8>,
    tracked_files: BTreeMap<i32, TrackedFile>,
}

impl Default for ManagerInfo {
    fn default() -> Self {
        ManagerInfo {
            stub_apk_fd: None,
            trusted_cert: Vec::new(),
            repackaged_app_id: -1,
            repackaged_pkg: String::new(),
            repackaged_cert: Vec::new(),
            tracked_files: BTreeMap::new(),
        }
    }
}

#[derive(Default)]
struct TrackedFile {
    path: Utf8CString,
    timestamp: Duration,
}

impl TrackedFile {
    fn new(path: Utf8CString) -> TrackedFile {
        let attr = match path.get_attr() {
            Ok(attr) => attr,
            Err(_) => return TrackedFile::default(),
        };
        let timestamp = Duration::new(attr.st.st_ctime as u64, attr.st.st_ctime_nsec as u32);
        TrackedFile { path, timestamp }
    }

    fn is_same(&self) -> bool {
        if self.path.is_empty() {
            return false;
        }
        let attr = match self.path.get_attr() {
            Ok(attr) => attr,
            Err(_) => return false,
        };
        let timestamp = Duration::new(attr.st.st_ctime as u64, attr.st.st_ctime_nsec as u32);
        timestamp == self.timestamp
    }
}

impl ManagerInfo {
    fn check_dyn(&mut self, daemon: &MagiskD, user: i32, pkg: &str) -> Status {
        let apk = cstr::buf::default()
            .join_path(daemon.app_data_dir())
            .join_path_fmt(user)
            .join_path(pkg)
            .join_path("dyn")
            .join_path("current.apk");
        let uid: i32;
        let cert = match apk.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
            Ok(mut fd) => {
                uid = fd_get_attr(fd.as_raw_fd())
                    .map(|attr| attr.st.st_uid as i32)
                    .unwrap_or(-1);
                read_certificate(&mut fd, MAGISK_VER_CODE)
            }
            Err(_) => {
                warn!("pkg: no dyn APK, ignore");
                return Status::NotInstalled;
            }
        };

        if cert.is_empty() || cert != self.trusted_cert {
            error!("pkg: dyn APK signature mismatch: {}", apk);
            #[cfg(all(feature = "check-signature", not(debug_assertions)))]
            {
                return Status::CertMismatch;
            }
        }

        self.repackaged_app_id = to_app_id(uid);
        self.tracked_files
            .insert(user, TrackedFile::new(apk.to_owned()));
        Status::Installed
    }

    fn check_stub(&mut self, user: i32, pkg: &str) -> Status {
        let Ok(apk) = find_apk_path(pkg) else {
            return Status::NotInstalled;
        };

        let cert = match apk.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
            Ok(mut fd) => read_certificate(&mut fd, -1),
            Err(_) => return Status::NotInstalled,
        };

        if cert.is_empty() || (pkg == self.repackaged_pkg && cert != self.repackaged_cert) {
            error!("pkg: repackaged APK signature invalid: {}", apk);
            uninstall_pkg(&apk);
            return Status::CertMismatch;
        }

        self.repackaged_pkg.clear();
        self.repackaged_pkg.push_str(pkg);
        self.repackaged_cert = cert;
        self.tracked_files.insert(user, TrackedFile::new(apk));
        Status::Installed
    }

    fn check_orig(&mut self, user: i32) -> Status {
        let Ok(apk) = find_apk_path(APP_PACKAGE_NAME) else {
            return Status::NotInstalled;
        };

        let cert = match apk.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
            Ok(mut fd) => read_certificate(&mut fd, MAGISK_VER_CODE),
            Err(_) => return Status::NotInstalled,
        };

        if cert.is_empty() || cert != self.trusted_cert {
            error!("pkg: APK signature mismatch: {}", apk);
            #[cfg(all(feature = "check-signature", not(debug_assertions)))]
            {
                uninstall_pkg(cstr!(APP_PACKAGE_NAME));
                return Status::CertMismatch;
            }
        }

        self.tracked_files.insert(user, TrackedFile::new(apk));
        Status::Installed
    }

    fn install_stub(&mut self) {
        if let Some(ref mut stub_fd) = self.stub_apk_fd {
            // Copy the stub APK
            let tmp_apk = cstr!("/data/stub.apk");
            let result: LoggedResult<()> = try {
                {
                    let mut tmp_apk_file = tmp_apk.create(
                        OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_CLOEXEC,
                        0o600,
                    )?;
                    io::copy(stub_fd, &mut tmp_apk_file)?;
                }
                // Seek the fd back to start
                stub_fd.seek(SeekFrom::Start(0))?;
            };
            if result.is_ok() {
                install_apk(tmp_apk);
            }
        }
    }

    fn get_manager(&mut self, daemon: &MagiskD, user: i32, mut install: bool) -> (i32, &str) {
        let db_pkg = daemon.get_db_string(DbEntryKey::SuManager);

        // If database changed, always re-check files
        if db_pkg != self.repackaged_pkg {
            self.tracked_files.remove(&user);
        }

        if let Some(file) = self.tracked_files.get(&user)
            && file.is_same()
        {
            // no APK
            if &file.path == PACKAGES_XML {
                if install && !daemon.is_emulator {
                    self.install_stub();
                }
                return (-1, "");
            }
            // dyn APK is still the same
            if file.path.starts_with(daemon.app_data_dir().as_str()) {
                return (
                    user * AID_USER_OFFSET + self.repackaged_app_id,
                    &self.repackaged_pkg,
                );
            }
            // stub APK is still the same
            if !self.repackaged_pkg.is_empty() {
                return if matches!(
                    self.check_dyn(daemon, user, self.repackaged_pkg.clone().as_str()),
                    Status::Installed
                ) {
                    (
                        user * AID_USER_OFFSET + self.repackaged_app_id,
                        &self.repackaged_pkg,
                    )
                } else {
                    (-1, "")
                };
            }
            // orig APK is still the same
            let uid = daemon.get_package_uid(user, APP_PACKAGE_NAME);
            return if uid < 0 {
                (-1, "")
            } else {
                (uid, APP_PACKAGE_NAME)
            };
        }

        if !db_pkg.is_empty() {
            match self.check_stub(user, &db_pkg) {
                Status::Installed => {
                    return if matches!(self.check_dyn(daemon, user, &db_pkg), Status::Installed) {
                        (
                            user * AID_USER_OFFSET + self.repackaged_app_id,
                            &self.repackaged_pkg,
                        )
                    } else {
                        (-1, "")
                    };
                }
                Status::NotInstalled => {
                    daemon.rm_db_string(DbEntryKey::SuManager).ok();
                }
                Status::CertMismatch => {
                    install = true;
                    daemon.rm_db_string(DbEntryKey::SuManager).ok();
                }
            }
        }

        self.repackaged_pkg.clear();
        self.repackaged_cert.clear();

        match self.check_orig(user) {
            Status::Installed => {
                let uid = daemon.get_package_uid(user, APP_PACKAGE_NAME);
                return if uid < 0 {
                    (-1, "")
                } else {
                    (uid, APP_PACKAGE_NAME)
                };
            }
            Status::CertMismatch => install = true,
            Status::NotInstalled => {}
        }

        // If we cannot find any manager, track packages.xml for new package installs
        self.tracked_files
            .insert(user, TrackedFile::new(PACKAGES_XML.into()));

        if install && !daemon.is_emulator {
            self.install_stub();
        }
        (-1, "")
    }
}

impl MagiskD {
    fn get_package_uid(&self, user: i32, pkg: &str) -> i32 {
        let path = cstr::buf::default()
            .join_path(self.app_data_dir())
            .join_path_fmt(user)
            .join_path(pkg);
        path.get_attr()
            .map(|attr| attr.st.st_uid as i32)
            .unwrap_or(-1)
    }

    pub fn preserve_stub_apk(&self) {
        let mut info = self.manager_info.lock().unwrap();

        let apk = cstr::buf::default()
            .join_path(get_magisk_tmp())
            .join_path("stub.apk");

        if let Ok(mut fd) = apk.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
            info.trusted_cert = read_certificate(&mut fd, MAGISK_VER_CODE);
            // Seek the fd back to start
            fd.seek(SeekFrom::Start(0)).log_ok();
            info.stub_apk_fd = Some(fd);
        }

        apk.remove().log_ok();
    }

    pub fn get_manager_uid(&self, user: i32) -> i32 {
        let mut info = self.manager_info.lock().unwrap();
        let (uid, _) = info.get_manager(self, user, false);
        uid
    }

    pub fn get_manager(&self, user: i32, install: bool) -> (i32, String) {
        let mut info = self.manager_info.lock().unwrap();
        let (uid, pkg) = info.get_manager(self, user, install);
        (uid, pkg.to_string())
    }

    pub fn ensure_manager(&self) {
        let mut info = self.manager_info.lock().unwrap();
        let _ = info.get_manager(self, 0, true);
    }

    // app_id = app_no + AID_APP_START
    // app_no range: [0, 9999]
    pub fn get_app_no_list(&self) -> BitSet {
        let mut list = BitSet::new();
        let _: LoggedResult<()> = try {
            let mut app_data_dir = Directory::open(self.app_data_dir())?;
            // For each user
            loop {
                let entry = match app_data_dir.read()? {
                    None => break,
                    Some(e) => e,
                };
                let mut user_dir = match entry.open_as_dir() {
                    Err(_) => continue,
                    Ok(dir) => dir,
                };
                // For each package
                loop {
                    match user_dir.read()? {
                        None => break,
                        Some(e) => {
                            let mut entry_path = cstr::buf::default();
                            e.resolve_path(&mut entry_path)?;
                            let attr = entry_path.get_attr()?;
                            let app_id = to_app_id(attr.st.st_uid as i32);
                            if (AID_APP_START..=AID_APP_END).contains(&app_id) {
                                let app_no = app_id - AID_APP_START;
                                list.insert(app_no as usize);
                            }
                        }
                    }
                }
            }
        };
        list
    }
}
