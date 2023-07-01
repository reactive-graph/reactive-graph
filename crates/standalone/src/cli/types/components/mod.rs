use std::process::exit;
use std::sync::Arc;

use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;

use crate::cli::types::components::args::ComponentsArgs;
use crate::cli::types::components::commands::ComponentsCommands;
use crate::client::types::components::queries::CreateComponentVariables;
use crate::client::InexorRgfClient;
use crate::model::Component;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn components(client: &Arc<InexorRgfClient>, component_args: ComponentsArgs) {
    let Some(command) = component_args.commands else {
        eprintln!("[ERROR] Missing sub command");
        exit(255);
    };
    match command {
        ComponentsCommands::List => match client.types().components().get_all_components().await {
            Ok(Some(components)) => print_components(components),
            Ok(None) => println!("Components not found"),
            Err(e) => eprintln!("[ERROR] Failed to create component\n{e}"),
        },
        ComponentsCommands::Get(args) => match client.types().components().get_component_by_type(args).await {
            Ok(Some(component)) => print_component(component),
            Ok(None) => println!("Component not found"),
            Err(e) => eprintln!("[ERROR] Failed to create component\n{e}"),
        },
        ComponentsCommands::Create(args) => {
            let variables = CreateComponentVariables::builder()
                .namespace(args.ty.namespace)
                .name(args.ty.name)
                .description(args.description)
                // .properties(None)
                // .extensions(None)
                .build();
            match client.types().components().create_component_with_variables(variables).await {
                Ok(Some(component)) => print_component(component),
                Ok(None) => println!("Component not found"),
                Err(e) => {
                    eprintln!("[ERROR] Failed to create component\n{e}");
                }
            }
        }
    }
}

fn print_component(component: Component) {
    print_components(vec![component]);
}

fn print_components(components: Vec<Component>) {
    let components: Vec<crate::table_model::types::component::Component> = components.into_iter().map(|p| p.into()).collect();
    let table = Table::new(components)
        .with(Style::extended())
        .with(
            Modify::new(Segment::new(1.., 0..2))
                .with(Width::increase(22).priority())
                .with(Width::truncate(25).suffix("...")),
        )
        .to_string();
    println!("{}", table);
}
