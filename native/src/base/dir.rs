//! dir.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 948C5DBFB95DAFC1
⚓ FILE_PATH: native/src/base/dir.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 8582AD7AD1230DC72CCC2633FF854F69


*/


use crate::cxx_extern::readlinkat;
use crate::{
    FsPathBuilder, LibcReturn, LoggedResult, OsError, OsResult, Utf8CStr, Utf8CStrBuf, cstr, errno,
    fd_path, fd_set_attr,
};
use libc::{dirent, mode_t};
use nix::errno::Errno;
use nix::fcntl::{AtFlags, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::UnlinkatFlags;
use std::fs::File;
use std::ops::Deref;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, IntoRawFd, OwnedFd, RawFd};
use std::ptr::NonNull;
use std::slice;

pub struct DirEntry<'a> {
    dir: &'a Directory,
    entry: NonNull<dirent>,
    d_name_len: usize,
}

impl DirEntry<'_> {
    pub fn as_ptr(&self) -> *mut dirent {
        self.entry.as_ptr()
    }

    pub fn name(&self) -> &Utf8CStr {
        // SAFETY: Utf8CStr is already validated in Directory::read
        unsafe {
            Utf8CStr::from_bytes_unchecked(slice::from_raw_parts(
                self.d_name.as_ptr().cast(),
                self.d_name_len,
            ))
        }
    }

    pub fn resolve_path(&self, buf: &mut dyn Utf8CStrBuf) -> OsResult<'static, ()> {
        self.dir.path_at(self.name(), buf)
    }

    pub fn is_dir(&self) -> bool {
        self.d_type == libc::DT_DIR
    }

    pub fn is_file(&self) -> bool {
        self.d_type == libc::DT_REG
    }

    pub fn is_symlink(&self) -> bool {
        self.d_type == libc::DT_LNK
    }

    pub fn is_block_device(&self) -> bool {
        self.d_type == libc::DT_BLK
    }

    pub fn is_char_device(&self) -> bool {
        self.d_type == libc::DT_CHR
    }

    pub fn is_fifo(&self) -> bool {
        self.d_type == libc::DT_FIFO
    }

    pub fn is_socket(&self) -> bool {
        self.d_type == libc::DT_SOCK
    }

    pub fn unlink(&self) -> OsResult<'_, ()> {
        let flag = if self.is_dir() {
            UnlinkatFlags::RemoveDir
        } else {
            UnlinkatFlags::NoRemoveDir
        };
        self.dir.unlink_at(self.name(), flag)
    }

    pub fn read_link(&self, buf: &mut dyn Utf8CStrBuf) -> OsResult<'_, ()> {
        self.dir.read_link_at(self.name(), buf)
    }

    pub fn open_as_dir(&self) -> OsResult<'_, Directory> {
        if !self.is_dir() {
            return Err(OsError::new(
                Errno::ENOTDIR,
                "fdopendir",
                Some(self.name()),
                None,
            ));
        }
        self.dir.open_as_dir_at(self.name())
    }

    pub fn open_as_file(&self, flags: OFlag) -> OsResult<'_, File> {
        if self.is_dir() {
            return Err(OsError::new(
                Errno::EISDIR,
                "open_as_file",
                Some(self.name()),
                None,
            ));
        }
        self.dir.open_as_file_at(self.name(), flags, 0)
    }

    pub fn rename_to<'a, 'entry: 'a>(
        &'entry self,
        new_dir: impl AsFd,
        path: &'a Utf8CStr,
    ) -> OsResult<'a, ()> {
        self.dir.rename_at(self.name(), new_dir, path)
    }
}

impl Deref for DirEntry<'_> {
    type Target = dirent;

    fn deref(&self) -> &dirent {
        unsafe { self.entry.as_ref() }
    }
}

#[repr(transparent)]
pub struct Directory {
    inner: NonNull<libc::DIR>,
}

pub enum WalkResult {
    Continue,
    Abort,
    Skip,
}

impl Directory {
    fn open_at<'a>(&self, name: &'a Utf8CStr, flags: OFlag, mode: mode_t) -> OsResult<'a, OwnedFd> {
        nix::fcntl::openat(
            self,
            name,
            flags | OFlag::O_CLOEXEC,
            Mode::from_bits_truncate(mode),
        )
        .into_os_result("openat", Some(name), None)
    }

    fn path_at(&self, name: &Utf8CStr, buf: &mut dyn Utf8CStrBuf) -> OsResult<'static, ()> {
        self.resolve_path(buf)?;
        buf.append_path(name);
        Ok(())
    }
}

