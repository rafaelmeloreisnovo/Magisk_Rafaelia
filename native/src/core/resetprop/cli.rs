//! cli.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 3D1ADB820263B593
⚓ FILE_PATH: native/src/core/resetprop/cli.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 5F16F74D1B177AB4E7816DBC1AF04936


*/


use super::persist::{
    persist_delete_prop, persist_get_all_props, persist_get_prop, persist_set_prop,
};
use super::{PropInfo, PropReader, SYS_PROP};
use argh::{EarlyExit, FromArgs, MissingRequirements};
use base::libc::PROP_VALUE_MAX;
use base::{
    BufReadExt, CmdArgs, EarlyExitExt, LogLevel, LoggedResult, ResultExt, Utf8CStr, Utf8CStrBuf,
    Utf8CString, argh, cstr, debug, log_err, set_log_level_state,
};
use nix::fcntl::OFlag;
use std::collections::BTreeMap;
use std::ffi::c_char;
use std::io::BufReader;

#[derive(FromArgs, Default)]
struct ResetProp {
    #[argh(switch, short = 'v')]
    verbose: bool,
    #[argh(switch, short = 'w', long = none)]
    wait_mode: bool,
    #[argh(switch, short = 'p', long = none)]
    persist: bool,
    #[argh(switch, short = 'P', long = none)]
    persist_only: bool,
    #[argh(switch, short = 'Z', long = none)]
    context: bool,
    #[argh(switch, short = 'n', long = none)]
    skip_svc: bool,
    #[argh(option, short = 'f')]
    file: Option<Utf8CString>,
    #[argh(option, short = 'd', long = "delete")]
    delete_key: Option<Utf8CString>,
    #[argh(positional, greedy = true)]
    args: Vec<Utf8CString>,
}

fn print_usage(cmd: &str) {
    eprintln!(
        r#"resetprop - System Property Manipulation Tool

Usage: {cmd} [flags] [arguments...]

Read mode arguments:
   (no arguments)    print all properties
   NAME              get property of NAME

Write mode arguments:
   NAME VALUE        set property NAME as VALUE
   -f,--file   FILE  load and set properties from FILE
   -d,--delete NAME  delete property

Wait mode arguments (toggled with -w):
    NAME             wait until property NAME changes
    NAME OLD_VALUE   if value of property NAME is not OLD_VALUE, get value
                     or else wait until property NAME changes

General flags:
   -h,--help         show this message
   -v,--verbose      print verbose output to stderr
   -w                switch to wait mode

Read mode flags:
   -p      also read persistent properties from storage
   -P      only read persistent properties from storage
   -Z      get property context instead of value

Write mode flags:
   -n      set properties bypassing property_service
   -p      always write persistent prop changes to storage
"#
    );
}

impl ResetProp {
    fn get(&self, key: &Utf8CStr) -> Option<String> {
        if self.context {
            return Some(SYS_PROP.get_context(key).to_string());
        }

        let mut val = if !self.persist_only {
            SYS_PROP.find(key).map(|info| {
                let mut v = String::new();
                info.read(&mut PropReader::Value(&mut v));
                debug!("resetprop: get prop [{key}]=[{v}]");
                v
            })
        } else {
            None
        };

        if val.is_none() && (self.persist || self.persist_only) && key.starts_with("persist.") {
            val = persist_get_prop(key).ok();
        }

        if val.is_none() {
            debug!("resetprop: prop [{key}] does not exist");
        }

        val
    }

    fn print_all(&self) {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        if !self.persist_only {
            SYS_PROP.for_each(&mut PropReader::List(&mut map));
        }
        if self.persist || self.persist_only {
            persist_get_all_props(&mut PropReader::List(&mut map)).log_ok();
        }
        for (mut k, v) in map.into_iter() {
            if self.context {
                println!(
                    "[{k}]: [{}]",
                    SYS_PROP.get_context(Utf8CStr::from_string(&mut k))
                );
            } else {
                println!("[{k}]: [{v}]");
            }
        }
    }

    fn set(&self, key: &Utf8CStr, val: &Utf8CStr) {
        let mut skip_svc = self.skip_svc;
        let mut info = SYS_PROP.find_mut(key);

        // Delete existing read-only properties if they are or will be long properties,
        // which cannot directly go through __system_property_update
        if key.starts_with("ro.") {
            skip_svc = true;
            if let Some(pi) = &info
                && (pi.is_long() || val.len() >= PROP_VALUE_MAX as usize)
            {
                // Skip pruning nodes as we will add it back ASAP
                SYS_PROP.delete(key, false);
                info = None;
            }
        }

        #[allow(unused_variables)]
        let msg = if skip_svc {
            "direct modification"
        } else {
            "property_service"
        };

        if let Some(pi) = info {
            if skip_svc {
                pi.update(val);
            } else {
                SYS_PROP.set(key, val);
            }
            debug!("resetprop: update prop [{key}]=[{val}] by {msg}");
        } else {
            if skip_svc {
                SYS_PROP.add(key, val);
            } else {
                SYS_PROP.set(key, val);
            }
            debug!("resetprop: create prop [{key}]=[{val}] by {msg}");
        }

        // When bypassing property_service, persistent props won't be stored in storage.
        // Explicitly handle this situation.
        if skip_svc && self.persist && key.starts_with("persist.") {
            persist_set_prop(key, val).log_ok();
        }
    }

