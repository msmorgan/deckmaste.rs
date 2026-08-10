//! Force-expansion proof for every builtin keyword macro: bodies parse
//! LAZILY, so a load-passing plugin proves only the declaration shells —
//! this test invokes each macro (sample args for the parameterized ones)
//! and requires the body to deserialize as a real `KeywordAbility`.

use std::path::Path;
use std::sync::Arc;

use deckmaste_core::KeywordAbility;
use deckmaste_plugin::plugin::Plugin;

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
        ("Exalted", "Exalted"),
        ("BattleCry", "BattleCry"),
        ("Hexproof()", "Hexproof"),
        ("Hexproof(from: ColorIs(Blue))", "Hexproof"),
        ("Ward(cost: [Tap])", "Ward"),
        ("Kicker([Tap])", "Kicker"),
        ("Flashback([Tap])", "Flashback"),
        ("Evoke([Tap])", "Evoke"),
        ("Equip([Tap])", "Equip"),
        ("Fortify([Tap])", "Fortify"),
        ("Reconfigure([Tap])", "Reconfigure"),
        ("Enchant(Type(Creature))", "Enchant"),
        ("Protection(ColorIs(Red))", "Protection"),
        ("Crew(2)", "Crew"),
        ("Affinity(Type(Artifact))", "Affinity"),
        ("Cycling([Mana([Generic(2)])])", "Cycling"),
        ("Reinforce(1, [Mana([Generic(1),Green])])", "Reinforce"),
        ("Echo([Mana([Generic(1),Green])])", "Echo"),
        ("CumulativeUpkeep([Mana([Generic(1)])])", "CumulativeUpkeep"),
        ("Bushido(1)", "Bushido"),
        ("Wither", "Wither"),
        ("Infect", "Infect"),
        ("Modular(1)", "Modular"),
        ("Undying", "Undying"),
        ("Persist", "Persist"),
        ("Evolve", "Evolve"),
        ("Bloodthirst(6)", "Bloodthirst"),
        ("Graft(1)", "Graft"),
        ("Soulshift(3)", "Soulshift"),
        ("Afterlife(2)", "Afterlife"),
        ("Mentor", "Mentor"),
        ("Training", "Training"),
        ("Outlast([Mana([White])])", "Outlast"),
        ("Scavenge([Mana([Generic(2)])])", "Scavenge"),
        // core-copy-grammar Task 9: the copy-consuming keyword abilities.
        ("Embalm([Mana([Generic(3),White])])", "Embalm"),
        ("Eternalize([Mana([Generic(4),Black])])", "Eternalize"),
        ("Offspring([Mana([Generic(1)])])", "Offspring"),
        // Per-pip alternative payment ([CR#702.51a,702.66a,702.126a]): each
        // confers `PayPips` statics; all nullary.
        ("Convoke", "Convoke"),
        ("Delve", "Delve"),
        ("Improvise", "Improvise"),
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

#[test]
fn crew_expands_to_a_tap_total_activation() {
    use deckmaste_core::Ability;
    use deckmaste_core::Cmp;
    use deckmaste_core::CostComponent;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Stat;

    let plugin = builtin();
    let keyword: KeywordAbility = plugin.macros.read_str("Crew(3)").unwrap();
    let KeywordAbility::Expanded(expanded) = keyword else {
        panic!("Crew should retain its macro expansion");
    };
    let KeywordAbility::Composite { abilities, .. } = expanded.value.as_ref() else {
        panic!("Crew should lower to a composite keyword");
    };
    let [Ability::Activated(ability)] = abilities.as_slice() else {
        panic!("Crew should confer exactly one activated ability: {abilities:?}");
    };
    assert!(matches!(
        ability.cost.as_ref(),
        [CostComponent::TapTotal {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: deckmaste_core::Count::Literal(3),
            ..
        }]
    ));
    assert!(matches!(ability.effect, OneShotEffect::Continuously(_)));
}

