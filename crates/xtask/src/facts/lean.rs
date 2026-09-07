//! Lean registry tables. The overlay supplies only columns absent from
//! declarations.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

use anyhow::Context;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::{self};
use deckmaste_semantics::CoreDeed;
use deckmaste_semantics::DesignationConferrer;
use deckmaste_semantics::DesignationDecl;
use deckmaste_semantics::DesignationDef;
use deckmaste_semantics::DesignationScope;
use deckmaste_semantics::DesignationShape;
use ron::value::RawValue;
use serde::Deserialize;

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

fn surface(row: &NormalizedDeclaration) -> anyhow::Result<String> {
    row.spelling()
        .iter()
        .map(|part| match part {
            SpellingPart::Literal(text) => Ok(text.as_str()),
            SpellingPart::Param(_) => anyhow::bail!(
                "{}: a registry label cannot contain a parameter",
                row.identity().name()
            ),
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .map(|parts| parts.join(""))
}

fn string_list(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| quoted(value))
            .collect::<Vec<_>>()
            .join(", ")
    )
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

fn core_deed(deed: CoreDeed) -> &'static str {
    match deed {
        CoreDeed::Attack => ".attack",
        CoreDeed::Block => ".block",
        CoreDeed::Target => ".target",
        CoreDeed::Copy => ".copy",
        CoreDeed::Draw => ".draw",
        CoreDeed::GainLife => ".gainLife",
        CoreDeed::LoseGame => ".loseGame",
        CoreDeed::WinGame => ".winGame",
        CoreDeed::Spend => ".spend",
        CoreDeed::Trigger => ".trigger",
        CoreDeed::Put => ".put",
        CoreDeed::Return => ".return_",
        CoreDeed::GainControl => ".gainControl",
        CoreDeed::Unlock => ".unlock",
        CoreDeed::FullyUnlock => ".fullyUnlock",
    }
}

struct DesignationRow {
    labels: Vec<String>,
    decl: DesignationDecl,
}

fn designations(rows: &[NormalizedDeclaration]) -> anyhow::Result<Vec<DesignationRow>> {
    rows.iter()
        .filter(|row| row.identity().kind() == DeclarationKind::Designation)
        .map(|row| {
            let body = row.body().with_context(|| {
                format!(
                    "{}: designation has no declaration body",
                    row.identity().name()
                )
            })?;
            let decl: DesignationDecl = deckmaste_semantics::ron::options()
                .from_str(body.get_ron())
                .with_context(|| format!("reading designation {}", row.identity().name()))?;
            let labels = match &decl.definition {
                DesignationDef::Stored {
                    shape: DesignationShape::Enum(values),
                    scope,
                    ..
                } => {
                    // The enum's declared values name its members. Object-held
                    // members qualify the family name;
                    // game-held members stand alone.
                    let family = row.identity().name().to_lowercase();
                    values
                        .iter()
                        .map(|value| {
                            if *scope == DesignationScope::Game {
                                value.as_str().to_lowercase()
                            } else {
                                format!("{} {family}", value.as_str().to_lowercase())
                            }
                        })
                        .collect()
                }
                _ => vec![surface(row)?],
            };
            Ok(DesignationRow { labels, decl })
        })
        .collect()
}

fn conferrals(rows: &[DesignationRow], wanted: &DesignationConferrer) -> Vec<String> {
    rows.iter()
        .filter(|row| row.decl.conferrers.contains(wanted))
        .flat_map(|row| row.labels.iter().cloned())
        .collect()
}

fn table(out: &mut String, name: &str, ty: &str, rows: &[String]) {
    writeln!(
        out,
        "def {name} : List {ty} :=\n  [ {} ]\n",
        rows.join(",\n    ")
    )
    .unwrap();
}

