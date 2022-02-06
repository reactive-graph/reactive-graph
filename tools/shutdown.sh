#!/bin/sh
curl -g \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { instances { entities { update(label: \"/org/inexor/system/shutdown\", properties: [ { name: \"shutdown\", value: 3 } ] ) { id } } } }" }' \
  http://localhost:31415/graphql
