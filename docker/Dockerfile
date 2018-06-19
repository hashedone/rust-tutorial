FROM debian:stretch-backports

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    TERM=screen-256color \
    DEBIAN_FRONTEND=nointeractive

RUN apt-get update && apt-get install -y \
      htop \
      bash \
      curl \
      wget \
      git \
      software-properties-common \
      shellcheck \
      netcat \
      ranger \ 
      ack-grep \
      sqlite

RUN set -eux; \
    \
# this "case" statement is generated via "update.sh"
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        i386) rustArch='i686-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/archive/1.11.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup component add rls-preview rust-analysis rust-src; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN apt-get -t stretch-backports install -y neovim

# Neovim needs this so that <ctrl-h> can work
RUN infocmp $TERM | sed 's/kbs=^[hH]/kbs=\\177/' > /tmp/$TERM.ti
RUN tic /tmp/$TERM.ti

CMD ["/bin/bash"]

ADD nvimconfig /root/.config/nvim

RUN nvim -i NONE -c PlugInstall -c quitall > /dev/null 2>&1

