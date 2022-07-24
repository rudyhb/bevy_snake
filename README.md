# Requirements

- [rust](https://www.rust-lang.org/tools/install)

# Running natively

```
cargo run --release
```

# Running in browser

Make sure to have WASM support to your Rust installation:

```
rustup target install wasm32-unknown-unknown
```

### Run

use `wasm-server-runner`:

```
cargo install wasm-server-runner
```

Set up cargo to use it, in `.cargo/config.toml`:

```
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```

and run with:
```
cargo run --target wasm32-unknown-unknown
```

### Build

install [Trunk](https://trunkrs.dev/#install)

```
trunk build
```
