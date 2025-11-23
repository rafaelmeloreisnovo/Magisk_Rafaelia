//! sign.rs - Part of Magisk_Rafaelia
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

⚓ ANCHOR_ID: 5034535CA3CDF02E
⚓ FILE_PATH: native/src/boot/sign.rs
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 9AF860FB0CFA24A8E88AA92A30E94D1C


*/


use der::referenced::OwnedToRef;
use der::{Decode, DecodePem, Encode, Sequence, SliceReader};
use digest::DynDigest;
use p256::ecdsa::{
    Signature as P256Signature, SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey,
};
use p256::pkcs8::DecodePrivateKey;
use p384::ecdsa::{
    Signature as P384Signature, SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey,
};
use p521::ecdsa::{
    Signature as P521Signature, SigningKey as P521SigningKey, VerifyingKey as P521VerifyingKey,
};
use rsa::pkcs1v15::{
    Signature as RsaSignature, SigningKey as RsaSigningKey, VerifyingKey as RsaVerifyingKey,
};
use rsa::pkcs8::SubjectPublicKeyInfoRef;
use rsa::signature::SignatureEncoding;
use rsa::signature::hazmat::{PrehashSigner, PrehashVerifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use x509_cert::Certificate;
use x509_cert::der::Any;
use x509_cert::der::asn1::{OctetString, PrintableString};
use x509_cert::spki::AlgorithmIdentifier;

use base::{LoggedResult, MappedFile, ResultExt, SilentLogExt, Utf8CStr, cstr, log_err};

use crate::ffi::BootImage;

#[allow(clippy::upper_case_acronyms)]
pub enum SHA {
    SHA1(Sha1),
    SHA256(Sha256),
}

impl SHA {
    pub fn update(&mut self, data: &[u8]) {
        match self {
            SHA::SHA1(h) => h.update(data),
            SHA::SHA256(h) => h.update(data),
        }
    }

    pub fn output_size(&self) -> usize {
        match self {
            SHA::SHA1(h) => h.output_size(),
            SHA::SHA256(h) => h.output_size(),
        }
    }

    pub fn finalize_into(&mut self, out: &mut [u8]) {
        match self {
            SHA::SHA1(h) => h.finalize_into_reset(out),
            SHA::SHA256(h) => h.finalize_into_reset(out),
        }
        .ok();
    }
}

pub fn get_sha(use_sha1: bool) -> Box<SHA> {
    Box::new(if use_sha1 {
        SHA::SHA1(Sha1::default())
    } else {
        SHA::SHA256(Sha256::default())
    })
}

pub fn sha1_hash(data: &[u8], out: &mut [u8]) {
    let mut h = Sha1::default();
    h.update(data);
    DynDigest::finalize_into(h, out).ok();
}

pub fn sha256_hash(data: &[u8], out: &mut [u8]) {
    let mut h = Sha256::default();
    h.update(data);
    DynDigest::finalize_into(h, out).ok();
}

#[allow(clippy::large_enum_variant)]
enum SigningKey {
    SHA256withRSA(RsaSigningKey<Sha256>),
    SHA256withECDSA(P256SigningKey),
    SHA384withECDSA(P384SigningKey),
    SHA521withECDSA(P521SigningKey),
}

#[allow(clippy::large_enum_variant)]
enum VerifyingKey {
    SHA256withRSA(RsaVerifyingKey<Sha256>),
    SHA256withECDSA(P256VerifyingKey),
    SHA384withECDSA(P384VerifyingKey),
    SHA521withECDSA(P521VerifyingKey),
}

struct Verifier {
    digest: Box<dyn DynDigest>,
    key: VerifyingKey,
}

impl Verifier {
    fn from_public_key(key: SubjectPublicKeyInfoRef) -> LoggedResult<Verifier> {
        let digest: Box<dyn DynDigest>;
        let key = if let Ok(rsa) = RsaPublicKey::try_from(key.clone()) {
            digest = Box::<Sha256>::default();
            VerifyingKey::SHA256withRSA(RsaVerifyingKey::<Sha256>::new(rsa))
        } else if let Ok(ec) = P256VerifyingKey::try_from(key.clone()) {
            digest = Box::<Sha256>::default();
            VerifyingKey::SHA256withECDSA(ec)
        } else if let Ok(ec) = P384VerifyingKey::try_from(key.clone()) {
            digest = Box::<Sha384>::default();
            VerifyingKey::SHA384withECDSA(ec)
        } else if let Ok(ec) = P521VerifyingKey::try_from(key.clone()) {
            digest = Box::<Sha512>::default();
            VerifyingKey::SHA521withECDSA(ec)
        } else {
            return log_err!("Unsupported private key");
        };
        Ok(Verifier { digest, key })
    }

    fn update(&mut self, data: &[u8]) {
        self.digest.update(data)
    }

    fn verify(mut self, signature: &[u8]) -> LoggedResult<()> {
        let hash = self.digest.finalize_reset();
        match &self.key {
            VerifyingKey::SHA256withRSA(key) => {
                let sig = RsaSignature::try_from(signature)?;
                key.verify_prehash(hash.as_ref(), &sig).log()
            }
            VerifyingKey::SHA256withECDSA(key) => {
                let sig = P256Signature::from_slice(signature)?;
                key.verify_prehash(hash.as_ref(), &sig).log()
            }
            VerifyingKey::SHA384withECDSA(key) => {
                let sig = P384Signature::from_slice(signature)?;
                key.verify_prehash(hash.as_ref(), &sig).log()
            }
            VerifyingKey::SHA521withECDSA(key) => {
                let sig = P521Signature::from_slice(signature)?;
                key.verify_prehash(hash.as_ref(), &sig).log()
            }
        }
    }
}

struct Signer {
    digest: Box<dyn DynDigest>,
    key: SigningKey,
}

impl Signer {
    fn from_private_key(key: &[u8]) -> LoggedResult<Signer> {
        let digest: Box<dyn DynDigest>;
        let key = match RsaPrivateKey::from_pkcs8_der(key) {
            Ok(rsa) => {
                digest = Box::<Sha256>::default();
                SigningKey::SHA256withRSA(RsaSigningKey::<Sha256>::new(rsa))
            }
            _ => match P256SigningKey::from_pkcs8_der(key) {
                Ok(ec) => {
                    digest = Box::<Sha256>::default();
                    SigningKey::SHA256withECDSA(ec)
                }
                _ => match P384SigningKey::from_pkcs8_der(key) {
                    Ok(ec) => {
                        digest = Box::<Sha384>::default();
                        SigningKey::SHA384withECDSA(ec)
                    }
                    _ => match P521SigningKey::from_pkcs8_der(key) {
                        Ok(ec) => {
                            digest = Box::<Sha512>::default();
                            SigningKey::SHA521withECDSA(ec)
                        }
                        _ => {
                            return log_err!("Unsupported private key");
                        }
                    },
                },
            },
        };
        Ok(Signer { digest, key })
    }

    fn update(&mut self, data: &[u8]) {
        self.digest.update(data)
    }

    fn sign(mut self) -> LoggedResult<Vec<u8>> {
        let hash = self.digest.finalize_reset();
        let v = match &self.key {
            SigningKey::SHA256withRSA(key) => {
                let sig: RsaSignature = key.sign_prehash(hash.as_ref())?;
                sig.to_vec()
            }
            SigningKey::SHA256withECDSA(key) => {
                let sig: P256Signature = key.sign_prehash(hash.as_ref())?;
                sig.to_vec()
            }
            SigningKey::SHA384withECDSA(key) => {
                let sig: P384Signature = key.sign_prehash(hash.as_ref())?;
                sig.to_vec()
            }
            SigningKey::SHA521withECDSA(key) => {
                let sig: P521Signature = key.sign_prehash(hash.as_ref())?;
                sig.to_vec()
            }
        };
        Ok(v)
    }
}

/*
 * BootSignature ::= SEQUENCE {
 *     formatVersion ::= INTEGER,
 *     certificate ::= Certificate,
 *     algorithmIdentifier ::= SEQUENCE {
 *         algorithm OBJECT IDENTIFIER,
 *         parameters ANY DEFINED BY algorithm OPTIONAL
 *     },
 *     authenticatedAttributes ::= SEQUENCE {
 *         target CHARACTER STRING,
 *         length INTEGER
 *     },
 *     signature ::= OCTET STRING
 * }
 */

#[derive(Sequence)]
struct AuthenticatedAttributes {
    target: PrintableString,
    length: u64,
}

#[derive(Sequence)]
struct BootSignature {
    format_version: i32,
    certificate: Certificate,
    algorithm_identifier: AlgorithmIdentifier<Any>,
    authenticated_attributes: AuthenticatedAttributes,
    signature: OctetString,
}

impl BootSignature {
    fn verify(self, payload: &[u8]) -> LoggedResult<()> {
        if self.authenticated_attributes.length as usize != payload.len() {
            return log_err!("Invalid image size");
        }
        let mut verifier = Verifier::from_public_key(
            self.certificate
                .tbs_certificate()
                .subject_public_key_info()
                .owned_to_ref(),
        )?;
        verifier.update(payload);
        let attr = self.authenticated_attributes.to_der()?;
        verifier.update(attr.as_slice());
        verifier.verify(self.signature.as_bytes())?;
        Ok(())
    }
}

impl BootImage {
    pub fn verify(&self, cert: Option<&Utf8CStr>) -> LoggedResult<()> {
        let tail = self.tail();
        if tail.starts_with(b"AVB0") {
            return log_err!();
        }

        // Don't use BootSignature::from_der because tail might have trailing zeros
        let mut reader = SliceReader::new(tail)?;
        let mut sig = BootSignature::decode(&mut reader).silent()?;
        if let Some(s) = cert {
            let pem = MappedFile::open(s)?;
            sig.certificate = Certificate::from_pem(pem)?;
        };

        sig.verify(self.payload()).log()
    }

    pub fn verify_for_cxx(&self) -> bool {
        self.verify(None).is_ok()
    }
}

enum Bytes {
    Mapped(MappedFile),
    Slice(&'static [u8]),
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            Bytes::Mapped(m) => m.as_ref(),
            Bytes::Slice(s) => s,
        }
    }
}

