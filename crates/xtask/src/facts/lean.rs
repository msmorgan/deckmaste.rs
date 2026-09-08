//! Lean registry tables, derived from the `plugins_v2/builtin` declarations.
//!
//! A declaration's BODY is its `Definition` node (counters, subtypes,
//! designations) or its `Ability.keyword` term (keyword abilities), and every
//! column those carry is read from them: a counter's kind and holder, a
//! subtype's identity, a designation's six columns, a keyword's word and the
//! categories its definition is written in [CR#702.1], and — across families —
//! which keywords a counter is a counter of [CR#122.1b].
//!
//! Two overlays remain, and each is a column no declaration carries:
//! the keyword-ability GATE columns (`overlay`), and the keyword-action
//! checker columns (`action_overlay`), whose declarations' bodies are the
//! actions' own instructions. `subtype_rows`' frame column is a third: it
//! describes the card frame a subtype sits on [CR#714.1,715.1,709.5j], which
//! no conferral expresses. `semantics-v2-definition-bodies` records why each
//! stands.
//!
//! A body is read through `deckmaste_semantics_v2::ron::macro_set` — the same
//! reader configuration the v2 corpus uses — so a body written in the dialect
//! (positional application, bare embeds, bare numerals) reads here too.

use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

use anyhow::Context;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::{self};
use deckmaste_semantics_v2::ron::macro_set;
use deckmaste_semantics_v2::rules::Definition;
use deckmaste_semantics_v2::words::CardType;
use deckmaste_semantics_v2::words::CounterKind;
use deckmaste_semantics_v2::words::DesignationScope;
use deckmaste_semantics_v2::words::Kind;
use deckmaste_semantics_v2::words::RoomHalf;
use deckmaste_semantics_v2::words::Subtype;
use deckmaste_semantics_v2::words::Zone;
use serde::Deserialize;
use serde::de::IgnoredAny;

use super::KEYWORD_ROWS_EXEMPT;
use super::KEYWORD_STUBS_EXEMPT;
use super::Regime;
use super::Shape;
use super::action_overlay::ACTION_OVERLAY;
use super::normalize;
use super::overlay;

pub(super) const GENERATED: &str = "lean/Semantics/Check/Facts.lean";

fn quoted(value: &str) -> String {
    serde_json::to_string(value).expect("a string serializes to JSON")
}

/// A fact column's `Kind` as Lean's constructor. Only the two Entity kinds
/// [CR#109.1,102.1] carry a designation or a counter.
fn kind_of(kind: &Kind) -> anyhow::Result<&'static str> {
    match kind {
        Kind::Object => Ok(".object"),
        Kind::Player => Ok(".player"),
        other => anyhow::bail!("a fact column holds no {other:?}"),
    }
}

fn zone(zone: Zone) -> &'static str {
    match zone {
        Zone::Battlefield => ".battlefield",
        Zone::Graveyard => ".graveyard",
        Zone::Exile => ".exile",
        Zone::Hand => ".hand",
        Zone::Library => ".library",
        Zone::Stack => ".stack",
        Zone::Command => ".command",
    }
}

fn card_type(card_type: CardType) -> &'static str {
    match card_type {
        CardType::Creature => ".creature",
        CardType::Artifact => ".artifact",
        CardType::Land => ".land",
        CardType::Enchantment => ".enchantment",
        CardType::Instant => ".instant",
        CardType::Sorcery => ".sorcery",
        CardType::Planeswalker => ".planeswalker",
        CardType::Battle => ".battle",
        CardType::Kindred => ".kindred",
    }
}

fn room_half(half: RoomHalf) -> &'static str {
    match half {
        RoomHalf::Left => ".left",
        RoomHalf::Right => ".right",
    }
}

/// An optional column as Lean writes it.
fn optional(value: Option<&'static str>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| format!("some {value}"))
}

