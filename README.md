# Sofie

[![Crates.io downloads](https://img.shields.io/crates/d/sofie)](https://crates.io/crates/sofie) [![crates.io](https://img.shields.io/crates/v/sofie?style=flat-square)](https://crates.io/crates/sofie) [![Build Status](https://github.com/ararog/sofie/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/sofie/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/sofie) [![Documentation](https://docs.rs/sofie/badge.svg)](https://docs.rs/sofie/latest/sofie) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/sofie/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/sofie/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/sofie)


**The elegant full-stack web framework that makes Rust web development effortless**

**Sofie** is a comprehensive, production-ready web framework for Rust that combines simplicity with power. Built on top of VeTiS, it provides everything you need to build modern web applications - from REST APIs to full-featured web services - with a clean, intuitive API that makes development a joy.

## Why Sofie?

- **Developer Experience**: Ergonomic API design that feels natural and intuitive
- **High Performance**: Powered by VeTiS for blazing-fast request handling
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Production Ready**: Built-in security, middleware, and monitoring capabilities
- **All-in-One**: Full-stack features from routing to authentication
- **Modern Architecture**: Designed for today's web applications

## Quick Start

Add Sofie to your `Cargo.toml`:

```rust
sofie = { version = "0.1.0", features = ["vetis"] }
```

## Usage Example

Here's how simple it is to create a web application with Sofie:

```rust
use sofie::App;
use http_body_util::{Full};
use bytes::Bytes;
use hyper::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std_logger::Config::logfmt().init();

    let mut app = App::new();

    app.serve(|_| async move {
        Ok(Response::new(Full::new(Bytes::from("Hello World"))))
    }).await?;

    Ok(())
}

```

## Perfect For

- **REST APIs**: Build robust, scalable API services
- **Web Applications**: Create full-featured web applications
- **Microservices**: Develop lightweight, focused services
- **Backend Services**: Power modern frontend applications
- **Real-time Apps**: WebSocket and streaming applications
- **AI Integration**: Build AI-powered web services

## Supported Runtimes

- [tokio](https://github.com/tokio-rs/tokio) - High-performance async runtime
- [smol](https://github.com/smol-rs/smol) - Lightweight async runtime

## Crate Features

- **tokio-rt** (default) - Tokio runtime support
- **smol-rt** - Smol runtime support
- **vetis** - VeTiS server integration

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
