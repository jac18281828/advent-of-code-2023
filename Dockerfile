FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=advent-of-code-2023
WORKDIR /workspaces/${PROJECT}

USER jac
ENV USER=jac
ENV PATH=/home/${USER}/.cargo/bin:$PATH
# source $HOME/.cargo/env

COPY --chown=jac:jac . .

RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps
RUN cargo test
CMD cargo run
