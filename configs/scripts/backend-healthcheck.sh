#!/bin/bash

ULR_PROTOCOL=http

if [[ "$SERVER_TLS" == "true" ]]; then
    ULR_PROTOCOL=https
fi

curl -kf ${ULR_PROTOCOL}://${BACKEND_HOST}:${BACKEND_HTTP_PORT}/api/v0/health || exit 1