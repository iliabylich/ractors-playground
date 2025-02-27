#include "ruby.h"
#include "rust-atomics.h"

void rb_queue_mark(void *);
void rb_queue_free(void *);

const rb_data_type_t queue_data = {
    .function = {.dfree = rb_queue_free, .dmark = rb_queue_mark},
    .flags = RUBY_TYPED_FREE_IMMEDIATELY | RUBY_TYPED_FROZEN_SHAREABLE};

void rb_queue_free(void *ptr) {
  queue_t *queue = ptr;
  queue_drop(queue);
}

void rb_queue_mark(void *ptr) {
  queue_t *queue = ptr;
  queue_mark(queue, rb_gc_mark);
}

VALUE rb_queue_alloc(VALUE klass) {
  queue_t *queue;
  TypedData_Make_Struct0(obj, klass, queue_t, QUEUE_SIZE, &queue_data, queue);
  queue_alloc(queue);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_queue_initialize(VALUE self, VALUE cap) {
  queue_t *queue;
  TypedData_Get_Struct(self, queue_t, &queue_data, queue);
  queue_init(queue, FIX2LONG(cap));
  return Qnil;
}

VALUE rb_queue_try_push(VALUE self, VALUE value) {
  queue_t *queue;
  TypedData_Get_Struct(self, queue_t, &queue_data, queue);
  return queue_try_push(queue, value) ? Qtrue : Qfalse;
}

VALUE rb_queue_try_pop(VALUE self, VALUE fallback) {
  queue_t *queue;
  TypedData_Get_Struct(self, queue_t, &queue_data, queue);
  return queue_try_pop(queue, fallback);
}

static void init_queue(VALUE rb_mCAtomics) {
  VALUE rb_cQueue =
      rb_define_class_under(rb_mCAtomics, "SyncQueue", rb_cObject);
  rb_define_alloc_func(rb_cQueue, rb_queue_alloc);

  rb_define_method(rb_cQueue, "initialize", rb_queue_initialize, 1);
  rb_define_method(rb_cQueue, "try_push", rb_queue_try_push, 1);
  rb_define_method(rb_cQueue, "try_pop", rb_queue_try_pop, 1);
}
