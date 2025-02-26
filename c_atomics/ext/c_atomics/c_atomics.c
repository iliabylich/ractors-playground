#include "counter.h"
#include "fixed-size-object-pool.h"
#include "hashmap.h"
#include "plain-counter.h"
#include "queue.h"
#include <ruby.h>

//

RUBY_FUNC_EXPORTED void Init_c_atomics(void) {
  rb_ext_ractor_safe(true);

  VALUE rb_mCAtomics = rb_define_module("CAtomics");

  init_plain_counter(rb_mCAtomics);
  init_counter(rb_mCAtomics);
  init_hashmap(rb_mCAtomics);
  init_fixed_size_object_pool(rb_mCAtomics);
  init_queue(rb_mCAtomics);
}
