//! The attachment subtypes carry their attachment RULES as conferred `Innate`
//! data (spec conferral map), not as engine branches:
//!   - **Aura** confers `Innate(Static([Sba(Not(LegallyAttached(This)),
//!     Move(This, Graveyard))]))` ([CR#704.5m]).
//!   - **Equipment** confers `Innate(Static([May(Attach(This, Creature))]))`
//!     ([CR#301.5]) — under default-deny, being attachable to a creature is a
//!     granted capability.
//!   - **Fortification** confers `Innate(Static([May(Attach(This, Land))]))`
//!     ([CR#301.6]) — the granted attachable-to-a-land capability.
//!
//! These ride `Innate` so they survive "loses all abilities" and stay invisible
//! to card-facing ability queries, while the SBA sweep / `attachment_legal`
//! read them generically.

use std::path::Path;

use deckmaste_core::Ability;
use deckmaste_core::Condition;
use deckmaste_core::DeonticAction;
use deckmaste_core::Ident;
use deckmaste_core::Property;
use deckmaste_core::StaticEffect;
use deckmaste_core::Subtype;
use deckmaste_plugin::plugin::Plugin;

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// The single conferred ability of `subtype`, asserting it is exactly one
/// `Property::Ability(...)` and returning the (peeled-`Innate`) inner ability —
/// and asserting it WAS `Innate`.
fn sole_innate_ability(subtype: &Subtype) -> Ability {
    assert_eq!(
        subtype.confers.len(),
        1,
        "{} confers exactly one property; got {:?}",
        subtype.name,
        subtype.confers
    );
    let Property::Ability(a) = &subtype.confers[0] else {
        panic!(
            "{} confers an Ability property; got {:?}",
            subtype.name, subtype.confers[0]
        );
    };
    assert!(
        a.is_innate(),
        "{}'s conferred attachment rule is Innate (survives LoseAllAbilities); got {a:?}",
        subtype.name
    );
    a.peel_innate().clone()
}

/// Every static effect directly carried by a lowered ability, or empty.
fn static_effects(a: &Ability) -> Vec<StaticEffect> {
    match a {
        Ability::Static(s) => vec![s.body.clone()],
        _ => vec![],
    }
}

#[test]
fn aura_subtype_confers_innate_graveyard_sba() {
    let plugin = canon();
    let aura = plugin
        .subtypes
        .get(&Ident::from("Aura"))
        .expect("canon defines the Aura subtype");
    let inner = sole_innate_ability(aura);
    let effs = static_effects(&inner);
    // [CR#704.5m]: the must-be-attached graveyard SBA.
    assert!(
        effs.iter().any(|e| matches!(e,
            StaticEffect::Sba { when, .. }
                if matches!(&**when, Condition::Not(b) if matches!(**b, Condition::LegallyAttached(_))))),
        "Aura confers Sba(Not(LegallyAttached(This)), Move(This, Graveyard)) ([CR#704.5m]); got {effs:?}"
    );
}

#[test]
fn equipment_subtype_confers_innate_may_attach_creature() {
    let plugin = canon();
    let equipment = plugin
        .subtypes
        .get(&Ident::from("Equipment"))
        .expect("canon defines the Equipment subtype");
    let inner = sole_innate_ability(equipment);
    let effs = static_effects(&inner);
    // [CR#301.5]: Equipment may (only) be attached to a creature — a grant.
    assert!(
        effs.iter().any(|e| matches!(e,
            StaticEffect::Deontic(d) if matches!(d, deckmaste_core::Deontic::May(
                DeonticAction::Attach { .. })))),
        "Equipment confers May(Attach(This, Creature)) ([CR#301.5]); got {effs:?}"
    );
}

#[test]
fn fortification_subtype_confers_innate_may_attach_land() {
    let plugin = canon();
    let fort = plugin
        .subtypes
        .get(&Ident::from("Fortification"))
        .expect("canon defines the Fortification subtype");
    let inner = sole_innate_ability(fort);
    let effs = static_effects(&inner);
    // [CR#301.6]: Fortification may (only) be attached to a land — a grant.
    assert!(
        effs.iter().any(|e| matches!(e,
            StaticEffect::Deontic(d) if matches!(d, deckmaste_core::Deontic::May(
                DeonticAction::Attach { .. })))),
        "Fortification confers May(Attach(This, Land)) ([CR#301.6]); got {effs:?}"
    );
}
