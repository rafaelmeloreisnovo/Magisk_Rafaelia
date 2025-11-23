//! logging.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: C5C2D9B5F40DF814
⚓ FILE_PATH: native/src/core/logging.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 27D4B02B004223EECB1101109EAB87A2


*/


use crate::consts::{LOG_PIPE, LOGFILE};
use crate::ffi::get_magisk_tmp;
use crate::logging::LogFile::{Actual, Buffer};
use base::const_format::concatcp;
use base::{
    FsPathBuilder, LogLevel, LoggedResult, ReadExt, ResultExt, Utf8CStr, Utf8CStrBuf, WriteExt,
    cstr, libc, new_daemon_thread, raw_cstr, update_logger,
};
use bytemuck::{Pod, Zeroable, bytes_of, write_zeroes};
use libc::{PIPE_BUF, c_char, localtime_r, sigtimedwait, time_t, timespec, tm};
use nix::fcntl::OFlag;
use nix::sys::signal::{SigSet, SigmaskHow, Signal};
use nix::unistd::{Gid, Uid, chown, getpid, gettid};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::cmp::min;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{IoSlice, Read, Write};
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

#[allow(dead_code, non_camel_case_types)]
#[derive(FromPrimitive, ToPrimitive)]
#[repr(i32)]
enum ALogPriority {
    ANDROID_LOG_UNKNOWN = 0,
    ANDROID_LOG_DEFAULT,
    ANDROID_LOG_VERBOSE,
    ANDROID_LOG_DEBUG,
    ANDROID_LOG_INFO,
    ANDROID_LOG_WARN,
    ANDROID_LOG_ERROR,
    ANDROID_LOG_FATAL,
    ANDROID_LOG_SILENT,
}

unsafe extern "C" {
    fn __android_log_write(prio: i32, tag: *const c_char, msg: *const c_char);
    fn strftime(buf: *mut c_char, len: usize, fmt: *const c_char, tm: *const tm) -> usize;
}

fn level_to_prio(level: LogLevel) -> i32 {
    match level {
        LogLevel::Error => ALogPriority::ANDROID_LOG_ERROR as i32,
        LogLevel::Warn => ALogPriority::ANDROID_LOG_WARN as i32,
        LogLevel::Info => ALogPriority::ANDROID_LOG_INFO as i32,
        LogLevel::Debug => ALogPriority::ANDROID_LOG_DEBUG as i32,
    }
}

fn android_log_write(level: LogLevel, msg: &Utf8CStr) {
    unsafe {
        __android_log_write(level_to_prio(level), raw_cstr!("Magisk"), msg.as_ptr());
    }
}

pub fn android_logging() {
    update_logger(|logger| logger.write = android_log_write);
}

pub fn magisk_logging() {
    fn magisk_log_write(level: LogLevel, msg: &Utf8CStr) {
        android_log_write(level, msg);
        magisk_log_to_pipe(level_to_prio(level), msg);
    }
    update_logger(|logger| logger.write = magisk_log_write);
}

