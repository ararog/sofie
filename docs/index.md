---
layout: default
title: Sofie - The elegant full-stack web framework
nav_order: 1
description: "🌟 The elegant full-stack web framework that makes Rust web development effortless"
permalink: /
---
<div align="center">
<h1><b>Sofie</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/sofie?style=flat-square)](https://crates.io/crates/sofie) 
[![Build Status](https://github.com/ararog/sofie/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/sofie/actions/workflows/rust.yml) 
[![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Sophia** is a comprehensive, production-ready web framework for Rust that combines simplicity with power. Built on top of VeTiS, it provides everything you need to build modern web applications - from REST APIs to full-featured web services - with a clean, intuitive API that makes development a joy.

Built on top of [hyper](https://github.com/hyperium/hyper).

## 🗺️ Roadmap

Sophia is continuously evolving! Here's what we're working on:

### Security & Authentication

- **🔐 Authentication** - Multiple auth methods (JWT, OAuth, Session)
- **🛡️ Authorization** - Role-based access control
- **🔒 CSRF Protection** - Cross-site request forgery prevention

### Middleware & Features

- **🔌 Middleware System** - Composable request/response processing
- **⚡ Rate Limiting** - Protect against abuse and DoS attacks
- **📝 Sessions** - User session management
- **🌐 CORS** - Cross-origin resource sharing support

### Real-time & Monitoring

- **🔌 WebSocket** - Real-time bidirectional communication
- **📊 Metrics** - Application performance monitoring
- **📝 Logging** - Structured logging integration
- **🔍 Tracing** - Distributed tracing support

### AI & Future

- **🤖 AI Agents** - Built-in AI service integration
- **🚀 Advanced Features** - Cutting-edge web technologies

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
sofie = { version = "0.0.9", features = ["http1", "tokio-rt"] }
```

Basic usage:

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

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Sophia in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/sofie-templates`

## Documentation

- [API Reference](https://docs.rs/sofie)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
