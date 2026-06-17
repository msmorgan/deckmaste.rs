//! Force-expansion proof for every builtin keyword macro: bodies parse
//! LAZILY, so a load-passing plugin proves only the declaration shells —
//! this test invokes each macro (sample args for the parameterized ones)
//! and requires the body to deserialize as a real `KeywordAbility`.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::KeywordAbility;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

#[test]
fn every_builtin_keyword_macro_expands() {
    // (invocation, expected carried name) — args chosen to satisfy each
    // body's typed positions ([Tap] where a cost-component list lands,
    // a filter where a quality lands; unused params take anything).
    let cases = [
        ("Flying", "Flying"),
        ("Lifelink", "Lifelink"),
        ("Reach", "Reach"),
        ("Flash", "Flash"),
        ("Defender", "Defender"),
        ("Menace", "Menace"),
        ("Haste", "Haste"),
        ("Indestructible", "Indestructible"),
        ("Changeling", "Changeling"),
        ("Prowess", "Prowess"),
        ("Hexproof()", "Hexproof"),
        ("Hexproof(from: ColorIs(Blue))", "Hexproof"),
        ("Ward([Tap])", "Ward"),
        ("Kicker([Tap])", "Kicker"),
        ("Flashback([Tap])", "Flashback"),
        ("Equip([Tap])", "Equip"),
        ("Fortify([Tap])", "Fortify"),
        ("Reconfigure([Tap])", "Reconfigure"),
        ("Enchant(Type(Creature))", "Enchant"),
        ("Protection(ColorIs(Red))", "Protection"),
        ("Crew(2)", "Crew"),
        ("Affinity(Type(Artifact))", "Affinity"),
        ("Cycling([Mana([Generic(2)])])", "Cycling"),
    ];
    let plugin = builtin();
    for (invocation, name) in cases {
        let kw: KeywordAbility = plugin
            .macros
            .read_str(invocation)
            .unwrap_or_else(|e| panic!("expanding {invocation}: {e}"));
        let KeywordAbility::Expanded(expanded) = &kw else {
            panic!("expected Expanded for {invocation}, got {kw:?}");
        };
        assert_eq!(
            expanded.name.as_str(),
            name,
            "carried name for {invocation}"
        );
        assert!(
            matches!(&*expanded.value, KeywordAbility::Composite { name: n, .. } if n.as_str() == name),
            "body of {invocation} is a name-carrying Composite"
        );
    }
}

/// [CR#702.5a,303.4a,303.4f]: the **Enchant** keyword confers THREE abilities,
/// not just the legal-host restriction: (1) a targeting `Spell` (target spec
/// only, no-op effect) so cast targeting stays on the live `spell_targets`
/// path; (2) the removable `Cant(Attach(what: Ref(This), to: Not(Param(0))))`
/// host bound; (3) `AsEnters(Attach(Ref(This), Param(0)))` so the Aura enters
/// attached. (The conferral map in the spec.)
#[test]
fn enchant_confers_spell_cant_attach_and_as_enters() {
    use deckmaste_core::Ability;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::StaticEffect;

    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.extend(s.effects.iter().cloned()),
            Ability::Expanded(e) => statics(&e.value, out),
            _ => {}
        }
    }
    fn peel(e: &StaticEffect) -> &StaticEffect {
        match e {
            StaticEffect::Expanded(x) => peel(&x.value),
            other => other,
        }
    }

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Enchant(Type(Creature))")
        .expect("Enchant expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Enchant body is a Composite");
    };

    // (1) a targeting Spell ability.
    assert!(
        abilities
            .iter()
            .any(|a| matches!(a, Ability::Spell(s)
                if matches!(&s.effect, deckmaste_core::Effect::Targeted(t) if !t.targets.is_empty()))),
        "Enchant confers a targeting Spell ability ([CR#303.4a]); got {abilities:?}"
    );

    // Walk every Static effect (peel Expanded) looking for the two static rows.
    let mut effs = Vec::new();
    for a in abilities {
        statics(a, &mut effs);
    }
    // (2) the host-restriction Cant(Attach) row.
    assert!(
        effs.iter().any(|e| matches!(peel(e),
            StaticEffect::Deontic(d) if matches!(deontic_inner(d), Some(DeonticAction::Attach { .. })))),
        "Enchant confers Cant(Attach(... to Not(Param(0)))) ([CR#702.5a]); got {effs:?}"
    );
    // (3) the AsEnters self-replacement (enters attached).
    assert!(
        effs.iter()
            .any(|e| matches!(peel(e), StaticEffect::Replacement(r) if is_also(r))),
        "Enchant confers AsEnters(Attach(...)) ([CR#303.4f]); got {effs:?}"
    );
}

