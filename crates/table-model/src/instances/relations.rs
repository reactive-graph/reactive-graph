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
pub struct RelationInstance {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The type namespace.
    pub namespace: String,

    /// The type name.
    pub name: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,

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

impl From<reactive_graph_graph::RelationInstance> for RelationInstance {
    fn from(relation_instance: reactive_graph_graph::RelationInstance) -> Self {
        RelationInstance {
            outbound_id: relation_instance.outbound_id,
            namespace: relation_instance.namespace(),
            name: relation_instance.type_name(),
            inbound_id: relation_instance.inbound_id,
            description: relation_instance.description,
            properties: PropertyInstances::from(relation_instance.properties).0,
            components: ComponentTypeIds::from(relation_instance.components).0,
            extensions: Extensions::from(relation_instance.extensions).0,
        }
    }
}

pub struct RelationInstancesTableOptions;

impl TableOptions for RelationInstancesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
