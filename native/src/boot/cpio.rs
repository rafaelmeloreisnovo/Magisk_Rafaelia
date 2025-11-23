//! cpio.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 66AA90CDF5EFA8D5
⚓ FILE_PATH: native/src/boot/cpio.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: B070D9CDC015D290F5DE7CD4148FBF83


*/


#![allow(clippy::useless_conversion)]

use argh::FromArgs;
use base::argh;
use bytemuck::{Pod, Zeroable, from_bytes};
use num_traits::cast::AsPrimitive;
use size::{Base, Size, Style};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::mem::size_of;
use std::process::exit;
use std::str;

use crate::check_env;
use crate::compress::{get_decoder, get_encoder};
use crate::ffi::FileFormat;
use crate::patch::{patch_encryption, patch_verity};
use base::libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP,
    S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR, dev_t, gid_t, major, makedev, minor, mknod,
    mode_t, uid_t,
};
use base::nix::fcntl::OFlag;
use base::{
    BytesExt, EarlyExitExt, LoggedResult, MappedFile, OptionExt, ResultExt, Utf8CStr, Utf8CStrBuf,
    WriteExt, cstr, log_err,
};

#[derive(FromArgs)]
struct CpioCommand {
    #[argh(subcommand)]
    action: CpioAction,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum CpioAction {
    Test(Test),
    Restore(Restore),
    Patch(Patch),
    Exists(Exists),
    Backup(Backup),
    Remove(Remove),
    Move(Move),
    Extract(Extract),
    MakeDir(MakeDir),
    Link(Link),
    Add(Add),
    List(List),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "test")]
struct Test {}

#[derive(FromArgs)]
#[argh(subcommand, name = "restore")]
struct Restore {}

#[derive(FromArgs)]
#[argh(subcommand, name = "patch")]
struct Patch {}

#[derive(FromArgs)]
#[argh(subcommand, name = "exists")]
struct Exists {
    #[argh(positional, arg_name = "entry")]
    path: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "backup")]
struct Backup {
    #[argh(positional, arg_name = "orig")]
    origin: String,
    #[argh(switch, short = 'n')]
    skip_compress: bool,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "rm")]
struct Remove {
    #[argh(positional, arg_name = "entry")]
    path: String,
    #[argh(switch, short = 'r')]
    recursive: bool,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "mv")]
struct Move {
    #[argh(positional, arg_name = "source")]
    from: String,
    #[argh(positional, arg_name = "dest")]
    to: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "extract")]
struct Extract {
    #[argh(positional, greedy)]
    paths: Vec<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "mkdir")]
struct MakeDir {
    #[argh(positional, from_str_fn(parse_mode))]
    mode: mode_t,
    #[argh(positional, arg_name = "entry")]
    dir: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "ln")]
struct Link {
    #[argh(positional, arg_name = "entry")]
    src: String,
    #[argh(positional, arg_name = "target")]
    dst: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
struct Add {
    #[argh(positional, from_str_fn(parse_mode))]
    mode: mode_t,
    #[argh(positional, arg_name = "entry")]
    path: String,
    #[argh(positional, arg_name = "infile")]
    file: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "ls")]
struct List {
    #[argh(positional, default = r#"String::from("/")"#)]
    path: String,
    #[argh(switch, short = 'r')]
    recursive: bool,
}

