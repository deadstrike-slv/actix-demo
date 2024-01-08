# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:latest as builder

# Setup working directory
WORKDIR /usr/src/actix-demo
COPY . .
COPY .env.docker .env

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

EXPOSE 8000
# Build application
RUN cargo install --path .


## ---------------------------------------------------
## 2 - Deploy Stage
##
## Use a distroless image for minimal container size
## - Copy `libpq` dependencies into the image (Required by diesel)
## - Copy application files into the image
## ---------------------------------------------------
#FROM gcr.io/distroless/cc-debian11
#
## Set the architecture argument (arm64, i.e. aarch64 as default)
## For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
#ARG ARCH=aarch64
#
## libpq related (required by diesel)
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
#COPY --from=builder /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
#COPY --from=builder /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
#
## Application files
#COPY --from=builder /usr/local/cargo/bin/actix-demo /usr/local/bin/actix-demo
#COPY --from=builder /usr/src/actix_demo/.env /.env