fn argument_schema(params: &[&str]) -> anyhow::Result<String> {
    let slots = params
        .iter()
        .map(|param| {
            let (shape, domain) = match *param {
                "Cost" => (".cost", "none"),
                "Quality" => (
                    ".quality",
                    if params == ["Quality", "Cost"] { "some .object" } else { "none" },
                ),
                "Subject" => (".subject", "none"),
                "Amount" => (".number", "none"),
                "Ability" => (".ability", "none"),
                "Condition" => (".deckCondition", "none"),
                other => anyhow::bail!("unsupported keyword argument type {other}"),
            };
            Ok(format!("⟨{shape}, {domain}⟩"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(format!("[{}]", slots.join(", ")))
}

// The existing variant overlay is also consumed by the frozen Idris generator.
// Lean emits each variant as an ordinary ordered schema.
fn variant_params(shape: Shape) -> &'static [&'static str] {
    match shape {
        Shape::Nothing => &[],
        Shape::Cost => &["Cost"],
        Shape::Quality => &["Quality"],
        Shape::Subject => &["Subject"],
        Shape::Number => &["Amount"],
        Shape::Ability => &["Ability"],
        Shape::CompoundQuality => &["Quality", "Cost"],
        Shape::CompoundNumber => &["Amount", "Cost"],
        Shape::DeckCondition => &["Condition"],
    }
}

fn add_schema(schemas: &mut Vec<String>, params: &[&str]) -> anyhow::Result<()> {
    let schema = argument_schema(params)?;
    if !schemas.contains(&schema) {
        schemas.push(schema);
    }
    // The quality-qualified cost variant also admits its unqualified cost.
    if params == ["Quality", "Cost"] {
        add_schema(schemas, &["Cost"])?;
    }
    Ok(())
}

fn table(out: &mut String, name: &str, ty: &str, rows: &[String]) {
    writeln!(
        out,
        "def {name} : List {ty} :=\n  [ {} ]\n",
        rows.join(",\n    ")
    )
    .unwrap();
}

/// The keyword a keyword-ability declaration defines, with the abilities its
/// definition is written as [CR#702.1] — the `Ability.keyword` term its body
/// IS. Both columns are the declaration's own; the gate columns beside them
/// are not (see `overlay`).
pub(super) struct KeywordDefinition {
    pub(super) word: String,
    pub(super) categories: Vec<&'static str>,
    pub(super) params: Vec<String>,
}

/// A keyword-ability declaration's body, read for its shape alone.
///
/// The body is an `Ability.keyword` term whose payload may write `Param(n)` —
/// a parameter reference only a macro EXPANSION resolves — so it cannot be
/// read as a `deckmaste_semantics_v2::abilities::Ability`. What the table
/// needs of it is the keyword it defines and the categories its definition is
/// written in [CR#702.1], and both are visible without the payload: serde
/// skips a field these types do not declare, so every `Param` inside one goes
/// past as an ignored value.
#[derive(Deserialize)]
#[expect(
    dead_code,
    reason = "a field is declared so the reader accepts it and skips its value"
)]
enum KeywordBody {
    Keyword {
        keyword: String,
        params: IgnoredAny,
        body: Vec<DefinitionAbility>,
    },
}

/// One ability of a keyword's definition, read for its category alone
/// [CR#113.3b,113.3c,113.3d]. A definition is written in one of the three
/// [CR#702.1], so a body ability of any other shape is a refusal.
#[derive(Deserialize)]
#[expect(
    dead_code,
    reason = "a field is declared so the reader accepts it and skips its value"
)]
enum DefinitionAbility {
    Static {
        spec: IgnoredAny,
    },
    Triggered {
        event: IgnoredAny,
        alternatives: IgnoredAny,
        r#while: IgnoredAny,
        joins: IgnoredAny,
        timing: IgnoredAny,
        limit: IgnoredAny,
        intervening: IgnoredAny,
        instruction: IgnoredAny,
    },
    Activated {
        cost: IgnoredAny,
        instruction: IgnoredAny,
        timing: IgnoredAny,
        limit: IgnoredAny,
        guard: IgnoredAny,
        activator: IgnoredAny,
    },
}