pub(crate) fn print_cpio_usage() {
    eprintln!(
        r#"Usage: magiskboot cpio <incpio> [commands...]

Do cpio commands to <incpio> (modifications are done in-place).
Each command is a single argument; add quotes for each command.

Supported commands:
  exists ENTRY
    Return 0 if ENTRY exists, else return 1
  ls [-r] [PATH]
    List PATH ("/" by default); specify [-r] to list recursively
  rm [-r] ENTRY
    Remove ENTRY, specify [-r] to remove recursively
  mkdir MODE ENTRY
    Create directory ENTRY with permissions MODE
  ln TARGET ENTRY
    Create a symlink to TARGET with the name ENTRY
  mv SOURCE DEST
    Move SOURCE to DEST
  add MODE ENTRY INFILE
    Add INFILE as ENTRY with permissions MODE; replaces ENTRY if exists
  extract [ENTRY OUT]
    Extract ENTRY to OUT, or extract all entries to current directory
  test
    Test the cpio's status. Return values:
    0:stock    1:Magisk    2:unsupported
  patch
    Apply ramdisk patches
    Configure with env variables: KEEPVERITY KEEPFORCEENCRYPT
  backup ORIG [-n]
    Create ramdisk backups from ORIG, specify [-n] to skip compression
  restore
    Restore ramdisk from ramdisk backup stored within incpio
"#
    )
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C, packed)]
struct CpioHeader {
    magic: [u8; 6],
    ino: [u8; 8],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    nlink: [u8; 8],
    mtime: [u8; 8],
    filesize: [u8; 8],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    rdevmajor: [u8; 8],
    rdevminor: [u8; 8],
    namesize: [u8; 8],
    check: [u8; 8],
}

struct Cpio {
    entries: BTreeMap<String, Box<CpioEntry>>,
}

struct CpioEntry {
    mode: mode_t,
    uid: uid_t,
    gid: gid_t,
    rdevmajor: dev_t,
    rdevminor: dev_t,
    data: Vec<u8>,
}

impl Cpio {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn load_from_data(data: &[u8]) -> LoggedResult<Self> {
        let mut cpio = Cpio::new();
        let mut pos = 0_usize;
        while pos < data.len() {
            let hdr_sz = size_of::<CpioHeader>();
            let hdr = from_bytes::<CpioHeader>(&data[pos..(pos + hdr_sz)]);
            if &hdr.magic != b"070701" {
                return log_err!("invalid cpio magic");
            }
            pos += hdr_sz;
            let name_sz = x8u(&hdr.namesize)? as usize;
            let name = Utf8CStr::from_bytes(&data[pos..(pos + name_sz)])?.to_string();
            pos += name_sz;
            pos = align_4(pos);
            if name == "." || name == ".." {
                continue;
            }
            if name == "TRAILER!!!" {
                match data[pos..].find(b"070701") {
                    Some(x) => pos += x,
                    None => break,
                }
                continue;
            }
            let file_sz = x8u(&hdr.filesize)? as usize;
            let entry = Box::new(CpioEntry {
                mode: x8u(&hdr.mode)?.as_(),
                uid: x8u(&hdr.uid)?.as_(),
                gid: x8u(&hdr.gid)?.as_(),
                rdevmajor: x8u(&hdr.rdevmajor)?.as_(),
                rdevminor: x8u(&hdr.rdevminor)?.as_(),
                data: data[pos..(pos + file_sz)].to_vec(),
            });
            pos += file_sz;
            cpio.entries.insert(name, entry);
            pos = align_4(pos);
        }
        Ok(cpio)
    }

    fn load_from_file(path: &Utf8CStr) -> LoggedResult<Self> {
        eprintln!("Loading cpio: [{path}]");
        let file = MappedFile::open(path)?;
        Self::load_from_data(file.as_ref())
    }

    fn dump(&self, path: &str) -> LoggedResult<()> {
        eprintln!("Dumping cpio: [{path}]");
        let mut file = File::create(path)?;
        let mut pos = 0usize;
        let mut inode = 300000i64;
        for (name, entry) in &self.entries {
            pos += file.write(
                format!(
                    "070701{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
                    inode,
                    entry.mode,
                    entry.uid,
                    entry.gid,
                    1,
                    0,
                    entry.data.len(),
                    0,
                    0,
                    entry.rdevmajor,
                    entry.rdevminor,
                    name.len() + 1,
                    0
                ).as_bytes(),
            )?;
            pos += file.write(name.as_bytes())?;
            pos += file.write(&[0])?;
            file.write_zeros(align_4(pos) - pos)?;
            pos = align_4(pos);
            pos += file.write(&entry.data)?;
            file.write_zeros(align_4(pos) - pos)?;
            pos = align_4(pos);
            inode += 1;
        }
        pos += file.write(
            format!("070701{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
                    inode, 0o755, 0, 0, 1, 0, 0, 0, 0, 0, 0, 11, 0
            ).as_bytes()
        )?;
        pos += file.write("TRAILER!!!\0".as_bytes())?;
        file.write_zeros(align_4(pos) - pos)?;
        Ok(())
    }

