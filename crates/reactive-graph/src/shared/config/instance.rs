use clap::Parser;

#[derive(Parser, Debug)]
pub struct InstanceConfigArgs {
    /// The name of the instance.
    #[arg(short = 'n', long = "instance-name", env = "REACTIVE_GRAPH_INSTANCE_NAME")]
    pub name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long = "instance-description", env = "REACTIVE_GRAPH_INSTANCE_DESCRIPTION")]
    pub description: Option<String>,
}
