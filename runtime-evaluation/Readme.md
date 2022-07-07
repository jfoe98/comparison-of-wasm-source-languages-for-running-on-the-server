# Runtime evaluation
This directory contains the source code for evaluation the compiled Wasm binaries at runtime.

## Runtime evaluation
The first entry point of this tool is used to perform the runtime evaluation of the compiled wasm binaries. The Wasm binaries are executing by using the standalone Wasm runtime Wasmtime and its Embedding API. The measurement is repeated several times per Wasm module. The startup time, shutdown time, and execution time are measured during the execution. The resulting metrics are sent to the Metrics Server via HTTP.

## Collecting metrics from WASI log
When running the runtime evaluation with trace logging enabled, Wasmtime prints several information per WASI call. The second entrypoint of this tool is used to parse the log, associate a section of the log with a concrete workload execution and collect the amount and kind of performed WASI calls by analyzing the log entries. The resulting metrics are sent to the Metrics Server via HTTP.