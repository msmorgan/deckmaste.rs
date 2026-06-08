use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::continuous::Duration;
use crate::{Count, Effect, Event, Expansion, Filter, Phase};

/// A replacement effect: the CR's closed template list ([CR#614]).
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream — `AsEnters` and other Replacement macros); `Serialize` is **manual**
/// so `Expanded` writes the invocation back rather than the literal struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Replacement {
    /// "If [event] would happen, [effect] instead" — replace ([CR#614.1a]).
    Instead { would: Event, instead: Effect },
    /// Skip a step or phase — omit ([CR#614.1b]).
    Skip { what: Phase },
    /// "If [event] would happen, [event] and [effect]" — augment, all-at-once
    /// ([CR#614.1c]). `AsEnters` is a prelude macro over this.
    Also { would: Event, also: Effect },
    /// A remembered `Replacement` macro invocation (`AsEnters`, …).
    Expanded(Expansion<Replacement>),
}

impl Serialize for Replacement {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON.
        match self {
            Replacement::Instead { would, instead } => {
                #[derive(Serialize)]
                struct Instead<'a> {
                    would: &'a Event,
                    instead: &'a Effect,
                }
                serializer.serialize_newtype_variant(
                    "Replacement",
                    0,
                    "Instead",
                    &Instead { would, instead },
                )
            }
            Replacement::Skip { what } => {
                #[derive(Serialize)]
                struct Skip<'a> {
                    what: &'a Phase,
                }
                serializer.serialize_newtype_variant("Replacement", 1, "Skip", &Skip { what })
            }
            Replacement::Also { would, also } => {
                #[derive(Serialize)]
                struct Also<'a> {
                    would: &'a Event,
                    also: &'a Effect,
                }
                serializer.serialize_newtype_variant(
                    "Replacement",
                    2,
                    "Also",
                    &Also { would, also },
                )
            }
            // The invocation, not the struct.
            Replacement::Expanded(e) => e.serialize(serializer),
        }
    }
}

/// A prevention effect ([CR#615]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Prevention {
    /// Prevent the next N damage from `from` to `to` ([CR#615.7]).
    PreventNext {
        n: Count,
        from: Filter,
        to: Filter,
        duration: Duration,
    },
    /// Prevent the next instance of damage ([CR#615.8]).
    PreventNextInstance { from: Filter, to: Filter },
    /// Prevent all damage from `from` to `to` for a duration ([CR#615.10]).
    PreventAll {
        from: Filter,
        to: Filter,
        duration: Duration,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_reads_flat() {
        let parsed: Replacement = crate::ron::options()
            .from_str("Skip(what: Beginning(Upkeep))")
            .unwrap();
        assert_eq!(
            parsed,
            Replacement::Skip {
                what: Phase::Beginning(crate::BeginningStep::Upkeep)
            },
        );
    }

    /// The three primitive arms round-trip through the manual `Serialize`.
    #[test]
    fn primitives_round_trip() {
        for source in [
            "Skip(what: Beginning(Upkeep))",
            "Also(would: ZoneMove(what: Any, to: Battlefield), also: Tap(This))",
            "Instead(would: ZoneMove(what: Any, to: Graveyard), instead: Tap(This))",
        ] {
            let parsed: Replacement = crate::ron::options().from_str(source).unwrap();
            let written = crate::ron::options().to_string(&parsed).unwrap();
            let reparsed: Replacement = crate::ron::options().from_str(&written).unwrap();
            assert_eq!(parsed, reparsed, "round-trip mismatch for {source}");
        }
    }
}
