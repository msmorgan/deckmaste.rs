//! The attachment subtypes carry their attachment RULES as conferred
//! ABILITY-FREE data, not as engine branches:
//!   - **Aura** confers `StateBased(Not(LegallyAttached(This)), Move(This,
//!     Graveyard))` ([CR#704.5m]) — a [CR#704] game action is not an ability
//!     ([CR#704.1a]).
//!   - **Equipment** confers `Static(May(Attach(This, Creature)))`
//!     ([CR#301.5]) — under default-deny, being attachable to a creature is a
//!     granted capability, and [CR#113.12] makes stating that quality no
//!     ability at all.
//!   - **Fortification** confers `Static(May(Attach(This, Land)))`
//!     ([CR#301.6]) — the granted attachable-to-a-land capability.
//!
//! Because none of them is an ability, they cannot be reached by "loses all
//! abilities" and never appear in a card-facing ability list, while the SBA
//! sweep / `attachment_legal` read them generically.

use std::path::Path;

use deckmaste_core::Condition;
use deckmaste_core::DeonticAction;
use deckmaste_core::Ident;
use deckmaste_core::Property;
use deckmaste_core::StaticSpec;
use deckmaste_core::Subtype;
use deckmaste_plugin::plugin::Plugin;

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// The single conferred property of `subtype`.
fn sole_confer(subtype: &Subtype) -> &Property {
    assert_eq!(
        subtype.confers.len(),
        1,
        "{} confers exactly one property; got {:?}",
        subtype.name,
        subtype.confers
    );
    &subtype.confers[0]
}

/// The static effect of an ABILITY-FREE `Property::Static` conferral — the
/// structural home of a type rule ([CR#113.12]).
fn sole_static_rule(subtype: &Subtype) -> StaticSpec {
    let Property::Static(region) = sole_confer(subtype) else {
        panic!(
            "{} confers an ability-free Static rule; got {:?}",
            subtype.name,
            sole_confer(subtype)
        );
    };
    region.body.clone()
}

#[test]
fn aura_subtype_confers_state_based_graveyard_rule() {
    let plugin = canon();
    let aura = plugin
        .subtypes
        .get(&Ident::from("Aura"))
        .expect("canon defines the Aura subtype");
    // [CR#704.5m]: the must-be-attached graveyard rule, ability-free.
    let confer = sole_confer(aura);
    assert!(
        matches!(confer, Property::StateBased { condition, .. }
            if matches!(&**condition, Condition::Not(b) if matches!(**b, Condition::LegallyAttached(_)))),
        "Aura confers StateBased(Not(LegallyAttached(This)), Move(This, Graveyard)) \
         ([CR#704.5m]); got {confer:?}"
    );
}

#[test]
fn equipment_subtype_confers_may_attach_creature() {
    let plugin = canon();
    let equipment = plugin
        .subtypes
        .get(&Ident::from("Equipment"))
        .expect("canon defines the Equipment subtype");
    let rule = sole_static_rule(equipment);
    // [CR#301.5]: Equipment may (only) be attached to a creature — a grant.
    assert!(
        matches!(
            &rule,
            StaticSpec::Deontic(deckmaste_core::Deontic::May(DeonticAction::Attach { .. }))
        ),
        "Equipment confers May(Attach(This, Creature)) ([CR#301.5]); got {rule:?}"
    );
}

#[test]
fn fortification_subtype_confers_may_attach_land() {
    let plugin = canon();
    let fort = plugin
        .subtypes
        .get(&Ident::from("Fortification"))
        .expect("canon defines the Fortification subtype");
    let rule = sole_static_rule(fort);
    // [CR#301.6]: Fortification may (only) be attached to a land — a grant.
    assert!(
        matches!(
            &rule,
            StaticSpec::Deontic(deckmaste_core::Deontic::May(DeonticAction::Attach { .. }))
        ),
        "Fortification confers May(Attach(This, Land)) ([CR#301.6]); got {rule:?}"
    );
}
