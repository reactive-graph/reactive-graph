use clap::Command;
use clap_complete::Generator;
use clap_complete::generate;

pub fn print_shell_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
