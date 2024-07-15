FROM alpine:3.16 AS builder

ARG VERSION=0.1.1
ARG TARGETPLATFORM
ARG PLATFORM=${TARGETPLATFORM#linux/}

WORKDIR /home/fcpst

RUN apk add --no-cache curl tar gzip \
 && curl -LO https://github.com/clownUR/fcpst/releases/download/v${VERSION}/fcpst-${VERSION}_linux_${PLATFORM}.tar.gz \
 && tar xvfz fcpst-${VERSION}_linux_${PLATFORM}.tar.gz 

FROM alpine:3.16

ARG VERSION=0.1.1

LABEL org.opencontainers.image.source https://github.com/clownUR/fcpst

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D nonroot \
  && mkdir -p /workdir

COPY --from=builder /home/fcpst/fcpst-${VERSION}/fcpst /opt/fcpst/fcpst

WORKDIR /workdir
USER nonroot

ENTRYPOINT [ "/opt/fcpst/fcpst" ]
