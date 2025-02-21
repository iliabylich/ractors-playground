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

static void init_counter(void) {
  VALUE rb_cAtomicCounter = rb_define_class("AtomicCounter", rb_cObject);
  rb_define_alloc_func(rb_cAtomicCounter, rb_atomic_counter_alloc);
  rb_define_method(rb_cAtomicCounter, "increment", rb_atomic_counter_increment,
                   0);
  rb_define_method(rb_cAtomicCounter, "read", rb_atomic_counter_read, 0);
}
