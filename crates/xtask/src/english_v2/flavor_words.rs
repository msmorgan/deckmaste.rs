use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::ensure;
use clap::Args;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_data::scryfall::Catalog;

const AUTHORED_ONSET_OVERRIDES: &[(&str, Onset)] = &[
    ("... Catch", Onset::Consonant),
    ("10,000 Needles", Onset::Consonant),
];

#[derive(Debug, Args)]
pub(super) struct FlavorWordsArgs {
    /// MTGJSON atomic-card snapshot supplying Vintage-playable Oracle text.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    data: PathBuf,
    /// Independent Scryfall flavor-word catalog.
    #[arg(long, default_value = "data/catalogs/flavor-words.json")]
    catalog: PathBuf,
    /// Generated Comprehensive Rules ability-word catalog.
    #[arg(long, default_value = "data/gen/catalogs/ability-words.txt")]
    ability_words: PathBuf,
    /// Generated flavor-word stub directory.
    #[arg(long, default_value = "plugins_v2/builtin/macros/stubs/flavor_words")]
    output: PathBuf,
    /// Check that the generated stubs are byte-for-byte current.
    #[arg(
        long,
        required_unless_present = "regenerate",
        conflicts_with = "regenerate"
    )]
    check: bool,
    /// Rewrite the generated stubs to match the current census.
    #[arg(long, required_unless_present = "check", conflicts_with = "check")]
    regenerate: bool,
}

pub(super) fn run(args: &FlavorWordsArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let expected = expected_stubs(args)?;
    let actual = read_stub_files(&args.output)?;
    let diff = StubDiff::between(&expected, &actual);

    if args.check {
        ensure!(
            diff.is_empty(),
            "flavor-word stubs differ ({} expected):\n{diff}",
            expected.len(),
        );
        writeln!(
            output,
            "flavor-word stubs are up to date ({} files)",
            expected.len()
        )?;
        return Ok(());
    }

    ensure!(args.regenerate, "a flavor-word action is required");
    if diff.is_empty() {
        writeln!(
            output,
            "flavor-word stubs are already up to date ({} files)",
            expected.len()
        )?;
        return Ok(());
    }

    writeln!(output, "flavor-word stub diff:\n{diff}")?;
    write_stubs(&args.output, &expected, &actual)?;
    writeln!(output, "regenerated {} flavor-word stubs", expected.len())?;
    Ok(())
}

fn expected_stubs(args: &FlavorWordsArgs) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
    let catalog_bytes =
        fs::read(&args.catalog).with_context(|| format!("reading {}", args.catalog.display()))?;
    let catalog = Catalog::parse(&catalog_bytes)
        .with_context(|| format!("parsing {}", args.catalog.display()))?;
    let catalog = catalog
        .data
        .iter()
        .map(deckmaste_data::DataStr::as_str)
        .collect::<BTreeSet<_>>();

    let ability_word_text = fs::read_to_string(&args.ability_words)
        .with_context(|| format!("reading {}", args.ability_words.display()))?;
    let ability_words = ability_word_text.lines().collect::<BTreeSet<_>>();

    let card_bytes =
        fs::read(&args.data).with_context(|| format!("reading {}", args.data.display()))?;
    let cards = AtomicCards::parse(&card_bytes)
        .with_context(|| format!("parsing {}", args.data.display()))?;

    let surfaces = deckmaste_data::flavor_words::census(&cards, &catalog, &ability_words);
    let onset_overrides = authored_onset_overrides(&surfaces)?;
    let mut stubs = BTreeMap::new();
    let mut identities = BTreeMap::new();
    for surface in surfaces {
        let name = declaration_name(surface)?;
        if let Some(other) = identities.insert(name.clone(), surface) {
            anyhow::bail!(
                "flavor words {other:?} and {surface:?} collide on declaration name {name:?}",
            );
        }
        stubs.insert(
            format!("{name}.ron"),
            render_stub(&name, surface, &onset_overrides)?,
        );
    }
    Ok(stubs)
}

fn authored_onset_overrides(
    surfaces: &BTreeSet<&str>,
) -> anyhow::Result<BTreeMap<&'static str, Onset>> {
    ensure_onset_override_inventory(surfaces, AUTHORED_ONSET_OVERRIDES)
}