fn keyword_rows(
    declarations: &[NormalizedDeclaration],
    designations: &[DesignationRow],
) -> anyhow::Result<Vec<String>> {
    let overlays = overlay();
    for declared in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::KeywordAbility)
    {
        let label = declared.identity().name();
        anyhow::ensure!(
            overlays.iter().any(|row| row.label == label)
                || KEYWORD_STUBS_EXEMPT
                    .iter()
                    .any(|exempt| exempt.label == label),
            "{label}: keyword declaration has no gate-column overlay"
        );
    }
    overlays
        .iter()
        .map(|row| {
            let declared = declarations.iter().find(|decl| {
                decl.identity().kind() == DeclarationKind::KeywordAbility
                    && decl.identity().name() == row.label
            });
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
                    .params()
                    .unwrap_or_default()
                    .iter()
                    .map(macro_def::ParameterType::as_str)
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
            for (set, field) in [
                (row.counter_eligible, "counterEligible"),
                (row.functions_on_stack, "functionsOnStack"),
                (row.on_spell_card, "onInstantOrSorceryCard"),
                (row.paid_cost, "paidCost"),
                (row.bodied, "bodied"),
                (row.wants_modes, "wantsModes"),
            ] {
                if set {
                    fields.push(format!("{field} := true"));
                }
            }
            if !row.on_permanent_card {
                fields.push("onPermanentCard := false".into());
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
            let conferred = conferrals(
                designations,
                &DesignationConferrer::KeywordAbility(row.label.into()),
            );
            if !conferred.is_empty() {
                fields.push(format!("confers := {}", string_list(&conferred)));
            }
            Ok(format!("{{ {} }}", fields.join(", ")))
        })
        .collect()
}

fn action_rows(
    declarations: &[NormalizedDeclaration],
    designations: &[DesignationRow],
) -> anyhow::Result<Vec<String>> {
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
        let conferred = conferrals(
            designations,
            &DesignationConferrer::KeywordAction(name.into()),
        );
        let fields = fields
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim();
        let fields = if conferred.is_empty() {
            fields.to_owned()
        } else {
            format!(
                "{fields}{}confers := {}",
                if fields.is_empty() { "" } else { ", " },
                string_list(&conferred)
            )
        };
        result.push(format!("({}, {{ {fields} }})", quoted(label)));
    }
    for &(label, _) in ACTION_OVERLAY {
        anyhow::ensure!(
            seen.contains(label),
            "{label}: action overlay has no registry declaration"
        );
    }
    Ok(result)
}

#[derive(Deserialize)]
#[serde(rename = "Counter")]
struct CounterRow {
    #[serde(default)]
    scope: deckmaste_semantics::CounterScope,
    #[serde(default)]
    confers: Vec<Box<RawValue>>,
}

// Only the bearing's structure is needed to distinguish the dedicated P/T and
// keyword counter forms. The expressions inside those bearings remain opaque to
// this classifier.
#[derive(Deserialize)]
enum Bearing {
    Continuous(serde::de::IgnoredAny, CounterModification),
    #[serde(other)]
    Other,
}
#[derive(Deserialize)]
enum CounterModification {
    Power(serde::de::IgnoredAny),
    Toughness(serde::de::IgnoredAny),
    Several(Vec<CounterModification>),
    GainAbility(CounterAbility),
    #[serde(other)]
    Other,
}
#[derive(Deserialize)]
enum CounterAbility {
    Keyword(serde::de::IgnoredAny),
    #[serde(other)]
    Other,
}
impl CounterModification {
    fn dedicated(&self) -> bool {
        match self {
            Self::Power(_) | Self::Toughness(_) | Self::GainAbility(CounterAbility::Keyword(_)) => {
                true
            }
            Self::Several(parts) => !parts.is_empty() && parts.iter().all(Self::dedicated),
            _ => false,
        }
    }
}

