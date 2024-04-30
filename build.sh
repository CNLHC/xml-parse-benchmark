#!/bin/sh
set -eux

ROOT=$(dirname "$0" | realpath) 


cd "${ROOT}/lang/golang" || exit
go mod tidy
go build

cd "${ROOT}/lang/rust/" || exit
cargo update
cargo build --release

python3 -m pip install lxml

