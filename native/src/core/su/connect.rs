//! connect.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 16970CAC39FD4659
⚓ FILE_PATH: native/src/core/su/connect.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: E029A22892946339E51AF513C256D308


*/


use super::SuInfo;
use super::db::RootSettings;
use crate::consts::{INTERNAL_DIR, MAGISK_FILE_CON};
use crate::daemon::to_user_id;
use crate::ffi::{SuPolicy, SuRequest, get_magisk_tmp};
use crate::socket::IpcRead;
use ExtraVal::{Bool, Int, IntList, Str};
use base::{
    BytesExt, FileAttr, LibcReturn, LoggedResult, ResultExt, Utf8CStrBuf, cstr, fork_dont_care,
};
use nix::fcntl::OFlag;
use nix::poll::{PollFd, PollFlags, PollTimeout};
use num_traits::AsPrimitive;
use std::fmt::Write;
use std::fs::File;
use std::os::fd::AsFd;
use std::os::unix::net::UCred;
use std::process::{Command, exit};

struct Extra<'a> {
    key: &'static str,
    value: ExtraVal<'a>,
}

enum ExtraVal<'a> {
    Int(i32),
    Bool(bool),
    Str(&'a str),
    IntList(&'a [u32]),
}

impl Extra<'_> {
    fn add_intent(&self, cmd: &mut Command) {
        match self.value {
            Int(i) => {
                cmd.args(["--ei", self.key, &i.to_string()]);
            }
            Bool(b) => {
                cmd.args(["--ez", self.key, &b.to_string()]);
            }
            Str(s) => {
                cmd.args(["--es", self.key, s]);
            }
            IntList(list) => {
                cmd.args(["--es", self.key]);
                let mut tmp = String::new();
                list.iter().for_each(|i| write!(&mut tmp, "{i},").unwrap());
                tmp.pop();
                cmd.arg(&tmp);
            }
        }
    }

    fn add_bind(&self, cmd: &mut Command) {
        let mut tmp: String;
        match self.value {
            Int(i) => {
                tmp = format!("{}:i:{}", self.key, i);
            }
            Bool(b) => {
                tmp = format!("{}:b:{}", self.key, b);
            }
            Str(s) => {
                let s = s.replace("\\", "\\\\").replace(":", "\\:");
                tmp = format!("{}:s:{}", self.key, s);
            }
            IntList(list) => {
                tmp = format!("{}:s:", self.key);
                if !list.is_empty() {
                    list.iter().for_each(|i| write!(&mut tmp, "{i},").unwrap());
                    tmp.pop();
                }
            }
        }
        cmd.args(["--extra", &tmp]);
    }

    fn add_bind_legacy(&self, cmd: &mut Command) {
        match self.value {
            Str(s) => {
                let tmp = format!("{}:s:{}", self.key, s);
                cmd.args(["--extra", &tmp]);
            }
            _ => self.add_bind(cmd),
        }
    }
}

pub(super) struct SuAppContext<'a> {
    pub(super) cred: UCred,
    pub(super) request: &'a SuRequest,
    pub(super) info: &'a SuInfo,
    pub(super) settings: &'a mut RootSettings,
    pub(super) sdk_int: i32,
}

