use clap::Parser;

#[derive(Parser, Debug)]
pub struct InstanceConfigArguments {
    // Instance
    /// The name of the instance.
    #[arg(short = 'n', long, env = "REACTIVE_GRAPH_INSTANCE_NAME")]
    pub instance_name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long, env = "REACTIVE_GRAPH_INSTANCE_DESCRIPTION")]
    pub instance_description: Option<String>,
}
