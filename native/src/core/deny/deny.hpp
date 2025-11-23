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

#include <string_view>

#define ISOLATED_MAGIC "isolated"

namespace DenyRequest {
enum : int {
    ENFORCE,
    DISABLE,
    ADD,
    REMOVE,
    LIST,
    STATUS,

    END
};
}

namespace DenyResponse {
enum : int {
    OK,
    ENFORCED,
    NOT_ENFORCED,
    ITEM_EXIST,
    ITEM_NOT_EXIST,
    INVALID_PKG,
    NO_NS,
    ERROR,

    END
};
}

// CLI entries
int enable_deny();
int disable_deny();
int add_list(int client);
int rm_list(int client);
void ls_list(int client);

bool proc_context_match(int pid, std::string_view context);
void *logcat(void *arg);
extern bool logcat_exit;