/// [CR#702.51a,702.66a,702.126a]: the per-pip alternative-payment keywords
/// confer the actual `PayPips` STATICS, not merely a name-carrying Composite —
/// `every_builtin_keyword_macro_expands` checks only the printed name, so a
/// body that dropped or mangled the `PayPips` rows would slip past it. Convoke
/// confers a `Generic` tap clause plus one `Colored(c)` tap clause per color;
/// delve a single `Generic` EXILE clause; improvise a single `Generic` tap
/// clause filtered to artifacts.
#[test]
fn convoke_delve_improvise_confer_pay_pips_statics() {
    use deckmaste_core::Ability;
    use deckmaste_core::Color;
    use deckmaste_core::PayAct;
    use deckmaste_core::PipClass;
    use deckmaste_core::StaticEffect;

    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.push(s.as_ref().clone()),
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
    // Expand a keyword invocation to the flat list of its `PayPips` rows.
    fn pay_pips(plugin: &Plugin, invocation: &str) -> Arc<[(PipClass, PayAct)]> {
        let kw: KeywordAbility = plugin
            .macros
            .read_str(invocation)
            .unwrap_or_else(|e| panic!("expanding {invocation}: {e}"));
        let KeywordAbility::Expanded(expanded) = &kw else {
            panic!("expected Expanded for {invocation}, got {kw:?}");
        };
        let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
            panic!("{invocation} body is a Composite");
        };
        let mut effs = Vec::new();
        for a in abilities {
            statics(a, &mut effs);
        }
        effs.iter()
            .filter_map(|e| match peel(e) {
                StaticEffect::PayPips(class, act) => Some((*class, act.clone())),
                _ => None,
            })
            .collect()
    }

    let plugin = builtin();

    // Convoke: one Generic tap clause + one Colored(c) tap clause per color, all
    // `TapToPay` (never `ExileToPay`) ([CR#702.51a]).
    let convoke = pay_pips(&plugin, "Convoke");
    assert!(
        convoke
            .iter()
            .all(|(_, act)| matches!(act, PayAct::TapToPay(_))),
        "every convoke clause taps to pay; got {convoke:?}"
    );
    assert!(
        convoke.iter().any(|(c, _)| matches!(c, PipClass::Generic)),
        "convoke confers a Generic tap clause; got {convoke:?}"
    );
    for color in [
        Color::White,
        Color::Blue,
        Color::Black,
        Color::Red,
        Color::Green,
    ] {
        assert!(
            convoke
                .iter()
                .any(|(c, _)| matches!(c, PipClass::Colored(cc) if *cc == color)),
            "convoke confers a Colored({color:?}) tap clause; got {convoke:?}"
        );
    }

    // Delve: a single Generic EXILE clause ([CR#702.66a]).
    let delve = pay_pips(&plugin, "Delve");
    assert!(
        matches!(delve.as_ref(), [(PipClass::Generic, PayAct::ExileToPay(_))]),
        "delve confers exactly one Generic ExileToPay clause; got {delve:?}"
    );

    // Improvise: a single Generic TAP clause ([CR#702.126a]).
    let improvise = pay_pips(&plugin, "Improvise");
    assert!(
        matches!(
            improvise.as_ref(),
            [(PipClass::Generic, PayAct::TapToPay(_))]
        ),
        "improvise confers exactly one Generic TapToPay clause; got {improvise:?}"
    );
}

