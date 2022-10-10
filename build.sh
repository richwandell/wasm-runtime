#!/usr/bin/env bash

docker run -it --rm -v "$PWD":/src -w /src kirillt/wabt wat2wasm "$1" -o "$2"

