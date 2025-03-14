#include <ruby.h>

void rb_log_on_mark_mark(void *);
void rb_log_on_mark_free(void *);

typedef struct {
  VALUE this;
} log_on_mark_t;

const rb_data_type_t log_on_mark_data = {
    .function = {.dfree = rb_log_on_mark_free, .dmark = rb_log_on_mark_mark},
    .flags = RUBY_TYPED_FROZEN_SHAREABLE};

void rb_log_on_mark_free(void *ptr) {
  log_on_mark_t *log_on_mark = ptr;
  fprintf(stderr, "[log-on-mark] Freeing %lu\n", log_on_mark->this);
}

void rb_log_on_mark_mark(void *ptr) {
  log_on_mark_t *log_on_mark = ptr;
  fprintf(stderr, "[log-on-mark] Marking %lu\n", log_on_mark->this);
}

VALUE rb_log_on_mark_alloc(VALUE klass) {
  log_on_mark_t *log_on_mark;
  TypedData_Make_Struct0(obj, klass, log_on_mark_t, sizeof(log_on_mark_t),
                         &log_on_mark_data, log_on_mark);
  log_on_mark->this = obj;
  //   VALUE rb_cRactor = rb_const_get(rb_cObject, rb_intern("Ractor"));
  //   rb_funcall(rb_cRactor, rb_intern("make_shareable"), 1, obj);
  return obj;
}

static void init_log_on_mark(VALUE rb_mCAtomics) {
  VALUE rb_cLogOnMark =
      rb_define_class_under(rb_mCAtomics, "LogOnMark", rb_cObject);
  rb_define_alloc_func(rb_cLogOnMark, rb_log_on_mark_alloc);
}