/// [CR#702.5a,303.4a,303.4f]: the **Enchant** keyword confers THREE abilities,
/// not just the legal-host grant: (1) a targeting `Spell` (target spec
/// only, no-op effect) so cast targeting stays on the live `spell_targets`
/// path; (2) the removable `May(Attach(what: Ref(This), to: Param(0)))`
/// host grant (default-deny: being attachable to the quality is granted);
/// (3) `AsEnters(Attach(Ref(This), Param(0)))` so the Aura enters attached.
/// (The conferral map in the spec.)
#[test]
fn enchant_confers_spell_may_attach_and_as_enters() {
    use deckmaste_core::Ability;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::StaticEffect;

    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.push(s.as_ref().clone()),
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
                if matches!(&s.effect, deckmaste_core::OneShotEffect::Targeted(t) if !t.targets.is_empty()))),
        "Enchant confers a targeting Spell ability ([CR#303.4a]); got {abilities:?}"
    );

    // Walk every Static effect (peel Expanded) looking for the two static rows.
    let mut effs = Vec::new();
    for a in abilities {
        statics(a, &mut effs);
    }
    // (2) the host-grant May(Attach) row.
    assert!(
        effs.iter().any(|e| matches!(peel(e),
            StaticEffect::Deontic(d) if matches!(deontic_inner(d), Some(DeonticAction::Attach { .. })))),
        "Enchant confers May(Attach(... to Param(0))) ([CR#702.5a]); got {effs:?}"
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
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Timing;

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
        matches!(act.window, Some(Timing::SorcerySpeed)),
        "fortify is sorcery-speed ([CR#702.67a]); got {:?}",
        act.window
    );
    let OneShotEffect::Targeted(t) = &act.effect else {
        panic!(
            "fortify's effect is a Targeted wrapper; got {:?}",
            act.effect
        );
    };
    assert!(!t.targets.is_empty(), "fortify targets a land");
    assert!(
        matches!(&*t.effect, OneShotEffect::Act(Action::Attach { .. })),
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
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Timing;

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
    let acts: Arc<[_]> = abilities
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
            .all(|a| matches!(a.window, Some(Timing::SorcerySpeed))),
        "both reconfigure abilities are sorcery-speed ([CR#702.151a])"
    );
    // One attaches, one unattaches.
    assert!(
        acts.iter().any(|a| matches!(&a.effect,
            OneShotEffect::Targeted(t) if matches!(&*t.effect, OneShotEffect::Act(Action::Attach { .. })))),
        "reconfigure has an Attach ability"
    );
    assert!(
        acts.iter()
            .any(|a| matches!(&a.effect, OneShotEffect::Act(Action::Unattach(_)))),
        "reconfigure has an Unattach ability"
    );
}

/// [CR#702.107a]: **Outlast** confers an Activated ability — sorcery-speed,
/// whose cost is the printed [cost] PLUS `{T}` ([CR#107.5]), and whose effect
/// puts a +1/+1 counter on this creature ([CR#122.1a]). The printed cost is the
/// macro's list param, inlined ahead of the fixed `Tap` by the generic
/// `Splice(Param(0))` list-splice ([typed-holes delta 5]) — flat at read time.
#[test]
fn outlast_confers_sorcery_speed_tap_put_counter() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Cost;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Timing;
    use deckmaste_core::ron::options as ron_options;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Outlast([Mana([White])])")
        .expect("Outlast expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Outlast", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Outlast body is a Composite");
    };
    let act = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .expect("Outlast confers an Activated ability");

    // (1) Activate only as a sorcery ([CR#702.107a]/[CR#602.5d]).
    assert!(
        matches!(act.window, Some(Timing::SorcerySpeed)),
        "outlast is sorcery-speed ([CR#702.107a]); got {:?}",
        act.window
    );

    // (2) Cost = printed cost ({W}) THEN {T}. `Splice(Param(0))` inlines the
    // printed cost ahead of the fixed `Tap` at read time, so the cost is FLAT.
    let flat_cost: Cost = ron_options().from_str("[Mana([White]), Tap]").unwrap();
    assert_eq!(
        act.cost, flat_cost,
        "outlast cost is the param cost plus {{T}}, spliced flat ([CR#702.107a])"
    );

    // (3) OneShotEffect puts one +1/+1 counter on THIS creature ([CR#122.1a]).
    // PutCounters is agent-silent ([CR#122.1..122.6]).
    let OneShotEffect::Act(Action::PutCounters(_, counter, count)) = &act.effect else {
        panic!("outlast's effect is PutCounters; got {:?}", act.effect);
    };
    assert_eq!(
        counter.as_str(),
        "P1P1Counter",
        "outlast puts a +1/+1 counter ([CR#122.1a])"
    );
    assert_eq!(*count, Count::Literal(1), "outlast puts ONE counter");
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
    use deckmaste_core::Countable;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Zone;

    // Walk every Static effect (peel Expanded) and look for an Sba row.
    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.push(s.as_ref().clone()),
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
    let canonical = Condition::And(
        vec![
            Condition::Compare(
                Count::CountOf(Countable::Objects(Arc::new(Predicate::And(
                    vec![
                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                            Predicate::Ref(Reference::You),
                        ))),
                    ]
                    .into(),
                )))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Arc::new(Condition::Matches(
                Reference::You,
                Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
            ))),
        ]
        .into(),
    );
    assert_eq!(
        when,
        Arc::new(canonical),
        "Ascend macro's Sba gate drifted from the canonical Ascend gate"
    );
}

