#!/bin/sh

scp -r user@measurement-vm:~/Projects/compiling-to-wasm-standalone-runtime-evaluation/logs/build.log \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/logs/

scp -r user@measurement-vm:~/Projects/compiling-to-wasm-standalone-runtime-evaluation/logs/runtime-evaluation.log \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/logs/

scp -r user@measurement-vm:~/Projects/compiling-to-wasm-standalone-runtime-evaluation/logs/wasi-log.log \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/logs/