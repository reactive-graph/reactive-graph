use clap::Parser;

#[derive(Parser, Debug)]
pub struct GenerateCertificateArgs {
    /// Country name.
    pub country_name: Option<String>,

    /// Organization name.
    pub organization_name: Option<String>,

    /// Common name.
    pub common_name: Option<String>,
    // #[command(subcommand)]
    // pub commands: GenerateCertificateCommands,
}

impl Default for GenerateCertificateArgs {
    fn default() -> Self {
        Self {
            country_name: Some(String::from("io")),
            organization_name: Some(String::from("reactive-graph")),
            common_name: Some(String::from("localhost")),
        }
    }
}
