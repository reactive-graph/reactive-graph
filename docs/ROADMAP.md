# Inexor - Reactive Graph Flow - ROADMAP

## Inexor Graph / GraphQL

* GraphQL Dev Tools
  -[x] Visualize the type system as graph
    * https://github.com/APIs-guru/graphql-voyager
    * http://localhost:31415/flow-editor/voyager.html
  -[x] Embed a GraphQL client
    * https://altair.sirmuel.design/docs/integrations/altair-static.html
    * http://localhost:31415/flow-editor/altair.html
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

## Application / Core Entity System

### Components

* ComponentManager
  * Query Entity Types by Component
  * Query Relation Types by Component
* Allow adding components to entity instances and relation instances after initial construction
  * Add properties (if not already present)
  * Add component behaviours
* Allow removing components from entity instances and relation instances
  * Remove component behaviours
  * Do not remove properties

### Behaviour Management

* Remove field behaviours from EntityType and RelationType
* Add field behaviours to EntityInstance and RelationInstance (reflects the effectively applied behaviours)
* Each behaviour should add itself to the field behaviours if applied on a EntityInstance or RelationInstance
* Each behaviour should remove itself from the field behaviours if applied on a EntityInstance or RelationInstance
* (Query instances by behaviour)
* ComponentBehaviourManager
  * add_behaviours_to_entity: Iterate through the components of an EntityInstance and call add_behaviours_to_entity with component
  * add_behaviours_to_relation: Iterate through the components of an EntityInstance and call add_behaviours_to_relation with component
  * remove_behaviours_from_entity: Iterate through the components of an EntityInstance and call remove_behaviours_from_entity with component
  * remove_behaviours_from_relation: Iterate through the components of an EntityInstance and call remove_behaviours_from_relation with component
* ComponentBehaviourProvider
  * fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);
  * fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);
  * fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);
  * fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);
