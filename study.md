## 编译环境
```
rustup install nightly-2023-04-18
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-04-18
cargo +nightly-2023-04-18 build --release
```

## run tests
```
cargo +nightly-2023-04-18  test --package pallet-kitties --lib -- tests --nocapture
```