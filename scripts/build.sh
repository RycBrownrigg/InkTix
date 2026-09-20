#!/bin/bash

echo "Building InkTix Contract..."

cd contracts/inktix
cargo contract build --release

cd ../..
echo "Build completed!"