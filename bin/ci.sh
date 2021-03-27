#!/usr/bin/env bash

set -e

repo_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." >/dev/null 2>&1 && pwd )"

cd "$repo_dir"
echo "Building package ..."
wasm-pack build --target nodejs --release
echo ""
echo "Adding @urbdyn/ to package.json ..."
sed -i.bak 's|"name": "petgraph-wasm",|"name": "@urbdyn/petgraph-wasm",|g' "$repo_dir/pkg/package.json"
