#include "rust-atomics.h"
#include <ruby.h>

const rb_data_type_t plain_counter_data = {
    .function = {.dfree = RUBY_DEFAULT_FREE},
    .flags = RUBY_TYPED_FROZEN_SHAREABLE};

VALUE rb_plain_counter_alloc(VALUE klass) {
  plain_counter_t *counter;
  TypedData_Make_Struct0(obj, klass, plain_counter_t, PLAIN_COUNTER_SIZE,
                         &plain_counter_data, counter);
  plain_counter_init(counter, 0);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_plain_counter_increment(VALUE self) {
  plain_counter_t *counter;
  TypedData_Get_Struct(self, plain_counter_t, &plain_counter_data, counter);
  plain_counter_increment(counter);
  return Qnil;
}

VALUE rb_plain_counter_read(VALUE self) {
  plain_counter_t *counter;
  TypedData_Get_Struct(self, plain_counter_t, &plain_counter_data, counter);
  return LONG2FIX(plain_counter_read(counter));
}

static void init_plain_counter(VALUE rb_mCAtomics) {
  VALUE rb_cPlainCounter =
      rb_define_class_under(rb_mCAtomics, "PlainCounter", rb_cObject);
  rb_define_alloc_func(rb_cPlainCounter, rb_plain_counter_alloc);
  rb_define_method(rb_cPlainCounter, "increment", rb_plain_counter_increment,
                   0);
  rb_define_method(rb_cPlainCounter, "read", rb_plain_counter_read, 0);
}
