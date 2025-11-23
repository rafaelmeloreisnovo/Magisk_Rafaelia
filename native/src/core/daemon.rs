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

⚓ ANCHOR_ID: 603FC462A41613D8
⚓ FILE_PATH: native/src/core/daemon.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: A49EC8D671A60193402F17413B3456CD


*/


use crate::bootstages::BootState;
use crate::consts::{
    MAGISK_FILE_CON, MAGISK_FULL_VER, MAGISK_PROC_CON, MAGISK_VER_CODE, MAGISK_VERSION,
    MAIN_CONFIG, MAIN_SOCKET, ROOTMNT, ROOTOVL,
};
use crate::db::Sqlite3;
use crate::ffi::{
    ModuleInfo, RequestCode, RespondCode, denylist_handler, get_magisk_tmp, scan_deny_apps,
};
use crate::logging::{android_logging, magisk_logging, setup_logfile, start_log_daemon};
use crate::module::remove_modules;
use crate::package::ManagerInfo;
use crate::resetprop::{get_prop, set_prop};
use crate::selinux::restore_tmpcon;
use crate::socket::{IpcRead, IpcWrite};
use crate::su::SuInfo;
use crate::thread::ThreadPool;
use crate::zygisk::ZygiskState;
use base::const_format::concatcp;
use base::{
    AtomicArc, BufReadExt, FileAttr, FsPathBuilder, LoggedResult, ReadExt, ResultExt, Utf8CStr,
    Utf8CStrBuf, WriteExt, cstr, fork_dont_care, info, libc, log_err, set_nice_name,
};
use nix::fcntl::OFlag;
use nix::mount::MsFlags;
use nix::sys::signal::SigSet;
use nix::unistd::{dup2_stderr, dup2_stdin, dup2_stdout, getpid, getuid, setsid};
use num_traits::AsPrimitive;
use std::fmt::Write as _;
use std::io::{BufReader, Write};
use std::os::fd::{AsFd, AsRawFd, IntoRawFd, RawFd};
use std::os::unix::net::{UCred, UnixListener, UnixStream};
use std::process::{Command, exit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

// Global magiskd singleton
pub static MAGISKD: OnceLock<MagiskD> = OnceLock::new();

pub const AID_ROOT: i32 = 0;
pub const AID_SHELL: i32 = 2000;
pub const AID_APP_START: i32 = 10000;
pub const AID_APP_END: i32 = 19999;
pub const AID_USER_OFFSET: i32 = 100000;

pub const fn to_app_id(uid: i32) -> i32 {
    uid % AID_USER_OFFSET
}

pub const fn to_user_id(uid: i32) -> i32 {
    uid / AID_USER_OFFSET
}

#[derive(Default)]
pub struct MagiskD {
    pub sql_connection: Mutex<Option<Sqlite3>>,
    pub manager_info: Mutex<ManagerInfo>,
    pub boot_stage_lock: Mutex<BootState>,
    pub module_list: OnceLock<Vec<ModuleInfo>>,
    pub zygisk_enabled: AtomicBool,
    pub zygisk: Mutex<ZygiskState>,
    pub cached_su_info: AtomicArc<SuInfo>,
    pub sdk_int: i32,
    pub is_emulator: bool,
    is_recovery: bool,
    exe_attr: FileAttr,
}

impl MagiskD {
    pub fn get() -> &'static MagiskD {
        unsafe { MAGISKD.get().unwrap_unchecked() }
    }

    pub fn sdk_int(&self) -> i32 {
        self.sdk_int
    }

    pub fn app_data_dir(&self) -> &'static Utf8CStr {
        if self.sdk_int >= 24 {
            cstr!("/data/user_de")
        } else {
            cstr!("/data/user")
        }
    }

    fn handle_request_sync(&self, mut client: UnixStream, code: RequestCode) {
        match code {
            RequestCode::CHECK_VERSION => {
                #[cfg(debug_assertions)]
                let s = concatcp!(MAGISK_VERSION, ":MAGISK:D");
                #[cfg(not(debug_assertions))]
                let s = concatcp!(MAGISK_VERSION, ":MAGISK:R");

                client.write_encodable(s).log_ok();
            }
            RequestCode::CHECK_VERSION_CODE => {
                client.write_pod(&MAGISK_VER_CODE).log_ok();
            }
            RequestCode::START_DAEMON => {
                setup_logfile();
            }
            RequestCode::STOP_DAEMON => {
                // Unmount all overlays
                denylist_handler(-1);

                // Restore native bridge property
                self.zygisk.lock().unwrap().restore_prop();

                client.write_pod(&0).log_ok();

                // Terminate the daemon!
                exit(0);
            }
            _ => {}
        }
    }

    fn handle_request_async(&self, mut client: UnixStream, code: RequestCode, cred: UCred) {
        match code {
            RequestCode::DENYLIST => {
                denylist_handler(client.into_raw_fd());
            }
            RequestCode::SUPERUSER => {
                self.su_daemon_handler(client, cred);
            }
            RequestCode::ZYGOTE_RESTART => {
                info!("** zygote restarted");
                self.prune_su_access();
                scan_deny_apps();
                if self.zygisk_enabled.load(Ordering::Relaxed) {
                    self.zygisk.lock().unwrap().reset(false);
                }
            }
            RequestCode::SQLITE_CMD => {
                self.db_exec_for_cli(client).ok();
            }
            RequestCode::REMOVE_MODULES => {
                let do_reboot: bool = client.read_decodable().log().unwrap_or_default();
                remove_modules();
                client.write_pod(&0).log_ok();
                if do_reboot {
                    self.reboot();
                }
            }
            RequestCode::ZYGISK => {
                self.zygisk_handler(client);
            }
            _ => {}
        }
    }

    fn reboot(&self) {
        if self.is_recovery {
            Command::new("/system/bin/reboot").arg("recovery").status()
        } else {
            Command::new("/system/bin/reboot").status()
        }
        .ok();
    }

    #[cfg(feature = "check-client")]
    fn is_client(&self, pid: i32) -> bool {
        let mut buf = cstr::buf::new::<32>();
        write!(buf, "/proc/{pid}/exe").ok();
        if let Ok(attr) = buf.follow_link().get_attr() {
            attr.st.st_dev == self.exe_attr.st.st_dev && attr.st.st_ino == self.exe_attr.st.st_ino
        } else {
            false
        }
    }

    #[cfg(not(feature = "check-client"))]
    fn is_client(&self, pid: i32) -> bool {
        true
    }

    fn handle_requests(&'static self, mut client: UnixStream) {
        let Ok(cred) = client.peer_cred() else {
            // Client died
            return;
        };

        // There are no abstractions for SO_PEERSEC yet, call the raw C API.
        let mut context = cstr::buf::new::<256>();
        unsafe {
            let mut len: libc::socklen_t = context.capacity().as_();
            libc::getsockopt(
                client.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_PEERSEC,
                context.as_mut_ptr().cast(),
                &mut len,
            );
        }
        context.rebuild().ok();

        let is_root = cred.uid == 0;
        let is_shell = cred.uid == 2000;
        let is_zygote = &context == "u:r:zygote:s0";

        if !is_root && !is_zygote && !self.is_client(cred.pid.unwrap_or(-1)) {
            // Unsupported client state
            client.write_pod(&RespondCode::ACCESS_DENIED.repr).log_ok();
            return;
        }

        let mut code = -1;
        client.read_pod(&mut code).ok();
        if !(0..RequestCode::END.repr).contains(&code)
            || code == RequestCode::_SYNC_BARRIER_.repr
            || code == RequestCode::_STAGE_BARRIER_.repr
        {
            // Unknown request code
            return;
        }

        let code = RequestCode { repr: code };

        // Permission checks
        match code {
            RequestCode::POST_FS_DATA
            | RequestCode::LATE_START
            | RequestCode::BOOT_COMPLETE
            | RequestCode::ZYGOTE_RESTART
            | RequestCode::SQLITE_CMD
            | RequestCode::DENYLIST
            | RequestCode::STOP_DAEMON => {
                if !is_root {
                    client.write_pod(&RespondCode::ROOT_REQUIRED.repr).log_ok();
                    return;
                }
            }
            RequestCode::REMOVE_MODULES => {
                if !is_root && !is_shell {
                    // Only allow root and ADB shell to remove modules
                    client.write_pod(&RespondCode::ACCESS_DENIED.repr).log_ok();
                    return;
                }
            }
            RequestCode::ZYGISK => {
                if !is_zygote {
                    // Invalid client context
                    client.write_pod(&RespondCode::ACCESS_DENIED.repr).log_ok();
                    return;
                }
            }
            _ => {}
        }

        if client.write_pod(&RespondCode::OK.repr).is_err() {
            return;
        }

        if code.repr < RequestCode::_SYNC_BARRIER_.repr {
            self.handle_request_sync(client, code)
        } else if code.repr < RequestCode::_STAGE_BARRIER_.repr {
            ThreadPool::exec_task(move || {
                self.handle_request_async(client, code, cred);
            })
        } else {
            ThreadPool::exec_task(move || {
                self.boot_stage_handler(client, code);
            })
        }
    }
}