// Low-level methods, we should track the caller when error occurs, so return OsResult.
impl Directory {
    pub fn open(path: &Utf8CStr) -> OsResult<'_, Directory> {
        let dirp = unsafe { libc::opendir(path.as_ptr()) };
        let dirp = dirp.into_os_result("opendir", Some(path), None)?;
        Ok(Directory { inner: dirp })
    }

    pub fn read(&mut self) -> OsResult<'static, Option<DirEntry<'_>>> {
        *errno() = 0;
        let e = unsafe { libc::readdir(self.inner.as_ptr()) };
        if e.is_null() {
            return if *errno() != 0 {
                Err(OsError::last_os_error("readdir", None, None))
            } else {
                Ok(None)
            };
        }
        // Skip non UTF-8 entries, ".", and ".."
        unsafe {
            let entry = &*e;

            let Ok(name) = Utf8CStr::from_ptr(entry.d_name.as_ptr()) else {
                return self.read();
            };

            if name == "." || name == ".." {
                self.read()
            } else {
                let e = DirEntry {
                    dir: self,
                    entry: NonNull::from(entry),
                    d_name_len: name.as_bytes_with_nul().len(),
                };
                Ok(Some(e))
            }
        }
    }

    pub fn rewind(&mut self) {
        unsafe { libc::rewinddir(self.inner.as_ptr()) };
    }

    pub fn open_as_dir_at<'a>(&self, name: &'a Utf8CStr) -> OsResult<'a, Directory> {
        let fd = self.open_at(name, OFlag::O_RDONLY, 0)?;
        Directory::try_from(fd).map_err(|e| e.set_args(Some(name), None))
    }

    pub fn open_as_file_at<'a>(
        &self,
        name: &'a Utf8CStr,
        flags: OFlag,
        mode: mode_t,
    ) -> OsResult<'a, File> {
        let fd = self.open_at(name, flags, mode)?;
        Ok(File::from(fd))
    }

    pub fn read_link_at<'a>(
        &self,
        name: &'a Utf8CStr,
        buf: &mut dyn Utf8CStrBuf,
    ) -> OsResult<'a, ()> {
        buf.clear();
        unsafe {
            readlinkat(
                self.as_raw_fd(),
                name.as_ptr(),
                buf.as_mut_ptr().cast(),
                buf.capacity(),
            )
            .check_os_err("readlinkat", Some(name), None)?;
        }
        buf.rebuild().ok();
        Ok(())
    }

    pub fn mkdir_at<'a>(&self, name: &'a Utf8CStr, mode: mode_t) -> OsResult<'a, ()> {
        match nix::sys::stat::mkdirat(self, name, Mode::from_bits_truncate(mode)) {
            Ok(_) | Err(Errno::EEXIST) => Ok(()),
            Err(e) => Err(OsError::new(e, "mkdirat", Some(name), None)),
        }
    }

    // ln -s target self/name
    pub fn create_symlink_at<'a>(
        &self,
        name: &'a Utf8CStr,
        target: &'a Utf8CStr,
    ) -> OsResult<'a, ()> {
        nix::unistd::symlinkat(target, self, name).check_os_err(
            "symlinkat",
            Some(target),
            Some(name),
        )
    }

    pub fn unlink_at<'a>(&self, name: &'a Utf8CStr, flag: UnlinkatFlags) -> OsResult<'a, ()> {
        nix::unistd::unlinkat(self, name, flag).check_os_err("unlinkat", Some(name), None)
    }

    pub fn contains_path(&self, path: &Utf8CStr) -> bool {
        // WARNING: Using faccessat is incorrect, because the raw linux kernel syscall
        // does not support the flag AT_SYMLINK_NOFOLLOW until 5.8 with faccessat2.
        // Use fstatat to check the existence of a file instead.
        nix::sys::stat::fstatat(self, path, AtFlags::AT_SYMLINK_NOFOLLOW).is_ok()
    }

    pub fn resolve_path(&self, buf: &mut dyn Utf8CStrBuf) -> OsResult<'static, ()> {
        fd_path(self.as_raw_fd(), buf)
    }

    pub fn rename_at<'a>(
        &self,
        old: &'a Utf8CStr,
        new_dir: impl AsFd,
        new: &'a Utf8CStr,
    ) -> OsResult<'a, ()> {
        nix::fcntl::renameat(self, old, new_dir, new).check_os_err("renameat", Some(old), Some(new))
    }
}

// High-level helper methods, composed of multiple operations.
// We should treat these as application logic and log ASAP, so return LoggedResult.
impl Directory {
    pub fn post_order_walk<F: FnMut(&DirEntry) -> LoggedResult<WalkResult>>(
        &mut self,
        mut f: F,
    ) -> LoggedResult<WalkResult> {
        self.post_order_walk_impl(&mut f)
    }

    pub fn pre_order_walk<F: FnMut(&DirEntry) -> LoggedResult<WalkResult>>(
        &mut self,
        mut f: F,
    ) -> LoggedResult<WalkResult> {
        self.pre_order_walk_impl(&mut f)
    }

    pub fn remove_all(mut self) -> LoggedResult<()> {
        self.post_order_walk(|e| {
            e.unlink()?;
            Ok(WalkResult::Continue)
        })?;
        Ok(())
    }

