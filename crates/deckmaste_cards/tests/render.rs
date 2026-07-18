use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::render::CardView;
use deckmaste_cards::render::RenderedCard;
use deckmaste_cards::render::render;
use deckmaste_cards::render::render_card_face;
use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::KeywordAbility;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::Type;

fn canon_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon")
}

fn builtin_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}

fn testing_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing")
}

fn testing_face(name: &str) -> CardFace {
    let plugin = Plugin::load_with_sibling_prelude(testing_path()).unwrap();
    match plugin.card(name).unwrap() {
        Card::Normal(f) => f,
        other @ Card::TwoFaced { .. } => panic!("expected a normal card, got {other:?}"),
    }
}

fn face(name: &str) -> CardFace {
    let plugin = Plugin::load_with_sibling_prelude(canon_path()).unwrap();
    match plugin.card(name).unwrap() {
        Card::Normal(f) => f,
        other @ Card::TwoFaced { .. } => panic!("expected a normal card, got {other:?}"),
    }
}

#[test]
fn renders_vanilla_grizzly_bears() {
    let r = render_card_face(&face("Grizzly Bears"));
    assert_eq!(
        r,
        RenderedCard {
            name: "Grizzly Bears".to_string(),
            mana_cost: "{1}{G}".to_string(),
            type_line: "Creature — Bear".to_string(),
            rules: vec![],
            pt: Some("2/2".to_string()),
        }
    );
}

#[test]
fn renders_keyword_only_wall_of_stone() {
    let r = render_card_face(&face("Wall of Stone"));
    assert_eq!(r.type_line, "Creature — Wall");
    assert_eq!(r.pt, Some("0/8".to_string()));
    assert_eq!(r.rules, vec!["Defender".to_string()]);
}

#[test]
fn renders_spell_lightning_bolt() {
    let r = render_card_face(&face("Lightning Bolt"));
    assert_eq!(r.mana_cost, "{R}");
    assert_eq!(r.type_line, "Instant");
    assert_eq!(
        r.rules,
        vec!["Lightning Bolt deals 3 damage to any target.".to_string()]
    );
}

/// `MustPay(actor: ControllerOf(Target(0)), …)` renders the derived-player
/// reference as "its controller" ([CR#109.4]), not a debug marker — the
/// canonical Mana Leak punisher ([CR#118.12a]).
#[test]
fn renders_must_pay_controller_of_mana_leak() {
    let r = render_card_face(&face("Mana Leak"));
    assert_eq!(r.mana_cost, "{1}{U}");
    assert_eq!(r.type_line, "Instant");
    assert_eq!(
        r.rules,
        vec!["Counter target spell unless its controller pays {3}.".to_string()]
    );
}

/// `MustPay(actor: Coalesce([ControllerOf(Target(0)), Target(0)]), …)` — the
/// Rhystic-toll payer ([CR#118.12a]) derived from an `AnyTarget` slot
/// ([CR#115.4]) — renders the coalesced first-non-null payer as the single
/// player anaphor "that player" ([CR#109.4]), not a debug marker. The
/// `Reference::Coalesce` accept fixture (`Rhystic-toll DealDamage`).
#[test]
fn renders_must_pay_coalesce_rhystic_toll() {
    let r = render_card_face(&testing_face("Rhystic-toll DealDamage"));
    assert_eq!(r.mana_cost, "{2}{R}");
    assert_eq!(r.type_line, "Instant");
    assert_eq!(
        r.rules,
        vec![
            "Rhystic-toll DealDamage deals 3 damage to any target unless that player pays {2}."
                .to_string()
        ]
    );
}

/// A dynamic damage amount prints the oracle X-form with its "where X is …"
/// definition clause — the `where_x` adjunct survives to the render — and
/// the `ability_word` metadata prefixes the line ([CR#207.2c]).
#[test]
fn renders_domain_count_tribal_flames() {
    let r = render_card_face(&face("Tribal Flames"));
    assert_eq!(r.mana_cost, "{1}{R}");
    assert_eq!(r.type_line, "Sorcery");
    assert_eq!(
        r.rules,
        vec![
            "Domain — Tribal Flames deals X damage to any target, where X is the number \
             of basic land types among lands you control."
                .to_string()
        ]
    );
}

