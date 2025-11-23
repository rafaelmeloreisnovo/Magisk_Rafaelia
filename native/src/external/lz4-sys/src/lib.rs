//! lib.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: F59EC7178B9210DB
⚓ FILE_PATH: native/src/external/lz4-sys/src/lib.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 911951EDB75CD44664B9B061AA6E60A2


*/


#![allow(unexpected_cfgs)]
#![no_std]
extern crate libc;

#[cfg(not(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
)))]
pub use libc::{c_char, c_int, c_uint, c_ulonglong, c_void, size_t};

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
extern crate alloc;

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
mod wasm_shim;

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
extern crate std;

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
pub use std::os::raw::{c_char, c_int, c_uint, c_ulonglong, c_void};

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[allow(non_camel_case_types)]
pub type size_t = usize;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LZ4FCompressionContext(pub *mut c_void);
unsafe impl Send for LZ4FCompressionContext {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LZ4FDecompressionContext(pub *mut c_void);
unsafe impl Send for LZ4FDecompressionContext {}

pub type LZ4FErrorCode = size_t;

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum BlockSize {
    Default = 0, // Default - 64KB
    Max64KB = 4,
    Max256KB = 5,
    Max1MB = 6,
    Max4MB = 7,
}

impl BlockSize {
    pub fn get_size(&self) -> usize {
        match self {
            &BlockSize::Default | &BlockSize::Max64KB => 64 * 1024,
            &BlockSize::Max256KB => 256 * 1024,
            &BlockSize::Max1MB => 1 * 1024 * 1024,
            &BlockSize::Max4MB => 4 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum BlockMode {
    Linked = 0,
    Independent,
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum ContentChecksum {
    NoChecksum = 0,
    ChecksumEnabled,
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum FrameType {
    Frame = 0,
    SkippableFrame,
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum BlockChecksum {
    NoBlockChecksum = 0,
    BlockChecksumEnabled,
}

#[derive(Debug)]
#[repr(C)]
pub struct LZ4FFrameInfo {
    pub block_size_id: BlockSize,
    pub block_mode: BlockMode,
    pub content_checksum_flag: ContentChecksum,
    pub frame_type: FrameType,
    pub content_size: c_ulonglong,
    pub dict_id: c_uint,
    pub block_checksum_flag: BlockChecksum,
}

#[derive(Debug)]
#[repr(C)]
pub struct LZ4FPreferences {
    pub frame_info: LZ4FFrameInfo,
    pub compression_level: c_uint, // 0 == default (fast mode); values above 16 count as 16
    pub auto_flush: c_uint,        // 1 == always flush : reduce need for tmp buffer
    pub favor_dec_speed: c_uint,   // 1 == favor decompression speed over ratio, requires level 10+
    pub reserved: [c_uint; 3],
}

#[derive(Debug)]
#[repr(C)]
pub struct LZ4FCompressOptions {
    pub stable_src: c_uint, /* 1 == src content will remain available on future calls
                             * to LZ4F_compress(); avoid saving src content within tmp
                             * buffer as future dictionary */
    pub reserved: [c_uint; 3],
}

#[derive(Debug)]
#[repr(C)]
pub struct LZ4FDecompressOptions {
    pub stable_dst: c_uint, /* guarantee that decompressed data will still be there on next
                             * function calls (avoid storage into tmp buffers) */
    pub reserved: [c_uint; 3],
}

#[derive(Debug)]
#[repr(C)]
pub struct LZ4StreamEncode(c_void);

#[derive(Debug)]
#[repr(C)]
pub struct LZ4StreamDecode(c_void);

pub const LZ4F_VERSION: c_uint = 100;

extern "C" {

    // int LZ4_compress_default(const char* source, char* dest, int sourceSize, int maxDestSize);
    #[allow(non_snake_case)]
    pub fn LZ4_compress_default(
        source: *const c_char,
        dest: *mut c_char,
        sourceSize: c_int,
        maxDestSize: c_int,
    ) -> c_int;

    // int LZ4_compress_fast (const char* source, char* dest, int sourceSize, int maxDestSize, int acceleration);
    #[allow(non_snake_case)]
    pub fn LZ4_compress_fast(
        source: *const c_char,
        dest: *mut c_char,
        sourceSize: c_int,
        maxDestSize: c_int,
        acceleration: c_int,
    ) -> c_int;

    // int LZ4_compress_HC (const char* src, char* dst, int srcSize, int dstCapacity, int compressionLevel);
    #[allow(non_snake_case)]
    pub fn LZ4_compress_HC(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        compressionLevel: c_int,
    ) -> c_int;

    // int LZ4_decompress_safe (const char* source, char* dest, int compressedSize, int maxDecompressedSize);
    #[allow(non_snake_case)]
    pub fn LZ4_decompress_safe(
        source: *const c_char,
        dest: *mut c_char,
        compressedSize: c_int,
        maxDecompressedSize: c_int,
    ) -> c_int;

    // unsigned    LZ4F_isError(LZ4F_errorCode_t code);
    pub fn LZ4F_isError(code: size_t) -> c_uint;

    // const char* LZ4F_getErrorName(LZ4F_errorCode_t code);
    pub fn LZ4F_getErrorName(code: size_t) -> *const c_char;

    // LZ4F_createCompressionContext() :
    // The first thing to do is to create a compressionContext object, which will be used in all
    // compression operations.
    // This is achieved using LZ4F_createCompressionContext(), which takes as argument a version
    // and an LZ4F_preferences_t structure.
    // The version provided MUST be LZ4F_VERSION. It is intended to track potential version
    // differences between different binaries.
    // The function will provide a pointer to a fully allocated LZ4F_compressionContext_t object.
    // If the result LZ4F_errorCode_t is not zero, there was an error during context creation.
    // Object can release its memory using LZ4F_freeCompressionContext();
    //
    // LZ4F_errorCode_t LZ4F_createCompressionContext(
    //                                   LZ4F_compressionContext_t* LZ4F_compressionContextPtr,
    //                                   unsigned version);
    pub fn LZ4F_createCompressionContext(
        ctx: &mut LZ4FCompressionContext,
        version: c_uint,
    ) -> LZ4FErrorCode;

    // LZ4F_errorCode_t LZ4F_freeCompressionContext(
    //                                  LZ4F_compressionContext_t LZ4F_compressionContext);
    pub fn LZ4F_freeCompressionContext(ctx: LZ4FCompressionContext) -> LZ4FErrorCode;

    // LZ4F_compressBegin() :
    // will write the frame header into dstBuffer.
    // dstBuffer must be large enough to accommodate a header (dstMaxSize). Maximum header
    // size is 19 bytes.
    // The LZ4F_preferences_t structure is optional : you can provide NULL as argument, all
    // preferences will then be set to default.
    // The result of the function is the number of bytes written into dstBuffer for the header
    // or an error code (can be tested using LZ4F_isError())
    //
    // size_t LZ4F_compressBegin(LZ4F_compressionContext_t compressionContext,
    //                           void* dstBuffer,
    //                           size_t dstMaxSize,
    //                           const LZ4F_preferences_t* preferencesPtr);
    pub fn LZ4F_compressBegin(
        ctx: LZ4FCompressionContext,
        dstBuffer: *mut u8,
        dstMaxSize: size_t,
        preferencesPtr: *const LZ4FPreferences,
    ) -> LZ4FErrorCode;

    // LZ4F_compressBound() :
    // Provides the minimum size of Dst buffer given srcSize to handle worst case situations.
    // preferencesPtr is optional : you can provide NULL as argument, all preferences will then
    // be set to default.
    // Note that different preferences will produce in different results.
    //
    // size_t LZ4F_compressBound(size_t srcSize, const LZ4F_preferences_t* preferencesPtr);
    pub fn LZ4F_compressBound(
        srcSize: size_t,
        preferencesPtr: *const LZ4FPreferences,
    ) -> LZ4FErrorCode;

    // LZ4F_compressUpdate()
    // LZ4F_compressUpdate() can be called repetitively to compress as much data as necessary.
    // The most important rule is that dstBuffer MUST be large enough (dstMaxSize) to ensure
    // compression completion even in worst case.
    // If this condition is not respected, LZ4F_compress() will fail (result is an errorCode)
    // You can get the minimum value of dstMaxSize by using LZ4F_compressBound()
    // The LZ4F_compressOptions_t structure is optional : you can provide NULL as argument.
    // The result of the function is the number of bytes written into dstBuffer : it can be zero,
    // meaning input data was just buffered.
    // The function outputs an error code if it fails (can be tested using LZ4F_isError())
    //
    // size_t LZ4F_compressUpdate(LZ4F_compressionContext_t compressionContext,
    //                            void* dstBuffer,
    //                            size_t dstMaxSize,
    //                            const void* srcBuffer,
    //                            size_t srcSize,
    //                            const LZ4F_compressOptions_t* compressOptionsPtr);
    pub fn LZ4F_compressUpdate(
        ctx: LZ4FCompressionContext,
        dstBuffer: *mut u8,
        dstMaxSize: size_t,
        srcBuffer: *const u8,
        srcSize: size_t,
        compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> size_t;

    // LZ4F_flush()
    // Should you need to create compressed data immediately, without waiting for a block
    // to be be filled, you can call LZ4_flush(), which will immediately compress any remaining
    // data buffered within compressionContext.
    // The LZ4F_compressOptions_t structure is optional : you can provide NULL as argument.
    // The result of the function is the number of bytes written into dstBuffer
    // (it can be zero, this means there was no data left within compressionContext)
    // The function outputs an error code if it fails (can be tested using LZ4F_isError())
    //
    // size_t LZ4F_flush(LZ4F_compressionContext_t compressionContext,
    //                   void* dstBuffer,
    //                   size_t dstMaxSize,
    //                   const LZ4F_compressOptions_t* compressOptionsPtr);
    pub fn LZ4F_flush(
        ctx: LZ4FCompressionContext,
        dstBuffer: *mut u8,
        dstMaxSize: size_t,
        compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> LZ4FErrorCode;

    // LZ4F_compressEnd()
    // When you want to properly finish the compressed frame, just call LZ4F_compressEnd().
    // It will flush whatever data remained within compressionContext (like LZ4_flush())
    // but also properly finalize the frame, with an endMark and a checksum.
    // The result of the function is the number of bytes written into dstBuffer
    // (necessarily >= 4 (endMark size))
    // The function outputs an error code if it fails (can be tested using LZ4F_isError())
    // The LZ4F_compressOptions_t structure is optional : you can provide NULL as argument.
    // compressionContext can then be used again, starting with LZ4F_compressBegin().
    //
    // size_t LZ4F_compressEnd(LZ4F_compressionContext_t compressionContext,
    //                         void* dstBuffer,
    //                         size_t dstMaxSize,
    //                         const LZ4F_compressOptions_t* compressOptionsPtr);
    pub fn LZ4F_compressEnd(
        ctx: LZ4FCompressionContext,
        dstBuffer: *mut u8,
        dstMaxSize: size_t,
        compressOptionsPtr: *const LZ4FCompressOptions,
    ) -> LZ4FErrorCode;

    // LZ4F_createDecompressionContext() :
    // The first thing to do is to create a decompressionContext object, which will be used
    // in all decompression operations.
    // This is achieved using LZ4F_createDecompressionContext().
    // The version provided MUST be LZ4F_VERSION. It is intended to track potential version
    // differences between different binaries.
    // The function will provide a pointer to a fully allocated and initialized
    // LZ4F_decompressionContext_t object.
    // If the result LZ4F_errorCode_t is not OK_NoError, there was an error during
    // context creation.
    // Object can release its memory using LZ4F_freeDecompressionContext();
    //
    // LZ4F_errorCode_t LZ4F_createDecompressionContext(LZ4F_decompressionContext_t* ctxPtr,
    //                                                  unsigned version);
    pub fn LZ4F_createDecompressionContext(
        ctx: &mut LZ4FDecompressionContext,
        version: c_uint,
    ) -> LZ4FErrorCode;

    // LZ4F_errorCode_t LZ4F_freeDecompressionContext(LZ4F_decompressionContext_t ctx);
    pub fn LZ4F_freeDecompressionContext(ctx: LZ4FDecompressionContext) -> LZ4FErrorCode;

    // LZ4F_getFrameInfo()
    // This function decodes frame header information, such as blockSize.
    // It is optional : you could start by calling directly LZ4F_decompress() instead.
    // The objective is to extract header information without starting decompression, typically
    // for allocation purposes.
    // LZ4F_getFrameInfo() can also be used *after* starting decompression, on a
    // valid LZ4F_decompressionContext_t.
    // The number of bytes read from srcBuffer will be provided within *srcSizePtr
    // (necessarily <= original value).
    // You are expected to resume decompression from where it stopped (srcBuffer + *srcSizePtr)
    // The function result is an hint of how many srcSize bytes LZ4F_decompress() expects for
    // next call, or an error code which can be tested using LZ4F_isError().
    //
    // size_t LZ4F_getFrameInfo(LZ4F_decompressionContext_t ctx,
    // 					LZ4F_frameInfo_t* frameInfoPtr,
    // 					const void* srcBuffer, size_t* srcSizePtr);
    pub fn LZ4F_getFrameInfo(
        ctx: LZ4FDecompressionContext,
        frameInfoPtr: &mut LZ4FFrameInfo,
        srcBuffer: *const u8,
        srcSizePtr: &mut size_t,
    ) -> LZ4FErrorCode;

    // LZ4F_decompress()
    // Call this function repetitively to regenerate data compressed within srcBuffer.
    // The function will attempt to decode *srcSizePtr bytes from srcBuffer, into dstBuffer of
    // maximum size *dstSizePtr.
    //
    // The number of bytes regenerated into dstBuffer will be provided within *dstSizePtr
    // (necessarily <= original value).
    //
    // The number of bytes read from srcBuffer will be provided within *srcSizePtr
    // (necessarily <= original value).
    // If number of bytes read is < number of bytes provided, then decompression operation
    // is not completed. It typically happens when dstBuffer is not large enough to contain
    // all decoded data.
    // LZ4F_decompress() must be called again, starting from where it stopped
    // (srcBuffer + *srcSizePtr)
    // The function will check this condition, and refuse to continue if it is not respected.
    //
    // dstBuffer is supposed to be flushed between each call to the function, since its content
    // will be overwritten.
    // dst arguments can be changed at will with each consecutive call to the function.
    //
    // The function result is an hint of how many srcSize bytes LZ4F_decompress() expects for
    // next call.
    // Schematically, it's the size of the current (or remaining) compressed block + header of
    // next block.
    // Respecting the hint provides some boost to performance, since it does skip intermediate
    // buffers.
    // This is just a hint, you can always provide any srcSize you want.
    // When a frame is fully decoded, the function result will be 0. (no more data expected)
    // If decompression failed, function result is an error code, which can be tested
    // using LZ4F_isError().
    //
    // size_t LZ4F_decompress(LZ4F_decompressionContext_t ctx,
    //                        void* dstBuffer, size_t* dstSizePtr,
    //                        const void* srcBuffer, size_t* srcSizePtr,
    //                        const LZ4F_decompressOptions_t* optionsPtr);
    pub fn LZ4F_decompress(
        ctx: LZ4FDecompressionContext,
        dstBuffer: *mut u8,
        dstSizePtr: &mut size_t,
        srcBuffer: *const u8,
        srcSizePtr: &mut size_t,
        optionsPtr: *const LZ4FDecompressOptions,
    ) -> LZ4FErrorCode;

    // int LZ4_versionNumber(void)
    pub fn LZ4_versionNumber() -> c_int;

    // int LZ4_compressBound(int isize)
    pub fn LZ4_compressBound(size: c_int) -> c_int;

    // LZ4_stream_t* LZ4_createStream(void)
    pub fn LZ4_createStream() -> *mut LZ4StreamEncode;

    // int LZ4_compress_continue(LZ4_stream_t* LZ4_streamPtr,
    //                           const char* source,
    //                           char* dest,
    //                           int inputSize)
    pub fn LZ4_compress_continue(
        LZ4_stream: *mut LZ4StreamEncode,
        source: *const u8,
        dest: *mut u8,
        input_size: c_int,
    ) -> c_int;

    // int LZ4_freeStream(LZ4_stream_t* LZ4_streamPtr)
    pub fn LZ4_freeStream(LZ4_stream: *mut LZ4StreamEncode) -> c_int;

    // int LZ4_setStreamDecode (LZ4_streamDecode_t* LZ4_streamDecode,
    //                          const char* dictionary,
    //                          int dictSize)
    pub fn LZ4_setStreamDecode(
        LZ4_stream: *mut LZ4StreamDecode,
        dictionary: *const u8,
        dict_size: c_int,
    ) -> c_int;

    // LZ4_streamDecode_t* LZ4_createStreamDecode(void)
    pub fn LZ4_createStreamDecode() -> *mut LZ4StreamDecode;

    // int LZ4_decompress_safe_continue(LZ4_streamDecode_t* LZ4_streamDecode,
    //                                  const char* source,
    //                                  char* dest,
    //                                  int compressedSize,
    //                                  int maxDecompressedSize)
    pub fn LZ4_decompress_safe_continue(
        LZ4_stream: *mut LZ4StreamDecode,
        source: *const u8,
        dest: *mut u8,
        compressed_size: c_int,
        max_decompressed_size: c_int,
    ) -> c_int;

    // int LZ4_freeStreamDecode(LZ4_streamDecode_t* LZ4_stream)
    pub fn LZ4_freeStreamDecode(LZ4_stream: *mut LZ4StreamDecode) -> c_int;

    // LZ4F_resetDecompressionContext()
    // In case of an error, the context is left in "undefined" state.
    // In which case, it's necessary to reset it, before re-using it.
    // This method can also be used to abruptly stop any unfinished decompression,
    // and start a new one using same context resources.
    pub fn LZ4F_resetDecompressionContext(ctx: LZ4FDecompressionContext);

}

#[test]
fn test_version_number() {
    unsafe {
        LZ4_versionNumber();
    }
}

#[test]
fn test_frame_info_size() {
    assert_eq!(core::mem::size_of::<LZ4FFrameInfo>(), 32);
}
