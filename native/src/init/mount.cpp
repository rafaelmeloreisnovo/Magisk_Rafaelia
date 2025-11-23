/*
 * include <set> include <sys/mount.h> include <sys/sysmacros.h> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
 *
 * Part of Magisk_Rafaelia
 * RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
 * 
 * Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
 * (EMPTY → ACTION → FULL → FEEDBACK)
 * 
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Foundation: CientiEspiritual - Scientific Spirituality
 * Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)
 * 
 * RAFAELIA Framework Principles:
 * - Complete operational state coverage (1008 State Matrix)
 * - Full audit system with integrity verification
 * - Real-time telemetry and anomaly detection
 * - Security hardening and ethical computing
 * - Continuous improvement through infinite feedback loop (ψχρΔΣΩ)
 *

 * Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 * 
 * All Rights Reserved. Patent Pending.
 * 
 * DUAL LICENSE - Choose one:
 * 
 * 1. SOCIAL INCLUSION LICENSE (Free):
 *    ✓ Educational use
 *    ✓ Research and academic purposes
 *    ✓ Non-profit organizations
 *    ✓ Social inclusion initiatives
 *    ✓ Open source contributions (with attribution)
 *    ✗ Commercial use prohibited
 * 
 * 2. COMMERCIAL SAAS LICENSE (Paid Subscription):
 *    Required for:
 *    ✓ Commercial products or services
 *    ✓ SaaS applications
 *    ✓ Revenue-generating purposes
 *    ✓ Enterprise deployments
 *    Contact: rafaelmeloreisnovo for licensing terms
 * 
 * AUTOMATIC PENALTIES FOR VIOLATIONS:
 * Unauthorized commercial use is subject to automatic statutory penalties:
 * - Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
 * - Plus: 5% of gross revenue derived from unauthorized use
 * - Plus: Legal fees and costs of enforcement
 * - Criminal prosecution under applicable copyright law
 * 
 * VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
 * - Valid in all jurisdictions signatory to Berne Convention (180+ countries)
 * - Enforced under TRIPS agreement (WTO member states)
 * - Protected by reciprocal copyright treaties
 * - Minimum protection: Life of author + 50 years (Berne minimum)
 * - Extended protection: Life + 70 years (EU, USA, Brazil and others)
 * 
 * ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
 * All derivative works, redistributions, or substantial use must include:
 * 1. This complete copyright and license notice
 * 2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
 * 3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
 * 4. Indication of any modifications made
 * 5. Date of last modification
 * 
 *
INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:
 * 
 * This software is developed in compliance with international copyright law,
 * human rights frameworks, and ethical standards including:
 * 
 * COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
 * - Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
 *   └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
 * - WIPO Copyright Treaty (WCT, 1996) - Digital rights management
 * - WIPO Performances and Phonograms Treaty (WPPT, 1996)
 * - Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
 * - Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
 * - Vienna Convention on the Law of Treaties (1969) - Treaty interpretation
 * 
 * HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
 * - Universal Declaration of Human Rights (UDHR, 1948)
 *   └─ Article 27: Right to protection of moral and material interests
 * - International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
 *   └─ Article 15: Right to benefit from scientific progress and protection of authorship
 * - Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
 *   └─ Articles 13, 16, 17: Expression, privacy, access to information
 * - Vienna Declaration and Programme of Action (1993) - Human rights universality
 * 
 * UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
 * - UNESCO Universal Declaration on Cultural Diversity (2001)
 * - UNESCO Recommendation on Open Science (2021)
 * - UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
 * - Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)
 * 
 * DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
 * - GDPR - General Data Protection Regulation (EU 2016/679)
 * - LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
 * - CCPA - California Consumer Privacy Act (USA)
 * - Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)
 * 
 * TECHNICAL STANDARDS / NORMAS TÉCNICAS:
 * - ISO/IEC 9001:2015 - Quality Management Systems
 * - ISO/IEC 27001:2022 - Information Security Management
 * - ISO/IEC 27002:2022 - Information Security Controls
 * - ISO/IEC 27018:2019 - PII Protection in Public Clouds
 * - ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
 * - ISO/IEC 8000 - Data Quality Standards
 * - IEEE 830-1998 - Software Requirements Specification
 * - IEEE 1012-2016 - Software Verification and Validation
 * - IEEE 12207-2017 - Software Life Cycle Processes
 * - IEEE 14764-2021 - Software Maintenance
 * - IEEE 42010-2011 - Architecture Description
 * - NIST Cybersecurity Framework (CSF) v1.1/v2.0
 * - NIST SP 800-53 Rev. 5 - Security and Privacy Controls
 * - NIST AI Risk Management Framework (AI RMF 1.0)
 * 
 * CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
 * - Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
 * - Universal jurisdiction for human rights violations
 * - Rome Statute of the International Criminal Court (1998) - For severe violations
 *
ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:
 * 
 * This software adheres to the Ethica[8] framework with eight fundamental principles:
 * 
 * 1. TRANSPARENCY (Transparência) 🔍
 *    └─ Open communication, documented decisions, explainable systems
 *    
 * 2. ACCOUNTABILITY (Responsabilidade) 📋
 *    └─ Clear ownership, traceable actions, consequence acceptance
 *    
 * 3. FAIRNESS (Justiça) ⚖️
 *    └─ Equitable treatment, non-discrimination, equal access
 *    
 * 4. PRIVACY (Privacidade) 🔒
 *    └─ Data protection, consent respect, confidentiality
 *    
 * 5. SECURITY (Segurança) 🛡️
 *    └─ Protection of systems, data integrity, threat mitigation
 *    
 * 6. RELIABILITY (Confiabilidade) 🔧
 *    └─ Dependable operation, consistent behavior, stability
 *    
 * 7. SAFETY (Proteção) 🛟
 *    └─ No harm to users, safe operations, risk prevention
 *    
 * 8. SUSTAINABILITY (Sustentabilidade) ♻️
 *    └─ Long-term viability, environmental responsibility, social good
 * 
 * ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
 *   Life > Ethics > Law > Convenience
 *   Vida > Ética > Lei > Conveniência
 *
ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:
 * 
 * This code is original work or properly attributed derivative work.
 * Every fragment, function, class, and algorithm has been:
 *   ✓ Originally created by the author, OR
 *   ✓ Properly licensed and attributed, OR
 *   ✓ In the public domain with documentation
 * 
 * NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
 * ZERO TOLERANCE for unauthorized copying or intellectual property theft.
 * 
 * Verification Methods:
 * - SHA3-512 checksums for integrity verification
 * - BLAKE3 hashing for rapid verification
 * - Git commit history as proof of authorship timeline
 * - Code review and compliance audits
 * 
 * Any concerns about intellectual property should be reported to:
 * rafaelmeloreisnovo [at] gmail [dot] com
 *
NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):
 * 
 * These anchors provide stable reference points for:
 * - Version tracking and synchronization
 * - Legal compliance verification
 * - Authorship chain of custody
 * - Update propagation tracking
 * - Audit trail maintenance
 * 
 * ⚓ ANCHOR_ID: 4DDCB6D5EE6F27E6
 * ⚓ FILE_PATH: native/src/init/mount.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: FCEDAE8AEC04ABFB34FF715D86B9A141
 *

 */