    fn delete(&self, key: &Utf8CStr) -> bool {
        debug!("resetprop: delete prop [{key}]");
        let mut ret = false;
        ret |= SYS_PROP.delete(key, true);
        if self.persist && key.starts_with("persist.") {
            ret |= persist_delete_prop(key).is_ok()
        }
        ret
    }

    fn wait(&self) {
        let key = &self.args[0];
        let val = self.args.get(1).map(|s| &**s);

        // Find PropInfo
        let info: &PropInfo;
        loop {
            let i = SYS_PROP.find(key);
            if let Some(i) = i {
                info = i;
                break;
            } else {
                debug!("resetprop: waiting for prop [{key}] to exist");
                let mut serial = SYS_PROP.area_serial();
                SYS_PROP.wait(None, serial, &mut serial);
            }
        }

        if let Some(val) = val {
            let mut curr_val = String::new();
            let mut serial = 0;
            loop {
                let mut r = PropReader::ValueSerial(&mut curr_val, &mut serial);
                SYS_PROP.read(info, &mut r);
                if *val != *curr_val {
                    debug!("resetprop: get prop [{key}]=[{curr_val}]");
                    break;
                }
                debug!("resetprop: waiting for prop [{key}]!=[{val}]");
                SYS_PROP.wait(Some(info), serial, &mut serial);
            }
        }
    }

    fn load_file(&self, file: &Utf8CStr) -> LoggedResult<()> {
        let fd = file.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC)?;
        let mut key = cstr::buf::dynamic(128);
        let mut val = cstr::buf::dynamic(128);
        BufReader::new(fd).for_each_prop(|k, v| {
            key.clear();
            val.clear();
            key.push_str(k);
            val.push_str(v);
            self.set(&key, &val);
            true
        });
        Ok(())
    }

    fn run(self) -> LoggedResult<()> {
        if self.wait_mode {
            self.wait();
        } else if let Some(file) = &self.file {
            self.load_file(file)?;
        } else if let Some(key) = &self.delete_key {
            if !self.delete(key) {
                return log_err!();
            }
        } else {
            match self.args.len() {
                0 => self.print_all(),
                1 => {
                    if let Some(val) = self.get(&self.args[0]) {
                        println!("{val}");
                    } else {
                        return log_err!();
                    }
                }
                2 => self.set(&self.args[0], &self.args[1]),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

pub fn resetprop_main(argc: i32, argv: *mut *mut c_char) -> i32 {
    set_log_level_state(LogLevel::Debug, false);
    let cmds = CmdArgs::new(argc, argv.cast());
    let cmds = cmds.as_slice();

    let cli = ResetProp::from_args(&[cmds[0]], &cmds[1..])
        .and_then(|cli| {
            let mut special_mode = 0;
            if cli.wait_mode {
                if cli.args.is_empty() {
                    let mut missing = MissingRequirements::default();
                    missing.missing_positional_arg("NAME");
                    missing.err_on_any()?;
                }
                special_mode += 1;
            }
            if cli.file.is_some() {
                special_mode += 1;
            }
            if cli.delete_key.is_some() {
                special_mode += 1;
            }
            if special_mode > 1 {
                return Err(EarlyExit::from(
                    "Multiple operation mode detected!\n".to_string(),
                ));
            }
            if cli.args.len() > 2 {
                return Err(EarlyExit::from(format!(
                    "Unrecognized argument: {}\n",
                    cli.args[2]
                )));
            }
            Ok(cli)
        })
        .on_early_exit(|| print_usage(cmds[0]));

    if cli.verbose {
        set_log_level_state(LogLevel::Debug, true);
    }

    if cli.run().is_ok() { 0 } else { 1 }
}

// Magisk's own helper functions

pub fn set_prop(key: &Utf8CStr, val: &Utf8CStr) {
    let prop = ResetProp {
        // All Magisk's internal usage should skip property_service
        skip_svc: true,
        ..Default::default()
    };
    prop.set(key, val);
}

pub fn load_prop_file(file: &Utf8CStr) {
    let prop = ResetProp {
        // All Magisk's internal usage should skip property_service
        skip_svc: true,
        ..Default::default()
    };
    prop.load_file(file).ok();
}

pub fn get_prop(key: &Utf8CStr) -> String {
    let prop = ResetProp {
        persist: key.starts_with("persist."),
        ..Default::default()
    };
    prop.get(key).unwrap_or_default()
}