/// [CR#702.29a]: **Cycling** confers an Activated ability that functions from
/// HAND, whose cost is the printed cost followed by "discard this card", and
/// whose effect is "draw a card". The printed cost is the macro's list param,
/// inlined ahead of the fixed discard-self by the generic `Splice(Param(0))`
/// list-splice ([typed-holes delta 5]) — so the cost reads FLAT at expansion
/// time, no nested `Cost` wrapper, no post-read normalize step.
#[test]
fn cycling_confers_from_hand_discard_self_draw() {
    use deckmaste_core::Ability;
    use deckmaste_core::Cost;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Zone;

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

    // (2) Cost = printed cost ({2}) THEN discard this card (the `DiscardThis`
    // cost macro — the bound single-move discard composite, [CR#702.29a]).
    // `Splice(Param(0))` inlines the printed cost ahead of the fixed
    // discard-self at read time, so the cost is FLAT — no nested `Cost`
    // wrapper. Read back through the same macro set so the remembered
    // `Expanded` shape matches exactly.
    let flat_cost: Cost = plugin
        .macros
        .read_str("[Mana([Generic(2)]), DiscardThis]")
        .unwrap();
    assert_eq!(
        act.cost, flat_cost,
        "cycling cost is the printed cost + discard this card, spliced flat"
    );

    // (3) OneShotEffect = draw a card — the `Draw(1)` macro (Cycling's body
    // uses `effect: Draw(1)`), read back through the same macro set so the
    // remembered `Expanded` shape matches exactly.
    let expected_effect: OneShotEffect = plugin.macros.read_str("Draw(1)").unwrap();
    assert_eq!(act.effect, expected_effect, "cycling draws a card");
}

