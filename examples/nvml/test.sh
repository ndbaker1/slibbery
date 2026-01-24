#!/usr/bin/env bash

docker run \
    --rm \
    --user $(id -u):$(id -u) \
    --workdir=/workdir \
    -v $(pwd):/workdir \
    -e SHARED_LIBRARY_PATH=/workdir/libnvidia-ml.so.590.48.01 \
    -it \
    rust \
    bash -c "cargo build && mv -n target/debug/libmock_lib.so libnvidia-ml.so.1 && LD_LIBRARY_PATH=. ./nvidia-smi"
