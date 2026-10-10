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
use deckmaste_semantics_v2::abilities::Compulsion;
use deckmaste_semantics_v2::abilities::DeonticRider;
use deckmaste_semantics_v2::abilities::StaticSpec;
use deckmaste_semantics_v2::designations;
use deckmaste_semantics_v2::keywords;
use deckmaste_semantics_v2::phrase::EventSource;
use deckmaste_semantics_v2::phrase::GameEvent;
use deckmaste_semantics_v2::phrase::NounPhrase;
use deckmaste_semantics_v2::phrase::ZoneExpr;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::ron::macro_set;
use deckmaste_semantics_v2::rules::Conferral;
use deckmaste_semantics_v2::rules::Definition;
use deckmaste_semantics_v2::words::CardType;
use deckmaste_semantics_v2::words::Deed;
use deckmaste_semantics_v2::words::DesignationScope;
use deckmaste_semantics_v2::words::KeywordLabel;
use deckmaste_semantics_v2::words::Kind;
use deckmaste_semantics_v2::words::MarkerWord;
use deckmaste_semantics_v2::words::RoomHalf;
use deckmaste_semantics_v2::words::Subtype;
use deckmaste_semantics_v2::words::Zone;
use macro_ron::MacroSet;
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
        CardType::Dungeon => ".dungeon",
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
    // An instruction is no keyword argument: gift's effect is written into
    // its definition, and its keyword term carries no argument for it.
    let slots = params
        .iter()
        .filter(|param| **param != "Instruction")
        .map(|param| {
            let (shape, domain) = match *param {
                "Cost" => (".cost", "none"),
                "Quality" => (
                    ".quality",
                    if params == ["Quality", "Cost"] { "some .object" } else { "none" },
                ),
                "Predicate" => (".subject", "none"),
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
        Shape::Subject => &["Predicate"],
        Shape::Number => &["Amount"],
        Shape::Ability => &["Ability"],
        Shape::CompoundQuality => &["Quality", "Cost"],
        Shape::CompoundNumber => &["Amount", "Cost"],
        Shape::QualityAmount => &["Quality", "Amount"],
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
    /// Each component's category; `None` is a component qualifying the
    /// ability printed with the keyword, whose category is that ability's
    /// and which the overlay's `definition` names from the CR entry
    /// [CR#702.169a] (`resolved_categories`).
    pub(super) categories: Vec<Option<&'static str>>,
    /// Each component's stack regime as its form gives it, beside
    /// `categories`.
    pub(super) regimes: Vec<FormRegime>,
    pub(super) params: Vec<String>,
    /// The signatures of the general forms carrying this keyword's label
    /// with a definition of their own (`devourQuality` for devour
    /// [CR#702.82c]): each is one more argument schema on this row.
    pub(super) general_forms: Vec<Vec<String>>,
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
        keyword: KeywordLabel,
        params: IgnoredAny,
        body: Vec<IgnoredAny>,
    },
}

/// The category of one ability of a keyword's definition
/// [CR#113.3a,113.3b,113.3c,113.3d]. A definition is written in these four
/// [CR#702.1] — a spell ability as the half a keyword represents on an instant
/// or sorcery spell [CR#702.174b] — so a body ability of any other shape is a
/// refusal.
///
/// A qualified ability has the category of the ability it wraps
/// [CR#113.10a]. Where that is the keyword's own `Ability` argument (the
/// sample invocation's argument has no category), the component's category
/// is the printed ability's, `None` here.
fn category(ability: &Ability) -> anyhow::Result<Option<&'static str>> {
    Ok(Some(match ability {
        Ability::Static { .. } => ".static",
        Ability::Triggered { .. } => ".triggered",
        Ability::Activated { .. } => ".activated",
        Ability::Spell { .. } => ".spell",
        Ability::Qualified { ability, .. } => {
            return match &**ability {
                Ability::MayBeginOnBattlefield | Ability::Keyword { .. } => Ok(None),
                inner => category(inner),
            };
        }
        other => anyhow::bail!(
            "a keyword definition is static, triggered, activated or spell: {other:?}"
        ),
    }))
}

/// The stack regime a definition component's form gives it, as Lean's
/// `Ability.regimeFits` reads it: `Open` where the form admits more than one
/// and the keyword's own rule says which (a permission to cast the card
/// [CR#702.34a,702.117a]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FormRegime {
    Determined(Option<Regime>),
    Open,
}

