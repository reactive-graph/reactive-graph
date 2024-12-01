use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use reactive_graph_graph::NamespacedTypeGetter;
use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

#[derive(Clone, Debug, Tabled)]
pub struct FlowInstance {
    /// The type namespace.
    pub namespace: String,

    /// The type name.
    pub name: String,

    /// Textual description of the flow instance.
    pub description: String,

    // /// The components.
    // #[tabled(display_with("Self::display_component_type_ids", self))]
    // pub components: Vec<ComponentTypeId>,
    //
    // /// The property instances.
    // #[tabled(display_with("Self::display_property_instances", self))]
    // pub properties: Vec<PropertyInstance>,
    //
    // /// The extensions.
    // #[tabled(display_with("Self::display_extensions", self))]
    // pub extensions: Vec<Extension>,
    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

impl FlowInstance {
    // fn display_component_type_ids(&self) -> String {
    //     match self.inline_format {
    //         TableInlineFormat::Table => display_component_type_ids_inline_str(&self.components),
    //         TableInlineFormat::Html => display_component_type_ids_html_inline(&self.components),
    //     }
    // }
    //
    // fn display_property_instances(&self) -> String {
    //     match self.inline_format {
    //         TableInlineFormat::Table => display_property_instances_inline_str(&self.properties),
    //         TableInlineFormat::Html => display_property_instances_html_inline(&self.properties),
    //     }
    // }
    //
    // fn display_extensions(&self) -> String {
    //     // println!("{:?}", &self.inline_format);
    //     match self.inline_format {
    //         TableInlineFormat::Table => display_extensions_inline_str(&self.extensions),
    //         TableInlineFormat::Html => display_extensions_html_inline(&self.extensions),
    //     }
    // }
}

impl TableInlineFormatSetter for FlowInstance {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::FlowInstance> for FlowInstance {
    fn from(flow_instance: reactive_graph_graph::FlowInstance) -> Self {
        FlowInstance {
            namespace: flow_instance.namespace(),
            name: flow_instance.type_name(),
            description: flow_instance.description,
            // properties: PropertyInstances::from(flow_instance.properties).0,
            // components: ComponentTypeIds::from(flow_instance.components).0,
            // extensions: Extensions::from(flow_instance.extensions).0,
            inline_format: Default::default(),
        }
    }
}

pub struct FlowInstancesTableOptions;

impl TableOptions for FlowInstancesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
