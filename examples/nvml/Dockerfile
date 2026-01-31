FROM rust AS builder

WORKDIR /workdir

ARG DRIVER_VERSION=580.126.09

# TODO: you'll need to adjust these on the builder to target the host.
# 
# downloads the nvidia management library .so file and uses it to build the nvml
# interception library. needs to be cache in a low layer to prevent long rebuilds.
COPY download-* .
RUN DRIVER_VERSION=${DRIVER_VERSION} ./download-libnvidia-ml.sh

# only need the source code.
COPY src src
COPY Cargo.* .

# make sure the SONAME is created. this is what gets used to identify identical
# .so files during the symlink process.
ENV RUSTFLAGS="-C link-arg=-Wl,-soname,libnvidia-ml.so.1"

# run a debug build using the downloaded shared lib.
RUN SHARED_LIBRARY_PATH=/workdir/libnvidia-ml.so.${DRIVER_VERSION} cargo build

FROM alpine AS runner
# copy built rust shared lib into default workdir under the name of the nvidia
# management library.
COPY --from=builder /workdir/target/debug/libmock_lib.so libnvidia-ml.so.1
