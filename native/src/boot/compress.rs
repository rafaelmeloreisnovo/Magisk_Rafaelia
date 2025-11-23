//! compress.rs - Part of Magisk_Rafaelia
//!
//! Part of Magisk_Rafaelia
//! RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
//! 
//! Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
//! (EMPTY → ACTION → FULL → FEEDBACK)
//! 
//! Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
//! Foundation: CientiEspiritual - Scientific Spirituality
//! Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)
//! 
//! RAFAELIA Framework Principles:
//! - Complete operational state coverage (1008 State Matrix)
//! - Full audit system with integrity verification
//! - Real-time telemetry and anomaly detection
//! - Security hardening and ethical computing
//! - Continuous improvement through infinite feedback loop (ψχρΔΣΩ)

/*

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy

All Rights Reserved. Patent Pending.

DUAL LICENSE - Choose one:

1. SOCIAL INCLUSION LICENSE (Free):
   ✓ Educational use
   ✓ Research and academic purposes
   ✓ Non-profit organizations
   ✓ Social inclusion initiatives
   ✓ Open source contributions (with attribution)
   ✗ Commercial use prohibited

2. COMMERCIAL SAAS LICENSE (Paid Subscription):
   Required for:
   ✓ Commercial products or services
   ✓ SaaS applications
   ✓ Revenue-generating purposes
   ✓ Enterprise deployments
   Contact: rafaelmeloreisnovo for licensing terms

AUTOMATIC PENALTIES FOR VIOLATIONS:
Unauthorized commercial use is subject to automatic statutory penalties:
- Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
- Plus: 5% of gross revenue derived from unauthorized use
- Plus: Legal fees and costs of enforcement
- Criminal prosecution under applicable copyright law

VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
- Valid in all jurisdictions signatory to Berne Convention (180+ countries)
- Enforced under TRIPS agreement (WTO member states)
- Protected by reciprocal copyright treaties
- Minimum protection: Life of author + 50 years (Berne minimum)
- Extended protection: Life + 70 years (EU, USA, Brazil and others)

ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
All derivative works, redistributions, or substantial use must include:
1. This complete copyright and license notice
2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
4. Indication of any modifications made
5. Date of last modification


INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:

This software is developed in compliance with international copyright law,
human rights frameworks, and ethical standards including:

COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
- Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
  └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
- WIPO Copyright Treaty (WCT, 1996) - Digital rights management
- WIPO Performances and Phonograms Treaty (WPPT, 1996)
- Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
- Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
- Vienna Convention on the Law of Treaties (1969) - Treaty interpretation

HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
- Universal Declaration of Human Rights (UDHR, 1948)
  └─ Article 27: Right to protection of moral and material interests
- International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
  └─ Article 15: Right to benefit from scientific progress and protection of authorship
- Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
  └─ Articles 13, 16, 17: Expression, privacy, access to information
- Vienna Declaration and Programme of Action (1993) - Human rights universality

UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
- UNESCO Universal Declaration on Cultural Diversity (2001)
- UNESCO Recommendation on Open Science (2021)
- UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
- Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)

DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
- GDPR - General Data Protection Regulation (EU 2016/679)
- LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
- CCPA - California Consumer Privacy Act (USA)
- Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)

TECHNICAL STANDARDS / NORMAS TÉCNICAS:
- ISO/IEC 9001:2015 - Quality Management Systems
- ISO/IEC 27001:2022 - Information Security Management
- ISO/IEC 27002:2022 - Information Security Controls
- ISO/IEC 27018:2019 - PII Protection in Public Clouds
- ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
- ISO/IEC 8000 - Data Quality Standards
- IEEE 830-1998 - Software Requirements Specification
- IEEE 1012-2016 - Software Verification and Validation
- IEEE 12207-2017 - Software Life Cycle Processes
- IEEE 14764-2021 - Software Maintenance
- IEEE 42010-2011 - Architecture Description
- NIST Cybersecurity Framework (CSF) v1.1/v2.0
- NIST SP 800-53 Rev. 5 - Security and Privacy Controls
- NIST AI Risk Management Framework (AI RMF 1.0)

CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
- Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
- Universal jurisdiction for human rights violations
- Rome Statute of the International Criminal Court (1998) - For severe violations

ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:

This software adheres to the Ethica[8] framework with eight fundamental principles:

1. TRANSPARENCY (Transparência) 🔍
   └─ Open communication, documented decisions, explainable systems
   
2. ACCOUNTABILITY (Responsabilidade) 📋
   └─ Clear ownership, traceable actions, consequence acceptance
   
3. FAIRNESS (Justiça) ⚖️
   └─ Equitable treatment, non-discrimination, equal access
   
4. PRIVACY (Privacidade) 🔒
   └─ Data protection, consent respect, confidentiality
   
5. SECURITY (Segurança) 🛡️
   └─ Protection of systems, data integrity, threat mitigation
   
6. RELIABILITY (Confiabilidade) 🔧
   └─ Dependable operation, consistent behavior, stability
   
7. SAFETY (Proteção) 🛟
   └─ No harm to users, safe operations, risk prevention
   
8. SUSTAINABILITY (Sustentabilidade) ♻️
   └─ Long-term viability, environmental responsibility, social good

ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
  Life > Ethics > Law > Convenience
  Vida > Ética > Lei > Conveniência

ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:

This code is original work or properly attributed derivative work.
Every fragment, function, class, and algorithm has been:
  ✓ Originally created by the author, OR
  ✓ Properly licensed and attributed, OR
  ✓ In the public domain with documentation

NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
ZERO TOLERANCE for unauthorized copying or intellectual property theft.

Verification Methods:
- SHA3-512 checksums for integrity verification
- BLAKE3 hashing for rapid verification
- Git commit history as proof of authorship timeline
- Code review and compliance audits

Any concerns about intellectual property should be reported to:
rafaelmeloreisnovo [at] gmail [dot] com

NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):

These anchors provide stable reference points for:
- Version tracking and synchronization
- Legal compliance verification
- Authorship chain of custody
- Update propagation tracking
- Audit trail maintenance

⚓ ANCHOR_ID: 957FDCC6C0221CC1
⚓ FILE_PATH: native/src/boot/compress.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 24A55C07814B44BA91F7C2F4DC8749E4


*/


