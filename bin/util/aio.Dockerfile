FROM rust:1.87.0-bullseye AS builder

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery
COPY ./bin/util ./bin/util

# Compile bin
RUN cargo build -p komodo_util --release

# Copy binaries to distroless base
FROM gcr.io/distroless/cc

COPY --from=builder /builder/target/release/util /usr/local/bin/util

CMD [ "util" ]

LABEL org.opencontainers.image.source=https://github.com/moghtech/komodo
LABEL org.opencontainers.image.description="Komodo Util"
LABEL org.opencontainers.image.licenses=GPL-3.0