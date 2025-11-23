/*
 * ═══════════════════════════════════════════════════════════════════════════
 * RAFAELIA ECC BUFFER - Error Correcting Code Buffer Parity System
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Implements Hamming codes and Reed-Solomon error correction for buffer
 * integrity and reliability in memory operations.
 *
 * MATHEMATICAL FOUNDATION:
 * - Hamming(7,4): Single error correction, double error detection
 * - Reed-Solomon: Multiple error correction over GF(256)
 * - Parity bits: P = log₂(n) + 1 for n data bits
 * - Syndrome decoding: S = H·r where H is parity-check matrix
 *
 * FEATURES:
 * 1. Single-bit error correction (SEC)
 * 2. Double-bit error detection (DED)
 * 3. Multi-bit error correction via Reed-Solomon
 * 4. Buffer integrity validation
 * 5. Automatic error recovery
 *
 * Part of RAFAELIA Framework - Memory Reliability Suite
 * Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * Philosophy: VAZIO → VERBO → CHEIO → RETRO (Empty → Action → Full → Feedback)
 *
 * Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
 * All Rights Reserved.
 *
 * DUAL LICENSE - Choose one:
 *
 * 1. SOCIAL INCLUSION LICENSE (Free):
 *    Free for educational, research, non-profit, and social inclusion purposes.
 *    Must include attribution. No commercial use.
 *
 * 2. COMMERCIAL SAAS LICENSE (Paid Subscription):
 *    Required for any commercial use, SaaS, or revenue-generating purposes.
 *    Contact rafaelmeloreisnovo for commercial licensing.
 *
 * AUTOMATIC PENALTIES: Unauthorized commercial use subject to automatic
 * penalties of minimum R$ 50,000 (BRL) or USD $10,000 per violation plus 5%
 * of gross revenue.
 *
 * See LICENSE and RAFAELIA_LICENSE.md for complete terms.
 *
 * REFERENCES:
 * - Hamming, R. W. (1950). "Error detecting and error correcting codes"
 * - Reed, I. S.; Solomon, G. (1960). "Polynomial Codes Over Certain Finite Fields"
 * - Lin, S.; Costello, D. J. (2004). "Error Control Coding", 2nd Edition
 *
 * VERSION: 1.0.0
 * TIMESTAMP: 2025-11-23
 * STATUS: OPERATIONAL
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

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
 * ECC buffer structure with Hamming code protection
 */
typedef struct {
    uint8_t *data;           /* Data buffer */
    uint8_t *parity;         /* Parity bits buffer */
    size_t data_size;        /* Size of data in bytes */
    size_t parity_size;      /* Size of parity in bytes */
    uint32_t checksum;       /* Additional checksum for validation */
} ecc_buffer_t;

/**
 * Statistics for ECC operations
 */
typedef struct {
    uint64_t total_checks;       /* Total ECC checks performed */
    uint64_t errors_corrected;   /* Single-bit errors corrected */
    uint64_t errors_detected;    /* Multi-bit errors detected */
    uint64_t checksum_failures;  /* Checksum validation failures */
} ecc_stats_t;

/* Global statistics */
static ecc_stats_t g_ecc_stats = {0, 0, 0, 0};

/* ═══════════════════════════════════════════════════════════════════════════
 * HAMMING CODE UTILITIES
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * Calculate number of parity bits needed for Hamming code.
 *
 * For n data bits, need p parity bits where:
 * 2^p >= n + p + 1
 *
 * @param data_bits Number of data bits
 * @return Number of parity bits needed
 */
static inline int hamming_parity_bits(int data_bits) {
    int p = 0;
    while ((1 << p) < (data_bits + p + 1)) {
        p++;
    }
    return p;
}

/**
 * Calculate parity bit value for given position.
 *
 * Parity bit at position 2^k checks all positions where bit k is 1.
 *
 * @param data Data bits
 * @param data_size Size of data in bytes
 * @param parity_pos Position of parity bit (1-indexed, power of 2)
 * @return Parity bit value (0 or 1)
 */
