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

#ifndef RAFAELIA_ECC_BUFFER_H
#define RAFAELIA_ECC_BUFFER_H

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ═══════════════════════════════════════════════════════════════════════════
 * TYPE DEFINITIONS
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * ECC status codes
 */
typedef enum {
    ECC_OK = 0,              /* No errors detected */
    ECC_CORRECTED = 1,       /* Single error corrected */
    ECC_DETECTED = 2,        /* Multiple errors detected (uncorrectable) */
    ECC_ERROR = -1           /* ECC computation error */
} ecc_status_t;

/**
 * ECC buffer structure (opaque)
 */
typedef struct ecc_buffer_t ecc_buffer_t;

/**
 * Statistics for ECC operations
 */
typedef struct {
    uint64_t total_checks;       /* Total ECC checks performed */
    uint64_t errors_corrected;   /* Single-bit errors corrected */
    uint64_t errors_detected;    /* Multi-bit errors detected */
    uint64_t checksum_failures;  /* Checksum validation failures */
} ecc_stats_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * PUBLIC API
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * Create ECC-protected buffer.
 *
 * @param size Size of data buffer in bytes
 * @return Pointer to ECC buffer structure, or NULL on error
 */
ecc_buffer_t* ecc_buffer_create(size_t size);

/**
 * Destroy ECC buffer and free resources.
 *
 * @param buf Buffer to destroy
 */
void ecc_buffer_destroy(ecc_buffer_t *buf);

/**
 * Encode data with ECC protection.
 *
 * @param buf ECC buffer
 * @param data Data to encode
 * @param size Size of data
 * @return ECC_OK on success, ECC_ERROR on failure
 */
ecc_status_t ecc_buffer_encode(ecc_buffer_t *buf, const uint8_t *data, size_t size);

/**
 * Decode and verify ECC-protected data.
 *
 * @param buf ECC buffer
 * @return ECC status code
 */
ecc_status_t ecc_buffer_decode(ecc_buffer_t *buf);

/**
 * Get pointer to data in ECC buffer.
 *
 * @param buf ECC buffer
 * @return Pointer to data, or NULL on error
 */
uint8_t* ecc_buffer_get_data(ecc_buffer_t *buf);

/**
 * Get size of data in ECC buffer.
 *
 * @param buf ECC buffer
 * @return Size in bytes, or 0 on error
 */
size_t ecc_buffer_get_size(ecc_buffer_t *buf);

/**
 * Get ECC statistics.
 *
 * @return Pointer to global statistics structure
 */
const ecc_stats_t* ecc_get_stats(void);

/**
 * Reset ECC statistics.
 */
void ecc_reset_stats(void);

/**
 * Calculate memory overhead for ECC.
 *
 * @param data_size Size of data in bytes
 * @return Overhead percentage
 */
double ecc_buffer_overhead(size_t data_size);

#ifdef __cplusplus
}
#endif

#endif /* RAFAELIA_ECC_BUFFER_H */
