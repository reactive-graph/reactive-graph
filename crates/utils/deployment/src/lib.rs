use serde::Deserialize;
use std::env::VarError;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Deserialize)]
struct Deployment {
    pub target_dirs: Vec<String>,
}

#[derive(Debug, Error)]
pub enum DeploymentError {
    #[error("Could not read .deployment.toml: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Could not parse .deployment.toml: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Could not read env var: {0}")]
    EnvVarError(#[from] VarError),
}

pub fn deploy_plugin(filename: &str) -> Result<(), DeploymentError> {
    let toml_string = fs::read_to_string("./.deployment.toml")?;
    let deployment: Deployment = toml::from_str(&toml_string)?;
    let mut crate_out_dir = var("CRATE_OUT_DIR")?;
    crate_out_dir.push_str("/");
    crate_out_dir.push_str(filename);
    for target_dir in deployment.target_dirs {
        for entry in glob::glob(crate_out_dir.as_str()).unwrap() {
            if let Ok(source_path) = entry {
                let file_name = source_path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".so") || file_name.ends_with(".dll") {
                    let mut target_path = PathBuf::from(&target_dir);
                    target_path.push(file_name);
                    println!("Copy plugin from {} to {}", source_path.display(), target_path.display());
                    match fs::copy(source_path, target_path) {
                        Ok(_) => println!("Success"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }
    Ok(())
}
