//! cstr.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: DD6E072F320CA896
⚓ FILE_PATH: native/src/base/cstr.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 0DF9DCE1668B9CF26EDE0FF1BEF81DDE


*/


use cxx::{ExternType, type_id};
use libc::c_char;
use nix::NixPath;
use std::borrow::Borrow;
use std::cmp::{Ordering, min};
use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError, OsStr};
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Deref;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::{FromStr, Utf8Error};
use std::{fmt, mem, slice, str};
use thiserror::Error;

use crate::slice_from_ptr_mut;

// Utf8CStr types are UTF-8 validated and null terminated strings.
//
// Several Utf8CStr types:
//
// Utf8CStr: can only exist as reference, similar to &str
// Utf8CString: dynamically sized buffer allocated on the heap, similar to String
// Utf8CStrBufRef: reference to a fixed sized buffer
// Utf8CStrBufArr<N>: fixed sized buffer allocated on the stack
//
// For easier usage, please use the helper functions in cstr::buf.
//
// In most cases, these are the types being used
//
// &Utf8CStr: whenever a printable null terminated string is needed
// &mut dyn Utf8CStrBuf: whenever we need a buffer that needs to support appending
//                       strings to the end, and has to be null terminated
// &mut dyn Utf8CStrBuf: whenever we need a pre-allocated buffer that is large enough to fit
//                       in the result, and has to be null terminated
//
// All types dereferences to &Utf8CStr.
// Utf8CString, Utf8CStrBufRef, and Utf8CStrBufArr<N> implements Utf8CStrBuf.

// Public helper functions

pub mod buf {
    use super::{Utf8CStrBufArr, Utf8CStrBufRef, Utf8CString};

    #[inline(always)]
    pub fn dynamic(capacity: usize) -> Utf8CString {
        Utf8CString::with_capacity(capacity)
    }

    #[inline(always)]
    pub fn default() -> Utf8CStrBufArr<4096> {
        Utf8CStrBufArr::default()
    }

    #[inline(always)]
    pub fn new<const N: usize>() -> Utf8CStrBufArr<N> {
        Utf8CStrBufArr::new()
    }

    #[inline(always)]
    pub fn wrap(buf: &mut [u8]) -> Utf8CStrBufRef<'_> {
        Utf8CStrBufRef::from(buf)
    }

    #[inline(always)]
    pub unsafe fn wrap_ptr<'a>(buf: *mut u8, len: usize) -> Utf8CStrBufRef<'a> {
        unsafe { Utf8CStrBufRef::from_ptr(buf, len) }
    }
}

// Trait definitions

pub trait Utf8CStrBuf: Display + Write + AsRef<Utf8CStr> + Deref<Target = Utf8CStr> {
    // The length of the string without the terminating null character.
    // assert_true(len <= capacity - 1)
    fn len(&self) -> usize;
    fn push_str(&mut self, s: &str) -> usize;
    // The capacity of the internal buffer. The maximum string length this buffer can contain
    // is capacity - 1, because the last byte is reserved for the terminating null character.
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn as_mut_ptr(&mut self) -> *mut c_char;
    fn truncate(&mut self, new_len: usize);
    // Rebuild the Utf8CStr based on the contents of the internal buffer. Required after any
    // unsafe modifications directly though the pointer obtained from self.as_mut_ptr().
    // If an error is returned, the internal buffer will be reset, resulting in an empty string.
    fn rebuild(&mut self) -> Result<(), StrErr>;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait StringExt {
    fn nul_terminate(&mut self) -> &mut [u8];
}

impl StringExt for String {
    fn nul_terminate(&mut self) -> &mut [u8] {
        self.reserve(1);
        // SAFETY: the string is reserved to have enough capacity to fit in the null byte
        // SAFETY: the null byte is explicitly added outside the string's length
        unsafe {
            let buf = slice::from_raw_parts_mut(self.as_mut_ptr(), self.len() + 1);
            *buf.get_unchecked_mut(self.len()) = b'\0';
            buf
        }
    }
}

impl StringExt for PathBuf {
    #[allow(mutable_transmutes)]
    fn nul_terminate(&mut self) -> &mut [u8] {
        self.reserve(1);
        // SAFETY: the PathBuf is reserved to have enough capacity to fit in the null byte
        // SAFETY: the null byte is explicitly added outside the PathBuf's length
        unsafe {
            let bytes: &mut [u8] = mem::transmute(self.as_mut_os_str().as_bytes());
            let buf = slice::from_raw_parts_mut(bytes.as_mut_ptr(), bytes.len() + 1);
            *buf.get_unchecked_mut(bytes.len()) = b'\0';
            buf
        }
    }
}

pub struct Utf8CString(String);

impl Default for Utf8CString {
    fn default() -> Self {
        Utf8CString::with_capacity(256)
    }
}

impl Utf8CString {
    pub fn with_capacity(capacity: usize) -> Utf8CString {
        Utf8CString::from(String::with_capacity(capacity))
    }

