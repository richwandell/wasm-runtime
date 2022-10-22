#!/usr/bin/env bash


for FILE in wat/*; do
  if [[ $FILE == *".wat"* ]]; then
    OUTFILE=${FILE/.wat/.wasm}
    OUTFILE=${OUTFILE/wat/wasm}
    echo "compiling $FILE into $OUTFILE"
    docker run -it --rm -v "$PWD":/src -w /src kirillt/wabt wat2wasm "$FILE" -o "$OUTFILE"
  fi
done

for FILE in wat/*; do if [[ $FILE == *".wat"* ]]; then OUTFILE=${FILE/.wat/.wasm} OUTFILE=${OUTFILE/wat/wasm} echo "compiling $FILE into $OUTFILE" wat2wasm "$FILE" -o "$OUTFILE"; fi done



