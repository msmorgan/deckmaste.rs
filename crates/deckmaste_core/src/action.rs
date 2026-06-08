use std::fmt;

use serde::de::{self, Deserializer, EnumAccess, VariantAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::mana::ManaSpec;
use crate::{Count, Expansion, IdentSeed, Reference, Selection, Token};

/// An intrinsic game verb ([CR#700,701]) whose **agent is the source object or
/// the effect itself**, not a player: the source deals the damage, the effect
/// destroys/returns the object. Player-performed verbs (draw, sacrifice, …)
/// carry an explicit agent and live on [`PlayerAction`], reached through `By`.
///
/// Authored RON writes a bare [`PlayerAction`] in an effect slot (`Draw(1)`,
/// `Tap(This)`, `Sacrifice(This)`) and it reads as `By(Reference::You, …)` —
/// the implicit-you default, via the macro layer's `embeds_untagged` hook
/// (`Action` is registered with `.embeds_untagged()`). An explicit different
/// agent is written `By(Target(0), Draw(3))` (Ancestral Recall) and read
/// natively. Both `Serialize` and `Deserialize` are manual to carry that: the
/// reader tries `Action`'s own variants first, then falls through to a
/// `PlayerAction`; the writer emits a `By(You, …)` bare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    /// Deal an amount of damage to a selection ([CR#120.1]) — the source object
    /// is the agent.
    DealDamage(Selection, Count),
    /// Destroy a selected permanent ([CR#701.8]).
    Destroy(Selection),
    /// Return a selection to its owner's hand.
    ReturnToHand(Selection),
    /// A named player performs the [`PlayerAction`] ([CR#608.2]). `By(You, …)`
    /// is the implicit-you default and is written bare in RON.
    By(Reference, PlayerAction),
}

/// A verb a **player** performs, carrying an explicit agent via
/// [`Action::By`]. Authored bare in effect slots (the agent defaults to
/// `You`); a cost (`CostComponent::Do`) holds a bare `PlayerAction` whose
/// agent is implicitly the payer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum PlayerAction {
    /// Draw a number of cards ([CR#121.1]).
    Draw(Count),
    /// Discard a number of cards ([CR#701.9]).
    Discard(Count),
    /// Gain an amount of life ([CR#119.3]).
    GainLife(Count),
    /// Lose an amount of life — pay-life when in a cost ([CR#119.3]).
    LoseLife(Count),
    /// Add mana to the player's mana pool ([CR#106.4]).
    AddMana(Count, ManaSpec),
    /// Create a number of token permanents ([CR#111.1,701.7]).
    Create(Count, Token),
    /// Sacrifice a selected permanent ([CR#701.21]).
    Sacrifice(Selection),
    /// Exile a selection ([CR#701.13]).
    Exile(Selection),
    /// Tap a selection ([CR#701.26a]).
    Tap(Selection),
    /// Untap a selection ([CR#701.26b]).
    Untap(Selection),
    /// A remembered `PlayerAction` macro invocation.
    Expanded(Expansion<PlayerAction>),
}

impl PlayerAction {
    /// Whether this verb may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets ([CR#601.2b..601.2c]). Cost-eligible
    /// verbs are the self-directed ones a player can pay with — sacrifice,
    /// exile, tap, untap, discard, and pay-life (`LoseLife`).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        matches!(
            self,
            PlayerAction::Sacrifice(_)
                | PlayerAction::Exile(_)
                | PlayerAction::Tap(_)
                | PlayerAction::Untap(_)
                | PlayerAction::Discard(_)
                | PlayerAction::LoseLife(_)
        )
    }
}

