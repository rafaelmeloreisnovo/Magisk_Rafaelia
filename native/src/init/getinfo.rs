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

use crate::ffi::{BootConfig, MagiskInit, backup_init};
use base::{BytesExt, MappedFile, cstr};

impl BootConfig {
    #[allow(unused_imports, unused_unsafe)]
    pub(crate) fn print(&self) {
        use base::{Utf8CStr, debug};
        debug!("skip_initramfs=[{}]", self.skip_initramfs);
        debug!("force_normal_boot=[{}]", self.force_normal_boot);
        debug!("rootwait=[{}]", self.rootwait);
        unsafe {
            debug!(
                "slot=[{}]",
                Utf8CStr::from_ptr_unchecked(self.slot.as_ptr())
            );
            debug!(
                "dt_dir=[{}]",
                Utf8CStr::from_ptr_unchecked(self.dt_dir.as_ptr())
            );
            debug!(
                "fstab_suffix=[{}]",
                Utf8CStr::from_ptr_unchecked(self.fstab_suffix.as_ptr())
            );
            debug!(
                "hardware=[{}]",
                Utf8CStr::from_ptr_unchecked(self.hardware.as_ptr())
            );
            debug!(
                "hardware.platform=[{}]",
                Utf8CStr::from_ptr_unchecked(self.hardware_plat.as_ptr())
            );
        }
        debug!("emulator=[{}]", self.emulator);
        debug!("partition_map=[{:?}]", self.partition_map);
    }
}

impl MagiskInit {
    pub(crate) fn check_two_stage(&self) -> bool {
        cstr!("/first_stage_ramdisk").exists() ||
            cstr!("/second_stage_resources").exists() ||
            cstr!("/system/bin/init").exists() ||
            // Use the apex folder to determine whether 2SI (Android 10+)
            cstr!("/apex").exists() ||
            // If we still have no indication, parse the original init and see what's up
            MappedFile::open(backup_init())
                .map(|data| data.contains(b"selinux_setup"))
                .unwrap_or(false)
    }
}
