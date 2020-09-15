FROM rustlang/rust:nightly-buster-slim

# Clang because of RocksDB
RUN apt-get update && apt-get install -y clang 
WORKDIR /build
COPY . .
RUN rustup target add wasm32-unknown-unknown && cargo build --release

FROM debian:buster-slim

ENV RUST_BACKTRACE 1

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    curl && \
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete && \
	useradd -m -u 1000 -U -s /bin/sh -d /social-network-node substrate

COPY --from=0 /build/target/release/social-network-node /usr/local/bin

USER substrate

RUN /usr/local/bin/social-network-node --version

EXPOSE 30333 9933 9944
VOLUME ["/social-network-node"]

ENTRYPOINT ["/usr/local/bin/social-network-node"]