//! Force-expansion proof for every builtin keyword macro: bodies parse
//! LAZILY, so a load-passing plugin proves only the declaration shells —
//! this test invokes each macro (sample args for the parameterized ones)
//! and requires the body to deserialize as a real `KeywordAbility`.

use std::path::Path;
use std::sync::Arc;

use deckmaste_core::KeywordAbility;
use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_semantics::KeywordAbility as SemanticKeywordAbility;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn read_keyword(plugin: &Plugin, invocation: &str) -> KeywordAbility {
    let semantic: SemanticKeywordAbility = plugin
        .macros
        .read_str(invocation)
        .unwrap_or_else(|error| panic!("expanding {invocation}: {error}"));
    semantic.lower()
}

fn lower_activated_cost(cost: deckmaste_semantics::Cost) -> deckmaste_core::Cost {
    deckmaste_semantics::ActivatedAbility {
        ability_word: None,
        cost,
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect: deckmaste_semantics::OneShotEffect::Sequentially([].into()),
    }
    .lower()
    .cost
}

fn assert_mana_then_discard_this(
    actual: &deckmaste_core::Cost,
    printed: deckmaste_semantics::Cost,
) {
    use deckmaste_core::Action;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Destination;
    use deckmaste_core::Instruction;
    use deckmaste_core::Reference;
    use deckmaste_core::Zone;

    let printed = lower_activated_cost(printed);
    let [expected_mana] = printed.0.as_ref() else {
        panic!("printed keyword cost lowers to one mana component");
    };
    let [
        actual_mana,
        CostComponent::Act {
            dest: None,
            action: discard,
        },
    ] = actual.0.as_ref()
    else {
        panic!("keyword cost is mana followed by discard-this; got {actual:?}");
    };
    assert_eq!(actual_mana, expected_mana, "printed mana cost is preserved");
    let Action::Composite { body, .. } = discard.as_action() else {
        panic!("discard-this cost is a composite action; got {discard:?}");
    };
    assert!(
        matches!(
            body.as_ref(),
            Instruction::Act {
                action: Action::Move(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Destination::Zone(Zone::Graveyard),
                    _,
                    _,
                ),
                ..
            }
        ),
        "keyword cost discards this card; got {discard:?}"
    );
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
        let kw: SemanticKeywordAbility = plugin
            .macros
            .read_str(invocation)
            .unwrap_or_else(|e| panic!("expanding {invocation}: {e}"));
        let SemanticKeywordAbility::Expanded(expanded) = &kw else {
            panic!("expected Expanded for {invocation}, got {kw:?}");
        };
        assert_eq!(
            expanded.name.as_str(),
            name,
            "carried name for {invocation}"
        );
        assert!(
            matches!(&*expanded.value, SemanticKeywordAbility::Composite { name: n, .. } if n.as_str() == name),
            "body of {invocation} is a name-carrying Composite"
        );
    }
}