#[test]
fn renders_keywords_and_etb_trigger_baleful_strix() {
    let r = render_card_face(&face("Baleful Strix"));
    assert_eq!(r.mana_cost, "{U}{B}");
    assert_eq!(r.type_line, "Artifact Creature — Bird");
    assert_eq!(r.pt, Some("1/1".to_string()));
    assert_eq!(
        r.rules,
        vec![
            "Flying, deathtouch".to_string(),
            "When Baleful Strix enters, draw a card.".to_string(),
        ]
    );
}

#[test]
fn renders_state_trigger_goblin_medics() {
    let r = render_card_face(&face("Goblin Medics"));
    assert_eq!(
        r.rules,
        vec!["Whenever Goblin Medics becomes tapped, it deals 1 damage to any target.".to_string()]
    );
}

#[test]
fn renders_anthem_glorious_anthem() {
    let r = render_card_face(&face("Glorious Anthem"));
    assert_eq!(r.type_line, "Enchantment");
    assert_eq!(
        r.rules,
        vec!["Creatures you control get +1/+1.".to_string()]
    );
}

#[test]
fn renders_elesh_norn() {
    let r = render_card_face(&face("Elesh Norn, Grand Cenobite"));
    assert_eq!(r.type_line, "Legendary Creature — Phyrexian Praetor");
    assert_eq!(r.pt, Some("4/7".to_string()));
    assert_eq!(
        r.rules,
        vec![
            "Vigilance".to_string(),
            "Other creatures you control get +2/+2.".to_string(),
            "Creatures your opponents control get -2/-2.".to_string(),
        ]
    );
}

/// A `ModifyPlayer` continuous static over a player attribute: Exploration's
/// extra land play ([CR#305.2]).
#[test]
fn renders_modify_player_exploration() {
    let r = render_card_face(&face("Exploration"));
    assert_eq!(r.type_line, "Enchantment");
    assert_eq!(
        r.rules,
        vec!["You may play an additional land on each of your turns.".to_string()]
    );
}

/// Reliquary Tower's "no maximum hand size" cap removal ([CR#402.2]) plus its
/// `{T}: Add {C}` mana ability.
#[test]
fn renders_modify_player_reliquary_tower() {
    let r = render_card_face(&face("Reliquary Tower"));
    assert_eq!(r.type_line, "Land");
    assert_eq!(r.rules[0], "You have no maximum hand size.".to_string());
}

/// A `TriggerMultiplier` static: Panharmonicon's artifact/creature-ETB doubler
/// ([CR#603.2d]).
#[test]
fn renders_trigger_multiplier_panharmonicon() {
    let r = render_card_face(&face("Panharmonicon"));
    assert_eq!(r.type_line, "Artifact");
    assert_eq!(
        r.rules,
        vec![
            "If an artifact or creature entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.".to_string()
        ]
    );
}

/// The `Delve` keyword ([CR#702.66a]) — a `PayPips` static — renders to its
/// printed keyword name, and the non-cost "Draw three cards." effect renders
/// alongside it. No `[unrendered]` fallback markers leak (the `PayPips` static
/// must not surface raw debug text).
#[test]
fn renders_delve_treasure_cruise() {
    let r = render_card_face(&face("Treasure Cruise"));
    assert_eq!(r.mana_cost, "{7}{U}");
    assert_eq!(r.type_line, "Sorcery");
    assert_eq!(
        r.rules[0], "Delve",
        "the keyword line renders the printed name"
    );
    assert!(
        r.rules.iter().any(|line| line.contains("Draw")),
        "the draw effect renders: {:?}",
        r.rules
    );
    assert!(
        r.rules.iter().all(|line| !line.contains("[unrendered")),
        "no fallback markers leak: {:?}",
        r.rules
    );
}