/// [CR#702.77a]: **Reinforce N—[cost]** confers an Activated ability that
/// functions from HAND, whose cost is the printed cost followed by "discard
/// this card" (the Cycling discard-this construction), and whose effect is "put
/// N +1/+1 counters on target creature". The printed cost is the macro's list
/// param (`Param(1)`), inlined ahead of the fixed discard-self by the generic
/// `Splice(Param(1))` list-splice (flat at read time); N is `Param(0)`, the
/// counter amount.
#[test]
fn reinforce_confers_from_hand_discard_self_put_counters() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Cost;
    use deckmaste_core::Count;
    use deckmaste_core::CounterRef;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Reinforce(2, [Mana([Generic(1),Green])])")
        .expect("Reinforce expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Reinforce", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Reinforce body is a Composite");
    };
    let act = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .expect("Reinforce confers an Activated ability");

    // (1) Functions from hand ([CR#702.77a]).
    assert_eq!(act.from, Some(Zone::Hand), "reinforce activates from hand");

    // (2) Cost = printed cost ({1}{G}) THEN discard this card (the
    // `DiscardThis` cost macro — the bound single-move discard composite,
    // [CR#702.29a]). `Splice(Param(1))` inlines the printed cost ahead of the
    // fixed discard-self at read time, so the cost is FLAT — no nested `Cost`
    // wrapper. Read back through the same macro set.
    let flat_cost: Cost = plugin
        .macros
        .read_str("[Mana([Generic(1),Green]), DiscardThis]")
        .unwrap();
    assert_eq!(
        act.cost, flat_cost,
        "reinforce cost is the printed cost + discard this card, spliced flat"
    );

    // (3) OneShotEffect = put N +1/+1 counters on target creature ([CR#702.77a]).
    let OneShotEffect::Targeted(t) = &act.effect else {
        panic!(
            "reinforce's effect is a Targeted wrapper; got {:?}",
            act.effect
        );
    };
    assert_eq!(t.targets.len(), 1, "reinforce targets exactly one creature");
    let TargetSpec::Target(_, filter) = &t.targets[0] else {
        panic!("expected a Target spec; got {:?}", t.targets[0]);
    };
    assert_eq!(
        filter,
        &Predicate::r#type(Type::Creature),
        "reinforce targets a creature; got {filter:?}"
    );
    // Inner effect places N (= Param(0) = 2) +1/+1 counters on the target.
    // PutCounters is agent-silent ([CR#122.1]).
    let OneShotEffect::Act(Action::PutCounters(sel, counter, count)) = &*t.effect else {
        panic!(
            "reinforce's inner effect is PutCounters; got {:?}",
            t.effect
        );
    };
    assert_eq!(
        *sel,
        Reference::It,
        "reinforce puts counters on the announced target (the It anaphor)"
    );
    assert_eq!(
        counter,
        &CounterRef::from("P1P1Counter"),
        "reinforce places +1/+1 counters; got {counter:?}"
    );
    assert_eq!(
        *count,
        Count::Literal(2),
        "reinforce places N counters (Param(0))"
    );
}

/// [CR#702.97a]: **Scavenge** confers an Activated ability that functions from
/// the GRAVEYARD, whose cost is the printed cost followed by "exile this card
/// from your graveyard", that activates only as a sorcery, and whose effect
/// puts a number of +1/+1 counters equal to this card's power on target
/// creature. The printed cost is the macro's list param, inlined ahead of the
/// fixed exile-self by the generic `Splice(Param(0))` list-splice (Cycling's
/// discard-self twin) — flat at read time, no nested `Cost` wrapper.
#[test]
fn scavenge_confers_from_graveyard_exile_self_sorcery_counters() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Cost;
    use deckmaste_core::Count;
    use deckmaste_core::CounterRef;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Timing;
    use deckmaste_core::Zone;
    use deckmaste_core::ron::options as ron_options;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Scavenge([Mana([Generic(2)])])")
        .expect("Scavenge expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Scavenge", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Scavenge body is a Composite");
    };
    let act = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Activated(act) => Some(act),
            _ => None,
        })
        .expect("Scavenge confers an Activated ability");

    // (1) Functions from the graveyard ([CR#702.97a]).
    assert_eq!(
        act.from,
        Some(Zone::Graveyard),
        "scavenge activates from the graveyard"
    );

    // (2) Activate only as a sorcery ([CR#702.97a], [CR#602.5d]).
    assert_eq!(
        act.window,
        Some(Timing::SorcerySpeed),
        "scavenge is sorcery-speed"
    );

    // (3) Cost = printed cost ({2}) THEN exile this card. Exile is a pure zone
    // move now (`Move(This, Exile)`, [CR#701.13]). `Splice(Param(0))` inlines the
    // printed cost ahead of the fixed exile-self at read time (Cycling's
    // discard-self twin), so the cost is FLAT — no nested `Cost` wrapper.
    let flat_cost: Cost = ron_options()
        .from_str("[Mana([Generic(2)]), Do(Move(This, Exile))]")
        .unwrap();
    assert_eq!(
        act.cost, flat_cost,
        "scavenge cost is the printed cost + exile this card, spliced flat"
    );

    // (4) OneShotEffect = put +1/+1 counters equal to this card's power on target
    // creature ([CR#702.97a]). One creature target, inner PutCounters reads
    // `StatOf(This, Power)` for the magnitude.
    let OneShotEffect::Targeted(t) = &act.effect else {
        panic!(
            "scavenge's effect is a Targeted wrapper; got {:?}",
            act.effect
        );
    };
    assert_eq!(t.targets.len(), 1, "scavenge targets exactly one creature");
    let TargetSpec::Target(_, _) = &t.targets[0] else {
        panic!("expected a Target spec; got {:?}", t.targets[0]);
    };
    let OneShotEffect::Act(Action::PutCounters(sel, kind, count)) = &*t.effect else {
        panic!(
            "scavenge's inner effect puts counters on a player verb; got {:?}",
            t.effect
        );
    };
    assert_eq!(
        sel,
        &Reference::It,
        "scavenge counters land on the announced target ([CR#702.97a])"
    );
    assert_eq!(
        kind,
        &CounterRef::from("P1P1Counter"),
        "scavenge places +1/+1 counters"
    );
    assert_eq!(
        count,
        &Count::StatOf(Reference::This, Stat::Power),
        "scavenge counter count = this card's power ([CR#702.97a])"
    );
}

