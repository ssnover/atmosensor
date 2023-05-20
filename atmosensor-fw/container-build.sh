#!/bin/bash

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

cd atmosensor-fw
docker run -v $(pwd):/app fw-cross-compile

cd $start_dir