use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::container::TableOptions;
use crate::types::component::display_component_type_ids_inline;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::extension::display_extensions_inline;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::property_type::display_property_types_inline;
use crate::types::property_type::PropertyType;
use crate::types::property_type::PropertyTypes;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub struct RelationType {
    /// The namespace of the outbound entity type.
    pub outbound_type_namespace: String,

    /// The name of the outbound entity type.
    pub outbound_type_name: String,

    /// The namespace of the relation type.
    pub namespace: String,

    /// The name of the relation type.
    pub name: String,

    /// The namespace of the inbound entity type.
    pub inbound_type_namespace: String,

    /// The name of the inbound entity type.
    pub inbound_type_name: String,

    /// Textual description of the relation type.
    // #[tabled(skip)]
    pub description: String,

    /// The components.
    #[tabled(display_with("display_component_type_ids_inline"))]
    pub components: Vec<ComponentTypeId>,

    /// The property types.
    #[tabled(display_with("display_property_types_inline"))]
    pub properties: Vec<PropertyType>,

    /// The extensions.
    #[tabled(display_with("display_extensions_inline"))]
    pub extensions: Vec<Extension>,
}

impl From<reactive_graph_graph::RelationType> for RelationType {
    fn from(relation_type: reactive_graph_graph::RelationType) -> Self {
        RelationType {
            outbound_type_namespace: relation_type.outbound_type.namespace(),
            outbound_type_name: relation_type.outbound_type.type_name(),
            namespace: relation_type.namespace(),
            name: relation_type.type_name(),
            inbound_type_namespace: relation_type.inbound_type.namespace(),
            inbound_type_name: relation_type.inbound_type.type_name(),
            description: relation_type.description,
            components: ComponentTypeIds::from(relation_type.components).0,
            properties: PropertyTypes::from(relation_type.properties).0,
            extensions: Extensions::from(relation_type.extensions).0,
        }
    }
}

pub struct RelationTypesTableOptions;

impl TableOptions for RelationTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}