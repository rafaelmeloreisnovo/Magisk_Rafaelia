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

#![feature(try_blocks)]

pub use base;
use std::fmt::Write;

use crate::ffi::SePolicy;

#[path = "../include/consts.rs"]
mod consts;

#[cfg(not(feature = "no-main"))]
mod cli;
mod rules;
mod statement;

#[cxx::bridge]
pub mod ffi {
    struct Xperm {
        low: u16,
        high: u16,
        reset: bool,
    }

    struct SePolicy {
        #[cxx_name = "impl"]
        _impl: UniquePtr<sepol_impl>,
    }

    unsafe extern "C++" {
        include!("policy.hpp");
        include!("../base/include/base.hpp");

        #[cxx_name = "Utf8CStr"]
        type Utf8CStrRef<'a> = base::Utf8CStrRef<'a>;

        type sepol_impl;

        fn allow(self: &mut SePolicy, s: Vec<&str>, t: Vec<&str>, c: Vec<&str>, p: Vec<&str>);
        fn deny(self: &mut SePolicy, s: Vec<&str>, t: Vec<&str>, c: Vec<&str>, p: Vec<&str>);
        fn auditallow(self: &mut SePolicy, s: Vec<&str>, t: Vec<&str>, c: Vec<&str>, p: Vec<&str>);
        fn dontaudit(self: &mut SePolicy, s: Vec<&str>, t: Vec<&str>, c: Vec<&str>, p: Vec<&str>);
        fn allowxperm(self: &mut SePolicy, s: Vec<&str>, t: Vec<&str>, c: Vec<&str>, p: Vec<Xperm>);
        fn auditallowxperm(
            self: &mut SePolicy,
            s: Vec<&str>,
            t: Vec<&str>,
            c: Vec<&str>,
            p: Vec<Xperm>,
        );
        fn dontauditxperm(
            self: &mut SePolicy,
            s: Vec<&str>,
            t: Vec<&str>,
            c: Vec<&str>,
            p: Vec<Xperm>,
        );
        fn permissive(self: &mut SePolicy, t: Vec<&str>);
        fn enforce(self: &mut SePolicy, t: Vec<&str>);
        fn typeattribute(self: &mut SePolicy, t: Vec<&str>, a: Vec<&str>);
        #[cxx_name = "type"]
        fn type_(self: &mut SePolicy, t: &str, a: Vec<&str>);
        fn attribute(self: &mut SePolicy, t: &str);
        fn type_transition(self: &mut SePolicy, s: &str, t: &str, c: &str, d: &str, o: &str);
        fn type_change(self: &mut SePolicy, s: &str, t: &str, c: &str, d: &str);
        fn type_member(self: &mut SePolicy, s: &str, t: &str, c: &str, d: &str);
        fn genfscon(self: &mut SePolicy, s: &str, t: &str, c: &str);
        #[allow(dead_code)]
        fn strip_dontaudit(self: &mut SePolicy);

        fn print_rules(self: &SePolicy);
        fn to_file(self: &SePolicy, file: Utf8CStrRef) -> bool;

        #[Self = SePolicy]
        fn from_file(file: Utf8CStrRef) -> SePolicy;
        #[Self = SePolicy]
        fn from_split() -> SePolicy;
        #[Self = SePolicy]
        fn compile_split() -> SePolicy;
        #[Self = SePolicy]
        fn from_data(data: &[u8]) -> SePolicy;
    }

    extern "Rust" {
        #[Self = SePolicy]
        fn xperm_to_string(perm: &Xperm) -> String;
    }
}

impl SePolicy {
    fn xperm_to_string(perm: &ffi::Xperm) -> String {
        let mut s = String::new();
        if perm.reset {
            s.push('~');
        }
        if perm.low == perm.high {
            s.write_fmt(format_args!("{{ {:#06X} }}", perm.low)).ok();
        } else {
            s.write_fmt(format_args!("{{ {:#06X}-{:#06X} }}", perm.low, perm.high))
                .ok();
        }
        s
    }
}
