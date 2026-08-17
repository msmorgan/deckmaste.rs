//! Human-readable expansion of the English-v2 construction declaration.

mod audit;
mod corpus;
mod coverage_lock;
mod parse;
mod roundtrip;

use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use deckmaste_construction_core::DeclarationKind;
use deckmaste_construction_core::Expansion;
use deckmaste_construction_core::ItemKey;
use deckmaste_construction_core::NamedKind;
use deckmaste_construction_core::TerminalBindingDeclarationKind;

#[derive(Debug, Args)]
pub struct EnglishV2Args {
    #[command(subcommand)]
    command: EnglishV2Command,
}

#[derive(Debug, Subcommand)]
enum EnglishV2Command {
    /// Print generated items and counted declaration escape hatches.
    Expand,
    /// Audit every normalized corpus face with the English-v2 parser.
    Parse(ParseArgs),
    /// Gate exact rendering for every corpus face the parser accepts.
    Roundtrip(RoundtripArgs),
}

#[derive(Debug, clap::Args)]
struct CorpusArgs {
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    data: PathBuf,
    #[arg(long, default_value = "data/gen/catalogs")]
    catalogs: PathBuf,
}

#[derive(Debug, clap::Args)]
struct ParseArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    require_complete: bool,
    #[arg(long, default_value = "english-v2-coverage.lock")]
    lock: PathBuf,
    #[arg(long)]
    bless: bool,
}

#[derive(Debug, clap::Args)]
struct RoundtripArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    require_clean: bool,
}

pub fn run(args: &EnglishV2Args) -> anyhow::Result<()> {
    match &args.command {
        EnglishV2Command::Expand => {
            let mut stdout = std::io::stdout().lock();
            run_from_path(&production_declaration_path(), &mut stdout)
        }
        EnglishV2Command::Parse(args) => {
            let mut stdout = std::io::stdout().lock();
            parse::run(args, &mut stdout)
        }
        EnglishV2Command::Roundtrip(args) => {
            let mut stdout = std::io::stdout().lock();
            roundtrip::run(args, &mut stdout)
        }
    }
}

fn production_declaration_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_english_v2/src/constructions.rs")
}

fn run_from_path(path: &Path, output: &mut dyn Write) -> anyhow::Result<()> {
    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let rendered = expand_source(&source)?;
    output
        .write_all(rendered.as_bytes())
        .context("writing English-v2 declaration expansion")
}

pub fn expand_source(source: &str) -> anyhow::Result<String> {
    let expansion = expansion_from_source(source)?;
    render_expansion(&expansion)
}

fn expansion_from_source(source: &str) -> anyhow::Result<Expansion> {
    let invocation =
        deckmaste_construction_core::invocation_from_source(source).map_err(anyhow::Error::new)?;
    deckmaste_construction_core::generate(invocation.tokens).map_err(anyhow::Error::new)
}

fn render_expansion(expansion: &Expansion) -> anyhow::Result<String> {
    let mut output = String::new();
    for item in expansion.items() {
        let key = item_key_name(&item.key);
        writeln!(&mut output, "// === {key} ===").expect("writing to String cannot fail");
        for origin in &item.origins {
            writeln!(
                &mut output,
                "// origin: {} {}",
                declaration_kind_name(origin.kind()),
                origin.name()
            )
            .expect("writing to String cannot fail");
        }
        output.push_str(
            &deckmaste_construction_core::format_generated_item(item)
                .map_err(anyhow::Error::new)?,
        );
    }

    output.push_str("// === counted escape hatches ===\n");
    let report = expansion.escape_hatches();
    writeln!(
        &mut output,
        "// terminal bindings ({})",
        report.terminal_bindings().len()
    )
    .expect("writing to String cannot fail");
    for binding in report.terminal_bindings() {
        let kind = match binding.kind() {
            TerminalBindingDeclarationKind::Codec => "codec",
            TerminalBindingDeclarationKind::Identity => "identity",
        };
        writeln!(&mut output, "// - {kind} {}", binding.name())
            .expect("writing to String cannot fail");
    }
    writeln!(
        &mut output,
        "// checked constructor bindings ({})",
        report.checked_constructor_bindings().len()
    )
    .expect("writing to String cannot fail");
    for name in report.checked_constructor_bindings() {
        writeln!(&mut output, "// - construction {name}").expect("writing to String cannot fail");
    }
    writeln!(&mut output, "// roots ({})", report.roots().len())
        .expect("writing to String cannot fail");
    for name in report.roots() {
        writeln!(&mut output, "// - root {name}").expect("writing to String cannot fail");
    }
    Ok(output)
}

