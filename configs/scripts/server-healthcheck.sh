#!/bin/bash

ULR_PROTOCOL=http

if [[ "$SERVER_TLS" == "true" ]]; then
    ULR_PROTOCOL=https
fi

curl -kf ${ULR_PROTOCOL}://${SERVER_HOST}:${SERVER_HTTP_PORT}/api/v0/health || exit 1