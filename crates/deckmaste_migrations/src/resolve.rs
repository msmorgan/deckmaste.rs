//! `resolve` — the ability-by-ability rewriter. For each `cards/*.ron.todo`,
//! every `TodoAbility::Unparsed(line)` is run through an ordered registry of
//! ability parsers; the first that structures the line becomes a
//! `TodoAbility::Parsed(<bare ability RON>)`. Pure per-card map; idempotent.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::template::index::TemplateIndex;
use deckmaste_core::plugin::is_ron_todo_file;

use crate::todo_card::RawIdent;
use crate::todo_card::TodoAbility;
use crate::todo_card::TodoCard;
use crate::todo_card::TodoCardFace;
use crate::todo_card::render;

/// The coarse card category a parser needs to decide framing: a `Spell` card
/// is an instant or sorcery (its effect text is a `Spell` ability); everything
/// else is a `Permanent` (effect text lives inside triggered/activated/static
/// frames). Computed once per face and handed to every parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardKind {
    Spell,
    Permanent,
}

impl CardKind {
    /// Classifies a face by its raw type line.
    #[must_use]
    pub fn of(types: &[RawIdent]) -> Self {
        if types
            .iter()
            .any(|t| matches!(t.0.as_str(), "Instant" | "Sorcery"))
        {
            CardKind::Spell
        } else {
            CardKind::Permanent
        }
    }
}

/// What a parser resolves a line *in*: the card category plus the reverse
/// [`TemplateIndex`], so a parser can match a line back to the macro whose
/// template renders it (`English → macro`). Built once per resolve run and
/// handed to every parser.
pub struct ResolveCtx<'a> {
    pub kind: CardKind,
    pub index: &'a TemplateIndex,
}

/// One ability parser: a normalized oracle line -> the bare RON of one ability
/// (`Flying`, `Activated(cost: [Tap], effect: AddMana(1, Green))`), or `None`
/// to decline.
pub type AbilityParser = fn(&str, &ResolveCtx) -> anyhow::Result<Option<String>>;

/// The registry, in priority order. First match wins. The macro-template parser
/// leads: it routes a line back through the registered macros' templates before
/// the bespoke parsers re-encode the same mechanics by hand.
pub const REGISTRY: &[AbilityParser] = &[
    crate::parsers::macro_template::resolve_line,
    crate::parsers::mana_ability::resolve_line,
    crate::parsers::keyword_ability::resolve_line,
    crate::parsers::spell_ability::resolve_line,
    crate::parsers::triggered_ability::resolve_line,
    crate::parsers::activated_ability::resolve_line,
    crate::parsers::replacement::resolve_line,
    crate::parsers::static_ability::resolve_line,
];

/// The Ascend gate ([CR#702.131a,702.131b]) — KEEP IN SYNC with
/// `plugins/builtin/macros/keyword/Ascend.ron`.
const ASCEND_GATE: &str = "AllOf([Compare(CountOf(AllOf([InZone(Battlefield), \
ControlledBy(Ref(You))])), AtLeast, Literal(10)), Not(Is(You, Designated(\"CitysBlessing\")))])";

/// [CR#702.131a]: fold an Ascend keyword on a SPELL into the front of its spell
/// effect, then drop the keyword. `effect` is the last field of the rendered
/// `Spell(...)` ability, so its value runs from `effect: ` to the closing `)`.
fn fold_spell_ascend(face: &mut TodoCardFace) -> bool {
    let has_ascend = face
        .abilities
        .iter()
        .any(|a| matches!(a, TodoAbility::Parsed(s) if s == "Keyword(Ascend)"));
    if !has_ascend {
        return false;
    }
    let mut wrapped = false;
    for ability in &mut face.abilities {
        if let TodoAbility::Parsed(s) = ability
            && let Some(idx) = s.find("effect: ")
            && s.starts_with("Spell(")
            && s.ends_with(')')
        {
            let head = &s[..idx + "effect: ".len()];
            let effect_val = &s[idx + "effect: ".len()..s.len() - 1];
            *ability = TodoAbility::Parsed(format!(
                "{head}Sequence([If(condition: {ASCEND_GATE}, then: GetDesignation(\"CitysBlessing\")), {effect_val}]))"
            ));
            wrapped = true;
            break;
        }
    }
    if wrapped {
        face.abilities
            .retain(|a| !matches!(a, TodoAbility::Parsed(s) if s == "Keyword(Ascend)"));
    }
    wrapped
}

