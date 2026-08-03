//! Ability subterm enumeration.
//!
//! Provenance recovery is value-keyed: a core ability is looked up by its own
//! value to find the authored term it compiled from. That only works if every
//! authored ability the engine can put on an object was indexed, including the
//! ones buried inside other abilities — a layer-6 `GainAbility` payload
//! ([CR#613.1f]) is pushed onto an object as a verbatim clone, so it must be a
//! key in its own right.
//!
//! [`AbilitySubterms`] is hand-written, not derived: a generic `Subterms<T>`
//! would need a blanket `impl<T> Subterms<T> for T`, which overlaps every other
//! impl and stable Rust rejects. Being hand-written it is also incomplete by
//! construction — a grammar position nobody taught it about is silently not
//! traversed. That is why
//! `walker_reaches_every_gain_ability_in_the_canon_corpus`
//! (`deckmaste_plugin/tests/provenance.rs`) matches every `GainAbility` payload
//! in a `Debug` rendering of the whole corpus against the abilities the walker
//! actually reached: the oracle does not share the walker's idea of which
//! positions exist, so a missed position fails a test instead of quietly
//! degrading prose.

use std::sync::Arc;

use crate::Ability;

/// Pushes every [`Ability`] reachable from this value, stopping AT each one.
///
/// "Stopping at" is the contract that makes one level of nesting meaningful:
/// reaching an `Ability` pushes it and does not descend into it, so a caller
/// that wants the whole tree recurses on what it gets back.
/// [`Ability::nested_abilities`] is that one level;
/// [`ProvenanceIndex`](../../deckmaste_plugin/provenance) recurses.
pub trait AbilitySubterms {
    /// Appends this value's ability subterms to `out`, in pre-order.
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>);
}

impl<T: AbilitySubterms + ?Sized> AbilitySubterms for Arc<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        (**self).push_abilities(out);
    }
}

impl<T: AbilitySubterms> AbilitySubterms for [T] {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        for x in self {
            x.push_abilities(out);
        }
    }
}

impl<T: AbilitySubterms> AbilitySubterms for Vec<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        self.as_slice().push_abilities(out);
    }
}

impl<T: AbilitySubterms> AbilitySubterms for Option<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        if let Some(x) = self {
            x.push_abilities(out);
        }
    }
}

impl AbilitySubterms for Ability {
    /// The stopping point: push, do not descend. Descending is
    /// [`Ability::nested_abilities`]'s job, one level at a time.
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        out.push(self);
    }
}

impl AbilitySubterms for crate::Card {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Normal(f) => f.push_abilities(out),
            Self::TwoFaced { front, back, .. } => {
                front.push_abilities(out);
                back.push_abilities(out);
            }
        }
    }
}

impl AbilitySubterms for crate::CardFace {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        self.abilities.push_abilities(out);
    }
}

impl AbilitySubterms for crate::KeywordAbility {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Composite { abilities, .. } => abilities.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            Self::FirstStrike
            | Self::DoubleStrike
            | Self::Deathtouch
            | Self::Trample
            | Self::Vigilance => {}
        }
    }
}

impl AbilitySubterms for crate::StaticEffect {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Modify(_, m) => m.push_abilities(out),
            Self::Each(_, e) | Self::Conditionally(_, e) => e.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            // "X becomes a copy of Y" ([CR#707.4]) — a copy delivery site;
            // see `AbilitySubterms for CopySpec`.
            Self::BecomesCopy(_, spec) => spec.push_abilities(out),
            // No other `StaticEffect` shape carries an `Ability`. Spelled as a
            // catch-all rather than an exhaustive list because this enum grows;
            // the corpus oracle is what catches a new ability-bearing variant.
            _ => {}
        }
    }
}

impl AbilitySubterms for crate::Modification {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        // Exhaustive on purpose: this is THE layer-6 grant position
        // ([CR#613.1f]), the one the provenance index exists to catch. A new
        // variant here must be a compile error, not an oracle failure.
        match self {
            Self::GainAbility(a) => a.push_abilities(out),
            Self::Several(ms) => ms.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            Self::Power(_)
            | Self::Toughness(_)
            | Self::SwitchPowerToughness
            | Self::Colors(_)
            | Self::CardTypes(_)
            | Self::Subtypes(_)
            | Self::Supertypes(_)
            | Self::LoseAbility(_)
            | Self::LoseAllAbilities
            | Self::CantHaveAbility(_)
            | Self::SetController(_)
            | Self::SetText(_)
            | Self::AllCreatureTypes
            | Self::BaseLoyalty(_)
            | Self::BaseDefense(_)
            | Self::BecomeBasicLandType(_) => {}
        }
    }
}

impl AbilitySubterms for crate::OneShotEffect {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Act(a) => a.push_abilities(out),
            Self::Sequentially(es) | Self::Simultaneously(es) => es.push_abilities(out),
            Self::Continuously(c) => c.effect.push_abilities(out),
            Self::Until(_, es) => es.push_abilities(out),
            Self::Label(l) => l.effect.push_abilities(out),
            Self::SeparatePiles(s) => s.then.push_abilities(out),
            Self::ChoosePile(c) => c.then.push_abilities(out),
            Self::May(m) => {
                m.effect.push_abilities(out);
                m.if_did.push_abilities(out);
                m.if_not.push_abilities(out);
            }
            Self::If(i) => {
                i.then.push_abilities(out);
                i.otherwise.push_abilities(out);
            }
            Self::AdditionalCost(a) => a.body.push_abilities(out),
            Self::Each(e) => e.effect.push_abilities(out),
            Self::With(w) => w.body.push_abilities(out),
            Self::Distribute(d) => d.body.push_abilities(out),
            Self::Noting(n) => n.effect.push_abilities(out),
            Self::Delayed(t) | Self::Reflexive(t) => t.effect.push_abilities(out),
            Self::Modal(m) => {
                for mode in m.modes.iter() {
                    mode.effect.push_abilities(out);
                }
            }
            Self::Targeted(t) => t.effect.push_abilities(out),
            Self::Repeat(_, e) | Self::Batch(_, e) => e.push_abilities(out),
            Self::RevealUntil(r) => r.body.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
        }
    }
}

