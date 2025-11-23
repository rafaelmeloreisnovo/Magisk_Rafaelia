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

use crate::ffi::FileFormat;
use base::{Utf8CStr, cstr, libc};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl FromStr for FileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Self::GZIP),
            "zopfli" => Ok(Self::ZOPFLI),
            "xz" => Ok(Self::XZ),
            "lzma" => Ok(Self::LZMA),
            "bzip2" => Ok(Self::BZIP2),
            "lz4" => Ok(Self::LZ4),
            "lz4_legacy" => Ok(Self::LZ4_LEGACY),
            "lz4_lg" => Ok(Self::LZ4_LG),
            _ => Err(()),
        }
    }
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_cstr())
    }
}

impl FileFormat {
    fn as_cstr(&self) -> &'static Utf8CStr {
        match *self {
            Self::GZIP => cstr!("gzip"),
            Self::ZOPFLI => cstr!("zopfli"),
            Self::LZOP => cstr!("lzop"),
            Self::XZ => cstr!("xz"),
            Self::LZMA => cstr!("lzma"),
            Self::BZIP2 => cstr!("bzip2"),
            Self::LZ4 => cstr!("lz4"),
            Self::LZ4_LEGACY => cstr!("lz4_legacy"),
            Self::LZ4_LG => cstr!("lz4_lg"),
            Self::DTB => cstr!("dtb"),
            Self::ZIMAGE => cstr!("zimage"),
            _ => cstr!("raw"),
        }
    }
}

impl FileFormat {
    pub fn ext(&self) -> &'static str {
        match *self {
            Self::GZIP | Self::ZOPFLI => "gz",
            Self::LZOP => "lzo",
            Self::XZ => "xz",
            Self::LZMA => "lzma",
            Self::BZIP2 => "bz2",
            Self::LZ4 | Self::LZ4_LEGACY | Self::LZ4_LG => "lz4",
            _ => "",
        }
    }

    pub fn is_compressed(&self) -> bool {
        matches!(
            *self,
            Self::GZIP
                | Self::ZOPFLI
                | Self::XZ
                | Self::LZMA
                | Self::BZIP2
                | Self::LZ4
                | Self::LZ4_LEGACY
                | Self::LZ4_LG
        )
    }

    pub fn formats() -> String {
        [
            Self::GZIP,
            Self::ZOPFLI,
            Self::XZ,
            Self::LZMA,
            Self::BZIP2,
            Self::LZ4,
            Self::LZ4_LEGACY,
            Self::LZ4_LG,
        ]
        .map(|f| f.to_string())
        .join(" ")
    }
}

// C++ FFI

pub fn fmt2name(fmt: FileFormat) -> *const libc::c_char {
    fmt.as_cstr().as_ptr()
}

pub fn fmt_compressed(fmt: FileFormat) -> bool {
    fmt.is_compressed()
}

pub fn fmt_compressed_any(fmt: FileFormat) -> bool {
    fmt.is_compressed() || matches!(fmt, FileFormat::LZOP)
}