#[test]
fn renders_must_attack_goblin_brigand() {
    let r = render_card_face(&face("Goblin Brigand"));
    assert_eq!(
        r.rules,
        vec!["Goblin Brigand attacks each combat if able.".to_string()]
    );
}

#[test]
fn renders_pacifism() {
    let r = render_card_face(&face("Pacifism"));
    assert_eq!(r.type_line, "Enchantment — Aura");
    // The adjacent can't-attack + can't-block pair merges to the printed
    // single clause.
    assert!(
        r.rules
            .contains(&"Enchanted creature can't attack or block.".to_string()),
        "rules: {:?}",
        r.rules
    );
    // No leaked fallback markers anywhere (the Enchant keyword line must render
    // too):
    assert!(
        r.rules.iter().all(|line| !line.contains("[unrendered")),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_sequence_brainstorm() {
    let r = render_card_face(&face("Brainstorm"));
    assert_eq!(r.mana_cost, "{U}");
    assert_eq!(
        r.rules,
        vec![
            "Draw three cards, then put two cards from your hand on top of your library in \
             any order."
                .to_string()
        ]
    );
}

// ── Derived / token path ─────────────────────────────────────────────────────

/// A synthesized token: no mana cost, no printed text, but a valid type line
/// and P/T from the runtime-assembled `CardView`.
#[test]
fn renders_a_synthesized_token() {
    let types = [Type::Creature.def()];
    let subs = [Subtype {
        name: "Goblin".into(),
        types: vec![Type::Creature, Type::Kindred],
        confers: vec![],
    }];
    let p = StatValue::Number(1);
    let t = StatValue::Number(1);
    let view = CardView {
        name: "Goblin",
        mana_cost: None,
        supertypes: &[],
        types: &types,
        subtypes: &subs,
        power: Some(&p),
        toughness: Some(&t),
        abilities: &[],
    };
    let r = render(&view);
    assert_eq!(r.mana_cost, "");
    assert_eq!(r.type_line, "Creature — Goblin");
    assert_eq!(r.pt, Some("1/1".to_string()));
    assert!(r.rules.is_empty());
}

/// A derived live object: base stats replaced by layer-applied values, a
/// keyword granted by a continuous effect present on the view.  No printed
/// mana cost (the view carries `None`).
#[test]
fn renders_a_derived_pumped_flier() {
    let types = [Type::Creature.def()];
    let p = StatValue::Number(4);
    let t = StatValue::Number(4);
    let fly = Ability::Keyword(KeywordAbility::Composite {
        name: "Flying".into(),
        abilities: vec![],
    });
    let abilities = [fly];
    let view = CardView {
        name: "Grizzly Bears",
        mana_cost: None,
        supertypes: &[],
        types: &types,
        subtypes: &[],
        power: Some(&p),
        toughness: Some(&t),
        abilities: &abilities,
    };
    let r = render(&view);
    assert_eq!(r.pt, Some("4/4".to_string()));
    assert_eq!(r.rules, vec!["Flying".to_string()]);
}

// ── Full-canon breadth sweep ─────────────────────────────────────────────────

fn canon_card_names() -> Vec<String> {
    std::fs::read_dir(canon_path().join("cards"))
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|e| {
            let p = e.path();
            (p.extension().and_then(|x| x.to_str()) == Some("ron"))
                .then(|| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .flatten()
        })
        .collect()
}

/// Totality sweep: the renderer must never panic on any canon card face.
/// Faces that use out-of-scope grammar will produce `[unrendered: …]` markers,
/// which is intentional — this test only asserts no panic and a non-empty type
/// line.  The `eprintln!` output shows the marker count when run with
/// `-- --nocapture`.
#[test]
fn renders_every_canon_card_without_panicking() {
    let plugin = Plugin::load_with_sibling_prelude(canon_path()).unwrap();
    let mut markers = 0usize;
    let mut total = 0usize;
    for name in canon_card_names() {
        let Ok(card) = plugin.card(&name) else { continue };
        let faces: Vec<CardFace> = match card {
            Card::Normal(f) => vec![f],
            Card::TwoFaced { front, back, .. } => vec![front, back],
        };
        for f in faces {
            let r = render_card_face(&f);
            assert!(!r.type_line.is_empty(), "{name}: empty type line");
            total += 1;
            if r.rules.iter().any(|l| l.contains("[unrendered")) {
                markers += 1;
            }
        }
    }
    eprintln!(
        "rendered {total} canon faces; {markers} still contain an [unrendered] marker (out-of-scope grammar)"
    );
    assert!(total > 0, "no canon cards found");
}

/// Anchor completeness: the cards exercised by the golden tests must
/// render with NO `[unrendered]` markers (they are in-scope by definition).
#[test]
fn anchor_cards_fully_rendered() {
    for name in [
        "Lightning Bolt",
        "Baleful Strix",
        "Glorious Anthem",
        "Elesh Norn, Grand Cenobite",
        "Goblin Brigand",
        "Brainstorm",
        "Wall of Stone",
        "Grizzly Bears",
        "Goblin Medics",
        "Pacifism",
        "Mana Leak",
    ] {
        let r = render_card_face(&face(name));
        let blob = format!("{} {} {:?}", r.mana_cost, r.type_line, r.rules);
        assert!(
            !blob.contains("[unrendered"),
            "{name} leaked a marker: {blob}"
        );
    }
}

// ── Coverage A: GainLife / LoseLife, Destroy, each-<noun> selections ─────────

#[test]
fn renders_gain_life_deepwood_tantiv() {
    let r = render_card_face(&face("Deepwood Tantiv"));
    // Whenever it becomes blocked, you gain 2 life.
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Whenever Deepwood Tantiv becomes blocked, you gain 2 life."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_damage_to_each_creature_pyroclasm() {
    let r = render_card_face(&face("Pyroclasm"));
    assert_eq!(
        r.rules,
        vec!["Pyroclasm deals 2 damage to each creature.".to_string()]
    );
}

#[test]
fn renders_damage_to_each_player_flame_rift() {
    let r = render_card_face(&face("Flame Rift"));
    assert_eq!(
        r.rules,
        vec!["Flame Rift deals 4 damage to each player.".to_string()]
    );
}

#[test]
fn renders_synthesized_lose_life_and_destroy() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::CardFace;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    // "Lose 3 life." spell
    let lose = CardFace {
        name: "Test Drain".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::LoseLife(Count::Literal(3)),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&lose).rules,
        vec!["You lose 3 life.".to_string()]
    );

    // "Destroy target creature." spell — the `Destroy(It)` verb renders via
    // its macro template (the provenance the migrations parser and corpus both
    // carry), so build it through the real plugin rather than a raw composite.
    let plugin = Plugin::load(builtin_path()).unwrap();
    let destroy_verb: OneShotEffect = plugin.macros.read_str("Destroy(Target(0))").unwrap();
    let destroy = CardFace {
        name: "Test Smite".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![TargetSpec::Target(Quantity::one(), Predicate::creature())],
                destroy_verb,
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&destroy).rules,
        vec!["Destroy target creature.".to_string()]
    );
}

