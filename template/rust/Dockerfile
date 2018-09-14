# we use this to build a binary for the alpine container below
FROM rust:latest as BUILD
RUN apt update && apt install musl-tools -y
ADD function/ /app/
WORKDIR /app

# Install target for alpine image (which uses musl)
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Alpine container which will run our app
FROM alpine:latest
ENV WATCHDOG_VERSION 0.9.0
# Add non root user
RUN addgroup -S app && adduser app -S -G app

# Copy artifact from builder
COPY --from=BUILD /app/target/x86_64-unknown-linux-musl/release/* /app/

RUN chown -R app /app

# Install watchdog
RUN apk --no-cache add curl ${ADDITIONAL_PACKAGE} \
    && echo "Pulling watchdog binary from Github." \
    && curl -sSL https://github.com/openfaas/faas/releases/download/$WATCHDOG_VERSION/fwatchdog > /usr/bin/fwatchdog \
    && chmod +x /usr/bin/fwatchdog \
    && apk del curl --no-cache

RUN chmod +x /usr/bin/fwatchdog

USER app
ENV fprocess="/app/faas-rust"

HEALTHCHECK --interval=1s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

