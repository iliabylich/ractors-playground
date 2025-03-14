build-atomics:
    cd rust-atomics && cargo test && cargo build --release && cbindgen --output rust-atomics.h
    cd c_atomics && rake clean && rake compile

build-compile-commands-json:
    ruby build-compile-commands-json.rb

test:
    @just build-atomics
    ruby tests/shared-memory.rb
    ruby tests/require-test.rb
    ruby tests/plain-counter.rb ractors
    ruby tests/atomic-counter.rb ractors
    ruby tests/concurrent-hash-map.rb ractors
    ruby tests/fixed-size-object-pool.rb ractors
    ruby tests/test-framework.rb

mpmc-queue-simulation:
    cd rust-atomics && cargo run --bin mpmc_queue --features simulation 2>&1 | uniq
