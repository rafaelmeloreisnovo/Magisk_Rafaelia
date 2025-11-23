//! daemon.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: F7C63FB52B090ED2
⚓ FILE_PATH: native/src/core/zygisk/daemon.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 77EFDF2B264B05AEC2A915A0456D49D8


*/


use crate::consts::MODULEROOT;
use crate::daemon::{MagiskD, to_user_id};
use crate::ffi::{ZygiskRequest, ZygiskStateFlags, get_magisk_tmp, update_deny_flags};
use crate::resetprop::{get_prop, set_prop};
use crate::socket::{IpcRead, UnixSocketExt};
use base::libc::STDOUT_FILENO;
use base::{
    Directory, FsPathBuilder, LoggedResult, ResultExt, Utf8CStr, WriteExt, cstr, fork_dont_care,
    libc, log_err, raw_cstr, warn,
};
use nix::fcntl::OFlag;
use std::fmt::Write;
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::ptr;
use std::sync::atomic::Ordering;

const NBPROP: &Utf8CStr = cstr!("ro.dalvik.vm.native.bridge");
const ZYGISKLDR: &str = "libzygisk.so";
const UNMOUNT_MASK: u32 =
    ZygiskStateFlags::ProcessOnDenyList.repr | ZygiskStateFlags::DenyListEnforced.repr;

pub fn zygisk_should_load_module(flags: u32) -> bool {
    flags & UNMOUNT_MASK != UNMOUNT_MASK && flags & ZygiskStateFlags::ProcessIsMagiskApp.repr == 0
}

#[allow(unused_variables)]
fn exec_zygiskd(is_64_bit: bool, remote: UnixStream) {
    // This fd has to survive exec
    unsafe {
        libc::fcntl(remote.as_raw_fd(), libc::F_SETFD, 0);
    }

    // Start building the exec arguments

    #[cfg(target_pointer_width = "64")]
    let magisk = if is_64_bit { "magisk" } else { "magisk32" };

    #[cfg(target_pointer_width = "32")]
    let magisk = "magisk";

    let exe = cstr::buf::new::<64>()
        .join_path(get_magisk_tmp())
        .join_path(magisk);

    let mut fd_str = cstr::buf::new::<16>();
    write!(fd_str, "{}", remote.as_raw_fd()).ok();
    unsafe {
        libc::execl(
            exe.as_ptr(),
            raw_cstr!(""),
            raw_cstr!("zygisk"),
            raw_cstr!("companion"),
            fd_str.as_ptr(),
            ptr::null() as *const libc::c_char,
        );
        libc::exit(-1);
    }
}

#[derive(Default)]
pub struct ZygiskState {
    pub lib_name: String,
    sockets: (Option<UnixStream>, Option<UnixStream>),
    start_count: u32 = 1,
}

impl ZygiskState {
    fn connect_zygiskd(&mut self, mut client: UnixStream, daemon: &MagiskD) -> LoggedResult<()> {
        let is_64_bit: bool = client.read_decodable()?;
        let socket = if is_64_bit {
            &mut self.sockets.1
        } else {
            &mut self.sockets.0
        };

        if let Some(fd) = socket {
            // Make sure the socket is still valid
            let mut pfd = libc::pollfd {
                fd: fd.as_raw_fd(),
                events: 0,
                revents: 0,
            };
            if unsafe { libc::poll(&mut pfd, 1, 0) } != 0 || pfd.revents != 0 {
                // Any revent means error
                *socket = None;
            }
        }

        let socket = if let Some(fd) = socket {
            fd
        } else {
            // Create a new socket pair and fork zygiskd process
            let (local, remote) = UnixStream::pair()?;
            if fork_dont_care() == 0 {
                exec_zygiskd(is_64_bit, remote);
            }
            *socket = Some(local);
            let local = socket.as_mut().unwrap();
            if let Some(module_fds) = daemon.get_module_fds(is_64_bit) {
                local.send_fds(&module_fds)?;
            }
            if local.read_decodable::<i32>()? != 0 {
                return log_err!();
            }
            local
        };
        socket.send_fds(&[client.as_raw_fd()])?;
        Ok(())
    }

    pub fn reset(&mut self, mut restore: bool) {
        if restore {
            self.start_count = 1;
        } else {
            self.sockets = (None, None);
            self.start_count += 1;
            if self.start_count > 3 {
                warn!("zygote crashed too many times, rolling-back");
                restore = true;
            }
        }

        if restore {
            self.restore_prop();
        } else {
            self.set_prop();
        }
    }

