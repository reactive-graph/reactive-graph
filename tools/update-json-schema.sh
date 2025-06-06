#!/bin/bash
cargo +stable build --release
echo "Generate JSON schema"
target/release/reactive-graph components json-schema > schema/json/component.schema.json
target/release/reactive-graph entity-types json-schema > schema/json/entity-type.schema.json
target/release/reactive-graph relation-types json-schema > schema/json/relation-type.schema.json
target/release/reactive-graph flow-types json-schema > schema/json/flow-type.schema.json
target/release/reactive-graph entity-instances json-schema > schema/json/entity-instance.schema.json
target/release/reactive-graph relation-instances json-schema > schema/json/relation-instance.schema.json
target/release/reactive-graph flow-instances json-schema > schema/json/flow-instance.schema.json
