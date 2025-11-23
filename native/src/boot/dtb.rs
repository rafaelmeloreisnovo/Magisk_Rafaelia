//! dtb.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: CCFEA32F9B6D069D
⚓ FILE_PATH: native/src/boot/dtb.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: FFFFFB578963054B52C14090E1973F40


*/


use argh::FromArgs;
use base::{LoggedResult, MappedFile, Utf8CStr, argh};
use fdt::node::{FdtNode, NodeProperty};
use fdt::{Fdt, FdtError};
use std::cell::UnsafeCell;

use crate::check_env;
use crate::patch::patch_verity;

#[derive(FromArgs)]
#[argh(subcommand)]
pub(crate) enum DtbAction {
    Print(Print),
    Patch(Patch),
    Test(Test),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "print")]
pub(crate) struct Print {
    #[argh(switch, short = 'f', long = none)]
    fstab: bool,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "patch")]
pub(crate) struct Patch {}

#[derive(FromArgs)]
#[argh(subcommand, name = "test")]
pub(crate) struct Test {}

pub(crate) fn print_dtb_usage() {
    eprintln!(
        r#"Usage: magiskboot dtb <file> <action> [args...]
Do dtb related actions to <file>.

Supported actions:
  print [-f]
    Print all contents of dtb for debugging
    Specify [-f] to only print fstab nodes
  patch
    Search for fstab and remove verity/avb
    Modifications are done directly to the file in-place
    Configure with env variables: KEEPVERITY
  test
    Test the fstab's status
    Return values:
    0:valid    1:error"#
    );
}

const MAX_PRINT_LEN: usize = 32;

fn print_node(node: &FdtNode) {
    fn pretty_node(depth_set: &[bool]) {
        let mut depth_set = depth_set.iter().peekable();
        while let Some(depth) = depth_set.next() {
            let last = depth_set.peek().is_none();
            if *depth {
                if last {
                    print!("├── ");
                } else {
                    print!("│   ");
                }
            } else if last {
                print!("└── ");
            } else {
                print!("    ");
            }
        }
    }

    fn pretty_prop(depth_set: &[bool]) {
        let mut depth_set = depth_set.iter().peekable();
        while let Some(depth) = depth_set.next() {
            let last = depth_set.peek().is_none();
            if *depth {
                if last {
                    print!("│  ");
                } else {
                    print!("│   ");
                }
            } else if last {
                print!("└─ ");
            } else {
                print!("    ");
            }
        }
    }

    fn do_print_node(node: &FdtNode, depth_set: &mut Vec<bool>) {
        pretty_node(depth_set);
        let depth = depth_set.len();
        depth_set.push(true);
        println!("{}", node.name);
        let mut properties = node.properties().peekable();
        let mut children = node.children().peekable();
        while let Some(NodeProperty { name, value }) = properties.next() {
            let size = value.len();
            let is_str = !(size > 1 && value[0] == 0)
                && matches!(value.last(), Some(0u8) | None)
                && value.iter().all(|c| *c == 0 || (*c >= 32 && *c < 127));

            if depth_set[depth] && properties.peek().is_none() && children.peek().is_none() {
                depth_set[depth] = false;
            }

            pretty_prop(depth_set);
            if is_str {
                println!(
                    "[{}]: [\"{}\"]",
                    name,
                    if value.is_empty() {
                        ""
                    } else {
                        unsafe { Utf8CStr::from_bytes_unchecked(value) }
                    }
                );
            } else if size > MAX_PRINT_LEN {
                println!("[{name}]: <bytes>({size})");
            } else {
                println!("[{name}]: {value:02x?}");
            }
        }

        while let Some(child) = children.next() {
            if depth_set[depth] && children.peek().is_none() {
                depth_set[depth] = false;
            }
            do_print_node(&child, depth_set);
        }
        depth_set.pop();
    }

    do_print_node(node, &mut vec![]);
}