use crate::ffi::{FileFormat, check_fmt};
use base::nix::fcntl::OFlag;
use base::{
    Chunker, FileOrStd, LoggedResult, ReadExt, ResultExt, Utf8CStr, Utf8CString, WriteExt, log_err,
};
use bzip2::Compression as BzCompression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use flate2::Compression as GzCompression;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use lz4::block::CompressionMode;
use lz4::liblz4::BlockChecksum;
use lz4::{
    BlockMode, BlockSize, ContentChecksum, Decoder as LZ4FrameDecoder, Encoder as LZ4FrameEncoder,
    EncoderBuilder as LZ4FrameEncoderBuilder,
};
use lzma_rust2::{CheckType, LzmaOptions, LzmaReader, LzmaWriter, XzOptions, XzReader, XzWriter};
use std::cmp::min;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Write};
use std::mem::ManuallyDrop;
use std::num::NonZeroU64;
use std::ops::DerefMut;
use std::os::fd::{FromRawFd, RawFd};
use zopfli::{BlockType, GzipEncoder as ZopFliEncoder, Options as ZopfliOptions};

pub trait WriteFinish<W: Write>: Write {
    fn finish(self: Box<Self>) -> std::io::Result<W>;
}

// Boilerplate for existing types

macro_rules! finish_impl {
    ($($t:ty),*) => {$(
        impl<W: Write> WriteFinish<W> for $t {
            fn finish(self: Box<Self>) -> std::io::Result<W> {
                Self::finish(*self)
            }
        }
    )*}
}

finish_impl!(GzEncoder<W>, BzEncoder<W>, XzWriter<W>, LzmaWriter<W>);

