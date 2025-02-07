build-atomics:
    cd rust-atomics && cargo build --release && cbindgen --output rust-atomics.h
    cd c_atomics && rake clean && rake compile

build-compile-commands-json:
    ruby build-compile-commands-json.rb
