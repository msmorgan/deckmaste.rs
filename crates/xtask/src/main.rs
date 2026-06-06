//! Workspace automation: a pure dispatcher over the other crates' CLI
//! entry points — no subcommand logic of its own.

use std::ffi::{OsStr, OsString};

const SUBCOMMANDS: &str = "validate | migrate | card";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os();
    let _argv0 = args.next();
    let subcommand = args.next();
    // Entry points expect full argv; synthesize the program name the
    // subcommand was invoked as so clap's usage/errors read right.
    let prog = |name: &str| std::iter::once(OsString::from(format!("cargo xtask {name}")));

    match subcommand.as_deref().and_then(OsStr::to_str) {
        Some("validate") => deckmaste_cards::cli::validate(prog("validate").chain(args)),
        Some("card") => deckmaste_cards::cli::card(prog("card").chain(args)),
        Some("migrate") => deckmaste_migrations::cli::run(prog("migrate").chain(args)),
        Some(other) => anyhow::bail!("unknown subcommand {other:?}; expected {SUBCOMMANDS}"),
        // A subcommand that isn't UTF-8 is unknown, not missing.
        None if subcommand.is_some() => {
            anyhow::bail!("unknown subcommand {subcommand:?}; expected {SUBCOMMANDS}")
        }
        None => anyhow::bail!("usage: cargo xtask <{SUBCOMMANDS}> [args...]"),
    }
}
