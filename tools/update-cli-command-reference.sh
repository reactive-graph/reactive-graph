#!/bin/bash
cargo +stable build
echo "Generate CLI Command Reference"
target/release/reactive-graph print-markdown-help > docs/cli/reference/reactive-graph.md
target/release/reactive-graph-client print-markdown-help > docs/cli/reference/reactive-graph-client.md
target/release/reactive-graph-server print-markdown-help > docs/cli/reference/reactive-graph-server.md
target/release/reactive-graph-tooling print-markdown-help > docs/cli/reference/reactive-graph-tooling.md
