# Inexor - Reactive Graph Flow - ROADMAP

## Inexor Graph / GraphQL

* GraphQL Dev Tools
  - [x] Visualize the type system as graph
    * https://github.com/APIs-guru/graphql-voyager
    * http://localhost:31415/flow-editor/voyager.html
  - [x] Embed a GraphQL client
    * https://altair.sirmuel.design/docs/integrations/altair-static.html
    * http://localhost:31415/flow-editor/altair.html
  - [ ] Inexor - Reactive Graph Flow - Type Explorer
    * The Type Explorer shows the type system (Components, Entity Types, Relation Types)
    * Sigma JS
  - [ ] Inexor - Reactive Graph Flow - Graph Explorer
    * The Graph Explorer shows the actual content (Entity Instances, Relation Instances, Flows)
    * Sigma JS
    * Navigate through clicking on a node
  - [ ] Inexor - Reactive Graph Flow - Flow Editor
    * Flow Editor lets you create reactive flows
    * diagram.js
* GraphQL Components
  * Navigate (back) from component to entity type
  * Navigate (back) from component to relation type
* Search by properties
* Subscriptions
  * https://async-graphql.github.io/async-graphql/en/subscription.html
* High-Level GraphQL
  * Dynamically generate queries and mutations for entity types
  * Dynamically generate queries and mutations for relation types
* For server applications a persistent datastore would be interesting
  * IndraDB datastore for Neo4J
  * Example: https://github.com/indradb/postgres

## Inexor Frontend

Create a library which can be reused for frontend applications.

* Inexor Graph Library
  * TypeScript
  * Public NPM Package @inexor/reactive-graph-flow-client
  * Consumes the GraphQL API
  * Models for types
    * Component
    * Entity Type
    * Relation Type
  * Models for instances
    * Entity Instance
    * Relation Instance
    * Flow
  * Queries

## Inexor Reactive Graph Flow Editor

- [ ] Uses @inexor/reactive-graph-flow-client