impl DefinitionAbility {
    fn category(&self) -> &'static str {
        match self {
            DefinitionAbility::Static { .. } => ".static",
            DefinitionAbility::Triggered { .. } => ".triggered",
            DefinitionAbility::Activated { .. } => ".activated",
        }
    }
}

/// Every keyword-ability declaration's definition, in declaration order.
pub(super) fn keyword_definitions(
    declarations: &[NormalizedDeclaration],
) -> anyhow::Result<Vec<KeywordDefinition>> {
    let mut result = Vec::new();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAbility)
    {
        let name = declared.identity().name();
        let body = declared
            .body()
            .with_context(|| format!("{name}: keyword ability has no declaration body"))?;
        let KeywordBody::Keyword { keyword, body, .. } = macro_set()
            .read_str(body.get_ron())
            .with_context(|| format!("reading keyword ability {name}"))?;
        let categories = body.iter().map(DefinitionAbility::category).collect();
        result.push(KeywordDefinition {
            word: keyword,
            categories,
            params: declared
                .params()
                .unwrap_or_default()
                .iter()
                .map(|param| macro_def::ParameterType::as_str(param).to_owned())
                .collect(),
        });
    }
    Ok(result)
}

/// The keywords a counter declaration makes a counter of [CR#122.1b]. The
/// registry declares one counter per keyword the rule names, so eligibility is
/// read off the counter family rather than restated on the keyword row.
pub(super) fn counter_eligible_keywords(
    declarations: &[NormalizedDeclaration],
) -> anyhow::Result<BTreeSet<String>> {
    let dialect = macro_set();
    let mut result = BTreeSet::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::CounterKind)
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: counter has no declaration body"))?;
        let body: Definition = dialect
            .read_str(body.get_ron())
            .with_context(|| format!("reading counter {name}"))?;
        if let Definition::Counter {
            kind: CounterKind::Keyword { keyword },
            ..
        } = body
        {
            result.insert(keyword);
        }
    }
    Ok(result)
}

fn keyword_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let overlays = overlay();
    let definitions = keyword_definitions(declarations)?;
    let counter_keywords = counter_eligible_keywords(declarations)?;
    for definition in &definitions {
        anyhow::ensure!(
            overlays.iter().any(|row| row.label == definition.word)
                || KEYWORD_STUBS_EXEMPT
                    .iter()
                    .any(|exempt| exempt.label == definition.word),
            "{}: keyword declaration has no gate-column overlay",
            definition.word
        );
    }
    overlays
        .iter()
        .map(|row| {
            let declared = definitions
                .iter()
                .find(|definition| definition.word == row.label);
            anyhow::ensure!(
                declared.is_some()
                    || KEYWORD_ROWS_EXEMPT
                        .iter()
                        .any(|exempt| exempt.label == row.label),
                "{}: keyword overlay has no registry declaration",
                row.label
            );
            let mut schemas = Vec::new();
            if let Some(declared) = declared {
                let params = declared
                    .params
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>();
                add_schema(&mut schemas, &params)
                    .with_context(|| format!("{}: invalid argument schema", row.label))?;
            }
            for extra in row.extra {
                add_schema(&mut schemas, variant_params(*extra))?;
            }
            anyhow::ensure!(
                !schemas.is_empty(),
                "{}: no admitted argument schema",
                row.label
            );
            let mut fields = vec![
                format!("word := {}", quoted(row.label)),
                format!("argumentSchemas := [{}]", schemas.join(", ")),
            ];
            if counter_keywords.contains(row.label) {
                fields.push("counterEligible := true".into());
            }
            for (set, field) in [
                (row.functions_on_stack, "functionsOnStack"),
                (row.on_spell_card, "onInstantOrSorceryCard"),
                (row.paid_cost, "paidCost"),
                (row.wants_modes, "wantsModes"),
            ] {
                if set {
                    fields.push(format!("{field} := true"));
                }
            }
            if !row.on_permanent_card {
                fields.push("onPermanentCard := false".into());
            }
            // The categories a keyword's definition is written in [CR#702.1]
            // are the categories of the definition the declaration WRITES.
            // Where a declaration writes none — the workbench cannot spell it,
            // and the file records the STOP — the CR entry still names them,
            // and only those rows keep an overlay value.
            let categories = declared
                .map(|declared| declared.categories.clone())
                .filter(|categories| !categories.is_empty())
                .unwrap_or_else(|| row.definition.iter().map(|c| c.lean()).collect());
            if !categories.is_empty() {
                fields.push(format!("definition := [{}]", categories.join(", ")));
            }
            if let Some(regime) = row.regime {
                fields.push(format!(
                    "regime := some {}",
                    match regime {
                        Regime::AtCasting => ".atCasting",
                        Regime::AtResolution => ".atResolution",
                    }
                ));
            }
            Ok(format!("{{ {} }}", fields.join(", ")))
        })
        .collect()
}

