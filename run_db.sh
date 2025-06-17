#!/bin/bash

docker rm -f surrealdb

docker run -d \
    --name surrealdb \
    --restart=always \
    -v `pwd`/testdata:/testdata \
    --user $(id -u) \
    -p 8000:8000 \
    surrealdb/surrealdb:latest start --user root --pass root rocksdb:/testdata/testbase.db