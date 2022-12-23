FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /oci-spec-rs
WORKDIR /oci-spec-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /oci-spec-rs/fuzz/target/x86_64-unknown-linux-gnu/release/oci-parse-fuzz /