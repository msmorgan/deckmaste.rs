use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use deckmaste_construction_core::Expansion;
use deckmaste_english_v2::parser::SelectionExceptionInfo;
use serde::Serialize;

use super::ReportArgs;
use super::expansion_from_source;
use super::production_declaration_path;

#[derive(Debug, Serialize)]
struct CountedReport {
    schema_version: u32,
    /// Top-level `construction` declarations; the canonical figure for landing
    /// records, emitted only by this command.
    construction_count: usize,
    card_name_catalog_rows: usize,
    parenthetical_inventory: super::corpus::ParentheticalInventory,
    noun_morphology: NounMorphologyCensus,
    licensing_checkers: super::licensing_checkers::LicensingCheckerCensus,
    mapping_layers: Vec<CountedEntry>,
    handwritten_codecs: Vec<CountedEntry>,
    stored_form_tags: Vec<CountedEntry>,
    stored_spelling_codecs: Vec<CountedEntry>,
    morphology_irregulars: Vec<MorphologyIrregular>,
    selection_exceptions: Vec<CountedEntry>,
    terminal_bindings: Vec<CountedEntry>,
    checked_constructor_bindings: Vec<CountedEntry>,
    roots: Vec<CountedEntry>,
    abstract_products: Vec<String>,
    abstract_sums: Vec<String>,
    optional_roles: Vec<String>,
    sequence_roles: Vec<String>,
    sequence_feature_roles: Vec<String>,
    uniform_separators: Vec<String>,
    positional_separator_tables: Vec<String>,
    terminators: Vec<String>,
    stored_separator_fields: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
struct NounMorphologyCensus {
    total: usize,
    derived_plural: usize,
    explicit_plural: usize,
    unavailable_plural: usize,
}

#[derive(Debug, Serialize)]
struct CountedEntry {
    identity: String,
    rationale: Option<String>,
    removal_target: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct MorphologyIrregular {
    identity: String,
    overrides: Vec<MorphologyOverride>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct MorphologyOverride {
    feature: String,
    surface: String,
}

#[derive(Debug)]
struct BuiltinNounMorphology {
    census: NounMorphologyCensus,
    irregulars: Vec<MorphologyIrregular>,
}

pub(super) fn run(args: &ReportArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let path = production_declaration_path();
    let source =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let report = build_report_from_source(&source)?;
    let rendered = if args.json { render_json(&report)? } else { render_human(&report) };
    output
        .write_all(rendered.as_bytes())
        .context("writing English-v2 counted escape-hatch report")
}

fn build_report_from_source(source: &str) -> anyhow::Result<CountedReport> {
    let expansion = expansion_from_source(source)?;
    let declaration_provenance = super::declaration_provenance(source)?;
    let licensing_checkers = super::licensing_checkers::from_expansion(source, &expansion)?;
    let selection_exceptions = deckmaste_english_v2::parser::selection_exception_inventory()
        .map_err(anyhow::Error::new)
        .context("validating English-v2 selection exception inventory")?;
    let builtin_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
    let builtin_nouns = builtin_noun_morphology(&builtin_root)?;
    let card_name_catalog_rows = super::card_name_catalog_row_count()?;
    let parenthetical_inventory = super::corpus::parenthetical_inventory()?;
    build_report(
        &expansion,
        declaration_provenance.construction_count,
        card_name_catalog_rows,
        parenthetical_inventory,
        &selection_exceptions,
        builtin_nouns,
        licensing_checkers,
    )
}

fn build_report(
    expansion: &Expansion,
    construction_count: usize,
    card_name_catalog_rows: usize,
    parenthetical_inventory: super::corpus::ParentheticalInventory,
    selection_exceptions: &[SelectionExceptionInfo],
    builtin_nouns: BuiltinNounMorphology,
    licensing_checkers: super::licensing_checkers::LicensingCheckerCensus,
) -> anyhow::Result<CountedReport> {
    let escape_hatches = expansion.escape_hatches();
    let morphology_irregulars = escape_hatches
        .morphology_irregulars()
        .iter()
        .map(|irregular| MorphologyIrregular {
            identity: irregular.identity().to_owned(),
            overrides: irregular
                .overrides()
                .iter()
                .map(|row| MorphologyOverride {
                    feature: row.feature().to_owned(),
                    surface: row.surface().to_owned(),
                })
                .collect(),
        })
        .chain(builtin_nouns.irregulars)
        .collect();
    let mut report = CountedReport {
        schema_version: 6,
        construction_count,
        card_name_catalog_rows,
        parenthetical_inventory,
        noun_morphology: builtin_nouns.census,
        licensing_checkers,
        mapping_layers: plain_entries(escape_hatches.mapping_layers(), None, None),
        handwritten_codecs: plain_entries(
            escape_hatches.handwritten_codecs(),
            Some("stage 4 handwritten codec declaration"),
            Some("generated_terminal_tiers"),
        ),
        stored_form_tags: plain_entries(escape_hatches.stored_form_tags(), None, None),
        stored_spelling_codecs: plain_entries(
            escape_hatches.stored_spelling_codecs(),
            Some("stores a non-derivable context spelling choice"),
            None,
        ),
        morphology_irregulars,
        selection_exceptions: selection_exceptions
            .iter()
            .map(|exception| CountedEntry {
                identity: exception.id.to_owned(),
                rationale: Some(exception.rationale.to_owned()),
                removal_target: None,
            })
            .collect(),
        terminal_bindings: escape_hatches
            .terminal_bindings()
            .iter()
            .map(|binding| CountedEntry {
                identity: binding.canonical_identity(),
                rationale: Some("stage 4 terminal binding".to_owned()),
                removal_target: Some("generated_terminal_tiers".to_owned()),
            })
            .collect(),
        checked_constructor_bindings: vec![],
        roots: plain_entries(
            escape_hatches.roots(),
            Some("declared parser entry point"),
            None,
        ),
        abstract_products: escape_hatches.abstract_products().to_vec(),
        abstract_sums: escape_hatches.abstract_sums().to_vec(),
        optional_roles: escape_hatches.optional_roles().to_vec(),
        sequence_roles: escape_hatches.sequence_roles().to_vec(),
        sequence_feature_roles: escape_hatches.sequence_feature_roles().to_vec(),
        uniform_separators: escape_hatches.uniform_separators().to_vec(),
        positional_separator_tables: escape_hatches.positional_separator_tables().to_vec(),
        terminators: escape_hatches.terminators().to_vec(),
        stored_separator_fields: escape_hatches.stored_separator_fields().to_vec(),
    };
    validate_and_sort(&mut report)?;
    Ok(report)
}

fn builtin_noun_morphology(root: &Path) -> anyhow::Result<BuiltinNounMorphology> {
    use deckmaste_construction_core::macro_def::DerivedSurface;
    use deckmaste_construction_core::macro_def::Grammar;
    use deckmaste_construction_core::macro_def::GrammarRecipe;
    use deckmaste_construction_core::macro_def::Metadata;

    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(root)
        .map_err(anyhow::Error::new)
        .with_context(|| format!("authenticating builtin-v2 sources below {}", root.display()))?;
    let mut census = NounMorphologyCensus {
        total: 0,
        derived_plural: 0,
        explicit_plural: 0,
        unavailable_plural: 0,
    };
    let mut irregulars = Vec::new();
    let reader = deckmaste_construction_core::macro_def::declaration_macro_set()
        .map_err(anyhow::Error::msg)?;

    for normalized in declarations {
        let path = normalized.provenance().path();
        let source = fs::read_to_string(path)
            .with_context(|| format!("reading authenticated noun source {}", path.display()))?;
        let raw = reader
            .read_str::<macro_ron::MacroDef<Metadata>>(&source)
            .map_err(anyhow::Error::new)
            .with_context(|| format!("reparsing authenticated noun source {}", path.display()))?;
        let kind = normalized.identity().kind();
        if !matches!(
            kind,
            deckmaste_construction_core::macro_def::DeclarationKind::Type
                | deckmaste_construction_core::macro_def::DeclarationKind::TurnPart
                | deckmaste_construction_core::macro_def::DeclarationKind::Subtype(_)
        ) {
            continue;
        }
        let name = normalized.identity().name();
        let grammar = raw.metadata().grammar.as_ref();
        let Some(Grammar::Noun { plural, .. }) = grammar else {
            continue;
        };

        if !matches!(
            normalized
                .grammar()
                .map(deckmaste_construction_core::macro_def::GrammarRow::recipe),
            Some(GrammarRecipe::Noun)
        ) {
            bail!(
                "raw noun grammar in {} disappeared during production normalization",
                path.display(),
            );
        }

        census.total += 1;
        match plural {
            DerivedSurface::Derived => census.derived_plural += 1,
            DerivedSurface::Override(surface) => {
                census.explicit_plural += 1;
                irregulars.push(MorphologyIrregular {
                    identity: format!("lexeme:{}/{}", declaration_kind_key(kind), name),
                    overrides: vec![MorphologyOverride {
                        feature: "plural".to_owned(),
                        surface: surface.clone(),
                    }],
                });
            }
            DerivedSurface::Unavailable => census.unavailable_plural += 1,
        }
    }
    // Unattested open plurals remain unavailable and exist only in this census;
    // report evidence is restricted to authenticated authored overrides.
    validate_noun_morphology_census(census)?;
    Ok(BuiltinNounMorphology { census, irregulars })
}

fn validate_noun_morphology_census(census: NounMorphologyCensus) -> anyhow::Result<()> {
    if census.total != census.derived_plural + census.explicit_plural + census.unavailable_plural {
        bail!(
            "builtin noun morphology has an unclassifiable surface: total {} != derived {} + explicit {} + unavailable {}",
            census.total,
            census.derived_plural,
            census.explicit_plural,
            census.unavailable_plural,
        );
    }
    Ok(())
}

#[cfg(test)]
fn builtin_noun_morphology_census(root: &Path) -> anyhow::Result<NounMorphologyCensus> {
    builtin_noun_morphology(root).map(|morphology| morphology.census)
}

fn declaration_kind_key(
    kind: deckmaste_construction_core::macro_def::DeclarationKind,
) -> &'static str {
    match kind {
        deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction => "keyword_action",
        deckmaste_construction_core::macro_def::DeclarationKind::KeywordAbility => {
            "keyword_ability"
        }
        deckmaste_construction_core::macro_def::DeclarationKind::AbilityWord => "ability_word",
        deckmaste_construction_core::macro_def::DeclarationKind::FlavorWord => "flavor_word",
        deckmaste_construction_core::macro_def::DeclarationKind::Subtype(category) => {
            match category {
                deckmaste_construction_core::macro_def::SubtypeCategory::Artifact => {
                    "artifact_subtype"
                }
                deckmaste_construction_core::macro_def::SubtypeCategory::Battle => "battle_subtype",
                deckmaste_construction_core::macro_def::SubtypeCategory::Creature => {
                    "creature_subtype"
                }
                deckmaste_construction_core::macro_def::SubtypeCategory::Enchantment => {
                    "enchantment_subtype"
                }
                deckmaste_construction_core::macro_def::SubtypeCategory::Land => "land_subtype",
                deckmaste_construction_core::macro_def::SubtypeCategory::Planeswalker => {
                    "planeswalker_subtype"
                }
                deckmaste_construction_core::macro_def::SubtypeCategory::Spell => "spell_subtype",
            }
        }
        deckmaste_construction_core::macro_def::DeclarationKind::Type => "type",
        deckmaste_construction_core::macro_def::DeclarationKind::TurnPart => "turn_part",
        deckmaste_construction_core::macro_def::DeclarationKind::CounterKind => "counter_kind",
        deckmaste_construction_core::macro_def::DeclarationKind::Designation => "designation",
    }
}

fn plain_entries(
    identities: &[String],
    rationale: Option<&str>,
    removal_target: Option<&str>,
) -> Vec<CountedEntry> {
    identities
        .iter()
        .map(|identity| CountedEntry {
            identity: identity.clone(),
            rationale: rationale.map(str::to_owned),
            removal_target: removal_target.map(str::to_owned),
        })
        .collect()
}

fn validate_and_sort(report: &mut CountedReport) -> anyhow::Result<()> {
    let mut irregular_identities = std::collections::HashSet::new();
    for irregular in &report.morphology_irregulars {
        if !irregular_identities.insert(irregular.identity.as_str()) {
            bail!(
                "duplicate identity `{}` in morphology irregulars",
                irregular.identity
            );
        }
    }
    for (category, entries) in categories_mut(report) {
        entries.sort_by(|left, right| left.identity.cmp(&right.identity));
        for pair in entries.windows(2) {
            if pair[0].identity == pair[1].identity {
                bail!("duplicate identity `{}` in {category}", pair[0].identity);
            }
        }
    }
    Ok(())
}

fn categories_mut(report: &mut CountedReport) -> [(&'static str, &mut Vec<CountedEntry>); 7] {
    [
        ("mapping layers", &mut report.mapping_layers),
        ("handwritten codecs", &mut report.handwritten_codecs),
        ("stored form tags", &mut report.stored_form_tags),
        ("stored spelling codecs", &mut report.stored_spelling_codecs),
        ("selection exceptions", &mut report.selection_exceptions),
        ("terminal bindings", &mut report.terminal_bindings),
        ("roots", &mut report.roots),
    ]
}

fn render_json(report: &CountedReport) -> anyhow::Result<String> {
    let mut rendered =
        serde_json::to_string_pretty(report).context("serializing counted report")?;
    rendered.push('\n');
    Ok(rendered)
}

fn render_human(report: &CountedReport) -> String {
    let mut output = String::from("English v2 counted escape hatches\n");
    writeln!(
        &mut output,
        "construction declarations ({})",
        report.construction_count,
    )
    .expect("writing to String cannot fail");
    writeln!(
        &mut output,
        "card-name catalog rows ({})",
        report.card_name_catalog_rows,
    )
    .expect("writing to String cannot fail");
    for (kind, rows) in [
        (
            "rules-bearing parentheticals",
            &report.parenthetical_inventory.rules_bearing,
        ),
        (
            "reminder parentheticals followed by text",
            &report.parenthetical_inventory.reminder_followed_by_text,
        ),
    ] {
        writeln!(&mut output, "{kind} ({})", rows.len()).expect("writing to String cannot fail");
        for row in rows {
            writeln!(
                &mut output,
                "  - surface={:?} occurrences={}",
                row.surface, row.occurrences,
            )
            .expect("writing to String cannot fail");
        }
    }
    writeln!(
        &mut output,
        "noun morphology (total={}, derived +s={}, explicit={}, unavailable={})",
        report.noun_morphology.total,
        report.noun_morphology.derived_plural,
        report.noun_morphology.explicit_plural,
        report.noun_morphology.unavailable_plural,
    )
    .expect("writing to String cannot fail");
    writeln!(
        &mut output,
        "licensing checkers (permitted={}, forbidden={})",
        report.licensing_checkers.permitted_total(),
        report.licensing_checkers.forbidden_total(),
    )
    .expect("writing to String cannot fail");
    for checker in report.licensing_checkers.rows() {
        writeln!(
            &mut output,
            "  - identity={:?} kind={:?}",
            checker.identity(),
            checker.kind(),
        )
        .expect("writing to String cannot fail");
    }
    for (category, entries) in categories(report) {
        writeln!(&mut output, "{category} ({})", entries.len())
            .expect("writing to String cannot fail");
        for entry in entries {
            write!(&mut output, "  - identity={:?}", entry.identity)
                .expect("writing to String cannot fail");
            if let Some(rationale) = &entry.rationale {
                write!(&mut output, " rationale={rationale:?}")
                    .expect("writing to String cannot fail");
            }
            if let Some(removal_target) = &entry.removal_target {
                write!(&mut output, " removal_target={removal_target:?}")
                    .expect("writing to String cannot fail");
            }
            output.push('\n');
        }
        if category == "stored spelling codecs" {
            writeln!(
                &mut output,
                "morphology irregulars ({})",
                report.morphology_irregulars.len()
            )
            .expect("writing to String cannot fail");
            for irregular in &report.morphology_irregulars {
                writeln!(&mut output, "  - identity={:?}", irregular.identity)
                    .expect("writing to String cannot fail");
                for row in &irregular.overrides {
                    writeln!(
                        &mut output,
                        "    - feature={:?} surface={:?}",
                        row.feature, row.surface
                    )
                    .expect("writing to String cannot fail");
                }
            }
        }
    }
    for (category, entries) in structural_categories(report) {
        writeln!(&mut output, "{category} ({})", entries.len())
            .expect("writing to String cannot fail");
        for identity in entries {
            writeln!(&mut output, "  - identity={identity:?}")
                .expect("writing to String cannot fail");
        }
    }
    output
}

fn categories(report: &CountedReport) -> [(&'static str, &[CountedEntry]); 8] {
    [
        ("mapping layers", &report.mapping_layers),
        ("handwritten codecs", &report.handwritten_codecs),
        ("stored form tags", &report.stored_form_tags),
        ("stored spelling codecs", &report.stored_spelling_codecs),
        ("selection exceptions", &report.selection_exceptions),
        ("terminal bindings", &report.terminal_bindings),
        (
            "checked constructor bindings",
            &report.checked_constructor_bindings,
        ),
        ("roots", &report.roots),
    ]
}

fn structural_categories(report: &CountedReport) -> [(&'static str, &[String]); 9] {
    [
        ("abstract products", &report.abstract_products),
        ("abstract sums", &report.abstract_sums),
        ("optional roles", &report.optional_roles),
        ("sequence roles", &report.sequence_roles),
        ("sequence feature roles", &report.sequence_feature_roles),
        ("uniform separators", &report.uniform_separators),
        (
            "positional separator tables",
            &report.positional_separator_tables,
        ),
        ("terminators", &report.terminators),
        ("stored separator fields", &report.stored_separator_fields),
    ]
}

#[cfg(test)]
mod tests {
    use serde::Deserialize as _;

    use super::*;

    const PRODUCTION_SOURCE: &str =
        include_str!("../../../deckmaste_english_v2/src/constructions.rs");

    const SCHEMA4_STRUCTURAL_SOURCE: &str = r#"
        constructions! {
            construction node: Node {
                element NodeValue {}
                form node = "node";
            }
            abstract sum ZetaChoice { node: Node, }
            abstract sum AlphaChoice { node: Node, }
            abstract sum MuChoice { node: Node, }
            abstract sum BetaChoice { node: Node, }
            abstract product ZetaHolder {
                maybe_zeta: opt Node,
                items_zeta: seq ZetaChoice separated by " / " terminated by ".",
            }
            require len(ZetaHolder.items_zeta) >= 1;
            abstract product AlphaHolder {
                maybe_alpha: opt Node,
                items_alpha: seq AlphaChoice separated by " + " terminated by "!",
            }
            require len(AlphaHolder.items_alpha) >= 1;
            abstract product MuHolder {
                maybe_mu: opt Node,
                items_mu: seq MuChoice
                    separated by position {
                        pair = " and ";
                        first = ", ";
                        middle = ", ";
                        last = ", and ";
                    }
                    terminated by "?",
            }
            require len(MuHolder.items_mu) >= 2;
            abstract product BetaHolder {
                maybe_beta: opt Node,
                items_beta: seq BetaChoice
                    separated by position {
                        pair = " or ";
                        first = "; ";
                        middle = "; ";
                        last = "; or ";
                    }
                    terminated by ":",
            }
            require len(BetaHolder.items_beta) >= 2;
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        }
    "#;

    #[test]
    fn schema5_projects_every_sealed_structural_row_in_source_order() {
        let report = build_report_from_source(SCHEMA4_STRUCTURAL_SOURCE)
            .expect("structural report fixture builds");
        let rendered = render_json(&report).expect("schema 5 serializes");
        let json: serde_json::Value = serde_json::from_str(&rendered).unwrap();

        assert_eq!(json["schema_version"], 6);
        assert_eq!(
            json["abstract_products"],
            serde_json::json!(["ZetaHolder", "AlphaHolder", "MuHolder", "BetaHolder"])
        );
        assert_eq!(
            json["abstract_sums"],
            serde_json::json!(["ZetaChoice", "AlphaChoice", "MuChoice", "BetaChoice"])
        );
        assert_eq!(
            json["optional_roles"],
            serde_json::json!([
                "ZetaHolder.maybe_zeta",
                "AlphaHolder.maybe_alpha",
                "MuHolder.maybe_mu",
                "BetaHolder.maybe_beta"
            ])
        );
        assert_eq!(
            json["sequence_roles"],
            serde_json::json!([
                "ZetaHolder.items_zeta",
                "AlphaHolder.items_alpha",
                "MuHolder.items_mu",
                "BetaHolder.items_beta"
            ])
        );
        assert_eq!(json["sequence_feature_roles"], serde_json::json!([]));
        assert_eq!(
            json["uniform_separators"],
            serde_json::json!(["ZetaHolder.items_zeta", "AlphaHolder.items_alpha"])
        );
        assert_eq!(
            json["positional_separator_tables"],
            serde_json::json!(["MuHolder.items_mu", "BetaHolder.items_beta"])
        );
        assert_eq!(
            json["terminators"],
            serde_json::json!([
                "ZetaHolder.items_zeta",
                "AlphaHolder.items_alpha",
                "MuHolder.items_mu",
                "BetaHolder.items_beta"
            ])
        );
        assert_eq!(json["stored_separator_fields"], serde_json::json!([]));
    }

    #[test]
    fn schema6_pins_exact_top_level_key_order_and_has_no_legacy_mode() {
        let report = build_report_from_source(PRODUCTION_SOURCE)
            .expect("production declaration report builds");
        let rendered = render_json(&report).expect("production report serializes");

        assert_eq!(
            raw_top_level_keys(&rendered),
            [
                "schema_version",
                "construction_count",
                "card_name_catalog_rows",
                "parenthetical_inventory",
                "noun_morphology",
                "licensing_checkers",
                "mapping_layers",
                "handwritten_codecs",
                "stored_form_tags",
                "stored_spelling_codecs",
                "morphology_irregulars",
                "selection_exceptions",
                "terminal_bindings",
                "checked_constructor_bindings",
                "roots",
                "abstract_products",
                "abstract_sums",
                "optional_roles",
                "sequence_roles",
                "sequence_feature_roles",
                "uniform_separators",
                "positional_separator_tables",
                "terminators",
                "stored_separator_fields",
            ]
        );
        assert!(rendered.contains("\"schema_version\": 6"));
        assert!(!rendered.contains("\"schema_version\": 5"));
        assert!(!rendered.contains("\"schema_version\": 4"));
        assert!(!rendered.contains("\"schema_version\": 3"));
    }

    fn entry(identity: &str) -> CountedEntry {
        CountedEntry {
            identity: identity.to_owned(),
            rationale: None,
            removal_target: None,
        }
    }

    fn report() -> CountedReport {
        CountedReport {
            schema_version: 6,
            construction_count: 0,
            card_name_catalog_rows: 0,
            parenthetical_inventory: super::super::corpus::ParentheticalInventory {
                rules_bearing: vec![],
                reminder_followed_by_text: vec![],
            },
            noun_morphology: NounMorphologyCensus {
                total: 0,
                derived_plural: 0,
                explicit_plural: 0,
                unavailable_plural: 0,
            },
            licensing_checkers: super::super::licensing_checkers::LicensingCheckerCensus::default(),
            mapping_layers: vec![entry("zeta"), entry("alpha")],
            handwritten_codecs: vec![],
            stored_form_tags: vec![],
            stored_spelling_codecs: vec![],
            morphology_irregulars: vec![],
            selection_exceptions: vec![],
            terminal_bindings: vec![],
            checked_constructor_bindings: vec![],
            roots: vec![],
            abstract_products: vec![],
            abstract_sums: vec![],
            optional_roles: vec![],
            sequence_roles: vec![],
            sequence_feature_roles: vec![],
            uniform_separators: vec![],
            positional_separator_tables: vec![],
            terminators: vec![],
            stored_separator_fields: vec![],
        }
    }

    #[test]
    fn validation_sorts_each_category_and_json_keeps_empty_arrays() {
        let mut report = report();
        validate_and_sort(&mut report).expect("distinct identities validate");

        assert_eq!(
            report
                .mapping_layers
                .iter()
                .map(|entry| entry.identity.as_str())
                .collect::<Vec<_>>(),
            ["alpha", "zeta"]
        );
        let json = render_json(&report).expect("report serializes");
        assert!(json.contains("\"handwritten_codecs\": []"));
        assert!(json.ends_with('\n'));
    }

    #[test]
    fn validation_rejects_duplicate_identities_within_a_category() {
        let mut report = report();
        report.mapping_layers = vec![entry("same"), entry("same")];

        let error = validate_and_sort(&mut report).expect_err("duplicates are invalid");
        assert!(error.to_string().contains("mapping layers"));
        assert!(error.to_string().contains("same"));
    }

    #[test]
    fn morphology_irregular_validation_preserves_source_order() {
        let mut report = report();
        report.morphology_irregulars = vec![
            morphology_irregular("test:morphology/Zeta", &[("bare", "zeta")]),
            morphology_irregular("test:morphology/Alpha", &[("bare", "alpha")]),
        ];

        validate_and_sort(&mut report).expect("distinct irregular identities validate");

        assert_eq!(
            morphology_identities(&report.morphology_irregulars),
            ["test:morphology/Zeta", "test:morphology/Alpha"]
        );
    }

    #[test]
    fn morphology_irregular_duplicate_rejection_does_not_reorder_input() {
        let mut report = report();
        report.morphology_irregulars = vec![
            morphology_irregular("test:morphology/Zeta", &[("bare", "first")]),
            morphology_irregular("test:morphology/Alpha", &[("bare", "middle")]),
            morphology_irregular("test:morphology/Zeta", &[("bare", "last")]),
        ];

        let error = validate_and_sort(&mut report).expect_err("duplicate irregular is invalid");

        assert!(error.to_string().contains("morphology irregulars"));
        assert!(error.to_string().contains("test:morphology/Zeta"));
        assert_eq!(
            morphology_identities(&report.morphology_irregulars),
            [
                "test:morphology/Zeta",
                "test:morphology/Alpha",
                "test:morphology/Zeta",
            ]
        );
    }

    #[test]
    fn rendering_is_deterministic_and_human_fields_are_line_safe() {
        let mut report = report();
        report.mapping_layers = vec![CountedEntry {
            identity: "first\nsecond".to_owned(),
            rationale: Some("why\tstill".to_owned()),
            removal_target: Some("target\rnext".to_owned()),
        }];
        report.morphology_irregulars = vec![morphology_irregular(
            "test:morphology/Gamma",
            &[("bare", "are"), ("third_person_singular", "is")],
        )];
        validate_and_sort(&mut report).expect("distinct identities validate");

        assert_eq!(
            render_json(&report).expect("first JSON rendering"),
            render_json(&report).expect("repeat JSON rendering")
        );
        assert_eq!(
            render_human(&report),
            concat!(
                "English v2 counted escape hatches\n",
                "construction declarations (0)\n",
                "card-name catalog rows (0)\n",
                "rules-bearing parentheticals (0)\n",
                "reminder parentheticals followed by text (0)\n",
                "noun morphology (total=0, derived +s=0, explicit=0, unavailable=0)\n",
                "licensing checkers (permitted=0, forbidden=0)\n",
                "mapping layers (1)\n",
                "  - identity=\"first\\nsecond\" rationale=\"why\\tstill\" removal_target=\"target\\rnext\"\n",
                "handwritten codecs (0)\n",
                "stored form tags (0)\n",
                "stored spelling codecs (0)\n",
                "morphology irregulars (1)\n",
                "  - identity=\"test:morphology/Gamma\"\n",
                "    - feature=\"bare\" surface=\"are\"\n",
                "    - feature=\"third_person_singular\" surface=\"is\"\n",
                "selection exceptions (0)\n",
                "terminal bindings (0)\n",
                "checked constructor bindings (0)\n",
                "roots (0)\n",
                "abstract products (0)\n",
                "abstract sums (0)\n",
                "optional roles (0)\n",
                "sequence roles (0)\n",
                "sequence feature roles (0)\n",
                "uniform separators (0)\n",
                "positional separator tables (0)\n",
                "terminators (0)\n",
                "stored separator fields (0)\n"
            )
        );
    }

    #[test]
    fn production_report_preserves_escape_hatch_invariants() {
        let report = build_report_from_source(PRODUCTION_SOURCE)
            .expect("production declaration report builds");

        assert_eq!(report.schema_version, 6);
        let escape_hatch_zeros = [
            report.mapping_layers.len(),
            report.handwritten_codecs.len(),
            report.stored_form_tags.len(),
            report.selection_exceptions.len(),
            report.terminal_bindings.len(),
            report.checked_constructor_bindings.len(),
            report.stored_separator_fields.len(),
        ];
        assert_eq!(
            escape_hatch_zeros, [0; 7],
            "categories intentionally overlap and have no unique total"
        );
        assert_eq!(
            identities(&report.stored_spelling_codecs),
            ["SelfReferenceSpelling"]
        );
        assert!(!report.morphology_irregulars.is_empty());
        assert!(report.morphology_irregulars.iter().all(|irregular| {
            !irregular.identity.is_empty()
                && !irregular.overrides.is_empty()
                && irregular
                    .overrides
                    .iter()
                    .all(|row| !row.feature.is_empty() && !row.surface.is_empty())
        }));
        let construction_irregulars = report
            .morphology_irregulars
            .iter()
            .filter(|irregular| {
                !matches!(
                    irregular.identity.split('/').next(),
                    Some(
                        "lexeme:artifact_subtype"
                            | "lexeme:battle_subtype"
                            | "lexeme:creature_subtype"
                            | "lexeme:enchantment_subtype"
                            | "lexeme:land_subtype"
                            | "lexeme:planeswalker_subtype"
                            | "lexeme:spell_subtype"
                            | "lexeme:type"
                    )
                )
            })
            .map(|irregular| {
                (
                    irregular.identity.as_str(),
                    irregular
                        .overrides
                        .iter()
                        .map(|row| (row.feature.as_str(), row.surface.as_str()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            construction_irregulars,
            [
                ("lexeme:CommonNoun/Ability", vec![("plural", "abilities")]),
                ("lexeme:CommonNoun/Copy", vec![("plural", "copies")]),
                ("lexeme:CommonNoun/Library", vec![("plural", "libraries")],),
                ("lexeme:CommonNoun/Tax", vec![("plural", "taxes")]),
                (
                    "lexeme:CommonNoun/Toughness",
                    vec![("plural", "toughnesses")],
                ),
                ("lexeme:CommonNoun/Die", vec![("plural", "dice")]),
            ],
        );
        assert!(
            report.morphology_irregulars.iter().all(|irregular| {
                !irregular.identity.starts_with("lexeme:VerbLexeme/")
                    || matches!(
                        irregular.identity.as_str(),
                        "lexeme:VerbLexeme/Have" | "lexeme:VerbLexeme/Be"
                    )
            }),
            "only the two intentionally irregular verb lexemes remain"
        );
        assert!(report.handwritten_codecs.is_empty());
        assert!(
            report.stored_form_tags.is_empty(),
            "guarded demonstrative selection derives its form from stored words",
        );
        assert!(
            report.selection_exceptions.is_empty(),
            "guarded demonstrative selection does not alter specificity",
        );
        assert!(report.terminal_bindings.is_empty());
        assert!(report.checked_constructor_bindings.is_empty());
        assert!(
            !report.roots.is_empty() && report.roots.iter().all(|entry| !entry.identity.is_empty()),
            "the root inventory is reported, not pinned, but it is never empty",
        );
        for entries in [
            &report.abstract_products,
            &report.abstract_sums,
            &report.optional_roles,
            &report.sequence_roles,
            &report.sequence_feature_roles,
            &report.uniform_separators,
            &report.positional_separator_tables,
            &report.terminators,
        ] {
            assert!(entries.iter().all(|entry| !entry.is_empty()));
            let unique = entries.iter().collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                unique.len(),
                entries.len(),
                "structural report rows are unique"
            );
        }
        assert!(
            report
                .abstract_products
                .iter()
                .any(|entry| entry == "OracleText")
        );
        assert!(report.stored_separator_fields.is_empty());
    }
    #[test]
    fn authenticated_raw_noun_overrides_match_literal_evidence_one_for_one() {
        use deckmaste_construction_core::macro_def::DerivedSurface;
        use deckmaste_construction_core::macro_def::Grammar;
        use deckmaste_construction_core::macro_def::Metadata;

        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
        let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(&root)
            .expect("builtin-v2 sources authenticate independently");
        let reader = deckmaste_construction_core::macro_def::declaration_macro_set()
            .expect("declaration meta-macros are valid");
        let mut actual = Vec::new();
        for normalized in declarations {
            let source = std::fs::read_to_string(normalized.provenance().path())
                .expect("authenticated source remains readable");
            let raw = reader
                .read_str::<macro_ron::MacroDef<Metadata>>(&source)
                .expect("authenticated source reparses independently");
            let name = normalized.identity().name();
            let kind_key = match normalized.identity().kind() {
                deckmaste_construction_core::macro_def::DeclarationKind::Type => "type",
                deckmaste_construction_core::macro_def::DeclarationKind::TurnPart => "turn_part",
                deckmaste_construction_core::macro_def::DeclarationKind::Subtype(category) => {
                    match category {
                        deckmaste_construction_core::macro_def::SubtypeCategory::Artifact => {
                            "artifact_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Battle => {
                            "battle_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Creature => {
                            "creature_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Enchantment => {
                            "enchantment_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Land => {
                            "land_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Planeswalker => {
                            "planeswalker_subtype"
                        }
                        deckmaste_construction_core::macro_def::SubtypeCategory::Spell => {
                            "spell_subtype"
                        }
                    }
                }
                _ => continue,
            };
            let grammar = raw.metadata().grammar.as_ref();
            let Some(Grammar::Noun {
                plural: DerivedSurface::Override(surface),
                ..
            }) = grammar
            else {
                continue;
            };
            actual.push((format!("lexeme:{kind_key}/{name}"), surface.clone()));
        }

        assert_eq!(
            actual,
            expected_irregulars()
                .into_iter()
                .filter(|(identity, _)| { !identity.starts_with("lexeme:CommonNoun/") })
                .map(|(identity, rows)| (identity.to_owned(), rows[0].1.to_owned()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn schema6_preserves_legacy_field_order_before_structural_inventory() {
        let report = build_report_from_source(PRODUCTION_SOURCE)
            .expect("production declaration report builds");
        let rendered = render_json(&report).expect("production report serializes");

        assert_eq!(
            raw_top_level_keys(&rendered),
            [
                "schema_version",
                "construction_count",
                "card_name_catalog_rows",
                "parenthetical_inventory",
                "noun_morphology",
                "licensing_checkers",
                "mapping_layers",
                "handwritten_codecs",
                "stored_form_tags",
                "stored_spelling_codecs",
                "morphology_irregulars",
                "selection_exceptions",
                "terminal_bindings",
                "checked_constructor_bindings",
                "roots",
                "abstract_products",
                "abstract_sums",
                "optional_roles",
                "sequence_roles",
                "sequence_feature_roles",
                "uniform_separators",
                "positional_separator_tables",
                "terminators",
                "stored_separator_fields",
            ]
        );
        assert!(has_legacy_morphology_field_order(&rendered));
    }

    #[test]
    fn legacy_order_oracle_rejects_reordered_and_nested_decoys() {
        let reordered = r#"{
            "stored_spelling_codecs": [],
            "decoy": { "morphology_irregulars": [] },
            "selection_exceptions": [],
            "morphology_irregulars": []
        }"#;

        assert_eq!(
            raw_top_level_keys(reordered),
            [
                "stored_spelling_codecs",
                "decoy",
                "selection_exceptions",
                "morphology_irregulars",
            ]
        );
        assert!(!has_legacy_morphology_field_order(reordered));
    }

    #[test]
    fn builtin_noun_morphology_census_reports_changed_totals_but_keeps_its_partition_law() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
        let census = builtin_noun_morphology_census(&root)
            .expect("authenticated builtin-v2 noun sources census cleanly");

        assert_eq!(
            census.total,
            census.derived_plural + census.explicit_plural + census.unavailable_plural
        );

        let unclassifiable = validate_noun_morphology_census(NounMorphologyCensus {
            total: census.total + 1,
            ..census
        })
        .expect_err("the partition law still rejects an unclassifiable surface");
        assert!(
            unclassifiable
                .to_string()
                .contains("unclassifiable surface")
        );
    }

    fn identities(entries: &[CountedEntry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.identity.as_str())
            .collect()
    }

    fn raw_top_level_keys(rendered: &str) -> Vec<String> {
        struct Keys(Vec<String>);

        impl<'de> serde::Deserialize<'de> for Keys {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct KeysVisitor;

                impl<'de> serde::de::Visitor<'de> for KeysVisitor {
                    type Value = Keys;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("a top-level JSON object")
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut keys = Vec::new();
                        while let Some(key) = map.next_key::<String>()? {
                            keys.push(key);
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                        Ok(Keys(keys))
                    }
                }

                deserializer.deserialize_map(KeysVisitor)
            }
        }

        Keys::deserialize(&mut serde_json::Deserializer::from_str(rendered))
            .expect("rendered report is a top-level JSON object")
            .0
    }

    fn has_legacy_morphology_field_order(rendered: &str) -> bool {
        raw_top_level_keys(rendered).windows(3).any(|keys| {
            keys == [
                "stored_spelling_codecs",
                "morphology_irregulars",
                "selection_exceptions",
            ]
        })
    }

    fn morphology_irregular(identity: &str, overrides: &[(&str, &str)]) -> MorphologyIrregular {
        MorphologyIrregular {
            identity: identity.to_owned(),
            overrides: overrides
                .iter()
                .map(|&(feature, surface)| MorphologyOverride {
                    feature: feature.to_owned(),
                    surface: surface.to_owned(),
                })
                .collect(),
        }
    }

    fn morphology_identities(entries: &[MorphologyIrregular]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.identity.as_str())
            .collect()
    }

    fn expected_irregulars() -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
        vec![
            ("lexeme:CommonNoun/Ability", vec![("plural", "abilities")]),
            ("lexeme:CommonNoun/Copy", vec![("plural", "copies")]),
            ("lexeme:CommonNoun/Library", vec![("plural", "libraries")]),
            ("lexeme:CommonNoun/Tax", vec![("plural", "taxes")]),
            (
                "lexeme:CommonNoun/Toughness",
                vec![("plural", "toughnesses")],
            ),
            ("lexeme:CommonNoun/Die", vec![("plural", "dice")]),
            (
                "lexeme:artifact_subtype/equipment",
                vec![("plural", "Equipment")],
            ),
            (
                "lexeme:artifact_subtype/spacecraft",
                vec![("plural", "Spacecraft")],
            ),
            (
                "lexeme:creature_subtype/aetherborn",
                vec![("plural", "Aetherborn")],
            ),
            ("lexeme:creature_subtype/ally", vec![("plural", "Allies")]),
            ("lexeme:creature_subtype/army", vec![("plural", "Armies")]),
            ("lexeme:creature_subtype/dwarf", vec![("plural", "Dwarves")]),
            (
                "lexeme:creature_subtype/eldrazi",
                vec![("plural", "Eldrazi")],
            ),
            ("lexeme:creature_subtype/elf", vec![("plural", "Elves")]),
            ("lexeme:creature_subtype/fish", vec![("plural", "Fish")]),
            ("lexeme:creature_subtype/fungus", vec![("plural", "Fungi")]),
            ("lexeme:creature_subtype/hero", vec![("plural", "Heroes")]),
            (
                "lexeme:creature_subtype/kithkin",
                vec![("plural", "Kithkin")],
            ),
            (
                "lexeme:creature_subtype/mercenary",
                vec![("plural", "Mercenaries")],
            ),
            (
                "lexeme:creature_subtype/merfolk",
                vec![("plural", "Merfolk")],
            ),
            ("lexeme:creature_subtype/mouse", vec![("plural", "Mice")]),
            ("lexeme:creature_subtype/myr", vec![("plural", "Myr")]),
            (
                "lexeme:creature_subtype/octopus",
                vec![("plural", "Octopuses")],
            ),
            ("lexeme:creature_subtype/ox", vec![("plural", "Oxen")]),
            (
                "lexeme:creature_subtype/pegasus",
                vec![("plural", "Pegasi")],
            ),
            (
                "lexeme:creature_subtype/samurai",
                vec![("plural", "Samurai")],
            ),
            (
                "lexeme:creature_subtype/treefolk",
                vec![("plural", "Treefolk")],
            ),
            (
                "lexeme:creature_subtype/werewolf",
                vec![("plural", "Werewolves")],
            ),
            ("lexeme:creature_subtype/wolf", vec![("plural", "Wolves")]),
            ("lexeme:creature_subtype/zubera", vec![("plural", "Zubera")]),
            ("lexeme:land_subtype/plains", vec![("plural", "Plains")]),
            ("lexeme:type/sorcery", vec![("plural", "sorceries")]),
        ]
    }
}
