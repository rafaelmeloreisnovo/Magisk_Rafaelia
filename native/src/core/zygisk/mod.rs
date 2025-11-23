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

mod daemon;

use crate::thread::ThreadPool;
use base::{fd_get_attr, libc};
pub use daemon::{ZygiskState, zygisk_should_load_module};
use std::os::fd::RawFd;

#[unsafe(no_mangle)]
extern "C" fn exec_companion_entry(client: RawFd, companion_handler: extern "C" fn(RawFd)) {
    ThreadPool::exec_task(move || {
        let Ok(s1) = fd_get_attr(client) else {
            return;
        };

        companion_handler(client);

        // Only close client if it is the same file so we don't
        // accidentally close a re-used file descriptor.
        // This check is required because the module companion
        // handler could've closed the file descriptor already.
        if let Ok(s2) = fd_get_attr(client)
            && s1.st.st_dev == s2.st.st_dev
            && s1.st.st_ino == s2.st.st_ino
        {
            unsafe { libc::close(client) };
        }
    });
}
