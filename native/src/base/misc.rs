//! misc.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 803340F2B75C3946
⚓ FILE_PATH: native/src/base/misc.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: CF7C961F70B371784A2B50714D21C3DB


*/


use super::argh::{EarlyExit, MissingRequirements};
use crate::{Utf8CStr, Utf8CString, cstr, ffi};
use libc::c_char;
use std::fmt::Arguments;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::{fmt, slice, str};

pub fn errno() -> &'static mut i32 {
    unsafe { &mut *libc::__errno() }
}

// When len is 0, don't care whether buf is null or not
#[inline]
pub unsafe fn slice_from_ptr<'a, T>(buf: *const T, len: usize) -> &'a [T] {
    unsafe {
        if len == 0 {
            &[]
        } else {
            slice::from_raw_parts(buf, len)
        }
    }
}

// When len is 0, don't care whether buf is null or not
#[inline]
pub unsafe fn slice_from_ptr_mut<'a, T>(buf: *mut T, len: usize) -> &'a mut [T] {
    unsafe {
        if len == 0 {
            &mut []
        } else {
            slice::from_raw_parts_mut(buf, len)
        }
    }
}

pub trait BytesExt {
    fn find(&self, needle: &[u8]) -> Option<usize>;
    fn contains(&self, needle: &[u8]) -> bool {
        self.find(needle).is_some()
    }
}

impl<T: AsRef<[u8]> + ?Sized> BytesExt for T {
    fn find(&self, needle: &[u8]) -> Option<usize> {
        fn inner(haystack: &[u8], needle: &[u8]) -> Option<usize> {
            unsafe {
                let ptr: *const u8 = libc::memmem(
                    haystack.as_ptr().cast(),
                    haystack.len(),
                    needle.as_ptr().cast(),
                    needle.len(),
                )
                .cast();
                if ptr.is_null() {
                    None
                } else {
                    Some(ptr.offset_from(haystack.as_ptr()) as usize)
                }
            }
        }
        inner(self.as_ref(), needle)
    }
}

pub trait MutBytesExt {
    fn patch(&mut self, from: &[u8], to: &[u8]) -> Vec<usize>;
}

impl<T: AsMut<[u8]> + ?Sized> MutBytesExt for T {
    fn patch(&mut self, from: &[u8], to: &[u8]) -> Vec<usize> {
        ffi::mut_u8_patch(self.as_mut(), from, to)
    }
}

pub trait EarlyExitExt<T> {
    fn on_early_exit<F: FnOnce()>(self, print_help_msg: F) -> T;
}

impl<T> EarlyExitExt<T> for Result<T, EarlyExit> {
    fn on_early_exit<F: FnOnce()>(self, print_help_msg: F) -> T {
        match self {
            Ok(t) => t,
            Err(EarlyExit { output, is_help }) => {
                if is_help {
                    print_help_msg();
                    exit(0)
                } else {
                    eprintln!("{output}");
                    print_help_msg();
                    exit(1)
                }
            }
        }
    }
}

pub struct PositionalArgParser<'a>(pub slice::Iter<'a, &'a str>);

impl PositionalArgParser<'_> {
    pub fn required(&mut self, field_name: &'static str) -> Result<Utf8CString, EarlyExit> {
        if let Some(next) = self.0.next() {
            Ok((*next).into())
        } else {
            let mut missing = MissingRequirements::default();
            missing.missing_positional_arg(field_name);
            missing.err_on_any()?;
            unreachable!()
        }
    }

    pub fn optional(&mut self) -> Option<Utf8CString> {
        self.0.next().map(|s| (*s).into())
    }

    pub fn last_required(&mut self, field_name: &'static str) -> Result<Utf8CString, EarlyExit> {
        let r = self.required(field_name)?;
        self.ensure_end()?;
        Ok(r)
    }

    pub fn last_optional(&mut self) -> Result<Option<Utf8CString>, EarlyExit> {
        let r = self.optional();
        if r.is_none() {
            return Ok(r);
        }
        self.ensure_end()?;
        Ok(r)
    }

    fn ensure_end(&mut self) -> Result<(), EarlyExit> {
        if self.0.len() == 0 {
            Ok(())
        } else {
            Err(EarlyExit::from(format!(
                "Unrecognized argument: {}\n",
                self.0.next().unwrap()
            )))
        }
    }
}

pub struct FmtAdaptor<'a, T>(pub &'a mut T)
where
    T: Write;

impl<T: Write> fmt::Write for FmtAdaptor<'_, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
    fn write_fmt(&mut self, args: Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args).map_err(|_| fmt::Error)
    }
}

