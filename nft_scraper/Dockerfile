# The cache layer.
FROM rust:1.57 AS base

ENV USER=root

WORKDIR /code/app
COPY . /code/app
RUN rm -rf .cargo \
    && mkdir -p .cargo \
    && cargo fetch \
    && cargo vendor >> .cargo/config.toml

CMD [ "cargo", "test", "--offline" ]

# The builder.
FROM base AS builder

RUN rustup component add rustfmt
RUN cargo build --release --offline

# The final build.
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl1.1 libc6 libgcc1 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /code/app/target/release/nft-scraper /usr/bin/nft-scraper
EXPOSE 4444
ENTRYPOINT [ "/usr/bin/nft-scraper" ]
