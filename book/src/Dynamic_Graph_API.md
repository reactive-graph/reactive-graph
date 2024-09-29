# Dynamic Graph API

### Advantages

1. It is easier to query or modify because you operate on real types not abstract types
2. Queries and mutations are more readable
3. The navigation through the Dynamic Graph is much easier and more natural
4It is easier to use by the consumer because it's not necessary to convert the abstract types into the real types as
   you already operate on real types
5It is possible to use a code generator to generate types from the Dynamic Graph schema

### Limitations

1. Not everything is possible
2. The Dynamic Graph schema contains all [Components](./Model_Component.md), [Entity Types](./Model_Entity_Type.md) and
   [Relation Types](./Model_Relation_Type.md) __as defined by the type system__. The Reactive Graph Flow enables you to
   add further components to existing [Entity Instances](./Model_Entity_Instance.md) or
   [Relation Instances](./Model_Relation_Instance.md). The schema simply cannot contain which components have been added
   to which instances at runtime. Querying for these properties is possible but makes the query invalid.
3. Due to the parsing and resolving at runtime the Dynamic Graph is notable slower than the
   [GraphQL API](./GraphQL_API.md) which is generated at compile time.

## GraphQL Endpoint

The GraphQL endpoint can be reached at `http://hostname/31415/dynamic-graph` or `ws://hostname/31415/dynamic-graph`.

The GraphQL schema documentation is automatically generated using the documentation of the
[Components](./Model_Component.md), [Entity Types](./Model_Entity_Type.md) or
[Relation Types](./Model_Relation_Type.md).

### Schema Introspection

The GraphQL Server allows introspection and returns a GraphQL schema including documentation. With this it is possible
to validate queries and mutations and some tools can use the schema to provide autocompletion for creating queries.

```admonish tip "Schema Regeneration"
The GraphQL schema is regenerated each time a [Component](./Model_Component.md), an
[Entity Type](./Model_Entity_Type.md) or a [Relation Type](./Model_Relation_Type.md) is added or removed.
```


## GraphQL Tools

* [Altair](https://altair.sirmuel.design/)
* [GraphQL Voyager](https://apis.guru/graphql-voyager/)
* [Firefox Addon GraphQL Developer Tools](https://addons.mozilla.org/de/firefox/addon/graphql-developer-tools/)

## Usage

{{ graphql_playground(config="/examples/dynamic-graph/dynamic-graph.json") }}
