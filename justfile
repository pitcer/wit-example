script-path := "target/wasm32-unknown-unknown/debug/script.wasm"

run: create-script-component
    cargo run --package runtime

create-script-component: build-script
    wasm-tools component new {{script-path}} -o script-component.wasm

build-script:
    cargo build --package script --target wasm32-unknown-unknown

script-wat:
    wasm2wat {{script-path}}
