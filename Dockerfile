FROM rust:1.48 as builder

ARG WORKDIR=/usr/src/app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \ 
       tzdata \
       libpq-dev \
    && apt-get -y clean \
    && rm -rf /var/lib/apt/lists/*

ENV TZ Asia/Tokyo

RUN mkdir $WORKDIR
WORKDIR $WORKDIR

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 3000

