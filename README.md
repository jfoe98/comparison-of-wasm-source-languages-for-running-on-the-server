# Compiling to WASM: Evaluation of different compilers and source languages in standalone WASM Runtimes
This repository provides the complete source code and implementations for the paper **Performance Comparison of WebAssembly Source Languages and Compilers for Running WebAssembly on the Server**.

## Languages
In order to evaluate different compilers and source languages regarding their quality for compiling to WebAssembly, all workloads need to be implemented in different source languages and compiled to Wasm. The subdirectory [languages](languages/Readme.md) contains the source code of all workloads implemented in different source languages.

## Runtime evaluation
To evaluate the performance of Wasm binaries in a standalone Wasm runtime, we built our own evaluation tool based on the Wasmtime Embedding API. The subdirectory [runtime-evaluation](runtime-evaluation/Readme.md) contains all code for executing Wasm binaries and collecting runtime performance metrics.

## Evaluation
The subdirectory [evaluation](evaluation/Readme.md) contains a Jupyter Notebook for evaluation the measurement results by analyzing the raw data and creating different plots.

## Metrics server
In our measurements, during the compilation and execution of the Wasm binaries, several metrics are collected. The Metrics Server provides a simple HTTP API where we send the collecting metrics in order to be persisted.

## Measurement orchestration
The script [measurement.sh](measurement.sh) orchestrates the measurement. It first triggers the compilation of the source code that is implemented in different source languages to WebAssembly. Afterwards, the runtime evaluation is started to measure the performance of the Wasm binaries at runtime.
