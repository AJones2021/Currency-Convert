# Rust Currency Converter

This is a template for a Rust application that's using [Slint](https://slint.rs) for the user interface.

This project will ask the amount and what you want to convert to such as USD to EUR.


## Installing and starting Project

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up currency-convert project with the following
    ```
    cargo generate --git https://github.com/slint-ui/slint-rust-template --name currency-convert
    cd currency-convert
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```


## Next Adding to the Cargo.toml

[dependencies]
slint = "1.0"
reqwest = { version = "0.11", features = ["blocking"]}
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = [] }

[build-dependencies]
slint-build = "1.0"