/// A predefined token ([CR#111.10]) renders by its bare name — the
/// bidirectional truth the `create a <Name> token` parser routes back to.
#[test]
fn renders_named_predefined_token() {
    use deckmaste_core::Action;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::TokenName;
    use deckmaste_core::TokenSpec;

    let treasure = CardFace {
        name: "Test Hoard".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(deckmaste_core::SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(
                    Count::Literal(1),
                    TokenSpec::Named(TokenName::from("Treasure")),
                    vec![],
                ),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&treasure).rules,
        vec!["Create a Treasure token.".to_string()]
    );

    let two_food = CardFace {
        name: "Test Feast".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(deckmaste_core::SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(
                    Count::Literal(2),
                    TokenSpec::Named(TokenName::from("Food")),
                    vec![],
                ),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&two_food).rules,
        vec!["Create two Food tokens.".to_string()]
    );
}

// ── Coverage B: "dies" event + non-self trigger subjects ─────────────────────

#[test]
fn renders_dies_trigger_footlight_fiend() {
    let r = render_card_face(&face("Footlight Fiend"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "When Footlight Fiend dies, it deals 1 damage to any target."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_creature_dies_trigger_moonlit_wake() {
    let r = render_card_face(&face("Moonlit Wake"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Whenever a creature dies, you gain 1 life."),
        "rules: {:?}",
        r.rules
    );
}

// A qualified (self-excluded, controller-restricted) trigger subject must
// keep its qualifiers — "another creature you control", not the bare "a
// creature" a dropped filter would print.
#[test]
fn renders_filtered_subject_dies_trigger() {
    let r = render_card_face(&testing_face("Filtered Subject Triggers"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Whenever another creature you control dies, you gain 1 life."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_filtered_subject_ltb_trigger() {
    let r = render_card_face(&testing_face("Filtered Subject Triggers"));
    assert!(
        r.rules.iter().any(|l| l
            == "Whenever another creature you control leaves the battlefield, you gain 1 life."),
        "rules: {:?}",
        r.rules
    );
}

// ── Coverage C: bare Modify + full Modification vocabulary ──────────────────

#[test]
fn renders_set_colors_darkest_hour() {
    assert_eq!(
        render_card_face(&face("Darkest Hour")).rules,
        vec!["All creatures are black.".to_string()]
    );
}

#[test]
fn renders_gain_ability_serras_blessing() {
    assert_eq!(
        render_card_face(&face("Serra's Blessing")).rules,
        vec!["Creatures you control have vigilance.".to_string()]
    );
}

#[test]
fn renders_humility() {
    assert_eq!(
        render_card_face(&face("Humility")).rules,
        vec!["All creatures lose all abilities and have base power and toughness 1/1.".to_string()]
    );
}

#[test]
fn renders_scope_of_singular() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Count;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticEffect;
    let face = CardFace {
        name: "Test Aura".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Modify(
            Reference::This,
            Modification::Several(vec![
                Modification::Power(deckmaste_core::NumericOp::Up(Count::Literal(1))),
                Modification::Toughness(deckmaste_core::NumericOp::Up(Count::Literal(1))),
            ]),
        ))],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Test Aura gets +1/+1.".to_string()]
    );
}

/// An Aura whose static effect pumps its host: `Modify(AttachHostOf(This), …)`
/// → "Enchanted creature gets +2/+2."
#[test]
fn renders_aura_host_pump() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Count;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticEffect;
    let face = CardFace {
        name: "Test Buff Aura".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Modify(
            Reference::AttachHostOf(Arc::new(Reference::This)),
            Modification::Several(vec![
                Modification::Power(deckmaste_core::NumericOp::Up(Count::Literal(2))),
                Modification::Toughness(deckmaste_core::NumericOp::Up(Count::Literal(2))),
            ]),
        ))],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Enchanted creature gets +2/+2.".to_string()]
    );
}