#[test]
fn crew_expands_to_a_tap_total_activation() {
    use deckmaste_core::Ability;
    use deckmaste_core::Cmp;
    use deckmaste_core::CollectionOp;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Duration;
    use deckmaste_core::Instruction;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::TurnMarker;

    let plugin = builtin();
    let keyword = read_keyword(&plugin, "Crew(3)");
    let KeywordAbility::Composite { abilities, .. } = &keyword else {
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
    let [Instruction::Continuously(effect)] = ability.effect.body.as_ref() else {
        panic!("Crew's activation creates one continuous effect")
    };
    assert_eq!(effect.duration, Duration::FixedUntil(TurnMarker::EndOfTurn));
    assert!(matches!(
        effect.effect.as_ref(),
        StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::CardTypes(CollectionOp::Add(kind)),
        ) if kind.as_str() == "Creature"
    ));
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
    use deckmaste_core::StaticSpec;

    fn statics(a: &Ability, out: &mut Vec<StaticSpec>) {
        if let Ability::Static(s) = a {
            out.push(s.body.clone());
        }
    }
    fn peel(e: &StaticSpec) -> &StaticSpec {
        e
    }
    // Expand a keyword invocation to the flat list of its `PayPips` rows.
    fn pay_pips(plugin: &Plugin, invocation: &str) -> Arc<[(PipClass, PayAct)]> {
        let kw = read_keyword(plugin, invocation);
        let KeywordAbility::Composite { abilities, .. } = &kw else {
            panic!("{invocation} body is a Composite");
        };
        let mut effs = Vec::new();
        for a in abilities {
            statics(a, &mut effs);
        }
        effs.iter()
            .filter_map(|e| match peel(e) {
                StaticSpec::PayPips(class, act) => Some((*class, act.clone())),
                _ => None,
            })
            .collect()
    }

    let plugin = builtin();

    // Convoke: one Generic tap clause + one Colored(c) tap clause per color,
    // all `TapToPay` (never `ExileToPay`) ([CR#702.51a]).
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
    use deckmaste_core::StaticSpec;

    fn statics(a: &Ability, out: &mut Vec<StaticSpec>) {
        if let Ability::Static(s) = a {
            out.push(s.body.clone());
        }
    }
    fn peel(e: &StaticSpec) -> &StaticSpec {
        e
    }

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Enchant(Type(Creature))");
    let KeywordAbility::Composite { abilities, .. } = &kw else {
        panic!("Enchant body is a Composite");
    };

    // (1) a targeting Spell ability.
    assert!(
        abilities
            .iter()
            .any(|a| matches!(a, Ability::Spell(s) if !s.targets.is_empty())),
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
            StaticSpec::Deontic(d) if matches!(deontic_inner(d), DeonticAction::Attach { .. }))),
        "Enchant confers May(Attach(... to Param(0))) ([CR#702.5a]); got {effs:?}"
    );
    // (3) the AsEnters self-replacement (enters attached).
    assert!(
        effs.iter()
            .any(|e| matches!(peel(e), StaticSpec::Replacement(r) if is_also(r))),
        "Enchant confers AsEnters(Attach(...)) ([CR#303.4f]); got {effs:?}"
    );
}

