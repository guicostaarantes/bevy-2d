Bevy 2D web game

To develop:
`cargo run --target wasm32-unknown-unknown`

To build:
`cargo build --release --target wasm32-unknown-unknown`
`wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-2d.wasm`
