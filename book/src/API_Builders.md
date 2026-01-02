# Builders

The builder pattern is useful when you would otherwise require many constructors or where construction has side effects.

## Component Builder

Constructs a new component programmatically.

```rust
fn build_component() -> Component {
    ComponentBuilder::new("namespace", "component_name")
        .description("A test component with several properties")
        // Provide properties
        .property("property_1", DataType::String)
        .property_from(PropertyType::new("property_2", DataType::Bool))
        .string_property("property_3")
        .bool_property("property_4")
        .number_property("property_5")
        .array_property("property_6")
        .object_property("property_7")
        .input_property("property_8", DataType::Bool)
        .output_property("property_9", DataType::Bool)
        // Provide extensions
        .extension("security", json!({
            roles: [
                "ADMIN"
            ]
        }))
        .build()
}
```

> [!TIP]
> The component is not yet registered.
> Register the component using `ComponentManager::register`

## Entity Type Builder

Constructs a new entity type programmatically.

```rust
fn build_entity_type() -> EntityType {
    EntityTypeBuilder::new("namespace", "entity_type_name")
        .description("A test entity type")
        // Compose with the labeled component
        .component("labeled")
        // Provide additional properties
        .property("property_1", DataType::String)
        .property_from(PropertyType::new("property_2", DataType::Bool))
        .string_property("property_3")
        .bool_property("property_4")
        .number_property("property_5")
        .input_property("property_6", DataType::Bool)
        .output_property("property_7", DataType::Bool)
        // Provide extensions
        .extension("usage", json!("ls [OPTION]... [FILE]..."))
        .build()
}
```

> [!TIP]
> The entity type is not yet registered.
> Register the entity type using `EntityTypeManager::register`

## Relation Type Builder

Constructs a new relation type programmatically.

```rust
fn build_relation_type() -> RelationType {
    RelationTypeBuilder::new(
        // Outbound Entity Type
        "teleport",
        // Relation Type Name
        "teleports_to",
        // Inbound Entity Type
        "tele_destination"
    )
        .description("A relation type which connects a teleport with a tele destination. The property weight defines the probability")
        // Compose with the weighted component
        .component("weighted")
        // Provide additional properties
        .property("property_1", DataType::String)
        .property_from(PropertyType::new("property_2", DataType::Bool))
        .string_property("property_3")
        .bool_property("property_4")
        .number_property("property_5")
        .input_property("property_6", DataType::Bool)
        .output_property("property_7", DataType::Bool)
        // Provide extensions
        .extension("flow_arrow_color", json!("#ff7700"))
        .extension("flow_arrow_labeling", json!("teleports from {outbound.name} to {inbound.name}"))
        .build()
}
```

> [!TIP]
> The relation type is not yet registered.
> Register the relation type using `RelationTypeManager::register`

## Entity Instance Builder

Constructs a new entity instance programmatically. This is the non-reactive variant.

```rust
fn build_entity_instance(type_name: String) -> EntityInstance {
    EntityInstanceBuilder::new(type_name)
        .id(id)
        .property("property_1", json!("value_1"))
        .get()
}

fn build_entity_instance_from_type(entity_type: EntityType) -> EntityInstance {
    EntityInstanceBuilder::from(entity_type)
        .id(id)
        .property("property_1", json!("value_1"))
        .get()
}
```

> [!TIP]
> The entity instance is non-reactive and not yet registered.
> Create a reactive instance and register it using `ReactiveEntityManager::create_reactive_instance`

## Relation Instance Builder

Constructs a new relation instance programmatically. This is the non-reactive variant.

> [!TIP]
> The relation instance is non-reactive and not yet registered.
> Create a reactive instance and register it using `ReactiveRelationManager::create_reactive_instance`

## Reactive Entity Instance Builder

Constructs a new reactive entity instance programmatically.

> [!TIP]
> The reactive entity instance is fully functional, but not yet registered.
> Register the reactive entity instance using `ReactiveEntityManager::register_reactive_instance`

## Reactive Relation Instance Builder

Constructs a new reactive relation instance programmatically.

> [!TIP]
> The reactive relation instance is fully functional, but not yet registered.
> Register the reactive relation instance using `ReactiveRelationManager::register_reactive_instance`

## Flow Builder
