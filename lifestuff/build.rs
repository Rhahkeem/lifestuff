use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use lifestuff_types::Cli;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("RUST_COMPLETION_DIR").unwrap_or_else(|_| {
        let target_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
        let completion_dir = target_dir.join("completion");
        fs::create_dir_all(&completion_dir).expect("Failed to create completion directory");
        completion_dir.to_string_lossy().to_string()
    });

    let mut cmd = Cli::command();
    generate_to(Shell::Zsh, &mut cmd, "lifestuff", &out_dir)
        .expect("Failed to generate completions");
}
