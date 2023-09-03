# User Stories

## Models

### Base Models (Serializable)

- [x] `Component`
- [x] `EntityType`
- [x] `EntityInstance`
    - [x] From Vertex Properties
- [x] `RelationInstance`
    - [x] From Edge Properties
- [x] `Flow`
    - [x] List of entity instances
    - [x] List of relation instances

### Reactive Models (Non-Serializable, Managed by a Registry)

- [x] `ReactivePropertyInstance`
    - [x] Not serializable
    - [x] Getter
    - [x] Typed Getters
    - [x] Setter
    - [x] Send (Send but not set)
    - [x] Tick (Resend)
- [x] `ReactiveEntity`
    - [x] Not serializable
    - [x] Construct ReactiveEntity from Vertex
    - [x] Construct Properties
    - [x] Typed Getters
    - [x] Setter
- [x] `ReactiveRelation`
    - [x] Not serializable
    - [x] Construct ReactiveRelation from Edge
    - [x] Construct Properties
    - [x] Typed Getters
    - [x] Setter
- [x] `ReactiveFlow`
    - [x] List of `ReactiveEntity`
    - [x] List of `ReactiveRelation`

## Behaviours

The idea is to wrap functionality around `ReactiveEntity`s and `ReactiveRelation`s.

A `ReactiveEntity` has properties with streams but doesn't do anything yet.

The reactive behaviour implements the behaviour of a type. For example the AND

### `EntityInstanceBehaviour`s

- [x] `LogicalGate`
- [x] `ArithmeticGate`
- [x] `TrigonometricOperation`

### `RelationInstanceBehaviour`s

- [x] `Connector`
    - [x] `Connector::from_relation(ReactiveRelation)`
    - [x] `Connector::new(OutboundEntity, OutboundPropName, InboundEntity, InboundPropName)`
    - [x] `Connector::connect`
    - [x] `Connector::disconnect`
    - [ ] Optionally: Initially send value down the stream
    - [ ] Optionally: Pause + Resume

## APIs

- [x] `Component Manager`
- [x] `EntityTypeManager`
- [x] `EntityVertexManager`
- [x] `EntityInstanceManager`
- [x] `ReactiveEntityManager`
- [x] `RelationTypeManager`
- [x] `RelationEdgeManager`
- [x] `RelationInstanceManager`
- [x] `ReactiveRelationManager`
    - [x] Resolves which behaviour(s) should be applied on an entity
    - [x] Delegation to Registry
- [x] `ReactiveEntityManager` delegates to `EntityBehaviourManager`
- [x] `ReactiveRelationManager` delegates to `RelationBehaviourManager`
- [x] `EntityBehaviourManager` delegates to `EntityBehaviourRegistries`
- [x] `RelationBehaviourManager` delegates to `RelationBehaviourRegistries`
- [x] `EntityBehaviourRegistry`
- [x] `RelationBehaviourRegistry`
- [x] `EntityBehaviourFactory`
- [x] `RelationBehaviourFactory`
- [x] `EntityBehaviour`
- [x] `RelationBehaviour`
- [x] `FlowManager`
- [x] `ReactiveFlowManager`

## Service Layer Implementations

- [x] `ComponentManagerImpl`
    - [x] Store references of `Component`
    - [x] Has Component by Name
    - [x] Register Component
    - [x] Get All Components
    - [x] Get Component By Name
    - [x] Delete Component By Name
    - [x] Export Component To JSON File
    - [x] Import Component From JSON File
    - [x] Unit Tests
- [x] `EntityTypeManagerImpl`
    - [x] Store references of `EntityType`
    - [x] Has Entity Type by Name
    - [x] Register Entity Type
        - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
    - [x] Create Entity Type
    - [x] Get Entity Type by Name
    - [x] Delete Entity Type By Name
    - [x] Export Entity Type To JSON File
    - [x] Import Entity Type From JSON File
    - [x] Unit Tests