/// Mirrors `super::ensure_card_name_onset_override_inventory`: one two-way
/// closure rule for both authored override inventories.
fn ensure_onset_override_inventory(
    surfaces: &BTreeSet<&str>,
    authored: &[(&'static str, Onset)],
) -> anyhow::Result<BTreeMap<&'static str, Onset>> {
    let mut overrides = BTreeMap::new();
    for &(surface, onset) in authored {
        ensure!(
            overrides.insert(surface, onset).is_none(),
            "flavor-word onset override inventory contains duplicate surface {surface:?}",
        );
    }

    let derived_exceptional = surfaces
        .iter()
        .filter(|surface| {
            deckmaste_construction_core::macro_def::normalize_surface_onset(surface, None).is_none()
        })
        .copied()
        .collect::<BTreeSet<_>>();
    let reviewed_exceptional = overrides.keys().copied().collect::<BTreeSet<_>>();
    ensure!(
        derived_exceptional == reviewed_exceptional,
        "flavor-word onset override inventory differs from census exceptions: derived {derived_exceptional:?}, reviewed {reviewed_exceptional:?}",
    );

    Ok(overrides)
}

fn declaration_name(surface: &str) -> anyhow::Result<String> {
    let expanded = surface.replace("...", " Ellipsis ");
    let mut name = expanded
        .split(|character: char| character.is_whitespace() || matches!(character, '|' | '-'))
        .flat_map(|word| {
            let mut characters = word.chars();
            characters
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(characters)
        })
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>();
    if name.starts_with(char::is_numeric) {
        name.insert_str(0, "Flavor");
    }
    ensure!(
        !name.is_empty(),
        "flavor word {surface:?} cannot form a declaration name",
    );
    Ok(name)
}

fn render_stub(
    name: &str,
    surface: &str,
    onset_overrides: &BTreeMap<&str, Onset>,
) -> anyhow::Result<Vec<u8>> {
    let name = serde_json::to_string(name).context("serializing a flavor-word declaration name")?;
    let serialized_surface =
        serde_json::to_string(surface).context("serializing a flavor-word surface")?;
    let onset = match onset_overrides.get(surface) {
        Some(Onset::Consonant) => ", onset: Consonant",
        Some(Onset::Vowel) => ", onset: Vowel",
        None => {
            ensure!(
                deckmaste_construction_core::macro_def::normalize_surface_onset(surface, None)
                    .is_some(),
                "flavor word {surface:?} has no classifiable onset or authored override",
            );
            ""
        }
    };
    Ok(format!(
        "FlavorWord(\n    name: {name},\n    spelling: {serialized_surface},\n    grammar: FixedTerm(surface: {serialized_surface}{onset}),\n)\n"
    )
    .into_bytes())
}

fn read_stub_files(root: &Path) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
    if !root.exists() {
        return Ok(BTreeMap::new());
    }
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry.with_context(|| format!("reading an entry in {}", root.display()))?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("ron") {
            continue;
        }
        let name = entry.file_name().into_string().map_err(|name| {
            anyhow::anyhow!("non-UTF-8 stub filename {}", Path::new(&name).display())
        })?;
        let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
        files.insert(name, bytes);
    }
    Ok(files)
}