fn action_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAction)
    {
        let name = declared.identity().name();
        let &(label, fields) = ACTION_OVERLAY
            .iter()
            .find(|(label, _)| normalize(label) == normalize(name))
            .with_context(|| format!("{name}: keyword action has no checker-column overlay"))?;
        anyhow::ensure!(
            seen.insert(label),
            "{name}: duplicate action overlay key {label}"
        );
        let fields = fields
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim();
        let body = if fields.is_empty() { "{}".to_owned() } else { format!("{{ {fields} }}") };
        result.push(format!("({}, {body})", quoted(label)));
    }
    for &(label, _) in ACTION_OVERLAY {
        anyhow::ensure!(
            seen.contains(label),
            "{label}: action overlay has no registry declaration"
        );
    }
    Ok(result)
}

fn counter_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let dialect = macro_set();
    let mut result = Vec::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::CounterKind)
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: counter has no declaration body"))?;
        let body: Definition = dialect
            .read_str(body.get_ron())
            .with_context(|| format!("reading counter {name}"))?;
        let Definition::Counter { kind, holder, .. } = body else {
            anyhow::bail!("{name}: a counter declaration defines a counter");
        };
        // `counterFacts` is the table of NAMED counters, which is what
        // `Check/Words.lean` looks a bare label up in. A counter Lean names
        // with a `CounterKind` constructor of its own — a +X/+Y counter
        // [CR#122.1a] or a keyword counter [CR#122.1b] — is not looked up by
        // label and contributes no row.
        let CounterKind::Named { label } = kind else {
            continue;
        };
        result.push(format!(
            "\u{27e8}{}, {}\u{27e9}",
            quoted(&label),
            kind_of(&holder)?
        ));
    }
    Ok(result)
}

/// Every label the designation declarations declare, in declaration order.
pub(super) fn designation_declared_labels(
    declarations: &[NormalizedDeclaration],
) -> anyhow::Result<Vec<String>> {
    let dialect = macro_set();
    let mut result = Vec::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::Designation)
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: designation has no declaration body"))?;
        let rows: Vec<Definition> = dialect
            .read_str(body.get_ron())
            .with_context(|| format!("reading designation {name}"))?;
        for definition in rows {
            let Definition::Designation { label, .. } = definition else {
                anyhow::bail!("{name}: a designation declaration defines designations");
            };
            result.push(label);
        }
    }
    Ok(result)
}

fn designation_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let dialect = macro_set();
    let mut result = Vec::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::Designation)
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: designation has no declaration body"))?;
        let rows: Vec<Definition> = dialect
            .read_str(body.get_ron())
            .with_context(|| format!("reading designation {name}"))?;
        anyhow::ensure!(!rows.is_empty(), "{name}: designation declares no fact row");
        for definition in rows {
            let Definition::Designation {
                label,
                scope,
                effectful,
                zone: in_zone,
                r#type,
                half,
            } = definition
            else {
                anyhow::bail!("{name}: a designation declaration defines designations");
            };
            let scope = match &scope {
                DesignationScope::HeldBy { holder } => format!(".heldBy {}", kind_of(holder)?),
                DesignationScope::HeldByCard => ".heldByCard".to_owned(),
                DesignationScope::HeldByGame => ".heldByGame".to_owned(),
            };
            result.push(format!(
                "{{ label := {}, scope := {scope}, effectful := {effectful}, zone := {}, type := {}, half := {} }}",
                quoted(&label),
                optional(in_zone.map(zone)),
                optional(r#type.map(card_type)),
                optional(half.map(room_half)),
            ));
        }
    }
    Ok(result)
}

