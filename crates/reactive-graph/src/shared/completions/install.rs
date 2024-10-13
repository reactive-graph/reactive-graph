use crate::shared::completions::error::InstallShellCompletionError;
use clap::Command;
use clap_complete::generate;
use clap_complete::Generator;
use clap_complete::Shell;
use std::fs::create_dir_all;

pub fn install_shell_completions<G: Generator>(gen: G, shell: Shell, cmd: &mut Command) -> Result<(), InstallShellCompletionError> {
    let bin_name = cmd.get_name().to_string();

    let path = match shell {
        Shell::Fish => {
            let dirs = xdg::BaseDirectories::new().map_err(InstallShellCompletionError::BaseDirectories)?;
            dirs.place_config_file(format!("fish/completions/{bin_name}.fish"))
                .map_err(InstallShellCompletionError::Io)?
        }
        Shell::Bash => format!("/usr/share/bash-completion/completions/{bin_name}").into(),
        Shell::Zsh => format!("/usr/share/zsh/functions/Completion/Base/_{bin_name}").into(),
        _ => {
            return Err(InstallShellCompletionError::UnsupportedShell(shell));
        }
    };

    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(InstallShellCompletionError::Io)?;
    }

    eprintln!("Writing completions to {}", path.display());

    let mut buffer = Vec::with_capacity(512);
    generate(gen, cmd, &bin_name, &mut buffer);
    std::fs::write(path, buffer).map_err(InstallShellCompletionError::Io)?;
    Ok(())
}