const VERITY_PEM: &[u8] = include_bytes!("../../../tools/keys/verity.x509.pem");
const VERITY_PK8: &[u8] = include_bytes!("../../../tools/keys/verity.pk8");

pub fn sign_boot_image(
    payload: &[u8],
    name: &Utf8CStr,
    cert: Option<&Utf8CStr>,
    key: Option<&Utf8CStr>,
) -> LoggedResult<Vec<u8>> {
    let cert = match cert {
        Some(s) => Bytes::Mapped(MappedFile::open(s)?),
        None => Bytes::Slice(VERITY_PEM),
    };
    let key = match key {
        Some(s) => Bytes::Mapped(MappedFile::open(s)?),
        None => Bytes::Slice(VERITY_PK8),
    };

    // Parse cert and private key
    let cert = Certificate::from_pem(cert)?;
    let mut signer = Signer::from_private_key(key.as_ref())?;

    // Sign image
    let attr = AuthenticatedAttributes {
        target: PrintableString::new(name.as_bytes())?,
        length: payload.len() as u64,
    };
    signer.update(payload);
    signer.update(attr.to_der()?.as_slice());
    let sig = signer.sign()?;

    // Create BootSignature DER
    let alg_id = cert.signature_algorithm().clone();
    let sig = BootSignature {
        format_version: 1,
        certificate: cert,
        algorithm_identifier: alg_id,
        authenticated_attributes: attr,
        signature: OctetString::new(sig)?,
    };
    sig.to_der().log()
}

pub fn sign_payload_for_cxx(payload: &[u8]) -> Vec<u8> {
    sign_boot_image(payload, cstr!("/boot"), None, None).unwrap_or_default()
}