    fn rm(&mut self, path: &str, recursive: bool) {
        let path = norm_path(path);
        if self.entries.remove(&path).is_some() {
            eprintln!("Removed entry [{path}]");
        }
        if recursive {
            let path = path + "/";
            self.entries.retain(|k, _| {
                if k.starts_with(&path) {
                    eprintln!("Removed entry [{k}]");
                    false
                } else {
                    true
                }
            })
        }
    }

    fn extract_entry(&self, path: &str, out: &mut String) -> LoggedResult<()> {
        let entry = self
            .entries
            .get(path)
            .ok_or_log_msg(|w| w.write_str("No such file"))?;
        eprintln!("Extracting entry [{path}] to [{out}]");

        let out = Utf8CStr::from_string(out);

        let mut buf = cstr::buf::default();

        // Make sure its parent directories exist
        if let Some(dir) = out.parent_dir() {
            buf.push_str(dir);
            buf.mkdirs(0o755)?;
        }

        let mode: mode_t = (entry.mode & 0o777).into();

        match entry.mode & S_IFMT {
            S_IFDIR => out.mkdir(mode)?,
            S_IFREG => {
                let mut file = out.create(
                    OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_WRONLY | OFlag::O_CLOEXEC,
                    mode,
                )?;
                file.write_all(&entry.data)?;
            }
            S_IFLNK => {
                buf.clear();
                buf.push_str(str::from_utf8(entry.data.as_slice())?);
                out.create_symlink_to(&buf)?;
            }
            S_IFBLK | S_IFCHR => {
                let dev = makedev(entry.rdevmajor.try_into()?, entry.rdevminor.try_into()?);
                unsafe { mknod(out.as_ptr().cast(), entry.mode, dev) };
            }
            _ => {
                return log_err!("unknown entry type");
            }
        }
        Ok(())
    }

    fn extract(&self, path: Option<&mut String>, out: Option<&mut String>) -> LoggedResult<()> {
        let path = path.map(|s| norm_path(s.as_str()));
        if let (Some(path), Some(out)) = (&path, out) {
            return self.extract_entry(path, out);
        } else {
            for path in self.entries.keys() {
                if path == "." || path == ".." {
                    continue;
                }
                self.extract_entry(path, &mut path.clone())?;
            }
        }
        Ok(())
    }

    fn exists(&self, path: &str) -> bool {
        self.entries.contains_key(&norm_path(path))
    }

    fn add(&mut self, mode: mode_t, path: &str, file: &mut String) -> LoggedResult<()> {
        if path.ends_with('/') {
            return log_err!("path cannot end with / for add");
        }
        let file = Utf8CStr::from_string(file);
        let attr = file.get_attr()?;

        let mut content = Vec::<u8>::new();
        let rdevmajor: dev_t;
        let rdevminor: dev_t;

        // Treat symlinks as regular files as symlinks are created by the 'ln TARGET ENTRY' command
        let mode = if attr.is_file() || attr.is_symlink() {
            rdevmajor = 0;
            rdevminor = 0;
            file.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC)?
                .read_to_end(&mut content)?;
            mode | S_IFREG
        } else {
            rdevmajor = major(attr.st.st_rdev.as_()).as_();
            rdevminor = minor(attr.st.st_rdev.as_()).as_();
            if attr.is_block_device() {
                mode | S_IFBLK
            } else if attr.is_char_device() {
                mode | S_IFCHR
            } else {
                return log_err!("unsupported file type");
            }
        };

