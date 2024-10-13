use clap::Parser;

#[cfg(target_os = "linux")]
#[derive(Parser, Debug)]
pub struct InstanceConfigArguments {
    // Instance
    /// The name of the instance.
    #[arg(short = 'n', long, env = "REACTIVE_GRAPH_INSTANCE_NAME")]
    pub(crate) instance_name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long, env = "REACTIVE_GRAPH_INSTANCE_DESCRIPTION")]
    pub(crate) instance_description: Option<String>,
}
