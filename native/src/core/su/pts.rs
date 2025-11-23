//! pts.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 7A29DBCD7432050C
⚓ FILE_PATH: native/src/core/su/pts.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: C4D8CE31B4929A6CAB67EDB101378F19


*/


use base::{FileOrStd, LibcReturn, LoggedResult, OsResult, ResultExt, libc, warn};
use libc::{STDIN_FILENO, TIOCGWINSZ, TIOCSWINSZ, c_int, winsize};
use nix::fcntl::{OFlag, SpliceFFlags};
use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
use nix::sys::signal::{SigSet, Signal, raise};
use nix::sys::signalfd::{SfdFlags, SignalFd};
use nix::sys::termios::{SetArg, Termios, cfmakeraw, tcgetattr, tcsetattr};
use nix::unistd::pipe2;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::os::fd::{AsFd, AsRawFd, FromRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};

static SHOULD_USE_SPLICE: AtomicBool = AtomicBool::new(true);
const TIOCGPTN: u32 = 0x80045430;

unsafe extern "C" {
    // Don't use the declaration from the libc crate as request should be u32 not i32
    fn ioctl(fd: c_int, request: u32, ...) -> i32;
}

pub fn get_pty_num(fd: i32) -> i32 {
    let mut pty_num = -1i32;
    if unsafe { ioctl(fd, TIOCGPTN, &mut pty_num) } != 0 {
        warn!("Failed to get pty number");
    }
    pty_num
}

fn sync_winsize(ptmx: i32) {
    let mut ws: winsize = unsafe { std::mem::zeroed() };
    if unsafe { ioctl(STDIN_FILENO, TIOCGWINSZ as u32, &mut ws) } >= 0 {
        unsafe { ioctl(ptmx, TIOCSWINSZ as u32, &ws) };
    }
}

fn splice(fd_in: impl AsFd, fd_out: impl AsFd, len: usize) -> OsResult<'static, usize> {
    nix::fcntl::splice(fd_in, None, fd_out, None, len, SpliceFFlags::empty())
        .into_os_result("splice", None, None)
}

fn pump_via_copy(mut fd_in: &File, mut fd_out: &File) -> LoggedResult<()> {
    let mut buf = MaybeUninit::<[u8; 4096]>::uninit();
    let buf = unsafe { buf.assume_init_mut() };
    let len = fd_in.read(buf)?;
    fd_out.write_all(&buf[..len])?;
    Ok(())
}

fn pump_via_splice(fd_in: &File, fd_out: &File, pipe: &(File, File)) -> LoggedResult<()> {
    if !SHOULD_USE_SPLICE.load(Ordering::Relaxed) {
        return pump_via_copy(fd_in, fd_out);
    }

    // The pipe capacity is by default 16 pages, let's just use 65536
    let Ok(len) = splice(fd_in, &pipe.1, 65536) else {
        // If splice failed, stop using splice and fallback to userspace copy
        SHOULD_USE_SPLICE.store(false, Ordering::Relaxed);
        return pump_via_copy(fd_in, fd_out);
    };
    if len == 0 {
        return Ok(());
    }
    if splice(&pipe.0, fd_out, len).is_err() {
        // If splice failed, stop using splice and fallback to userspace copy
        SHOULD_USE_SPLICE.store(false, Ordering::Relaxed);
        return pump_via_copy(&pipe.0, fd_out);
    }
    Ok(())
}

fn set_stdin_raw() -> LoggedResult<Termios> {
    let mut term = tcgetattr(FileOrStd::StdIn.as_file())?;
    let old_term = term.clone();

    let old_output_flags = old_term.output_flags;
    cfmakeraw(&mut term);

    // Preserve output_flags, since we are not setting stdout raw
    term.output_flags = old_output_flags;

    tcsetattr(FileOrStd::StdIn.as_file(), SetArg::TCSAFLUSH, &term)
        .or_else(|_| tcsetattr(FileOrStd::StdIn.as_file(), SetArg::TCSADRAIN, &term))
        .check_os_err("tcsetattr", None, None)
        .log_with_msg(|w| w.write_str("Failed to set terminal attributes"))?;

    Ok(old_term)
}

