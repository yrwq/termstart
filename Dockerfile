FROM rustlang/rust:nightly-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm@latest

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install trunk and wasm-bindgen-cli
RUN cargo binstall trunk
RUN cargo install wasm-bindgen-cli

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Expose port
EXPOSE 3000

# Default command
CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "3000"]