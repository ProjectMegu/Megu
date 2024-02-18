FROM ubuntu:23.10

ARG BINARYEN_VERSION=116
ARG BINARYEN_URL="https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz"
ARG WABT_VERSION=1.0.34
ARG WABT_URL="https://github.com/WebAssembly/wabt/releases/download/${WABT_VERSION}/wabt-${WABT_VERSION}-ubuntu.tar.gz"

RUN apt-get update && apt-get install -y wget curl git clang

RUN mkdir /work
WORKDIR /work

RUN wget -O binaryen.tar.gz ${BINARYEN_URL}
RUN mkdir /root/.binaryen && tar -xzvf binaryen.tar.gz -C /root/.binaryen --strip-components 1
RUN wget -O wabt.tar.gz ${WABT_URL}
RUN mkdir /root/.wabt && tar -xzvf wabt.tar.gz -C /root/.wabt --strip-components 1

WORKDIR /

RUN rm -r -f /work
ENV PATH=/root/.binaryen/bin:/root/.wabt/bin:${PATH}

# install rust & components
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y