fn write_stubs(
    root: &Path,
    expected: &BTreeMap<String, Vec<u8>>,
    actual: &BTreeMap<String, Vec<u8>>,
) -> anyhow::Result<()> {
    fs::create_dir_all(root).with_context(|| format!("creating {}", root.display()))?;
    for stale in actual.keys().filter(|name| !expected.contains_key(*name)) {
        let path = root.join(stale);
        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
    }
    for (name, bytes) in expected {
        let path = root.join(name);
        if actual.get(name) != Some(bytes) {
            fs::write(&path, bytes).with_context(|| format!("writing {}", path.display()))?;
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct StubDiff {
    missing: Vec<String>,
    changed: Vec<String>,
    stale: Vec<String>,
}

impl StubDiff {
    fn between(expected: &BTreeMap<String, Vec<u8>>, actual: &BTreeMap<String, Vec<u8>>) -> Self {
        Self {
            missing: expected
                .keys()
                .filter(|name| !actual.contains_key(*name))
                .cloned()
                .collect(),
            changed: expected
                .iter()
                .filter(|(name, bytes)| actual.get(*name).is_some_and(|old| old != *bytes))
                .map(|(name, _)| name.clone())
                .collect(),
            stale: actual
                .keys()
                .filter(|name| !expected.contains_key(*name))
                .cloned()
                .collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.missing.is_empty() && self.changed.is_empty() && self.stale.is_empty()
    }
}

impl fmt::Display for StubDiff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for name in &self.missing {
            writeln!(formatter, "+ {name}")?;
        }
        for name in &self.changed {
            writeln!(formatter, "~ {name}")?;
        }
        for name in &self.stale {
            writeln!(formatter, "- {name}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_onset_is_explicit_even_when_the_recipe_can_classify_the_surface() {
        let overrides = BTreeMap::from([("Aerial Blast", Onset::Vowel)]);

        let stub = render_stub("AerialBlast", "Aerial Blast", &overrides).unwrap();

        assert!(String::from_utf8(stub).unwrap().contains("onset: Vowel"));
    }

    #[test]
    fn unclassifiable_surface_without_an_authored_onset_is_rejected() {
        let error = render_stub("Infinity", "∞", &BTreeMap::new()).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("has no classifiable onset or authored override")
        );
    }

    #[test]
    fn onset_override_inventory_rejects_a_missing_exception() {
        let surfaces = BTreeSet::from(["∞"]);

        let error = ensure_onset_override_inventory(&surfaces, &[]).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("inventory differs from census exceptions")
        );
    }

    #[test]
    fn onset_override_inventory_rejects_a_stale_exception() {
        let surfaces = BTreeSet::new();

        let error = ensure_onset_override_inventory(&surfaces, &[("∞", Onset::Vowel)]).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("inventory differs from census exceptions")
        );
    }

    #[test]
    fn onset_override_inventory_rejects_an_override_on_a_classifiable_surface() {
        let surfaces = BTreeSet::from(["Aerial Blast"]);

        let error = ensure_onset_override_inventory(&surfaces, &[("Aerial Blast", Onset::Vowel)])
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("inventory differs from census exceptions")
        );
    }

    #[test]
    fn onset_override_inventory_rejects_a_duplicate_exception() {
        let surfaces = BTreeSet::from(["∞"]);

        let error =
            ensure_onset_override_inventory(&surfaces, &[("∞", Onset::Vowel), ("∞", Onset::Vowel)])
                .unwrap_err();

        assert!(error.to_string().contains("contains duplicate surface"));
    }

    #[test]
    fn declaration_names_are_stable_for_punctuation_and_numeric_initials() {
        assert_eq!(declaration_name("Aerial Blast").unwrap(), "AerialBlast");
        assert_eq!(declaration_name("Allons-y!").unwrap(), "AllonsY");
        assert_eq!(declaration_name("Bigby's Hand").unwrap(), "BigbysHand");
        assert_eq!(declaration_name("... Catch").unwrap(), "EllipsisCatch");
        assert_eq!(declaration_name("Throw ...").unwrap(), "ThrowEllipsis");
        assert_eq!(
            declaration_name("10,000 Needles").unwrap(),
            "Flavor10000Needles"
        );
    }

    #[test]
    fn stub_diff_is_sorted_and_classifies_each_kind_of_change() {
        let expected = BTreeMap::from([
            ("Added.ron".to_owned(), b"new".to_vec()),
            ("Changed.ron".to_owned(), b"new".to_vec()),
        ]);
        let actual = BTreeMap::from([
            ("Changed.ron".to_owned(), b"old".to_vec()),
            ("Stale.ron".to_owned(), b"old".to_vec()),
        ]);

        let diff = StubDiff::between(&expected, &actual);

        assert_eq!(
            diff,
            StubDiff {
                missing: vec!["Added.ron".to_owned()],
                changed: vec!["Changed.ron".to_owned()],
                stale: vec!["Stale.ron".to_owned()],
            }
        );
        assert_eq!(
            diff.to_string(),
            "+ Added.ron\n~ Changed.ron\n- Stale.ron\n"
        );
    }

    #[test]
    fn stub_writer_converges_changed_directories_without_touching_other_files() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("Changed.ron"), b"old").unwrap();
        fs::write(root.path().join("Stale.ron"), b"old").unwrap();
        fs::write(root.path().join("README.md"), b"keep").unwrap();
        let actual = read_stub_files(root.path()).unwrap();
        let expected = BTreeMap::from([
            ("Added.ron".to_owned(), b"new".to_vec()),
            ("Changed.ron".to_owned(), b"new".to_vec()),
        ]);

        write_stubs(root.path(), &expected, &actual).unwrap();

        assert_eq!(read_stub_files(root.path()).unwrap(), expected);
        assert_eq!(fs::read(root.path().join("README.md")).unwrap(), b"keep");
    }
}
