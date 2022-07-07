#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

repetitions=$1
opt_levels=('optimized' 'unoptimized')

build() {
    language=$1
    echo "Building wasm modules for source language $language"

    cd ./"$language"
    
    for opt_level in "${opt_levels[@]}"
    do
        ./build.sh "$opt_level"
    done
    
    cd ..

    echo "Finished building wasm modules for source language $language"
}

for i in $(seq 1 "$repetitions")
do
    echo "Repetition $i"
    build c-ems
    build c-wasisdk
    build go
    build rust
    build typescript
done

build ruby

echo "Finished building"

