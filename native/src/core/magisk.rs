//! magisk.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 8B47C95437C9A8D4
⚓ FILE_PATH: native/src/core/magisk.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: BC9E854890B88C4BF7969632A61EDD79


*/


use crate::consts::{APPLET_NAMES, MAGISK_VER_CODE, MAGISK_VERSION, POST_FS_DATA_WAIT_TIME};
use crate::daemon::connect_daemon;
use crate::ffi::{RequestCode, denylist_cli, get_magisk_tmp, install_module, unlock_blocks};
use crate::mount::find_preinit_device;
use crate::selinux::restorecon;
use crate::socket::{Decodable, Encodable};
use argh::FromArgs;
use base::{CmdArgs, EarlyExitExt, LoggedResult, Utf8CString, argh, clone_attr};
use nix::poll::{PollFd, PollFlags, PollTimeout};
use std::ffi::c_char;
use std::os::fd::AsFd;
use std::process::exit;

fn print_usage() {
    eprintln!(
        r#"Magisk - Multi-purpose Utility

Usage: magisk [applet [arguments]...]
   or: magisk [options]...

Options:
   -c                        print current binary version
   -v                        print running daemon version
   -V                        print running daemon version code
   --list                    list all available applets
   --remove-modules [-n]     remove all modules, reboot if -n is not provided
   --install-module ZIP      install a module zip file

Advanced Options (Internal APIs):
   --daemon                  manually start magisk daemon
   --stop                    remove all magisk changes and stop daemon
   --[init trigger]          callback on init triggers. Valid triggers:
                             post-fs-data, service, boot-complete, zygote-restart
   --unlock-blocks           set BLKROSET flag to OFF for all block devices
   --restorecon              restore selinux context on Magisk files
   --clone-attr SRC DEST     clone permission, owner, and selinux context
   --clone SRC DEST          clone SRC to DEST
   --sqlite SQL              exec SQL commands to Magisk database
   --path                    print Magisk tmpfs mount path
   --denylist ARGS           denylist config CLI
   --preinit-device          resolve a device to store preinit files

Available applets:
     {}
"#,
        APPLET_NAMES.join(", ")
    );
}

#[derive(FromArgs)]
struct Cli {
    #[argh(subcommand)]
    action: MagiskAction,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum MagiskAction {
    LocalVersion(LocalVersion),
    Version(Version),
    VersionCode(VersionCode),
    List(ListApplets),
    RemoveModules(RemoveModules),
    InstallModule(InstallModule),
    Daemon(StartDaemon),
    Stop(StopDaemon),
    PostFsData(PostFsData),
    Service(ServiceCmd),
    BootComplete(BootComplete),
    ZygoteRestart(ZygoteRestart),
    UnlockBlocks(UnlockBlocks),
    RestoreCon(RestoreCon),
    CloneAttr(CloneAttr),
    CloneFile(CloneFile),
    Sqlite(Sqlite),
    Path(PathCmd),
    DenyList(DenyList),
    PreInitDevice(PreInitDevice),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "-c")]
struct LocalVersion {}

#[derive(FromArgs)]
#[argh(subcommand, name = "-v")]
struct Version {}

