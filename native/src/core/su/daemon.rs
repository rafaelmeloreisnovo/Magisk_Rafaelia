//! daemon.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: FDED07A8D3F02E44
⚓ FILE_PATH: native/src/core/su/daemon.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: CE9B317264444E46953BE68453AE1FEB


*/


use super::connect::SuAppContext;
use super::db::RootSettings;
use crate::daemon::{AID_ROOT, AID_SHELL, MagiskD, to_app_id, to_user_id};
use crate::db::{DbSettings, MultiuserMode, RootAccess};
use crate::ffi::{SuPolicy, SuRequest, exec_root_shell};
use crate::socket::IpcRead;
use base::{LoggedResult, ResultExt, WriteExt, debug, error, exit_on_error, libc, warn};
use std::os::fd::IntoRawFd;
use std::os::unix::net::{UCred, UnixStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[allow(unused_imports)]
use std::os::fd::AsRawFd;

const DEFAULT_SHELL: &str = "/system/bin/sh";

impl Default for SuRequest {
    fn default() -> Self {
        SuRequest {
            target_uid: AID_ROOT,
            target_pid: -1,
            login: false,
            keep_env: false,
            drop_cap: false,
            shell: DEFAULT_SHELL.to_string(),
            command: "".to_string(),
            context: "".to_string(),
            gids: vec![],
        }
    }
}

pub struct SuInfo {
    pub(super) uid: i32,
    pub(super) eval_uid: i32,
    pub(super) mgr_pkg: String,
    pub(super) mgr_uid: i32,
    cfg: DbSettings,
    access: Mutex<AccessInfo>,
}

struct AccessInfo {
    settings: RootSettings,
    timestamp: Instant,
}

impl Default for SuInfo {
    fn default() -> Self {
        SuInfo {
            uid: -1,
            eval_uid: -1,
            cfg: Default::default(),
            mgr_pkg: Default::default(),
            mgr_uid: -1,
            access: Default::default(),
        }
    }
}

impl Default for AccessInfo {
    fn default() -> Self {
        AccessInfo {
            settings: Default::default(),
            timestamp: Instant::now(),
        }
    }
}

impl SuInfo {
    fn allow(uid: i32) -> SuInfo {
        let access = RootSettings {
            policy: SuPolicy::Allow,
            log: false,
            notify: false,
        };
        SuInfo {
            uid,
            access: Mutex::new(AccessInfo::new(access)),
            ..Default::default()
        }
    }

    fn deny(uid: i32) -> SuInfo {
        let access = RootSettings {
            policy: SuPolicy::Deny,
            log: false,
            notify: false,
        };
        SuInfo {
            uid,
            access: Mutex::new(AccessInfo::new(access)),
            ..Default::default()
        }
    }
}

impl AccessInfo {
    fn new(settings: RootSettings) -> AccessInfo {
        AccessInfo {
            settings,
            timestamp: Instant::now(),
        }
    }

    fn is_fresh(&self) -> bool {
        self.timestamp.elapsed() < Duration::from_secs(3)
    }

    fn refresh(&mut self) {
        self.timestamp = Instant::now();
    }
}

impl MagiskD {
    pub fn su_daemon_handler(&self, mut client: UnixStream, cred: UCred) {
        debug!(
            "su: request from uid=[{}], pid=[{}], client=[{}]",
            cred.uid,
            cred.pid.unwrap_or(-1),
            client.as_raw_fd()
        );

        let mut req = match client.read_decodable::<SuRequest>().log() {
            Ok(req) => req,
            Err(_) => {
                warn!("su: remote process probably died, abort");
                client.write_pod(&SuPolicy::Deny.repr).ok();
                return;
            }
        };

        let info = self.get_su_info(cred.uid as i32);
        {
            let mut access = info.access.lock().unwrap();

            // Talk to su manager
            let mut app = SuAppContext {
                cred,
                request: &req,
                info: &info,
                settings: &mut access.settings,
                sdk_int: self.sdk_int(),
            };
            app.connect_app();

            // Before unlocking, refresh the timestamp
            access.refresh();

            if access.settings.policy == SuPolicy::Restrict {
                req.drop_cap = true;
            }

            if access.settings.policy == SuPolicy::Deny {
                warn!("su: request rejected ({})", info.uid);
                client.write_pod(&SuPolicy::Deny.repr).ok();
                return;
            }
        }

        // At this point, the root access is granted.
        // Fork a child root process and monitor its exit value.
        let child = unsafe { libc::fork() };
        if child == 0 {
            debug!("su: fork handler");

            // Abort upon any error occurred
            exit_on_error(true);

            // ack
            client.write_pod(&0).ok();

            exec_root_shell(
                client.into_raw_fd(),
                cred.pid.unwrap_or(-1),
                &mut req,
                info.cfg.mnt_ns,
            );
            return;
        }
        if child < 0 {
            error!("su: fork failed, abort");
            return;
        }

        // Wait result
        debug!("su: waiting child pid=[{}]", child);
        let mut status = 0;
        let code = unsafe {
            if libc::waitpid(child, &mut status, 0) > 0 {
                libc::WEXITSTATUS(status)
            } else {
                -1
            }
        };
        debug!("su: return code=[{}]", code);
        client.write_pod(&code).ok();
    }

    fn get_su_info(&self, uid: i32) -> Arc<SuInfo> {
        if uid == AID_ROOT {
            return Arc::new(SuInfo::allow(AID_ROOT));
        }

        let cached = self.cached_su_info.load();
        if cached.uid == uid && cached.access.lock().unwrap().is_fresh() {
            return cached;
        }

        let info = self.build_su_info(uid);
        self.cached_su_info.store(info.clone());
        info
    }

    #[cfg(feature = "su-check-db")]
    fn build_su_info(&self, uid: i32) -> Arc<SuInfo> {
        let result: LoggedResult<Arc<SuInfo>> = try {
            let cfg = self.get_db_settings()?;

            // Check multiuser settings
            let eval_uid = match cfg.multiuser_mode {
                MultiuserMode::OwnerOnly => {
                    if to_user_id(uid) != 0 {
                        return Arc::new(SuInfo::deny(uid));
                    }
                    uid
                }
                MultiuserMode::OwnerManaged => to_app_id(uid),
                _ => uid,
            };

            let mut access = RootSettings::default();
            self.get_root_settings(eval_uid, &mut access)?;

            // We need to talk to the manager, get the app info
            let (mgr_uid, mgr_pkg) =
                if access.policy == SuPolicy::Query || access.log || access.notify {
                    self.get_manager(to_user_id(eval_uid), true)
                } else {
                    (-1, String::new())
                };

            // If it's the manager, allow it silently
            if to_app_id(uid) == to_app_id(mgr_uid) {
                return Arc::new(SuInfo::allow(uid));
            }

            // Check su access settings
            match cfg.root_access {
                RootAccess::Disabled => {
                    warn!("Root access is disabled!");
                    return Arc::new(SuInfo::deny(uid));
                }
                RootAccess::AdbOnly => {
                    if uid != AID_SHELL {
                        warn!("Root access limited to ADB only!");
                        return Arc::new(SuInfo::deny(uid));
                    }
                }
                RootAccess::AppsOnly => {
                    if uid == AID_SHELL {
                        warn!("Root access is disabled for ADB!");
                        return Arc::new(SuInfo::deny(uid));
                    }
                }
                _ => {}
            };

            // If still not determined, check if manager exists
            if access.policy == SuPolicy::Query && mgr_uid < 0 {
                return Arc::new(SuInfo::deny(uid));
            }

            // Finally, the SuInfo
            Arc::new(SuInfo {
                uid,
                eval_uid,
                mgr_pkg,
                mgr_uid,
                cfg,
                access: Mutex::new(AccessInfo::new(access)),
            })
        };

        result.unwrap_or(Arc::new(SuInfo::deny(uid)))
    }

    #[cfg(not(feature = "su-check-db"))]
    fn build_su_info(&self, uid: i32) -> Arc<SuInfo> {
        Arc::new(SuInfo::allow(uid))
    }
}