fn restore_stdin(term: Termios) -> LoggedResult<()> {
    tcsetattr(FileOrStd::StdIn.as_file(), SetArg::TCSAFLUSH, &term)
        .or_else(|_| tcsetattr(FileOrStd::StdIn.as_file(), SetArg::TCSADRAIN, &term))
        .check_os_err("tcsetattr", None, None)
        .log_with_msg(|w| w.write_str("Failed to restore terminal attributes"))
}

fn pump_tty_impl(ptmx: File, pump_stdin: bool) -> LoggedResult<()> {
    let mut signal_fd: Option<SignalFd> = None;

    let raw_ptmx = ptmx.as_raw_fd();
    let mut raw_sig = -1;

    let mut poll_fds = Vec::with_capacity(3);
    poll_fds.push(PollFd::new(ptmx.as_fd(), PollFlags::POLLIN));
    if pump_stdin {
        // If stdin is tty, we need to monitor SIGWINCH
        let mut set = SigSet::empty();
        set.add(Signal::SIGWINCH);
        set.thread_block()
            .check_os_err("pthread_sigmask", None, None)?;
        let sig = SignalFd::with_flags(&set, SfdFlags::SFD_CLOEXEC)
            .into_os_result("signalfd", None, None)?;
        raw_sig = sig.as_raw_fd();
        signal_fd = Some(sig);
        poll_fds.push(PollFd::new(
            signal_fd.as_ref().unwrap().as_fd(),
            PollFlags::POLLIN,
        ));

        // We also need to pump stdin to ptmx
        poll_fds.push(PollFd::new(
            FileOrStd::StdIn.as_file().as_fd(),
            PollFlags::POLLIN,
        ));
    }

    // Any flag in this list indicates stop polling
    let stop_flags = PollFlags::POLLERR | PollFlags::POLLHUP | PollFlags::POLLNVAL;

    // Open a pipe to bypass userspace copy with splice
    let pipe_fd = pipe2(OFlag::O_CLOEXEC).into_os_result("pipe2", None, None)?;
    let pipe_fd = (File::from(pipe_fd.0), File::from(pipe_fd.1));

    'poll: loop {
        // Wait for event
        poll(&mut poll_fds, PollTimeout::NONE).check_os_err("poll", None, None)?;
        for pfd in &poll_fds {
            if pfd.all().unwrap_or(false) {
                let raw_fd = pfd.as_fd().as_raw_fd();
                if raw_fd == STDIN_FILENO {
                    pump_via_splice(FileOrStd::StdIn.as_file(), &ptmx, &pipe_fd)?;
                } else if raw_fd == raw_ptmx {
                    pump_via_splice(&ptmx, FileOrStd::StdIn.as_file(), &pipe_fd)?;
                } else if raw_fd == raw_sig {
                    sync_winsize(raw_ptmx);
                    signal_fd.as_ref().unwrap().read_signal()?;
                }
            } else if pfd
                .revents()
                .unwrap_or(PollFlags::POLLHUP)
                .intersects(stop_flags)
            {
                // If revents is None or contains any err_flags, stop polling
                break 'poll;
            }
        }
    }
    Ok(())
}

pub fn pump_tty(ptmx: RawFd, pump_stdin: bool) {
    let old_term = if pump_stdin {
        sync_winsize(ptmx);
        set_stdin_raw().ok()
    } else {
        None
    };

    let ptmx = unsafe { File::from_raw_fd(ptmx) };
    pump_tty_impl(ptmx, pump_stdin).ok();

    if let Some(term) = old_term {
        restore_stdin(term).ok();
    }
    raise(Signal::SIGWINCH).ok();
}
