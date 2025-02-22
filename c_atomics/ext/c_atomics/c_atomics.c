#include "counter.h"
#include "hashmap.h"
#include "plain-counter.h"
#include <ruby.h>

//

RUBY_FUNC_EXPORTED void Init_c_atomics(void) {
  rb_ext_ractor_safe(true);

  init_plain_counter();
  init_counter();
  init_hashmap();
}
