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

struct boot_img_hdr_v1 {
    char magic[BOOT_MAGIC_SIZE];   /* "ANDROID!" */
    uint32_t kernel_size;          /* size in bytes */
    uint32_t kernel_addr;          /* physical load addr */
    uint32_t ramdisk_size;         /* size in bytes */
    uint32_t ramdisk_addr;
    uint32_t second_size;
    uint32_t second_addr;
    uint32_t tags_addr;
    uint32_t page_size;
    uint32_t dt_size;
    uint32_t unused;               /* reserved / future */
    char name[16];
    char cmdline[512];
} __attribute__((packed));

/* Generic constants */
#define DEFAULT_PAGE_SIZE 2048U
#define BOOT_HEADER_MAX_SIZE (64 * 1024) /* cap for safety */

/* Align helper */
static inline size_t align_to(size_t off, size_t page) {
    if (page == 0) return off;
    return ((off + page - 1) / page) * page;
}

#endif /* RAFAELIA_BOOTIMG_H */
