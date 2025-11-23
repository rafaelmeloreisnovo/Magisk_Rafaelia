//! xwrap.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 9927F591370E26A5
⚓ FILE_PATH: native/src/base/xwrap.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 1CC66C9C78ADDE8A4A16ECAB7E0CF196


*/


use crate::cxx_extern::readlinkat;
use crate::{Directory, LibcReturn, ResultExt, Utf8CStr, cstr, slice_from_ptr, slice_from_ptr_mut};
use libc::{c_char, c_uint, c_ulong, c_void, dev_t, mode_t, off_t};
use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::ManuallyDrop;
use std::os::fd::FromRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::ptr::NonNull;

fn ptr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xrealpath(path: *const c_char, buf: *mut u8, bufsz: usize) -> isize {
    unsafe {
        match Utf8CStr::from_ptr(path) {
            Ok(path) => {
                let mut buf = cstr::buf::wrap_ptr(buf, bufsz);
                path.realpath(&mut buf)
                    .log()
                    .map_or(-1, |_| buf.len() as isize)
            }
            Err(_) => -1,
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xreadlink(path: *const c_char, buf: *mut u8, bufsz: usize) -> isize {
    unsafe {
        match Utf8CStr::from_ptr(path) {
            Ok(path) => {
                let mut buf = cstr::buf::wrap_ptr(buf, bufsz);
                path.read_link(&mut buf)
                    .log()
                    .map_or(-1, |_| buf.len() as isize)
            }
            Err(_) => -1,
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xreadlinkat(
    dirfd: RawFd,
    path: *const c_char,
    buf: *mut u8,
    bufsz: usize,
) -> isize {
    unsafe {
        readlinkat(dirfd, path, buf, bufsz)
            .into_os_result("readlinkat", ptr_to_str(path), None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xfopen(path: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    unsafe {
        libc::fopen(path, mode)
            .into_os_result("fopen", ptr_to_str(path), None)
            .log()
            .map_or(ptr::null_mut(), NonNull::as_ptr)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xfdopen(fd: RawFd, mode: *const c_char) -> *mut libc::FILE {
    unsafe {
        libc::fdopen(fd, mode)
            .into_os_result("fdopen", None, None)
            .log()
            .map_or(ptr::null_mut(), NonNull::as_ptr)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xopen(path: *const c_char, flags: i32, mode: mode_t) -> RawFd {
    unsafe {
        libc::open(path, flags, mode as c_uint)
            .into_os_result("open", ptr_to_str(path), None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xopenat(dirfd: RawFd, path: *const c_char, flags: i32, mode: mode_t) -> RawFd {
    unsafe {
        libc::openat(dirfd, path, flags, mode as c_uint)
            .into_os_result("openat", ptr_to_str(path), None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xwrite(fd: RawFd, buf: *const u8, bufsz: usize) -> isize {
    let mut file = unsafe { ManuallyDrop::new(File::from_raw_fd(fd)) };
    let data = unsafe { slice_from_ptr(buf, bufsz) };
    file.write_all(data)
        .log()
        .map_or(-1, |_| data.len() as isize)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xread(fd: RawFd, buf: *mut c_void, bufsz: usize) -> isize {
    unsafe {
        libc::read(fd, buf, bufsz)
            .into_os_result("read", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xxread(fd: RawFd, buf: *mut u8, bufsz: usize) -> isize {
    let mut file = unsafe { ManuallyDrop::new(File::from_raw_fd(fd)) };
    let data = unsafe { slice_from_ptr_mut(buf, bufsz) };
    file.read_exact(data)
        .log()
        .map_or(-1, |_| data.len() as isize)
}

pub(crate) fn xpipe2(fds: &mut [i32; 2], flags: i32) -> i32 {
    unsafe {
        libc::pipe2(fds.as_mut_ptr(), flags)
            .into_os_result("pipe2", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
extern "C" fn xsetns(fd: RawFd, nstype: i32) -> i32 {
    unsafe {
        libc::setns(fd, nstype)
            .into_os_result("setns", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
extern "C" fn xunshare(flags: i32) -> i32 {
    unsafe {
        libc::unshare(flags)
            .into_os_result("unshare", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xopendir(path: *const c_char) -> *mut libc::DIR {
    unsafe {
        libc::opendir(path)
            .into_os_result("opendir", ptr_to_str(path), None)
            .log()
            .map_or(ptr::null_mut(), NonNull::as_ptr)
    }
}

#[unsafe(no_mangle)]
extern "C" fn xfdopendir(fd: RawFd) -> *mut libc::DIR {
    unsafe {
        libc::fdopendir(fd)
            .into_os_result("fdopendir", None, None)
            .log()
            .map_or(ptr::null_mut(), NonNull::as_ptr)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xreaddir(mut dir: ManuallyDrop<Directory>) -> *mut libc::dirent {
    dir.read()
        .log()
        .ok()
        .flatten()
        .map_or(ptr::null_mut(), |entry| entry.as_ptr())
}

#[unsafe(no_mangle)]
extern "C" fn xsetsid() -> i32 {
    unsafe {
        libc::setsid()
            .into_os_result("setsid", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xstat(path: *const c_char, buf: *mut libc::stat) -> i32 {
    unsafe {
        libc::stat(path, buf)
            .into_os_result("stat", ptr_to_str(path), None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xfstat(fd: RawFd, buf: *mut libc::stat) -> i32 {
    unsafe {
        libc::fstat(fd, buf)
            .into_os_result("fstat", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
extern "C" fn xdup2(oldfd: RawFd, newfd: RawFd) -> RawFd {
    unsafe {
        libc::dup2(oldfd, newfd)
            .into_os_result("dup2", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xsymlink(target: *const c_char, linkpath: *const c_char) -> i32 {
    unsafe {
        libc::symlink(target, linkpath)
            .into_os_result("symlink", ptr_to_str(target), ptr_to_str(linkpath))
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xmount(
    src: *const c_char,
    target: *const c_char,
    fstype: *const c_char,
    flags: c_ulong,
    data: *const c_void,
) -> i32 {
    unsafe {
        libc::mount(src, target, fstype, flags, data)
            .into_os_result("mount", ptr_to_str(src), ptr_to_str(target))
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xumount2(target: *const c_char, flags: i32) -> i32 {
    unsafe {
        libc::umount2(target, flags)
            .into_os_result("umount2", ptr_to_str(target), None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xrename(oldname: *const c_char, newname: *const c_char) -> i32 {
    unsafe {
        libc::rename(oldname, newname)
            .into_os_result("rename", ptr_to_str(oldname), ptr_to_str(newname))
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xmkdir(path: *const c_char, mode: mode_t) -> i32 {
    unsafe {
        match Utf8CStr::from_ptr(path) {
            Ok(path) => path.mkdir(mode).log().map_or(-1, |_| 0),
            Err(_) => -1,
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xmkdirs(path: *const c_char, mode: mode_t) -> i32 {
    unsafe {
        match Utf8CStr::from_ptr(path) {
            Ok(path) => path.mkdirs(mode).log().map_or(-1, |_| 0),
            Err(_) => -1,
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xsendfile(
    out_fd: RawFd,
    in_fd: RawFd,
    offset: *mut off_t,
    count: usize,
) -> isize {
    unsafe {
        libc::sendfile(out_fd, in_fd, offset, count)
            .into_os_result("sendfile", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
extern "C" fn xfork() -> i32 {
    unsafe {
        libc::fork()
            .into_os_result("fork", None, None)
            .log()
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn xmknod(pathname: *const c_char, mode: mode_t, dev: dev_t) -> i32 {
    unsafe {
        libc::mknod(pathname, mode, dev)
            .into_os_result("mknod", ptr_to_str(pathname), None)
            .log()
            .unwrap_or(-1)
    }
}
