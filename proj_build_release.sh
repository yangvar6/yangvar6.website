#!/bin/bash
# Building the thing for release typoshit


logok() {
    echo -e "\033[32m$@\033[0m"
}
logerr() {
    echo -e "\033[31m$@\033[0m"
}
run() {
  "$@"
  if [ $? -ne 0 ]; then
    logerr "Command failed: $@"
    exit 1
  fi
}




pushd "./dist"
    run rm *
popd



# generating tailwind styles
run tailwindcss --optimize -m \
    -i style/input.css \
    -o style/output.css
logok "Builded Tailwind styles"



# trunking
run trunk build --release --minify
logok "Builded trunk"



# wasm compacting with binaryen

WASM_FILE=$(ls ./dist/*.wasm 2>/dev/null | head -n 1)

if [ -z "$WASM_FILE" ]; then
  echo "No .wasm file found in ./dist"
  exit 1
fi

# usually 256K
echo "Before minimization wasm size: $(du -h "$WASM_FILE" | cut -f1)" 
run wasm-opt -Oz \
    --converge \
    --all-features \
    --output "$WASM_FILE" \
    "$WASM_FILE" 
logok "Optimized $WASM_FILE"
# usually 232K (wow that what I call minimization my lord!)
echo "After minimization wasm size: $(du -h "$WASM_FILE" | cut -f1)" 