use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use reactive_graph_graph::NamespacedTypeGetter;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Segment;

#[derive(Clone, Debug, Tabled)]
pub struct FlowInstance {
    /// The type namespace.
    pub namespace: String,

    /// The type name.
    pub name: String,

    /// Textual description of the flow instance.
    pub description: String,

    // pub extensions: Vec<Extension>,
    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

impl TableInlineFormatSetter for FlowInstance {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::FlowInstance> for FlowInstance {
    fn from(flow_instance: reactive_graph_graph::FlowInstance) -> Self {
        FlowInstance {
            namespace: flow_instance.path().to_string(),
            name: flow_instance.type_name().to_string(),
            description: flow_instance.description,
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
