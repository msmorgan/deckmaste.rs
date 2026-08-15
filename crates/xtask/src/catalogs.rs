//! Derive the catalog files consumed by deckmaste from local authoritative
//! sources, without querying Scryfall.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use deckmaste_catalogs::CatalogSet;
use deckmaste_catalogs::compare_directories;
use deckmaste_catalogs::legacy::LegacyCatalogSet;

#[derive(Debug, Args)]
pub struct CatalogArgs {
    #[command(subcommand)]
    command: CatalogCommand,
}

#[derive(Debug, Subcommand)]
enum CatalogCommand {
    /// Generate the canonical catalogs consumed by `english_v2`.
    Generate(GenerateArgs),
    /// Check the canonical catalogs against freshly generated output.
    Check(CheckArgs),
    /// Generate legacy bare-text catalogs for legacy consumers.
    Text(TextArgs),
}

#[derive(Debug, Args)]
struct GenerateArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// MTGJSON atomic card snapshot used only for observed keyword variants.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    atomic: PathBuf,

    /// Directory for the canonical catalogs.
    #[arg(long, default_value = "data/gen/catalogs")]
    output: PathBuf,
}

#[derive(Debug, Args)]
struct CheckArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// MTGJSON atomic card snapshot used for card names.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    atomic: PathBuf,

    /// Canonical catalog directory to check.
    #[arg(long, default_value = "data/gen/catalogs")]
    checked: PathBuf,
}

#[derive(Debug, Args)]
struct TextArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// MTGJSON atomic card snapshot used only for observed keyword variants.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    atomic: PathBuf,

    /// Directory for the twelve legacy bare-text catalogs.
    #[arg(long, default_value = "data/gen/catalogs-legacy")]
    output: PathBuf,
}

pub fn run(args: &CatalogArgs) -> anyhow::Result<()> {
    match &args.command {
        CatalogCommand::Generate(args) => generate(args),
        CatalogCommand::Check(args) => check(args),
        CatalogCommand::Text(args) => text(args),
    }
}

fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
    generate_catalogs(&args.cr, &args.atomic)?.write_to(&args.output)
}

fn check(args: &CheckArgs) -> anyhow::Result<()> {
    let temp = tempfile::tempdir().context("creating catalog check directory")?;
    let generated = temp.path().join("catalogs");
    generate_catalogs(&args.cr, &args.atomic)?.write_to(&generated)?;
    let diff = compare_directories(&generated, &args.checked)?;
    if diff.is_empty() {
        println!("catalogs are up to date");
        Ok(())
    } else {
        anyhow::bail!("catalogs differ:\n{diff}")
    }
}

fn text(args: &TextArgs) -> anyhow::Result<()> {
    let cr =
        fs::read_to_string(&args.cr).with_context(|| format!("reading {}", args.cr.display()))?;
    let atomic =
        fs::read(&args.atomic).with_context(|| format!("reading {}", args.atomic.display()))?;
    LegacyCatalogSet::generate(&cr, &atomic)?.write_to(&args.output)
}