fn for_each_fdt<F: FnMut(usize, Fdt) -> LoggedResult<()>>(
    file: &Utf8CStr,
    rw: bool,
    mut f: F,
) -> LoggedResult<()> {
    eprintln!("Loading dtbs from [{file}]");
    let file = if rw {
        MappedFile::open_rw(file)?
    } else {
        MappedFile::open(file)?
    };
    let mut buf = Some(file.as_ref());
    let mut dtb_num = 0usize;
    while let Some(slice) = buf {
        let slice = if let Some(pos) = slice.windows(4).position(|w| w == b"\xd0\x0d\xfe\xed") {
            &slice[pos..]
        } else {
            break;
        };
        if slice.len() < 40 {
            break;
        }
        let fdt = match Fdt::new(slice) {
            Err(FdtError::BufferTooSmall) => {
                eprintln!("dtb.{dtb_num:04} is truncated");
                break;
            }
            Ok(fdt) => fdt,
            e => e?,
        };

        let size = fdt.total_size();

        f(dtb_num, fdt)?;

        dtb_num += 1;
        buf = Some(&slice[size..]);
    }
    Ok(())
}

fn find_fstab<'b, 'a: 'b>(fdt: &'b Fdt<'a>) -> Option<FdtNode<'b, 'a>> {
    fdt.all_nodes().find(|node| node.name == "fstab")
}

fn dtb_print(file: &Utf8CStr, fstab: bool) -> LoggedResult<()> {
    for_each_fdt(file, false, |n, fdt| {
        if fstab {
            if let Some(fstab) = find_fstab(&fdt) {
                eprintln!("Found fstab in dtb.{n:04}");
                print_node(&fstab);
            }
        } else if let Some(mut root) = fdt.find_node("/") {
            eprintln!("Printing dtb.{n:04}");
            if root.name.is_empty() {
                root.name = "/";
            }
            print_node(&root);
        }
        Ok(())
    })
}

fn dtb_test(file: &Utf8CStr) -> LoggedResult<bool> {
    let mut ret = true;
    for_each_fdt(file, false, |_, fdt| {
        if let Some(fstab) = find_fstab(&fdt) {
            for child in fstab.children() {
                if child.name != "system" {
                    continue;
                }
                if let Some(mount_point) = child.property("mnt_point")
                    && mount_point.value == b"/system_root\0"
                {
                    ret = false;
                    break;
                }
            }
        }
        Ok(())
    })?;
    Ok(ret)
}

fn dtb_patch(file: &Utf8CStr) -> LoggedResult<bool> {
    let keep_verity = check_env("KEEPVERITY");
    let mut patched = false;
    for_each_fdt(file, true, |n, fdt| {
        for node in fdt.all_nodes() {
            if node.name != "chosen" {
                continue;
            }
            if let Some(boot_args) = node.property("bootargs") {
                boot_args.value.windows(14).for_each(|w| {
                    if w == b"skip_initramfs" {
                        let w = unsafe {
                            &mut *std::mem::transmute::<&[u8], &UnsafeCell<[u8]>>(w).get()
                        };
                        w[..=4].copy_from_slice(b"want");
                        eprintln!("Patch [skip_initramfs] -> [want_initramfs] in dtb.{n:04}");
                        patched = true;
                    }
                });
            }
        }
        if keep_verity {
            return Ok(());
        }
        if let Some(fstab) = find_fstab(&fdt) {
            for child in fstab.children() {
                if let Some(flags) = child.property("fsmgr_flags") {
                    let flags = unsafe {
                        &mut *std::mem::transmute::<&[u8], &UnsafeCell<[u8]>>(flags.value).get()
                    };
                    if patch_verity(flags) != flags.len() {
                        patched = true;
                    }
                }
            }
        }
        Ok(())
    })?;
    Ok(patched)
}

pub(crate) fn dtb_commands(file: &Utf8CStr, action: &DtbAction) -> LoggedResult<bool> {
    match action {
        DtbAction::Print(Print { fstab }) => {
            dtb_print(file, *fstab)?;
            Ok(true)
        }
        DtbAction::Test(_) => Ok(dtb_test(file)?),
        DtbAction::Patch(_) => Ok(dtb_patch(file)?),
    }
}
