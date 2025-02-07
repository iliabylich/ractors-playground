require 'json'

includes = [
    RbConfig::CONFIG["rubyhdrdir"],
    RbConfig::CONFIG["rubyarchhdrdir"],
    File.join(__dir__, "rust-atomics")
].map { |dir| " -I#{dir}" }.join

content = [
    {
        directory: __dir__,
        file: "c_atomics/ext/c_atomics/c_atomics.c",
        command: "clang #{includes} c_atomics/ext/c_atomics/c_atomics.c"
    }
]

File.write("compile_commands.json", content.to_json)