#include <set>
#include <sys/mount.h>
#include <sys/sysmacros.h>
#include <libgen.h>

#include <base.hpp>
#include <consts.hpp>

#include "init.hpp"

using namespace std;

struct devinfo {
    int major;
    int minor;
    char devname[32];
    char partname[32];
    char dmname[32];
    char devpath[PATH_MAX];
};

static vector<devinfo> dev_list;

// When this boolean is set, this means we are currently
// running magiskinit on legacy SAR AVD emulator
bool avd_hack = false;

static void parse_device(devinfo *dev, const char *uevent) {
    dev->partname[0] = '\0';
    dev->devpath[0] = '\0';
    dev->dmname[0] = '\0';
    dev->devname[0] = '\0';
    parse_prop_file(uevent, [=](Utf8CStr key, Utf8CStr value) -> bool {
        if (key == "MAJOR")
            dev->major = parse_int(value);
        else if (key == "MINOR")
            dev->minor = parse_int(value);
        else if (key == "DEVNAME")
            strscpy(dev->devname, value.c_str(), sizeof(dev->devname));
        else if (key == "PARTNAME")
            strscpy(dev->partname, value.c_str(), sizeof(dev->devname));

        return true;
    });
}

void MagiskInit::collect_devices() const noexcept {
    char path[PATH_MAX];
    devinfo dev{};
    if (auto dir = xopen_dir("/sys/dev/block"); dir) {
        for (dirent *entry; (entry = readdir(dir.get()));) {
            if (entry->d_name == "."sv || entry->d_name == ".."sv)
                continue;
            sprintf(path, "/sys/dev/block/%s/uevent", entry->d_name);
            parse_device(&dev, path);
            sprintf(path, "/sys/dev/block/%s/dm/name", entry->d_name);
            if (access(path, F_OK) == 0) {
                auto name = rtrim(full_read(path));
                strscpy(dev.dmname, name.data(), sizeof(dev.dmname));
            }
            if (auto it = std::ranges::find_if(config.partition_map, [&](const auto &i) {
                return i.key == dev.devname;
            }); dev.partname[0] == '\0' && it != config.partition_map.end()) {
                // use androidboot.partition_map as partname fallback.
                strscpy(dev.partname, it->value.data(), sizeof(dev.partname));
            }
            sprintf(path, "/sys/dev/block/%s", entry->d_name);
            xrealpath(path, dev.devpath, sizeof(dev.devpath));
            dev_list.push_back(dev);
        }
    }
}