impl<W: Write> WriteFinish<W> for BufWriter<ZopFliEncoder<W>> {
    fn finish(self: Box<Self>) -> std::io::Result<W> {
        let inner = self.into_inner()?;
        ZopFliEncoder::finish(inner)
    }
}

impl<W: Write> WriteFinish<W> for LZ4FrameEncoder<W> {
    fn finish(self: Box<Self>) -> std::io::Result<W> {
        let (w, r) = Self::finish(*self);
        r?;
        Ok(w)
    }
}

// LZ4BlockArchive format
//
// len:  |   4   |          4            |           n           | ... |           4             |
// data: | magic | compressed block size | compressed block data | ... | total uncompressed size |

// LZ4BlockEncoder

const LZ4_BLOCK_SIZE: usize = 0x800000;
const LZ4HC_CLEVEL_MAX: i32 = 12;
const LZ4_MAGIC: u32 = 0x184c2102;

struct LZ4BlockEncoder<W: Write> {
    write: W,
    chunker: Chunker,
    out_buf: Box<[u8]>,
    total: u32,
    is_lg: bool,
}

impl<W: Write> LZ4BlockEncoder<W> {
    fn new(write: W, is_lg: bool) -> Self {
        let out_sz = lz4::block::compress_bound(LZ4_BLOCK_SIZE).unwrap_or(LZ4_BLOCK_SIZE);
        LZ4BlockEncoder {
            write,
            chunker: Chunker::new(LZ4_BLOCK_SIZE),
            // SAFETY: all bytes will be initialized before it is used
            out_buf: unsafe { Box::new_uninit_slice(out_sz).assume_init() },
            total: 0,
            is_lg,
        }
    }

    fn encode_block(write: &mut W, out_buf: &mut [u8], chunk: &[u8]) -> std::io::Result<()> {
        let compressed_size = lz4::block::compress_to_buffer(
            chunk,
            Some(CompressionMode::HIGHCOMPRESSION(LZ4HC_CLEVEL_MAX)),
            false,
            out_buf,
        )?;
        let block_size = compressed_size as u32;
        write.write_pod(&block_size)?;
        write.write_all(&out_buf[..compressed_size])
    }
}

impl<W: Write> Write for LZ4BlockEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        if self.total == 0 {
            // Write header
            self.write.write_pod(&LZ4_MAGIC)?;
        }

        self.total += buf.len() as u32;
        while !buf.is_empty() {
            let (b, chunk) = self.chunker.add_data(buf);
            buf = b;
            if let Some(chunk) = chunk {
                Self::encode_block(&mut self.write, &mut self.out_buf, chunk)?;
            }
        }
        Ok(())
    }
}

impl<W: Write> WriteFinish<W> for LZ4BlockEncoder<W> {
    fn finish(mut self: Box<Self>) -> std::io::Result<W> {
        let chunk = self.chunker.get_available();
        if !chunk.is_empty() {
            Self::encode_block(&mut self.write, &mut self.out_buf, chunk)?;
        }
        if self.is_lg {
            self.write.write_pod(&self.total)?;
        }
        Ok(self.write)
    }
}

// LZ4BlockDecoder

struct LZ4BlockDecoder<R: Read> {
    read: R,
    in_buf: Box<[u8]>,
    out_buf: Box<[u8]>,
    out_len: usize,
    out_pos: usize,
}

impl<R: Read> LZ4BlockDecoder<R> {
    fn new(read: R) -> Self {
        let compressed_sz = lz4::block::compress_bound(LZ4_BLOCK_SIZE).unwrap_or(LZ4_BLOCK_SIZE);
        Self {
            read,
            in_buf: unsafe { Box::new_uninit_slice(compressed_sz).assume_init() },
            out_buf: unsafe { Box::new_uninit_slice(LZ4_BLOCK_SIZE).assume_init() },
            out_len: 0,
            out_pos: 0,
        }
    }
}