/// The `Subtype` a subtype declaration's body defines.
///
/// Every subtype declaration's body IS its `Definition` node, derived by the
/// `Subtype`/`SpellSubtype` meta from the declaration's category and spelling
/// (`plugins-v2-subtypes-macro-only`), so there is nowhere else to read the
/// identity from and no fallback to a name.
fn subtype_of(row: &NormalizedDeclaration) -> anyhow::Result<Subtype> {
    if !matches!(row.identity().kind(), DeclarationKind::Subtype(_)) {
        anyhow::bail!("{}: not a subtype declaration", row.identity().name());
    }
    let body = row
        .body()
        .with_context(|| format!("{}: subtype has no declaration body", row.identity().name()))?;
    match macro_set().read_str::<Definition>(body.get_ron()) {
        Ok(Definition::Subtype { subtype, .. }) => Ok(subtype),
        _ => anyhow::bail!(
            "{}: subtype declaration does not define a subtype",
            row.identity().name()
        ),
    }
}

fn subtype(subtype: &Subtype) -> String {
    match subtype {
        Subtype::Spell { label } => format!(".spell {}", quoted(label)),
        Subtype::Of { host, label } => format!(".of {} {}", card_type(*host), quoted(label)),
    }
}

/// The declarations whose body is not a `Definition`. EMPTY since
/// `plugins-v2-subtypes-macro-only` derived every subtype body from its
/// declaration: the guard stays so that a subtype declaration failing to
/// define a subtype is a refusal, and so that a future exception has to be
/// written down here to exist.
const SUBTYPE_DEFINITION_STOPS: &[&str] = &[];

/// Every subtype declaration defines its subtype.
fn subtype_definitions_read(rows: &[NormalizedDeclaration]) -> anyhow::Result<()> {
    let dialect = macro_set();
    for row in rows
        .iter()
        .filter(|row| matches!(row.identity().kind(), DeclarationKind::Subtype(_)))
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: subtype has no declaration body"))?;
        let read = dialect.read_str::<Definition>(body.get_ron());
        anyhow::ensure!(
            matches!(read, Ok(Definition::Subtype { .. }))
                || SUBTYPE_DEFINITION_STOPS.contains(&name),
            "{name}: subtype declaration does not define a subtype"
        );
    }
    Ok(())
}

fn subtype_rows(rows: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    subtype_definitions_read(rows)?;
    // The frame column is not a definition-node field: it says what the CARD
    // FRAME a subtype sits on looks like [CR#714.1,715.1,709.5j], which no
    // conferral expresses. Keyed by declaration name, so no label mapping
    // stands between the overlay and the registry.
    let overlays = [
        ("saga", SubtypeCategory::Enchantment, ".chapters"),
        ("adventure", SubtypeCategory::Spell, ".adventureInset"),
        ("room", SubtypeCategory::Enchantment, ".doors"),
    ];
    overlays
        .into_iter()
        .map(|(name, category, frame)| {
            let row = rows
                .iter()
                .find(|row| {
                    row.identity().kind() == DeclarationKind::Subtype(category)
                        && row.identity().name() == name
                })
                .with_context(|| format!("{name}: frame overlay has no subtype declaration"))?;
            Ok(format!(
                "\u{27e8}{}, {frame}\u{27e9}",
                subtype(&subtype_of(row)?)
            ))
        })
        .collect()
}

