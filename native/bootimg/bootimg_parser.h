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

#ifndef RAFAELIA_BOOTIMG_PARSER_H
#define RAFAELIA_BOOTIMG_PARSER_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    RAF_BOOT_UNKNOWN = 0,
    RAF_BOOT_V1,
    RAF_BOOT_VENDOR, /* vendor_boot present */
    RAF_BOOT_V3,     /* heurística: newer header layout */
    RAF_BOOT_V4
} raf_boot_version_t;

struct boot_image_info {
    raf_boot_version_t version;
    uint32_t page_size;        /* page size used for alignment */
    size_t header_size;        /* bytes consumed by header region (page aligned) */

    /* kernel */
    size_t kernel_offset;
    size_t kernel_size;

    /* ramdisk */
    size_t ramdisk_offset;
    size_t ramdisk_size;

    /* second (optional) */
    size_t second_offset;
    size_t second_size;

    /* device tree / dtbo */
    size_t dtb_offset;
    size_t dtb_size;

    /* vendor_boot (if present) */
    int has_vendor_boot;
    size_t vendor_boot_offset;
    size_t vendor_boot_size;

    /* raw pointers into original buffer (optional convenience) */
    const uint8_t *raw_buf;
    size_t raw_buf_size;
};

/*
 * Parse the provided buffer (in_buf/in_sz) and fill boot_image_info.
 * This function does NOT allocate for payloads; it only computes offsets/sizes.
 *
 * Returns:
 *   0 on success (boot_image_info filled)
 *  -1 invalid args
 *  -2 not a recognizable boot image
 *  -3 buffer too small/incomplete
 */
int parse_boot_image(const uint8_t *in_buf, size_t in_sz, struct boot_image_info *out);

#ifdef __cplusplus
}
#endif

#endif /* RAFAELIA_BOOTIMG_PARSER_H */