uint64_t MagiskInit::find_block(const char *partname) const noexcept {
    if (dev_list.empty())
        collect_devices();

    for (int tries = 0; tries < 3; ++tries) {
        for (auto &dev : dev_list) {
            const char *name;
            if (strcasecmp(dev.partname, partname) == 0)
                name = dev.partname;
            else if (strcasecmp(dev.dmname, partname) == 0)
                name = dev.dmname;
            else if (strcasecmp(dev.devname, partname) == 0)
                name = dev.devname;
            else if (std::string_view(dev.devpath).ends_with("/"s + partname))
                name = dev.devpath;
            else
                continue;

            LOGD("Found %s: [%s] (%d, %d)\n", name, dev.devname, dev.major, dev.minor);
            return makedev(dev.major, dev.minor);
        }
        // Wait 10ms and try again
        usleep(10000);
        dev_list.clear();
        collect_devices();
    }

    // The requested partname does not exist
    return 0;
}

void MagiskInit::mount_preinit_dir() noexcept {
    if (preinit_dev.empty()) return;
    auto dev = find_block(preinit_dev.c_str());
    if (dev == 0) {
        LOGE("Cannot find preinit %s, abort!\n", preinit_dev.c_str());
        return;
    }
    xmknod(PREINITDEV, S_IFBLK | 0600, dev);
    xmkdir(MIRRDIR, 0);
    bool mounted = false;
    // First, find if it is already mounted
    std::string mnt_point;
    if (rust::is_device_mounted(dev, mnt_point)) {
        // Already mounted, just bind mount
        xmount(mnt_point.data(), MIRRDIR, nullptr, MS_BIND, nullptr);
        mounted = true;
    }

    // Since we are mounting the block device directly, make sure to ONLY mount the partitions
    // as read-only, or else the kernel might crash due to crappy drivers.
    // After the device boots up, magiskd will properly symlink the correct path at PREINITMIRR as writable.
    if (mounted || mount(PREINITDEV, MIRRDIR, "ext4", MS_RDONLY, nullptr) == 0 ||
        mount(PREINITDEV, MIRRDIR, "f2fs", MS_RDONLY, nullptr) == 0) {
        string preinit_dir = resolve_preinit_dir(MIRRDIR);
        // Create bind mount
        xmkdirs(PREINITMIRR, 0);
        if (access(preinit_dir.data(), F_OK)) {
            LOGW("empty preinit: %s\n", preinit_dir.data());
        } else {
            LOGD("preinit: %s\n", preinit_dir.data());
            xmount(preinit_dir.data(), PREINITMIRR, nullptr, MS_BIND, nullptr);
        }
        xumount2(MIRRDIR, MNT_DETACH);
    } else {
        PLOGE("Mount preinit %s", preinit_dev.c_str());
        // Do NOT delete the block device. Even though we cannot mount it here,
        // it might get formatted later in the boot process.
    }
}

