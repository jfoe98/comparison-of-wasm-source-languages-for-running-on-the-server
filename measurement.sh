#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

repetitions=15
GITLAB_ACCESS_TOKEN="glpat-rP9YDBc-uuoE2fU4F6-f"

ensure_dependencies() {
    if ! docker info > /dev/null 2>&1
    then
        echo "Docker is not running. Please start docker to run this script."
        exit 1
    fi

    if ! [ -x "$(command -v curl)" ]
    then
        echo "Curl is not installed. Please install curl to run this script."
    fi
}

prepare_measurements() {
    curl --output runtime_evaluation --location --header "PRIVATE-TOKEN: $GITLAB_ACCESS_TOKEN" "https://gitlab.com/api/v4/projects/37277116/jobs/artifacts/main/raw/runtime-evaluation/target/release/runtime_evaluation?job=build-runtime-evaluation"
    chmod +x ./runtime_evaluation
}

clean() {
    echo "Start cleaning system"

    # Removing all containers and images
    docker container prune -f
    docker image prune -a -f

    echo "Cleaning system finished"
}

run_build() {
    echo "Starting building wasm modules from all source languages"

    cd ./languages
    ./build.sh $repetitions &> ../logs/build.log
    cd ..

    echo "Building finished"
}

run_runtime_evaluation() {
    cd ./runtime-evaluation

    echo "Preparing evaluation tool"
    prepare_measurements
    echo "Preparing runtime evaluation tool finished"

    echo "Starting runtime evaluation"
    export REPETITIONS=$repetitions
    ./runtime_evaluation &> ../logs/runtime-evaluation.log
    echo "Runtime evaluation finished"

    echo "Starting single evaluation run to measure wasi syscalls"
    export REPETITIONS=1
    export RUST_LOG=wasi_common=trace
    WASI_LOG_FILE=../logs/wasi-log.log

    export NOT_REPORT_METRICS=true
    ./runtime_evaluation &> $WASI_LOG_FILE
    export NOT_REPORT_METRICS=false
    ./runtime_evaluation parse-wasi-log $WASI_LOG_FILE
    echo "Finished single evaluation run to measure wasi syscalls"
}

ensure_dependencies
clean

mkdir -p ./logs

run_build
run_runtime_evaluation
