#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

optimization_level=$1
language="rust"

check_parameters() {
    if [ -z ${optimization_level+x} ]
    then 
        echo "Parameter 'optimization_level' is not set. Not executing build.";
        exit 1
    fi
}

build_docker_image() {
    docker build -t build_"$language" .
}

build() {
    workload_name=$1
    opt_level=$2

    workload="$workload_name"_"$opt_level"

    if [ "$opt_level" = "optimized" ]
    then
        opt_parameter="release-optimized"
    else
        opt_parameter="release-unoptimized"
    fi

    start=$(date +%s%N)

    docker run --rm -v "$PWD":/build build_"$language" \
      cargo build --profile "$opt_parameter" --target=wasm32-wasi

    end=$(date +%s%N)

    if [ -z ${NOT_REPORT_METRICS+x} ];
    then
        ../../../utils/write-metric.sh "$language" compile_time "$workload" $(($((end-start))/1000000))
        ../../../utils/write-metric.sh "$language" binary_size "$workload" "$(stat -c%s "./target/wasm32-wasi/release-$optimization_level/$workload_name.wasm")"
    else
        echo "Development mode: not reporting metrics"
    fi
}

copy_artefact() {
    workload_name=$1
    opt_level=$2

    mkdir -p output
    cp ./target/wasm32-wasi/release-"$opt_level"/"$workload_name".wasm ./output/"$workload_name"_"$opt_level".wasm
}

clean() {
    rm -rf ./target
}

print_versions() {
    docker run --rm build_"$language" cargo --version
    docker run --rm build_"$language" rustup --version
    docker run --rm build_"$language" rustc --version
}

check_parameters
build_docker_image
print_versions > "$language"-version.info

echo "Start building wasm modules for source language $language. Opt-Level: $optimization_level"

for directory_path in ./*; do
    if [ -d "$directory_path" ]; then
        directory_name=$(basename "$directory_path")
        echo "Building $directory_path..." && \
            cd "$directory_path" && \
            clean && \
            build "$directory_name" "$optimization_level" && \
            copy_artefact "$directory_name" "$optimization_level" && \
            cd ..
    fi
done

echo "Finished building wasm modules for source language $language."