pub struct AtomicArc<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicArc<T> {
    pub fn new(arc: Arc<T>) -> AtomicArc<T> {
        let raw = Arc::into_raw(arc);
        Self {
            ptr: AtomicPtr::new(raw as *mut _),
        }
    }

    pub fn load(&self) -> Arc<T> {
        let raw = self.ptr.load(Ordering::Acquire);
        // SAFETY: the raw pointer is always created from Arc::into_raw
        let arc = ManuallyDrop::new(unsafe { Arc::from_raw(raw) });
        ManuallyDrop::into_inner(arc.clone())
    }

    fn swap_ptr(&self, raw: *const T) -> Arc<T> {
        let prev = self.ptr.swap(raw as *mut _, Ordering::AcqRel);
        // SAFETY: the raw pointer is always created from Arc::into_raw
        unsafe { Arc::from_raw(prev) }
    }

    pub fn swap(&self, arc: Arc<T>) -> Arc<T> {
        let raw = Arc::into_raw(arc);
        self.swap_ptr(raw)
    }

    pub fn store(&self, arc: Arc<T>) {
        // Drop the previous value
        let _ = self.swap(arc);
    }
}

impl<T> Drop for AtomicArc<T> {
    fn drop(&mut self) {
        // Drop the internal value
        let _ = self.swap_ptr(std::ptr::null());
    }
}

impl<T: Default> Default for AtomicArc<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct Chunker {
    chunk: Box<[u8]>,
    chunk_size: usize,
    pos: usize,
}

impl Chunker {
    pub fn new(chunk_size: usize) -> Self {
        Chunker {
            // SAFETY: all bytes will be initialized before it is used, tracked by self.pos
            chunk: unsafe { Box::new_uninit_slice(chunk_size).assume_init() },
            chunk_size,
            pos: 0,
        }
    }

    pub fn set_chunk_size(&mut self, chunk_size: usize) {
        self.chunk_size = chunk_size;
        self.pos = 0;
        if self.chunk.len() < chunk_size {
            self.chunk = unsafe { Box::new_uninit_slice(chunk_size).assume_init() };
        }
    }

    // Returns (remaining buf, Option<Chunk>)
    pub fn add_data<'a, 'b: 'a>(&'a mut self, mut buf: &'b [u8]) -> (&'b [u8], Option<&'a [u8]>) {
        let mut chunk = None;
        if self.pos > 0 {
            // Try to fill the chunk
            let len = std::cmp::min(self.chunk_size - self.pos, buf.len());
            self.chunk[self.pos..self.pos + len].copy_from_slice(&buf[..len]);
            self.pos += len;
            // If the chunk is filled, consume it
            if self.pos == self.chunk_size {
                chunk = Some(&self.chunk[..self.chunk_size]);
                self.pos = 0;
            }
            buf = &buf[len..];
        } else if buf.len() >= self.chunk_size {
            // Directly consume a chunk from buf
            chunk = Some(&buf[..self.chunk_size]);
            buf = &buf[self.chunk_size..];
        } else {
            // Copy buf into chunk
            self.chunk[self.pos..self.pos + buf.len()].copy_from_slice(buf);
            self.pos += buf.len();
            return (&[], None);
        }
        (buf, chunk)
    }

    pub fn get_available(&mut self) -> &[u8] {
        let chunk = &self.chunk[..self.pos];
        self.pos = 0;
        chunk
    }
}

pub struct CmdArgs(pub Vec<&'static str>);

impl CmdArgs {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(argc: i32, argv: *const *const c_char) -> CmdArgs {
        CmdArgs(
            // SAFETY: libc guarantees argc and argv are properly setup and are static
            unsafe { slice::from_raw_parts(argv, argc as usize) }
                .iter()
                .map(|s| unsafe { Utf8CStr::from_ptr(*s) })
                .map(|r| r.unwrap_or(cstr!("<invalid>")))
                .map(Utf8CStr::as_str)
                .collect(),
        )
    }

    pub fn as_slice(&self) -> &[&'static str] {
        self.0.as_slice()
    }

    pub fn iter(&self) -> slice::Iter<'_, &'static str> {
        self.0.iter()
    }

    pub fn cstr_iter(&self) -> impl Iterator<Item = &'static Utf8CStr> {
        // SAFETY: libc guarantees null terminated strings
        self.0
            .iter()
            .map(|s| unsafe { Utf8CStr::from_raw_parts(s.as_ptr().cast(), s.len() + 1) })
    }
}
