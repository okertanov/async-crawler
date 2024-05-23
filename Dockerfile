FROM rust:1.78.0 as builder
WORKDIR /usr/src/async-crawler
COPY . .
RUN cargo install --path .

FROM rust:1.78.0 as production
COPY --from=builder /usr/local/cargo/bin/async-crawler /usr/local/bin/async-crawler
ENV RUST_BACKTRACE=1
ENTRYPOINT [ "/usr/local/bin/async-crawler" ]
