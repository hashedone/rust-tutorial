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
      python-dev \
      python-pip \
      python3-dev \
      python3-pip \
      ctags \
      shellcheck \
      netcat \
      ranger \ 
      ack-grep \
      sqlite3 \
      # For python crypto libraries
      libssl-dev \
      libffi-dev \
      locales \
      # For Youcompleteme
      cmake

# Generally a good idea to have these, extensions sometimes need them
RUN locale-gen pl_PL.UTF-8
ENV LANG pl_PL.UTF-8
ENV LANGUAGE pl_PL:en
ENV LC_ALL pl_PL.UTF-8

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

