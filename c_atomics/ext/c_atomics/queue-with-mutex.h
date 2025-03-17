#include "rust-atomics.h"
#include <ruby.h>

void rb_queue_with_mutex_mark(void *);
void rb_queue_with_mutex_free(void *);

const rb_data_type_t queue_with_mutex_data = {
    .function = {.dfree = rb_queue_with_mutex_free,
                 .dmark = rb_queue_with_mutex_mark},
    .flags = RUBY_TYPED_FROZEN_SHAREABLE};

void rb_queue_with_mutex_free(void *ptr) {
  queue_with_mutex_t *queue = ptr;
  queue_with_mutex_drop(queue);
}

void rb_queue_with_mutex_mark(void *ptr) {
  queue_with_mutex_t *queue = ptr;
  queue_with_mutex_mark(queue, rb_gc_mark);
}

VALUE rb_queue_with_mutex_alloc(VALUE klass) {
  queue_with_mutex_t *queue;
  TypedData_Make_Struct0(obj, klass, queue_with_mutex_t, QUEUE_WITH_MUTEX_SIZE,
                         &queue_with_mutex_data, queue);
  queue_with_mutex_alloc(queue);
  VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

VALUE rb_queue_with_mutex_initialize(VALUE self, VALUE cap) {
  queue_with_mutex_t *queue;
  TypedData_Get_Struct(self, queue_with_mutex_t, &queue_with_mutex_data, queue);
  queue_with_mutex_init(queue, FIX2LONG(cap));
  return Qnil;
}

VALUE rb_queue_with_mutex_try_push(VALUE self, VALUE value) {
  queue_with_mutex_t *queue;
  TypedData_Get_Struct(self, queue_with_mutex_t, &queue_with_mutex_data, queue);
  return queue_with_mutex_try_push(queue, value) ? Qtrue : Qfalse;
}

VALUE rb_queue_with_mutex_try_pop(VALUE self, VALUE fallback) {
  queue_with_mutex_t *queue;
  TypedData_Get_Struct(self, queue_with_mutex_t, &queue_with_mutex_data, queue);
  return queue_with_mutex_try_pop(queue, fallback);
}

static void init_queue_with_mutex(VALUE rb_mCAtomics) {
  VALUE rb_cQueueWithMutex =
      rb_define_class_under(rb_mCAtomics, "QueueWithMutex", rb_cObject);
  rb_define_alloc_func(rb_cQueueWithMutex, rb_queue_with_mutex_alloc);

  rb_define_method(rb_cQueueWithMutex, "initialize",
                   rb_queue_with_mutex_initialize, 1);
  rb_define_method(rb_cQueueWithMutex, "try_push", rb_queue_with_mutex_try_push,
                   1);
  rb_define_method(rb_cQueueWithMutex, "try_pop", rb_queue_with_mutex_try_pop,
                   1);
}
