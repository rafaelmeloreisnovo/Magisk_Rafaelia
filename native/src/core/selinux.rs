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

use crate::consts::{DATABIN, LOG_PIPE, MAGISK_LOG_CON, MAGISKDB, MODULEROOT, SECURE_DIR};
use crate::ffi::get_magisk_tmp;
use base::{Directory, FsPathBuilder, LoggedResult, ResultExt, Utf8CStr, Utf8CStrBuf, cstr, libc};
use nix::fcntl::OFlag;
use std::io::Write;

const UNLABEL_CON: &Utf8CStr = cstr!("u:object_r:unlabeled:s0");
const SYSTEM_CON: &Utf8CStr = cstr!("u:object_r:system_file:s0");
const ADB_CON: &Utf8CStr = cstr!("u:object_r:adb_data_file:s0");
const ROOT_CON: &Utf8CStr = cstr!("u:object_r:rootfs:s0");

fn restore_syscon_from_unlabeled(
    path: &mut dyn Utf8CStrBuf,
    con: &mut dyn Utf8CStrBuf,
) -> LoggedResult<()> {
    let dir_path_len = path.len();
    if path.get_secontext(con).log().is_ok() && con.as_str() == UNLABEL_CON {
        path.set_secontext(SYSTEM_CON)?;
    }
    let mut dir = Directory::open(path)?;
    while let Some(ref e) = dir.read()? {
        path.truncate(dir_path_len);
        path.append_path(e.name());
        if e.is_dir() {
            restore_syscon_from_unlabeled(path, con)?;
        } else if (e.is_file() || e.is_symlink())
            && path.get_secontext(con).log().is_ok()
            && con.as_str() == UNLABEL_CON
        {
            path.set_secontext(SYSTEM_CON)?;
        }
    }
    Ok(())
}

fn restore_syscon(path: &mut dyn Utf8CStrBuf) -> LoggedResult<()> {
    let dir_path_len = path.len();
    path.set_secontext(SYSTEM_CON)?;
    unsafe { libc::lchown(path.as_ptr(), 0, 0) };
    let mut dir = Directory::open(path)?;
    while let Some(ref e) = dir.read()? {
        path.truncate(dir_path_len);
        path.append_path(e.name());
        if e.is_dir() {
            restore_syscon(path)?;
        } else if e.is_file() || e.is_symlink() {
            path.set_secontext(SYSTEM_CON)?;
            unsafe { libc::lchown(path.as_ptr(), 0, 0) };
        }
    }
    Ok(())
}

pub(crate) fn restorecon() {
    if let Ok(mut file) = cstr!("/sys/fs/selinux/context")
        .open(OFlag::O_WRONLY | OFlag::O_CLOEXEC)
        .log()
        && file.write_all(ADB_CON.as_bytes_with_nul()).is_ok()
    {
        cstr!(SECURE_DIR).set_secontext(ADB_CON).log_ok();
    }

    let mut path = cstr::buf::default();
    let mut con = cstr::buf::new::<1024>();
    path.push_str(MODULEROOT);
    path.set_secontext(SYSTEM_CON).log_ok();
    restore_syscon_from_unlabeled(&mut path, &mut con).log_ok();

    path.clear();
    path.push_str(DATABIN);
    restore_syscon(&mut path).log_ok();
    unsafe { libc::chmod(cstr!(MAGISKDB).as_ptr(), 0o000) };
}

pub(crate) fn restore_tmpcon() -> LoggedResult<()> {
    let tmp = get_magisk_tmp();
    if tmp == "/sbin" {
        tmp.set_secontext(ROOT_CON)?;
    } else {
        unsafe { libc::chmod(tmp.as_ptr(), 0o711) };
    }

    let mut path = cstr::buf::default();
    let mut dir = Directory::open(tmp)?;
    while let Some(ref e) = dir.read()? {
        if !e.is_symlink() {
            e.resolve_path(&mut path)?;
            path.set_secontext(SYSTEM_CON).log_ok();
        }
    }

    path.clear();
    path.append_path(tmp).append_path(LOG_PIPE);
    path.set_secontext(cstr!(MAGISK_LOG_CON))?;

    Ok(())
}

pub(crate) fn lgetfilecon(path: &Utf8CStr, con: &mut [u8]) -> bool {
    let mut con = cstr::buf::wrap(con);
    path.get_secontext(&mut con).is_ok()
}

pub(crate) fn setfilecon(path: &Utf8CStr, con: &Utf8CStr) -> bool {
    path.follow_link().set_secontext(con).is_ok()
}