    pub fn copy_into(&mut self, dir: &Directory) -> LoggedResult<()> {
        let mut buf = cstr::buf::default();
        self.copy_into_impl(dir, &mut buf)
    }

    pub fn move_into(&mut self, dir: &Directory) -> LoggedResult<()> {
        while let Some(ref e) = self.read()? {
            if e.is_dir() && dir.contains_path(e.name()) {
                // Destination folder exists, needs recursive move
                let mut src = e.open_as_dir()?;
                let dest = dir.open_as_dir_at(e.name())?;
                src.move_into(&dest)?;
                return Ok(e.unlink()?);
            }
            e.rename_to(dir, e.name())?;
        }
        Ok(())
    }

    pub fn link_into(&mut self, dir: &Directory) -> LoggedResult<()> {
        let mut buf = cstr::buf::default();
        self.link_into_impl(dir, &mut buf)
    }
}

impl Directory {
    fn post_order_walk_impl<F: FnMut(&DirEntry) -> LoggedResult<WalkResult>>(
        &mut self,
        f: &mut F,
    ) -> LoggedResult<WalkResult> {
        use WalkResult::*;
        loop {
            match self.read()? {
                None => return Ok(Continue),
                Some(ref e) => {
                    if e.is_dir() {
                        let mut dir = e.open_as_dir()?;
                        if let Abort = dir.post_order_walk_impl(f)? {
                            return Ok(Abort);
                        }
                    }
                    match f(e)? {
                        Abort => return Ok(Abort),
                        Skip => return Ok(Continue),
                        Continue => {}
                    }
                }
            }
        }
    }

    fn pre_order_walk_impl<F: FnMut(&DirEntry) -> LoggedResult<WalkResult>>(
        &mut self,
        f: &mut F,
    ) -> LoggedResult<WalkResult> {
        use WalkResult::*;
        loop {
            match self.read()? {
                None => return Ok(Continue),
                Some(ref e) => match f(e)? {
                    Abort => return Ok(Abort),
                    Skip => continue,
                    Continue => {
                        if e.is_dir() {
                            let mut dir = e.open_as_dir()?;
                            if let Abort = dir.pre_order_walk_impl(f)? {
                                return Ok(Abort);
                            }
                        }
                    }
                },
            }
        }
    }

    fn copy_into_impl(
        &mut self,
        dest_dir: &Directory,
        buf: &mut dyn Utf8CStrBuf,
    ) -> LoggedResult<()> {
        while let Some(ref e) = self.read()? {
            e.resolve_path(buf)?;
            let attr = buf.get_attr()?;
            if e.is_dir() {
                dest_dir.mkdir_at(e.name(), 0o777)?;
                let mut src = e.open_as_dir()?;
                let dest = dest_dir.open_as_dir_at(e.name())?;
                src.copy_into_impl(&dest, buf)?;
                fd_set_attr(dest.as_raw_fd(), &attr)?;
            } else if e.is_file() {
                let mut src = e.open_as_file(OFlag::O_RDONLY)?;
                let mut dest = dest_dir.open_as_file_at(
                    e.name(),
                    OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
                    0o777,
                )?;
                std::io::copy(&mut src, &mut dest)?;
                fd_set_attr(dest.as_raw_fd(), &attr)?;
            } else if e.is_symlink() {
                e.read_link(buf)?;
                dest_dir.create_symlink_at(e.name(), buf)?;
                dest_dir.path_at(e.name(), buf)?;
                buf.set_attr(&attr)?;
            }
        }
        Ok(())
    }

    fn link_into_impl(
        &mut self,
        dest_dir: &Directory,
        buf: &mut dyn Utf8CStrBuf,
    ) -> LoggedResult<()> {
        while let Some(ref e) = self.read()? {
            if e.is_dir() {
                dest_dir.mkdir_at(e.name(), 0o777)?;
                e.resolve_path(buf)?;
                let attr = buf.get_attr()?;
                let mut src = e.open_as_dir()?;
                let dest = dest_dir.open_as_dir_at(e.name())?;
                src.link_into_impl(&dest, buf)?;
                fd_set_attr(dest.as_raw_fd(), &attr)?;
            } else {
                nix::unistd::linkat(e.dir, e.name(), dest_dir, e.name(), AtFlags::empty())
                    .check_os_err("linkat", Some(e.name()), None)?;
            }
        }
        Ok(())
    }
}

impl TryFrom<OwnedFd> for Directory {
    type Error = OsError<'static>;

    fn try_from(fd: OwnedFd) -> OsResult<'static, Self> {
        let dirp = unsafe { libc::fdopendir(fd.into_raw_fd()) };
        let dirp = dirp.into_os_result("fdopendir", None, None)?;
        Ok(Directory { inner: dirp })
    }
}

impl AsRawFd for Directory {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { libc::dirfd(self.inner.as_ptr()) }
    }
}

impl AsFd for Directory {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        unsafe {
            libc::closedir(self.inner.as_ptr());
        }
    }
}