/// A triggered component keys on its event's regime: a cast trigger at
/// casting [CR#702.153a], any other event off the stack [CR#702.175a]
/// (Lean `bodyEventRegime`).
fn event_regime(event: &GameEvent) -> Option<Regime> {
    match event {
        GameEvent::Casts { .. } => Some(Regime::AtCasting),
        GameEvent::NthOccurrence { event, .. } | GameEvent::Causes { event, .. } => {
            event_regime(event)
        }
        _ => None,
    }
}

/// A static component's regime (Lean `StaticSpec.regimeFits`): an
/// additional cost or a cost change is determined as the spell is cast
/// [CR#601.2f], and a Mana Substitute applies as its total cost is paid
/// [CR#702.51b,601.2h], a Mode Change as its modes are chosen
/// [CR#601.2b,700.2a], and a gained target as targets are chosen
/// [CR#601.2c,702.140a]; a permission to cast the card is open; any other static
/// ability functions off the stack.
fn static_regime(spec: &StaticSpec) -> FormRegime {
    match spec {
        StaticSpec::WithBindings { body, .. } | StaticSpec::InCaller { body, .. } => {
            static_regime(body)
        }
        StaticSpec::Conditional { spec, .. } => static_regime(spec),
        // An ability's cost is determined as it is activated, not as a spell
        // is cast [CR#602.2b,601.2f] (power-up [CR#702.193a]).
        StaticSpec::CostShift {
            subject:
                NounPhrase::AsMarker {
                    marker: MarkerWord::Ability,
                    ..
                },
            ..
        } => FormRegime::Determined(None),
        StaticSpec::AddedCost { .. }
        | StaticSpec::CostShift { .. }
        | StaticSpec::PaySubstitute { .. }
        | StaticSpec::ModeChoice { .. }
        | StaticSpec::AddsTarget { .. } => FormRegime::Determined(Some(Regime::AtCasting)),
        StaticSpec::AltCost { .. } => FormRegime::Open,
        StaticSpec::Replacement {
            event:
                GameEvent::ZoneChange {
                    subject,
                    from: Some(from),
                    ..
                },
            ..
        } if leaves_the_stack(subject, from) => FormRegime::Open,
        StaticSpec::DeonticRule {
            compulsion, rider, ..
        } if matches!(**compulsion, Compulsion::Permit)
            && matches!(**rider, DeonticRider::Play { .. }) =>
        {
            FormRegime::Open
        }
        _ => FormRegime::Determined(None),
    }
}

/// Whether a zone change is the source's own move off the stack: a
/// replacement of it functions while the spell is on the stack, and the
/// keyword's own rule says which regime it takes (rebound [CR#702.88a]; Lean
/// `StaticSpec.regimeFits`).
fn leaves_the_stack(subject: &NounPhrase, from: &EventSource) -> bool {
    let EventSource::Zones { zones } = from else {
        return false;
    };
    names_this(subject)
        && matches!(
            zones.as_slice(),
            [ZoneExpr::Zone {
                zone: Zone::Stack,
                ..
            }]
        )
}

/// Lean `NounPhrase.namesThis`: the source, bare or as a type or marker view.
fn names_this(subject: &NounPhrase) -> bool {
    match subject {
        NounPhrase::This => true,
        NounPhrase::WithBindings { body, .. }
        | NounPhrase::InCaller { body, .. }
        | NounPhrase::FirstMention { phrase: body, .. }
        | NounPhrase::AsType { subject: body, .. }
        | NounPhrase::AsMarker { subject: body, .. } => names_this(body),
        _ => false,
    }
}

/// The stack regime one component of a keyword's definition takes from its
/// form (Lean `Ability.regimeFits`); a spell ability is followed as the spell
/// resolves [CR#113.3a].
fn form_regime(ability: &Ability) -> FormRegime {
    match ability {
        Ability::Qualified { ability, .. } => form_regime(ability),
        Ability::Static { spec } => static_regime(spec),
        Ability::Triggered { event, .. } => FormRegime::Determined(event_regime(event)),
        Ability::Spell { .. } => FormRegime::Determined(Some(Regime::AtResolution)),
        _ => FormRegime::Determined(None),
    }
}