        self.entries.insert(
            norm_path(path),
            Box::new(CpioEntry {
                mode,
                uid: 0,
                gid: 0,
                rdevmajor,
                rdevminor,
                data: content,
            }),
        );
        eprintln!("Add file [{path}] ({mode:04o})");
        Ok(())
    }

    fn mkdir(&mut self, mode: mode_t, dir: &str) {
        self.entries.insert(
            norm_path(dir),
            Box::new(CpioEntry {
                mode: mode | S_IFDIR,
                uid: 0,
                gid: 0,
                rdevmajor: 0,
                rdevminor: 0,
                data: vec![],
            }),
        );
        eprintln!("Create directory [{dir}] ({mode:04o})");
    }

    fn ln(&mut self, src: &str, dst: &str) {
        self.entries.insert(
            norm_path(dst),
            Box::new(CpioEntry {
                mode: S_IFLNK,
                uid: 0,
                gid: 0,
                rdevmajor: 0,
                rdevminor: 0,
                data: norm_path(src).as_bytes().to_vec(),
            }),
        );
        eprintln!("Create symlink [{dst}] -> [{src}]");
    }

    fn mv(&mut self, from: &str, to: &str) -> LoggedResult<()> {
        let entry = self
            .entries
            .remove(&norm_path(from))
            .ok_or_log_msg(|w| w.write_fmt(format_args!("No such entry {from}")))?;
        self.entries.insert(norm_path(to), entry);
        eprintln!("Move [{from}] -> [{to}]");
        Ok(())
    }

    fn ls(&self, path: &str, recursive: bool) {
        let path = norm_path(path);
        let path = if path.is_empty() {
            path
        } else {
            "/".to_string() + path.as_str()
        };
        for (name, entry) in &self.entries {
            let p = "/".to_string() + name.as_str();
            if !p.starts_with(&path) {
                continue;
            }
            let p = p.strip_prefix(&path).unwrap();
            if !p.is_empty() && !p.starts_with('/') {
                continue;
            }
            if !recursive && !p.is_empty() && p.matches('/').count() > 1 {
                continue;
            }
            println!("{entry}\t{name}");
        }
    }
}

const MAGISK_PATCHED: i32 = 1 << 0;
const UNSUPPORTED_CPIO: i32 = 1 << 1;

impl Cpio {
    fn patch(&mut self) {
        let keep_verity = check_env("KEEPVERITY");
        let keep_force_encrypt = check_env("KEEPFORCEENCRYPT");
        eprintln!(
            "Patch with flag KEEPVERITY=[{keep_verity}] KEEPFORCEENCRYPT=[{keep_force_encrypt}]"
        );
        self.entries.retain(|name, entry| {
            let fstab = (!keep_verity || !keep_force_encrypt)
                && entry.mode & S_IFMT == S_IFREG
                && !name.starts_with(".backup")
                && !name.starts_with("twrp")
                && !name.starts_with("recovery")
                && name.starts_with("fstab");
            if !keep_verity {
                if fstab {
                    eprintln!("Found fstab file [{name}]");
                    let len = patch_verity(entry.data.as_mut_slice());
                    if len != entry.data.len() {
                        entry.data.resize(len, 0);
                    }
                } else if name == "verity_key" {
                    return false;
                }
            }
            if !keep_force_encrypt && fstab {
                let len = patch_encryption(entry.data.as_mut_slice());
                if len != entry.data.len() {
                    entry.data.resize(len, 0);
                }
            }
            true
        });
    }

    fn test(&self) -> i32 {
        for file in [
            "sbin/launch_daemonsu.sh",
            "sbin/su",
            "init.xposed.rc",
            "boot/sbin/launch_daemonsu.sh",
        ] {
            if self.exists(file) {
                return UNSUPPORTED_CPIO;
            }
        }
        for file in [
            ".backup/.magisk",
            "init.magisk.rc",
            "overlay/init.magisk.rc",
        ] {
            if self.exists(file) {
                return MAGISK_PATCHED;
            }
        }
        0
    }

    fn restore(&mut self) -> LoggedResult<()> {
        let mut backups = HashMap::<String, Box<CpioEntry>>::new();
        let mut rm_list = String::new();
        self.entries
            .extract_if(.., |name, _| name.starts_with(".backup/"))
            .for_each(|(name, mut entry)| {
                if name == ".backup/.rmlist" {
                    if let Ok(data) = str::from_utf8(&entry.data) {
                        rm_list.push_str(data);
                    }
                } else if name != ".backup/.magisk" {
                    let new_name = if name.ends_with(".xz") && entry.decompress() {
                        &name[8..name.len() - 3]
                    } else {
                        &name[8..]
                    };
                    eprintln!("Restore [{name}] -> [{new_name}]");
                    backups.insert(new_name.to_string(), entry);
                }
            });
        self.rm(".backup", false);
        if rm_list.is_empty() && backups.is_empty() {
            self.entries.clear();
            return Ok(());
        }
        for rm in rm_list.split('\0') {
            if !rm.is_empty() {
                self.rm(rm, false);
            }
        }
        self.entries.extend(backups);

        Ok(())
    }

