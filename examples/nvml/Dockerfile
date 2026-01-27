FROM rust AS builder

WORKDIR /workdir

ARG DRIVER_VERSION=580.126.09

# TODO: you'll need to adjust these on the builder to target the host.
# 
# downloads the nvidia management library .so file and uses it to build the nvml
# interception library. needs to be cache in a low layer to prevent long rebuilds.
RUN wget https://developer.download.nvidia.com/compute/nvidia-driver/redist/nvidia_driver/linux-sbsa/nvidia_driver-linux-sbsa-${DRIVER_VERSION}-archive.tar.xz
RUN xz -d nvidia_driver-linux-sbsa-${DRIVER_VERSION}-archive.tar.xz
RUN tar xvf nvidia_driver-linux-sbsa-${DRIVER_VERSION}-archive.tar \
        nvidia_driver-linux-sbsa-${DRIVER_VERSION}-archive/lib/libnvidia-ml.so.${DRIVER_VERSION} \
        --strip-components=2

# only need the source code.
COPY src src
COPY Cargo.* .

# run a debug build using the downloaded shared lib.
RUN SHARED_LIBRARY_PATH=/workdir/libnvidia-ml.so.${DRIVER_VERSION} cargo build

FROM alpine AS runner
# copy built rust shared lib into default workdir under the name of the nvidia
# management library.
COPY --from=builder /workdir/target/debug/libmock_lib.so libnvidia-ml.so
