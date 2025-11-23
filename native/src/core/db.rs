//! db.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 6897D55085101919
⚓ FILE_PATH: native/src/core/db.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: E6B860402851EC159DD07991B4AEE236


*/


#![allow(improper_ctypes, improper_ctypes_definitions)]
use crate::daemon::{MAGISKD, MagiskD};
use crate::ffi::{
    DbEntryKey, DbStatement, DbValues, MntNsMode, open_and_init_db, sqlite3, sqlite3_errstr,
};
use crate::socket::{IpcRead, IpcWrite};
use DbArg::{Integer, Text};
use base::{LoggedResult, ResultExt, Utf8CStr};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::c_void;
use std::io::{BufReader, BufWriter};
use std::os::unix::net::UnixStream;
use std::pin::Pin;
use std::ptr;
use std::ptr::NonNull;
use thiserror::Error;

fn sqlite_err_str(code: i32) -> &'static Utf8CStr {
    // SAFETY: sqlite3 always returns UTF-8 strings
    unsafe { Utf8CStr::from_ptr_unchecked(sqlite3_errstr(code)) }
}

#[repr(transparent)]
#[derive(Error, Debug)]
#[error("sqlite3: {}", sqlite_err_str(self.0))]
pub struct SqliteError(i32);

pub type SqliteResult<T> = Result<T, SqliteError>;

pub trait SqliteReturn {
    fn sql_result(self) -> SqliteResult<()>;
}

impl SqliteReturn for i32 {
    fn sql_result(self) -> SqliteResult<()> {
        if self != 0 {
            Err(SqliteError(self))
        } else {
            Ok(())
        }
    }
}

pub trait SqlTable {
    fn on_row(&mut self, columns: &[String], values: &DbValues);
}

impl<T> SqlTable for T
where
    T: FnMut(&[String], &DbValues),
{
    fn on_row(&mut self, columns: &[String], values: &DbValues) {
        self.call_mut((columns, values))
    }
}

#[derive(Default)]
pub struct DbSettings {
    pub root_access: RootAccess,
    pub multiuser_mode: MultiuserMode,
    pub mnt_ns: MntNsMode,
    pub boot_count: i32,
    pub denylist: bool,
    pub zygisk: bool,
}

#[repr(i32)]
#[derive(Default, FromPrimitive)]
pub enum RootAccess {
    Disabled,
    AppsOnly,
    AdbOnly,
    #[default]
    AppsAndAdb,
}

#[repr(i32)]
#[derive(Default, FromPrimitive)]
pub enum MultiuserMode {
    #[default]
    OwnerOnly,
    OwnerManaged,
    User,
}

impl Default for MntNsMode {
    fn default() -> Self {
        MntNsMode::Requester
    }
}

impl DbEntryKey {
    fn to_str(self) -> &'static str {
        match self {
            DbEntryKey::RootAccess => "root_access",
            DbEntryKey::SuMultiuserMode => "multiuser_mode",
            DbEntryKey::SuMntNs => "mnt_ns",
            DbEntryKey::DenylistConfig => "denylist",
            DbEntryKey::ZygiskConfig => "zygisk",
            DbEntryKey::BootloopCount => "bootloop",
            DbEntryKey::SuManager => "requester",
            _ => "",
        }
    }
}

impl SqlTable for DbSettings {
    fn on_row(&mut self, columns: &[String], values: &DbValues) {
        let mut key = "";
        let mut value = 0;
        for (i, column) in columns.iter().enumerate() {
            if column == "key" {
                key = values.get_text(i as i32);
            } else if column == "value" {
                value = values.get_int(i as i32);
            }
        }
        match key {
            "root_access" => self.root_access = RootAccess::from_i32(value).unwrap_or_default(),
            "multiuser_mode" => {
                self.multiuser_mode = MultiuserMode::from_i32(value).unwrap_or_default()
            }
            "mnt_ns" => self.mnt_ns = MntNsMode { repr: value },
            "denylist" => self.denylist = value != 0,
            "zygisk" => self.zygisk = value != 0,
            "bootloop" => self.boot_count = value,
            _ => {}
        }
    }
}

#[repr(transparent)]
pub struct Sqlite3(NonNull<sqlite3>);
unsafe impl Send for Sqlite3 {}

type SqlBindCallback = Option<unsafe extern "C" fn(*mut c_void, i32, Pin<&mut DbStatement>) -> i32>;
type SqlExecCallback = Option<unsafe extern "C" fn(*mut c_void, &[String], &DbValues)>;

unsafe extern "C" {
    fn sql_exec_impl(
        db: *mut sqlite3,
        sql: &str,
        bind_callback: SqlBindCallback,
        bind_cookie: *mut c_void,
        exec_callback: SqlExecCallback,
        exec_cookie: *mut c_void,
    ) -> i32;
}

pub enum DbArg<'a> {
    Text(&'a str),
    Integer(i64),
}

struct DbArgs<'a> {
    args: &'a [DbArg<'a>],
    curr: usize,
}