impl AbilitySubterms for crate::Action {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::GetEmblem(_, abilities) => abilities.push_abilities(out),
            // A created token's abilities are the token's own ([CR#111.3]) and
            // reach an object verbatim through `Cards::push_token`, so they are
            // a grant position like any other. `riders` can carry
            // `EnterRider::AsCopy`, a fourth copy delivery site with its own
            // `CopySpec` — see `AbilitySubterms for CopySpec`.
            Self::Create { token, riders, .. } => {
                token.push_abilities(out);
                riders.push_abilities(out);
            }
            // "`agent` casts a copy of `spec`" ([CR#707.12]) — a copy delivery
            // site; see `AbilitySubterms for CopySpec`.
            Self::CastCopy(_, spec) => spec.push_abilities(out),
            // As with `StaticEffect`: 44 variants, few of them ability-bearing,
            // and the list churns. The corpus oracle catches any ability-bearing
            // variant the canon corpus actually exercises — a position no canon
            // card reaches is caught by nothing, which is why the positions
            // that DO bear abilities are named above rather than left to it.
            _ => {}
        }
    }
}

impl AbilitySubterms for crate::CopySpec {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        self.exceptions.push_abilities(out);
    }
}

impl AbilitySubterms for crate::CopyException {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        // Exhaustive on purpose, like `Modification`: `Modify` is the
        // [CR#707.9a] "except it has [ability]" clause, a `GainAbility`
        // payload that lives ONLY inside a `CopySpec` (see the module-level
        // reasoning at `AbilitySubterms for TokenSpec`'s call site) — a new
        // variant here must be a compile error, not an oracle failure.
        match self {
            Self::Modify(m) => m.push_abilities(out),
            Self::AdditionalEffect(rider) => rider.push_abilities(out),
            Self::Retain(_) => {}
        }
    }
}

impl AbilitySubterms for crate::EnterRider {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            // The fourth copy delivery site ([CR#707.5]); see
            // `AbilitySubterms for CopySpec`.
            Self::AsCopy(spec) => spec.push_abilities(out),
            Self::Tapped
            | Self::FaceDown
            | Self::UnderControlOf(_)
            | Self::UnderOwnersControl
            | Self::Attacking(_)
            | Self::WithCounters(..) => {}
        }
    }
}

impl AbilitySubterms for crate::TokenSpec {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Token(t) => t.abilities.push_abilities(out),
            // A predefined token ([CR#111.10]) resolves to an OWNED `Token`
            // built on demand, so `ProvenanceIndex::insert_predefined_tokens`
            // indexes that closed set from the definitions themselves.
            Self::Named(_) => {}
            // A copy token's COPIABLE characteristics are the copied
            // object's ([CR#707.2]) and are indexed wherever that object's
            // own abilities were — but a copy EXCEPTION can itself carry a
            // `GainAbility` payload ([CR#707.9a], "except it has [ability]")
            // that exists ONLY inside this `CopySpec`; the engine pushes it
            // onto the object verbatim
            // (`deckmaste_engine::copy::apply_modification`), the same
            // verbatim-clone shape as the layer-6 grant `Modification`
            // already covers above. It must be indexed from here or nothing
            // ever reaches it.
            Self::Copy(spec) => spec.push_abilities(out),
        }
    }
}

impl AbilitySubterms for crate::Property {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a Ability>) {
        match self {
            Self::Ability(a) => a.push_abilities(out),
            // The keyword-counter grant position ([CR#122.1b,613.1f]): the
            // layer pass folds this `Modification` out of the counter registry
            // and pushes a `GainAbility` payload VERBATIM — unwrapped, unlike
            // the `Innate` form `Property::conferred_ability` emits.
            Self::Continuous(_, m) => m.push_abilities(out),
            // Effect flavors, not conferral: they execute in the [CR#704.3]
            // sweep / as turn-based actions, and confer nothing on their
            // bearer.
            Self::StateBased { .. } | Self::TurnBased { .. } => {}
        }
    }
}

impl Ability {
    /// Every `Ability` nested one level inside this one, in pre-order.
    ///
    /// "One level" means: reachable without passing through another `Ability`.
    /// A grant inside a grant is found by recursing on the result.
    ///
    /// Both `Innate` and `Expanded` are look-through wrappers whose payload
    /// lowers to the same core value the wrapper does, so a value-keyed index
    /// that inserts pre-order and never overwrites keeps the OUTER spelling —
    /// which is the one carrying the invocation template.
    #[must_use]
    pub fn nested_abilities(&self) -> Vec<&Ability> {
        let mut out = Vec::new();
        match self {
            Self::Static(e) => e.push_abilities(&mut out),
            Self::Activated(a) => a.effect.push_abilities(&mut out),
            Self::Triggered(a) => a.effect.push_abilities(&mut out),
            Self::Spell(a) => a.effect.push_abilities(&mut out),
            Self::Keyword(k) => k.push_abilities(&mut out),
            Self::Innate(a) => out.push(a),
            Self::Expanded(e) => out.push(&e.value),
        }
        out
    }
}
