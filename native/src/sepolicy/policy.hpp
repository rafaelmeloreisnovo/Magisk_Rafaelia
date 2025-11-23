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

#include <map>
#include <string_view>
#include <rust/cxx.h>

#include <sepol/policydb/policydb.h>

using Str = rust::Str;

struct Xperm;

class sepol_impl {
    avtab_ptr_t find_avtab_node(avtab_key_t *key, avtab_extended_perms_t *xperms);
    avtab_ptr_t insert_avtab_node(avtab_key_t *key);
    avtab_ptr_t get_avtab_node(avtab_key_t *key, avtab_extended_perms_t *xperms);
    void print_type(FILE *fp, type_datum_t *type);
    void print_avtab(FILE *fp, avtab_ptr_t node);
    void print_filename_trans(FILE *fp, hashtab_ptr_t node);

    bool add_rule(Str s, Str t, Str c, Str p, int effect, bool invert);
    void add_rule(type_datum_t *src, type_datum_t *tgt, class_datum_t *cls, perm_datum_t *perm, int effect, bool invert);
    void add_xperm_rule(type_datum_t *src, type_datum_t *tgt, class_datum_t *cls, const Xperm &p, int effect);
    bool add_xperm_rule(Str s, Str t, Str c, const Xperm &p, int effect);
    bool add_type_rule(Str s, Str t, Str c, Str d, int effect);
    bool add_filename_trans(Str s, Str t, Str c, Str d, Str o);
    bool add_genfscon(Str fs_name, Str path, Str context);
    bool add_type(Str type_name, uint32_t flavor);
    bool set_type_state(Str type_name, bool permissive);
    void add_typeattribute(type_datum_t *type, type_datum_t *attr);
    bool add_typeattribute(Str type, Str attr);

    policydb *db;

    std::map<std::string_view, std::array<const char *, 32>> class_perm_names;

    friend struct SePolicy;

public:
    sepol_impl(policydb *db) : db(db) {}
    ~sepol_impl();
};