impl SuAppContext<'_> {
    fn exec_cmd(&self, action: &'static str, extras: &[Extra], use_provider: bool) {
        let user = to_user_id(self.info.eval_uid);
        let user = user.to_string();

        if use_provider {
            let provider = format!("content://{}.provider", self.info.mgr_pkg);
            let mut cmd = Command::new("/system/bin/app_process");
            cmd.args([
                "/system/bin",
                "com.android.commands.content.Content",
                "call",
                "--uri",
                &provider,
                "--user",
                &user,
                "--method",
                action,
            ]);
            if self.sdk_int >= 30 {
                extras.iter().for_each(|e| e.add_bind(&mut cmd))
            } else {
                extras.iter().for_each(|e| e.add_bind_legacy(&mut cmd))
            }
            cmd.env("CLASSPATH", "/system/framework/content.jar");

            if let Ok(output) = cmd.output()
                && !output.stderr.contains(b"Error")
                && !output.stdout.contains(b"Error")
            {
                // The provider call succeed
                return;
            }
        }

        let mut cmd = Command::new("/system/bin/app_process");
        cmd.args([
            "/system/bin",
            "com.android.commands.am.Am",
            "start",
            "-p",
            &self.info.mgr_pkg,
            "--user",
            &user,
            "-a",
            "android.intent.action.VIEW",
            "-f",
            // FLAG_ACTIVITY_NEW_TASK|FLAG_ACTIVITY_MULTIPLE_TASK|
            // FLAG_ACTIVITY_EXCLUDE_FROM_RECENTS|FLAG_INCLUDE_STOPPED_PACKAGES
            "0x18800020",
            "--es",
            "action",
            action,
        ]);
        extras.iter().for_each(|e| e.add_intent(&mut cmd));
        cmd.env("CLASSPATH", "/system/framework/am.jar");

        // Sometimes `am start` will fail, keep trying until it works
        loop {
            if let Ok(output) = cmd.output()
                && !output.stdout.is_empty()
            {
                break;
            }
        }
    }

    fn app_request(&mut self) {
        let mut fifo = cstr::buf::new::<64>();
        fifo.write_fmt(format_args!(
            "{}/{}/su_request_{}",
            get_magisk_tmp(),
            INTERNAL_DIR,
            self.cred.pid.unwrap_or(-1)
        ))
        .ok();

        let fd: LoggedResult<File> = try {
            let mut attr = FileAttr::new();
            attr.st.st_mode = 0o600;
            attr.st.st_uid = self.info.mgr_uid.as_();
            attr.st.st_gid = self.info.mgr_uid.as_();
            attr.con.push_str(MAGISK_FILE_CON);

            fifo.mkfifo(0o600)?;
            fifo.set_attr(&attr)?;

            let extras = [
                Extra {
                    key: "fifo",
                    value: Str(&fifo),
                },
                Extra {
                    key: "uid",
                    value: Int(self.info.eval_uid),
                },
                Extra {
                    key: "pid",
                    value: Int(self.cred.pid.unwrap_or(-1)),
                },
            ];
            self.exec_cmd("request", &extras, false);

            // Open with O_RDWR to prevent FIFO open block
            let fd = fifo.open(OFlag::O_RDWR | OFlag::O_CLOEXEC)?;
            let mut pfd = [PollFd::new(fd.as_fd(), PollFlags::POLLIN)];

            // Wait for data input for at most 70 seconds
            nix::poll::poll(&mut pfd, PollTimeout::try_from(70 * 1000).unwrap())
                .check_os_err("poll", None, None)?;
            fd
        };

        fifo.remove().log_ok();

        if let Ok(mut fd) = fd {
            self.settings.policy = SuPolicy {
                repr: fd
                    .read_decodable::<i32>()
                    .log()
                    .map(i32::from_be)
                    .unwrap_or(SuPolicy::Deny.repr),
            };
        } else {
            self.settings.policy = SuPolicy::Deny;
        };
    }

    fn app_notify(&self) {
        let extras = [
            Extra {
                key: "from.uid",
                value: Int(self.cred.uid.as_()),
            },
            Extra {
                key: "pid",
                value: Int(self.cred.pid.unwrap_or(-1).as_()),
            },
            Extra {
                key: "policy",
                value: Int(self.settings.policy.repr),
            },
        ];
        self.exec_cmd("notify", &extras, true);
    }

    fn app_log(&self) {
        let command = if self.request.command.is_empty() {
            &self.request.shell
        } else {
            &self.request.command
        };
        let extras = [
            Extra {
                key: "from.uid",
                value: Int(self.cred.uid.as_()),
            },
            Extra {
                key: "to.uid",
                value: Int(self.request.target_uid),
            },
            Extra {
                key: "pid",
                value: Int(self.cred.pid.unwrap_or(-1).as_()),
            },
            Extra {
                key: "policy",
                value: Int(self.settings.policy.repr),
            },
            Extra {
                key: "target",
                value: Int(self.request.target_pid),
            },
            Extra {
                key: "context",
                value: Str(&self.request.context),
            },
            Extra {
                key: "gids",
                value: IntList(&self.request.gids),
            },
            Extra {
                key: "command",
                value: Str(command),
            },
            Extra {
                key: "notify",
                value: Bool(self.settings.notify),
            },
        ];
        self.exec_cmd("log", &extras, true);
    }

    pub(super) fn connect_app(&mut self) {
        // If policy is undetermined, show dialog for user consent
        if self.settings.policy == SuPolicy::Query {
            self.app_request();
        }

        if !self.settings.log && !self.settings.notify {
            return;
        }

        if fork_dont_care() != 0 {
            return;
        }

        // Notify su usage to application
        if self.settings.log {
            self.app_log();
        } else if self.settings.notify {
            self.app_notify();
        }

        exit(0);
    }
}
