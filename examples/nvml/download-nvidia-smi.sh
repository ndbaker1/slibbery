#!/usr/bin/env bash

ARCH=$(uname -m)
DRIVER_VERSION=${DRIVER_VERSION:-'580.126.09'}
DRIVER_PACKAGE_ARCH=${ARCH/aarch64/sbsa}

wget https://developer.download.nvidia.com/compute/nvidia-driver/redist/nvidia_driver/linux-${DRIVER_PACKAGE_ARCH}/nvidia_driver-linux-${DRIVER_PACKAGE_ARCH}-${DRIVER_VERSION}-archive.tar.xz
xz -d nvidia_driver-linux-${DRIVER_PACKAGE_ARCH}-${DRIVER_VERSION}-archive.tar.xz
tar xvf nvidia_driver-linux-${DRIVER_PACKAGE_ARCH}-${DRIVER_VERSION}-archive.tar \
    nvidia_driver-linux-${DRIVER_PACKAGE_ARCH}-${DRIVER_VERSION}-archive/sbin/nvidia-smi \
    --strip-components=2
rm nvidia_driver-linux-${DRIVER_PACKAGE_ARCH}-${DRIVER_VERSION}-archive.tar
