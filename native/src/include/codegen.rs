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

use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io, process};
use std::{env, path::PathBuf};

use cxx_gen::{Include, IncludeKind, Opt};

trait ResultExt<T> {
    fn ok_or_exit(self) -> T;
}

impl<T, E: Display> ResultExt<T> for Result<T, E> {
    fn ok_or_exit(self) -> T {
        match self {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error occurred: {e}");
                process::exit(1);
            }
        }
    }
}

fn write_if_diff<P: AsRef<Path>>(path: P, bytes: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    if let Ok(orig) = fs::read(path) {
        // Do not modify the file if content is the same to make incremental build more optimal
        if orig.as_slice() == bytes {
            return Ok(());
        }
    }
    let mut f = File::create(path)?;
    f.write_all(bytes)
}

pub fn gen_cxx_binding(name: &str) {
    println!("cargo:rerun-if-changed=lib.rs");
    let mut opt = Opt::default();
    opt.cxx_impl_annotations = Some("[[gnu::always_inline]]".to_string());
    opt.include.push(Include {
        path: "rust/cxx.h".to_string(),
        kind: IncludeKind::Bracketed,
    });
    let code = cxx_gen::generate_header_and_cc_with_path("lib.rs", &opt);
    write_if_diff(format!("{name}.cpp"), code.implementation.as_slice()).ok_or_exit();
    write_if_diff(format!("{name}.hpp"), code.header.as_slice()).ok_or_exit();
}

#[allow(dead_code)]
pub fn gen_rust_flags() {
    println!("cargo:rerun-if-env-changed=MAGISK_VERSION");
    println!("cargo:rerun-if-env-changed=MAGISK_VER_CODE");
    println!("cargo:rerun-if-env-changed=MAGISK_VERSION_CODE");

    let version = env::var("MAGISK_VERSION").unwrap_or_else(|_| "dev".to_string());
    let version_code = env::var("MAGISK_VER_CODE")
        .or_else(|_| env::var("MAGISK_VERSION_CODE"))
        .unwrap_or_else(|_| "30400".to_string());
    let out_dir = PathBuf::from(env::var("OUT_DIR").ok_or_exit());

    let content = format!(
        "pub const MAGISK_VERSION: &str = \"{version}\";\npub const MAGISK_VER_CODE: i32 = {version_code};\n"
    );
    write_if_diff(out_dir.join("flags.rs"), content.as_bytes()).ok_or_exit();
}
