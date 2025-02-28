#include "ruby.h"
#include "rust-atomics.h"
#include <ruby/thread.h>

void rb_slow_object_mark(void *);

const rb_data_type_t slow_object_data = {
    .function = {.dmark = rb_slow_object_mark},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

void rb_slow_object_mark(void *ptr) {
  slow_object_t *slow = ptr;
  slow_object_mark(slow, rb_gc_mark);
}

VALUE rb_slow_object_alloc(VALUE klass) {
  slow_object_t *slow;
  TypedData_Make_Struct0(obj, klass, slow_object_t, SLOW_OBJECT_SIZE,
                         &slow_object_data, slow);
  slow_object_alloc(slow);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_slow_object_initialize(VALUE self, VALUE n) {
  slow_object_t *slow;
  TypedData_Get_Struct(self, slow_object_t, &slow_object_data, slow);
  slow_object_init(slow, FIX2LONG(n));
  return Qnil;
}

VALUE rb_slow_object_slow_op(VALUE self) {
  slow_object_t *slow;
  TypedData_Get_Struct(self, slow_object_t, &slow_object_data, slow);
  slow_object_slow_op(slow);
  return Qnil;
}

void *slow_object_slow_op_outer(void *obj) {
  slow_object_slow_op(obj);
  return NULL;
}

VALUE rb_slow_object_slow_op_no_gvl_lock(VALUE self) {
  slow_object_t *slow;
  TypedData_Get_Struct(self, slow_object_t, &slow_object_data, slow);
  rb_thread_call_without_gvl(slow_object_slow_op_outer, slow, NULL, NULL);
  return Qnil;
}

static void init_slow_object(VALUE rb_mCAtomics) {
  VALUE rb_cSlowObject =
      rb_define_class_under(rb_mCAtomics, "SlowObject", rb_cObject);
  rb_define_alloc_func(rb_cSlowObject, rb_slow_object_alloc);
  rb_define_method(rb_cSlowObject, "initialize", rb_slow_object_initialize, 1);
  rb_define_method(rb_cSlowObject, "slow_op", rb_slow_object_slow_op, 0);
  rb_define_method(rb_cSlowObject, "slow_op_no_gvl_lock",
                   rb_slow_object_slow_op_no_gvl_lock, 0);
}
