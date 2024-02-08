FROM ubuntu:23.10

# from base.dockerfile
# https://apt.llvm.org
ENV deb='deb http://apt.llvm.org/mantic/ llvm-toolchain-mantic-16 main'
ENV debsrc='deb-src http://apt.llvm.org/mantic/ llvm-toolchain-mantic-16 main'

RUN --mount=type=cache,target=/var/lib/apt/lists \
    --mount=type=cache,target=/var/cache/apt/archives \
    apt-get update && \
    apt-get install -y wget git gnupg curl build-essential

RUN --mount=type=cache,target=/var/lib/apt/lists \
    --mount=type=cache,target=/var/cache/apt/archives \
    echo ${deb} >> /etc/apt/sources.list && \
    echo ${debsrc} >> /etc/apt/sources.list && \
    wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    apt-get update && \
    apt-get install -y \
    # llvm
    libllvm16 llvm-16 llvm-16-dev \
    # linker
    lld-16 \
    # clang (dev runtime)
    clang-16

# install rust & components
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y