use crate::table_model::container::TableOptions;
use crate::table_model::instances::properties::display_property_instances_inline;
use crate::table_model::instances::properties::PropertyInstance;
use crate::table_model::instances::properties::PropertyInstances;
use crate::table_model::types::extension::display_extensions_inline;
use crate::table_model::types::extension::Extension;
use crate::table_model::types::extension::Extensions;
use reactive_graph_graph::NamespacedTypeGetter;
use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;
use uuid::Uuid;

#[derive(Clone, Debug, Tabled)]
pub(crate) struct EntityInstance {
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
            extensions: Extensions::from(entity_instance.extensions).0,
        }
    }
}

pub(crate) struct EntityInstancesTableOptions;

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
