//! Lean registry tables, derived from the `plugins_v2/builtin` declarations.
//!
//! A declaration's BODY is its `Definition` node (counters, subtypes,
//! designations) or its `Ability.keyword` term (keyword abilities), and every
//! column those carry is read from them: a counter's name and holder, a
//! subtype's identity, a designation's six columns, a keyword's word and the
//! categories its definition is written in [CR#702.1], and — across families —
//! which keywords a counter is a counter of [CR#122.1b].
//!
//! Two overlays remain, and each is a column no declaration carries: the
//! keyword-ability GATE columns (`overlay`), and `subtype_rows`' frame column,
//! which describes the card frame a subtype sits on [CR#714.1,715.1,709.5j] and
//! which no conferral expresses. `semantics-v2-definition-bodies` records why
//! each stands.
//!
//! The keyword actions' checker columns are not here at all: `Check/Words.lean`
//! hand-writes that table, and this module emits only the label list
//! `Proofs/Tables.lean` pins it against.
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
use deckmaste_semantics_v2::abilities::Ability;
use deckmaste_semantics_v2::abilities::StaticSpec;
use deckmaste_semantics_v2::designations;
use deckmaste_semantics_v2::keywords;
use deckmaste_semantics_v2::phrase::NounPhrase;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::ron::macro_set;
use deckmaste_semantics_v2::rules::Conferral;
use deckmaste_semantics_v2::rules::Definition;
use deckmaste_semantics_v2::words::CardType;
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
use super::overlay;
use crate::expansions::Samples;
use crate::expansions::Uninstantiated;
use crate::expansions::instantiate;

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

/// A keyword-ability declaration's built term, read for its shape alone: the
/// keyword it defines and how many abilities its definition holds. The
/// abilities may be written in macros and may hold `Param(n)` holes, so they
/// go past unread here; their categories are read from the declaration's
/// INVOKED term, through the plugin's own macros (`keyword_categories`).
#[derive(Deserialize)]
#[expect(
    dead_code,
    reason = "a field is declared so the reader accepts it and skips its value"
)]
enum KeywordShape {
    Keyword {
        keyword: String,
        params: IgnoredAny,
        body: Vec<IgnoredAny>,
    },
}

/// The category of one ability of a keyword's definition
/// [CR#113.3b,113.3c,113.3d]. A definition is written in one of the three
/// [CR#702.1], so a body ability of any other shape is a refusal.
fn category(ability: &Ability) -> anyhow::Result<&'static str> {
    Ok(match ability {
        Ability::Static { .. } => ".static",
        Ability::Triggered { .. } => ".triggered",
        Ability::Activated { .. } => ".activated",
        other => anyhow::bail!("a keyword definition is static, triggered or activated: {other:?}"),
    })
}

/// The categories of a keyword ability's definition, read from the term the
/// declaration builds when it is invoked with the deterministic sample
/// arguments `cargo xtask expansions` prints it with — the abilities as the
/// reader expands them, macros and all.
fn keyword_categories(
    plugin: &Plugin,
    samples: &Samples,
    name: &str,
) -> anyhow::Result<Vec<&'static str>> {
    let declaration = plugin
        .declarations
        .get(&(
            macro_ron::Ident::from(keywords::KEYWORD_ABILITY),
            macro_ron::Ident::from(name),
        ))
        .with_context(|| format!("{name}: the plugin registers no such keyword ability"))?;
    let read = |invocation: &str| {
        plugin
            .macros
            .read_str::<Ability>(invocation)
            .map_err(|error| error.to_string())
    };
    let ability = match instantiate(samples, name, &declaration.definition.params, read) {
        Ok(instantiated) => instantiated.value,
        Err(Uninstantiated::NoSlots(reason)) => {
            anyhow::bail!("{name}: the declaration cannot be invoked: {reason}")
        }
        Err(Uninstantiated::NoneRead { first_error, .. }) => {
            anyhow::bail!("{name}: no sample invocation reads: {first_error}")
        }
    };
    let Ability::Keyword { body, .. } = ability else {
        anyhow::bail!("{name}: the declaration does not build a keyword term");
    };
    body.iter().map(category).collect()
}

