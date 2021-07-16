FROM rust:latest as build

WORKDIR /usr/app
COPY . .

RUN cargo build --release

FROM rust:latest

COPY ./rusty-ip.conf /app/rusty-ip.conf
COPY --from=build /usr/app/target/release/rusty-ip /app/rusty-ip

ENTRYPOINT ["/app/rusty-ip", "-c", "/app/rusty-ip.conf"]
