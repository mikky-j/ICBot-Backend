
# Justfile

start:
    dfx start --background

deploy:
    dfx deploy

# Define a task to run candid extractor on a Rust crate
gen crate:
    cargo build -p {{crate}} --release --target wasm32-unknown-unknown
    candid-extractor target/wasm32-unknown-unknown/release/{{crate}}.wasm > src/{{crate}}/{{crate}}.did
    dfx generate

