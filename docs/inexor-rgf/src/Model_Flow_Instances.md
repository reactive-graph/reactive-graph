# Model: Flow Instances

## What is a flow instance?

A flow instance is a collection of entity instances and relation instances. Most of the time, a flow instance serves a
specific purpose.

## Wrapper Entity Instance

A flow instance is itself an entity instance. The properties of the entity instance can be viewed as inputs and outputs
for the entire flow instance. The entity instance which is the flow instance is called `wrapper entity instance`.

```mermaid
graph LR;
    F(Flow Instance = Entity Instance);
    IPI1(Input Property Instance)===>F;
    IPI2(Input Property Instance)===>F;
    IPI3(Input Property Instance)===>F;
    F===>OPI1(Output Property Instance);
    F===>OPI2(Output Property Instance);
    F===>OPI3(Output Property Instance);
```

## Nested Flows

Flow instances are nestable. That is, an entity instance that is itself a flow instance can be used in another flow
instance. These nested flow instances can be thought of as subprograms or sub flows. The input properties of the subflow
can be considered as parameters of the subprogram and the output properties of the subflow can be considered as the
function result of the subprogram.

An entity instance can exist in several flow instances at the same time. For example, a TOML configuration loaded into
an entity instance can be used in multiple flow instances.

### Nested Flow Instance Example

```mermaid
graph LR;
    subgraph Outer-Flow-Instance
        direction LR
        E1(Entity Instance);
        E2(Entity Instance);
        E3(Entity Instance);
        E4(Entity Instance);
        E5(Entity Instance);
        E1--->|connector|IF1E1
        E4--->|connector|IF1E1
        E4--->|connector|E5
        subgraph Inner-Flow-Instance-1
            direction LR
            IF1E1-->|connector|IF1E2
        end
        IF1E2--->E2
        E2--->|connector|IF2E1
        E5--->|connector|IF2E1
        subgraph Inner-Flow-2
            direction LR
            IF2E1-->|connector|IF2E2
        end
        IF2E2--->|connector|E3
    end
```

## GraphQL

```admonish tip "GraphQL"
* [GraphQL Queries and Mutations](./GraphQL_API_Flow_Instances.md)
```