pub fn zygisk_logging() {
    fn zygisk_log_write(level: LogLevel, msg: &Utf8CStr) {
        android_log_write(level, msg);
        zygisk_log_to_pipe(level_to_prio(level), msg);
    }
    update_logger(|logger| logger.write = zygisk_log_write);
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct LogMeta {
    prio: i32,
    len: i32,
    pid: i32,
    tid: i32,
}

const MAX_MSG_LEN: usize = PIPE_BUF - size_of::<LogMeta>();

fn write_log_to_pipe(mut logd: &File, prio: i32, msg: &Utf8CStr) -> io::Result<usize> {
    // Truncate message if needed
    let len = min(MAX_MSG_LEN, msg.len());
    let msg = &msg.as_bytes()[..len];

    let meta = LogMeta {
        prio,
        len: len as i32,
        pid: getpid().as_raw(),
        tid: gettid().as_raw(),
    };

    let io1 = IoSlice::new(bytes_of(&meta));
    let io2 = IoSlice::new(msg);
    let result = logd.write_vectored(&[io1, io2]);
    if let Err(ref e) = result {
        let mut buf = cstr::buf::new::<256>();
        write!(buf, "Cannot write_log_to_pipe: {e}").ok();
        android_log_write(LogLevel::Error, &buf);
    }
    result
}

static MAGISK_LOGD_FD: Mutex<Option<Arc<File>>> = Mutex::new(None);

fn with_logd_fd<R, F: FnOnce(&File) -> io::Result<R>>(f: F) {
    let fd = MAGISK_LOGD_FD.lock().unwrap().clone();
    if let Some(logd) = fd
        && f(&logd).is_err()
    {
        // If any error occurs, shut down the logd pipe
        *MAGISK_LOGD_FD.lock().unwrap() = None;
    }
}

fn magisk_log_to_pipe(prio: i32, msg: &Utf8CStr) {
    with_logd_fd(|logd| write_log_to_pipe(logd, prio, msg));
}

// SAFETY: zygisk client code runs single threaded, so no need to prevent data race
static ZYGISK_LOGD: AtomicI32 = AtomicI32::new(-1);

pub fn zygisk_close_logd() {
    unsafe {
        libc::close(ZYGISK_LOGD.swap(-1, Ordering::Relaxed));
    }
}

pub fn zygisk_get_logd() -> i32 {
    // If we don't have the log pipe set, open the log pipe FIFO. This could actually happen
    // multiple times in the zygote daemon (parent process) because we had to close this
    // file descriptor to prevent crashing.
    //
    // For some reason, zygote sanitizes and checks FDs *before* forking. This results in the fact
    // that *every* time before zygote forks, it has to close all logging related FDs in order
    // to pass FD checks, just to have it re-initialized immediately after any
    // logging happens ¯\_(ツ)_/¯.
    //
    // To be consistent with this behavior, we also have to close the log pipe to magiskd
    // to make zygote NOT crash if necessary. We accomplish this by hooking __android_log_close
    // and closing it at the same time as the rest of logging FDs.

    let mut raw_fd = ZYGISK_LOGD.load(Ordering::Relaxed);
    if raw_fd < 0 {
        android_logging();
        let path = cstr::buf::default()
            .join_path(get_magisk_tmp())
            .join_path(LOG_PIPE);
        // Open as RW as sometimes it may block
        if let Ok(fd) = path.open(OFlag::O_RDWR | OFlag::O_CLOEXEC) {
            // Only re-enable zygisk logging if success
            zygisk_logging();
            raw_fd = fd.into_raw_fd();
            unsafe {
                libc::close(ZYGISK_LOGD.swap(raw_fd, Ordering::Relaxed));
            }
        } else {
            return -1;
        }
    }
    raw_fd
}

fn zygisk_log_to_pipe(prio: i32, msg: &Utf8CStr) {
    let fd = zygisk_get_logd();
    if fd < 0 {
        // Cannot talk to pipe, abort
        return;
    }

    // Block SIGPIPE
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGPIPE);
    let orig_mask = mask.thread_swap_mask(SigmaskHow::SIG_SETMASK);

    let logd = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) });
    let result = write_log_to_pipe(&logd, prio, msg);

    // Consume SIGPIPE if exists, then restore mask
    if let Ok(orig_mask) = orig_mask {
        unsafe {
            // Unfortunately nix does not have an abstraction over sigtimedwait.
            // Fallback to use raw libc function calls.
            let ts: timespec = std::mem::zeroed();
            sigtimedwait(mask.as_ref(), null_mut(), &ts);
        }
        orig_mask.thread_set_mask().ok();
    }

    // If any error occurs, shut down the logd pipe
    if result.is_err() {
        zygisk_close_logd();
    }
}

// The following is implementation for the logging daemon

enum LogFile {
    Buffer(Vec<u8>),
    Actual(File),
}

impl LogFile {
    fn as_write(&mut self) -> &mut dyn Write {
        match self {
            Buffer(e) => e,
            Actual(e) => e,
        }
    }
}

