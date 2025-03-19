#include "ruby/internal/anyargs.h"
#include <ruby.h>

VALUE rb_obj_to_address(VALUE self, VALUE obj) { return LONG2NUM(obj); }
VALUE rb_address_to_obj(VALUE self, VALUE obj) { return NUM2LONG(obj); }

static void init_object_address(VALUE rb_mCAtomics) {
  rb_define_global_function("obj_to_address", rb_obj_to_address, 1);
  rb_define_global_function("address_to_obj", rb_address_to_obj, 1);
}