/// The categories of a keyword ability's definition, read from the term the
/// declaration builds when it is invoked with the deterministic sample
/// arguments `cargo xtask expansions` prints it with — the abilities as the
/// reader expands them, macros and all.
fn keyword_categories(
    plugin: &Plugin,
    samples: &Samples,
    name: &str,
) -> anyhow::Result<Vec<(Option<&'static str>, FormRegime)>> {
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
    let Ability::Keyword { keyword, body, .. } = ability else {
        anyhow::bail!("{name}: the declaration does not build a keyword term");
    };
    // A bare form written as its general form ("devour N" is devour
    // [quality] N over creatures [CR#702.82a,702.82c]) carries the same
    // keyword, whose own components are the definition's (Lean
    // `Ability.generalFormOf`).
    body.iter()
        .flat_map(|ability| match ability {
            Ability::Keyword {
                keyword: inner,
                body,
                ..
            } if inner == &keyword => body.iter().collect::<Vec<_>>(),
            other => vec![other],
        })
        .map(|ability| Ok((category(ability)?, form_regime(ability))))
        .collect()
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
    let mut general_forms: Vec<(String, String, Vec<String>)> = Vec::new();
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
        // A labelled declaration defines no keyword, so it has no row: a
        // variant (`giftACard` is gift's `gift(draw(1))`) shares its
        // keyword's, and a general form (`devourQuality`, its own
        // definition under devour's label) adds its signature to it.
        if keywords::is_variant(body.get_ron())
            .map_err(|reason| anyhow::anyhow!("{name}: {reason}"))?
        {
            if term.starts_with("Keyword(") {
                let label = keywords::carried_label(name, body.get_ron())
                    .map_err(|reason| anyhow::anyhow!("{name}: {reason}"))?;
                general_forms.push((
                    name.to_owned(),
                    overlay_key(&label),
                    declared
                        .params()
                        .unwrap_or_default()
                        .iter()
                        .map(|param| macro_def::ParameterType::as_str(param).to_owned())
                        .collect(),
                ));
            }
            continue;
        }
        let KeywordShape::Keyword { keyword, body, .. } = macro_set()
            .read_str(&term)
            .with_context(|| format!("reading keyword ability {name}"))?;
        let components = if body.is_empty() {
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
            word: overlay_key(keyword.name()),
            categories: components.iter().map(|(category, _)| *category).collect(),
            regimes: components.iter().map(|(_, regime)| *regime).collect(),
            params: declared
                .params()
                .unwrap_or_default()
                .iter()
                .map(|param| macro_def::ParameterType::as_str(param).to_owned())
                .collect(),
            general_forms: Vec::new(),
        });
    }
    for (name, word, params) in general_forms {
        result
            .iter_mut()
            .find(|definition| definition.word == word)
            .with_context(|| format!("{name}: carries the label of no declared keyword {word}"))?
            .general_forms
            .push(params);
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

/// Every registry definition `plugin_dir` declares, by family: each counter
/// declaration's node under its name, each subtype declaration's node under
/// `<category>/<name>`, and each designation under its label (a declaration
/// listing members defines one per member). `cargo xtask lean definitions`
/// proves `Definition.check` empty for every one.
///
/// Every declaration of the three registry kinds is read, and one whose body
/// does not read as its family's `Definition` constructor fails the read: the
/// gate has no skip list.
pub(crate) fn registry_definitions(plugin_dir: &Path) -> anyhow::Result<RegistryDefinitions> {
    let declarations = macro_def::read_builtin_v2(plugin_dir)
        .with_context(|| format!("reading the declarations of {}", plugin_dir.display()))?;
    let counters = counter_definitions(&declarations, plugin_dir)?;
    for (name, definition) in &counters {
        anyhow::ensure!(
            matches!(definition, Definition::Counter { .. }),
            "{name}: a counter declaration defines a counter"
        );
    }
    let plugin =
        Plugin::load(plugin_dir).with_context(|| format!("loading {}", plugin_dir.display()))?;
    let mut subtypes = Vec::new();
    for row in &declarations {
        let DeclarationKind::Subtype(category) = row.identity().kind() else {
            continue;
        };
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: subtype has no declaration body"))?;
        let definition: Definition = plugin
            .macros
            .read_str(body.get_ron())
            .with_context(|| format!("reading subtype {name}"))?;
        anyhow::ensure!(
            matches!(definition, Definition::Subtype { .. }),
            "{name}: a subtype declaration defines a subtype"
        );
        subtypes.push((format!("{category}/{name}"), definition));
    }
    let designations = designation_definitions(&declarations)?
        .into_iter()
        .map(|(declared, definition)| {
            let label = match &definition {
                Definition::Designation { label, .. } => label.name().to_owned(),
                _ => anyhow::bail!("{declared}: a designation declaration defines designations"),
            };
            Ok(if label == declared {
                (label, definition)
            } else {
                (format!("{declared}/{label}"), definition)
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let mut card_types = Vec::new();
    for row in &declarations {
        if row.identity().kind() != DeclarationKind::Type {
            continue;
        }
        let name = row.identity().name();
        let body = row
            .body()
            .with_context(|| format!("{name}: card type has no declaration body"))?;
        let definition: Definition = plugin
            .macros
            .read_str(body.get_ron())
            .with_context(|| format!("reading card type {name}"))?;
        anyhow::ensure!(
            matches!(definition, Definition::CardType { .. }),
            "{name}: a type declaration defines a card type"
        );
        card_types.push((name.to_owned(), definition));
    }
    let relations = relation_definitions(&plugin)?;
    Ok(RegistryDefinitions {
        counters,
        subtypes,
        designations,
        card_types,
        relations,
    })
}

/// The registry definitions of one plugin, by family, each beside the name
/// the gate reports it under.
pub(crate) struct RegistryDefinitions {
    pub(crate) counters: Vec<(String, Definition)>,
    pub(crate) subtypes: Vec<(String, Definition)>,
    pub(crate) designations: Vec<(String, Definition)>,
    pub(crate) card_types: Vec<(String, Definition)>,
    pub(crate) relations: Vec<(String, Definition)>,
}

/// The family kind of a relation declaration (`macros/meta/Relation.ron`).
/// A relation is no construction of the English grammar, so the declaration
/// reader of `deckmaste_construction_core` does not read the family; the
/// plugin loader does.
const RELATION: &str = "Relation";

/// Every relation the relation declarations define, in name order, each
/// under the name it is declared by, which is its label
/// [CR#702.55b,702.95b,702.99b].
fn relation_definitions(plugin: &Plugin) -> anyhow::Result<Vec<(String, Definition)>> {
    let mut result = Vec::new();
    for ((kind, name), declaration) in &plugin.declarations {
        if kind.as_str() != RELATION {
            continue;
        }
        let definition: Definition = plugin
            .macros
            .read_str(declaration.definition.body())
            .with_context(|| format!("reading relation {name}"))?;
        let Definition::Relation { label, .. } = &definition else {
            anyhow::bail!("{name}: a relation declaration defines a relation");
        };
        anyhow::ensure!(
            label.name() == name.as_str(),
            "{name}: a relation's label is the name it is declared under, not `{}`",
            label.name()
        );
        result.push((name.as_str().to_owned(), definition));
    }
    Ok(result)
}

fn relation_rows(root: &Path) -> anyhow::Result<Vec<String>> {
    let plugin_dir = root.join("plugins_v2/builtin");
    let plugin =
        Plugin::load(&plugin_dir).with_context(|| format!("loading {}", plugin_dir.display()))?;
    relation_definitions(&plugin)?
        .into_iter()
        .map(|(name, definition)| {
            // `relationTable` is the table `Check/Words.lean` looks every
            // relation up in, by name.
            let Definition::Relation { label, left, right } = definition else {
                anyhow::bail!("{name}: a relation declaration defines a relation");
            };
            Ok(format!(
                "{{ label := {}, left := {}, right := {} }}",
                quoted(label.name()),
                kind_of(&left)?,
                kind_of(&right)?
            ))
        })
        .collect()
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
                result.insert(overlay_key(keyword.name()));
            }
        }
    }
    Ok(result)
}

/// The overlay key of a keyword ability. The gate-column overlay is shared
/// with the Idris generator, which keys a keyword by its capitalized label
/// (`FirstStrike`); the Lean facts and the grammar key it by the declaration's
/// name (`firstStrike`, `docs/decisions/semantics-v2.md` §7, ruling
/// 2026-10-06), which is that key with its first letter lowered
/// ([`keyword_name`]). The capitalized key stays inside this generator.
pub(super) fn overlay_key(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// The keyword ability declaration name an overlay key names.
fn keyword_name(overlay_key: &str) -> String {
    let mut chars = overlay_key.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// A written definition's components, each with its regime: the one its
/// form determines, or the overlay's `open_regimes` entry where the form
/// leaves it open (none there is a component that functions off the stack).
/// An entry for a component whose form determines its regime is refused.
fn written_components(
    row: &super::Row,
    declared: &KeywordDefinition,
) -> anyhow::Result<Vec<(&'static str, Option<Regime>)>> {
    for (index, _) in row.open_regimes {
        anyhow::ensure!(
            declared.regimes.get(*index) == Some(&FormRegime::Open),
            "{}: component {index} takes its regime from its form, not the overlay",
            row.label
        );
    }
    let mut components = Vec::new();
    for (index, (category, form)) in declared.categories.iter().zip(&declared.regimes).enumerate()
    {
        let Some(category) = category else {
            // The printed ability's own: each category the CR entry admits.
            anyhow::ensure!(
                !row.definition.is_empty(),
                "{}: component {index} qualifies the printed ability, so the overlay names \
                 its categories",
                row.label
            );
            components.extend(
                row.definition
                    .iter()
                    .map(|(category, regime)| (category.lean(), *regime)),
            );
            continue;
        };
        let regime = match form {
            FormRegime::Determined(regime) => *regime,
            FormRegime::Open => row
                .open_regimes
                .iter()
                .find(|(at, _)| *at == index)
                .map(|(_, regime)| *regime),
        };
        components.push((*category, regime));
    }
    Ok(components)
}

/// A declared definition's categories, each printed-ability component read
/// as the categories the overlay names for it (`written_components`).
pub(super) fn resolved_categories(
    row: &super::Row,
    declared: &KeywordDefinition,
) -> Vec<&'static str> {
    declared
        .categories
        .iter()
        .flat_map(|category| match category {
            Some(category) => vec![*category],
            None => row.definition.iter().map(|(category, _)| category.lean()).collect(),
        })
        .collect()
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
                for general in &declared.general_forms {
                    let params = general.iter().map(String::as_str).collect::<Vec<_>>();
                    add_schema(&mut schemas, &params).with_context(|| {
                        format!("{}: invalid general-form argument schema", row.label)
                    })?;
                }
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
                format!("word := .named {}", quoted(&keyword_name(row.label))),
                format!("argumentSchemas := [{}]", schemas.join(", ")),
            ];
            if counter_keywords.contains(row.label) {
                fields.push("counterEligible := true".into());
            }
            for (set, field) in [
                (row.functions_on_stack, "functionsOnStack"),
                (row.on_spell_card, "onInstantOrSorceryCard"),
                (row.paid_cost, "paidCost"),
                (row.cost_chooses_player, "costChoosesPlayer"),
                (row.wants_modes, "wantsModes"),
            ] {
                if set {
                    fields.push(format!("{field} := true"));
                }
            }
            if !row.on_permanent_card {
                fields.push("onPermanentCard := false".into());
            }
            // The components a keyword's definition is written in [CR#702.1]
            // are those of the definition the declaration WRITES, each with
            // the regime its form gives it, or the overlay's where the form
            // leaves it open. Where a declaration writes none — the workbench
            // cannot spell it, and the file records the STOP — the CR entry
            // still names them, and only those rows keep an overlay value.
            let components = match declared.filter(|declared| !declared.categories.is_empty()) {
                Some(declared) => written_components(row, declared)?,
                None => row
                    .definition
                    .iter()
                    .map(|(category, regime)| (category.lean(), *regime))
                    .collect(),
            };
            // A keyword's own regime summarises its components': the first
            // stack regime among them.
            let regime = if components.is_empty() {
                row.regime
            } else {
                let summary = components.iter().find_map(|(_, regime)| *regime);
                anyhow::ensure!(
                    row.regime == summary,
                    "{}: the overlay regime {:?} is not its components' summary {summary:?}",
                    row.label,
                    row.regime
                );
                summary
            };
            if !components.is_empty() {
                let parts = components
                    .iter()
                    .map(|(category, regime)| {
                        format!(
                            "⟨{category}, {}⟩",
                            regime.map_or_else(
                                || "none".to_owned(),
                                |r| format!("some {}", r.lean())
                            )
                        )
                    })
                    .collect::<Vec<_>>();
                fields.push(format!("definition := [{}]", parts.join(", ")));
            }
            if let Some(regime) = regime {
                fields.push(format!("regime := some {}", regime.lean()));
            }
            Ok(format!("{{ {} }}", fields.join(", ")))
        })
        .collect()
}

