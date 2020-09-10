FROM debian:stretch-slim

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /substrate substrate

# add substrate binary to docker image
COPY ./target/release/social-network-node /usr/local/bin

USER substrate

# check if executable works in this container
RUN /usr/local/bin/social-network-node --version

EXPOSE 30333 9933 9944
VOLUME ["/social-network-node"]

ENTRYPOINT ["/usr/local/bin/social-network-node"]