FROM nvidia/cuda:12.4.1-cudnn-devel-ubuntu22.04

WORKDIR /app

RUN apt-get update && apt-get install -y wget
RUN wget https://github.com/Kitware/CMake/releases/download/v3.28.6/cmake-3.28.6-linux-x86_64.sh
RUN sh cmake-3.28.6-linux-x86_64.sh --skip-license --prefix=/usr/local

# C/C++
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    ca-certificates \
    python3-dev \
    python3-pip \
    clang \
    libclang-dev


# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN apt-get install -y libomp-dev
ENV OMP_NUM_THREADS=1

# I use it for testing, anyway you should use --release flag
RUN cargo build

ENTRYPOINT ["/bin/sh", "-c"]