// ── Coverage D: OneShotEffect::Continuously + Duration suffix
// ──────────────────────

/// Synthesized pump spell: "Target creature gets +3/+3 until end of turn."
/// Exercises `OneShotEffect::Continuously` → `static_effect` (inner) +
/// `duration_suffix(FixedUntil(EndOfTurn))`.
#[test]
fn renders_continuously_pump_until_eot() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Continuously;
    use deckmaste_core::Count;
    use deckmaste_core::Duration;
    use deckmaste_core::Modification;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::TurnMarker;
    use deckmaste_core::Type;
    let face = CardFace {
        name: "Test Pump".into(),
        types: vec![Type::Instant.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![TargetSpec::Target(Quantity::one(), Predicate::creature())],
                OneShotEffect::Continuously(Continuously {
                    effect: Arc::new(StaticEffect::Modify(
                        Reference::Target(0),
                        Modification::Several(vec![
                            Modification::Power(deckmaste_core::NumericOp::Up(Count::Literal(3))),
                            Modification::Toughness(deckmaste_core::NumericOp::Up(Count::Literal(
                                3,
                            ))),
                        ]),
                    )),
                    duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
                }),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Target creature gets +3/+3 until end of turn.".to_string()]
    );
}

/// Synthesized combat-restriction spell: "Target creature can't block this
/// turn." Exercises `OneShotEffect::Continuously` wrapping a `Deontic`
/// (`Cant(Block(by: …))`) — the "this turn." trailer (never "until end of
/// turn.", [CR#509.1b]) plus `deontic_subject`'s generalized `Ref(It)` ->
/// the announced target's phrase.
#[test]
fn renders_continuously_cant_block_eot() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Continuously;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Duration;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::TurnMarker;
    use deckmaste_core::Type;
    let face = CardFace {
        name: "Test Block Restriction".into(),
        types: vec![Type::Instant.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![TargetSpec::Target(Quantity::one(), Predicate::creature())],
                OneShotEffect::Continuously(Continuously {
                    effect: Arc::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
                        by: Predicate::Ref(Reference::Target(0)),
                        on: Predicate::Any,
                        count: None,
                    }))),
                    duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
                }),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Target creature can't block this turn.".to_string()]
    );
}

