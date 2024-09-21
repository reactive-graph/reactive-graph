use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::instances::properties::display_property_instances_html_inline;
use crate::instances::properties::display_property_instances_inline_str;
use crate::instances::properties::PropertyInstance;
use crate::instances::properties::PropertyInstances;
use crate::types::component::display_component_type_ids_html_inline;
use crate::types::component::display_component_type_ids_inline_str;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::extension::display_extensions_html_inline;
use crate::types::extension::display_extensions_inline_str;
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

    /// The components.
    #[tabled(display_with("Self::display_component_type_ids", self))]
    pub components: Vec<ComponentTypeId>,

    /// The property instances.
    #[tabled(display_with("Self::display_property_instances", self))]
    pub properties: Vec<PropertyInstance>,

    /// The extensions.
    #[tabled(display_with("Self::display_extensions", self))]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

impl EntityInstance {
    fn display_component_type_ids(&self) -> String {
        match self.inline_format {
            TableInlineFormat::Table => display_component_type_ids_inline_str(&self.components),
            TableInlineFormat::Html => display_component_type_ids_html_inline(&self.components),
        }
    }

    fn display_property_instances(&self) -> String {
        match self.inline_format {
            TableInlineFormat::Table => display_property_instances_inline_str(&self.properties),
            TableInlineFormat::Html => display_property_instances_html_inline(&self.properties),
        }
    }

    fn display_extensions(&self) -> String {
        // println!("{:?}", &self.inline_format);
        match self.inline_format {
            TableInlineFormat::Table => display_extensions_inline_str(&self.extensions),
            TableInlineFormat::Html => display_extensions_html_inline(&self.extensions),
        }
    }
}

impl TableInlineFormatSetter for EntityInstance {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::EntityInstance> for EntityInstance {
    fn from(entity_instance: reactive_graph_graph::EntityInstance) -> Self {
        EntityInstance {
            id: entity_instance.id,
            namespace: entity_instance.namespace(),
            name: entity_instance.type_name(),
            description: entity_instance.description,
            components: ComponentTypeIds::from(entity_instance.components).0,
            properties: PropertyInstances::from(entity_instance.properties).0,
            extensions: Extensions::from(entity_instance.extensions).0,
            inline_format: Default::default(),
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
