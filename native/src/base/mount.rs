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

use crate::{LibcReturn, OsResult, Utf8CStr};
use nix::mount::{MntFlags, MsFlags, mount, umount2};

impl Utf8CStr {
    pub fn bind_mount_to<'a>(&'a self, path: &'a Utf8CStr, rec: bool) -> OsResult<'a, ()> {
        let flag = if rec {
            MsFlags::MS_REC
        } else {
            MsFlags::empty()
        };
        mount(
            Some(self),
            path,
            None::<&Utf8CStr>,
            flag | MsFlags::MS_BIND,
            None::<&Utf8CStr>,
        )
        .check_os_err("bind_mount", Some(self), Some(path))
    }

    pub fn remount_mount_point_flags(&self, flags: MsFlags) -> OsResult<'_, ()> {
        mount(
            None::<&Utf8CStr>,
            self,
            None::<&Utf8CStr>,
            MsFlags::MS_BIND | MsFlags::MS_REMOUNT | flags,
            None::<&Utf8CStr>,
        )
        .check_os_err("remount", Some(self), None)
    }

    pub fn remount_mount_flags(&self, flags: MsFlags) -> OsResult<'_, ()> {
        mount(
            None::<&Utf8CStr>,
            self,
            None::<&Utf8CStr>,
            MsFlags::MS_REMOUNT | flags,
            None::<&Utf8CStr>,
        )
        .check_os_err("remount", Some(self), None)
    }

    pub fn remount_with_data(&self, data: &Utf8CStr) -> OsResult<'_, ()> {
        mount(
            None::<&Utf8CStr>,
            self,
            None::<&Utf8CStr>,
            MsFlags::MS_REMOUNT,
            Some(data),
        )
        .check_os_err("remount", Some(self), None)
    }

    pub fn move_mount_to<'a>(&'a self, path: &'a Utf8CStr) -> OsResult<'a, ()> {
        mount(
            Some(self),
            path,
            None::<&Utf8CStr>,
            MsFlags::MS_MOVE,
            None::<&Utf8CStr>,
        )
        .check_os_err("move_mount", Some(self), Some(path))
    }

    pub fn unmount(&self) -> OsResult<'_, ()> {
        umount2(self, MntFlags::MNT_DETACH).check_os_err("unmount", Some(self), None)
    }

    pub fn set_mount_private(&self, rec: bool) -> OsResult<'_, ()> {
        let flag = if rec {
            MsFlags::MS_REC
        } else {
            MsFlags::empty()
        };
        mount(
            None::<&Utf8CStr>,
            self,
            None::<&Utf8CStr>,
            flag | MsFlags::MS_PRIVATE,
            None::<&Utf8CStr>,
        )
        .check_os_err("set_mount_private", Some(self), None)
    }
}