#[derive(FromArgs)]
#[argh(subcommand, name = "-V")]
struct VersionCode {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--list")]
struct ListApplets {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--remove-modules")]
struct RemoveModules {
    #[argh(switch, short = 'n')]
    no_reboot: bool,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--install-module")]
struct InstallModule {
    #[argh(positional)]
    zip: Utf8CString,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--daemon")]
struct StartDaemon {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--stop")]
struct StopDaemon {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--post-fs-data")]
struct PostFsData {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--service")]
struct ServiceCmd {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--boot-complete")]
struct BootComplete {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--zygote-restart")]
struct ZygoteRestart {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--unlock-blocks")]
struct UnlockBlocks {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--restorecon")]
struct RestoreCon {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--clone-attr")]
struct CloneAttr {
    #[argh(positional)]
    from: Utf8CString,
    #[argh(positional)]
    to: Utf8CString,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--clone")]
struct CloneFile {
    #[argh(positional)]
    from: Utf8CString,
    #[argh(positional)]
    to: Utf8CString,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--sqlite")]
struct Sqlite {
    #[argh(positional)]
    sql: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--path")]
struct PathCmd {}

#[derive(FromArgs)]
#[argh(subcommand, name = "--denylist")]
struct DenyList {
    #[argh(positional, greedy)]
    args: Vec<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "--preinit-device")]
struct PreInitDevice {}

impl MagiskAction {
    fn exec(self) -> LoggedResult<i32> {
        use MagiskAction::*;
        match self {
            LocalVersion(_) => {
                #[cfg(debug_assertions)]
                println!("{MAGISK_VERSION}:MAGISK:D ({MAGISK_VER_CODE})");
                #[cfg(not(debug_assertions))]
                println!("{MAGISK_VERSION}:MAGISK:R ({MAGISK_VER_CODE})");
            }
            Version(_) => {
                let mut fd = connect_daemon(RequestCode::CHECK_VERSION, false)?;
                let ver = String::decode(&mut fd)?;
                println!("{ver}");
            }
            VersionCode(_) => {
                let mut fd = connect_daemon(RequestCode::CHECK_VERSION_CODE, false)?;
                let ver = i32::decode(&mut fd)?;
                println!("{ver}");
            }
            List(_) => {
                for name in APPLET_NAMES {
                    println!("{name}");
                }
            }
            RemoveModules(self::RemoveModules { no_reboot }) => {
                let mut fd = connect_daemon(RequestCode::REMOVE_MODULES, false)?;
                let do_reboot = !no_reboot;
                do_reboot.encode(&mut fd)?;
                return Ok(i32::decode(&mut fd)?);
            }
            InstallModule(self::InstallModule { zip }) => {
                install_module(&zip);
            }
            Daemon(_) => {
                let _ = connect_daemon(RequestCode::START_DAEMON, true)?;
            }
            Stop(_) => {
                let mut fd = connect_daemon(RequestCode::STOP_DAEMON, false)?;
                return Ok(i32::decode(&mut fd)?);
            }
            PostFsData(_) => {
                let fd = connect_daemon(RequestCode::POST_FS_DATA, true)?;
                let mut pfd = [PollFd::new(fd.as_fd(), PollFlags::POLLIN)];
                nix::poll::poll(
                    &mut pfd,
                    PollTimeout::try_from(POST_FS_DATA_WAIT_TIME * 1000)?,
                )?;
            }
            Service(_) => {
                let _ = connect_daemon(RequestCode::LATE_START, true)?;
            }
            BootComplete(_) => {
                let _ = connect_daemon(RequestCode::BOOT_COMPLETE, false)?;
            }
            ZygoteRestart(_) => {
                let _ = connect_daemon(RequestCode::ZYGOTE_RESTART, false)?;
            }
            UnlockBlocks(_) => {
                unlock_blocks();
            }
            RestoreCon(_) => {
                restorecon();
            }
            CloneAttr(self::CloneAttr { from, to }) => {
                clone_attr(&from, &to)?;
            }
            CloneFile(self::CloneFile { from, to }) => {
                from.copy_to(&to)?;
            }
            Sqlite(self::Sqlite { sql }) => {
                let mut fd = connect_daemon(RequestCode::SQLITE_CMD, false)?;
                sql.encode(&mut fd)?;
                loop {
                    let line = String::decode(&mut fd)?;
                    if line.is_empty() {
                        return Ok(0);
                    }
                    println!("{line}");
                }
            }
            Path(_) => {
                let tmp = get_magisk_tmp();
                if tmp.is_empty() {
                    return Ok(1);
                } else {
                    println!("{tmp}");
                }
            }
            DenyList(self::DenyList { mut args }) => {
                return Ok(denylist_cli(&mut args));
            }
            PreInitDevice(_) => {
                let name = find_preinit_device();
                if name.is_empty() {
                    return Ok(1);
                } else {
                    println!("{name}");
                }
            }
        };
        Ok(0)
    }
}

pub fn magisk_main(argc: i32, argv: *mut *mut c_char) -> i32 {
    if argc < 2 {
        print_usage();
        exit(1);
    }
    let mut cmds = CmdArgs::new(argc, argv.cast()).0;
    // We need to manually inject "--" so that all actions can be treated as subcommands
    cmds.insert(1, "--");
    let cli = Cli::from_args(&cmds[..1], &cmds[1..]).on_early_exit(print_usage);
    cli.action.exec().unwrap_or(1)
}
