//! Derive the catalog files consumed by deckmaste from local authoritative
//! sources, without querying Scryfall.

use std::fs;
use std::fs::File;
use std::io::BufReader;
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
    /// Generate the canonical catalogs the English tooling consumes.
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

    /// Scryfall Oracle Cards JSONL snapshot supplying Vintage-playable card
    /// face names.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    oracle_cards: PathBuf,

    /// Directory for the canonical catalogs.
    #[arg(long, default_value = "data/gen/catalogs")]
    output: PathBuf,
}

#[derive(Debug, Args)]
struct CheckArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// Scryfall Oracle Cards JSONL snapshot used for card names.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    oracle_cards: PathBuf,

    /// Canonical catalog directory to check.
    #[arg(long, default_value = "data/gen/catalogs")]
    checked: PathBuf,
}

#[derive(Debug, Args)]
struct TextArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// Scryfall Oracle Cards JSONL snapshot used for observed keyword variants.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    oracle_cards: PathBuf,

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
    generate_catalogs(&args.cr, &args.oracle_cards)?.write_to(&args.output)
}

fn check(args: &CheckArgs) -> anyhow::Result<()> {
    let temp = tempfile::tempdir().context("creating catalog check directory")?;
    let generated = temp.path().join("catalogs");
    generate_catalogs(&args.cr, &args.oracle_cards)?.write_to(&generated)?;
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
    let oracle_cards = File::open(&args.oracle_cards)
        .with_context(|| format!("opening {}", args.oracle_cards.display()))?;
    LegacyCatalogSet::generate(&cr, BufReader::new(oracle_cards))
        .with_context(|| {
            format!(
                "extracting legacy catalogs from CR {}; keyword augmentation source {}",
                args.cr.display(),
                args.oracle_cards.display()
            )
        })?
        .write_to(&args.output)
}

fn generate_catalogs(cr_path: &Path, oracle_cards_path: &Path) -> anyhow::Result<CatalogSet> {
    let cr =
        fs::read_to_string(cr_path).with_context(|| format!("reading {}", cr_path.display()))?;
    let oracle_cards = File::open(oracle_cards_path)
        .with_context(|| format!("opening {}", oracle_cards_path.display()))?;
    CatalogSet::generate(&cr, BufReader::new(oracle_cards)).with_context(|| {
        format!(
            "extracting canonical catalogs from CR {}; card-names source {}",
            cr_path.display(),
            oracle_cards_path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

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

    const ORACLE_FIXTURE: &str = concat!(
        r#"{"object":"card","id":"printing-legal","oracle_id":"oracle-legal","name":"Legal Name","layout":"normal","type_line":"Creature","keywords":[],"legalities":{"vintage":"legal"}}"#,
        "\n",
    );

    #[test]
    fn generate_writes_the_complete_canonical_catalog_inventory() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        let output = root.path().join("canonical");

        run(&generate_args(cr, oracle, output.clone())).unwrap();

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
    fn generate_command_creates_a_missing_output_parent() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        let output = root.path().join("missing/gen/catalogs");

        run(&generate_args(cr, oracle, output.clone())).unwrap();

        assert_eq!(directory_bytes(&output).len(), 14);
    }

    #[test]
    fn check_accepts_an_unchanged_canonical_catalog_directory() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        let output = root.path().join("canonical");
        run(&generate_args(cr.clone(), oracle.clone(), output.clone())).unwrap();

        run(&check_args(cr, oracle, output)).unwrap();
    }

    #[test]
    fn check_reports_changed_missing_and_unexpected_entries_without_mutating_them() {
        for mutation in ["changed", "missing", "unexpected"] {
            let root = tempfile::tempdir().unwrap();
            let (cr, oracle) = write_sources(root.path());
            let output = root.path().join("canonical");
            run(&generate_args(cr.clone(), oracle.clone(), output.clone())).unwrap();

            match mutation {
                "changed" => fs::write(output.join("ability-words.txt"), b"edited\n").unwrap(),
                "missing" => fs::remove_file(output.join("ability-words.txt")).unwrap(),
                "unexpected" => fs::write(output.join("unexpected.txt"), b"extra\n").unwrap(),
                _ => unreachable!(),
            }
            let before = directory_bytes(&output);

            let error = run(&check_args(cr, oracle, output.clone())).unwrap_err();
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
        let (cr, oracle) = write_sources(root.path());
        let output = root.path().join("legacy");

        run(&text_args(cr, oracle, output.clone())).unwrap();

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

    #[test]
    fn canonical_oracle_errors_name_card_names_and_custom_source_paths() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        fs::write(&oracle, b"{").unwrap();

        let error = run(&generate_args(
            cr.clone(),
            oracle.clone(),
            root.path().join("output"),
        ))
        .unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("extracting canonical catalogs"));
        assert!(message.contains(&oracle.display().to_string()));
        assert!(message.contains("Scryfall Oracle Cards JSONL"));
    }

    #[test]
    fn canonical_cr_shape_errors_name_the_custom_source_path() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        fs::write(&cr, CR_FIXTURE.replace("701.2. Scry", "701.2 Scry")).unwrap();

        let error = run(&generate_args(
            cr.clone(),
            oracle.clone(),
            root.path().join("output"),
        ))
        .unwrap_err();

        assert_eq!(
            format!("{error:#}"),
            format!(
                "extracting canonical catalogs from CR {}; card-names source {}: keyword-actions: malformed 701 keyword heading \"701.2 Scry\"",
                cr.display(),
                oracle.display()
            )
        );
    }

    #[test]
    fn legacy_oracle_errors_name_keyword_augmentation_and_custom_source_paths() {
        let root = tempfile::tempdir().unwrap();
        let (cr, oracle) = write_sources(root.path());
        fs::write(&oracle, b"{").unwrap();

        let error = run(&text_args(
            cr.clone(),
            oracle.clone(),
            root.path().join("output"),
        ))
        .unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("extracting legacy catalogs"));
        assert!(message.contains(&oracle.display().to_string()));
        assert!(message.contains("Scryfall Oracle Cards JSONL"));
    }

    fn write_sources(root: &Path) -> (PathBuf, PathBuf) {
        let cr = root.join("custom-rules.txt");
        let oracle = root.join("custom-oracle-cards.jsonl");
        fs::write(&cr, CR_FIXTURE).unwrap();
        fs::write(&oracle, ORACLE_FIXTURE).unwrap();
        (cr, oracle)
    }

    fn generate_args(cr: PathBuf, oracle: PathBuf, output: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Generate(GenerateArgs {
                cr,
                oracle_cards: oracle,
                output,
            }),
        }
    }

    fn check_args(cr: PathBuf, oracle: PathBuf, checked: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Check(CheckArgs {
                cr,
                oracle_cards: oracle,
                checked,
            }),
        }
    }

    fn text_args(cr: PathBuf, oracle: PathBuf, output: PathBuf) -> CatalogArgs {
        CatalogArgs {
            command: CatalogCommand::Text(TextArgs {
                cr,
                oracle_cards: oracle,
                output,
            }),
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