    fn backup(&mut self, origin: &mut String, skip_compress: bool) -> LoggedResult<()> {
        let mut backups = HashMap::<String, Box<CpioEntry>>::new();
        let mut rm_list = String::new();
        backups.insert(
            ".backup".to_string(),
            Box::new(CpioEntry {
                mode: S_IFDIR,
                uid: 0,
                gid: 0,
                rdevmajor: 0,
                rdevminor: 0,
                data: vec![],
            }),
        );
        let origin = Utf8CStr::from_string(origin);
        let mut o = Cpio::load_from_file(origin)?;
        o.rm(".backup", true);
        self.rm(".backup", true);

        let mut lhs = o.entries.into_iter().peekable();
        let mut rhs = self.entries.iter().peekable();

        loop {
            enum Action<'a> {
                Backup(String, Box<CpioEntry>),
                Record(&'a String),
                Noop,
            }
            let action = match (lhs.peek(), rhs.peek()) {
                (Some((l, _)), Some((r, re))) => match l.as_str().cmp(r.as_str()) {
                    Ordering::Less => {
                        let (l, le) = lhs.next().unwrap();
                        Action::Backup(l, le)
                    }
                    Ordering::Greater => Action::Record(rhs.next().unwrap().0),
                    Ordering::Equal => {
                        let (l, le) = lhs.next().unwrap();
                        let action = if re.data != le.data {
                            Action::Backup(l, le)
                        } else {
                            Action::Noop
                        };
                        rhs.next();
                        action
                    }
                },
                (Some(_), None) => {
                    let (l, le) = lhs.next().unwrap();
                    Action::Backup(l, le)
                }
                (None, Some(_)) => Action::Record(rhs.next().unwrap().0),
                (None, None) => {
                    break;
                }
            };
            match action {
                Action::Backup(name, mut entry) => {
                    let backup = if !skip_compress && entry.compress() {
                        format!(".backup/{name}.xz")
                    } else {
                        format!(".backup/{name}")
                    };
                    eprintln!("Backup [{name}] -> [{backup}]");
                    backups.insert(backup, entry);
                }
                Action::Record(name) => {
                    eprintln!("Record new entry: [{name}] -> [.backup/.rmlist]");
                    rm_list.push_str(&format!("{name}\0"));
                }
                Action::Noop => {}
            }
        }
        if !rm_list.is_empty() {
            backups.insert(
                ".backup/.rmlist".to_string(),
                Box::new(CpioEntry {
                    mode: S_IFREG,
                    uid: 0,
                    gid: 0,
                    rdevmajor: 0,
                    rdevminor: 0,
                    data: rm_list.as_bytes().to_vec(),
                }),
            );
        }
        self.entries.extend(backups);

        Ok(())
    }
}

impl CpioEntry {
    pub(crate) fn compress(&mut self) -> bool {
        if self.mode & S_IFMT != S_IFREG {
            return false;
        }
        let mut encoder = get_encoder(FileFormat::XZ, Vec::new());
        let Ok(data): std::io::Result<Vec<u8>> = (try {
            encoder.write_all(&self.data)?;
            encoder.finish()?
        }) else {
            eprintln!("xz compression failed");
            return false;
        };

        self.data = data;
        true
    }

    pub(crate) fn decompress(&mut self) -> bool {
        if self.mode & S_IFMT != S_IFREG {
            return false;
        }

        let Ok(data): std::io::Result<Vec<u8>> = (try {
            let mut decoder = get_decoder(FileFormat::XZ, Cursor::new(&self.data));
            let mut data = Vec::new();
            std::io::copy(decoder.as_mut(), &mut data)?;
            data
        }) else {
            eprintln!("xz compression failed");
            return false;
        };

        self.data = data;
        true
    }
}

