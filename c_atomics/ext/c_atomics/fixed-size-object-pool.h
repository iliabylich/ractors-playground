#include "ruby.h"
#include "ruby/internal/anyargs.h"
#include "ruby/internal/arithmetic/long.h"
#include "rust-atomics.h"

void rb_fixed_size_object_pool_mark(void *);
void rb_fixed_size_object_pool_free(void *);

const rb_data_type_t fixed_size_object_pool_data = {
    .function = {.dfree = rb_fixed_size_object_pool_free,
                 .dmark = rb_fixed_size_object_pool_mark},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

void rb_fixed_size_object_pool_free(void *ptr) {
  fixed_size_object_pool_t *hashmap = ptr;
  fixed_size_object_pool_drop(hashmap);
}

void rb_fixed_size_object_pool_mark(void *ptr) {
  fixed_size_object_pool_t *hashmap = ptr;
  fixed_size_object_pool_mark(hashmap, rb_gc_mark);
}

VALUE rb_fixed_size_object_pool_alloc(VALUE klass) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Make_Struct0(obj, klass, fixed_size_object_pool_t,
                         FIXED_SIZE_OBJECT_POOL_SIZE,
                         &fixed_size_object_pool_data, rust_obj);
  fixed_size_object_pool_alloc(rust_obj);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_fixed_size_object_pool_initialize(VALUE self, VALUE size) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  fixed_size_object_pool_init(rust_obj, FIX2LONG(size), 1000, rb_yield);
  return Qnil;
}

VALUE rb_fixed_size_object_pool_pop(VALUE self) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  return fixed_size_object_pool_pop(rust_obj, Qnil);
}

VALUE rb_fixed_size_object_pool_push(VALUE self, VALUE value) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  fixed_size_object_pool_push(rust_obj, value);
  return Qnil;
}

static void init_fixed_size_object_pool(void) {
  VALUE rb_cFixedSizeObjectPool =
      rb_define_class("FixedSizeObjectPool", rb_cObject);
  rb_define_alloc_func(rb_cFixedSizeObjectPool,
                       rb_fixed_size_object_pool_alloc);
  rb_define_method(rb_cFixedSizeObjectPool, "initialize",
                   rb_fixed_size_object_pool_initialize, 1);
  rb_define_method(rb_cFixedSizeObjectPool, "pop",
                   rb_fixed_size_object_pool_pop, 0);
  rb_define_method(rb_cFixedSizeObjectPool, "push",
                   rb_fixed_size_object_pool_push, 1);
}
