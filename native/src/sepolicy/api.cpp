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

#include <base.hpp>

#include "include/sepolicy.hpp"

using Str = rust::Str;
using StrVec = rust::Vec<rust::Str>;
using Xperms = rust::Vec<Xperm>;

#if 0
template<typename Arg>
std::string as_str(const Arg &arg) {
    if constexpr (std::is_same_v<Arg, Xperm>) {
        return (std::string) SePolicy::xperm_to_string(arg);
    } else if constexpr (std::is_same_v<Arg, rust::Str>) {
        return arg.empty() ? "*" : (std::string) arg;
    }
}

// Print out all rules going through public API for debugging
template<typename ...Args>
static void print_rule(const char *action, Args ...args) {
    std::string s;
    s = (... + (" " + as_str(args)));
    LOGD("%s%s\n", action, s.data());
}
#else
#define print_rule(...) ((void) 0)
#endif

template<typename F, typename ...T>
requires(std::invocable<F, T...>)
static inline void expand(F &&f, T &&...args) {
    f(std::forward<T>(args)...);
}

template<typename ...T>
static inline void expand(Str s, T &&...args) {
    expand(std::forward<T>(args)..., s);
}

template<typename ...T>
static inline void expand(const StrVec &vec, T &&...args) {
    if (vec.empty()) {
        expand(std::forward<T>(args)..., rust::Str{});
    } else {
        for (auto s : vec) {
            expand(std::forward<T>(args)..., s);
        }
    }
}

template<typename ...T>
static inline void expand(const Xperms &vec, T &&...args) {
    for (auto &p : vec) {
        expand(std::forward<T>(args)..., p);
    }
}

void SePolicy::allow(StrVec src, StrVec tgt, StrVec cls, StrVec perm) noexcept {
    expand(src, tgt, cls, perm, [this](auto ...args) {
        print_rule("allow", args...);
        impl->add_rule(args..., AVTAB_ALLOWED, false);
    });
}

void SePolicy::deny(StrVec src, StrVec tgt, StrVec cls, StrVec perm) noexcept {
    expand(src, tgt, cls, perm, [this](auto ...args) {
        print_rule("deny", args...);
        impl->add_rule(args..., AVTAB_ALLOWED, true);
    });
}

void SePolicy::auditallow(StrVec src, StrVec tgt, StrVec cls, StrVec perm) noexcept {
    expand(src, tgt, cls, perm, [this](auto ...args) {
        print_rule("auditallow", args...);
        impl->add_rule(args..., AVTAB_AUDITALLOW, false);
    });
}

void SePolicy::dontaudit(StrVec src, StrVec tgt, StrVec cls, StrVec perm) noexcept {
    expand(src, tgt, cls, perm, [this](auto ...args) {
        print_rule("dontaudit", args...);
        impl->add_rule(args..., AVTAB_AUDITDENY, true);
    });
}

void SePolicy::permissive(StrVec types) noexcept {
    expand(types, [this](auto ...args) {
        print_rule("permissive", args...);
        impl->set_type_state(args..., true);
    });
}

void SePolicy::enforce(StrVec types) noexcept {
    expand(types, [this](auto ...args) {
        print_rule("enforce", args...);
        impl->set_type_state(args..., false);
    });
}

void SePolicy::typeattribute(StrVec types, StrVec attrs) noexcept {
    expand(types, attrs, [this](auto ...args) {
        print_rule("typeattribute", args...);
        impl->add_typeattribute(args...);
    });
}

void SePolicy::type(Str type, StrVec attrs) noexcept {
    expand(type, attrs, [this](auto name, auto attr) {
        print_rule("type", name, attr);
        impl->add_type(name, TYPE_TYPE) && impl->add_typeattribute(name, attr);
    });
}

void SePolicy::attribute(Str name) noexcept {
    expand(name, [this](auto ...args) {
        print_rule("attribute", args...);
        impl->add_type(args..., TYPE_ATTRIB);
    });
}

void SePolicy::type_transition(Str src, Str tgt, Str cls, Str def, Str obj) noexcept {
    expand(src, tgt, cls, def, obj, [this](auto s, auto t, auto c, auto d, auto o) {
        if (!o.empty()) {
            print_rule("type_transition", s, t, c, d, o);
            impl->add_filename_trans(s, t, c, d, o);
        } else {
            print_rule("type_transition", s, t, c, d);
            impl->add_type_rule(s, t, c, d, AVTAB_TRANSITION);
        }
    });
}

void SePolicy::type_change(Str src, Str tgt, Str cls, Str def) noexcept {
    expand(src, tgt, cls, def, [this](auto ...args) {
        print_rule("type_change", args...);
        impl->add_type_rule(args..., AVTAB_CHANGE);
    });
}

void SePolicy::type_member(Str src, Str tgt, Str cls, Str def) noexcept {
    expand(src, tgt, cls, def, [this](auto ...args) {
        print_rule("type_member", args...);
        impl->add_type_rule(args..., AVTAB_MEMBER);
    });
}

void SePolicy::genfscon(Str fs_name, Str path, Str ctx) noexcept {
    expand(fs_name, path, ctx, [this](auto ...args) {
        print_rule("genfscon", args...);
        impl->add_genfscon(args...);
    });
}

void SePolicy::allowxperm(StrVec src, StrVec tgt, StrVec cls, Xperms xperm) noexcept {
    expand(src, tgt, cls, xperm, [this](auto ...args) {
        print_rule("allowxperm", args...);
        impl->add_xperm_rule(args..., AVTAB_XPERMS_ALLOWED);
    });
}

void SePolicy::auditallowxperm(StrVec src, StrVec tgt, StrVec cls, Xperms xperm) noexcept {
    expand(src, tgt, cls, xperm, [this](auto ...args) {
        print_rule("auditallowxperm", args...);
        impl->add_xperm_rule(args..., AVTAB_XPERMS_AUDITALLOW);
    });
}

void SePolicy::dontauditxperm(StrVec src, StrVec tgt, StrVec cls, Xperms xperm) noexcept {
    expand(src, tgt, cls, xperm, [this](auto ...args) {
        print_rule("dontauditxperm", args...);
        impl->add_xperm_rule(args..., AVTAB_XPERMS_DONTAUDIT);
    });
}
