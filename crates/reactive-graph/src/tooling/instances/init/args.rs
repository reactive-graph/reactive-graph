use crate::tooling::instances::args::ChownArgs;
use crate::tooling::instances::certificates::args::GenerateCertificateArgs;
use crate::tooling::releases::args::ReleaseArgs;
use crate::tooling::repository::args::RepositoryArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InitInstanceArgs {
    #[clap(flatten)]
    pub chown: ChownArgs,

    #[clap(flatten)]
    pub certificate: GenerateCertificateArgs,

    #[clap(flatten)]
    pub release: ReleaseArgs,

    #[clap(flatten)]
    pub repository: RepositoryArgs,
}