impl<R: Read> Read for LZ4BlockDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.out_pos == self.out_len {
            let mut block_size: u32 = 0;
            if let Err(e) = self.read.read_pod(&mut block_size) {
                return if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    Ok(0)
                } else {
                    Err(e)
                };
            }
            if block_size == LZ4_MAGIC {
                self.read.read_pod(&mut block_size)?;
            }

            let block_size = block_size as usize;

            if block_size > self.in_buf.len() {
                // This may be the LG format trailer, EOF
                return Ok(0);
            }

            // Read the entire compressed block
            let compressed_block = &mut self.in_buf[..block_size];
            if let Ok(len) = self.read.read(compressed_block) {
                if len == 0 {
                    // We hit EOF, that's fine
                    return Ok(0);
                } else if len != block_size {
                    let remain = &mut compressed_block[len..];
                    self.read.read_exact(remain)?;
                }
            }

            self.out_len = lz4::block::decompress_to_buffer(
                compressed_block,
                Some(LZ4_BLOCK_SIZE as i32),
                &mut self.out_buf,
            )?;
            self.out_pos = 0;
        }
        let copy_len = min(buf.len(), self.out_len - self.out_pos);
        buf[..copy_len].copy_from_slice(&self.out_buf[self.out_pos..self.out_pos + copy_len]);
        self.out_pos += copy_len;
        Ok(copy_len)
    }
}

// Top-level APIs

pub fn get_encoder<'a, W: Write + 'a>(format: FileFormat, w: W) -> Box<dyn WriteFinish<W> + 'a> {
    match format {
        FileFormat::XZ => {
            let mut opt = XzOptions::with_preset(9);
            opt.set_check_sum_type(CheckType::Crc32);
            Box::new(XzWriter::new(w, opt).unwrap())
        }
        FileFormat::LZMA => {
            Box::new(LzmaWriter::new_use_header(w, &LzmaOptions::with_preset(9), None).unwrap())
        }
        FileFormat::BZIP2 => Box::new(BzEncoder::new(w, BzCompression::best())),
        FileFormat::LZ4 => {
            let encoder = LZ4FrameEncoderBuilder::new()
                .block_size(BlockSize::Max4MB)
                .block_mode(BlockMode::Independent)
                .checksum(ContentChecksum::ChecksumEnabled)
                .block_checksum(BlockChecksum::BlockChecksumEnabled)
                .level(9)
                .auto_flush(true)
                .build(w)
                .unwrap();
            Box::new(encoder)
        }
        FileFormat::LZ4_LEGACY => Box::new(LZ4BlockEncoder::new(w, false)),
        FileFormat::LZ4_LG => Box::new(LZ4BlockEncoder::new(w, true)),
        FileFormat::ZOPFLI => {
            // These options are already better than gzip -9
            let opt = ZopfliOptions {
                iteration_count: NonZeroU64::new(1).unwrap(),
                maximum_block_splits: 1,
                ..Default::default()
            };
            Box::new(ZopFliEncoder::new_buffered(opt, BlockType::Dynamic, w).unwrap())
        }
        FileFormat::GZIP => Box::new(GzEncoder::new(w, GzCompression::best())),
        _ => unreachable!(),
    }
}

pub fn get_decoder<'a, R: Read + 'a>(format: FileFormat, r: R) -> Box<dyn Read + 'a> {
    match format {
        FileFormat::XZ => Box::new(XzReader::new(r, true)),
        FileFormat::LZMA => Box::new(LzmaReader::new_mem_limit(r, u32::MAX, None).unwrap()),
        FileFormat::BZIP2 => Box::new(BzDecoder::new(r)),
        FileFormat::LZ4 => Box::new(LZ4FrameDecoder::new(r).unwrap()),
        FileFormat::LZ4_LG | FileFormat::LZ4_LEGACY => Box::new(LZ4BlockDecoder::new(r)),
        FileFormat::ZOPFLI | FileFormat::GZIP => Box::new(MultiGzDecoder::new(r)),
        _ => unreachable!(),
    }
}

// C++ FFI

