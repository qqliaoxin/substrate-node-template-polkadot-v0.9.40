## 编译环境
```
rustup install nightly-2023-03-18
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-18
cargo +nightly-2023-03-18 build --release
```
## macOs needs to be installed
```
cargo update -p parity-db
```

## run tests
```
cargo +nightly-2023-03-18  test --package pallet-kitties --lib -- tests --nocapture

cargo test --package pallet-kitties --lib -- tests::created_checks_events --exact --nocapture 
```

## build
```
cargo build --release
```

##  benchmarks
```
cargo build --release --features runtime-benchmarks

./target/release/node-template benchmark pallet --chain dev --execution wasm --wasm-execution compiled --pallet pallet_poe --extrinsic "*" --steps 20 --repeat 10 --output ./pallets/poe/src/weights.rs --template .maintain/frame-weight-template.hbs
```