fn generate_catalogs(cr_path: &Path, atomic_path: &Path) -> anyhow::Result<CatalogSet> {
    let cr =
        fs::read_to_string(cr_path).with_context(|| format!("reading {}", cr_path.display()))?;
    let atomic =
        fs::read(atomic_path).with_context(|| format!("reading {}", atomic_path.display()))?;
    CatalogSet::generate(&cr, &atomic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    const CR_FIXTURE: &str = "\
205.2a The card types are artifact, creature, land, and sorcery.\n\
205.3g Artifacts have their own unique set of subtypes; these subtypes are called artifact types. The artifact types are Clue, and Vibranium.\n\
205.3h Enchantments have their own unique set of subtypes; these subtypes are called enchantment types. The enchantment types are Aura, and Saga.\n\
205.3i Lands have their own unique set of subtypes; these subtypes are called land types. The land types are Forest, Island, and Urza’s. Of that list, Forest is a basic land type.\n\
205.3j Planeswalkers have their own unique set of subtypes; these subtypes are called planeswalker types. The planeswalker types are Ajani, and Jace.\n\
205.3k Instants and sorceries share their lists of subtypes; these subtypes are called spell types. The spell types are Arcane, and Trap.\n\
205.3m Creatures and kindreds share their lists of subtypes; these subtypes are called creature types. One creature type is two words long: Time Lord. All other creature types are one word long: Advisor.\n\
205.3q Battles have a unique subtype, called a battle type. That battle type is Siege.\n\
205.4a An object can have one or more supertypes. A card’s supertypes are printed directly before its card types. The supertypes are basic, legendary, and snow.\n\
207.2c An ability word appears in italics at the beginning of some abilities. The ability words are landfall, and threshold.\n\
701.1. Keyword action introduction\n\
701.2. Scry\n\
701.3. Tap and Untap\n\
702.1. Keyword ability introduction\n\
702.2. Daybound and Nightbound\n\
702.3. ∞ (Infinity)\n\
122.1b A keyword counter on a permanent or on a card in a zone other than the battlefield causes that object to gain that keyword. The keywords that a keyword counter can be are flying, first strike, double strike, deathtouch, and hexproof, as well as any variants of those keywords.\n";

    const ATOMIC_FIXTURE: &str = r#"{
        "data": {
            "Legal Name": [{
                "name": "Legal Name", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "keywords": [], "legalities": {"vintage": "Legal"}
            }]
        }
    }"#;

    #[test]
    fn generate_writes_the_complete_canonical_catalog_inventory() {
        let root = tempfile::tempdir().unwrap();
        let (cr, atomic) = write_sources(root.path());
        let output = root.path().join("canonical");

        run(&generate_args(cr, atomic, output.clone())).unwrap();

        assert_eq!(
            directory_bytes(&output).keys().cloned().collect::<Vec<_>>(),
            [
                "ability-words.txt",
                "artifact-types.txt",
                "battle-types.txt",
                "card-names.txt",
                "card-types.txt",
                "counter-kind-phrases.txt",
                "creature-types.txt",
                "enchantment-types.txt",
                "keyword-abilities.txt",
                "keyword-actions.txt",
                "land-types.txt",
                "planeswalker-types.txt",
                "spell-types.txt",
                "supertypes.txt",
            ]
            .map(PathBuf::from)
            .to_vec()
        );
    }

    #[test]
    fn check_accepts_an_unchanged_canonical_catalog_directory() {
        let root = tempfile::tempdir().unwrap();
        let (cr, atomic) = write_sources(root.path());
        let output = root.path().join("canonical");
        run(&generate_args(cr.clone(), atomic.clone(), output.clone())).unwrap();

        run(&check_args(cr, atomic, output)).unwrap();
    }

    #[test]
    fn check_reports_changed_missing_and_unexpected_entries_without_mutating_them() {
        for mutation in ["changed", "missing", "unexpected"] {
            let root = tempfile::tempdir().unwrap();
            let (cr, atomic) = write_sources(root.path());
            let output = root.path().join("canonical");
            run(&generate_args(cr.clone(), atomic.clone(), output.clone())).unwrap();

            match mutation {
                "changed" => fs::write(output.join("ability-words.txt"), b"edited\n").unwrap(),
                "missing" => fs::remove_file(output.join("ability-words.txt")).unwrap(),
                "unexpected" => fs::write(output.join("unexpected.txt"), b"extra\n").unwrap(),
                _ => unreachable!(),
            }
            let before = directory_bytes(&output);

            let error = run(&check_args(cr, atomic, output.clone())).unwrap_err();
            assert!(
                error.to_string().starts_with("catalogs differ:"),
                "{mutation}"
            );
            assert_eq!(directory_bytes(&output), before, "{mutation}");
        }
    }

    #[test]
    fn text_writes_the_twelve_legacy_catalog_files_to_its_own_directory() {
        let root = tempfile::tempdir().unwrap();
        let (cr, atomic) = write_sources(root.path());
        let output = root.path().join("legacy");

        run(&text_args(cr, atomic, output.clone())).unwrap();

        assert_eq!(
            directory_bytes(&output).keys().cloned().collect::<Vec<_>>(),
            [
                "ability-words.txt",
                "artifact-types.txt",
                "battle-types.txt",
                "card-types.txt",
                "creature-types.txt",
                "enchantment-types.txt",
                "keyword-abilities.txt",
                "keyword-actions.txt",
                "land-types.txt",
                "planeswalker-types.txt",
                "spell-types.txt",
                "supertypes.txt",
            ]
            .map(PathBuf::from)
            .to_vec()
        );
    }

    fn write_sources(root: &Path) -> (PathBuf, PathBuf) {
        let cr = root.join("cr.txt");
        let atomic = root.join("AtomicCards.json");
        fs::write(&cr, CR_FIXTURE).unwrap();
        fs::write(&atomic, ATOMIC_FIXTURE).unwrap();
        (cr, atomic)
    }

    fn generate_args(cr: PathBuf, atomic: PathBuf, output: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Generate(GenerateArgs { cr, atomic, output }),
        }
    }

    fn check_args(cr: PathBuf, atomic: PathBuf, checked: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Check(CheckArgs {
                cr,
                atomic,
                checked,
            }),
        }
    }

    fn text_args(cr: PathBuf, atomic: PathBuf, output: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Text(TextArgs { cr, atomic, output }),
        }
    }

    fn directory_bytes(directory: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        fs::read_dir(directory)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (
                    PathBuf::from(path.file_name().unwrap()),
                    fs::read(path).unwrap(),
                )
            })
            .collect()
    }
}