/// Synthesized evasion spell: "Target creature can't be blocked this turn."
/// The passive `on`-anchored `Cant(Block(...))` — new render arm
/// ([CR#509.1b]).
#[test]
fn renders_continuously_cant_be_blocked_eot() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Continuously;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Duration;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::TurnMarker;
    use deckmaste_core::Type;
    let face = CardFace {
        name: "Test Unblockable".into(),
        types: vec![Type::Instant.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![TargetSpec::Target(Quantity::one(), Predicate::creature())],
                OneShotEffect::Continuously(Continuously {
                    effect: Arc::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
                        by: Predicate::Any,
                        on: Predicate::Ref(Reference::Target(0)),
                        count: None,
                    }))),
                    duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
                }),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Target creature can't be blocked this turn.".to_string()]
    );
}

/// Regression ([CR#509.1b] + [CR#608.2d]): the `That(<Sort>)` anaphor as a
/// deontic sentence SUBJECT ("That creature can't block this turn.") — a
/// non-targeted restriction referencing a prior clause's creature. Guards the
/// `reference_subject` `Reference::That` arm; before it existed this rendered
/// "[unrendered: That(Creature)] can't block this turn.".
#[test]
fn renders_that_creature_cant_block_eot() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Continuously;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Duration;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Sort;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::TurnMarker;
    use deckmaste_core::Type;
    let face = CardFace {
        name: "Test That Restriction".into(),
        types: vec![Type::Instant.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Continuously(Continuously {
                effect: Arc::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
                    by: Predicate::Ref(Reference::That(Sort::OfType(Type::Creature))),
                    on: Predicate::Any,
                    count: None,
                }))),
                duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            }),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["That creature can't block this turn.".to_string()]
    );
}

#[test]
fn renders_create_one_token() {
    use deckmaste_core::Action;
    use deckmaste_core::Color;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StatValue;
    use deckmaste_core::Token;
    use deckmaste_core::TokenSpec;
    let token = Token {
        color_indicator: vec![Color::Red],
        supertypes: vec![],
        types: vec![Type::Creature.def()],
        subtypes: vec![Subtype {
            name: "Goblin".into(),
            types: vec![Type::Creature],
            confers: vec![],
        }],
        abilities: vec![],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
    };
    let face = CardFace {
        name: "Test Maker".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(Count::Literal(1), TokenSpec::Token(token), vec![]),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Create a 1/1 red Goblin creature token.".to_string()]
    );
}

#[test]
fn renders_create_two_tokens() {
    use deckmaste_core::Action;
    use deckmaste_core::Color;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::StatValue;
    use deckmaste_core::Token;
    use deckmaste_core::TokenSpec;
    let token = Token {
        color_indicator: vec![Color::White],
        supertypes: vec![],
        types: vec![Type::Creature.def()],
        subtypes: vec![Subtype {
            name: "Soldier".into(),
            types: vec![Type::Creature],
            confers: vec![],
        }],
        abilities: vec![],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
    };
    let face = CardFace {
        name: "Test Muster".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(Count::Literal(2), TokenSpec::Token(token), vec![]),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Create two 1/1 white Soldier creature tokens.".to_string()]
    );
}