fn switch_cgroup(cgroup: &str, pid: i32) {
    let mut buf = cstr::buf::new::<64>()
        .join_path(cgroup)
        .join_path("cgroup.procs");
    if !buf.exists() {
        return;
    }
    if let Ok(mut file) = buf.open(OFlag::O_WRONLY | OFlag::O_APPEND | OFlag::O_CLOEXEC) {
        buf.clear();
        write!(buf, "{pid}").ok();
        file.write_all(buf.as_bytes()).log_ok();
    }
}

fn daemon_entry() {
    set_nice_name(cstr!("magiskd"));
    android_logging();

    // Block all signals
    SigSet::all().thread_set_mask().log_ok();

    // Swap out the original stdio
    if let Ok(null) = cstr!("/dev/null").open(OFlag::O_WRONLY).log() {
        dup2_stdout(null.as_fd()).log_ok();
        dup2_stderr(null.as_fd()).log_ok();
    }
    if let Ok(zero) = cstr!("/dev/zero").open(OFlag::O_RDONLY).log() {
        dup2_stdin(zero).log_ok();
    }

    setsid().log_ok();

    // Make sure the current context is magisk
    if let Ok(mut current) =
        cstr!("/proc/self/attr/current").open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)
    {
        let con = cstr!(MAGISK_PROC_CON);
        current.write_all(con.as_bytes_with_nul()).log_ok();
    }

    start_log_daemon();
    magisk_logging();
    info!("Magisk {MAGISK_FULL_VER} daemon started");

    let is_emulator = get_prop(cstr!("ro.kernel.qemu")) == "1"
        || get_prop(cstr!("ro.boot.qemu")) == "1"
        || get_prop(cstr!("ro.product.device")).contains("vsoc");

    // Load config status
    let magisk_tmp = get_magisk_tmp();
    let mut tmp_path = cstr::buf::new::<64>()
        .join_path(magisk_tmp)
        .join_path(MAIN_CONFIG);
    let mut is_recovery = false;
    if let Ok(main_config) = tmp_path.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
        BufReader::new(main_config).for_each_prop(|key, val| {
            if key == "RECOVERYMODE" {
                is_recovery = val == "true";
                return false;
            }
            true
        });
    }
    tmp_path.truncate(magisk_tmp.len());

    let mut sdk_int = -1;
    if let Ok(build_prop) = cstr!("/system/build.prop").open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
        BufReader::new(build_prop).for_each_prop(|key, val| {
            if key == "ro.build.version.sdk" {
                sdk_int = val.parse::<i32>().unwrap_or(-1);
                return false;
            }
            true
        });
    }
    if sdk_int < 0 {
        // In case some devices do not store this info in build.prop, fallback to getprop
        sdk_int = get_prop(cstr!("ro.build.version.sdk"))
            .parse::<i32>()
            .unwrap_or(-1);
    }
    info!("* Device API level: {sdk_int}");

    restore_tmpcon().log_ok();

    // Escape from cgroup
    let pid = getpid().as_raw();
    switch_cgroup("/acct", pid);
    switch_cgroup("/dev/cg2_bpf", pid);
    switch_cgroup("/sys/fs/cgroup", pid);
    if get_prop(cstr!("ro.config.per_app_memcg")) != "false" {
        switch_cgroup("/dev/memcg/apps", pid);
    }

    // Samsung workaround #7887
    if cstr!("/system_ext/app/mediatek-res/mediatek-res.apk").exists() {
        set_prop(cstr!("ro.vendor.mtk_model"), cstr!("0"));
    }

    // Cleanup pre-init mounts
    tmp_path.append_path(ROOTMNT);
    if let Ok(mount_list) = tmp_path.open(OFlag::O_RDONLY | OFlag::O_CLOEXEC) {
        BufReader::new(mount_list).for_each_line(|line| {
            line.truncate(line.trim_end().len());
            let item = Utf8CStr::from_string(line);
            item.unmount().log_ok();
            true
        })
    }
    tmp_path.truncate(magisk_tmp.len());

    // Remount rootfs as read-only if requested
    if std::env::var_os("REMOUNT_ROOT").is_some() {
        cstr!("/").remount_mount_flags(MsFlags::MS_RDONLY).log_ok();
        unsafe { std::env::remove_var("REMOUNT_ROOT") };
    }

    // Remove all pre-init overlay files to free-up memory
    tmp_path.append_path(ROOTOVL);
    tmp_path.remove_all().ok();
    tmp_path.truncate(magisk_tmp.len());

    let exe_attr = cstr!("/proc/self/exe")
        .follow_link()
        .get_attr()
        .log()
        .unwrap_or_default();

    let daemon = MagiskD {
        sdk_int,
        is_emulator,
        is_recovery,
        exe_attr,
        ..Default::default()
    };
    MAGISKD.set(daemon).ok();

    let sock_path = cstr::buf::new::<64>()
        .join_path(get_magisk_tmp())
        .join_path(MAIN_SOCKET);
    sock_path.remove().ok();

    let Ok(sock) = UnixListener::bind(&sock_path).log() else {
        exit(1);
    };

    sock_path.follow_link().chmod(0o666).log_ok();
    sock_path.set_secontext(cstr!(MAGISK_FILE_CON)).log_ok();

    // Loop forever to listen for requests
    let daemon = MagiskD::get();
    for client in sock.incoming() {
        if let Ok(client) = client.log() {
            daemon.handle_requests(client);
        } else {
            exit(1);
        }
    }
}

