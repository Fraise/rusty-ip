# rusty-ip

A what's my ip service, written in Rust.

When it receives a request, rusty-ip returns a JSON response as following:

```json
{
  "ip": "1.2.3.4",
  "forwarded_for": [
    "1.2.3.5",
    "1.2.3.6"
  ]
}
```

The `ip` field contains the ip of the sender, which is the address of the client if the service is opened to the
internet, the address of the Docker network, or the address of the reverse proxy if you used one.

The `forwarded_for` fields is a list containing the values of every `X-Forwarded-For` headers in the request.

## Build and run
### Using Docker

`docker build -t rusty-ip .`

`docker run -p 80:8080 rusty-ip`

The configuration file used is *rusty-ip.conf* located at the root of the repository.

### Bare metal

Install the [Rust toolchain](https://www.rust-lang.org/tools/install) and run `cargo build --release`.

The resulting binary is located at *rusty-ip/target/release*. 
You can then run it with `./rusty-ip -c /path/to/config.conf`.