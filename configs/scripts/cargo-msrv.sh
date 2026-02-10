#!/bin/bash

CURRENT_DIR=$(pwd)

cd ./modules/blog-cli && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/blog-client && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/blog-server && cargo msrv verify && cd "$CURRENT_DIR" || exit
cd ./modules/blog-fe && cargo msrv verify && cd "$CURRENT_DIR" || exit