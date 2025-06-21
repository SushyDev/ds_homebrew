FROM skylyrac/blocksds:slim-latest

# Install essential build tools and dependencies including ARM toolchain
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    jq \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    libclang-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust 1.87.0 to match your local environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.87.0 --profile minimal \
    && . ~/.cargo/env \
    && rustup toolchain install nightly --profile minimal \
    && rustup component add rust-src --toolchain nightly \
    && rustup target add armv5te-unknown-linux-gnueabi \
    && rustup target add armv5te-unknown-linux-gnueabi --toolchain nightly

# Install ndstool from cargo-nds
RUN . ~/.cargo/env \
    && cargo install --git https://github.com/SeleDreams/cargo-nds.git \
    && rm -rf ~/.cargo/registry

ENV PATH="/root/.cargo/bin:/opt/wonderful/thirdparty/blocksds/core/tools/ndstool:/opt/wonderful/toolchain/gcc-arm-none-eabi/bin:${PATH}"

WORKDIR /work
