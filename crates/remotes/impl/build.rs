use anyhow::Result;
// use vergen_gix::BuildBuilder;
use vergen_gix::CargoBuilder;
use vergen_gix::Emitter;
use vergen_gix::GixBuilder;
use vergen_gix::RustcBuilder;
use vergen_gix::SysinfoBuilder;

fn main() -> Result<()> {
    // Generate environment variables
    Emitter::default()
        // .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&GixBuilder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()
}
