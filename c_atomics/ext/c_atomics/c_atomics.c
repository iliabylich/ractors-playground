#include "counter.h"
#include "fixed-size-object-pool.h"
#include "hashmap.h"
#include "log-on-mark.h"
#include "mpmc-queue.h"
#include "object-address.h"
#include "plain-counter.h"
#include "queue.h"
#include "slow-object.h"
#include <ruby.h>

RUBY_FUNC_EXPORTED void Init_c_atomics(void) {
  rb_ext_ractor_safe(true);

  VALUE rb_mCAtomics = rb_define_module("CAtomics");

  init_plain_counter(rb_mCAtomics);
  init_counter(rb_mCAtomics);
  init_hashmap(rb_mCAtomics);
  init_fixed_size_object_pool(rb_mCAtomics);
  init_queue(rb_mCAtomics);
  init_slow_object(rb_mCAtomics);
  init_mpmc_queue(rb_mCAtomics);
  init_log_on_mark(rb_mCAtomics);
  init_object_address(rb_mCAtomics);
}
