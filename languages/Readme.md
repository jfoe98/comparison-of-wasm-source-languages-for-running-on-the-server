# Compiling from different source languages to Wasm
This directory contains all workload implementations in different source languages. The following source languages and compilers are used in our measurements:
- Emscripten (C)
- WASI SDK (C)
- TinyGo (Go)
- CRuby (Ruby)
- Rust
- AssemblyScript (Typescript)

This directory contains a subdirectory for each compiler. For each compiler, we built a *Dockerfile* with all dependencies needed by the corresponding compiler, and a script *build.sh*. The build script iterates over all workload implementations and compiles them from source code to Wasm. The compilation is performed by using Docker containers that are built from the defined *Dockerfiles*. During the build, the binary size and the compilation time are being measured. Each build is performed without compiler optimizations and with highest compiler optimizations.

The script [build.sh](build.sh) orchestrates the compilation for all compilers.