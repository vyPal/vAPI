FROM rust:1-alpine3.22 AS builder
RUN apk add --no-cache musl-dev \
    git

WORKDIR /vapi
COPY . /vapi

RUN rustup show active-toolchain || rustup toolchain install

RUN --mount=type=cache,sharing=private,target=/vapi/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && cp target/release/vapi ./vapi.release

FROM alpine:3.22

COPY --from=builder /vapi/vapi.release /bin/vapi

WORKDIR /vapi

RUN apk add --no-cache libgcc && chown 2613:2613 .

ENV RUST_BACKTRACE=1
EXPOSE 8080
ENTRYPOINT [ "/bin/vapi" ]
HEALTHCHECK CMD nc -z 127.0.0.1 8080 || exit 1
