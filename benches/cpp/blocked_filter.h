#pragma once
#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void* cpp_bf_create(uint64_t bits, uint64_t n);
void cpp_bf_destroy(void* filter);
void cpp_bf_insert_hashed(void* filter, uint64_t h1, uint64_t h2);
bool cpp_bf_contains_hashed(const void* filter, uint64_t h1, uint64_t h2);

#ifdef __cplusplus
}
#endif