    pub fn ensure_capacity(&mut self, capacity: usize) {
        if self.capacity() >= capacity {
            return;
        }
        self.0.reserve(capacity - self.0.len())
    }
}

impl AsRef<Utf8CStr> for Utf8CString {
    #[inline(always)]
    fn as_ref(&self) -> &Utf8CStr {
        // SAFETY: the internal string is always null terminated
        unsafe { mem::transmute(slice::from_raw_parts(self.0.as_ptr(), self.0.len() + 1)) }
    }
}

impl Utf8CStrBuf for Utf8CString {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push_str(&mut self, s: &str) -> usize {
        self.0.push_str(s);
        self.0.nul_terminate();
        s.len()
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn clear(&mut self) {
        self.0.clear();
        self.0.nul_terminate();
    }

    fn as_mut_ptr(&mut self) -> *mut c_char {
        self.0.as_mut_ptr().cast()
    }

    fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
        self.0.nul_terminate();
    }

    fn rebuild(&mut self) -> Result<(), StrErr> {
        // Temporarily move the internal String out
        let mut tmp = String::new();
        mem::swap(&mut tmp, &mut self.0);
        let (ptr, _, capacity) = tmp.into_raw_parts();

        unsafe {
            // Validate the entire buffer, including the unused part
            let bytes = slice::from_raw_parts(ptr, capacity);
            match Utf8CStr::from_bytes_until_nul(bytes) {
                Ok(s) => {
                    // Move the String with the new length back
                    self.0 = String::from_raw_parts(ptr, s.len(), capacity);
                }
                Err(e) => {
                    // Move the String with 0 length back
                    self.0 = String::from_raw_parts(ptr, 0, capacity);
                    self.0.nul_terminate();
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl From<String> for Utf8CString {
    fn from(mut value: String) -> Self {
        value.nul_terminate();
        Utf8CString(value)
    }
}

impl From<&str> for Utf8CString {
    fn from(value: &str) -> Self {
        let mut s = String::with_capacity(value.len() + 1);
        s.push_str(value);
        s.nul_terminate();
        Utf8CString(s)
    }
}

impl FromStr for Utf8CString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Borrow<Utf8CStr> for Utf8CString {
    fn borrow(&self) -> &Utf8CStr {
        self.deref()
    }
}

// UTF-8 validated + null terminated reference to buffer
pub struct Utf8CStrBufRef<'a> {
    used: usize,
    buf: &'a mut [u8],
}

impl<'a> Utf8CStrBufRef<'a> {
    pub unsafe fn from_ptr(buf: *mut u8, len: usize) -> Utf8CStrBufRef<'a> {
        unsafe { Self::from(slice_from_ptr_mut(buf, len)) }
    }
}

impl<'a> From<&'a mut [u8]> for Utf8CStrBufRef<'a> {
    fn from(buf: &'a mut [u8]) -> Utf8CStrBufRef<'a> {
        buf[0] = b'\0';
        Utf8CStrBufRef { used: 0, buf }
    }
}

// UTF-8 validated + null terminated buffer on the stack
pub struct Utf8CStrBufArr<const N: usize> {
    used: usize,
    buf: [u8; N],
}

impl<const N: usize> Utf8CStrBufArr<N> {
    pub fn new() -> Self {
        Utf8CStrBufArr {
            used: 0,
            buf: [0; N],
        }
    }
}

impl Default for Utf8CStrBufArr<4096> {
    fn default() -> Self {
        Utf8CStrBufArr::<4096>::new()
    }
}

#[derive(Debug, Error)]
pub enum StrErr {
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    CStrWithNullError(#[from] FromBytesWithNulError),
    #[error(transparent)]
    CStrUntilNullError(#[from] FromBytesUntilNulError),
    #[error("argument is null")]
    NullPointerError,
}

// UTF-8 validated + null terminated string slice
#[repr(transparent)]
pub struct Utf8CStr([u8]);

impl Utf8CStr {
    pub fn from_cstr(cstr: &CStr) -> Result<&Utf8CStr, StrErr> {
        // Validate the buffer during construction
        str::from_utf8(cstr.to_bytes())?;
        Ok(unsafe { Self::from_bytes_unchecked(cstr.to_bytes_with_nul()) })
    }

    fn from_bytes_until_nul(bytes: &[u8]) -> Result<&Utf8CStr, StrErr> {
        Self::from_cstr(CStr::from_bytes_until_nul(bytes)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<&Utf8CStr, StrErr> {
        Self::from_cstr(CStr::from_bytes_with_nul(bytes)?)
    }

    pub fn from_string(s: &mut String) -> &Utf8CStr {
        let buf = s.nul_terminate();
        // SAFETY: the null byte is explicitly added to the buffer
        unsafe { mem::transmute(buf) }
    }

    #[inline(always)]
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Utf8CStr {
        unsafe { mem::transmute(bytes) }
    }

    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> Result<&'a Utf8CStr, StrErr> {
        if ptr.is_null() {
            return Err(StrErr::NullPointerError);
        }
        Self::from_cstr(unsafe { CStr::from_ptr(ptr) })
    }

    pub unsafe fn from_ptr_unchecked<'a>(ptr: *const c_char) -> &'a Utf8CStr {
        unsafe {
            let cstr = CStr::from_ptr(ptr);
            Self::from_bytes_unchecked(cstr.to_bytes_with_nul())
        }
    }

    pub unsafe fn from_raw_parts<'a>(ptr: *const c_char, len: usize) -> &'a Utf8CStr {
        unsafe {
            let bytes = slice::from_raw_parts(ptr.cast(), len);
            Self::from_bytes_unchecked(bytes)
        }
    }

    #[inline(always)]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.0
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr().cast()
    }

    #[inline(always)]
    pub fn as_cstr(&self) -> &CStr {
        // SAFETY: Already validated as null terminated during construction
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.0) }
    }

    #[inline(always)]
    pub fn as_utf8_cstr(&self) -> &Utf8CStr {
        self
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: Already UTF-8 validated during construction
        // SAFETY: The length of the slice is at least 1 due to null termination check
        unsafe { str::from_utf8_unchecked(self.0.get_unchecked(..self.0.len() - 1)) }
    }
}

impl Deref for Utf8CStr {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl ToOwned for Utf8CStr {
    type Owned = Utf8CString;

    fn to_owned(&self) -> Utf8CString {
        let mut s = Utf8CString::with_capacity(self.len() + 1);
        s.push_str(self.as_str());
        s
    }
}

impl AsRef<Utf8CStr> for Utf8CStr {
    fn as_ref(&self) -> &Utf8CStr {
        self
    }
}

impl NixPath for Utf8CStr {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline(always)]
    fn with_nix_path<T, F>(&self, f: F) -> nix::Result<T>
    where
        F: FnOnce(&CStr) -> T,
    {
        Ok(f(self.as_cstr()))
    }
}

// Notice that we only implement ExternType on Utf8CStr *reference*
unsafe impl ExternType for &Utf8CStr {
    type Id = type_id!("Utf8CStr");
    type Kind = cxx::kind::Trivial;
}

macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

// Assert ABI layout
const_assert_eq!(size_of::<&Utf8CStr>(), size_of::<[usize; 2]>());
const_assert_eq!(align_of::<&Utf8CStr>(), align_of::<[usize; 2]>());

// File system path extensions types

#[repr(transparent)]
pub struct FsPathFollow(Utf8CStr);

impl AsRef<Utf8CStr> for FsPathFollow {
    #[inline(always)]
    fn as_ref(&self) -> &Utf8CStr {
        &self.0
    }
}

// impl<T: AsRef<Utf8CStr>> Deref<Target = Utf8CStr> for T { ... }
macro_rules! impl_cstr_deref {
    ($( ($t:ty, $($g:tt)*) )*) => {$(
        impl<$($g)*> Deref for $t {
            type Target = Utf8CStr;

            #[inline(always)]
            fn deref(&self) -> &Utf8CStr {
                self.as_ref()
            }
        }
    )*}
}

impl_cstr_deref!(
    (Utf8CStrBufRef<'_>,)
    (Utf8CStrBufArr<N>, const N: usize)
    (Utf8CString,)
    (FsPathFollow,)
);

// impl<T: Deref<Target = Utf8CStr>> BoilerPlate for T { ... }
macro_rules! impl_cstr_misc {
    ($( ($t:ty, $($g:tt)*) )*) => {$(
        impl<$($g)*> AsRef<str> for $t {
            #[inline(always)]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl<$($g)*> AsRef<CStr> for $t {
            #[inline(always)]
            fn as_ref(&self) -> &CStr {
                self.as_cstr()
            }
        }
        impl<$($g)*> AsRef<OsStr> for $t {
            #[inline(always)]
            fn as_ref(&self) -> &OsStr {
                OsStr::new(self.as_str())
            }
        }
        impl<$($g)*> AsRef<Path> for $t {
            #[inline(always)]
            fn as_ref(&self) -> &Path {
                Path::new(self.as_str())
            }
        }
        impl<$($g)*> Display for $t {
            #[inline(always)]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Display::fmt(self.as_str(), f)
            }
        }
        impl<$($g)*> Debug for $t {
            #[inline(always)]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Debug::fmt(self.as_str(), f)
            }
        }
        impl<$($g)*> PartialEq<str> for $t {
            #[inline(always)]
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }
        impl<$($g)*> PartialEq<$t> for str {
            #[inline(always)]
            fn eq(&self, other: &$t) -> bool {
                self == other.as_str()
            }
        }
        impl<$($g)*> PartialEq<CStr> for $t {
            #[inline(always)]
            fn eq(&self, other: &CStr) -> bool {
                self.as_cstr() == other
            }
        }
        impl<$($g)*> PartialEq<$t> for CStr {
            #[inline(always)]
            fn eq(&self, other: &$t) -> bool {
                self == other.as_cstr()
            }
        }
        impl<T: AsRef<Utf8CStr> + ?Sized, $($g)*> PartialEq<T> for $t {
            #[inline(always)]
            fn eq(&self, other: &T) -> bool {
                self.as_bytes_with_nul() == other.as_ref().as_bytes_with_nul()
            }
        }
        impl<$($g)*> Eq for $t {}
        impl<$($g)*> PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<$($g)*> Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_str().cmp(other.as_str())
            }
        }
    )*}
}

impl_cstr_misc!(
    (Utf8CStr,)
    (Utf8CStrBufRef<'_>,)
    (Utf8CStrBufArr<N>, const N: usize)
    (Utf8CString,)
    (FsPathFollow,)
);

fn copy_cstr_truncate(dest: &mut [u8], src: &[u8]) -> usize {
    if dest.len() <= 1 {
        // Truncate
        return 0;
    }
    let len = min(src.len(), dest.len() - 1);
    if len > 0 {
        dest[..len].copy_from_slice(&src[..len]);
    }
    dest[len] = b'\0';
    len
}

// impl<T> AsRef<Utf8CStr> for T { ... }
// impl<T> Utf8CStrBuf for T { ... }
macro_rules! impl_cstr_buf {
    ($( ($t:ty, $($g:tt)*) )*) => {$(
        impl<$($g)*> AsRef<Utf8CStr> for $t {
            #[inline(always)]
            fn as_ref(&self) -> &Utf8CStr {
                // SAFETY: the internal buffer is always UTF-8 checked
                // SAFETY: self.used is guaranteed to always <= SIZE - 1
                unsafe { Utf8CStr::from_bytes_unchecked(self.buf.get_unchecked(..(self.used + 1))) }
            }
        }
        impl<$($g)*> Utf8CStrBuf for $t {
            #[inline(always)]
            fn len(&self) -> usize {
                self.used
            }
            #[inline(always)]
            fn push_str(&mut self, s: &str) -> usize {
                // SAFETY: self.used is guaranteed to always <= SIZE - 1
                let dest = unsafe { self.buf.get_unchecked_mut(self.used..) };
                let len = copy_cstr_truncate(dest, s.as_bytes());
                self.used += len;
                len
            }
            #[inline(always)]
            fn capacity(&self) -> usize {
                self.buf.len()
            }
            #[inline(always)]
            fn clear(&mut self) {
                self.buf[0] = b'\0';
                self.used = 0;
            }
            #[inline(always)]
            fn as_mut_ptr(&mut self) -> *mut c_char {
                self.buf.as_mut_ptr().cast()
            }
            fn truncate(&mut self, new_len: usize) {
                if self.used <= new_len {
                    return;
                }
                self.buf[new_len] = b'\0';
                self.used = new_len;
            }
            fn rebuild(&mut self) -> Result<(), StrErr> {
                // Validate the entire buffer, including the unused part
                match Utf8CStr::from_bytes_until_nul(&self.buf) {
                    Ok(s) => self.used = s.len(),
                    Err(e) => {
                        self.used = 0;
                        self.buf[0] = b'\0';
                        return Err(e);
                    }
                }
                Ok(())
            }
        }
    )*}
}

impl_cstr_buf!(
    (Utf8CStrBufRef<'_>,)
    (Utf8CStrBufArr<N>, const N: usize)
);

// impl<T: Utf8CStrBuf> Write for T { ... }
macro_rules! impl_cstr_buf_write {
    ($( ($t:ty, $($g:tt)*) )*) => {$(
        impl<$($g)*> Write for $t {
            #[inline(always)]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.push_str(s);
                Ok(())
            }
        }
    )*}
}

impl_cstr_buf_write!(
    (Utf8CStrBufRef<'_>,)
    (Utf8CStrBufArr<N>, const N: usize)
    (Utf8CString,)
);

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {{
        const NULL_STR: &str = $crate::const_format::concatcp!($str, "\0");
        #[allow(unused_unsafe)]
        unsafe {
            $crate::Utf8CStr::from_bytes_unchecked(NULL_STR.as_bytes())
        }
    }};
}

#[macro_export]
macro_rules! raw_cstr {
    ($str:expr) => {{ $crate::cstr!($str).as_ptr() }};
}
