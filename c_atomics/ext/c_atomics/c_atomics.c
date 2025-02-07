#include "ruby.h"
#include "rust-atomics.h"

const rb_data_type_t atomic_counter_data = {
    .function = {.dfree = RUBY_DEFAULT_FREE},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

VALUE rb_atomic_counter_alloc(VALUE klass) {
  atomic_counter_t *rust_obj;
  TypedData_Make_Struct0(obj, klass, atomic_counter_t, ATOMIC_COUNTER_SIZE,
                         &atomic_counter_data, rust_obj);
  atomic_counter_init(rust_obj, 0);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_atomic_counter_increment(VALUE self) {
  atomic_counter_t *rust_obj;
  TypedData_Get_Struct(self, atomic_counter_t, &atomic_counter_data, rust_obj);
  atomic_counter_increment(rust_obj);
  return Qnil;
}

VALUE rb_atomic_counter_read(VALUE self) {
  atomic_counter_t *rust_obj;
  TypedData_Get_Struct(self, atomic_counter_t, &atomic_counter_data, rust_obj);
  return LONG2FIX(atomic_counter_read(rust_obj));
}

//

void rb_concurrent_hash_map_mark(void *);
void rb_concurrent_hash_map_free(void *);

const rb_data_type_t concurrent_hash_map_data = {
    .function = {.dfree = rb_concurrent_hash_map_free,
                 .dmark = rb_concurrent_hash_map_mark},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

void rb_concurrent_hash_map_free(void *ptr) {
  concurrent_hash_map_t *hashmap = ptr;
  concurrent_hash_map_drop(hashmap);
}

void rb_concurrent_hash_map_mark(void *ptr) {
  concurrent_hash_map_t *hashmap = ptr;
  concurrent_hash_map_mark(hashmap, rb_gc_mark);
}

VALUE rb_concurrent_hash_map_alloc(VALUE klass) {
  concurrent_hash_map_t *rust_obj;
  TypedData_Make_Struct0(obj, klass, concurrent_hash_map_t,
                         CONCURRENT_HASH_MAP_SIZE, &concurrent_hash_map_data,
                         rust_obj);
  concurrent_hash_map_init(rust_obj);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_concurrent_hash_map_get(VALUE self, VALUE key) {
  concurrent_hash_map_t *rust_obj;
  TypedData_Get_Struct(self, concurrent_hash_map_t, &concurrent_hash_map_data,
                       rust_obj);
  return concurrent_hash_map_get(rust_obj, key, Qnil);
}

VALUE rb_concurrent_hash_map_set(VALUE self, VALUE key, VALUE value) {
  concurrent_hash_map_t *rust_obj;
  TypedData_Get_Struct(self, concurrent_hash_map_t, &concurrent_hash_map_data,
                       rust_obj);
  concurrent_hash_map_set(rust_obj, key, value);
  return Qnil;
}

VALUE rb_concurrent_hash_map_clear(VALUE self) {
  concurrent_hash_map_t *rust_obj;
  TypedData_Get_Struct(self, concurrent_hash_map_t, &concurrent_hash_map_data,
                       rust_obj);
  concurrent_hash_map_clear(rust_obj);
  return Qnil;
}

VALUE rb_concurrent_hash_map_fetch_and_modify(VALUE self, VALUE key) {
  rb_need_block();
  concurrent_hash_map_t *rust_obj;
  TypedData_Get_Struct(self, concurrent_hash_map_t, &concurrent_hash_map_data,
                       rust_obj);
  concurrent_hash_map_fetch_and_modify(rust_obj, key, rb_yield);
  return Qnil;
}

//

RUBY_FUNC_EXPORTED void Init_c_atomics(void) {
  rb_ext_ractor_safe(true);

  VALUE rb_cAtomicCounter = rb_define_class("AtomicCounter", rb_cObject);
  rb_define_alloc_func(rb_cAtomicCounter, rb_atomic_counter_alloc);
  rb_define_method(rb_cAtomicCounter, "increment", rb_atomic_counter_increment,
                   0);
  rb_define_method(rb_cAtomicCounter, "read", rb_atomic_counter_read, 0);

  VALUE rb_cConcurrentHashMap =
      rb_define_class("ConcurrentHashMap", rb_cObject);
  rb_define_alloc_func(rb_cConcurrentHashMap, rb_concurrent_hash_map_alloc);
  rb_define_method(rb_cConcurrentHashMap, "get", rb_concurrent_hash_map_get, 1);
  rb_define_method(rb_cConcurrentHashMap, "set", rb_concurrent_hash_map_set, 2);
  rb_define_method(rb_cConcurrentHashMap, "clear", rb_concurrent_hash_map_clear,
                   0);
  rb_define_method(rb_cConcurrentHashMap, "fetch_and_modify",
                   rb_concurrent_hash_map_fetch_and_modify, 1);
}
