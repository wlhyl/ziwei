FROM rust:1.69.0 as build
WORKDIR /app

# 编译swe
RUN mkdir /tmp/swe && cd /tmp/swe &&\
  wget https://www.astro.com/ftp/swisseph/swe_unix_src_2.10.03.tar.gz &&\
  tar xvzf swe_unix_src_2.10.03.tar.gz && \
  cd src && make libswe.so && cp libswe.so /app

COPY ./ /app/

RUN echo [source.crates-io] > cargo.config
RUN echo 'replace-with = "ustc"' >> cargo.config
RUN echo [source.ustc] >> cargo.config
RUN echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"' >> cargo.config

RUN RUSTFLAGS=-L/app cargo --config cargo.config install  --path . --root /tmp/app

FROM debian:buster-slim

WORKDIR /app

COPY --from=build /app/libswe.so /app/lib/libswe.so
COPY --from=build /tmp/app/bin/ziwei /app/bin/ziwei

ENV LD_LIBRARY_PATH=/app/lib

EXPOSE 8080

CMD /app/bin/ziwei