fn counter_rows(declarations: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    for row in declarations
        .iter()
        .filter(|row| row.identity().kind() == DeclarationKind::CounterKind)
    {
        let body = row.body().with_context(|| {
            format!("{}: counter has no declaration body", row.identity().name())
        })?;
        let counter: CounterRow = deckmaste_semantics::ron::options().from_str(body.get_ron())?;
        let dedicated = counter.confers.iter().any(|bearing| {
            matches!(deckmaste_semantics::ron::options().from_str::<Bearing>(bearing.get_ron()), Ok(Bearing::Continuous(_, modification)) if modification.dedicated())
        });
        if dedicated {
            continue;
        }
        let spelling = surface(row)?;
        let mut chars = spelling.chars();
        let label = chars
            .next()
            .with_context(|| "empty counter spelling")?
            .to_uppercase()
            .collect::<String>()
            + chars.as_str();
        let holder = match counter.scope {
            deckmaste_semantics::CounterScope::Object => ".object",
            deckmaste_semantics::CounterScope::Player => ".player",
        };
        result.push(format!("⟨{}, {holder}⟩", quoted(&label)));
    }
    Ok(result)
}

// Effectfulness, card-lifetime scope, and Room-half columns are absent from
// Stored.
const DESIGNATION_OVERLAY: &[(&str, bool, bool, Option<&str>)] = &[
    ("Commander", false, true, None),
    ("LeftHalfUnlocked", true, false, Some(".left")),
    ("RightHalfUnlocked", true, false, Some(".right")),
    ("Monarch", true, false, None),
    ("Initiative", true, false, None),
    ("CitysBlessing", true, false, None),
    ("EnduringStory", true, false, None),
    ("Goaded", true, false, None),
    ("RingBearer", true, false, None),
    ("Monstrous", true, false, None),
    ("Renowned", true, false, None),
    ("Suspected", true, false, None),
    ("Prepared", true, false, None),
    ("Sector", true, false, None),
    ("Saddled", true, false, None),
    ("Harnessed", true, false, None),
    ("Level", true, false, None),
    ("Solved", true, false, None),
    ("DayNight", true, false, None),
];

fn designation_rows(rows: &[DesignationRow]) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    for row in rows {
        let name = row.decl.name.as_str();
        // These checker columns are absent from the registry's stored-state
        // schema.
        let &(_, effectful, card, half) = DESIGNATION_OVERLAY
            .iter()
            .find(|(label, _, _, _)| *label == name)
            .with_context(|| format!("{name}: designation has no checker-column overlay"))?;
        let DesignationDef::Stored { scope, .. } = &row.decl.definition else {
            anyhow::bail!("{name}: derived designation has no checker-column overlay");
        };
        let (scope, zone) = match scope {
            DesignationScope::Player => (".heldBy .player", "none"),
            DesignationScope::Game => (".heldByGame", "none"),
            DesignationScope::Object if card => (".heldByCard", "none"),
            DesignationScope::Object => (".heldBy .object", "some .battlefield"),
        };
        for label in &row.labels {
            result.push(format!("{{ label := {}, scope := {scope}, effectful := {effectful}, zone := {zone}, type := none, half := {} }}", quoted(label), half.map_or("none".into(), |half| format!("some {half}"))));
        }
    }
    Ok(result)
}

fn subtype_rows(rows: &[NormalizedDeclaration]) -> anyhow::Result<Vec<String>> {
    // Frame columns are not yet present in the subtype declarations.
    let overlays = [
        ("Saga", SubtypeCategory::Enchantment, ".chapters"),
        ("Adventure", SubtypeCategory::Spell, ".adventureInset"),
        ("Room", SubtypeCategory::Enchantment, ".doors"),
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
            let subtype = match category {
                SubtypeCategory::Spell => format!(".spell {}", quoted(&surface(row)?)),
                _ => format!(".of .{category} {}", quoted(&surface(row)?)),
            };
            Ok(format!("⟨{subtype}, {frame}⟩"))
        })
        .collect()
}

