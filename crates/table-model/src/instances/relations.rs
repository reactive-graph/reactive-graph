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
pub struct RelationInstance {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The type namespace.
    pub namespace: String,

    /// The type name.
    pub name: String,

    /// The relation type instance id.
    pub instance_id: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,

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

fn display_component_type_ids(components: &[ComponentTypeId], relation_instance: &RelationInstance) -> String {
    match relation_instance.inline_format {
        TableInlineFormat::Table => display_component_type_ids_inline_str(components),
        TableInlineFormat::Html => display_component_type_ids_html_inline(components),
    }
}

fn display_property_instances(properties: &[PropertyInstance], relation_instance: &RelationInstance) -> String {
    match relation_instance.inline_format {
        TableInlineFormat::Table => display_property_instances_inline_str(properties),
        TableInlineFormat::Html => display_property_instances_html_inline(properties),
    }
}

fn display_extensions(extensions: &[Extension], relation_instance: &RelationInstance) -> String {
    match relation_instance.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for RelationInstance {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::RelationInstance> for RelationInstance {
    fn from(relation_instance: reactive_graph_graph::RelationInstance) -> Self {
        RelationInstance {
            outbound_id: relation_instance.outbound_id,
            namespace: relation_instance.path().to_string(),
            name: relation_instance.relation_type_id().type_name().to_string(),
            instance_id: relation_instance.instance_id().to_string(),
            inbound_id: relation_instance.inbound_id,
            description: relation_instance.description,
            properties: PropertyInstances::from(relation_instance.properties).0,
            components: ComponentTypeIds::from(relation_instance.components).0,
            extensions: Extensions::from(relation_instance.extensions).0,
            inline_format: Default::default(),
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

pub fn display_relation_instances_inline_str(relation_instances: &[RelationInstance]) -> String {
    if relation_instances.is_empty() {
        String::new()
    } else {
        display_relation_instances_inline(relation_instances).to_string()
    }
}

pub fn display_relation_instances_inline(relation_instances: &[RelationInstance]) -> Table {
    let relation_instances = relation_instances.to_vec();
    Table::new(relation_instances)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(22)))
        // .with(Modify::new(Columns::new(1..2)).with(Width::increase(22)))
        // .with(Modify::new(Columns::new(2..3)).with(Width::wrap(40)))
        // .with(Modify::new(Columns::new(3..4)).with(Width::wrap(80)))
        .to_owned()
}

pub fn display_relation_instances_html_inline(relation_instances: &[RelationInstance]) -> String {
    let relation_instances = relation_instances.to_vec();
    if relation_instances.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&relation_instances)))
        .to_string()
        .split_whitespace()
        .collect()
}
