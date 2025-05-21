# termstart

terminal themed startpage for web browser

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

### Database Setup

1. Install PostgreSQL if not already installed:

```bash
brew install postgresql
```

2. Start the PostgreSQL server:
```bash
brew services start postgresql
```
3. Create a database and user:
```bash
createdb termstart
psql termstart < backend/migrations/init.sql
```
4. Run the backend server:  
```bash
cd backend && cargo run
```

TODO create a docker file
