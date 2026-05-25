#include "blocked_filter.h"

#include <cmath>
#include <cstring>

#ifdef __aarch64__
#include <arm_neon.h>
#endif

static const uint64_t BLOCK_WORDS = 8;
static const uint64_t BLOCK_BITS = BLOCK_WORDS * 64; // 512

struct BlockedFilter {
    uint64_t k;
    uint64_t num_blocks;
    uint64_t block_mask;
    uint64_t* array;

    ~BlockedFilter() { delete[] array; }
};

// Mirror of Rust's BlockedFilter::probe_masks.
static void compute_probe_masks(uint64_t h1, uint64_t h2, uint64_t k, uint64_t masks[8]) {
    uint64_t step = (h1 * UINT64_C(0x9E3779B97F4A7C15)) | UINT64_C(1);
    std::memset(masks, 0, 8 * sizeof(uint64_t));
    for (uint64_t i = 0; i < k; i++) {
        uint64_t bit = (h2 + i * step) & (BLOCK_BITS - 1);
        masks[bit >> 6] |= UINT64_C(1) << (bit & 63);
    }
}

#ifdef __aarch64__
static bool neon_contains(const uint64_t* block, const uint64_t masks[8]) {
    uint64x2_t b0 = vld1q_u64(block);
    uint64x2_t b1 = vld1q_u64(block + 2);
    uint64x2_t b2 = vld1q_u64(block + 4);
    uint64x2_t b3 = vld1q_u64(block + 6);
    uint64x2_t m0 = vld1q_u64(masks);
    uint64x2_t m1 = vld1q_u64(masks + 2);
    uint64x2_t m2 = vld1q_u64(masks + 4);
    uint64x2_t m3 = vld1q_u64(masks + 6);
    uint64x2_t missing = vorrq_u64(
        vorrq_u64(vbicq_u64(m0, b0), vbicq_u64(m1, b1)),
        vorrq_u64(vbicq_u64(m2, b2), vbicq_u64(m3, b3)));
    return (vgetq_lane_u64(missing, 0) | vgetq_lane_u64(missing, 1)) == 0;
}

static void neon_insert(uint64_t* block, const uint64_t masks[8]) {
    uint64x2_t m0 = vld1q_u64(masks);
    uint64x2_t m1 = vld1q_u64(masks + 2);
    uint64x2_t m2 = vld1q_u64(masks + 4);
    uint64x2_t m3 = vld1q_u64(masks + 6);
    vst1q_u64(block,     vorrq_u64(vld1q_u64(block),     m0));
    vst1q_u64(block + 2, vorrq_u64(vld1q_u64(block + 2), m1));
    vst1q_u64(block + 4, vorrq_u64(vld1q_u64(block + 4), m2));
    vst1q_u64(block + 6, vorrq_u64(vld1q_u64(block + 6), m3));
}
#endif // __aarch64__

extern "C" {

void* cpp_bf_create(uint64_t bits, uint64_t n) {
    if (bits == 0 || n == 0) return nullptr;

    uint64_t num_blocks = (bits + BLOCK_BITS - 1) / BLOCK_BITS;
    if (num_blocks == 0) num_blocks = 1;

    uint64_t total_words = num_blocks * BLOCK_WORDS;
    uint64_t block_mask = ((num_blocks & (num_blocks - 1)) == 0) ? (num_blocks - 1) : 0;

    double m = static_cast<double>(num_blocks * BLOCK_BITS);
    uint64_t k = static_cast<uint64_t>(std::round((m / static_cast<double>(n)) * 0.6931471805599453));
    if (k < 1) k = 1;
    if (k > 30) k = 30;

    BlockedFilter* f = new BlockedFilter;
    f->k = k;
    f->num_blocks = num_blocks;
    f->block_mask = block_mask;
    f->array = new uint64_t[total_words]();
    return f;
}

void cpp_bf_destroy(void* filter) {
    delete static_cast<BlockedFilter*>(filter);
}

void cpp_bf_insert_hashed(void* filter, uint64_t h1, uint64_t h2) {
    BlockedFilter* f = static_cast<BlockedFilter*>(filter);
    uint64_t block_idx =
        (f->block_mask != 0 ? (h1 & f->block_mask) : (h1 % f->num_blocks)) * BLOCK_WORDS;
    uint64_t masks[8];
    compute_probe_masks(h1, h2, f->k, masks);
    uint64_t* block = f->array + block_idx;
#ifdef __aarch64__
    neon_insert(block, masks);
#else
    for (int j = 0; j < 8; j++) block[j] |= masks[j];
#endif
}

bool cpp_bf_contains_hashed(const void* filter, uint64_t h1, uint64_t h2) {
    const BlockedFilter* f = static_cast<const BlockedFilter*>(filter);
    uint64_t block_idx =
        (f->block_mask != 0 ? (h1 & f->block_mask) : (h1 % f->num_blocks)) * BLOCK_WORDS;
    uint64_t masks[8];
    compute_probe_masks(h1, h2, f->k, masks);
    const uint64_t* block = f->array + block_idx;
#ifdef __aarch64__
    return neon_contains(block, masks);
#else
    for (int j = 0; j < 8; j++) {
        if ((block[j] & masks[j]) != masks[j]) return false;
    }
    return true;
#endif
}

} // extern "C"
