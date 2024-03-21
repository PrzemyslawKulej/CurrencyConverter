FROM rust:1.58 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

ENV CURRENCY_CONVERTER_API_KEY=""

CMD ["currency_converter"]



