/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use base::nix::fcntl::OFlag;
use base::{LogLevel, SilentLogExt, Utf8CStr, cstr, libc, raw_cstr, update_logger};
use libc::{
    O_CLOEXEC, S_IFCHR, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, SYS_dup3, makedev, mknod,
    syscall,
};
use std::fs::File;
use std::io::{IoSlice, Write};
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

// SAFETY: magiskinit is single threaded
static mut KMSG: RawFd = -1;

pub fn setup_klog() {
    unsafe {
        // Shut down first 3 fds
        let mut fd = cstr!("/dev/null")
            .open(OFlag::O_RDWR | OFlag::O_CLOEXEC)
            .silent();
        if fd.is_err() {
            mknod(raw_cstr!("/null"), S_IFCHR | 0o666, makedev(1, 3));
            fd = cstr!("/null")
                .open(OFlag::O_RDWR | OFlag::O_CLOEXEC)
                .silent();
            cstr!("/null").remove().ok();
        }
        if let Ok(ref fd) = fd {
            syscall(SYS_dup3, fd, STDIN_FILENO, O_CLOEXEC);
            syscall(SYS_dup3, fd, STDOUT_FILENO, O_CLOEXEC);
            syscall(SYS_dup3, fd, STDERR_FILENO, O_CLOEXEC);
        }

        // Then open kmsg fd
        let mut fd = cstr!("/dev/kmsg")
            .open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)
            .silent();
        if fd.is_err() {
            mknod(raw_cstr!("/kmsg"), S_IFCHR | 0o666, makedev(1, 11));
            fd = cstr!("/kmsg")
                .open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)
                .silent();
            cstr!("/kmsg").remove().ok();
        }
        KMSG = fd.map(|fd| fd.into_raw_fd()).unwrap_or(-1);
    }

    // Disable kmsg rate limiting
    if let Ok(mut rate) =
        cstr!("/proc/sys/kernel/printk_devkmsg").open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)
    {
        writeln!(rate, "on").ok();
    }

    fn kmsg_log_write(_: LogLevel, msg: &Utf8CStr) {
        let fd = unsafe { KMSG };
        if fd >= 0 {
            let io1 = IoSlice::new("magiskinit: ".as_bytes());
            let io2 = IoSlice::new(msg.as_bytes());
            let mut kmsg = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) });
            let _ = kmsg.write_vectored(&[io1, io2]).ok();
        }
    }

    update_logger(|logger| logger.write = kmsg_log_write);
}
