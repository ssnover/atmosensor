#!/usr/bin/bash

dev_atmosensor=$(readlink -f /dev/atmosensor)

docker run \
    --device=$dev_atmosensor \
    --env INFLUXDB2_TOKEN=$INFLUXDB2_TOKEN \
    --name atmosensord \
    -it \
    atmosensord:latest \
    "/app/target/release/atmosensord $dev_atmosensor"