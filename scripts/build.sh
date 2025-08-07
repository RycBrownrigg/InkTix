#!/bin/bash

echo "Building InkTix Core Contract..."

cd contracts/inktix_core
cargo contract build --release

cd ../..
echo "Build completed!"