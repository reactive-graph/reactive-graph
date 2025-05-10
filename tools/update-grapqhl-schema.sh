#!/bin/bash
cargo +stable build --release
echo "Generate GraphQL schema"
target/release/reactive-graph schema reactive-graph-schema > schema/graphql/reactive-graph-schema.graphql
target/release/reactive-graph schema dynamic-graph-schema > schema/graphql/dynamic-graph-schema.graphql
target/release/reactive-graph schema reactive-graph-plugin-schema > schema/graphql/reactive-graph-plugin-schema.graphql
target/release/reactive-graph schema reactive-graph-runtime-schema > schema/graphql/reactive-graph-runtime-schema.graphql
