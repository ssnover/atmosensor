#!/bin/bash

set -ex

sudo apt install -y libusb-1.0-0-dev libudev-dev
cargo install probe-run
