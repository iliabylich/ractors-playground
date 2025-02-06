#include "atomic_counter.h"
#include <stdio.h>
#include <pthread.h>

typedef struct
{
  size_t counter;
} atomic_counter_t;

const rb_data_type_t atomic_counter_data = {
    .function = {
        .dfree = RUBY_DEFAULT_FREE},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

VALUE rb_atomic_counter_alloc(VALUE klass)
{
  atomic_counter_t *data;
  VALUE obj = TypedData_Make_Struct(klass, atomic_counter_t,
                                    &atomic_counter_data, data);
  data->counter = 0;
  return obj;
}

VALUE rb_atomic_counter_increment(VALUE self)
{
  atomic_counter_t *data;
  TypedData_Get_Struct(self, atomic_counter_t, &atomic_counter_data, data);
  __atomic_fetch_add(&data->counter, 1, __ATOMIC_SEQ_CST);
  return Qnil;
}

VALUE rb_atomic_counter_read(VALUE self)
{
  atomic_counter_t *data;
  TypedData_Get_Struct(self, atomic_counter_t, &atomic_counter_data, data);
  return LONG2FIX(data->counter);
}

VALUE rb_atomic_counter_write(VALUE self, VALUE value)
{
  atomic_counter_t *data;
  TypedData_Get_Struct(self, atomic_counter_t, &atomic_counter_data, data);
  data->counter = FIX2LONG(value);
  return Qnil;
}

RUBY_FUNC_EXPORTED void
Init_atomic_counter(void)
{
  rb_ext_ractor_safe(true);

  VALUE rb_cAtomicCounter = rb_define_class("AtomicCounter", rb_cObject);
  rb_define_alloc_func(rb_cAtomicCounter, rb_atomic_counter_alloc);
  rb_define_method(rb_cAtomicCounter, "increment", rb_atomic_counter_increment, 0);
  rb_define_method(rb_cAtomicCounter, "read", rb_atomic_counter_read, 0);
  rb_define_method(rb_cAtomicCounter, "write", rb_atomic_counter_write, 1);
}
