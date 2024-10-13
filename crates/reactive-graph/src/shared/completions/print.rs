use clap::Command;
use clap_complete::generate;
use clap_complete::Generator;

pub fn print_shell_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