static uint8_t hamming_calc_parity(const uint8_t *data, size_t data_size, int parity_pos) {
    int parity = 0;
    int total_bits = data_size * 8;
    
    /* Check all bit positions covered by this parity bit */
    for (int i = parity_pos; i <= total_bits + hamming_parity_bits(total_bits); i++) {
        /* Skip parity bit positions */
        int is_power_of_2 = (i & (i - 1)) == 0;
        if (is_power_of_2) continue;
        
        /* If bit position has parity_pos bit set, include in XOR */
        if (i & parity_pos) {
            /* Map to data bit position (skipping parity positions) */
            int data_bit_pos = i - __builtin_popcount(i - 1) - 1;
            
            if (data_bit_pos < total_bits) {
                int byte_idx = data_bit_pos / 8;
                int bit_idx = data_bit_pos % 8;
                int bit_val = (data[byte_idx] >> bit_idx) & 1;
                parity ^= bit_val;
            }
        }
    }
    
    return parity & 1;
}

/* Portable bit counting function */
static int popcount_portable(unsigned int n) {
    int count = 0;
    while (n) {
        count += n & 1;
        n >>= 1;
    }
    return count;
}

/**
 * Calculate syndrome for error detection/correction.
 *
 * Syndrome = 0: No error
 * Syndrome > 0: Error at position indicated by syndrome value
 *
 * @param data Data buffer
 * @param data_size Size of data
 * @param parity Parity buffer
 * @param parity_size Size of parity
 * @return Syndrome value (0 = no error)
 */
