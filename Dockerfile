FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown

# Install Linera (would need actual installation)
# RUN cargo install linera-service linera-sdk

WORKDIR /app

# Copy project
COPY . .

# Build contracts (when ready)
# RUN cargo build --release --target wasm32-unknown-unknown

# Install Python dependencies
RUN pip3 install -r ai-oracle/requirements.txt

# Install frontend dependencies
RUN cd frontend && npm install && cd ..

# Make run script executable
RUN chmod +x run.sh

EXPOSE 5173 8080

CMD ["./run.sh"]