pub(super) fn render(root: &Path) -> anyhow::Result<String> {
    let declarations = macro_def::read_builtin_v2(root.join("plugins_v2/builtin"))?;
    let designations = designations(&declarations)?;
    let identities: BTreeSet<_> = declarations
        .iter()
        .map(|row| (row.identity().kind(), row.identity().name()))
        .collect();
    let mut core_conferrals: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for row in &designations {
        for conferrer in row.decl.conferrers.iter() {
            let key = match conferrer {
                DesignationConferrer::KeywordAbility(name) => {
                    (DeclarationKind::KeywordAbility, name.as_str())
                }
                DesignationConferrer::KeywordAction(name) => {
                    (DeclarationKind::KeywordAction, name.as_str())
                }
                DesignationConferrer::CoreDeed(deed) => {
                    core_conferrals
                        .entry(core_deed(*deed))
                        .or_default()
                        .extend(row.labels.iter().cloned());
                    continue;
                }
            };
            anyhow::ensure!(
                identities.contains(&key),
                "{}: undeclared designation conferrer {}",
                row.decl.name,
                key.1
            );
        }
    }
    let mut out = String::from(
        "-- Generated by `cargo xtask facts generate`; edit registry declarations and xtask overlays.\nimport Semantics.Check.FactTypes\n\nnamespace Semantics\n\n",
    );
    table(
        &mut out,
        "actFacts",
        "(KeywordActionLabel × ActFacts)",
        &action_rows(&declarations, &designations)?,
    );
    table(
        &mut out,
        "keywordFacts",
        "KeywordFacts",
        &keyword_rows(&declarations, &designations)?,
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
        &designation_rows(&designations)?,
    );
    table(
        &mut out,
        "subtypeFacts",
        "SubtypeFacts",
        &subtype_rows(&declarations)?,
    );
    out.push_str("def coreDeedConferrals : CoreDeed → List DesignationLabel\n");
    for (deed, labels) in core_conferrals {
        writeln!(out, "  | {deed} => {}", string_list(&labels)).unwrap();
    }
    out.push_str("  | _ => []\n\nend Semantics\n");
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
        let source = root.join("plugins_v2/builtin/macros/stubs");
        let destination = temp.path().join("plugins_v2/builtin/macros/stubs");
        copy_tree(&source, &destination);
        temp
    }

    #[test]
    fn declaration_conferrer_changes_reach_the_generated_keyword_row() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/stubs/designations/EnduringStory.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            path,
            source.replace("KeywordAbility(\"Storied\")", "KeywordAbility(\"Renown\")"),
        )
        .unwrap();
        let generated = render(temp.path()).unwrap();
        let renown = generated
            .lines()
            .find(|line| line.contains("word := \"Renown\""))
            .unwrap();
        assert!(
            renown.contains("confers := [\"an enduring story\", \"renowned\"]"),
            "{renown}"
        );
        let storied = generated
            .lines()
            .find(|line| line.contains("word := \"Storied\""))
            .unwrap();
        assert!(!storied.contains("confers :="), "{storied}");
    }

    #[test]
    fn new_action_requires_a_checker_overlay() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/stubs/keyword_actions/TestAction.ron");
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
            .join("plugins_v2/builtin/macros/stubs/keyword_abilities/Ward.ron");
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
            .join("plugins_v2/builtin/macros/stubs/keyword_abilities/Ward.ron");
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
                .join("plugins_v2/builtin/macros/stubs/keyword_abilities/TestKeyword.ron"),
            "KeywordAbility(name: \"TestKeyword\", spelling: \"test keyword\")",
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("TestKeyword: keyword declaration has no gate-column overlay"),
            "{error}"
        );
    }

    #[test]
    fn undeclared_conferrers_are_rejected() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/stubs/designations/EnduringStory.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            path,
            source.replace(
                "KeywordAbility(\"Storied\")",
                "KeywordAbility(\"Undeclared\")",
            ),
        )
        .unwrap();
        let error = render(temp.path()).unwrap_err().to_string();
        assert!(
            error.contains("undeclared designation conferrer Undeclared"),
            "{error}"
        );
    }

    #[test]
    fn counter_scope_is_read_from_the_declaration() {
        let temp = fixture();
        let path = temp
            .path()
            .join("plugins_v2/builtin/macros/stubs/counter_kinds/Poison.ron");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(path, source.replace("scope: Player", "scope: Object")).unwrap();
        let generated = render(temp.path()).unwrap();
        assert!(generated.contains("⟨\"Poison\", .object⟩"));
        assert!(!generated.contains("⟨\"Poison\", .player⟩"));
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
