# frozen_string_literal: true

require "mkmf"

# Makes all symbols private by default to avoid unintended conflict
# with other gems. To explicitly export symbols you can use RUBY_FUNC_EXPORTED
# selectively, or entirely remove this flag.
append_cflags("-fvisibility=hidden")

rust_atomics_path = File.expand_path("../../../../rust-atomics", __FILE__)
$INCFLAGS << " -I#{rust_atomics_path}"
$LDFLAGS << " -L#{rust_atomics_path}/target/release -lrust_atomics"

create_makefile("c_atomics/c_atomics")