    pub fn set_prop(&mut self) {
        if !self.lib_name.is_empty() {
            return;
        }
        let orig = get_prop(NBPROP);
        self.lib_name = if orig.is_empty() || orig == "0" {
            ZYGISKLDR.to_string()
        } else {
            ZYGISKLDR.to_string() + &orig
        };
        set_prop(NBPROP, Utf8CStr::from_string(&mut self.lib_name));
        // Whether Huawei's Maple compiler is enabled.
        // If so, system server will be created by a special Zygote which ignores the native bridge
        // and make system server out of our control. Avoid it by disabling.
        if get_prop(cstr!("ro.maple.enable")) == "1" {
            set_prop(cstr!("ro.maple.enable"), cstr!("0"));
        }
    }

    pub fn restore_prop(&mut self) {
        let mut orig = "0".to_string();
        if self.lib_name.len() > ZYGISKLDR.len() {
            orig = self.lib_name[ZYGISKLDR.len()..].to_string();
        }
        set_prop(NBPROP, Utf8CStr::from_string(&mut orig));
        self.lib_name.clear();
    }
}

impl MagiskD {
    pub fn zygisk_handler(&self, mut client: UnixStream) {
        let _: LoggedResult<()> = try {
            let code = ZygiskRequest {
                repr: client.read_decodable()?,
            };
            match code {
                ZygiskRequest::GetInfo => self.get_process_info(client)?,
                ZygiskRequest::ConnectCompanion => self
                    .zygisk
                    .lock()
                    .unwrap()
                    .connect_zygiskd(client, self)
                    .log_with_msg(|w| w.write_str("zygiskd startup error"))?,
                ZygiskRequest::GetModDir => self.get_mod_dir(client)?,
                _ => {}
            }
        };
    }

    fn get_module_fds(&self, is_64_bit: bool) -> Option<Vec<RawFd>> {
        self.module_list.get().map(|module_list| {
            module_list
                .iter()
                .map(|m| if is_64_bit { m.z64 } else { m.z32 })
                // All fds passed over sockets have to be valid file descriptors.
                // To work around this issue, send over STDOUT_FILENO as an indicator of an
                // invalid fd as it will always be /dev/null in magiskd.
                .map(|fd| if fd < 0 { STDOUT_FILENO } else { fd })
                .collect()
        })
    }

    fn get_process_info(&self, mut client: UnixStream) -> LoggedResult<()> {
        let uid: i32 = client.read_decodable()?;
        let process: String = client.read_decodable()?;
        let is_64_bit: bool = client.read_decodable()?;
        let mut flags: u32 = 0;
        update_deny_flags(uid, &process, &mut flags);
        if self.get_manager_uid(to_user_id(uid)) == uid {
            flags |= ZygiskStateFlags::ProcessIsMagiskApp.repr
        }
        if self.uid_granted_root(uid) {
            flags |= ZygiskStateFlags::ProcessGrantedRoot.repr
        }

        // First send flags
        client.write_pod(&flags)?;

        // Next send modules
        if zygisk_should_load_module(flags)
            && let Some(module_fds) = self.get_module_fds(is_64_bit)
        {
            client.send_fds(&module_fds)?;
        }

        // If we're not in system_server, we are done
        if uid != 1000 || process != "system_server" {
            return Ok(());
        }

        // Read all failed modules
        let failed_ids: Vec<i32> = client.read_decodable()?;
        if let Some(module_list) = self.module_list.get() {
            for id in failed_ids {
                let path = cstr::buf::default()
                    .join_path(MODULEROOT)
                    .join_path(&module_list[id as usize].name)
                    .join_path("zygisk");
                // Create the unloaded marker file
                if let Ok(dir) = Directory::open(&path) {
                    dir.open_as_file_at(cstr!("unloaded"), OFlag::O_CREAT | OFlag::O_RDONLY, 0o644)
                        .log()
                        .ok();
                }
            }
        }

        Ok(())
    }

    fn get_mod_dir(&self, mut client: UnixStream) -> LoggedResult<()> {
        let id: i32 = client.read_decodable()?;
        let module = &self.module_list.get().unwrap()[id as usize];
        let dir = cstr::buf::default()
            .join_path(MODULEROOT)
            .join_path(&module.name);
        let fd = dir.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC)?;
        client.send_fds(&[fd.as_raw_fd()])?;
        Ok(())
    }
}

// FFI to C++
impl MagiskD {
    pub fn zygisk_enabled(&self) -> bool {
        self.zygisk_enabled.load(Ordering::Acquire)
    }
}