/// Every deed a keyword-action declaration enacts, in declaration order: the
/// checker's keyword action labels.
///
/// A declaration's deed is the one its own `Enact` wrapper carries: its name
/// verbatim (`collectEvidence`), since a keyword action is named by its
/// declaration name wherever a deed is wanted (`docs/decisions/semantics-v2.md`
/// §7, ruling 2026-10-06), unless the file names another declaration's deed
/// (`conniveN` writes `deed: connive`, one event for both forms
/// [CR#701.50f]), which is then listed once, under that name. A declaration
/// that names no deed (`deed: None`) or has no body is still named by its
/// declaration name where a deed is written (`theVerbed(search, One)`), so it
/// keeps its own label.
///
/// The columns themselves are the checker's own data, hand-written as
/// `actFacts` in `Check/Words.lean`; this list is what `Proofs/Tables.lean`
/// pins that table against in both directions, so a declaration added without
/// a row (or a row left behind by a retired declaration) fails the Lean build.
fn action_labels(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    Ok(keyword_action_deeds(declarations, plugin_dir)?
        .iter()
        .map(|label| quoted(label))
        .collect())
}

/// [`action_labels`] unquoted: every deed a keyword-action declaration
/// enacts, once each, in declaration order.
pub(super) fn keyword_action_deeds(
    declarations: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    let plugin =
        Plugin::load(plugin_dir).with_context(|| format!("loading {}", plugin_dir.display()))?;
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAction)
    {
        let name = declared.identity().name();
        let label = enacted_deed(&plugin, name)?.unwrap_or_else(|| name.to_owned());
        if seen.insert(label.clone()) {
            result.push(label);
        }
    }
    Ok(result)
}

