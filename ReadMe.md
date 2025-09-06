# Brahma-JS Monorepo

Welcome to **Brahma-JS**, an open-source high-performance server framework that bridges **Rust** (for networking and async core) with **Node.js** (for application logic). This repository is organized as a **monorepo**.

## Overview

Brahma-JS provides:

* **Rust core** (`brahma-core`) built with [napi-rs](https://napi.rs), managing sockets, HTTP parsing, and async execution.
* **Starter framework** (`brahma-firelight`) for quickly scaffolding new applications.
* **Tests** (`tests`) containing JavaScript checks powered by Vite.

## Monorepo Structure

```
/packages
  ├─ brahma-core      # Rust crate compiled to Node.js addon (N-API)
  ├─ brahma-firelight # Starter template framework for apps
  └─ tests            # JavaScript test suite (Vite based)
```

## Features

* 🚀 Written in Rust, exposed to Node.js via N-API.
* 🧵 Scales across multiple cores (Tokio runtime).
* 🔒 One-shot request/response guarantees.
* 📦 Organized as a monorepo for easy collaboration.

## Contributing

Pull requests are welcome! Please open an issue first to discuss major changes.

## License

MIT License © 2025 Brahma-JS Contributors