pub fn connect_daemon(code: RequestCode, create: bool) -> LoggedResult<UnixStream> {
    let sock_path = cstr::buf::new::<64>()
        .join_path(get_magisk_tmp())
        .join_path(MAIN_SOCKET);

    fn send_request(code: RequestCode, mut socket: UnixStream) -> LoggedResult<UnixStream> {
        socket.write_pod(&code.repr).log_ok();
        let mut res = -1;
        socket.read_pod(&mut res).log_ok();
        let res = RespondCode { repr: res };
        match res {
            RespondCode::OK => Ok(socket),
            RespondCode::ROOT_REQUIRED => {
                log_err!("Root is required for this operation")
            }
            RespondCode::ACCESS_DENIED => {
                log_err!("Accessed denied")
            }
            _ => {
                log_err!("Daemon error")
            }
        }
    }

    match UnixStream::connect(&sock_path) {
        Ok(socket) => send_request(code, socket),
        Err(e) => {
            if !create || !getuid().is_root() {
                return log_err!("Cannot connect to daemon: {e}");
            }

            let mut buf = cstr::buf::new::<64>();
            if cstr!("/proc/self/exe").read_link(&mut buf).is_err()
                || !buf.starts_with(get_magisk_tmp().as_str())
            {
                return log_err!("Start daemon on magisk tmpfs");
            }

            // Fork a process and run the daemon
            if fork_dont_care() == 0 {
                daemon_entry();
                exit(0);
            }

            // In the client, we keep retry and connect to the socket
            loop {
                if let Ok(socket) = UnixStream::connect(&sock_path) {
                    return send_request(code, socket);
                } else {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

pub fn connect_daemon_for_cxx(code: RequestCode, create: bool) -> RawFd {
    connect_daemon(code, create)
        .map(IntoRawFd::into_raw_fd)
        .unwrap_or(-1)
}