/// A keyword action's built definition, read for the deed alone. The
/// instruction may hold `Param(n)` holes, so it goes past unread here.
#[derive(Deserialize)]
#[expect(
    dead_code,
    reason = "a field is declared so the reader accepts it and skips its value"
)]
enum EnactShape {
    Enact { verb: Deed, instruction: IgnoredAny },
}

/// The label of the deed keyword action `name`'s definition enacts, through
/// `plugin`'s own macros (so a written `deed: connive` resolves to the deed
/// that name denotes); `None` when the definition is not an `Enact` wrapper
/// (`deed: None`, or no body).
fn enacted_deed(plugin: &Plugin, name: &str) -> anyhow::Result<Option<String>> {
    let declaration = plugin
        .declarations
        .get(&(
            macro_ron::Ident::from(keywords::KEYWORD_ACTION),
            macro_ron::Ident::from(name),
        ))
        .with_context(|| format!("{name}: the plugin registers no such keyword action"))?;
    let body = declaration.definition.body();
    if !body.trim_start().starts_with("Enact(") {
        return Ok(None);
    }
    let EnactShape::Enact { verb, .. } = plugin
        .macros
        .read_str(body)
        .with_context(|| format!("reading keyword action {name}'s deed"))?;
    match verb {
        Deed::Action { label } => Ok(Some(label)),
        other => anyhow::bail!("{name}: a keyword action enacts a keyword action deed: {other:?}"),
    }
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
fn subtype_of(macros: &MacroSet, row: &NormalizedDeclaration) -> anyhow::Result<Subtype> {
    if !matches!(row.identity().kind(), DeclarationKind::Subtype(_)) {
        anyhow::bail!("{}: not a subtype declaration", row.identity().name());
    }
    let body = row
        .body()
        .with_context(|| format!("{}: subtype has no declaration body", row.identity().name()))?;
    match macros.read_str::<Definition>(body.get_ron()) {
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
/// written down here to exist. Since `semantics-v2-subtype-rules` no subtype
/// carries an empty `rules` list for want of syntax either: Aura, Equipment,
/// Fortification and Saga write theirs.
const SUBTYPE_DEFINITION_STOPS: &[&str] = &[];

/// Every subtype declaration defines its subtype. Its `rules` may be written
/// over the builtin helpers (`mayBeAttachedTo`, `finalChapterNumber`), so the
/// body is read through the builtin plugin's macros, as a counter's is.
fn subtype_definitions_read(
    dialect: &MacroSet,
    rows: &[NormalizedDeclaration],
) -> anyhow::Result<()> {
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

fn subtype_rows(
    rows: &[NormalizedDeclaration],
    plugin_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    let plugin =
        Plugin::load(plugin_dir).with_context(|| format!("loading {}", plugin_dir.display()))?;
    subtype_definitions_read(&plugin.macros, rows)?;
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
                subtype(&subtype_of(&plugin.macros, row)?)
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
        &action_labels(&declarations, &root.join("plugins_v2/builtin"))?,
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
        "relationTable",
        "RelationFacts",
        &relation_rows(root)?,
    );
    table(
        &mut out,
        "subtypeFacts",
        "SubtypeFacts",
        &subtype_rows(&declarations, &root.join("plugins_v2/builtin"))?,
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
        assert!(table.contains("\"testAction\""), "{table}");
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
            // The body's cost hole is filled with a cost: a declaration whose
            // body no longer fits its signature has no invocation to read. The
            // spelling's `<Param(0)>` hole stays, now an amount.
            source
                .replace("params: [Cost]", "params:\n        [Amount]")
                .replace(", Param(0))", ", tapSymbol)"),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("word := .named \"ward\""))
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
            .find(|line| line.contains("word := .named \"ward\""))
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
            .find(|line| line.contains("word := .named \"ward\""))
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

    /// A spell ability is the half a keyword represents on an instant or
    /// sorcery spell, written under that class's guard [CR#113.3a,702.174b],
    /// so a definition holding one declares the spell category.
    #[test]
    fn a_class_guarded_spell_ability_declares_the_spell_category() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/storm.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source.replace(
                "    ],\n)",
                "        spell(doIf(matches(this, instantOrSorcery), draw(1))),\n    ],\n)",
            ),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("word := .named \"storm\""))
            .unwrap();
        assert!(
            row.contains(
                "definition := [⟨.triggered, some .atCasting⟩, ⟨.spell, some .atResolution⟩]"
            ),
            "{row}"
        );
    }

    /// Each component of a definition carries the regime its own form gives
    /// it: kicker's additional cost at casting and an enters trigger off the
    /// stack, the shape offspring's definition takes [CR#702.175a].
    #[test]
    fn each_component_carries_the_regime_its_form_gives() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/kicker.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source.replace(
                "    ],\n)",
                "        triggered(enters(thisPermanent), draw(1)),\n    ],\n)",
            ),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let row = generated
            .lines()
            .find(|line| line.contains("word := .named \"kicker\""))
            .unwrap();
        assert!(
            row.contains(
                "definition := [⟨.static, some .atCasting⟩, ⟨.triggered, none⟩], \
                 regime := some .atCasting"
            ),
            "{row}"
        );
    }

    /// An overlay regime names only a component whose form leaves it open (a
    /// permission to cast the card); surge's written as an additional cost
    /// takes its regime from its form.
    #[test]
    fn an_open_regime_names_only_an_open_component() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/surge.ron");
        let source = fs::read_to_string(&path).unwrap();
        let start = source.find("    body: [").unwrap();
        fs::write(
            &path,
            format!(
                "{}    body: [\n        static(addedCost(Param(0), true)),\n    ],\n)\n",
                &source[..start]
            ),
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("Surge: component 0 takes its regime from its form"),
            "{error}"
        );
    }

    /// A keyword's own regime is its components' summary: prowess written with
    /// an enters trigger has no stack regime, so its overlay's at-casting
    /// regime is refused.
    #[test]
    fn the_keyword_regime_is_its_components_summary() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/keyword_abilities/prowess.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            source.replace(
                "casts(you, a(and([inZone(stack), not(creature)])))",
                "enters(thisPermanent)",
            ),
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains(
                "Prowess: the overlay regime Some(AtCasting) is not its components' summary None"
            ),
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
                "grants(keyword: shadow)",
                "grants(shadow)",
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
                "{ word := .named \"shadow\", argumentSchemas := [[]], counterEligible := true,"
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
