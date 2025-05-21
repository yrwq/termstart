FROM rustlang/rust:nightly-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm@latest

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install trunk and wasm-bindgen-cli with specific versions
RUN cargo binstall trunk

RUN cargo install wasm-bindgen-cli

# Install cargo-watch for development
RUN cargo install cargo-watch

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Expose ports
EXPOSE 8080 3000

# Default command (can be overridden by docker-compose)
CMD ["cargo", "run"]