/// [CR#702.67a]: **Fortify** confers an Activated ability — sorcery-speed,
/// targeting a land you control, with an `Attach` effect (Equip's Land twin).
#[test]
fn fortify_confers_sorcery_speed_attach_activated() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Instruction;
    use deckmaste_core::Timing;
    let plugin = builtin();
    let kw = read_keyword(&plugin, "Fortify([Tap])");
    let KeywordAbility::Composite { abilities, .. } = &kw else {
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
    assert!(!act.targets.is_empty(), "fortify targets a land");
    assert!(
        matches!(
            act.effect.body.as_ref(),
            [Instruction::Act {
                action: Action::Attach { .. },
                ..
            }]
        ),
        "fortify's inner effect is Attach; got {:?}",
        act.effect.body
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
    use deckmaste_core::Instruction;
    use deckmaste_core::Timing;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Reconfigure([Tap])");
    let KeywordAbility::Composite { abilities, .. } = &kw else {
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
        acts.iter().any(|a| !a.targets.is_empty()
            && matches!(
                a.effect.body.as_ref(),
                [Instruction::Act {
                    action: Action::Attach { .. },
                    ..
                }]
            )),
        "reconfigure has an Attach ability"
    );
    assert!(
        acts.iter().any(|a| matches!(
            a.effect.body.as_ref(),
            [Instruction::Act {
                action: Action::Unattach(_),
                ..
            }]
        )),
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
    use deckmaste_core::Instruction;
    use deckmaste_core::Timing;
    use deckmaste_core::ron::options as ron_options;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Outlast([Mana([White])])");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Outlast body is a Composite");
    };
    assert_eq!(name.as_str(), "Outlast", "carried name");
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
    let flat_cost: Cost = ron_options()
        .from_str("[Mana([Simple(Specific(Color(White)))]), Tap]")
        .unwrap();
    assert_eq!(
        act.cost, flat_cost,
        "outlast cost is the param cost plus {{T}}, spliced flat ([CR#702.107a])"
    );

    // (3) Instruction puts one +1/+1 counter on THIS creature ([CR#122.1a]).
    // PutCounters is agent-silent ([CR#122.1..122.6]).
    let [
        Instruction::Act {
            action: Action::PutCounters(_, counter, count),
            ..
        },
    ] = act.effect.body.as_ref()
    else {
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
/// v1's generic `Sba { when, then }` primitive, which lowers onto core's
/// `ConditionallyDo` (the same shape the Aura graveyard rule uses, swept
/// generically). Proves the macro expands to a `Static` ability whose effects
/// carry a reachable `StaticSpec::ConditionallyDo`.
#[test]
fn ascend_macro_expands_to_static_conditionally_do() {
    use deckmaste_core::Ability;
    use deckmaste_core::StaticSpec;

    // Walk every Static effect (peel Expanded) and look for a state-checked row.
    fn statics(a: &Ability, out: &mut Vec<StaticSpec>) {
        if let Ability::Static(s) = a {
            out.push(s.body.clone());
        }
    }
    fn peel(e: &StaticSpec) -> &StaticSpec {
        e
    }

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Ascend");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Ascend body is a Composite");
    };
    assert_eq!(name.as_str(), "Ascend", "carried name");

    let mut effs = Vec::new();
    for a in abilities {
        statics(a, &mut effs);
    }
    let when = effs
        .iter()
        .find_map(|e| match peel(e) {
            StaticSpec::ConditionallyDo { when, .. } => Some(when.clone()),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "Ascend confers a Static carrying a ConditionallyDo ([CR#702.131b]); got \
                 {effs:?}"
            )
        });

    // Drift guard: the macro's `when` must equal the canonical Ascend gate
    // ([CR#702.131a,702.131b]) — the same typed `Condition` the spell-form
    // `ASCEND_GATE` and the engine helper use. A macro edit that diverges
    // fails.
    let canonical: deckmaste_semantics::Ability = plugin
        .macros
        .read_str(
            "Static(Sba(when: And([Compare(CountOf(Objects(And([InZone(Battlefield), \
             ControlledBy(Ref(You))]))), AtLeast, 10), Not(Matches(You, \
             Designated(\"CitysBlessing\")))]), then: GetDesignation(You, \
             \"CitysBlessing\")))",
        )
        .unwrap();
    let Ability::Static(canonical) = canonical.lower() else {
        panic!("canonical Ascend gate lowers as a Static ability");
    };
    let StaticSpec::ConditionallyDo {
        when: canonical, ..
    } = &canonical.body
    else {
        panic!("canonical Ascend gate lowers as a ConditionallyDo");
    };
    assert_eq!(
        &when, canonical,
        "Ascend macro's gate drifted from the canonical Ascend gate"
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
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Cycling([Mana([Generic(2)])])");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Cycling body is a Composite");
    };
    assert_eq!(name.as_str(), "Cycling", "carried name");
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
    // wrapper. The produced action's destination register is allocated in
    // the enclosing ability region, so compare the cost's semantic structure
    // rather than an isolated lowering's register ordinal.
    let printed_cost: deckmaste_semantics::Cost =
        plugin.macros.read_str("[Mana([Generic(2)])]").unwrap();
    assert_mana_then_discard_this(&act.cost, printed_cost);

    // (3) Effect = the independently expanded `Draw(1)` macro, lowered as an
    // activated-ability region so its controller reference receives the same
    // provenance-assigned register as Cycling's body. The reference lowering
    // carries the SAME announcement cost: a cost block's instructions define
    // into the region ahead of the body ([CR#601.2b]), so the discard
    // composite's product occupies a register the body's own definitions come
    // after.
    let expected_effect: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("Draw(1)").unwrap();
    let expected_cost: deckmaste_semantics::Cost = plugin
        .macros
        .read_str("[Mana([Generic(2)]), DiscardThis]")
        .unwrap();
    let expected = deckmaste_semantics::ActivatedAbility {
        ability_word: None,
        cost: expected_cost,
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect: expected_effect,
    }
    .lower()
    .effect;
    assert_eq!(act.effect, expected, "cycling draws a card");
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
    use deckmaste_core::Count;
    use deckmaste_core::CounterRef;
    use deckmaste_core::Instruction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Reinforce(2, [Mana([Generic(1),Green])])");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Reinforce body is a Composite");
    };
    assert_eq!(name.as_str(), "Reinforce", "carried name");
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
    // wrapper. Compare the structure because produced-value register ordinals
    // belong to the enclosing ability region.
    let printed_cost: deckmaste_semantics::Cost = plugin
        .macros
        .read_str("[Mana([Generic(1),Green])]")
        .unwrap();
    assert_mana_then_discard_this(&act.cost, printed_cost);

    // (3) Instruction = put N +1/+1 counters on target creature
    // ([CR#702.77a]).
    assert_eq!(
        act.targets.len(),
        1,
        "reinforce targets exactly one creature"
    );
    let TargetSpec::Target(_, filter) = &act.targets[0] else {
        panic!("expected a Target spec; got {:?}", act.targets[0]);
    };
    assert_eq!(
        &filter.body,
        &Predicate::r#type(Type::Creature),
        "reinforce targets a creature; got {filter:?}"
    );
    // Inner effect places N (= Param(0) = 2) +1/+1 counters on the target.
    // PutCounters is agent-silent ([CR#122.1]).
    let [
        Instruction::Act {
            action: Action::PutCounters(sel, counter, count),
            ..
        },
    ] = act.effect.body.as_ref()
    else {
        panic!(
            "reinforce's inner effect is PutCounters; got {:?}",
            act.effect.body
        );
    };
    assert_eq!(
        *sel,
        Reference::Reg(
            act.effect
                .params
                .iter()
                .find(|param| matches!(
                    param.provenance,
                    deckmaste_core::Provenance::AnnouncedTarget(0)
                ))
                .expect("target parameter")
                .def
                .into(),
        ),
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
    use deckmaste_core::Instruction;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Timing;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Scavenge([Mana([Generic(2)])])");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Scavenge body is a Composite");
    };
    assert_eq!(name.as_str(), "Scavenge", "carried name");
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
    // move now (`Move(This, Exile)`, [CR#701.13]). `Splice(Param(0))` inlines
    // the printed cost ahead of the fixed exile-self at read time
    // (Cycling's discard-self twin), so the cost is FLAT — no nested `Cost`
    // wrapper.
    let flat_cost: deckmaste_semantics::Cost = plugin
        .macros
        .read_str("[Mana([Generic(2)]), Do(Move(This, Exile))]")
        .unwrap();
    let flat_cost: Cost = flat_cost.lower();
    assert_eq!(
        act.cost, flat_cost,
        "scavenge cost is the printed cost + exile this card, spliced flat"
    );

    // (4) Instruction = put +1/+1 counters equal to this card's power on
    // target creature ([CR#702.97a]). One creature target, inner
    // PutCounters reads `StatOf(This, Power)` for the magnitude.
    assert_eq!(
        act.targets.len(),
        1,
        "scavenge targets exactly one creature"
    );
    let TargetSpec::Target(_, _) = &act.targets[0] else {
        panic!("expected a Target spec; got {:?}", act.targets[0]);
    };
    let [
        Instruction::Act {
            action: Action::PutCounters(sel, kind, count),
            ..
        },
    ] = act.effect.body.as_ref()
    else {
        panic!(
            "scavenge's inner effect puts counters on a player verb; got {:?}",
            act.effect.body
        );
    };
    assert_eq!(
        sel,
        &Reference::Reg(
            act.effect
                .params
                .iter()
                .find(|param| matches!(
                    param.provenance,
                    deckmaste_core::Provenance::AnnouncedTarget(0)
                ))
                .expect("target parameter")
                .def
                .into(),
        ),
        "scavenge counters land on the announced target ([CR#702.97a])"
    );
    assert_eq!(
        kind,
        &CounterRef::from("P1P1Counter"),
        "scavenge places +1/+1 counters"
    );
    assert_eq!(
        count,
        &Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
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
    use deckmaste_core::Instruction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::Stat;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Soulshift(3)");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Soulshift body is a Composite");
    };
    assert_eq!(name.as_str(), "Soulshift", "carried name");
    let trig = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Triggered(t) => Some(t),
            _ => None,
        })
        .expect("Soulshift confers a Triggered ability");
    // Dies trigger ([CR#700.4]) — the `ThisDies` macro invocation.
    assert!(
        matches!(
            &trig.event,
            EventFilter::ZoneChange {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            }
        ),
        "soulshift triggers on dies; got {:?}",
        trig.event
    );
    // The effect is a `May` ([CR#702.46a] "you may").
    let [Instruction::May(may)] = trig.effect.body.as_ref() else {
        panic!("soulshift's effect is a May; got {:?}", trig.effect);
    };
    // Target declarations are hoisted onto the triggered ability's region
    // ([CR#115.1,601.2c]).
    assert_eq!(trig.targets.len(), 1, "soulshift targets exactly one card");
    let TargetSpec::Target(_, filter) = &trig.targets[0] else {
        panic!("expected a Target spec; got {:?}", trig.targets[0]);
    };
    let Predicate::And(clauses) = &filter.body else {
        panic!("soulshift target is an And; got {filter:?}");
    };
    let controller = filter
        .params
        .iter()
        .find(|param| matches!(param.provenance, deckmaste_core::Provenance::Controller))
        .expect("target predicate controller parameter")
        .def
        .into();
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
        clauses.iter().any(
            |f| matches!(f, Predicate::Relation(RelationPredicate::Owner(o))
                if matches!(&**o, Predicate::Ref(Reference::Reg(reference)) if *reference == controller))
        ),
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
            &*may.effect,
            Instruction::Act {
                action: Action::Move(Reference::Reg(_), Destination::Zone(Zone::Hand), _, _,),
                ..
            }
        ),
        "soulshift returns target to hand; got {:?}",
        may.effect
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
    use deckmaste_core::Instruction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StatValue;
    use deckmaste_core::TokenSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    let plugin = builtin();
    let kw = read_keyword(&plugin, "Afterlife(2)");
    let KeywordAbility::Composite { name, abilities } = &kw else {
        panic!("Afterlife body is a Composite");
    };
    assert_eq!(name.as_str(), "Afterlife", "carried name");
    let trig = abilities
        .iter()
        .find_map(|a| match a {
            Ability::Triggered(t) => Some(t),
            _ => None,
        })
        .expect("Afterlife confers a Triggered ability");
    assert!(
        matches!(
            &trig.event,
            EventFilter::ZoneChange {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            }
        ),
        "afterlife triggers on dies; got {:?}",
        trig.event
    );
    // Create's agent is spelled ([CR#111.1]).
    let [
        Instruction::Act {
            action:
                Action::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count,
                    token: TokenSpec::Token(token),
                    ..
                },
            ..
        },
    ] = trig.effect.body.as_ref()
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
    // With flying — a lowered, name-carrying keyword ability.
    assert!(
        token.abilities.iter().any(|a| names_keyword(a, "Flying")),
        "token has flying; got {:?}",
        token.abilities
    );
}

/// Whether `a` is a lowered keyword ability whose carried name is `name`.
fn names_keyword(a: &deckmaste_core::Ability, name: &str) -> bool {
    use deckmaste_core::Ability;
    match a {
        Ability::Keyword(KeywordAbility::Composite { name: keyword, .. }) => {
            keyword.as_str() == name
        }
        _ => false,
    }
}

fn deontic_inner(d: &deckmaste_core::Deontic) -> &deckmaste_core::DeonticAction {
    use deckmaste_core::Deontic;
    match d {
        Deontic::Cant(a) | Deontic::May(a) | Deontic::Must(a) | Deontic::Gate(a, _) => a,
    }
}

fn is_also(r: &deckmaste_core::Replacement) -> bool {
    use deckmaste_core::Replacement;
    matches!(r, Replacement::Also { .. })
}
