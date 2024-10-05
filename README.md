# Implementing a realtime raytracer in rust using vulkan and slang

## HOWTO

To run in debug mode:
```sh
RUST_LOG=debug cargo run
```

To build and run in release mode:
```sh
cargo build --release
cargo run --release
```

## Shaders

The shaders are automatically built using the `build.rs` script.