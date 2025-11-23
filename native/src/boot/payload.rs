//! payload.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: BC75BBA13630AC7E
⚓ FILE_PATH: native/src/boot/payload.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: C7A5352C56249844E0A3A92F9F0E08D7


*/


use crate::compress::get_decoder;
use crate::ffi::check_fmt;
use crate::proto::update_metadata::DeltaArchiveManifest;
use crate::proto::update_metadata::mod_InstallOperation::Type;
use base::{LoggedError, LoggedResult, ReadSeekExt, ResultExt, WriteExt, error};
use byteorder::{BigEndian, ReadBytesExt};
use quick_protobuf::{BytesReader, MessageRead};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::os::fd::FromRawFd;

macro_rules! bad_payload {
    ($msg:literal) => {{
        error!(concat!("Invalid payload: ", $msg));
        LoggedError::default()
    }};
    ($($args:tt)*) => {{
        error!("Invalid payload: {}", format_args!($($args)*));
        LoggedError::default()
    }};
}

const PAYLOAD_MAGIC: &str = "CrAU";

pub fn extract_boot_from_payload(
    in_path: &str,
    partition_name: Option<&str>,
    out_path: Option<&str>,
) -> LoggedResult<()> {
    let mut reader = BufReader::new(if in_path == "-" {
        unsafe { File::from_raw_fd(0) }
    } else {
        File::open(in_path).log_with_msg(|w| write!(w, "Cannot open '{in_path}'"))?
    });

    let buf = &mut [0u8; 4];
    reader.read_exact(buf)?;

    if buf != PAYLOAD_MAGIC.as_bytes() {
        return Err(bad_payload!("invalid magic"));
    }

    let version = reader.read_u64::<BigEndian>()?;
    if version != 2 {
        return Err(bad_payload!("unsupported version: {}", version));
    }

    let manifest_len = reader.read_u64::<BigEndian>()? as usize;
    if manifest_len == 0 {
        return Err(bad_payload!("manifest length is zero"));
    }

    let manifest_sig_len = reader.read_u32::<BigEndian>()?;
    if manifest_sig_len == 0 {
        return Err(bad_payload!("manifest signature length is zero"));
    }

    let mut buf = vec![0; manifest_len];

    let manifest = {
        let manifest = &mut buf[..manifest_len];
        reader.read_exact(manifest)?;
        let mut br = BytesReader::from_bytes(manifest);
        DeltaArchiveManifest::from_reader(&mut br, manifest)?
    };
    if manifest.get_minor_version() != 0 {
        return Err(bad_payload!(
            "delta payloads are not supported, please use a full payload file"
        ));
    }

    let block_size = manifest.get_block_size() as u64;

    let partition = match partition_name {
        None => {
            let boot = manifest
                .partitions
                .iter()
                .find(|p| p.partition_name == "init_boot");
            let boot = match boot {
                Some(boot) => Some(boot),
                None => manifest
                    .partitions
                    .iter()
                    .find(|p| p.partition_name == "boot"),
            };
            boot.ok_or_else(|| bad_payload!("boot partition not found"))?
        }
        Some(name) => manifest
            .partitions
            .iter()
            .find(|p| p.partition_name.as_str() == name)
            .ok_or_else(|| bad_payload!("partition '{}' not found", name))?,
    };

    let out_str: String;
    let out_path = match out_path {
        None => {
            out_str = format!("{}.img", partition.partition_name);
            out_str.as_str()
        }
        Some(s) => s,
    };

    let mut out_file =
        File::create(out_path).log_with_msg(|w| write!(w, "Cannot write to '{out_path}'"))?;

    // Skip the manifest signature
    reader.skip(manifest_sig_len as usize)?;

    // Sort the install operations with data_offset so we will only ever need to seek forward
    // This makes it possible to support non-seekable input file descriptors
    let mut operations = partition.operations.clone();
    operations.sort_by_key(|e| e.data_offset.unwrap_or(0));
    let mut curr_data_offset: u64 = 0;

    for operation in operations.iter() {
        let data_len = operation
            .data_length
            .ok_or_else(|| bad_payload!("data length not found"))? as usize;

        let data_offset = operation
            .data_offset
            .ok_or_else(|| bad_payload!("data offset not found"))?;

        let data_type = operation.type_pb;

        buf.resize(data_len, 0u8);
        let data = &mut buf[..data_len];

        // Skip to the next offset and read data
        let skip = data_offset - curr_data_offset;
        reader.skip(skip as usize)?;
        reader.read_exact(data)?;
        curr_data_offset = data_offset + data_len as u64;

        let out_offset = operation
            .dst_extents
            .first()
            .ok_or_else(|| bad_payload!("dst extents not found"))?
            .start_block
            .ok_or_else(|| bad_payload!("start block not found"))?
            * block_size;

        match data_type {
            Type::REPLACE => {
                out_file.seek(SeekFrom::Start(out_offset))?;
                out_file.write_all(data)?;
            }
            Type::ZERO => {
                for ext in operation.dst_extents.iter() {
                    let out_seek = ext
                        .start_block
                        .ok_or_else(|| bad_payload!("start block not found"))?
                        * block_size;
                    let num_blocks = ext
                        .num_blocks
                        .ok_or_else(|| bad_payload!("num blocks not found"))?;
                    out_file.seek(SeekFrom::Start(out_seek))?;
                    out_file.write_zeros(num_blocks as usize)?;
                }
            }
            Type::REPLACE_BZ | Type::REPLACE_XZ => {
                out_file.seek(SeekFrom::Start(out_offset))?;
                let fmt = check_fmt(data);

                let mut decoder = get_decoder(fmt, Cursor::new(data));
                let Ok(_): std::io::Result<()> = (try {
                    std::io::copy(decoder.as_mut(), &mut out_file)?;
                }) else {
                    return Err(bad_payload!("decompression failed"));
                };
            }
            _ => return Err(bad_payload!("unsupported operation type")),
        };
    }

    Ok(())
}
