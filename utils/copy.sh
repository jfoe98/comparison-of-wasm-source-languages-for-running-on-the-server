#!/bin/sh

scp -r user@ubuntu-vm:/home/user/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/fibonacci/output/fibonacci_optimized.wasm \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/fibonacci/output/

scp -r user@ubuntu-vm:/home/user/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/fibonacci/output/fibonacci_unoptimized.wasm \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/fibonacci/output/

scp -r user@ubuntu-vm:/home/user/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/filesplit/output/filesplit_optimized.wasm \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/filesplit/output/

scp -r user@ubuntu-vm:/home/user/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/filesplit/output/filesplit_unoptimized.wasm \
    ~/Projects/Studium/5_Semester/webassembly/compiling-to-wasm-standalone-runtime-evaluation/languages/c-wasisdk/filesplit/output/