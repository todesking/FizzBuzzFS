FROM debian:12

RUN apt update && apt install -y --no-install-recommends \
  libfuse3-dev \
  fuse3 \
  curl \
  ca-certificates \
  gcc \
  pkg-config \
  procps \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
  && echo user_allow_other >> /etc/fuse.conf
ADD Cargo.toml Cargo.lock /root
RUN . "$HOME/.cargo/env" && cd /root && mkdir src && echo 'fn main() {}' >> src/main.rs && cargo build && rm -r src
ADD src /root/src
RUN . "$HOME/.cargo/env" && cd /root && cargo build
ADD shell.sh /root/shell.sh

ENTRYPOINT ["/root/shell.sh"]
CMD ["/bin/bash"]
