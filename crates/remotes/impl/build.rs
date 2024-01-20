use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate environment variables
    EmitBuilder::builder().build_date().git_branch().git_sha(false).rustc_semver().emit()?;
    Ok(())
}
