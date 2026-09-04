//! Per-card lowering diagnostics ([Core is explicit regions] law 12:
//! "resolution happens once, in lowering: … R1/R2 as per-card diagnostics").
//!
//! An unresolvable anaphor is a COMPILATION error carrying provenance, not an
//! eval-time refusal and not an anonymous panic from inside the tree walk. The
//! fixture here is a deliberately ambiguous card: a corpus compiler must be
//! able to name it and keep going.

use deckmaste_semantics as sem;

/// A card face carrying `abilities`, with everything else at its default.
fn face(name: &str, abilities: Vec<sem::Ability>) -> sem::CardFace {
    sem::CardFace {
        name: name.into(),
        types: vec![sem::TypeDef {
            name: "Instant".into(),
            permanent: false,
            confers: [].into(),
        }],
        abilities,
        ..sem::CardFace::default()
    }
}

/// "~ deals 2 damage to target opponent. ~ deals 5 damage to that opponent.
/// You gain that much life." — two magnitudes pinned by the same region, then
/// a bare "that much". The resolver refuses (R2); the rules supply no
/// proximity tiebreak ([CR#608.2c]).
fn ambiguous_card(name: &str) -> sem::Card {
    let damage = |amount| {
        sem::OneShotEffect::Act(sem::Action::DealDamage(
            sem::Reference::This,
            sem::Count::Literal(amount),
            sem::Reference::Opponent,
        ))
    };
    sem::Card::Normal(face(
        name,
        vec![sem::Ability::Spell(
            sem::SpellAbility {
                ability_word: None,
                effect: sem::OneShotEffect::Sequentially(
                    [
                        damage(2),
                        damage(5),
                        sem::OneShotEffect::Act(sem::Action::ChangeLife(
                            sem::Reference::You,
                            sem::LifeOp::Up(sem::Count::ThatMuch),
                        )),
                    ]
                    .into(),
                ),
            }
            .into(),
        )],
    ))
}

/// FIXTURE — the ambiguity is reported AGAINST THE CARD. `lower_card` returns
/// a `Diagnostic` naming the card and the reason; it does not unwind, so a
/// corpus compiler can attribute the failure and continue.
#[test]
fn an_ambiguous_card_produces_a_diagnostic_naming_the_card() {
    let error = deckmaste_lowering::lower_card(ambiguous_card("Ambiguous Anaphor Test"))
        .expect_err("a bare \"that much\" after two pinned magnitudes cannot be compiled");
    assert_eq!(&*error.card, "Ambiguous Anaphor Test");
    assert!(
        error.message.starts_with("Ambiguous Anaphor Test: "),
        "the diagnostic leads with the card it is about, got {:?}",
        error.message
    );
    assert!(
        error.message.contains("ambiguous discourse anaphor"),
        "the diagnostic states the refusal, got {:?}",
        error.message
    );
    // Provenance (ADR law 12): the diagnostic names the competing registers,
    // so an author can see WHICH two magnitudes collided.
    assert!(
        error.message.contains("antecedents are in scope"),
        "the diagnostic carries the competing antecedents, got {:?}",
        error.message
    );
}

/// The refusal is per-CARD, not per-corpus: the card that follows a bad one
/// still compiles, and its own name never leaks into the earlier diagnostic.
#[test]
fn a_refusal_does_not_poison_the_next_card() {
    let bad = deckmaste_lowering::lower_card(ambiguous_card("Bad Card")).unwrap_err();
    assert_eq!(&*bad.card, "Bad Card");

    let good = deckmaste_lowering::lower_card(sem::Card::Normal(face("Good Card", Vec::new())))
        .expect("a card with no anaphors compiles");
    let deckmaste_card::Card::Normal(lowered) = good else {
        panic!("a Normal card lowers to a Normal card");
    };
    assert_eq!(&*lowered.characteristics.name, "Good Card");
}

/// Only explicit resolver refusals use the diagnostic channel. An invariant
/// panic is still a panic, so `lower_card` cannot misreport a compiler bug as
/// a problem with the card being walked.
#[test]
#[should_panic(expected = "unbound role `missing` during semantic lowering")]
fn an_invariant_panic_is_not_rewritten_as_a_card_diagnostic() {
    let card = sem::Card::Normal(face(
        "Innocent Card",
        vec![sem::Ability::Spell(
            sem::SpellAbility {
                ability_word: None,
                effect: sem::OneShotEffect::Act(sem::Action::Move(
                    sem::Reference::Bound("missing".into()),
                    sem::Destination::Zone(sem::Zone::Exile),
                    [].into(),
                    None,
                )),
            }
            .into(),
        )],
    ));
    let _ = deckmaste_lowering::lower_card(card);
}

/// A card that compiles returns its lowered image, so `lower_card` is the
/// ordinary entry and not just an error probe.
#[test]
fn a_resolvable_card_lowers_through_the_diagnostic_entry() {
    let card = sem::Card::Normal(face(
        "Resolvable Card",
        vec![sem::Ability::Spell(
            sem::SpellAbility {
                ability_word: None,
                effect: sem::OneShotEffect::Act(sem::Action::DealDamage(
                    sem::Reference::This,
                    sem::Count::Literal(3),
                    sem::Reference::Opponent,
                )),
            }
            .into(),
        )],
    ));
    let lowered = deckmaste_lowering::lower_card(card).expect("one magnitude, no anaphor");
    let deckmaste_card::Card::Normal(face) = lowered else {
        panic!("a Normal card lowers to a Normal card");
    };
    assert_eq!(face.characteristics.abilities.len(), 1);
}

/// A bare `Source` reference is a damage-history query with no register to
/// read, so it is a per-card refusal rather than a lowering. Successor to the
/// retired `lowers_reference_source`, which asserted the identity lowering that
/// [Core is explicit regions] law 4 removed: the outcome changed from "lowers"
/// to "refuses", and this pins the new one.
#[test]
fn a_bare_source_reference_produces_a_diagnostic_naming_the_card() {
    let card = sem::Card::Normal(face(
        "Bare Source Test",
        vec![sem::Ability::Spell(
            sem::SpellAbility {
                ability_word: None,
                effect: sem::OneShotEffect::Act(sem::Action::DealDamage(
                    sem::Reference::Source,
                    sem::Count::Literal(1),
                    sem::Reference::Opponent,
                )),
            }
            .into(),
        )],
    ));

    let error = deckmaste_lowering::lower_card(card)
        .expect_err("a damage-source history read outside a condition cannot be compiled");
    assert_eq!(&*error.card, "Bare Source Test");
    assert!(
        error.message.contains("DealtDamageBy"),
        "the diagnostic names the condition that DOES admit the query, got {:?}",
        error.message
    );
}