/// `Action`'s own variant names — the ones the reader recognizes before
/// falling through to a [`PlayerAction`] (wrapped as `By(You, …)`). The macro
/// layer's `embeds_untagged` hook hands every other identifier-led value to
/// `visit_newtype_struct`.
const VARIANTS: &[&str] = &["DealDamage", "Destroy", "ReturnToHand", "By"];

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ActionVisitor;

        impl<'de> Visitor<'de> for ActionVisitor {
            type Value = Action;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an action or a player action")
            }

            /// The untagged-embed fall-through: an identifier the enum channel
            /// didn't recognize as one of `Action`'s own variants (or an
            /// `Action` macro) arrives here as newtype content. Reading a
            /// `PlayerAction` re-enters the macro layer under the
            /// `PlayerAction` namespace, so a bare `Draw(1)`/`Tap(This)` and
            /// `PlayerAction` macros alike read and wrap in `By(You, …)`.
            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                Ok(Action::By(
                    Reference::You,
                    PlayerAction::deserialize(deserializer)?,
                ))
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                use crate::de_util::Pair;

                let (ident, v) = data.variant_seed(IdentSeed)?;
                Ok(match ident.as_str() {
                    "DealDamage" => {
                        let (sel, n) = v.tuple_variant(2, Pair::<Selection, Count>::new())?;
                        Action::DealDamage(sel, n)
                    }
                    "Destroy" => Action::Destroy(v.newtype_variant()?),
                    "ReturnToHand" => Action::ReturnToHand(v.newtype_variant()?),
                    "By" => {
                        let (who, pa) =
                            v.tuple_variant(2, Pair::<Reference, PlayerAction>::new())?;
                        Action::By(who, pa)
                    }
                    // Not one of Action's own variants: wrap a bare player
                    // action in the implicit-you default. (Reached only without
                    // the macro layer's embed hook — e.g. a plain `ron` read;
                    // with the hook the fall-through arrives at
                    // `visit_newtype_struct` above.)
                    _ => Action::By(Reference::You, player_action_from(ident.as_str(), v)?),
                })
            }
        }

        deserializer.deserialize_enum("Action", VARIANTS, ActionVisitor)
    }
}

/// Builds a [`PlayerAction`] from an already-read variant tag and its access —
/// the fall-through path when no macro layer is present and the bare player
/// action arrives through `visit_enum`. Mirrors `PlayerAction`'s derived
/// shapes.
fn player_action_from<'de, A: VariantAccess<'de>>(
    ident: &str,
    v: A,
) -> Result<PlayerAction, A::Error> {
    use crate::de_util::Pair;

    Ok(match ident {
        "Draw" => PlayerAction::Draw(v.newtype_variant()?),
        "Discard" => PlayerAction::Discard(v.newtype_variant()?),
        "GainLife" => PlayerAction::GainLife(v.newtype_variant()?),
        "LoseLife" => PlayerAction::LoseLife(v.newtype_variant()?),
        "AddMana" => {
            let (n, spec) = v.tuple_variant(2, Pair::<Count, ManaSpec>::new())?;
            PlayerAction::AddMana(n, spec)
        }
        "Create" => {
            let (n, token) = v.tuple_variant(2, Pair::<Count, Token>::new())?;
            PlayerAction::Create(n, token)
        }
        "Sacrifice" => PlayerAction::Sacrifice(v.newtype_variant()?),
        "Exile" => PlayerAction::Exile(v.newtype_variant()?),
        "Tap" => PlayerAction::Tap(v.newtype_variant()?),
        "Untap" => PlayerAction::Untap(v.newtype_variant()?),
        "Expanded" => PlayerAction::Expanded(v.newtype_variant()?),
        other => {
            return Err(de::Error::custom(format_args!(
                "`{other}` is neither an Action variant nor a PlayerAction"
            )));
        }
    })
}

impl Serialize for Action {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON.
        match self {
            Action::DealDamage(sel, n) => {
                serializer.serialize_newtype_variant("Action", 0, "DealDamage", &(sel, n))
            }
            Action::Destroy(sel) => {
                serializer.serialize_newtype_variant("Action", 1, "Destroy", sel)
            }
            Action::ReturnToHand(sel) => {
                serializer.serialize_newtype_variant("Action", 2, "ReturnToHand", sel)
            }
            // `By(You, …)` is the implicit-you default: write the player action
            // bare so it round-trips through the embed. A different agent keeps
            // the explicit `By(other, …)` form.
            Action::By(Reference::You, pa) => pa.serialize(serializer),
            Action::By(other, pa) => {
                serializer.serialize_newtype_variant("Action", 3, "By", &(other, pa))
            }
        }
    }
}

