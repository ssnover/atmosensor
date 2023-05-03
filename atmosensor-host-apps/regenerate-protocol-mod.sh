#!/usr/bin/env bash

start_dir=$(pwd)

# cd to this script's directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}" )"

# Try to get to the root of the repository
while [ ! -e .git ];
do
  cd ..
  if [[ "$(pwd)" == "/" ]]; then
    echo "Executed from outside repo..."
    exit 1
  fi
done

echo "Generating Rust module for protocol..."

cd atmosensor-host-apps
cargo run --bin protocol-generator -- \
-p ../usb-protocol/protocol.json5 \
-o src/protocol.rs

echo "Done"

cd $start_dir