impl Display for CpioEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}\t{}\t{}\t{}\t{}:{}",
            match self.mode & S_IFMT {
                S_IFDIR => "d",
                S_IFREG => "-",
                S_IFLNK => "l",
                S_IFBLK => "b",
                S_IFCHR => "c",
                _ => "?",
            },
            if self.mode & S_IRUSR != 0 { "r" } else { "-" },
            if self.mode & S_IWUSR != 0 { "w" } else { "-" },
            if self.mode & S_IXUSR != 0 { "x" } else { "-" },
            if self.mode & S_IRGRP != 0 { "r" } else { "-" },
            if self.mode & S_IWGRP != 0 { "w" } else { "-" },
            if self.mode & S_IXGRP != 0 { "x" } else { "-" },
            if self.mode & S_IROTH != 0 { "r" } else { "-" },
            if self.mode & S_IWOTH != 0 { "w" } else { "-" },
            if self.mode & S_IXOTH != 0 { "x" } else { "-" },
            self.uid,
            self.gid,
            Size::from_bytes(self.data.len())
                .format()
                .with_style(Style::Abbreviated)
                .with_base(Base::Base10),
            self.rdevmajor,
            self.rdevminor,
        )
    }
}

pub(crate) fn cpio_commands(file: &Utf8CStr, cmds: &Vec<String>) -> LoggedResult<()> {
    let mut cpio = if file.exists() {
        Cpio::load_from_file(file)?
    } else {
        Cpio::new()
    };

    for cmd in cmds {
        if cmd.starts_with('#') {
            continue;
        }
        let mut cmd = CpioCommand::from_args(
            &["magiskboot", "cpio", file],
            cmd.split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .on_early_exit(print_cpio_usage);

        match &mut cmd.action {
            CpioAction::Test(_) => exit(cpio.test()),
            CpioAction::Restore(_) => cpio.restore()?,
            CpioAction::Patch(_) => cpio.patch(),
            CpioAction::Exists(Exists { path }) => {
                return if cpio.exists(path) {
                    Ok(())
                } else {
                    log_err!()
                };
            }
            CpioAction::Backup(Backup {
                origin,
                skip_compress,
            }) => cpio.backup(origin, *skip_compress)?,
            CpioAction::Remove(Remove { path, recursive }) => cpio.rm(path, *recursive),
            CpioAction::Move(Move { from, to }) => cpio.mv(from, to)?,
            CpioAction::MakeDir(MakeDir { mode, dir }) => cpio.mkdir(*mode, dir),
            CpioAction::Link(Link { src, dst }) => cpio.ln(src, dst),
            CpioAction::Add(Add { mode, path, file }) => cpio.add(*mode, path, file)?,
            CpioAction::Extract(Extract { paths }) => {
                if !paths.is_empty() && paths.len() != 2 {
                    log_err!("invalid arguments")?;
                }
                let mut it = paths.iter_mut();
                cpio.extract(it.next(), it.next())?;
            }
            CpioAction::List(List { path, recursive }) => {
                cpio.ls(path.as_str(), *recursive);
                return Ok(());
            }
        };
    }
    cpio.dump(file)?;
    Ok(())
}

fn x8u(x: &[u8; 8]) -> LoggedResult<u32> {
    // parse hex
    let mut ret = 0u32;
    let s = str::from_utf8(x).log_with_msg(|w| w.write_str("bad cpio header"))?;
    for c in s.chars() {
        ret = ret * 16
            + c.to_digit(16)
                .ok_or_log_msg(|w| w.write_str("bad cpio header"))?;
    }
    Ok(ret)
}

#[inline(always)]
fn align_4(x: usize) -> usize {
    (x + 3) & !3
}

#[inline(always)]
fn norm_path(path: &str) -> String {
    path.split('/')
        .filter(|x| !x.is_empty())
        .intersperse("/")
        .collect()
}

fn parse_mode(s: &str) -> Result<mode_t, String> {
    mode_t::from_str_radix(s, 8).map_err(|e| e.to_string())
}
