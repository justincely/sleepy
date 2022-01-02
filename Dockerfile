FROM rust as builder

WORKDIR /app
ADD . /app

RUN rustup default nightly && \
    rustup update && \
    cargo update && \
    cargo build --release


FROM rust:1-slim

WORKDIR /app
COPY --from=builder /app/target/release/sleepy /app/sleepy
COPY --from=builder /app/static/index.html /app/static/index.html
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

ENV ROCKET_ENV production

CMD ./sleepy
