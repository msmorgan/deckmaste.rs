use std::path::PathBuf;

use clap::Parser;

mod data;
mod layout;
mod migrations;
mod ron_output;

#[derive(Debug, Parser)]
struct Args {
    pub plugin_dir: PathBuf,
    pub migration_number: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.migration_number {
        Some(number) => migrations::apply(&args.plugin_dir, number),
        None => migrations::apply_all(&args.plugin_dir),
    }
}
