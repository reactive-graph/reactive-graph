#!/bin/sh
curl -g \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { instances { entities { update(id: \"6ba7b810-9e15-11d1-50b4-00c04fd530c7\", properties: [ { name: \"shutdown\", value: true } ] ) { id } } } }" }' \
  http://localhost:31415/graphql
