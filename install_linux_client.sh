#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install --path .

mkdir -p linux
cp install/bin/quatrix linux/quatrix
cp -r assets linux