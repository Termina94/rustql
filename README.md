# Set Up

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install wasm-bindgen-cli
cargo install cargo-make
cargo install cargo-watch

rustql-yew: npm i;
rustql-types: cargo build;
rustql-yew: cargo build;
rustql-api: cargo build;
```

# Start Dev

```
yew: trunk serve
api: cargo make dev
```

# Build Release

```
rustql-yew: trunk build --release
rustql-api: cargo build --release
```

# Create App

rustql-yew:
```
 npx tauri init
```

copy release rustql-api.exe to src-tauri/rustql-api-x86_64-pc-windows-msvc.exe

add to tauri.conf.json:
```
"externalBin": ["./rustql-api"],
```

npx tauri build