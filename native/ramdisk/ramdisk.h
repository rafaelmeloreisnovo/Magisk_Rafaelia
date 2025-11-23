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

#ifndef RAFAELIA_RAMDISK_H
#define RAFAELIA_RAMDISK_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* compression types detected by magic */
typedef enum {
    RD_COMP_NONE = 0,
    RD_COMP_GZIP,
    RD_COMP_XZ,
    RD_COMP_LZ4,
    RD_COMP_ZSTD,
    RD_COMP_UNKNOWN
} rd_comp_t;

/* feed callback - called for each chunk of decompressed data (only used for streaming APIs) */
typedef int (*rd_feed_cb_t)(const unsigned char *buf, size_t len, void *userdata);

/* detect compression type by magic bytes in buffer (len should be >= 4) */
rd_comp_t detect_compression_magic(const uint8_t *buf, size_t len);

/*
 * Decompress the compressed stream starting at current fd position into a temporary file.
 * The returned out_fd is a seekable fd positioned at beginning (caller must close it).
 * max_uncompressed: 0 = unlimited; otherwise cap on decompressed bytes.
 *
 * Returns 0 on success, negative on error.
 */
int decompress_fd_to_tempfile(int fd, uint64_t max_uncompressed, int *out_fd);

/*
 * High-level helper:
 *  - from an fd positioned at compressed ramdisk start, decompress to tempfile (with limit)
 *  - then parse the decompressed tempfile via cpio_parse_fd (from native/cpio)
 *
 * feed_cb is forwarded to cpio_parse_fd consumer expectation (the cpio parser will call it per file payload).
 *
 * Returns 0 on success, negative otherwise.
 */
int ramdisk_extract_and_parse_fd(int fd, uint64_t max_uncompressed, rd_feed_cb_t feed_cb, void *userdata);

/* utility: compute sha256 of a file descriptor content (seek restored) */
int rd_sha256_fd(int fd, unsigned char out[32]);

#ifdef __cplusplus
}
#endif

#endif /* RAFAELIA_RAMDISK_H */
