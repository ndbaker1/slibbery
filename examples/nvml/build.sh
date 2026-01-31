#!/usr/bin/env bash

# builds the the shared library inside the docker container, when then then be
# trasported to a host.

DRIVER_VERSION=${DRIVER_VERSION:-'580.126.09'}

docker build -t libnvidia-ml-so:latest -t libnvidia-ml-so:${DRIVER_VERSION} --build-arg DRIVER_VERSION=${DRIVER_VERSION} .
