use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Expansion, Filter, Ident, IdentSeed, Zone};

/// A turn step or phase (CR 5xx). `BeginningOf` triggers key off these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StepOrPhase {
    /// CR 502.
    Untap,
    /// CR 503.
    Upkeep,
    /// CR 504.
    Draw,
    /// CR 505 (first main phase).
    PrecombatMain,
    /// CR 507.
    BeginningOfCombat,
    /// CR 508.
    DeclareAttackers,
    /// CR 509.
    DeclareBlockers,
    /// CR 510.
    CombatDamage,
    /// CR 511.
    EndOfCombat,
    /// CR 505 (second main phase).
    PostcombatMain,
    /// CR 513.
    EndStep,
    /// CR 514.
    Cleanup,
}

/// Whose turn a step-based trigger watches (CR 503.1 "your upkeep", etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum WhoseTurn {
    /// The controller's own turn.
    Your,
    /// Every player's turn of that step.
    EachPlayers,
    /// An opponent's turn.
    AnOpponents,
}

/// The state a `StateBecomes` transition watches (CR 603.2e). A small set
/// today; variants accrete as cards force them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StateFilterEvent {
    /// Becomes tapped.
    Tapped,
    /// Becomes untapped.
    Untapped,
    /// Becomes attacking (CR 508.1a).
    Attacking,
    /// Becomes blocked (CR 509.1h).
    Blocked,
}

/// A trigger-event pattern, matched structurally against the action log
/// (CR 603.2). Declared event names (`Dies`, `Enters`, `Landfall`) are macros
/// over these forms. Each form binds fixed roles (`ThatObject`,
/// `ThatPlayer`) for the body.
///
/// Manual serde, like `Filter`/`Effect`: dispatch by name over one variant
/// list so unknown names at Event positions fall through to the macro layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// A verb was performed (CR 603.2 over the action log). `by`/`on` default
    /// to match-anything (`Filter::Any`).
    Performed { verb: Ident, by: Filter, on: Filter },
    /// An object changed zones (CR 603.6). `Dies` = `from: Battlefield,
    /// to: Graveyard` is a prelude macro over this.
    ZoneMove {
        what: Filter,
        from: Option<Zone>,
        to: Option<Zone>,
    },
    /// The beginning of a step or phase (CR 603.2, "at the beginning of …").
    BeginningOf(StepOrPhase, WhoseTurn),
    /// An object's state changed — transitions only (CR 603.2e).
    StateBecomes {
        of: Filter,
        becomes: StateFilterEvent,
    },
    /// Any of several events (CR 603.2, "whenever … or …").
    OneOfEvents(Vec<Event>),
    /// A remembered `Event` macro invocation (`Dies`, `Enters`, `Landfall`,
    /// …). Serialized as the invocation, not the struct.
    Expanded(Expansion<Event>),
}

/// Every name an Event position accepts. Must stay in sync with `visit_enum`
/// (the drift-guard test catches missing arms).
const VARIANTS: &[&str] = &[
    "Performed",
    "ZoneMove",
    "BeginningOf",
    "StateBecomes",
    "OneOfEvents",
    "Expanded",
];

/// `Performed`, deserialized as its own struct so the `by`/`on` defaults
/// apply (newtype-variant delegation, flat in RON via
/// `unwrap_variant_newtypes`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Performed {
    verb: Ident,
    #[serde(default = "Filter::any")]
    by: Filter,
    #[serde(default = "Filter::any")]
    on: Filter,
}

/// `ZoneMove`, deserialized as its own struct for the `from`/`to` Option
/// defaults.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct ZoneMove {
    what: Filter,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    from: Option<Zone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    to: Option<Zone>,
}