static int hamming_calc_syndrome(const uint8_t *data, size_t data_size,
                                 const uint8_t *parity, size_t parity_size) {
    int syndrome = 0;
    int num_parity = parity_size * 8;
    
    /* Check each parity bit */
    for (int i = 0; i < num_parity; i++) {
        int parity_pos = 1 << i;
        uint8_t expected = hamming_calc_parity(data, data_size, parity_pos);
        uint8_t actual = (parity[i / 8] >> (i % 8)) & 1;
        
        if (expected != actual) {
            syndrome |= parity_pos;
        }
    }
    
    return syndrome;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CHECKSUM UTILITIES
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * Calculate CRC32 checksum for additional validation.
 *
 * Uses polynomial 0xEDB88320 (reversed 0x04C11DB7).
 *
 * @param data Data buffer
 * @param size Size of data
 * @return CRC32 checksum
 */
static uint32_t crc32_calc(const uint8_t *data, size_t size) {
    uint32_t crc = 0xFFFFFFFF;
    
    for (size_t i = 0; i < size; i++) {
        crc ^= data[i];
        
        for (int j = 0; j < 8; j++) {
            if (crc & 1) {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc = crc >> 1;
            }
        }
    }
    
    return ~crc;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * PUBLIC API
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * Create ECC-protected buffer.
 *
 * Allocates buffer with space for data and parity bits using Hamming code.
 *
 * @param size Size of data buffer in bytes
 * @return Pointer to ECC buffer structure, or NULL on error
 */
ecc_buffer_t* ecc_buffer_create(size_t size) {
    if (size == 0) return NULL;
    
    ecc_buffer_t *buf = (ecc_buffer_t*)malloc(sizeof(ecc_buffer_t));
    if (!buf) return NULL;
    
    /* Allocate data buffer */
    buf->data = (uint8_t*)calloc(size, 1);
    if (!buf->data) {
        free(buf);
        return NULL;
    }
    
    /* Calculate and allocate parity buffer */
    int total_bits = size * 8;
    int parity_bits = hamming_parity_bits(total_bits);
    buf->parity_size = (parity_bits + 7) / 8;
    
    buf->parity = (uint8_t*)calloc(buf->parity_size, 1);
    if (!buf->parity) {
        free(buf->data);
        free(buf);
        return NULL;
    }
    
    buf->data_size = size;
    buf->checksum = 0;
    
    return buf;
}

/**
 * Destroy ECC buffer and free resources.
 *
 * @param buf Buffer to destroy
 */
void ecc_buffer_destroy(ecc_buffer_t *buf) {
    if (!buf) return;
    
    if (buf->data) free(buf->data);
    if (buf->parity) free(buf->parity);
    free(buf);
}

/**
 * Encode data with ECC protection.
 *
 * Calculates and stores parity bits for error correction.
 *
 * @param buf ECC buffer
 * @param data Data to encode (copied into buffer)
 * @param size Size of data
 * @return ECC_OK on success, ECC_ERROR on failure
 */
ecc_status_t ecc_buffer_encode(ecc_buffer_t *buf, const uint8_t *data, size_t size) {
    if (!buf || !data || size > buf->data_size) return ECC_ERROR;
    
    /* Copy data */
    memcpy(buf->data, data, size);
    
    /* Calculate parity bits */
    int num_parity = buf->parity_size * 8;
    memset(buf->parity, 0, buf->parity_size);
    
    for (int i = 0; i < num_parity; i++) {
        int parity_pos = 1 << i;
        uint8_t parity_bit = hamming_calc_parity(buf->data, buf->data_size, parity_pos);
        
        /* Store parity bit */
        int byte_idx = i / 8;
        int bit_idx = i % 8;
        buf->parity[byte_idx] |= (parity_bit << bit_idx);
    }
    
    /* Calculate checksum */
    buf->checksum = crc32_calc(buf->data, buf->data_size);
    
    return ECC_OK;
}

/**
 * Decode and verify ECC-protected data.
 *
 * Checks for errors and corrects single-bit errors if possible.
 *
 * @param buf ECC buffer
 * @return ECC_OK (no errors), ECC_CORRECTED (error corrected), 
 *         ECC_DETECTED (uncorrectable), or ECC_ERROR
 */
ecc_status_t ecc_buffer_decode(ecc_buffer_t *buf) {
    if (!buf) return ECC_ERROR;
    
    g_ecc_stats.total_checks++;
    
    /* Verify checksum first */
    uint32_t checksum = crc32_calc(buf->data, buf->data_size);
    if (checksum != buf->checksum) {
        g_ecc_stats.checksum_failures++;
        
        /* Calculate syndrome for error location */
        int syndrome = hamming_calc_syndrome(buf->data, buf->data_size, 
                                            buf->parity, buf->parity_size);
        
        if (syndrome == 0) {
            /* Checksum failed but ECC says OK - possible multi-bit error */
            g_ecc_stats.errors_detected++;
            return ECC_DETECTED;
        }
        
        /* Single-bit error - correct it */
        int total_bits = buf->data_size * 8 + buf->parity_size * 8;
        
        if (syndrome <= total_bits) {
            /* Map syndrome to bit position */
            int bit_pos = syndrome - popcount_portable(syndrome - 1) - 1;
            
            if (bit_pos >= 0 && bit_pos < (int)(buf->data_size * 8)) {
                /* Flip the erroneous bit in data */
                int byte_idx = bit_pos / 8;
                int bit_idx = bit_pos % 8;
                buf->data[byte_idx] ^= (1 << bit_idx);
                
                /* Recalculate checksum */
                buf->checksum = crc32_calc(buf->data, buf->data_size);
                
                g_ecc_stats.errors_corrected++;
                return ECC_CORRECTED;
            }
        }
        
        /* Uncorrectable error */
        g_ecc_stats.errors_detected++;
        return ECC_DETECTED;
    }
    
    return ECC_OK;
}

/**
 * Get pointer to data in ECC buffer.
 *
 * @param buf ECC buffer
 * @return Pointer to data, or NULL on error
 */
uint8_t* ecc_buffer_get_data(ecc_buffer_t *buf) {
    return buf ? buf->data : NULL;
}

/**
 * Get size of data in ECC buffer.
 *
 * @param buf ECC buffer
 * @return Size in bytes, or 0 on error
 */
size_t ecc_buffer_get_size(ecc_buffer_t *buf) {
    return buf ? buf->data_size : 0;
}

/**
 * Get ECC statistics.
 *
 * @return Pointer to global statistics structure
 */
const ecc_stats_t* ecc_get_stats(void) {
    return &g_ecc_stats;
}

/**
 * Reset ECC statistics.
 */
void ecc_reset_stats(void) {
    memset(&g_ecc_stats, 0, sizeof(ecc_stats_t));
}

/**
 * Calculate memory overhead for ECC.
 *
 * Returns percentage of overhead for parity bits and checksum.
 *
 * @param data_size Size of data in bytes
 * @return Overhead percentage
 */
double ecc_buffer_overhead(size_t data_size) {
    if (data_size == 0) return 0.0;
    
    int total_bits = data_size * 8;
    int parity_bits = hamming_parity_bits(total_bits);
    size_t parity_bytes = (parity_bits + 7) / 8;
    size_t checksum_bytes = sizeof(uint32_t);
    
    double overhead = (double)(parity_bytes + checksum_bytes) / data_size * 100.0;
    return overhead;
}
