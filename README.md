# Implementing a raytracer in rust using vulkan and slang

## Dependencies

This project relies on the following:
- [The vulkan SDK and the validation layers](https://www.lunarg.com/vulkan-sdk/) 
- [The rust programming language](https://www.rust-lang.org/)
- [The slang programming language and the $slangc$ compiler](https://shader-slang.com/)

## HOWTO

To run in debug mode:
```sh
RUST_LOG=debug cargo run
```
A file $output.log$ will be created with all the debug outcomes that occured during runtime.

To build and run in release mode:
```sh
cargo build --release
cargo run --release
```

## Shaders

The shaders are automatically built using the `build.rs` script.