/// [CR#702.46a]: **Soulshift N** confers a dies-triggered ability — `you may
/// return target Spirit card with mana value N or less from your graveyard to
/// your hand`. Proves the macro expands to a `Triggered(ThisDies)` whose effect
/// is a `May` over a `Targeted` whose one target filters
/// Spirit ∧ in-your-graveyard ∧ mana value ≤ N (the printed `Param(0)`), and
/// whose inner effect is `Move(Target(0), Hand)` — the announced target read
/// back positionally (the dies-trigger's own event role rules out the bare
/// `It` anaphor here).
#[test]
fn soulshift_confers_dies_may_return_spirit_from_graveyard() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Cmp;
    use deckmaste_core::Count;
    use deckmaste_core::Destination;
    use deckmaste_core::EventFilter;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::Stat;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Soulshift(3)")
        .expect("Soulshift expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Soulshift", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Soulshift body is a Composite");
    };
    let trig = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Triggered(t) => Some(t),
            _ => None,
        })
        .expect("Soulshift confers a Triggered ability");
    // Dies trigger ([CR#700.4]) — the `ThisDies` macro invocation.
    assert!(
        matches!(&trig.event, EventFilter::Expanded(e) if e.name.as_str() == "ThisDies"),
        "soulshift triggers on dies; got {:?}",
        trig.event
    );
    // The effect is a `May` ([CR#702.46a] "you may").
    let OneShotEffect::May(may) = &trig.effect else {
        panic!("soulshift's effect is a May; got {:?}", trig.effect);
    };
    // … over a `Targeted` ([CR#115.1,601.2c]).
    let OneShotEffect::Targeted(t) = &*may.effect else {
        panic!("soulshift's May wraps a Targeted; got {:?}", may.effect);
    };
    // One target whose filter is Spirit ∧ in-graveyard ∧ owned-by-you ∧ MV ≤ N.
    assert_eq!(t.targets.len(), 1, "soulshift targets exactly one card");
    let TargetSpec::Target(_, filter) = &t.targets[0] else {
        panic!("expected a Target spec; got {:?}", t.targets[0]);
    };
    let Predicate::And(clauses) = filter else {
        panic!("soulshift target is an And; got {filter:?}");
    };
    assert!(
        clauses
            .iter()
            .any(|f| matches!(f, Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) if s.name().as_str() == "Spirit")),
        "soulshift target filters Spirit ([CR#702.46a]); got {clauses:?}"
    );
    assert!(
        clauses
            .iter()
            .any(|f| matches!(f, Predicate::State(StatePredicate::InZone(Zone::Graveyard)))),
        "soulshift target is in a graveyard; got {clauses:?}"
    );
    assert!(
        clauses
            .iter()
            .any(|f| matches!(f, Predicate::Relation(RelationPredicate::Owner(o)) if matches!(&**o, Predicate::Ref(Reference::You)))),
        "soulshift target is owned by you (your graveyard); got {clauses:?}"
    );
    assert!(
        clauses.iter().any(|f| matches!(
            f,
            Predicate::Characteristic(CharacteristicPredicate::Stat(
                Stat::ManaValue,
                Cmp::AtMost,
                Count::Literal(3)
            ))
        )),
        "soulshift target has mana value ≤ N (the printed Param(0)) ([CR#202.3]); got {clauses:?}"
    );
    // Inner effect returns the chosen card to its owner's hand ([CR#400.7]).
    assert!(
        matches!(
            &*t.effect,
            OneShotEffect::Act(Action::Move(
                Reference::Target(0),
                Destination::Zone(Zone::Hand),
                _,
                _,
            ))
        ),
        "soulshift returns target to hand; got {:?}",
        t.effect
    );
}

