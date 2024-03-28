FROM rust:1.77.0-alpine as build
WORKDIR /app

RUN sed -i s/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g  /etc/apk/repositories
RUN apk add make musl-dev upx

# 编译swe
RUN mkdir /tmp/swe && cd /tmp/swe &&\
  wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz &&\
  tar xvzf swe.tar.gz && \
  cd swisseph-2.10.03 && \
  make libswe.a && cp libswe.a /app

COPY ./ /app/

RUN echo '[source.crates-io]' > cargo.config
RUN echo 'replace-with = "ustc"' >> cargo.config
RUN echo [source.ustc] >> cargo.config
RUN echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> cargo.config

RUN CARGO_HTTP_MULTIPLEXING=false RUSTFLAGS=-L/app cargo --config cargo.config install  --path api --root /tmp/app

RUN strip -s /tmp/app/bin/ziwei_api
RUN strip  --strip-debug /tmp/app/bin/ziwei_api
RUN upx /tmp/app/bin/ziwei_api

FROM alpine:3.19.1

WORKDIR /app

COPY --from=build /tmp/app/bin/ziwei_api /app/bin/ziwei_api


EXPOSE 8080

CMD /app/bin/ziwei_api
