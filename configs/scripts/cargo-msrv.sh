#!/bin/bash

CURRENT_DIR=$(pwd)

cd ./modules/cli && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/client && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/server && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/wasm-fe && cargo msrv verify && cd "$CURRENT_DIR" || exit