# we use this to build a binary for the alpine container below
FROM rust:latest as BUILD

ENV WATCHDOG_VERSION 0.9.0

RUN apt update && apt install musl-tools -y
RUN apt install -qq gcc-arm-linux-gnueabihf -y
ADD function/ /app/
WORKDIR /app

# Install target for alpine image (which uses musl)
RUN rustup target add armv7-unknown-linux-musleabihf
RUN mkdir .cargo
RUN echo '[target.armv7-unknown-linux-musleabihf]\nlinker = "arm-linux-gnueabihf-gcc"' > .cargo/config
RUN cargo build --release --target armv7-unknown-linux-musleabihf

# We need to do this here because we cannot execute chmod +x in arm container without qemu

ADD https://github.com/openfaas/faas/releases/download/$WATCHDOG_VERSION/fwatchdog-armhf fwatchdog
RUN chmod +x fwatchdog

# Alpine container which will run our app
FROM armhf/alpine:latest

# Copy artifact from builder
COPY --from=BUILD /app/target/armv7-unknown-linux-musleabihf/release/* /app/
COPY --from=BUILD /app/fwatchdog /usr/bin/fwatchdog

ENV fprocess="/app/faas-rust"

HEALTHCHECK --interval=1s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