#[test]
fn renders_enters_tapped_diregraf_ghoul() {
    let r = render_card_face(&face("Diregraf Ghoul"));
    assert!(
        r.rules.iter().any(|l| l == "Diregraf Ghoul enters tapped."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_kabira_crossroads() {
    let r = render_card_face(&face("Kabira Crossroads"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Kabira Crossroads enters tapped."),
        "rules: {:?}",
        r.rules
    );
    assert!(
        r.rules
            .iter()
            .any(|l| l == "When Kabira Crossroads enters, you gain 2 life."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_get_designation() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::CardFace;
    use deckmaste_core::Ident;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::Type;
    let face = CardFace {
        name: "Test Ascend".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GetDesignation(Ident::from("CitysBlessing")),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["You get the city's blessing.".to_string()]
    );
}

/// [CR#114.1]: "You get an emblem with «ability»." — the emblem's abilities
/// render through the same rules walk a card face uses, quoted as its text.
#[test]
fn renders_get_emblem() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::CardFace;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::Type;

    // The Glorious-Anthem static, parsed under the canon macro scope.
    let plugin = Plugin::load_with_sibling_prelude(canon_path()).unwrap();
    let emblem_ability: Ability = plugin
        .macros
        .read_str(
            "Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
             Modify(It, Several([Power(Up(1)), Toughness(Up(1))]))))",
        )
        .unwrap();
    let face = CardFace {
        name: "Test Emblem Granter".into(),
        types: vec![Type::Sorcery.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GetEmblem(vec![emblem_ability]),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        // No period outside the closing quote — the printed convention ends
        // the sentence with the quoted ability's own period.
        vec!["You get an emblem with \"Creatures you control get +1/+1.\"".to_string()]
    );
}

/// A graveyard-FUNCTIONING static ([CR#113.6,604.3]) renders its "As long as ~
/// is in your graveyard," function-zone qualifier (the incarnation cycle
/// shape).
#[test]
fn renders_graveyard_static_from_zone() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Modification;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Zone;
    let face = CardFace {
        name: "Test Incarnation".into(),
        types: vec![Type::Creature.def()],
        abilities: vec![Ability::r#static(StaticEffect::Conditionally(
            Condition::Matches(
                Reference::This,
                Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
            ),
            Arc::new(StaticEffect::Each(
                Selection::SelectAll(Predicate::And(vec![
                    Predicate::type_(Type::Creature),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ])),
                Arc::new(StaticEffect::Modify(
                    Reference::It,
                    Modification::Several(vec![
                        Modification::Power(deckmaste_core::NumericOp::Up(Count::Literal(1))),
                        Modification::Toughness(deckmaste_core::NumericOp::Up(Count::Literal(1))),
                    ]),
                )),
            )),
        ))],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec![
            "As long as Test Incarnation is in your graveyard, creatures you control get +1/+1."
                .to_string()
        ]
    );
}

/// A triggered ability with a `TurnOf` intervening-if ([CR#603.4]) renders the
/// "if it's …'s turn," clause between event and effect.
#[test]
fn renders_trigger_with_turnof_intervening_if() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Condition;
    use deckmaste_core::EventFilter;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::TriggeredAbility;
    use deckmaste_core::Zone;
    // `Draw(1)` is a slice-family verb macro (`Batch(1, Act(Composite(name:
    // Draw, …)))`) that renders via its template — build it through the real
    // plugin so it carries that `Expanded` provenance rather than an
    // unrendered raw `Batch`.
    let plugin = Plugin::load(builtin_path()).unwrap();
    let draw: OneShotEffect = plugin.macros.read_str("Draw(1)").unwrap();
    let face = CardFace {
        name: "Vigil Keeper".into(),
        types: vec![Type::Creature.def()],
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            event: EventFilter::ZoneChange {
                what: Predicate::type_(Type::Creature),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            },
            from: None,
            condition: Some(Condition::TurnOf(Predicate::Relation(
                RelationPredicate::OpponentOf(Arc::new(Predicate::Ref(Reference::You))),
            ))),
            limits: vec![],
            effect: draw,
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Whenever a creature dies, if it's an opponent's turn, draw a card.".to_string()]
    );
}