/// [CR#702.67a]: **Fortify** confers an Activated ability — sorcery-speed,
/// targeting a land you control, with an `Attach` effect (Equip's Land twin).
#[test]
fn fortify_confers_sorcery_speed_attach_activated() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Effect;
    use deckmaste_core::Window;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Fortify([Tap])")
        .expect("Fortify expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Fortify body is a Composite");
    };
    let act = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .expect("Fortify confers an Activated ability");
    assert!(
        matches!(act.window, Some(Window::SorcerySpeed)),
        "fortify is sorcery-speed ([CR#702.67a]); got {:?}",
        act.window
    );
    let Effect::Targeted(t) = &act.effect else {
        panic!(
            "fortify's effect is a Targeted wrapper; got {:?}",
            act.effect
        );
    };
    assert!(!t.targets.is_empty(), "fortify targets a land");
    assert!(
        matches!(&*t.effect, Effect::Act(Action::Attach { .. })),
        "fortify's inner effect is Attach; got {:?}",
        t.effect
    );
}

/// [CR#702.151a]: **Reconfigure** confers TWO activated abilities — attach to
/// another target creature you control (sorcery speed), and unattach if
/// attached (sorcery speed). The [CR#702.151b] creature-suppression static
/// ("isn't a creature while attached") is a documented engine SEAM (it needs
/// condition-gated layer-4 type removal, which the layer pipeline doesn't have
/// yet) and is intentionally NOT authored here — see Reconfigure.ron.
#[test]
fn reconfigure_confers_attach_and_unattach_activated() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Effect;
    use deckmaste_core::Window;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Reconfigure([Tap])")
        .expect("Reconfigure expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Reconfigure body is a Composite");
    };
    let acts: Vec<_> = abilities
        .iter()
        .filter_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .collect();
    assert_eq!(acts.len(), 2, "Reconfigure confers two activated abilities");
    // Both are sorcery-speed.
    assert!(
        acts.iter()
            .all(|a| matches!(a.window, Some(Window::SorcerySpeed))),
        "both reconfigure abilities are sorcery-speed ([CR#702.151a])"
    );
    // One attaches, one unattaches.
    assert!(
        acts.iter().any(|a| matches!(&a.effect,
            Effect::Targeted(t) if matches!(&*t.effect, Effect::Act(Action::Attach { .. })))),
        "reconfigure has an Attach ability"
    );
    assert!(
        acts.iter()
            .any(|a| matches!(&a.effect, Effect::Act(Action::Unattach(_)))),
        "reconfigure has an Unattach ability"
    );
}

