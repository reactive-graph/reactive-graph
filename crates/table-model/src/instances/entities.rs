use crate::container::TableOptions;
use crate::instances::properties::display_property_instances_inline;
use crate::instances::properties::PropertyInstance;
use crate::instances::properties::PropertyInstances;
use crate::types::component::display_component_type_ids_inline;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::extension::display_extensions_inline;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use reactive_graph_graph::NamespacedTypeGetter;
use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;
use uuid::Uuid;

#[derive(Clone, Debug, Tabled)]
pub struct EntityInstance {
    /// The id of the entity instance.
    pub id: Uuid,

    /// The type namespace.
    pub namespace: String,

    /// The type name.
    pub name: String,

    /// Textual description of the entity instance.
    pub description: String,

    /// The property types.
    #[tabled(display_with("display_property_instances_inline"))]
    pub properties: Vec<PropertyInstance>,

    /// The property types.
    #[tabled(display_with("display_component_type_ids_inline"))]
    pub components: Vec<ComponentTypeId>,

    /// The extensions.
    #[tabled(display_with("display_extensions_inline"))]
    pub extensions: Vec<Extension>,
}

impl From<reactive_graph_graph::EntityInstance> for EntityInstance {
    fn from(entity_instance: reactive_graph_graph::EntityInstance) -> Self {
        EntityInstance {
            id: entity_instance.id,
            namespace: entity_instance.namespace(),
            name: entity_instance.type_name(),
            description: entity_instance.description,
            properties: PropertyInstances::from(entity_instance.properties).0,
            components: ComponentTypeIds::from(entity_instance.components).0,
            extensions: Extensions::from(entity_instance.extensions).0,
        }
    }
}

pub struct EntityInstancesTableOptions;

impl TableOptions for EntityInstancesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
