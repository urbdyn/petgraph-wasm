#!/usr/bin/env bash

set -e

repo_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." >/dev/null 2>&1 && pwd )"

cd "$repo_dir"
wasm-pack build --target nodejs --release
