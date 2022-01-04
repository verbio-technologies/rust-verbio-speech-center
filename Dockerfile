FROM rust:1.57-slim-buster as build

COPY . /code

WORKDIR /code
RUN rustup component add rustfmt
RUN cargo build --release --all

# ===

FROM ubuntu:20.04
COPY --from=build /code/target/release/batch-client /opt/verbio/bin/batch-client
COPY --from=build /code/target/release/cli-client /opt/verbio/bin/cli-client
RUN apt update \
    && apt install -y ca-certificates \
    && rm -rf /var/lib/apt/lists

