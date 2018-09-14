# we use this to build a binary for the alpine container below
FROM rust:latest as BUILD
RUN apt update && apt install musl-tools -y
RUN apt install -qq gcc-arm-linux-gnueabihf -y
ADD function/ /app/
WORKDIR /app

# Install target for alpine image (which uses musl)
RUN rustup target add armv7-unknown-linux-musleabihf
RUN mkdir .cargo
RUN echo '[target.armv7-unknown-linux-musleabihf]\nlinker = "arm-linux-gnueabihf-gcc"' > .cargo/config
RUN cargo build --release --target armv7-unknown-linux-musleabihf

# Alpine container which will run our app
FROM alpine:latest
ENV WATCHDOG_VERSION 0.9.0
# Add non root user
RUN addgroup -S app && adduser app -S -G app

# Copy artifact from builder
COPY --from=BUILD /app/target/armv7-unknown-linux-musleabihf/release/* /app/

RUN chown -R app /app

# Install watchdog
RUN apk --no-cache add curl ${ADDITIONAL_PACKAGE} \
    && echo "Pulling watchdog binary from Github." \
    && curl -sSL https://github.com/openfaas/faas/releases/download/$WATCHDOG_VERSION/fwatchdog-armhf > /usr/bin/fwatchdog \
    && chmod +x /usr/bin/fwatchdog \
    && apk del curl --no-cache

RUN chmod +x /usr/bin/fwatchdog

USER app
ENV fprocess="/app/faas-rust"

HEALTHCHECK --interval=1s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

