#include <ruby.h>

VALUE rb_object_address_address(VALUE self) { return LONG2NUM(self); }

static void init_object_address(VALUE rb_mCAtomics) {
  VALUE rb_mObjectAddress =
      rb_define_module_under(rb_mCAtomics, "ObjectAddress");
  rb_define_method(rb_mObjectAddress, "address", rb_object_address_address, 0);
}
