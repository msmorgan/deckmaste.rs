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
    noun_morphology: NounMorphologyCensus,
    mapping_layers: Vec<CountedEntry>,
    handwritten_codecs: Vec<CountedEntry>,
    stored_form_tags: Vec<CountedEntry>,
    stored_spelling_codecs: Vec<CountedEntry>,
    selection_exceptions: Vec<CountedEntry>,
    terminal_bindings: Vec<CountedEntry>,
    checked_constructor_bindings: Vec<CountedEntry>,
    roots: Vec<CountedEntry>,
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
    let selection_exceptions = deckmaste_english_v2::parser::selection_exception_inventory()
        .map_err(anyhow::Error::new)
        .context("validating English-v2 selection exception inventory")?;
    let builtin_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2");
    let noun_morphology = builtin_noun_morphology_census(&builtin_root)?;
    build_report(&expansion, &selection_exceptions, noun_morphology)
}

fn build_report(
    expansion: &Expansion,
    selection_exceptions: &[SelectionExceptionInfo],
    noun_morphology: NounMorphologyCensus,
) -> anyhow::Result<CountedReport> {
    let escape_hatches = expansion.escape_hatches();
    let mut report = CountedReport {
        schema_version: 1,
        noun_morphology,
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
        checked_constructor_bindings: plain_entries(
            escape_hatches.checked_constructor_bindings(),
            Some("stage 4 checked constructor binding"),
            Some("generated_invariants"),
        ),
        roots: plain_entries(
            escape_hatches.roots(),
            Some("declared parser entry point"),
            None,
        ),
    };
    validate_and_sort(&mut report)?;
    Ok(report)
}