impl Serialize for PlayerAction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON. Mirrors the
        // shapes the derive produced, plus the `Expanded` invocation arm.
        match self {
            PlayerAction::Draw(n) => {
                serializer.serialize_newtype_variant("PlayerAction", 0, "Draw", n)
            }
            PlayerAction::Discard(n) => {
                serializer.serialize_newtype_variant("PlayerAction", 1, "Discard", n)
            }
            PlayerAction::GainLife(n) => {
                serializer.serialize_newtype_variant("PlayerAction", 2, "GainLife", n)
            }
            PlayerAction::LoseLife(n) => {
                serializer.serialize_newtype_variant("PlayerAction", 3, "LoseLife", n)
            }
            PlayerAction::AddMana(n, spec) => {
                serializer.serialize_newtype_variant("PlayerAction", 4, "AddMana", &(n, spec))
            }
            PlayerAction::Create(n, token) => {
                serializer.serialize_newtype_variant("PlayerAction", 5, "Create", &(n, token))
            }
            PlayerAction::Sacrifice(sel) => {
                serializer.serialize_newtype_variant("PlayerAction", 6, "Sacrifice", sel)
            }
            PlayerAction::Exile(sel) => {
                serializer.serialize_newtype_variant("PlayerAction", 7, "Exile", sel)
            }
            PlayerAction::Tap(sel) => {
                serializer.serialize_newtype_variant("PlayerAction", 8, "Tap", sel)
            }
            PlayerAction::Untap(sel) => {
                serializer.serialize_newtype_variant("PlayerAction", 9, "Untap", sel)
            }
            // The invocation, not the struct.
            PlayerAction::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count::Count;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Action { crate::ron::options().from_str(source).unwrap() }
    fn write(action: &Action) -> String { crate::ron::options().to_string(action).unwrap() }

    #[test]
    fn is_cost_eligible_covers_self_directed_verbs() {
        assert!(PlayerAction::Sacrifice(Selection::Ref(Reference::This)).is_cost_eligible());
        assert!(PlayerAction::Exile(Selection::Ref(Reference::This)).is_cost_eligible());
        assert!(PlayerAction::Tap(Selection::Ref(Reference::This)).is_cost_eligible());
        assert!(PlayerAction::Untap(Selection::Ref(Reference::This)).is_cost_eligible());
        assert!(PlayerAction::Discard(Count::Literal(1)).is_cost_eligible());
        assert!(PlayerAction::LoseLife(Count::Literal(1)).is_cost_eligible());

        assert!(!PlayerAction::Draw(Count::Literal(1)).is_cost_eligible());
        assert!(!PlayerAction::GainLife(Count::Literal(3)).is_cost_eligible());
        assert!(!PlayerAction::AddMana(Count::Literal(1), ManaSpec::AnyColor).is_cost_eligible());
    }

    /// A bare player verb reads as `By(You, …)` — the implicit-you default
    /// through the plain `ron` `visit_enum` fall-through (no macro layer).
    #[test]
    fn bare_player_verb_defaults_to_you() {
        assert_eq!(
            read("Draw(Literal(1))"),
            Action::By(Reference::You, PlayerAction::Draw(Count::Literal(1))),
        );
        assert_eq!(
            read("Sacrifice(This)"),
            Action::By(
                Reference::You,
                PlayerAction::Sacrifice(Selection::Ref(Reference::This)),
            ),
        );
        assert_eq!(
            read("Tap(This)"),
            Action::By(
                Reference::You,
                PlayerAction::Tap(Selection::Ref(Reference::This)),
            ),
        );
    }

    /// An explicit different agent reads natively — Ancestral Recall's
    /// `By(Target(0), Draw(3))`.
    #[test]
    fn explicit_agent_reads_natively() {
        assert_eq!(
            read("By(Target(0), Draw(Literal(3)))"),
            Action::By(Reference::Target(0), PlayerAction::Draw(Count::Literal(3)),),
        );
    }

    /// The source-agent verbs read natively.
    #[test]
    fn source_verbs_read_natively() {
        assert_eq!(
            read("DealDamage(Target(0), Literal(3))"),
            Action::DealDamage(Selection::Ref(Reference::Target(0)), Count::Literal(3)),
        );
        assert_eq!(
            read("Destroy(This)"),
            Action::Destroy(Selection::Ref(Reference::This)),
        );
    }

    /// `By(You, …)` writes the player action bare and round-trips.
    #[test]
    fn by_you_round_trips_bare() {
        let v = Action::By(Reference::You, PlayerAction::Draw(Count::Literal(1)));
        let written = write(&v);
        assert!(
            !written.contains("By("),
            "By(You, …) should write bare, got {written}"
        );
        assert_eq!(read(&written), v);
    }

    /// A non-`You` agent writes the explicit `By(other, …)` form and
    /// round-trips.
    #[test]
    fn by_other_round_trips() {
        let v = Action::By(Reference::Target(0), PlayerAction::Draw(Count::Literal(3)));
        let written = write(&v);
        assert!(
            written.contains("By("),
            "expected explicit By, got {written}"
        );
        assert_eq!(read(&written), v);
    }
}