pub(super) fn render(root: &Path) -> anyhow::Result<String> {
    let declarations = macro_def::read_builtin_v2(root.join("plugins_v2/builtin"))?;
    let mut out = String::from(
        "-- Generated by `cargo xtask facts generate`; edit registry declarations and xtask overlays.\nimport Semantics.Check.FactTypes\n\nnamespace Semantics\n\n",
    );
    table(
        &mut out,
        "actFacts",
        "(KeywordActionLabel × ActFacts)",
        &action_rows(&declarations)?,
    );
    table(
        &mut out,
        "keywordFacts",
        "KeywordFacts",
        &keyword_rows(&declarations)?,
    );
    table(
        &mut out,
        "counterFacts",
        "CounterFacts",
        &counter_rows(&declarations)?,
    );
    table(
        &mut out,
        "designationTable",
        "DesignationFacts",
        &designation_rows(&declarations)?,
    );
    table(
        &mut out,
        "subtypeFacts",
        "SubtypeFacts",
        &subtype_rows(&declarations)?,
    );
    out.push_str("end Semantics\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn copy_tree(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let dest = destination.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_tree(&entry.path(), &dest);
            } else {
                fs::copy(entry.path(), dest).unwrap();
            }
        }
    }

    fn fixture() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = root.join("plugins_v2/builtin/macros");
        let destination = temp.path().join("plugins_v2/builtin/macros");
        copy_tree(&source, &destination);
        temp
    }

    #[test]
    fn new_action_requires_a_checker_overlay() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_actions/TestAction.ron");
        fs::write(
            path,
            "KeywordAction(name: \"TestAction\", spelling: \"test action\")",
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("TestAction: keyword action has no checker-column overlay"),
            "{error}"
        );
    }

    #[test]
    fn declared_parameter_shapes_are_read_structurally() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/ward.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            path,
            source.replace("params: [Cost]", "params:\n        [Amount]"),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("word := \"Ward\""))
            .unwrap();
        assert!(
            row.contains("argumentSchemas := [[⟨.number, none⟩]]"),
            "{row}"
        );
        let idris = super::super::render(temp.path()).unwrap();
        let row = idris
            .lines()
            .find(|line| line.contains("word := \"Ward\""))
            .unwrap();
        assert!(row.contains("paramShapes := [NumberParam]"), "{row}");
    }

    #[test]
    fn declared_argument_lists_reach_lean_and_keep_registry_limits() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/ward.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source.replace("params: [Cost]", "params: [Amount, Cost]"),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("word := \"Ward\""))
            .unwrap();
        assert!(
            row.contains("argumentSchemas := [[⟨.number, none⟩, ⟨.cost, none⟩]]"),
            "{row}"
        );
        fs::write(
            &path,
            source.replace("params: [Cost]", "params: [Quality, Cost]"),
        )
        .unwrap();
        let reordered = render(temp.path()).unwrap();
        let row = reordered
            .lines()
            .find(|line| line.contains("word := \"Ward\""))
            .unwrap();
        assert!(
            row.contains(
                "argumentSchemas := [[⟨.quality, some .object⟩, ⟨.cost, none⟩], [⟨.cost, none⟩]]"
            ),
            "{row}"
        );

        fs::write(
            &path,
            source.replace("params: [Cost]", "params: [Cost, Amount]"),
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("has no consuming or deferred codec class"),
            "{error}"
        );
    }

    #[test]
    fn new_keyword_requires_a_gate_overlay() {
        let temp = fixture();
        fs::write(
            temp.path()
                .join("plugins_v2/builtin/macros/keyword_abilities/TestKeyword.ron"),
            "KeywordAbility(name: \"TestKeyword\", params: [], spelling: \"test keyword\", \
             body: Keyword(keyword: \"TestKeyword\", params: [], body: []))",
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("TestKeyword: keyword declaration has no gate-column overlay"),
            "{error}"
        );
    }

    #[test]
    fn counter_scope_is_read_from_the_declaration() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/counter_kinds/poison.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(path, source.replace("holder: Player", "holder: Object")).unwrap();
        let generated = render(temp.path()).unwrap();
        assert!(generated.contains("⟨\"Poison\", .object⟩"));
        assert!(!generated.contains("⟨\"Poison\", .player⟩"));
    }

    /// A counter Lean names with a `CounterKind` constructor of its own
    /// [CR#122.1a,122.1b] carries its conferral payload instead of a fact row,
    /// and the generated table of `named` counters leaves it out.
    #[test]
    fn a_counter_carrying_a_conferral_payload_declares_no_named_row() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/counter_kinds/chargeCounter.ron");
        let source = fs::read_to_string(&path).unwrap();
        assert!(
            render(temp.path())
                .unwrap()
                .contains("⟨\"Charge\", .object⟩")
        );
        fs::write(
            &path,
            source.replace(
                "kind: Named(label: \"Charge\")",
                "kind: Keyword(keyword: \"Flying\")",
            ),
        )
        .unwrap();
        assert!(!render(temp.path()).unwrap().contains("\"Charge\""));
    }

    /// Every designation column is the declaration's own, so changing one in
    /// the declaration changes the emitted row.
    #[test]
    fn designation_columns_are_read_from_the_declaration() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/designations/goaded.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source
                .replace("effectful: true", "effectful: false")
                .replace("zone: Battlefield", "zone: Graveyard")
                .replace("half: None", "half: Left"),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("label := \"goaded\""))
            .unwrap();
        assert!(
            row.contains(
                "effectful := false, zone := some .graveyard, type := none, half := some .left"
            ),
            "{row}"
        );
    }

    /// A designation whose body declares no row is a designation the checker
    /// would never find, so the generator refuses it.
    #[test]
    fn a_designation_declaring_no_row_is_refused() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/designations/goaded.ron");
        let source = fs::read_to_string(&path).unwrap();
        let start = source.find("body: [").unwrap();
        let end = source.rfind("],").unwrap();
        fs::write(
            &path,
            format!("{}body: [{}", &source[..start], &source[end..]),
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("goaded: designation declares no fact row"),
            "{error}"
        );
    }

    /// A body is read through the v2 dialect, so a constructor applied in its
    /// declared binder order is the same row as the same constructor applied
    /// by name — which is what lets the four stub families write the dialect.
    #[test]
    fn a_positional_body_reads_to_the_same_row() {
        const BODIES: [(&str, &str, &str); 2] = [
            (
                "designations/citysBlessing.ron",
                "scope: HeldBy(holder: Player)",
                "scope: HeldBy(Player)",
            ),
            (
                "counter_kinds/shieldCounter.ron",
                "Counter(kind: Named(label: \"Shield\"), holder: Object, confers: [])",
                "Counter(Named(\"Shield\"), Object, [])",
            ),
        ];

        let named = fixture();
        let positional = fixture();
        for (stub, by_name, by_order) in BODIES {
            let stub = Path::new("plugins_v2/builtin/macros").join(stub);
            let source = fs::read_to_string(named.path().join(&stub)).unwrap();
            assert!(
                source.contains(by_name) || source.contains(by_order),
                "{stub:?}"
            );
            fs::write(named.path().join(&stub), source.replace(by_order, by_name)).unwrap();
            fs::write(
                positional.path().join(&stub),
                source.replace(by_name, by_order),
            )
            .unwrap();
        }

        let by_name = render(named.path()).unwrap();
        assert!(
            by_name.contains("label := \"the city\'s blessing\", scope := .heldBy .player"),
            "the designation row moved"
        );
        assert!(
            by_name.contains("⟨\"Shield\", .object⟩"),
            "the counter row moved"
        );
        assert_eq!(by_name, render(positional.path()).unwrap());
    }

    #[test]
    fn stale_lean_module_fails_the_check() {
        let temp = fixture();
        fs::create_dir_all(temp.path().join("lean/Semantics/Check")).unwrap();
        fs::write(temp.path().join(GENERATED), "-- stale\n").unwrap();
        let error = super::super::run_check(temp.path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("Facts.lean is stale"), "{error}");
    }
}