/// The `EntersWithCounters` macro ([CR#122.6a]) — an `Ability`-kind macro
/// invocation (`plugins/builtin/macros/replacement/EntersWithCounters.ron`)
/// parsed through the REAL builtin plugin, so this exercises both the
/// `Ability::Expanded` render-peeling ([`deckmaste_cards::render`]'s
/// `rules()`, which previously dropped an `Ability`-kind macro invocation
/// silently — no prior card exercised that path) and the dedicated
/// `Replacement::Also` render arm that prints the idiomatic "~ enters with
/// ... on it." form (singular "a +1/+1 counter", plural "two +1/+1
/// counters") rather than the generic "As ~ enters, put ... on it."
/// fallback.
#[test]
fn renders_enters_with_counters_p1p1_singular_and_plural() {
    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Type;

    let plugin = Plugin::load(builtin_path()).unwrap();

    let one: Ability = plugin
        .macros
        .read_str("EntersWithCounters(P1P1Counter, 1)")
        .unwrap();
    let face_one = CardFace {
        name: "Test Permanent".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![one],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face_one).rules,
        vec!["Test Permanent enters with a +1/+1 counter on it.".to_string()]
    );

    let two: Ability = plugin
        .macros
        .read_str("EntersWithCounters(P1P1Counter, 2)")
        .unwrap();
    let face_two = CardFace {
        name: "Test Permanent".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![two],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face_two).rules,
        vec!["Test Permanent enters with two +1/+1 counters on it.".to_string()]
    );
}

/// A named (non-pip) counter kind — the shield-counter shape ([CR#122.1c]) —
/// renders through the same `counter_phrase` noun derivation
/// (`ShieldCounter` -> "shield") as the pip kinds above.
#[test]
fn renders_enters_with_counters_named_kind() {
    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Type;

    let plugin = Plugin::load(builtin_path()).unwrap();

    let shield: Ability = plugin
        .macros
        .read_str("EntersWithCounters(ShieldCounter, 1)")
        .unwrap();
    let face = CardFace {
        name: "Test Permanent".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![shield],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Test Permanent enters with a shield counter on it.".to_string()]
    );

    let charge: Ability = plugin
        .macros
        .read_str("EntersWithCounters(ChargeCounter, 3)")
        .unwrap();
    let face3 = CardFace {
        name: "Test Permanent".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![charge],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face3).rules,
        vec!["Test Permanent enters with three charge counters on it.".to_string()]
    );
}

/// A Saga's chapter abilities render with their Roman-numeral chapter markers
/// ([CR#714.2a] — "I", "II", "III") and the em-dash lead, effect rendered
/// structurally.
#[test]
fn renders_saga_chapter_roman_markers() {
    let r = render_card_face(&testing_face("Test Saga"));
    assert!(
        r.rules.contains(&"I — You gain 1 life.".to_string()),
        "chapter I renders its Roman marker: {:?}",
        r.rules
    );
    assert!(
        r.rules.contains(&"II — You gain 2 life.".to_string()),
        "chapter II: {:?}",
        r.rules
    );
    assert!(
        r.rules.contains(&"III — You gain 3 life.".to_string()),
        "chapter III: {:?}",
        r.rules
    );
}

/// A chapter RANGE ([CR#714.2c]) renders its numbers joined — "II, III —".
#[test]
fn renders_saga_chapter_range_marker() {
    let r = render_card_face(&testing_face("Test Saga Range"));
    assert!(
        r.rules.contains(&"I — You gain 1 life.".to_string()),
        "single chapter still renders: {:?}",
        r.rules
    );
    assert!(
        r.rules.contains(&"II, III — You gain 2 life.".to_string()),
        "range renders both markers joined: {:?}",
        r.rules
    );
}
