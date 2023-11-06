Bevy 2D web game

To develop:
`cargo run --target wasm32-unknown-unknown`

To build:
`cargo build --release --target wasm32-unknown-unknown`
`wasm-bindgen --out-dir ./www/wasm --target web ./target/wasm32-unknown-unknown/release/bevy-2d.wasm`
`wasm-opt -Os -o ./www/wasm/bevy-2d_bg.wasm ./www/wasm/bevy-2d_bg.wasm`
