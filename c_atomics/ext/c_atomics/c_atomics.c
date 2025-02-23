#include "counter.h"
#include "fixed-size-object-pool.h"
#include "hashmap.h"
#include "plain-counter.h"
#include "queue.h"
#include <ruby.h>

//

RUBY_FUNC_EXPORTED void Init_c_atomics(void) {
  rb_ext_ractor_safe(true);

  init_plain_counter();
  init_counter();
  init_hashmap();
  init_fixed_size_object_pool();
  init_queue();
}