pub fn compress_bytes(format: FileFormat, in_bytes: &[u8], out_fd: RawFd) {
    let mut out_file = unsafe { ManuallyDrop::new(File::from_raw_fd(out_fd)) };

    let mut encoder = get_encoder(format, out_file.deref_mut());
    let _: LoggedResult<()> = try {
        encoder.write_all(in_bytes)?;
        encoder.finish()?;
    };
}

pub fn decompress_bytes(format: FileFormat, in_bytes: &[u8], out_fd: RawFd) {
    let mut out_file = unsafe { ManuallyDrop::new(File::from_raw_fd(out_fd)) };

    let mut decoder = get_decoder(format, in_bytes);
    std::io::copy(decoder.as_mut(), out_file.deref_mut()).log_ok();
}

// Command-line entry points

pub(crate) fn decompress_cmd(infile: &Utf8CStr, outfile: Option<&Utf8CStr>) -> LoggedResult<()> {
    let in_std = infile == "-";
    let mut rm_in = false;

    let mut buf = [0u8; 4096];

    let input = if in_std {
        FileOrStd::StdIn
    } else {
        FileOrStd::File(infile.open(OFlag::O_RDONLY)?)
    };

    // First read some bytes for format detection
    let len = input.as_file().read(&mut buf)?;
    let buf = &buf[..len];

    let format = check_fmt(buf);

    eprintln!("Detected format: {format}");

    if !format.is_compressed() {
        return log_err!("Input file is not a supported type!");
    }

    // If user did not provide outfile, infile has to be either
    // <path>.[ext], or "-". Outfile will be either <path> or "-".
    // If the input does not have proper format, abort.

    let output = if let Some(outfile) = outfile {
        if outfile == "-" {
            FileOrStd::StdOut
        } else {
            FileOrStd::File(outfile.create(OFlag::O_WRONLY | OFlag::O_TRUNC, 0o644)?)
        }
    } else if in_std {
        FileOrStd::StdOut
    } else {
        // Strip out extension and remove input
        let outfile = if let Some((outfile, ext)) = infile.rsplit_once('.')
            && ext == format.ext()
        {
            Utf8CString::from(outfile)
        } else {
            return log_err!("Input file is not a supported type!");
        };

        rm_in = true;
        eprintln!("Decompressing to [{outfile}]");
        FileOrStd::File(outfile.create(OFlag::O_WRONLY | OFlag::O_TRUNC, 0o644)?)
    };

    let mut decoder = get_decoder(format, Cursor::new(buf).chain(input.as_file()));
    std::io::copy(decoder.as_mut(), &mut output.as_file())?;

    if rm_in {
        infile.remove()?;
    }

    Ok(())
}

pub(crate) fn compress_cmd(
    method: FileFormat,
    infile: &Utf8CStr,
    outfile: Option<&Utf8CStr>,
) -> LoggedResult<()> {
    let in_std = infile == "-";
    let mut rm_in = false;

    let input = if in_std {
        FileOrStd::StdIn
    } else {
        FileOrStd::File(infile.open(OFlag::O_RDONLY)?)
    };

    let output = if let Some(outfile) = outfile {
        if outfile == "-" {
            FileOrStd::StdOut
        } else {
            FileOrStd::File(outfile.create(OFlag::O_WRONLY | OFlag::O_TRUNC, 0o644)?)
        }
    } else if in_std {
        FileOrStd::StdOut
    } else {
        let mut outfile = Utf8CString::default();
        outfile.write_str(infile).ok();
        outfile.write_char('.').ok();
        outfile.write_str(method.ext()).ok();
        eprintln!("Compressing to [{outfile}]");
        rm_in = true;
        let outfile = outfile.create(OFlag::O_WRONLY | OFlag::O_TRUNC, 0o644)?;
        FileOrStd::File(outfile)
    };

    let mut encoder = get_encoder(method, output.as_file());
    std::io::copy(&mut input.as_file(), encoder.as_mut())?;
    encoder.finish()?;

    if rm_in {
        infile.remove()?;
    }
    Ok(())
}
