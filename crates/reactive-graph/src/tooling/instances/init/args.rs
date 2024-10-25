use crate::tooling::instances::args::ChownArgs;
use crate::tooling::instances::certificates::args::GenerateCertificateArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InitInstanceArgs {
    #[clap(flatten)]
    pub chown: ChownArgs,

    #[clap(flatten)]
    pub certificate: GenerateCertificateArgs,
}
