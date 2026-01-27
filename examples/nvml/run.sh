#!/usr/bin/env bash

# builds the the shared library inside the docker container and then writes it
# to the host. from there, the host can run using a local binary like
# nvidia-smi..
docker build . -t libnvidia-ml-so && docker run --rm --entrypoint cat libnvidia-ml-so:latest libnvidia-ml.so > libnvidia-ml.so.1 && $@
