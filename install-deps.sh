#!/bin/bash
set -e
echo "Installing system dependencies for Verity on Codespace..."

# System libraries (protobuf, OpenSSL, etc.)
sudo apt-get update
sudo apt-get install -y \
    protobuf-compiler \
    libssl-dev \
    pkg-config \
    clang \
    cmake \
    curl \
    wget \
    git \
    build-essential

# Rust components
rustup component add rustfmt clippy llvm-tools-preview
cargo install cargo-deny cargo-audit cargo-outdated

# TLA+ tools
if [ ! -f /usr/local/bin/tlc ]; then
    wget https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar -O /tmp/tla2tools.jar
    echo 'java -cp /tmp/tla2tools.jar tlc2.TLC "$@"' | sudo tee /usr/local/bin/tlc > /dev/null
    sudo chmod +x /usr/local/bin/tlc
fi

# Lean 4 (optional; leave CI to handle heavy usage)
# Codespace can install Lean 4 extension manually if needed.

# Verify Rust
rustc --version
cargo --version

echo "Dependencies installed. You can now run the Master Build Script."