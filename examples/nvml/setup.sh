#!/usr/bin/env bash

wget https://developer.download.nvidia.com/compute/nvidia-driver/redist/nvidia_driver/linux-x86_64/nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
xz -d nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
tar xvf nvidia_driver-linux-x86_64-590.48.01-archive.tar \
        nvidia_driver-linux-x86_64-590.48.01-archive/lib/libnvidia-ml.so.590.48.01 \
        nvidia_driver-linux-x86_64-590.48.01-archive/sbin/nvidia-smi \
        --strip-components=2
rm nvidia_driver-linux-x86_64-590.48.01-archive.tar

wget https://developer.download.nvidia.com/compute/cuda/redist/cuda_nvml_dev/linux-x86_64/cuda_nvml_dev-linux-x86_64-13.0.39-archive.tar.xz
xz -d cuda_nvml_dev-linux-x86_64-13.0.39-archive.tar.xz
tar xvf cuda_nvml_dev-linux-x86_64-13.0.39-archive.tar \
    cuda_nvml_dev-linux-x86_64-13.0.39-archive/include/nvml.h \
    --strip-components=2
rm cuda_nvml_dev-linux-x86_64-13.0.39-archive.tar
