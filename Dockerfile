FROM rust:alpine AS base
RUN apk add build-base ca-certificates sccache
RUN cargo install cargo-chef --version ^0.1
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/usr/local/cargo/git --mount=type=cache,target=$SCCACHE_DIR,sharing=locked cargo chef prepare --recipe-path recipe.json

FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/usr/local/cargo/git --mount=type=cache,target=$SCCACHE_DIR,sharing=locked cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/usr/local/cargo/git --mount=type=cache,target=$SCCACHE_DIR,sharing=locked cargo build --release --bin reactive-graph

FROM alpine AS reactive-graph
LABEL org.opencontainers.image.title="Reactive Graph"
LABEL org.opencontainers.image.description="Reactive Graph is a reactive runtime based on a graph database, empowering everyone to build reliable and efficient software"
LABEL org.opencontainers.image.vendor="Reactive Graph"
LABEL org.opencontainers.image.url="https://www.reactive-graph.io/"
LABEL org.opencontainers.image.source="https://github.com/reactive-graph/reactive-graph"
LABEL org.opencontainers.image.authors="info@reactive-graph.io"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.documentation="https://docs.reactive-graph.io/book/"
RUN apk add zsh nano curl
WORKDIR /opt/reactive-graph
COPY --from=builder --chown=reactive-graph:reactive-graph /app/target/release/reactive-graph .
RUN addgroup --gid 1000 reactive-graph
RUN adduser -h /opt/reactive-graph -s /bin/bash -G reactive-graph -u 1000 -D reactive-graph
USER reactive-graph
RUN ./reactive-graph instances init --uid 1000 --gid 1000
RUN ./reactive-graph instances config graphql --hostname "0.0.0.0" --secure true
RUN ./reactive-graph instances config instance --instance-name "Reactive Graph" --instance-description "Docker"
ENV PATH="$PATH:/opt/reactive-graph"
EXPOSE 31415
ENTRYPOINT [ "./reactive-graph" ]