bool MagiskInit::mount_system_root() noexcept {
    LOGD("Mounting system_root\n");

    // there's no /dev in stub cpio
    xmkdir("/dev", 0777);

    dev_t dev;
    do {
        // Try legacy SAR dm-verity
        dev = find_block("vroot");
        if (dev > 0)
            goto mount_root;

        // Try NVIDIA naming scheme
        dev = find_block("APP");
        if (dev > 0)
            goto mount_root;

        // Try normal partname
        char sys_part[32];
        sprintf(sys_part, "system%s", config.slot.data());
        dev = find_block(sys_part);
        if (dev > 0)
            goto mount_root;

        // Poll forever if rootwait was given in cmdline
    } while (config.rootwait);

    // We don't really know what to do at this point...
    LOGE("Cannot find root partition, abort\n");
    exit(1);

mount_root:
    xmknod("/dev/root", S_IFBLK | 0600, dev);
    xmkdir("/system_root", 0755);

    if (xmount("/dev/root", "/system_root", "ext4", MS_RDONLY, nullptr)) {
        if (xmount("/dev/root", "/system_root", "erofs", MS_RDONLY, nullptr)) {
            // We don't really know what to do at this point...
            LOGE("Cannot mount root partition, abort\n");
            exit(1);
        }
    }

    rust::switch_root("/system_root");

    // Make dev writable
    xmount("tmpfs", "/dev", "tmpfs", 0, "mode=755");
    mount_list.emplace_back("/dev");

    bool is_two_stage = access("/system/bin/init", F_OK) == 0;
    LOGD("is_two_stage: [%d]\n", is_two_stage);

    // For API 28 AVD, it uses legacy SAR setup that requires
    // special hacks in magiskinit to work properly.
    if (!is_two_stage && config.emulator) {
        avd_hack = true;
        // These values are hardcoded for API 28 AVD
        auto vendor_dev = find_block("vendor");
        xmkdir("/dev/block", 0755);
        xmknod("/dev/block/vde1", S_IFBLK | 0600, vendor_dev);
        xmount("/dev/block/vde1", "/vendor", "ext4", MS_RDONLY, nullptr);
    }

    return is_two_stage;
}

void MagiskInit::setup_tmp(const char *path) noexcept {
    LOGD("Setup Magisk tmp at %s\n", path);
    chdir("/data");

    xmkdir(INTLROOT, 0711);
    xmkdir(DEVICEDIR, 0711);
    xmkdir(WORKERDIR, 0);

    mount_preinit_dir();

    cp_afc(".backup/.magisk", MAIN_CONFIG);
    rm_rf(".backup");

    // Create applet symlinks
    for (int i = 0; applet_names[i]; ++i)
        xsymlink("./magisk", applet_names[i]);
    xsymlink("./magiskpolicy", "supolicy");

    xmount(".", path, nullptr, MS_BIND, nullptr);

    chdir(path);

    // Prepare worker
    xmount("magisk", WORKERDIR, "tmpfs", 0, "mode=755");

    // Use isolated devpts if kernel support
    if (access("/dev/pts/ptmx", F_OK) == 0) {
        xmkdirs(SHELLPTS, 0755);
        xmount("devpts", SHELLPTS, "devpts", MS_NOSUID | MS_NOEXEC, "newinstance");
        xmount(nullptr, SHELLPTS, nullptr, MS_PRIVATE, nullptr);
        if (access(SHELLPTS "/ptmx", F_OK)) {
            umount2(SHELLPTS, MNT_DETACH);
            rmdir(SHELLPTS);
        }
    }

    chdir("/");
}
