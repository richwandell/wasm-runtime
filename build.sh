#!/bin/bash

CI=$1

for FILE in wat/*; do
  if [[ $FILE == *".wat"* ]]; then
    OUTFILE=${FILE/.wat/.wasm}
    OUTFILE=${OUTFILE/wat/wasm}
    echo "compiling $FILE into $OUTFILE"
    if [[ "CI" == "$1" ]]; then
      wat2wasm "$FILE" -o "$OUTFILE"
    else
      docker run -it --rm -v "$PWD":/src -w /src kirillt/wabt wat2wasm "$FILE" -o "$OUTFILE"
    fi
  fi
done