/// [CR#702.131b]: **Ascend** on a permanent confers a state-checked static —
/// modeled as the generic `Sba { when, then }` primitive (the same shape the
/// Aura graveyard rule uses, swept generically). Proves the macro expands to a
/// `Static` ability whose effects carry a reachable `StaticEffect::Sba`.
#[test]
fn ascend_macro_expands_to_static_sba() {
    use deckmaste_core::Ability;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Filter;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationFilter;
    use deckmaste_core::StateFilter;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Zone;

    // Walk every Static effect (peel Expanded) and look for an Sba row.
    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.extend(s.effects.iter().cloned()),
            Ability::Expanded(e) => statics(&e.value, out),
            _ => {}
        }
    }
    fn peel(e: &StaticEffect) -> &StaticEffect {
        match e {
            StaticEffect::Expanded(x) => peel(&x.value),
            other => other,
        }
    }

    let plugin = builtin();
    let kw: KeywordAbility = plugin.macros.read_str("Ascend").expect("Ascend expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Ascend", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Ascend body is a Composite");
    };

    let mut effs = Vec::new();
    for a in abilities {
        statics(a, &mut effs);
    }
    let when = effs
        .iter()
        .find_map(|e| match peel(e) {
            StaticEffect::Sba { when, .. } => Some(when.clone()),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!("Ascend confers a Static carrying an Sba ([CR#702.131b]); got {effs:?}")
        });

    // Drift guard: the macro's Sba `when` must equal the canonical Ascend gate
    // ([CR#702.131a,702.131b]) — the same typed `Condition` the spell-form
    // `ASCEND_GATE` and the engine helper use. A macro edit that diverges fails.
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
        when,
        Box::new(canonical),
        "Ascend macro's Sba gate drifted from the canonical Ascend gate"
    );
}

/// [CR#702.29a]: **Cycling** confers an Activated ability that functions from
/// HAND, whose cost is the printed cost followed by "discard this card", and
/// whose effect is "draw a card". The printed cost is the macro's list param,
/// spliced ahead of the fixed discard-self as a nested `Cost`. Read is
/// FAITHFUL — the nested `Cost` survives lumpy — and `Cost::normalize` splices
/// it into one flat list.
#[test]
fn cycling_confers_from_hand_discard_self_draw() {
    use deckmaste_core::Ability;
    use deckmaste_core::Cost;
    use deckmaste_core::Effect;
    use deckmaste_core::Normalize;
    use deckmaste_core::Zone;
    use deckmaste_core::ron::options as ron_options;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Cycling([Mana([Generic(2)])])")
        .expect("Cycling expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Cycling", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Cycling body is a Composite");
    };
    let act = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .expect("Cycling confers an Activated ability");

    // (1) Functions from hand ([CR#702.29a]).
    assert_eq!(act.from, Some(Zone::Hand), "cycling activates from hand");

    // (2) Cost = printed cost ({2}) THEN discard this card. Read is faithful:
    // the printed cost rides in a nested `Cost` ahead of the fixed
    // discard-self, so the authored cost is LUMPY.
    let lumpy_cost: Cost = ron_options()
        .from_str("[Cost([Mana([Generic(2)])]), Do(Discard(count: Literal(1), what: This))]")
        .unwrap();
    assert_eq!(
        act.cost, lumpy_cost,
        "cycling cost reads lumpy (nested Cost survives the macro splice)"
    );
    // `.normalize()` splices the nested Cost into one flat list.
    let flat_cost: Cost = ron_options()
        .from_str("[Mana([Generic(2)]), Do(Discard(count: Literal(1), what: This))]")
        .unwrap();
    assert_eq!(
        act.cost.clone().normalize(),
        flat_cost,
        "cycling cost normalizes to printed cost + discard this card"
    );

    // (3) Effect = draw a card.
    let expected_effect: Effect = ron_options().from_str("Draw(Literal(1))").unwrap();
    assert_eq!(act.effect, expected_effect, "cycling draws a card");
}

fn deontic_inner(d: &deckmaste_core::Deontic) -> Option<&deckmaste_core::DeonticAction> {
    use deckmaste_core::Deontic;
    match d {
        Deontic::Cant(a) | Deontic::May(a) | Deontic::Must(a) | Deontic::Gate(a, _) => Some(a),
        Deontic::Expanded(e) => deontic_inner(&e.value),
    }
}

fn is_also(r: &deckmaste_core::Replacement) -> bool {
    use deckmaste_core::Replacement;
    match r {
        Replacement::Also { .. } => true,
        Replacement::Expanded(e) => is_also(&e.value),
        _ => false,
    }
}
