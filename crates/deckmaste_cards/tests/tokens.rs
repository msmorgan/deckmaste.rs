//! Integration tests for the three builtin token files: Treasure, Clue, Food.
//!
//! Each token is loaded via `Plugin::token(name)` (macro-aware reader) and
//! compared to the expected Rust value. A validate-level assertion confirms
//! that `validate_plugin` reports zero failures when tokens are included.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{
    Ability, Action, ActivatedAbility, CostComponent, Effect, ManaCost, ManaSpec, ManaSymbol,
    Reference, Selection, SimpleManaSymbol, Subtype, Token, Type,
};

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

/// `Mana([Generic(2)])` cost component.
fn mana_2() -> CostComponent {
    CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
        SimpleManaSymbol::Generic(2),
    )]))
}

/// `Do(Sacrifice(That(This)))` — the expanded form of `SacrificeThis`.
fn sacrifice_this() -> CostComponent {
    CostComponent::Do(Action::Sacrifice(Selection::That(Reference::This)))
}

fn artifact_subtype(name: &str) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Artifact],
    }
}

// CR 111.10a
#[test]
fn treasure_token_parses() {
    let token = builtin().token("Treasure").unwrap();
    assert_eq!(
        token,
        Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![artifact_subtype("Treasure")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![CostComponent::Tap, sacrifice_this()],
                targets: vec![],
                effect: Effect::Act(Action::AddMana(1, ManaSpec::AnyColor)),
            })],
        }
    );
}

// CR 111.10f
#[test]
fn clue_token_parses() {
    let token = builtin().token("Clue").unwrap();
    assert_eq!(
        token,
        Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![artifact_subtype("Clue")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![mana_2(), sacrifice_this()],
                targets: vec![],
                effect: Effect::Act(Action::DrawCards(1)),
            })],
        }
    );
}

// CR 111.10b
#[test]
fn food_token_parses() {
    let token = builtin().token("Food").unwrap();
    assert_eq!(
        token,
        Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![artifact_subtype("Food")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![mana_2(), CostComponent::Tap, sacrifice_this()],
                targets: vec![],
                effect: Effect::Act(Action::GainLife(3)),
            })],
        }
    );
}

/// `validate_plugin` on the builtin directory must report zero parse failures
/// and zero lint failures with the token files included.
#[test]
fn validate_builtin_with_tokens_has_no_failures() {
    let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
    let validation = deckmaste_cards::validate::validate_plugin(&builtin).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(
        validation.failures.is_empty(),
        "{} parse failure(s)",
        validation.failures.len()
    );
    assert!(
        validation.lint_failures.is_empty(),
        "{} lint failure(s)",
        validation.lint_failures.len()
    );
    // 7 cards + 3 tokens = 10 minimum.
    assert!(
        validation.valid >= 10,
        "only {} items validated",
        validation.valid
    );
}