unsafe extern "C" fn bind_arguments(v: *mut c_void, idx: i32, stmt: Pin<&mut DbStatement>) -> i32 {
    unsafe {
        let args = &mut *(v as *mut DbArgs<'_>);
        if args.curr < args.args.len() {
            let arg = &args.args[args.curr];
            args.curr += 1;
            match *arg {
                Text(v) => stmt.bind_text(idx, v),
                Integer(v) => stmt.bind_int64(idx, v),
            }
        } else {
            0
        }
    }
}

unsafe extern "C" fn read_db_row<T: SqlTable>(
    v: *mut c_void,
    columns: &[String],
    values: &DbValues,
) {
    unsafe {
        let table = &mut *(v as *mut T);
        table.on_row(columns, values);
    }
}

impl MagiskD {
    fn with_db<F: FnOnce(*mut sqlite3) -> i32>(&self, f: F) -> i32 {
        let mut db = self.sql_connection.lock().unwrap();
        if db.is_none() {
            let raw_db = open_and_init_db();
            *db = NonNull::new(raw_db).map(Sqlite3);
        }
        match *db {
            Some(ref mut db) => f(db.0.as_ptr()),
            _ => -1,
        }
    }

    fn db_exec_impl(
        &self,
        sql: &str,
        args: &[DbArg],
        exec_callback: SqlExecCallback,
        exec_cookie: *mut c_void,
    ) -> i32 {
        let mut bind_callback: SqlBindCallback = None;
        let mut bind_cookie: *mut c_void = ptr::null_mut();
        let mut db_args = DbArgs { args, curr: 0 };
        if !args.is_empty() {
            bind_callback = Some(bind_arguments);
            bind_cookie = (&mut db_args) as *mut DbArgs as *mut c_void;
        }
        self.with_db(|db| unsafe {
            sql_exec_impl(
                db,
                sql,
                bind_callback,
                bind_cookie,
                exec_callback,
                exec_cookie,
            )
        })
    }

    pub fn db_exec_with_rows<T: SqlTable>(&self, sql: &str, args: &[DbArg], out: &mut T) -> i32 {
        self.db_exec_impl(
            sql,
            args,
            Some(read_db_row::<T>),
            out as *mut T as *mut c_void,
        )
    }

    pub fn db_exec(&self, sql: &str, args: &[DbArg]) -> i32 {
        self.db_exec_impl(sql, args, None, ptr::null_mut())
    }

    pub fn set_db_setting(&self, key: DbEntryKey, value: i32) -> SqliteResult<()> {
        self.db_exec(
            "INSERT OR REPLACE INTO settings (key,value) VALUES(?,?)",
            &[Text(key.to_str()), Integer(value as i64)],
        )
        .sql_result()
    }

    pub fn get_db_setting(&self, key: DbEntryKey) -> i32 {
        // Get default values
        let mut val = match key {
            DbEntryKey::RootAccess => RootAccess::default() as i32,
            DbEntryKey::SuMultiuserMode => MultiuserMode::default() as i32,
            DbEntryKey::SuMntNs => MntNsMode::default().repr,
            DbEntryKey::DenylistConfig => 0,
            DbEntryKey::ZygiskConfig => self.is_emulator as i32,
            DbEntryKey::BootloopCount => 0,
            _ => -1,
        };
        let mut func = |_: &[String], values: &DbValues| {
            val = values.get_int(0);
        };
        self.db_exec_with_rows(
            "SELECT value FROM settings WHERE key=?",
            &[Text(key.to_str())],
            &mut func,
        )
        .sql_result()
        .log()
        .ok();
        val
    }

    pub fn get_db_settings(&self) -> SqliteResult<DbSettings> {
        let mut cfg = DbSettings {
            zygisk: self.is_emulator,
            ..Default::default()
        };
        self.db_exec_with_rows("SELECT * FROM settings", &[], &mut cfg)
            .sql_result()?;
        Ok(cfg)
    }

    pub fn get_db_string(&self, key: DbEntryKey) -> String {
        let mut val = "".to_string();
        let mut func = |_: &[String], values: &DbValues| {
            val.push_str(values.get_text(0));
        };
        self.db_exec_with_rows(
            "SELECT value FROM strings WHERE key=?",
            &[Text(key.to_str())],
            &mut func,
        )
        .sql_result()
        .log()
        .ok();
        val
    }

    pub fn rm_db_string(&self, key: DbEntryKey) -> SqliteResult<()> {
        self.db_exec("DELETE FROM strings WHERE key=?", &[Text(key.to_str())])
            .sql_result()
    }

    pub fn db_exec_for_cli(&self, mut file: UnixStream) -> LoggedResult<()> {
        let mut reader = BufReader::new(&mut file);
        let sql: String = reader.read_decodable()?;
        let mut writer = BufWriter::new(&mut file);
        let mut output_fn = |columns: &[String], values: &DbValues| {
            let mut out = "".to_string();
            for (i, column) in columns.iter().enumerate() {
                if i != 0 {
                    out.push('|');
                }
                out.push_str(column);
                out.push('=');
                out.push_str(values.get_text(i as i32));
            }
            writer.write_encodable(&out).log_ok();
        };
        self.db_exec_with_rows(&sql, &[], &mut output_fn);
        writer.write_encodable("").log()
    }
}

impl MagiskD {
    pub fn set_db_setting_for_cxx(&self, key: DbEntryKey, value: i32) -> bool {
        self.set_db_setting(key, value).log().is_ok()
    }
}

#[unsafe(export_name = "sql_exec_rs")]
unsafe extern "C" fn sql_exec_for_cxx(
    sql: &str,
    bind_callback: SqlBindCallback,
    bind_cookie: *mut c_void,
    exec_callback: SqlExecCallback,
    exec_cookie: *mut c_void,
) -> i32 {
    unsafe {
        MAGISKD.get().unwrap_unchecked().with_db(|db| {
            sql_exec_impl(
                db,
                sql,
                bind_callback,
                bind_cookie,
                exec_callback,
                exec_cookie,
            )
        })
    }
}
