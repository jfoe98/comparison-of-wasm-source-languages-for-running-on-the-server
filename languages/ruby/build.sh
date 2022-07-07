#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

#todo: Metriken hier noch erfassen oder nicht weil wenig vergleichbar?
docker build -t build_ruby .

docker run -dit --rm --name build-ruby build_ruby
docker cp build-ruby:/ruby/ruby ./ruby.wasm
docker container stop build-ruby