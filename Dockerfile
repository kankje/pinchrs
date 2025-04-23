FROM rust:1.86.0-alpine AS builder

RUN \
  --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  cargo install --path .


FROM scratch

COPY --from=builder /usr/local/cargo/bin/pinchrs /usr/local/bin/pinchrs

CMD ["pinchrs"]
