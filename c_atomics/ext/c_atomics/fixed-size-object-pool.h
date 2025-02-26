#include "ruby.h"
#include "ruby/internal/module.h"
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

VALUE rb_fixed_size_object_pool_initialize(VALUE self, VALUE size,
                                           VALUE timeout_in_ms) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  fixed_size_object_pool_init(rust_obj, FIX2LONG(size), FIX2LONG(timeout_in_ms),
                              rb_yield);
  return Qnil;
}

VALUE rb_fixed_size_object_pool_pop(VALUE self) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  PooledItem pooled = fixed_size_object_pool_pop(rust_obj);
  if (pooled.idx == 0 && pooled.rbobj == 0) {
    return Qnil;
  }
  VALUE ary = rb_ary_new_capa(2);
  rb_ary_push(ary, pooled.rbobj);
  rb_ary_push(ary, LONG2FIX(pooled.idx));
  return ary;
}

VALUE rb_fixed_size_object_pool_push(VALUE self, VALUE idx) {
  fixed_size_object_pool_t *rust_obj;
  TypedData_Get_Struct(self, fixed_size_object_pool_t,
                       &fixed_size_object_pool_data, rust_obj);
  fixed_size_object_pool_push(rust_obj, FIX2LONG(idx));
  return Qnil;
}

static void init_fixed_size_object_pool(VALUE rb_mCAtomics) {
  VALUE rb_cFixedSizeObjectPool =
      rb_define_class_under(rb_mCAtomics, "FixedSizeObjectPool", rb_cObject);
  rb_define_alloc_func(rb_cFixedSizeObjectPool,
                       rb_fixed_size_object_pool_alloc);
  rb_define_method(rb_cFixedSizeObjectPool, "initialize",
                   rb_fixed_size_object_pool_initialize, 2);
  rb_define_method(rb_cFixedSizeObjectPool, "pop",
                   rb_fixed_size_object_pool_pop, 0);
  rb_define_method(rb_cFixedSizeObjectPool, "push",
                   rb_fixed_size_object_pool_push, 1);
}
