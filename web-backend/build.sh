#!/bin/sh

export PKG_CONFIG_PATH=/home/private/local/lib/pkgconfig
export CMAKE_PREFIX_PATH=/home/private/local
cargo build --release --features server
