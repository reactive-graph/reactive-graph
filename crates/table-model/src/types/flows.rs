use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::instances::entities::EntityInstance;
use crate::instances::entities::display_entity_instances_html_inline;
use crate::instances::entities::display_entity_instances_inline_str;
use crate::instances::relations::RelationInstance;
use crate::instances::relations::display_relation_instances_html_inline;
use crate::instances::relations::display_relation_instances_inline_str;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::extension::display_extensions_html_inline;
use crate::types::extension::display_extensions_inline_str;
use crate::types::properties::display_property_types_html_inline;
use crate::types::properties::display_property_types_inline_str;
use crate::types::variables::Variables;
use reactive_graph_graph::NamespacedTypeGetter;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Segment;

#[derive(Clone, Debug, Tabled)]
pub struct FlowType {
    /// The namespace of the flow type.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The wrapper entity instance.
    #[tabled(skip)]
    pub wrapper_entity_instance: EntityInstance,

    /// The entity instances.
    // [tabled(display("Self::display_entity_instances", self))]
    #[tabled(skip)]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances.
    // [tabled(display("Self::display_relation_instances", self))]
    #[tabled(skip)]
    pub relation_instances: Vec<RelationInstance>,

    // pub wrapper_entity_instance: EntityInstance,
    /// The variables.
    #[tabled(display("display_variables", self))]
    pub variables: Variables,

    /// The extensions.
    #[tabled(display("display_extensions", self))]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

#[allow(unused)]
fn display_entity_instances(entity_instances: &[EntityInstance], flow_type: &FlowType) -> String {
    match flow_type.inline_format {
        TableInlineFormat::Table => display_entity_instances_inline_str(entity_instances),
        TableInlineFormat::Html => display_entity_instances_html_inline(entity_instances),
    }
}

#[allow(unused)]
fn display_relation_instances(relation_instances: &[RelationInstance], flow_type: &FlowType) -> String {
    match flow_type.inline_format {
        TableInlineFormat::Table => display_relation_instances_inline_str(relation_instances),
        TableInlineFormat::Html => display_relation_instances_html_inline(relation_instances),
    }
}

fn display_variables(variables: &Variables, flow_type: &FlowType) -> String {
    match flow_type.inline_format {
        TableInlineFormat::Table => display_property_types_inline_str(&variables.0),
        TableInlineFormat::Html => display_property_types_html_inline(&variables.0),
    }
}

fn display_extensions(extensions: &[Extension], flow_type: &FlowType) -> String {
    match flow_type.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for FlowType {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::FlowType> for FlowType {
    fn from(flow_type: reactive_graph_graph::FlowType) -> Self {
        // let x = flow_type.wrapper_entity_instance;
        let namespace = flow_type.namespace();
        let name = flow_type.type_name();
        let variables = Variables::from(flow_type.variables.clone());
        let entity_instances = flow_type
            .entity_instances
            .into_iter()
            .map(|(_, entity_instance)| entity_instance.into())
            .collect();
        let relation_instances = flow_type
            .relation_instances
            .into_iter()
            .map(|(_, relation_instance)| relation_instance.into())
            .collect();
        FlowType {
            namespace,
            name,
            description: flow_type.description,
            wrapper_entity_instance: flow_type.wrapper_entity_instance.into(),
            entity_instances,
            relation_instances,
            variables,
            extensions: Extensions::from(flow_type.extensions).0,
            inline_format: Default::default(),
        }
    }
}

pub struct FlowTypesTableOptions;

impl TableOptions for FlowTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
