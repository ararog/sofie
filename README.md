# Sophia

Full-stack web framework for Rust.

## Description

**Sophia** is a full-stack web framework for Rust.

## Install

```rust
sophia = { version = "0.1.0", features = ["vetis"] }
```

## Runtimes

- [tokio](https://github.com/tokio-rs/tokio)
- [smol](https://github.com/smol-rs/smol)

## Crate features

- tokio-rt (default)
- smol-rt

## Examples

```rust
use sophia::Sophia;
use http_body_util::{Full};
use bytes::Bytes;
use hyper::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std_logger::Config::logfmt().init();

    let mut sophia = Sophia::new(config);

    sophia.serve(|_| async move {
        Ok(Response::new(Full::new(Bytes::from("Hello World"))))
    }).await?;

    Ok(())
}

```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