/// Replaces every `Unparsed` line a parser in `registry` can structure with the
/// `Parsed` RON. Returns whether anything changed.
fn resolve_face(
    face: &mut TodoCardFace,
    registry: &[AbilityParser],
    index: &TemplateIndex,
) -> anyhow::Result<bool> {
    let ctx = ResolveCtx {
        kind: CardKind::of(&face.types),
        index,
    };
    let mut changed = false;
    for ability in &mut face.abilities {
        let TodoAbility::Unparsed(line) = ability else {
            continue;
        };
        for parser in registry {
            if let Some(ron) = parser(line, &ctx)? {
                *ability = TodoAbility::Parsed(ron);
                changed = true;
                break;
            }
        }
    }
    if ctx.kind == CardKind::Spell && fold_spell_ascend(face) {
        changed = true;
    }
    Ok(changed)
}

/// Resolves a whole card against the default [`REGISTRY`]. Returns whether
/// anything changed (so callers can skip rewriting unchanged files).
///
/// # Errors
/// If any parser in the registry returns an error.
pub fn resolve_card(card: &mut TodoCard, index: &TemplateIndex) -> anyhow::Result<bool> {
    resolve_card_with(card, REGISTRY, index)
}

/// `resolve_card` against a given registry (test seam).
///
/// # Errors
/// If any parser in `registry` returns an error.
pub fn resolve_card_with(
    card: &mut TodoCard,
    registry: &[AbilityParser],
    index: &TemplateIndex,
) -> anyhow::Result<bool> {
    let changed = match card {
        TodoCard::Normal(face) => resolve_face(face, registry, index)?,
        TodoCard::ModalDfc(front, back) => {
            let a = resolve_face(front, registry, index)?;
            let b = resolve_face(back, registry, index)?;
            a || b
        }
    };
    Ok(changed)
}

