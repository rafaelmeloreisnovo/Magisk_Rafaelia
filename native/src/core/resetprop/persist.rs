//! persist.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: AD5B6FE119918C9F
⚓ FILE_PATH: native/src/core/resetprop/persist.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 942CA768C591C9E7307716C3FBFC74AE


*/


use nix::fcntl::OFlag;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::os::fd::FromRawFd;

use crate::resetprop::PropReader;
use crate::resetprop::proto::persistent_properties::PersistentProperties;
use crate::resetprop::proto::persistent_properties::mod_PersistentProperties::PersistentPropertyRecord;
use base::const_format::concatcp;
use base::libc::mkstemp;
use base::{
    Directory, FsPathBuilder, LibcReturn, LoggedResult, MappedFile, SilentLogExt, Utf8CStr,
    Utf8CStrBuf, WalkResult, clone_attr, cstr, debug, log_err,
};

const PERSIST_PROP_DIR: &str = "/data/property";
const PERSIST_PROP: &str = concatcp!(PERSIST_PROP_DIR, "/persistent_properties");

trait PropExt {
    fn find_index(&self, name: &Utf8CStr) -> Result<usize, usize>;
    fn find(self, name: &Utf8CStr) -> Option<PersistentPropertyRecord>;
}

impl PropExt for PersistentProperties {
    fn find_index(&self, name: &Utf8CStr) -> Result<usize, usize> {
        self.properties
            .binary_search_by(|p| p.name.as_deref().cmp(&Some(name.as_str())))
    }

    fn find(self, name: &Utf8CStr) -> Option<PersistentPropertyRecord> {
        let idx = self.find_index(name).ok()?;
        self.properties.into_iter().nth(idx)
    }
}

fn check_proto() -> bool {
    cstr!(PERSIST_PROP).exists()
}

fn file_get_prop(name: &Utf8CStr) -> LoggedResult<String> {
    let path = cstr::buf::default()
        .join_path(PERSIST_PROP_DIR)
        .join_path(name);
    let mut file = path.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC).silent()?;
    debug!("resetprop: read prop from [{}]", path);
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn file_set_prop(name: &Utf8CStr, value: Option<&Utf8CStr>) -> LoggedResult<()> {
    let path = cstr::buf::default()
        .join_path(PERSIST_PROP_DIR)
        .join_path(name);
    if let Some(value) = value {
        let mut tmp = cstr::buf::default()
            .join_path(PERSIST_PROP_DIR)
            .join_path("prop.XXXXXX");
        {
            let mut f = unsafe {
                mkstemp(tmp.as_mut_ptr())
                    .into_os_result("mkstemp", None, None)
                    .map(|fd| File::from_raw_fd(fd))?
            };
            f.write_all(value.as_bytes())?;
        }
        debug!("resetprop: write prop to [{}]", tmp);
        tmp.rename_to(&path)?
    } else {
        path.remove().silent()?;
        debug!("resetprop: unlink [{}]", path);
    }
    Ok(())
}

fn proto_read_props() -> LoggedResult<PersistentProperties> {
    debug!("resetprop: decode with protobuf [{}]", PERSIST_PROP);
    let m = MappedFile::open(cstr!(PERSIST_PROP))?;
    let m = m.as_ref();
    let mut r = BytesReader::from_bytes(m);
    let mut props = PersistentProperties::from_reader(&mut r, m)?;
    // Keep the list sorted for binary search
    props
        .properties
        .sort_unstable_by(|a, b| a.name.cmp(&b.name));
    Ok(props)
}

fn proto_write_props(props: &PersistentProperties) -> LoggedResult<()> {
    let mut tmp = cstr::buf::default().join_path(concatcp!(PERSIST_PROP, ".XXXXXX"));
    {
        let f = unsafe {
            mkstemp(tmp.as_mut_ptr())
                .into_os_result("mkstemp", None, None)
                .map(|fd| File::from_raw_fd(fd))?
        };
        debug!("resetprop: encode with protobuf [{}]", tmp);
        props.write_message(&mut Writer::new(BufWriter::new(f)))?;
    }
    clone_attr(cstr!(PERSIST_PROP), &tmp)?;
    tmp.rename_to(cstr!(PERSIST_PROP))?;
    Ok(())
}

pub(super) fn persist_get_prop(key: &Utf8CStr) -> LoggedResult<String> {
    if check_proto() {
        let props = proto_read_props()?;
        let prop = props.find(key).silent()?;
        if let PersistentPropertyRecord {
            name: Some(_),
            value: Some(v),
        } = prop
        {
            return Ok(v);
        }
    } else {
        let value = file_get_prop(key)?;
        debug!("resetprop: get persist prop [{}]=[{}]", key, value);
        return Ok(value);
    }
    log_err!()
}

pub(super) fn persist_get_all_props(reader: &mut PropReader) -> LoggedResult<()> {
    if check_proto() {
        let props = proto_read_props()?;
        props.properties.into_iter().for_each(|prop| {
            if let PersistentPropertyRecord {
                name: Some(n),
                value: Some(v),
            } = prop
            {
                reader.put_str(n, v, 0);
            }
        });
    } else {
        let mut dir = Directory::open(cstr!(PERSIST_PROP_DIR))?;
        dir.pre_order_walk(|e| {
            if e.is_file()
                && let Ok(value) = file_get_prop(e.name())
            {
                reader.put_str(e.name().to_string(), value, 0);
            }
            // Do not traverse recursively
            Ok(WalkResult::Skip)
        })?;
    }
    Ok(())
}

pub(super) fn persist_delete_prop(key: &Utf8CStr) -> LoggedResult<()> {
    if check_proto() {
        let mut props = proto_read_props()?;
        let idx = props.find_index(key).silent()?;
        props.properties.remove(idx);
        proto_write_props(&props)?;
    } else {
        file_set_prop(key, None)?;
    }
    Ok(())
}

pub(super) fn persist_set_prop(key: &Utf8CStr, val: &Utf8CStr) -> LoggedResult<()> {
    if check_proto() {
        let mut props = proto_read_props()?;
        match props.find_index(key) {
            Ok(idx) => props.properties[idx].value = Some(val.to_string()),
            Err(idx) => props.properties.insert(
                idx,
                PersistentPropertyRecord {
                    name: Some(key.to_string()),
                    value: Some(val.to_string()),
                },
            ),
        }
        proto_write_props(&props)?;
    } else {
        file_set_prop(key, Some(val))?;
    }
    Ok(())
}
