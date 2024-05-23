#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install --path . --target x86_64-pc-windows-gnu

mkdir -p windows
cp install/bin/quatrix.exe windows/quatrix.exe
cp -r assets windows