/// Resolves every `cards/*.ron.todo` in `plugin_dir` in place.
///
/// # Errors
/// If a file isn't readable/parsable as a `TodoCard`, or isn't writable.
pub fn resolve_cards(plugin_dir: &Path) -> anyhow::Result<()> {
    // Load the plugin's macros (builtin prelude included) and build the reverse
    // template index ONCE, before the per-card loop — the macro-template parser
    // matches lines back to these macros.
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let index = TemplateIndex::build(&plugin.macros);
    let cards = crate::layout::PluginLayout::new(plugin_dir)?.cards_dir()?;
    let mut paths: Vec<_> = std::fs::read_dir(&cards)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()?;
    paths.sort();
    for path in paths {
        if !path.is_file() || !is_ron_todo_file(&path) {
            continue;
        }
        // A malformed `.ron.todo` aborts the run (via `?`): it means a bug in
        // the step that wrote it, which the engineer should fix before resolving.
        let source = std::fs::read_to_string(&path)?;
        let mut card: TodoCard = crate::ron_output::ron_options().from_str(&source)?;
        if resolve_card(&mut card, &index)? {
            std::fs::write(&path, render(&card)?)?;
            eprintln!("resolved {}", path.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A registry that structures the line "Flying" only.
    #[allow(clippy::unnecessary_wraps)]
    fn flying_only(line: &str, _ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
        Ok((line == "Flying").then(|| "Flying".to_owned()))
    }

    /// An empty reverse index — for tests that exercise the bespoke parsers,
    /// the macro-template parser declines on every line.
    fn no_index() -> TemplateIndex {
        TemplateIndex::default()
    }

    #[test]
    fn resolve_replaces_known_lines_only() {
        use deckmaste_core::ManaCost;
        let mut card = TodoCard::Normal(TodoCardFace {
            name: "X".into(),
            mana_cost: ManaCost::default(),
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![RawIdent("Creature".into())],
            subtypes: vec![],
            abilities: vec![
                TodoAbility::Unparsed("Flying".into()),
                TodoAbility::Unparsed("When ~ dies, draw a card.".into()),
            ],
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        });
        let changed = resolve_card_with(&mut card, &[flying_only], &no_index()).unwrap();
        assert!(changed);
        let TodoCard::Normal(face) = &card else { panic!() };
        assert!(matches!(&face.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        assert!(matches!(&face.abilities[1], TodoAbility::Unparsed(_))); // unchanged
        // Idempotent: a second pass changes nothing.
        assert!(!resolve_card_with(&mut card, &[flying_only], &no_index()).unwrap());
    }

    #[test]
    fn card_kind_classifies_by_type() {
        let ident = |s: &str| RawIdent(s.to_owned());
        assert_eq!(CardKind::of(&[ident("Instant")]), CardKind::Spell);
        assert_eq!(CardKind::of(&[ident("Sorcery")]), CardKind::Spell);
        assert_eq!(CardKind::of(&[ident("Creature")]), CardKind::Permanent);
        // Multi-type artifact-creature is a permanent; no Instant/Sorcery.
        assert_eq!(
            CardKind::of(&[ident("Artifact"), ident("Creature")]),
            CardKind::Permanent
        );
        // No type line at all defaults to permanent framing.
        assert_eq!(CardKind::of(&[]), CardKind::Permanent);
    }

    #[test]
    fn spell_card_resolves_damage_but_permanent_does_not() {
        // An instant with Lightning Bolt's line resolves to a Spell ability.
        let mut bolt = TodoCard::Normal(TodoCardFace {
            name: "Bolt".into(),
            types: vec![RawIdent("Instant".into())],
            abilities: vec![TodoAbility::Unparsed(
                "~ deals 3 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut bolt, &no_index()).unwrap());
        let TodoCard::Normal(face) = &bolt else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Spell(targets: [AnyTarget], effect: DealDamage(Target(0), 3))"
        ));

        // The same line on a creature is NOT a spell ability: it stays Unparsed.
        let mut creature = TodoCard::Normal(TodoCardFace {
            name: "X".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "~ deals 3 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(!resolve_card(&mut creature, &no_index()).unwrap());
        let TodoCard::Normal(face) = &creature else { panic!() };
        assert!(matches!(&face.abilities[0], TodoAbility::Unparsed(_)));

        // A Sorcery resolves an untargeted effect to a `Spell` with no targets field.
        let mut divination = TodoCard::Normal(TodoCardFace {
            name: "Divination".into(),
            types: vec![RawIdent("Sorcery".into())],
            abilities: vec![TodoAbility::Unparsed("Draw two cards.".into())],
            ..Default::default()
        });
        assert!(resolve_card(&mut divination, &no_index()).unwrap());
        let TodoCard::Normal(face) = &divination else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r) if r == "Spell(effect: Draw(2))"
        ));
    }

    #[test]
    fn creature_trigger_resolves_through_registry() {
        let mut card = TodoCard::Normal(TodoCardFace {
            name: "Arsonist".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "When ~ dies, it deals 1 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut card, &no_index()).unwrap());
        let TodoCard::Normal(face) = &card else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Triggered(event: ThisDies, targets: [AnyTarget], effect: DealDamage(Target(0), 1))"
        ));
    }

    #[test]
    fn creature_activated_resolves_through_registry() {
        let mut card = TodoCard::Normal(TodoCardFace {
            name: "Cellar Rat".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "{1}{B}, Sacrifice ~: Draw a card.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut card, &no_index()).unwrap());
        let TodoCard::Normal(face) = &card else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Activated(cost: [Mana([Generic(1),Black]), SacrificeThis], effect: Draw(1))"
        ));
    }

    #[test]
    fn permanent_static_anthem_resolves() {
        let mut lord = TodoCard::Normal(TodoCardFace {
            name: "Test Lord".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "Creatures you control get +1/+1.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut lord, &no_index()).unwrap());
        let TodoCard::Normal(face) = &lord else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])])"
        ));
    }

    #[test]
    fn resolve_modal_dfc_resolves_both_faces() {
        let mut card = TodoCard::ModalDfc(
            TodoCardFace {
                abilities: vec![TodoAbility::Unparsed("Flying".into())],
                ..Default::default()
            },
            TodoCardFace {
                abilities: vec![TodoAbility::Unparsed("Flying".into())],
                ..Default::default()
            },
        );
        assert!(resolve_card_with(&mut card, &[flying_only], &no_index()).unwrap());
        let TodoCard::ModalDfc(front, back) = &card else { panic!() };
        assert!(matches!(&front.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        assert!(matches!(&back.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        // Idempotent.
        assert!(!resolve_card_with(&mut card, &[flying_only], &no_index()).unwrap());
    }

    /// [CR#702.131a]: on a spell, Ascend is folded into the front of the spell
    /// effect (a Sequence) and the keyword ability is dropped — the grant
    /// resolves before any downstream "if you have the city's blessing"
    /// read. On a permanent, the keyword is left as-is (Task 5's static-Sba
    /// macro handles it).
    #[test]
    fn ascend_on_spell_folds_into_spell_effect() {
        // A Sorcery with Ascend + a draw effect, both already line-resolved.
        // `<E>` = `Draw(Literal(1))` — a CORE-PARSEABLE effect: this is route
        // (a) from the task. The migrations spell parser may emit macro-flavored
        // atoms (e.g. `Draw(2)`), but the round-trip assertion below uses the
        // BARE `deckmaste_core::ron::options()` reader — the same reader
        // `resolve_cards`/`graduate` round-trips card RON through — so the
        // fixture must be one the bare core reader accepts. `Draw(Literal(1))`
        // is exactly such a form (cf. the `Activated(... Draw(Literal(1)))`
        // round-trip test in `deckmaste_core::ability`).
        let mut face = TodoCardFace {
            name: "Test Spell".into(),
            types: vec![RawIdent("Sorcery".into())],
            abilities: vec![
                TodoAbility::Parsed("Keyword(Ascend)".into()),
                TodoAbility::Parsed("Spell(effect: Draw(Literal(1)))".into()),
            ],
            ..Default::default()
        };
        let changed = resolve_face(&mut face, &[], &no_index()).unwrap();
        assert!(changed);

        // The Ascend keyword is gone …
        assert!(
            !face.abilities.iter().any(|a| matches!(
                a,
                TodoAbility::Parsed(s) if s.contains("Keyword(Ascend)")
            )),
            "Ascend keyword stripped on a spell"
        );
        // … and the single Spell ability now wraps the grant first.
        let spell = face
            .abilities
            .iter()
            .find_map(|a| match a {
                TodoAbility::Parsed(s) if s.starts_with("Spell(") => Some(s.clone()),
                _ => None,
            })
            .expect("a Spell ability remains");
        assert!(
            spell.contains("Sequence(["),
            "effect wrapped in a Sequence: {spell}"
        );
        assert!(
            spell.contains("GetDesignation(\"CitysBlessing\")"),
            "grant present: {spell}"
        );
        // The original effect value is preserved verbatim inside the wrap.
        assert!(
            spell.contains("Draw(Literal(1))"),
            "original effect preserved: {spell}"
        );
        // The wrapped Spell string re-parses into a typed Ability (no garbage).
        let _: deckmaste_core::Ability = deckmaste_core::ron::options()
            .from_str(&spell)
            .expect("wrapped Spell re-parses");
    }

    /// On a permanent, Ascend is untouched by the post-pass.
    #[test]
    fn ascend_on_permanent_left_as_keyword() {
        let mut face = TodoCardFace {
            name: "Test Permanent".into(),
            types: vec![RawIdent("Enchantment".into())],
            abilities: vec![TodoAbility::Parsed("Keyword(Ascend)".into())],
            ..Default::default()
        };
        let _ = resolve_face(&mut face, &[], &no_index()).unwrap();
        assert!(
            face.abilities.iter().any(|a| matches!(
                a,
                TodoAbility::Parsed(s) if s == "Keyword(Ascend)"
            )),
            "Ascend keyword preserved on a permanent"
        );
    }

    /// Drift guard: the spell-form `ASCEND_GATE` string must parse to the same
    /// `Condition` the permanent-form macro and the engine use
    /// ([CR#702.131a,702.131b]). The canonical gate is "you control ten or more
    /// battlefield permanents AND you don't already have the city's blessing".
    /// If `ASCEND_GATE` is edited away from this shape, this fails loudly.
    #[test]
    fn ascend_gate_const_matches_canonical_condition() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Filter;
        use deckmaste_core::Reference;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;
        use deckmaste_core::Zone;

        let parsed: Condition = crate::ron_output::ron_options()
            .from_str(ASCEND_GATE)
            .expect("ASCEND_GATE parses as a Condition");

        let canonical = Condition::AllOf(vec![
            Condition::Compare(
                Count::CountOf(Box::new(Filter::AllOf(vec![
                    Filter::State(StateFilter::InZone(Zone::Battlefield)),
                    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                        Reference::You,
                    )))),
                ]))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Is(
                Reference::You,
                Filter::State(StateFilter::Designated("CitysBlessing".into())),
            ))),
        ]);

        assert_eq!(
            parsed, canonical,
            "ASCEND_GATE drifted from the canonical Ascend gate"
        );
    }
}
