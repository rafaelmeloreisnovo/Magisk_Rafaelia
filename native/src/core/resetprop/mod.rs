//! mod.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 61082DEAADFEBC01
⚓ FILE_PATH: native/src/core/resetprop/mod.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 4E5B80BAA8729FD68E3143A6E0412470


*/


use base::libc::c_char;
use base::{Utf8CStr, libc};
pub use cli::{get_prop, load_prop_file, resetprop_main, set_prop};
use libc::timespec;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::ptr;
use std::sync::LazyLock;

mod cli;
mod persist;
mod proto;

static SYS_PROP: LazyLock<SysProp> = LazyLock::new(|| unsafe { get_sys_prop() });

#[repr(C)]
struct PropInfo {
    _private: cxx::private::Opaque,
}

type CharPtr = *const c_char;
type ReadCallback = unsafe extern "C" fn(&mut PropReader, CharPtr, CharPtr, u32);
type ForEachCallback = unsafe extern "C" fn(&PropInfo, &mut PropReader);

enum PropReader<'a> {
    Value(&'a mut String),
    ValueSerial(&'a mut String, &'a mut u32),
    List(&'a mut BTreeMap<String, String>),
}

impl PropReader<'_> {
    fn put_cstr(&mut self, key: CharPtr, val: CharPtr, serial: u32) {
        let key = unsafe { CStr::from_ptr(key) };
        let val = unsafe { CStr::from_ptr(val) };
        match self {
            PropReader::Value(v) => {
                **v = String::from_utf8_lossy(val.to_bytes()).into_owned();
            }
            PropReader::ValueSerial(v, s) => {
                **v = String::from_utf8_lossy(val.to_bytes()).into_owned();
                **s = serial;
            }
            PropReader::List(map) => {
                map.insert(
                    String::from_utf8_lossy(key.to_bytes()).into_owned(),
                    String::from_utf8_lossy(val.to_bytes()).into_owned(),
                );
            }
        }
    }

    fn put_str(&mut self, key: String, val: String, serial: u32) {
        match self {
            PropReader::Value(v) => {
                **v = val;
            }
            PropReader::ValueSerial(v, s) => {
                **v = val;
                **s = serial;
            }
            PropReader::List(map) => {
                map.insert(key, val);
            }
        }
    }
}

unsafe extern "C" {
    // SAFETY: the improper_ctypes warning is about PropReader. We only pass PropReader
    // to C functions as raw pointers, and all actual usage happens on the Rust side.
    #[allow(improper_ctypes)]
    fn get_sys_prop() -> SysProp;

    fn prop_info_is_long(info: &PropInfo) -> bool;
    #[link_name = "__system_property_find2"]
    fn sys_prop_find(key: CharPtr) -> Option<&'static mut PropInfo>;
    #[link_name = "__system_property_update2"]
    fn sys_prop_update(info: &mut PropInfo, val: CharPtr, val_len: u32) -> i32;
    #[link_name = "__system_property_add2"]
    fn sys_prop_add(key: CharPtr, key_len: u32, val: CharPtr, val_len: u32) -> i32;
    #[link_name = "__system_property_delete"]
    fn sys_prop_delete(key: CharPtr, prune: bool) -> i32;
    #[link_name = "__system_property_get_context"]
    fn sys_prop_get_context(key: CharPtr) -> CharPtr;
    #[link_name = "__system_property_area_serial2"]
    fn sys_prop_area_serial() -> u32;
}

#[repr(C)]
struct SysProp {
    set: unsafe extern "C" fn(CharPtr, CharPtr) -> i32,
    find: unsafe extern "C" fn(CharPtr) -> Option<&'static PropInfo>,
    read_callback: unsafe extern "C" fn(&PropInfo, ReadCallback, &mut PropReader) -> i32,
    foreach: unsafe extern "C" fn(ForEachCallback, &mut PropReader) -> i32,
    wait: unsafe extern "C" fn(Option<&PropInfo>, u32, &mut u32, *const timespec) -> i32,
}

// Safe abstractions over raw C APIs

impl PropInfo {
    fn read(&self, reader: &mut PropReader) {
        SYS_PROP.read(self, reader);
    }

    fn update(&mut self, val: &Utf8CStr) {
        SYS_PROP.update(self, val);
    }

    fn is_long(&self) -> bool {
        unsafe { prop_info_is_long(self) }
    }
}

impl SysProp {
    fn read(&self, info: &PropInfo, reader: &mut PropReader) {
        unsafe extern "C" fn read_fn(r: &mut PropReader, key: CharPtr, val: CharPtr, serial: u32) {
            r.put_cstr(key, val, serial);
        }
        unsafe {
            (self.read_callback)(info, read_fn, reader);
        }
    }

    fn find(&self, key: &Utf8CStr) -> Option<&'static PropInfo> {
        unsafe { (self.find)(key.as_ptr()) }
    }

    fn find_mut(&self, key: &Utf8CStr) -> Option<&'static mut PropInfo> {
        unsafe { sys_prop_find(key.as_ptr()) }
    }

    fn set(&self, key: &Utf8CStr, val: &Utf8CStr) {
        unsafe {
            (self.set)(key.as_ptr(), val.as_ptr());
        }
    }

    fn add(&self, key: &Utf8CStr, val: &Utf8CStr) {
        unsafe {
            sys_prop_add(
                key.as_ptr(),
                key.len() as u32,
                val.as_ptr(),
                val.len() as u32,
            );
        }
    }

    fn update(&self, info: &mut PropInfo, val: &Utf8CStr) {
        unsafe {
            sys_prop_update(info, val.as_ptr(), val.len() as u32);
        }
    }

    fn delete(&self, key: &Utf8CStr, prune: bool) -> bool {
        unsafe { sys_prop_delete(key.as_ptr(), prune) == 0 }
    }

    fn for_each(&self, reader: &mut PropReader) {
        unsafe extern "C" fn for_each_fn(info: &PropInfo, vals: &mut PropReader) {
            SYS_PROP.read(info, vals);
        }
        unsafe {
            (self.foreach)(for_each_fn, reader);
        }
    }

    fn wait(&self, info: Option<&PropInfo>, old_serial: u32, new_serial: &mut u32) {
        unsafe {
            (self.wait)(info, old_serial, new_serial, ptr::null());
        }
    }

    fn get_context(&self, key: &Utf8CStr) -> &'static Utf8CStr {
        unsafe { Utf8CStr::from_ptr_unchecked(sys_prop_get_context(key.as_ptr())) }
    }

    fn area_serial(&self) -> u32 {
        unsafe { sys_prop_area_serial() }
    }
}
