script-path := "target/wasm32-wasi/release/script.wasm"

run script-name="script-component.wasm":
    cargo run --release --package runtime {{script-name}}

create-script-component: download-wasi-adapter build-script
    wasm-tools component new {{script-path}} --adapt wasi_snapshot_preview1.reactor.wasm \
        --output script-component.wasm

build-script:
    cargo build --release --package script --target wasm32-wasi

download-wasi-adapter:
    wget --no-clobber https://github.com/bytecodealliance/componentize-py/raw/67aa9be2627fde0b4e719384b29717a79441d3e6/adapters/2ad057d/wasi_snapshot_preview1.reactor.wasm

create-script-py-component: build-script-py

build-script-py:
    cd python/ && ../bin/componentize-py --wit-path ../wit/script.wit --world example \
        componentize script --output ../script-py.wasm

generate-bindings-py:
    componentize-py --wit-path wit/script.wit --world example bindings bindings/