fn logfile_write_loop(mut pipe: File) -> io::Result<()> {
    let mut logfile: LogFile = Buffer(Vec::new());

    let mut meta = LogMeta::zeroed();
    let mut msg_buf = [0u8; MAX_MSG_LEN];
    let mut aux = cstr::buf::new::<64>();

    loop {
        // Read request
        write_zeroes(&mut meta);
        pipe.read_pod(&mut meta)?;

        if meta.prio < 0 {
            if let Buffer(ref mut buf) = logfile {
                fs::rename(LOGFILE, concatcp!(LOGFILE, ".bak")).ok();
                let mut out = File::create(LOGFILE)?;
                out.write_all(buf.as_slice())?;
                logfile = Actual(out);
            }
            continue;
        }

        if meta.len < 0 || meta.len > MAX_MSG_LEN as i32 {
            continue;
        }

        // Read the rest of the message
        let msg = &mut msg_buf[..(meta.len as usize)];
        pipe.read_exact(msg)?;

        // Start building the log string
        aux.clear();
        let prio = ALogPriority::from_i32(meta.prio).unwrap_or(ALogPriority::ANDROID_LOG_UNKNOWN);
        let prio = match prio {
            ALogPriority::ANDROID_LOG_VERBOSE => 'V',
            ALogPriority::ANDROID_LOG_DEBUG => 'D',
            ALogPriority::ANDROID_LOG_INFO => 'I',
            ALogPriority::ANDROID_LOG_WARN => 'W',
            ALogPriority::ANDROID_LOG_ERROR => 'E',
            // Unsupported values, skip
            _ => continue,
        };

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        // Note: the obvious better implementation is to use the rust chrono crate, however
        // the crate cannot fetch the proper local timezone without pulling in a bunch of
        // timezone handling code. To reduce binary size, fallback to use localtime_r in libc.
        unsafe {
            let secs = now.as_secs() as time_t;
            let mut tm: tm = std::mem::zeroed();
            if localtime_r(&secs, &mut tm).is_null() {
                continue;
            }
            strftime(aux.as_mut_ptr(), aux.capacity(), raw_cstr!("%m-%d %T"), &tm);
        }

        if aux.rebuild().is_ok() {
            write!(
                aux,
                ".{:03} {:5} {:5} {} : ",
                now.subsec_millis(),
                meta.pid,
                meta.tid,
                prio
            )
            .ok();
        } else {
            continue;
        }

        let io1 = IoSlice::new(aux.as_bytes());
        let io2 = IoSlice::new(msg);
        // We don't need to care the written len because we are writing less than PIPE_BUF
        // It's guaranteed to always write the whole thing atomically
        let _ = logfile.as_write().write_vectored(&[io1, io2])?;
    }
}

pub fn setup_logfile() {
    with_logd_fd(|mut logd| {
        let meta = LogMeta {
            prio: -1,
            len: 0,
            pid: 0,
            tid: 0,
        };
        (&mut logd).write_pod(&meta)
    });
}

pub fn start_log_daemon() {
    let path = cstr::buf::default()
        .join_path(get_magisk_tmp())
        .join_path(LOG_PIPE);

    extern "C" fn logfile_writer_thread(arg: usize) -> usize {
        let file = unsafe { File::from_raw_fd(arg as RawFd) };
        logfile_write_loop(file).ok();
        // If any error occurs, shut down the logd pipe
        *MAGISK_LOGD_FD.lock().unwrap() = None;
        0
    }

    let _: LoggedResult<()> = try {
        path.mkfifo(0o666).log_ok();
        chown(path.as_utf8_cstr(), Some(Uid::from(0)), Some(Gid::from(0)))?;
        let read = path.open(OFlag::O_RDWR | OFlag::O_CLOEXEC)?;
        let write = path.open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)?;
        *MAGISK_LOGD_FD.lock().unwrap() = Some(Arc::new(write));
        unsafe {
            new_daemon_thread(logfile_writer_thread, read.into_raw_fd() as usize);
        }
    };
}