/// Every keyword-ability declaration's definition, in declaration order.
///
/// The keyword and the ability count come from the built term itself. A
/// definition with no abilities has no categories and is never invoked —
/// `gift` and the declarations taking an `Ability` argument have no sample
/// invocation that reads, and need none. One with abilities is invoked
/// through `plugin_dir`'s loaded plugin, and failing to invoke it is an
/// error, never an empty row.
pub(super) fn keyword_definitions(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<KeywordDefinition>> {
    let mut plugin = None;
    let mut result = Vec::new();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAbility)
    {
        let name = declared.identity().name();
        let body = declared
            .body()
            .with_context(|| format!("{name}: keyword ability has no declaration body"))?;
        let params = declared
            .params()
            .unwrap_or_default()
            .iter()
            .map(|param| macro_ron::ParamType::plain(param.as_str()))
            .collect();
        let term = keywords::keyword_ability_body(
            name,
            &macro_ron::Params::Positional(params),
            body.get_ron(),
        )
        .map_err(|reason| anyhow::anyhow!("{name}: {reason}"))?;
        let KeywordShape::Keyword { keyword, body, .. } = macro_set()
            .read_str(&term)
            .with_context(|| format!("reading keyword ability {name}"))?;
        let categories = if body.is_empty() {
            Vec::new()
        } else {
            if plugin.is_none() {
                plugin = Some(
                    Plugin::load(plugin_dir)
                        .with_context(|| format!("loading {}", plugin_dir.display()))?,
                );
            }
            let plugin = plugin.as_ref().expect("loaded above");
            keyword_categories(plugin, &Samples::new(&plugin.macros), name)?
        };
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

/// Every counter declaration's name and `Definition` node, in declaration
/// order. The meta-macro derives the body from the declaration's name, holder
/// and conferrals, and the conferrals may be written over the builtin helpers
/// (`macros/conferrals/`), so the body is read through the builtin plugin's
/// macros rather than the bare dialect.
fn counter_definitions(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<(String, Definition)>> {
    let plugin =
        Plugin::load(plugin_dir).with_context(|| format!("loading {}", plugin_dir.display()))?;
    let mut result = Vec::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::CounterKind)
    {
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: counter has no declaration body"))?;
        let body: Definition = plugin
            .macros
            .read_str(body.get_ron())
            .with_context(|| format!("reading counter {name}"))?;
        result.push((name.to_owned(), body));
    }
    Ok(result)
}

/// The keywords a counter declaration makes a counter of [CR#122.1b]. The
/// registry declares one counter per keyword the rule names, so eligibility is
/// read off the counter family rather than restated on the keyword row: a
/// keyword counter is the counter whose conferral grants its bearer the
/// keyword ("causes that object to gain that keyword").
pub(super) fn counter_eligible_keywords(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    for (_, body) in counter_definitions(declarations, plugin_dir)? {
        let Definition::Counter { confers, .. } = body else {
            continue;
        };
        for conferral in confers {
            if let Conferral::Property {
                spec:
                    StaticSpec::AbilityGrant {
                        subject: NounPhrase::This,
                        ability,
                    },
            } = conferral
                && let Ability::Keyword { keyword, .. } = *ability
            {
                result.insert(keyword);
            }
        }
    }
    Ok(result)
}

fn keyword_rows(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    let overlays = overlay();
    let definitions = keyword_definitions(declarations, plugin_dir)?;
    let counter_keywords = counter_eligible_keywords(declarations, plugin_dir)?;
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

/// Every keyword-action declaration's checker label, in declaration order.
///
/// The label is the one the declaration's own `Enact` deed carries
/// ([`keywords::keyword_action_label`]): the words the camelCase name spells,
/// each capitalized, so `collectEvidence` is "Collect Evidence".
///
/// The columns themselves are the checker's own data, hand-written as
/// `actFacts` in `Check/Words.lean`; this list is what `Proofs/Tables.lean`
/// pins that table against in both directions, so a declaration added without
/// a row (or a row left behind by a retired declaration) fails the Lean build.
fn action_labels(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAction)
    {
        let label = keywords::keyword_action_label(declared.identity().name());
        anyhow::ensure!(
            seen.insert(label.clone()),
            "{}: two keyword actions share the checker label {label}",
            declared.identity().name()
        );
        result.push(quoted(&label));
    }
    Ok(result)
}

fn counter_rows(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    for (name, body) in counter_definitions(declarations, plugin_dir)? {
        let Definition::Counter { kind, holder, .. } = body else {
            anyhow::bail!("{name}: a counter declaration defines a counter");
        };
        // `counterFacts` is the table `Check/Words.lean` looks every counter
        // kind up in, and a kind is its declaration's name, so every counter
        // declaration is one row labelled with that name.
        anyhow::ensure!(
            kind.name() == name,
            "{name}: a counter declaration's kind is its own name, not `{}`",
            kind.name()
        );
        result.push(format!(
            "\u{27e8}{}, {}\u{27e9}",
            quoted(kind.name()),
            kind_of(&holder)?
        ));
    }
    Ok(result)
}

/// Every designation the designation declarations define, in declaration
/// order, beside the declaration that defines it.
///
/// A declaration's meta-macro builds its definition from its name and columns
/// and hands it over beside its `members`; one definition per member where it
/// lists them (`dayNight` lists `day` and `night`), else the one, labelled
/// with the declaration's own name ([`designations::designation_definitions`],
/// the reader's own expansion). Each must be labelled with the name it is
/// defined under, which is the name `designationTable` is looked up by.
fn designation_definitions(
    declarations: &[NormalizedDeclaration],
) -> anyhow::Result<Vec<(String, Definition)>> {
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
        let defined = designations::designation_definitions(name, body.get_ron())
            .map_err(|reason| anyhow::anyhow!("reading designation {name}: {reason}"))?;
        anyhow::ensure!(
            !defined.is_empty(),
            "{name}: designation declares no fact row"
        );
        for (label, text) in defined {
            let definition: Definition = dialect
                .read_str(&text)
                .with_context(|| format!("reading designation {name}"))?;
            let Definition::Designation { label: read, .. } = &definition else {
                anyhow::bail!("{name}: a designation declaration defines designations");
            };
            anyhow::ensure!(
                read.name() == label,
                "{name}: a designation's label is the name it is declared under (`{label}`), \
                 not `{}`",
                read.name()
            );
            result.push((name.to_owned(), definition));
        }
    }
    Ok(result)
}

/// Every designation name the designation declarations declare, in
/// declaration order: a declaration's own name, or its members'.
pub(super) fn designation_declared_labels(
    declarations: &[NormalizedDeclaration],
) -> anyhow::Result<Vec<String>> {
    designation_definitions(declarations)?
        .into_iter()
        .map(|(name, definition)| match definition {
            Definition::Designation { label, .. } => Ok(label.name().to_owned()),
            _ => anyhow::bail!("{name}: a designation declaration defines designations"),
        })
        .collect()
}

fn designation_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    for (name, definition) in designation_definitions(declarations)? {
        // `designationTable` is the table `Check/Words.lean` looks every
        // designation up in, by name, so every designation is one row
        // labelled with its name.
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
            quoted(label.name()),
            optional(in_zone.map(zone)),
            optional(r#type.map(card_type)),
            optional(half.map(room_half)),
        ));
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
        "keywordActionLabels",
        "KeywordActionLabel",
        &action_labels(&declarations)?,
    );
    table(
        &mut out,
        "keywordFacts",
        "KeywordFacts",
        &keyword_rows(&declarations, &root.join("plugins_v2/builtin"))?,
    );
    table(
        &mut out,
        "counterFacts",
        "CounterFacts",
        &counter_rows(&declarations, &root.join("plugins_v2/builtin"))?,
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

    /// The checker columns are hand-written in `Check/Words.lean`, so a new
    /// keyword action is caught by `Proofs/Tables.lean`'s two-way pin rather
    /// than here. What this side owes that pin is the label: a declaration the
    /// generator did not list could gain no row and lose none.
    #[test]
    fn new_action_reaches_the_lean_label_list() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_actions/testAction.ron");
        fs::write(
            path,
            "KeywordAction(name: \"testAction\", spelling: \"test action\")",
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let table = generated
            .split("def keywordActionLabels")
            .nth(1)
            .expect("the generated module declares the label list")
            .split("\n\n")
            .next()
            .expect("the label list ends at the blank line");
        assert!(table.contains("\"Test Action\""), "{table}");
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
            // The body's cost hole stays a cost: a declaration whose body no
            // longer fits its signature has no invocation to read.
            source
                .replace("params: [Cost]", "params:\n        [Amount]")
                .replace("Param(0)", "tapSymbol"),
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
            source
                .replace("params: [Cost]", "params: [Amount, Cost]")
                .replace("Param(0)", "Param(1)"),
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
            source
                .replace("params: [Cost]", "params: [Quality, Cost]")
                .replace("Param(0)", "Param(1)"),
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
            "KeywordAbility(name: \"TestKeyword\", params: [], spelling: \"test keyword\")",
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
        assert!(generated.contains("⟨\"poison\", .object⟩"));
        assert!(!generated.contains("⟨\"poison\", .player⟩"));
    }

    /// A counter's kind is its declaration's name, so every counter
    /// declaration is a row labelled with that name: a +X/+Y counter
    /// [CR#122.1a] and a keyword counter [CR#122.1b] as much as a marker
    /// counter [CR#122.1]. (Re-spelled from
    /// `a_counter_carrying_a_conferral_payload_declares_no_named_row`, whose
    /// kind shapes are retired.)
    #[test]
    fn every_counter_declares_a_row_labelled_with_its_name() {
        let generated = render(fixture().path()).unwrap();
        for row in [
            "⟨\"chargeCounter\", .object⟩",
            "⟨\"p1p1Counter\", .object⟩",
            "⟨\"flyingCounter\", .object⟩",
            "⟨\"poison\", .player⟩",
        ] {
            assert!(generated.contains(row), "{row}");
        }
        assert!(!generated.contains("\"Charge\""));
    }

    /// A keyword counter is the counter whose conferral grants the keyword
    /// [CR#122.1b], so the keyword's eligibility follows the conferral rather
    /// than the counter's name.
    #[test]
    fn a_keyword_counter_is_read_off_its_conferral() {
        let declarations = macro_def::read_builtin_v2(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
        )
        .unwrap();
        let keywords = counter_eligible_keywords(
            &declarations,
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
        )
        .unwrap();
        assert_eq!(keywords.len(), 15, "{keywords:?}");
        assert!(keywords.contains("Flying") && keywords.contains("Shadow"));
        assert!(!keywords.contains("Charge"));
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
                .replace("zone: Battlefield,", "zone: Graveyard,\n    half: Left,"),
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

    /// A designation whose declaration names no designation the checker can
    /// look up is refused. Re-spelled from
    /// `a_designation_declaring_no_row_is_refused`: a declaration no longer
    /// writes its rows (the meta builds one from its name, or one per
    /// member), so the way left to declare no findable row is a member that
    /// is not a name — a quoted label, as every declaration wrote before.
    #[test]
    fn a_designation_member_that_is_no_name_is_refused() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/designations/goaded.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source.replace(
                "zone: Battlefield,",
                "zone: Battlefield,\n    members: [\"goaded\"],",
            ),
        )
        .unwrap();
        let error = format!("{:#}", render(temp.path()).unwrap_err());
        assert!(
            error.contains("designations/goaded.ron")
                && error.contains("a designation member is a bare name"),
            "{error}"
        );
    }

    /// A body is read through the v2 dialect, so a constructor or helper
    /// applied in its declared binder order is the same row as the same one
    /// applied by name — which is what lets the four stub families write the
    /// dialect. (The counter half was a counter body's `Counter(…)`; a counter
    /// declaration no longer writes its body, so it is re-spelled over the
    /// conferral helper a keyword counter writes, whose row is the keyword's
    /// `counterEligible` column.)
    #[test]
    fn a_positional_body_reads_to_the_same_row() {
        const BODIES: [(&str, &str, &str); 2] = [
            (
                "designations/citysBlessing.ron",
                "scope: HeldBy(holder: Player)",
                "scope: HeldBy(Player)",
            ),
            (
                "counter_kinds/shadowCounter.ron",
                "grants(ability: keyword(label: \"Shadow\"))",
                "grants(keyword(\"Shadow\"))",
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
            by_name.contains("label := \"citysBlessing\", scope := .heldBy .player"),
            "the designation row moved"
        );
        assert!(
            by_name.contains(
                "{ word := \"Shadow\", argumentSchemas := [[]], counterEligible := true,"
            ),
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
