use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::instances::properties::PropertyInstance;
use crate::instances::properties::PropertyInstances;
use crate::instances::properties::display_property_instances_html_inline;
use crate::instances::properties::display_property_instances_inline_str;
use crate::styles::modern_inline::modern_inline;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::component::display_component_type_ids_html_inline;
use crate::types::component::display_component_type_ids_inline_str;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::extension::display_extensions_html_inline;
use crate::types::extension::display_extensions_inline_str;
use reactive_graph_graph::NamespacedTypeGetter;
use table_to_html::HtmlTable;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Columns;
use tabled::settings::object::Segment;
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
    #[tabled(display("display_component_type_ids", self))]
    pub components: Vec<ComponentTypeId>,

    /// The property instances.
    #[tabled(display("display_property_instances", self))]
    pub properties: Vec<PropertyInstance>,

    /// The extensions.
    #[tabled(display("display_extensions", self))]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

fn display_component_type_ids(components: &[ComponentTypeId], entity_instance: &EntityInstance) -> String {
    match entity_instance.inline_format {
        TableInlineFormat::Table => display_component_type_ids_inline_str(components),
        TableInlineFormat::Html => display_component_type_ids_html_inline(components),
    }
}

fn display_property_instances(properties: &[PropertyInstance], entity_instance: &EntityInstance) -> String {
    match entity_instance.inline_format {
        TableInlineFormat::Table => display_property_instances_inline_str(properties),
        TableInlineFormat::Html => display_property_instances_html_inline(properties),
    }
}

fn display_extensions(extensions: &[Extension], entity_instance: &EntityInstance) -> String {
    // println!("{:?}", &self.inline_format);
    match entity_instance.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

// impl EntityInstance {
// }

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

pub fn display_entity_instances_inline_str(entity_instances: &[EntityInstance]) -> String {
    if entity_instances.is_empty() {
        String::new()
    } else {
        display_entity_instances_inline(entity_instances).to_string()
    }
}

pub fn display_entity_instances_inline(entity_instances: &[EntityInstance]) -> Table {
    let entity_instances = entity_instances.to_vec();
    Table::new(entity_instances)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(22)))
        // .with(Modify::new(Columns::new(1..2)).with(Width::increase(22)))
        // .with(Modify::new(Columns::new(2..3)).with(Width::wrap(40)))
        // .with(Modify::new(Columns::new(3..4)).with(Width::wrap(80)))
        .to_owned()
}

pub fn display_entity_instances_html_inline(entity_instances: &[EntityInstance]) -> String {
    let entity_instances = entity_instances.to_vec();
    if entity_instances.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&entity_instances)))
        .to_string()
        .split_whitespace()
        .collect()
}
