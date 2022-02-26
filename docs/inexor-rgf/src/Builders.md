# Builders

The builder pattern is useful when you would otherwise require many constructors or where construction has side effects.

```admonish tip "Cargo.toml"
Don't forget to include the dependency:
> [dependencies]<br>
> inexor-rgf-core-builder = { git = "https://github.com/aschaeffer/inexor-rgf-core-builder.git" }
```

## Component Builder

Constructs a new component programmatically.

```rust
fn build_component() -> Component {
    ComponentBuilder::new("test")
        .description("A test component with several properties")
        // Provide properties
        .property("property_1", DataType::String)
        .property_from(PropertyType::new("property_2", DataType::Bool))
        .string_property("property_3")
        .bool_property("property_4")
        .number_property("property_5")
        .input_property("property_6", DataType::Bool)
        .output_property("property_7", DataType::Bool)
        // Provide extensions
        .extension("security", json!({
            roles: [
                "ADMIN"
            ]
        }))
        .build()
}
```

```admonish tip "Register Component"
The component is not yet registered.

Register the component using `ComponentManager::register`
```

## Entity Type Builder

Constructs a new entity type programmatically.

```rust
fn build_entity_type() -> EntityType {
    EntityTypeBuilder::new("Test")
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

```admonish tip "Register Entity Type"
The entity type is not yet registered.

Register the entity type using `EntityTypeManager::register`
```

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

```admonish tip "Register Relation Type"
The relation type is not yet registered.

Register the relation type using `RelationTypeManager::register`
```

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

```admonish tip "Create Reactive Entity Instance"
The entity instance is non-reactive and not yet registered.

Create a reactive instance and register it using `ReactiveEntityInstanceManager::create_reactive_instance`
```

## Relation Instance Builder

Constructs a new relation instance programmatically. This is the non-reactive variant.

```admonish tip "Create Reactive Relation Instance"
The relation instance is non-reactive and not yet registered.

Create a reactive instance and register it using `ReactiveRelationInstanceManager::create_reactive_instance`
```

## Reactive Entity Instance Builder

Constructs a new reactive entity instance programmatically.

```admonish tip "Register Reactive Entity Instance"
The reactive entity instance is fully functional, but not yet registered.

Register the reactive entity instance using `ReactiveEntityInstanceManager::register_reactive_instance`
```

## Reactive Relation Instance Builder

Constructs a new reactive relation instance programmatically.

```admonish tip "Register Reactive Relation Instance"
The reactive relation instance is fully functional, but not yet registered.

Register the reactive relation instance using `ReactiveRelationInstanceManager::register_reactive_instance`
```


## Flow Builder