/// `StateBecomes`, deserialized as its own struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct StateBecomes {
    of: Filter,
    becomes: StateFilterEvent,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EventVisitor;

        impl<'de> Visitor<'de> for EventVisitor {
            type Value = Event;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("an event") }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Event, A::Error> {
                let (ident, v) = data.variant_seed(IdentSeed)?;
                // Adding a form? Update VARIANTS above to match.
                Ok(match ident.as_str() {
                    "Performed" => {
                        let p: Performed = v.newtype_variant()?;
                        Event::Performed {
                            verb: p.verb,
                            by: p.by,
                            on: p.on,
                        }
                    }
                    "ZoneMove" => {
                        let z: ZoneMove = v.newtype_variant()?;
                        Event::ZoneMove {
                            what: z.what,
                            from: z.from,
                            to: z.to,
                        }
                    }
                    "BeginningOf" => {
                        let (step, whose) = v.tuple_variant(2, crate::de_util::Pair::new())?;
                        Event::BeginningOf(step, whose)
                    }
                    "StateBecomes" => {
                        let s: StateBecomes = v.newtype_variant()?;
                        Event::StateBecomes {
                            of: s.of,
                            becomes: s.becomes,
                        }
                    }
                    "OneOfEvents" => Event::OneOfEvents(v.newtype_variant()?),
                    "Expanded" => Event::Expanded(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("Event", VARIANTS, EventVisitor)
    }
}

impl Serialize for Event {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Event::Performed { verb, by, on } => {
                let p = Performed {
                    verb: *verb,
                    by: by.clone(),
                    on: on.clone(),
                };
                serializer.serialize_newtype_variant("Event", 0, "Performed", &p)
            }
            Event::ZoneMove { what, from, to } => {
                let z = ZoneMove {
                    what: what.clone(),
                    from: *from,
                    to: *to,
                };
                serializer.serialize_newtype_variant("Event", 1, "ZoneMove", &z)
            }
            Event::BeginningOf(step, whose) => {
                serializer.serialize_newtype_variant("Event", 2, "BeginningOf", &(step, whose))
            }
            Event::StateBecomes { of, becomes } => {
                let s = StateBecomes {
                    of: of.clone(),
                    becomes: becomes.clone(),
                };
                serializer.serialize_newtype_variant("Event", 3, "StateBecomes", &s)
            }
            Event::OneOfEvents(events) => {
                serializer.serialize_newtype_variant("Event", 4, "OneOfEvents", events)
            }
            // The invocation, not the struct: `Expansion`'s Serialize emits it.
            Event::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, Type};

    fn read(source: &str) -> Event { crate::ron::options().from_str(source).unwrap() }

    /// `by`/`on` default to match-anything when omitted (the `Filter::Any`
    /// default is load-bearing — removing it breaks `Performed(verb: …)`).
    #[test]
    fn performed_defaults_by_and_on_to_any() {
        assert_eq!(
            read(r#"Performed(verb: "Sacrifice")"#),
            Event::Performed {
                verb: "Sacrifice".into(),
                by: Filter::Any,
                on: Filter::Any,
            },
        );
    }

    #[test]
    fn zone_move_options_default_none() {
        assert_eq!(
            read("ZoneMove(what: Type(Creature), from: Battlefield, to: Graveyard)"),
            Event::ZoneMove {
                what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
            },
        );
        assert_eq!(
            read("ZoneMove(what: Type(Creature))"),
            Event::ZoneMove {
                what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                from: None,
                to: None,
            },
        );
    }

    #[test]
    fn beginning_of_reads() {
        assert_eq!(
            read("BeginningOf(Upkeep, Your)"),
            Event::BeginningOf(StepOrPhase::Upkeep, WhoseTurn::Your),
        );
    }

    /// Every VARIANTS entry must be handled in `visit_enum`: a missing arm
    /// surfaces as serde's `unknown_variant`, which the macro layer would
    /// misreport as a failed macro lookup.
    #[test]
    fn variants_list_matches_visit_enum() {
        for &name in VARIANTS {
            if let Err(error) = crate::ron::options().from_str::<Event>(name) {
                let message = error.to_string();
                assert!(
                    !message.contains("Unexpected variant") && !message.contains("unknown variant"),
                    "VARIANTS entry `{name}` is not handled in visit_enum: {message}"
                );
            }
        }
    }
}