fn item_key_name(key: &ItemKey) -> String {
    match key {
        ItemKey::Named { kind, name } => {
            let kind = match kind {
                NamedKind::Type => "type",
                NamedKind::Trait => "trait",
                NamedKind::Function => "function",
                NamedKind::Constant => "constant",
            };
            format!("{kind} {name}")
        }
        ItemKey::Impl {
            trait_name: Some(trait_name),
            self_ty,
        } => format!("impl {trait_name} for {self_ty}"),
        ItemKey::Impl {
            trait_name: None,
            self_ty,
        } => format!("impl {self_ty}"),
    }
}

fn declaration_kind_name(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::Construction => "construction",
        DeclarationKind::Vocab => "vocab",
        DeclarationKind::Lexeme => "lexeme",
        DeclarationKind::Codec => "codec",
        DeclarationKind::Identity => "identity",
        DeclarationKind::Root => "root",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCTION_SOURCE: &str = include_str!("../../deckmaste_english_v2/src/constructions.rs");

    const EXPECTED_ITEM_KEYS: &[&str] = &[
        "type Ability",
        "type Sentence",
        "type Clause",
        "type NounPhrase",
        "type VerbPhrase",
        "type Amount",
        "type Spell",
        "type Triggered",
        "type Imperative",
        "type Declarative",
        "type WithWhere",
        "type EventClause",
        "type WhereClause",
        "type PronounNp",
        "type Common",
        "type DemonstrativeNp",
        "type TargetNp",
        "type SelfReferenceNp",
        "type CountNp",
        "type Destroy",
        "type Connive",
        "type DealDamage",
        "type GainLife",
        "type NumberAmount",
        "type VariableAmount",
        "type TriggerWord",
        "type Article",
        "type Demonstrative",
        "type Pronoun",
        "type Variable",
        "type NounLexeme",
        "type VerbLexeme",
        "impl Render for Ability",
        "impl Render for Sentence",
        "function render_sentence_body",
        "function render_clause",
        "function render_verb_phrase",
        "function render_noun_phrase",
        "function render_amount",
        "function render_trigger_word",
        "function render_article",
        "function render_demonstrative",
        "function render_pronoun",
        "function render_variable",
        "function agreement_for_noun_phrase",
        "function number_for_noun_phrase",
        "trait Visitor",
        "function walk_ability",
        "function walk_sentence",
        "function walk_clause",
        "function walk_noun_phrase",
        "function walk_verb_phrase",
        "function walk_amount",
        "function walk_noun",
        "function walk_spell",
        "function walk_triggered",
        "function walk_imperative",
        "function walk_declarative",
        "function walk_with_where",
        "function walk_event_clause",
        "function walk_where_clause",
        "function walk_pronoun_np",
        "function walk_common",
        "function walk_demonstrative_np",
        "function walk_target_np",
        "function walk_self_reference_np",
        "function walk_count_np",
        "function walk_destroy",
        "function walk_connive",
        "function walk_deal_damage",
        "function walk_gain_life",
        "function walk_number_amount",
        "function walk_variable_amount",
        "function walk_trigger_word",
        "function walk_article",
        "function walk_demonstrative",
        "function walk_pronoun",
        "function walk_variable",
        "function walk_sign",
        "function walk_self_reference_spelling",
        "function walk_noun_lexeme",
        "function walk_verb_lexeme",
        "function walk_signed_number",
        "function walk_catalog_identity",
        "type Category",
        "type Construction",
        "type RuleId",
        "impl RuleId",
        "constant RULES",
        "function build",
    ];

    #[test]
    fn production_expansion_prints_each_literal_item_key_once_with_every_origin() {
        let output = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let expansion =
            expansion_from_source(PRODUCTION_SOURCE).expect("production source validates");
        let headings = output
            .lines()
            .filter_map(|line| line.strip_prefix("// === ")?.strip_suffix(" ==="))
            .filter(|heading| *heading != "counted escape hatches")
            .collect::<Vec<_>>();

        assert_eq!(EXPECTED_ITEM_KEYS.len(), 90);
        assert_eq!(headings, EXPECTED_ITEM_KEYS);
        for (item, expected_key) in expansion.items().iter().zip(EXPECTED_ITEM_KEYS) {
            let header = format!("// === {expected_key} ===");
            assert_eq!(output.matches(&header).count(), 1, "{expected_key}");
            let after_header = output
                .split_once(&header)
                .expect("literal item header exists")
                .1;
            let item_section = after_header
                .split_once("// === ")
                .map_or(after_header, |(section, _)| section);
            let actual_origins = item_section
                .lines()
                .filter_map(|line| line.strip_prefix("// origin: "))
                .collect::<Vec<_>>();
            let expected_origins = item
                .origins
                .iter()
                .map(|origin| format!("{} {}", declaration_kind_name(origin.kind()), origin.name()))
                .collect::<Vec<_>>();
            assert_eq!(actual_origins, expected_origins, "{expected_key}");
        }
    }

    #[test]
    fn production_counted_escape_hatches_have_exact_literal_names_and_order() {
        let output = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let report = output
            .split_once("// === counted escape hatches ===\n")
            .expect("counted report heading")
            .1;
        assert_eq!(
            report,
            "// terminal bindings (5)\n\
             // - codec Noun\n\
             // - codec Sign\n\
             // - identity SelfReferenceSpelling\n\
             // - codec SignedNumber\n\
             // - identity CatalogIdentity\n\
             // checked constructor bindings (2)\n\
             // - construction Triggered\n\
             // - construction SelfReferenceNp\n\
             // roots (2)\n\
             // - root Ability\n\
             // - root Sentence\n"
        );
    }

    #[test]
    fn expansion_is_stable_and_the_generated_rust_reparses() {
        let first = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let second = expand_source(PRODUCTION_SOURCE).expect("repeat expands");
        assert_eq!(first, second);

        let parsed = syn::parse_file(&first).expect("comment headings preserve reparsable Rust");
        assert_eq!(parsed.items.len(), 90);
    }

    #[test]
    fn source_wiring_accepts_one_inline_invocation_and_preserves_core_diagnostics() {
        let output = expand_source(PRODUCTION_SOURCE).expect("one inline invocation is extracted");
        assert!(output.starts_with("// === type Ability ===\n"));
        let with_unrelated_macro = format!("helper!();\n{PRODUCTION_SOURCE}");
        assert_eq!(
            expand_source(&with_unrelated_macro).expect("unrelated macro is ignored"),
            output
        );

        for source in [
            "constructions! {} constructions! {}",
            "module::constructions! {}",
            "constructions!(include!(\"declarations.rs\"));",
        ] {
            let core_error = deckmaste_construction_core::invocation_from_source(source)
                .expect_err("core rejects fenced source")
                .to_string();
            let command_error = expand_source(source)
                .expect_err("xtask preserves core source error")
                .to_string();
            assert_eq!(command_error, core_error);
        }
    }

    #[test]
    fn validation_diagnostics_are_the_same_as_core() {
        let source = "constructions! { root Missing { punctuation = \".\"; eoi = true; standalone_render = true; } }";
        let invocation = deckmaste_construction_core::invocation_from_source(source).unwrap();
        let core_error = deckmaste_construction_core::generate(invocation.tokens)
            .expect_err("unknown root fails core validation")
            .to_string();
        let command_error = expand_source(source)
            .expect_err("unknown root fails xtask expansion")
            .to_string();
        assert_eq!(command_error, core_error);
    }

    #[test]
    fn file_runner_propagates_invalid_source_without_writing_output() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("constructions.rs");
        std::fs::write(&path, "constructions! {} constructions! {}").unwrap();
        let mut output = Vec::new();

        let error = run_from_path(&path, &mut output).expect_err("invalid input propagates");

        assert!(error.to_string().contains("exactly one"), "{error:#}");
        assert!(output.is_empty());
    }
}
