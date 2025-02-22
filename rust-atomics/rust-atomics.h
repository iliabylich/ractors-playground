#ifndef RUST_ATOMICS_H
#define RUST_ATOMICS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define PLAIN_COUNTER_SIZE 8

#define ATOMIC_COUNTER_SIZE 8

#define CONCURRENT_HASH_MAP_SIZE 40

#define FIXED_SIZE_OBJECT_POOL_SIZE 48

typedef struct atomic_counter_t atomic_counter_t;

typedef struct concurrent_hash_map_t concurrent_hash_map_t;

typedef struct fixed_size_object_pool_t fixed_size_object_pool_t;

typedef struct plain_counter_t plain_counter_t;

void plain_counter_init(plain_counter_t *counter, uint64_t n);

void plain_counter_increment(plain_counter_t *counter);

uint64_t plain_counter_read(const plain_counter_t *counter);

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

void fixed_size_object_pool_alloc(fixed_size_object_pool_t *pool);

void fixed_size_object_pool_init(fixed_size_object_pool_t *pool,
                                 uintptr_t max_size,
                                 uint64_t timeout_in_ms,
                                 unsigned long (*rb_make_obj)(unsigned long));

void fixed_size_object_pool_drop(fixed_size_object_pool_t *pool);

void fixed_size_object_pool_mark(const fixed_size_object_pool_t *pool, void (*f)(unsigned long));

unsigned long fixed_size_object_pool_pop(fixed_size_object_pool_t *pool, unsigned long fallback);

void fixed_size_object_pool_push(fixed_size_object_pool_t *pool, unsigned long value);

#endif  /* RUST_ATOMICS_H */