/// [CR#702.135a]: **Afterlife N** confers a dies-triggered ability — `create N
/// 1/1 white and black Spirit creature tokens with flying`. Proves the macro
/// expands to a `Triggered(ThisDies)` whose effect creates `Param(0)` tokens of
/// an inline creature `Token` that is W∧B (color indicator, [CR#202.2e]), 1/1
/// ([CR#111.3]), a Spirit, and carries flying.
#[test]
fn afterlife_confers_dies_create_spirit_tokens_with_flying() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Color;
    use deckmaste_core::Count;
    use deckmaste_core::EventFilter;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Reference;
    use deckmaste_core::StatValue;
    use deckmaste_core::TokenSpec;
    use deckmaste_core::Type;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Afterlife(2)")
        .expect("Afterlife expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    assert_eq!(expanded.name.as_str(), "Afterlife", "carried name");
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Afterlife body is a Composite");
    };
    let trig = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Triggered(t) => Some(t),
            _ => None,
        })
        .expect("Afterlife confers a Triggered ability");
    assert!(
        matches!(&trig.event, EventFilter::Expanded(e) if e.name.as_str() == "ThisDies"),
        "afterlife triggers on dies; got {:?}",
        trig.event
    );
    // Create's agent is spelled ([CR#111.1]).
    let OneShotEffect::Act(Action::Create {
        agent: Reference::You,
        count,
        token: TokenSpec::Token(token),
        ..
    }) = &trig.effect
    else {
        panic!(
            "afterlife's effect creates inline tokens; got {:?}",
            trig.effect
        );
    };
    // N = the printed Param(0).
    assert_eq!(*count, Count::Literal(2), "afterlife creates N tokens");
    // 1/1 ([CR#111.3]).
    assert_eq!(token.power, Some(StatValue::Number(1)), "token power 1");
    assert_eq!(
        token.toughness,
        Some(StatValue::Number(1)),
        "token toughness 1"
    );
    // White and black ([CR#202.2e] color indicator).
    assert!(
        token.color_indicator.contains(&Color::White)
            && token.color_indicator.contains(&Color::Black),
        "token is white and black; got {:?}",
        token.color_indicator
    );
    // A creature Spirit.
    assert!(
        token.types.iter().any(|t| t.name == Type::Creature.name()),
        "token is a creature"
    );
    assert!(
        token.subtypes.iter().any(|s| s.name.as_str() == "Spirit"),
        "token is a Spirit; got {:?}",
        token.subtypes
    );
    // With flying — a Keyword(Flying) ability (peel Expanded).
    assert!(
        token.abilities.iter().any(|a| names_keyword(a, "Flying")),
        "token has flying; got {:?}",
        token.abilities
    );
}

/// Whether `a` is (or peels to) a keyword ability whose carried name is `name`
/// — a `Keyword(Flying)` reads as `Ability::Keyword(KeywordAbility::Expanded)`,
/// and an outer `Ability::Expanded` wrapper is looked through.
fn names_keyword(a: &deckmaste_core::Ability, name: &str) -> bool {
    use deckmaste_core::Ability;
    match a {
        Ability::Expanded(e) => e.name.as_str() == name || names_keyword(&e.value, name),
        Ability::Keyword(KeywordAbility::Expanded(e)) => e.name.as_str() == name,
        _ => false,
    }
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
