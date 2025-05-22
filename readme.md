# termstart

terminal based startpage for web browser

## Docker setup (recommended)

A simple `docker compose up --build` should work.

## Setup

### install trunk

```bash
# source (recommended)
cargo install --locked trunk
# binary
cargo binstall trunk
# homebrew
brew install trunk
# for apple silicon install wasm-bindgen from source
cargo install --locked wasm-bindgen-cli
```

### install wasm target

```bash
rustup target add wasm32-unknown-unknown
```

### run the dev server

```bash
trunk serve --open
```