- [x] `RelationTypeManagerImpl`
    - [x] Store references of `RelationType`
    - [x] Has Relation Type by Name
    - [x] Register Relation Type
        - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
    - [x] Create Relation Type
    - [x] Get Relation Type by Name
    - [x] Delete Relation Type By Name
    - [x] Export Relation Type To JSON File
    - [x] Import Relation Type From JSON File
    - [x] Unit Tests
- [x] `EntityVertexManagerImpl`
    - [x] Has Vertex by UUID
    - [x] Get Vertex by UUID
    - [x] Get Vertex Properties by UUID
    - [x] Create Vertex
    - [x] Create Vertex with UUID
        - [x] Check if id exists in Datastore (must not exist)
        - [x] Create Vertex Properties
    - [x] Delete Vertex
    - [x] Unit Tests
- [x] `RelationEdgeManagerImpl`
    - [x] Has Edge by Outbound-UUID, type-name and Inbound-UUID
    - [x] Get Edge by Outbound-UUID, type-name and Inbound-UUID
    - [x] Get Edge Properties by Outbound-UUID, type-name and Inbound-UUID
    - [x] Create Edge
    - [x] Delete Edge By Outbound-UUID, type-name and Inbound-UUID
    - [x] Unit Tests
- [x] `EntityInstanceManagerImpl`
    - [x] Has Entity Instance by UUID
    - [x] Get Entity Instance by UUID
    - [x] Create Entity Instance
    - [x] Create Entity Instance with UUID
    - [x] Delete Entity Instance By UUID
    - [x] Import EntityInstance from JSON
    - [x] Export EntityInstance to JSON
        - [x] Create EntityInstance from Vertex
    - [x] Unit Tests
- [x] `RelationInstanceManagerImpl`
    - [x] Has Relation Instance by Outbound-UUID, type-name and Inbound-UUID
    - [x] Get Relation Instance by Outbound-UUID, type-name and Inbound-UUID
    - [x] Create Relation Instance
    - [x] Delete Relation Instance By Outbound-UUID, type-name and Inbound-UUID
    - [x] Import Relation Instance from JSON
    - [x] Export Relation Instance to JSON
    - [x] Unit Tests
- [x] `ReactiveEntityManagerImpl`
    - [x] Central registry of all `ReactiveEntity`s
    - [x] Create `ReactiveEntity` by UUID
    - [x] On Instantiation: Instantiate `EntityBehaviour`
    - [x] Check if id exists in HashMap (must not exist)
    - [x] Check if id exists in Datastore -> Manager
    - [x] Unit Tests
- [x] `ReactiveRelationManagerImpl`
    - [x] Central registry of all `ReactiveRelation`s
        * These are the actually "running" / "living" instances
    - [x] Create `ReactiveRelation` by UUID
        - [x] Get Relation Instance by EdgeKey from `RelationInstanceManager`
        - [ ] On Instantiation: Instantiate `ReactiveRelationBehaviour` by TYPE
            - [ ] Connector
    - [x] Unit Tests
- [x] `EntityInstanceBehaviourManager`
    - [x] Instantiate Behaviour
    - [x] Remove Behaviour
- [x] `RelationInstanceBehaviourManager`
    - [x] Instantiate Behaviour
    - [x] Remove Behaviour
- [x] `FlowManagerImpl`
    - [x] Create Flow: Creates entity and relation instances contained in the flow
    - [x] Delete Flow: Deletes entity and relation instances contained in the flow
    - [x] Import, Export
    - [ ] Unit Tests
- [x] `ReactiveFlowManagerImpl`
    - [x] Map<FlowId, List<FlowId>>
    - [x] Has
    - [x] Get
    - [x] Create
    - [x] Commit
    - [x] Delete
    - [x] Export
    - [x] Import
    - [ ] Unit Tests

## GraphQL

- [x] async-graphql
- [x] Queries for entity instances / relation instances / flows
- [x] Mutations for entity instances / relation instances / flows
- [ ] Subscriptions for entity instances / relation instances / flows