fn builtin_noun_morphology_census(root: &Path) -> anyhow::Result<NounMorphologyCensus> {
    use macro_ron::v2::Declaration;
    use macro_ron::v2::DerivedSurface;
    use macro_ron::v2::Grammar;
    use macro_ron::v2::GrammarRecipe;

    let declarations = macro_ron::v2::read_builtin_v2(root)
        .map_err(anyhow::Error::new)
        .with_context(|| format!("authenticating builtin-v2 sources below {}", root.display()))?;
    let mut census = NounMorphologyCensus {
        total: 0,
        derived_plural: 0,
        explicit_plural: 0,
        unavailable_plural: 0,
    };

    for normalized in declarations {
        let path = normalized.provenance().path();
        let source = fs::read_to_string(path)
            .with_context(|| format!("reading authenticated noun source {}", path.display()))?;
        let raw = macro_ron::v2::ron_options()
            .from_str::<Declaration>(&source)
            .map_err(anyhow::Error::new)
            .with_context(|| format!("reparsing authenticated noun source {}", path.display()))?;
        let (name, grammar) = match &raw {
            Declaration::Type(fields) => (&fields.name, fields.grammar.as_ref()),
            Declaration::Subtype(fields) => (&fields.name, fields.grammar.as_ref()),
            Declaration::KeywordAction(_)
            | Declaration::KeywordAbility(_)
            | Declaration::CounterKind(_)
            | Declaration::Designation(_) => continue,
        };
        let Some(Grammar::Noun { plural, .. }) = grammar else {
            continue;
        };

        if normalized.identity().kind() != raw.kind() || normalized.identity().name() != name {
            bail!(
                "raw noun identity in {} diverged from authenticated normalized identity {}",
                path.display(),
                normalized.identity(),
            );
        }
        if !matches!(
            normalized.grammar().map(macro_ron::v2::GrammarRow::recipe),
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
            DerivedSurface::Override(_) => census.explicit_plural += 1,
            DerivedSurface::Unavailable => census.unavailable_plural += 1,
        }
    }
    Ok(census)
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

fn categories_mut(report: &mut CountedReport) -> [(&'static str, &mut Vec<CountedEntry>); 8] {
    [
        ("mapping layers", &mut report.mapping_layers),
        ("handwritten codecs", &mut report.handwritten_codecs),
        ("stored form tags", &mut report.stored_form_tags),
        ("stored spelling codecs", &mut report.stored_spelling_codecs),
        ("selection exceptions", &mut report.selection_exceptions),
        ("terminal bindings", &mut report.terminal_bindings),
        (
            "checked constructor bindings",
            &mut report.checked_constructor_bindings,
        ),
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
        "noun morphology (total={}, derived +s={}, explicit={}, unavailable={})",
        report.noun_morphology.total,
        report.noun_morphology.derived_plural,
        report.noun_morphology.explicit_plural,
        report.noun_morphology.unavailable_plural,
    )
    .expect("writing to String cannot fail");
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

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCTION_SOURCE: &str =
        include_str!("../../../deckmaste_english_v2/src/constructions.rs");

    fn entry(identity: &str) -> CountedEntry {
        CountedEntry {
            identity: identity.to_owned(),
            rationale: None,
            removal_target: None,
        }
    }

    fn report() -> CountedReport {
        CountedReport {
            schema_version: 1,
            noun_morphology: NounMorphologyCensus {
                total: 0,
                derived_plural: 0,
                explicit_plural: 0,
                unavailable_plural: 0,
            },
            mapping_layers: vec![entry("zeta"), entry("alpha")],
            handwritten_codecs: vec![],
            stored_form_tags: vec![],
            stored_spelling_codecs: vec![],
            selection_exceptions: vec![],
            terminal_bindings: vec![],
            checked_constructor_bindings: vec![],
            roots: vec![],
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
    fn rendering_is_deterministic_and_human_fields_are_line_safe() {
        let mut report = report();
        report.mapping_layers = vec![CountedEntry {
            identity: "first\nsecond".to_owned(),
            rationale: Some("why\tstill".to_owned()),
            removal_target: Some("target\rnext".to_owned()),
        }];
        validate_and_sort(&mut report).expect("distinct identities validate");

        assert_eq!(
            render_json(&report).expect("first JSON rendering"),
            render_json(&report).expect("repeat JSON rendering")
        );
        assert_eq!(
            render_human(&report),
            concat!(
                "English v2 counted escape hatches\n",
                "noun morphology (total=0, derived +s=0, explicit=0, unavailable=0)\n",
                "mapping layers (1)\n",
                "  - identity=\"first\\nsecond\" rationale=\"why\\tstill\" removal_target=\"target\\rnext\"\n",
                "handwritten codecs (0)\n",
                "stored form tags (0)\n",
                "stored spelling codecs (0)\n",
                "selection exceptions (0)\n",
                "terminal bindings (0)\n",
                "checked constructor bindings (0)\n",
                "roots (0)\n"
            )
        );
    }

    #[test]
    fn production_baseline_has_exact_counted_escape_hatches() {
        let report = build_report_from_source(PRODUCTION_SOURCE)
            .expect("production declaration report builds");

        let categories = [
            &report.mapping_layers,
            &report.handwritten_codecs,
            &report.stored_form_tags,
            &report.stored_spelling_codecs,
            &report.selection_exceptions,
            &report.terminal_bindings,
            &report.checked_constructor_bindings,
            &report.roots,
        ];
        assert_eq!(
            categories.map(Vec::len),
            [0, 0, 0, 1, 0, 0, 2, 2],
            "categories intentionally overlap and have no unique total"
        );
        assert!(report.handwritten_codecs.is_empty());
        assert_eq!(
            identities(&report.stored_spelling_codecs),
            ["SelfReferenceSpelling"]
        );
        assert!(report.terminal_bindings.is_empty());
        assert_eq!(
            identities(&report.checked_constructor_bindings),
            ["SelfReferenceNp", "Triggered"]
        );
        assert_eq!(identities(&report.roots), ["Ability", "Sentence"]);
    }

    #[test]
    fn production_builtin_noun_morphology_census_is_exact() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2");
        let census = builtin_noun_morphology_census(&root)
            .expect("authenticated builtin-v2 noun sources census cleanly");

        assert_eq!(census.total, 472);
        assert_eq!(census.derived_plural, 146);
        assert_eq!(census.explicit_plural, 26);
        assert_eq!(census.unavailable_plural, 300);
        assert_eq!(
            census.total,
            census.derived_plural + census.explicit_plural + census.unavailable_plural
        );
    }

    fn identities(entries: &[CountedEntry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.identity.as_str())
            .collect()
    }
}
