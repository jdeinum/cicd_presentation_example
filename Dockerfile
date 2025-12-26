FROM rust:1.89-alpine AS chef
USER root
RUN apk add --no-cache musl-dev && \
    cargo install cargo-chef
WORKDIR /app

# create our plan
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build deps
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build our code
COPY . .
RUN cargo build --release --bin cicd_presentation_example

# runtime image
FROM alpine:latest AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/release/cicd_presentation_example /usr/local/bin/app

USER myuser

EXPOSE 8000

CMD ["/usr/local/bin/app"]
