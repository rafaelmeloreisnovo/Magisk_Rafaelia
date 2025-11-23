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

#define INTLROOT      ".magisk"
#define MIRRDIR       INTLROOT "/mirror"
#define PREINITMIRR   INTLROOT "/preinit"
#define DEVICEDIR     INTLROOT "/device"
#define PREINITDEV    DEVICEDIR "/preinit"
#define WORKERDIR     INTLROOT "/worker"
#define BBPATH        INTLROOT "/busybox"
#define ROOTOVL       INTLROOT "/rootdir"
#define SHELLPTS      INTLROOT "/pts"
#define MAIN_CONFIG   INTLROOT "/config"
#define MAIN_SOCKET   DEVICEDIR "/socket"

constexpr const char *applet_names[] = { "su", "resetprop", nullptr };

#define POST_FS_DATA_WAIT_TIME       40
#define POST_FS_DATA_SCRIPT_MAX_TIME 35

// Unconstrained domain the daemon and root processes run in
#define SEPOL_PROC_DOMAIN   "magisk"
#define MAGISK_PROC_CON     "u:r:" SEPOL_PROC_DOMAIN ":s0"
// Unconstrained file type that anyone can access
#define SEPOL_FILE_TYPE     "magisk_file"
#define MAGISK_FILE_CON     "u:object_r:" SEPOL_FILE_TYPE ":s0"
