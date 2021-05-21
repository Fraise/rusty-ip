FROM rust:latest as build

WORKDIR /usr/app
COPY . .

RUN cargo build --release

FROM rust:latest

RUN mkdir -p /app/config
COPY --from=build /usr/app/target/release/rusty-ip /app/rusty-ip

ENTRYPOINT ["/app/rusty-ip"]
