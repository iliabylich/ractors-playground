#ifndef RUST_ATOMICS_H
#define RUST_ATOMICS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define ATOMIC_COUNTER_SIZE 8

#define CONCURRENT_HASH_MAP_SIZE 40

#define CONCURRENT_HASH_MAP_ALIGN 8

typedef struct atomic_counter_t atomic_counter_t;

typedef struct concurrent_hash_map_t concurrent_hash_map_t;

void atomic_counter_init(atomic_counter_t *counter, uint64_t n);

void atomic_counter_increment(const atomic_counter_t *counter);

uint64_t atomic_counter_read(const atomic_counter_t *counter);

void concurrent_hash_map_init(concurrent_hash_map_t *hashmap);

void concurrent_hash_map_drop(concurrent_hash_map_t *hashmap);

void concurrent_hash_map_clear(const concurrent_hash_map_t *hashmap);

unsigned long concurrent_hash_map_get(const concurrent_hash_map_t *hashmap,
                                      unsigned long key,
                                      unsigned long fallback);

void concurrent_hash_map_set(const concurrent_hash_map_t *hashmap,
                             unsigned long key,
                             unsigned long value);

void concurrent_hash_map_mark(const concurrent_hash_map_t *hashmap, void (*f)(unsigned long));

void concurrent_hash_map_fetch_and_modify(const concurrent_hash_map_t *hashmap,
                                          unsigned long key,
                                          unsigned long (*f)(unsigned long));

#endif  /* RUST_ATOMICS_H */
