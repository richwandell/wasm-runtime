#!/usr/bin/env bash

rm wasm/*.wasm

if [ "$1" != "" ]; then
    FILE="$1"
    OUTFILE=${FILE/.wat/.wasm}
    OUTFILE=${OUTFILE/wat/wasm}
    echo "compiling $FILE into $OUTFILE"
    docker run --rm -v "$PWD":/src -w /src kirillt/wabt wat2wasm "$FILE" -o "$OUTFILE" --enable-bulk-memory
else
  for FILE in wat/*; do
    if [[ $FILE == *".wat"* ]]; then
      OUTFILE=${FILE/.wat/.wasm}
      OUTFILE=${OUTFILE/wat/wasm}
      echo "compiling $FILE into $OUTFILE"
      docker run --rm -v "$PWD":/src -w /src kirillt/wabt wat2wasm "$FILE" -o "$OUTFILE" --enable-bulk-memory
    fi
  done
fi





