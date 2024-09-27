use clap::Command;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManPagesGenerationError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(#[from] FromUtf8Error),
}

fn generate_man_pages(cmd: Command) -> Result<String, ManPagesGenerationError> {
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_page = String::from_utf8(buffer).map_err(ManPagesGenerationError::Utf8)?;
    Ok(man_page)
}

pub fn print_man_pages(cmd: Command) -> Result<(), ManPagesGenerationError> {
    let man_page = generate_man_pages(cmd)?;
    println!("{man_page}");
    Ok(())
}

pub fn install_man_pages(cmd: Command) -> Result<(), ManPagesGenerationError> {
    let bin_name = cmd.get_name().to_string();
    let path: PathBuf = format!("/usr/share/man/man1/{bin_name}.1").into();

    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(ManPagesGenerationError::Io)?;
    }

    eprintln!("Writing man pages to {}", path.display());

    let man_page = generate_man_pages(cmd)?;
    std::fs::write(path, man_page).map_err(ManPagesGenerationError::Io)?;
    Ok(())
}
