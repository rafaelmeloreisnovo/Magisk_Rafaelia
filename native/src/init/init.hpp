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

#pragma once

#define DEFAULT_DT_DIR "/proc/device-tree/firmware/android"
#define REDIR_PATH "/data/magiskinit"

#define PRELOAD_LIB    "/dev/preload.so"
#define PRELOAD_POLICY "/dev/sepolicy"
#define PRELOAD_ACK    "/dev/ack"

#ifdef __cplusplus

#include <base.hpp>
#include <sepolicy.hpp>

#include "init-rs.hpp"

int magisk_proxy_main(int, char *argv[]);
Utf8CStr backup_init();

// Expose some constants to Rust

static inline Utf8CStr split_plat_cil() {
    return SPLIT_PLAT_CIL;
};

static inline Utf8CStr preload_lib() {
    return PRELOAD_LIB;
}

static inline Utf8CStr preload_policy() {
    return PRELOAD_POLICY;
}

static inline Utf8CStr preload_ack() {
    return PRELOAD_ACK;
}


#endif
