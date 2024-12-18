use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use lifestuff_types::Cli;
use std::env;
fn main() {
    let out_dir = env::var("RUST_COMPLETION_DIR").unwrap();
    let mut cmd = Cli::command();

    generate_to(Shell::Zsh, &mut cmd, "lifestuff", out_dir).unwrap();
}
