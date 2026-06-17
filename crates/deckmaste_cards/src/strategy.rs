//! Data-driven play strategies: authored RON play policy, a peer of [`Deck`].
//!
//! A strategy is RON *data*, not a macro (the macro language has no control
//! flow): its "sensing" half reuses core's `Condition` / `Filter` / `Count` /
//! `Reference` vocabulary verbatim, and its branching is an ordered rule list
//! the evaluator walks — the same way the engine walks `Vec<Ability>`. The
//! types here are pure data; the evaluator that turns a strategy + game state
//! into a decision lives in `deckmaste_engine`.
//!
//! [`Deck`]: crate::Deck

use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::Expand;
use deckmaste_core::Expansion;
use deckmaste_core::Filter;
use deckmaste_core::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

/// Which end of a ranked candidate set a selector picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Extremum {
    /// The minimum by the selector's ranking count.
    Min,
    /// The maximum by the selector's ranking count.
    Max,
    /// First in enumeration order — no ranking applied.
    First,
}

/// Picks one option from the legal candidates by ranking them on a core
/// [`Count`] and taking the [`Extremum`] end, optionally narrowed to those
/// matching `among`. The workhorse of a strategy: most decisions reduce to
/// "the X-est legal option".
///
/// [`Count`]: deckmaste_core::Count
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub struct Selector {
    /// Which end of the ranking to take.
    pub pick: Extremum,
    /// The per-candidate quantity to rank by, evaluated with the candidate
    /// bound as `This`.
    pub by: Count,
    /// Narrowing of the legal set; `None` (omitted) = the whole legal set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub among: Option<Filter>,
}

/// How a strategy declares blocks. Coarse for v1; grows as block math matures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum BlockPolicy {
    /// Block as many attackers as possible.
    BlockAll,
    /// Declare no blocks.
    NoBlocks,
    /// Chump-block the single biggest attacker.
    ChumpBiggest,
}

/// A chosen play at a decision point — the genuinely new "choose a play" half
/// of the strategy language. Cards never choose their controller's plays, so
/// this has no card analog; but every option it carries reuses [`Selector`]
/// (and through it core's `Count`/`Filter`). `Preference` is itself a macroable
/// kind, so choose-a-play vocabulary (`AttackAll`, `Mulligan`, …) can be
/// authored as macros that expand to these literal variants.
#[derive(Debug, Clone, PartialEq, Eq, SupportsMacros)]
pub enum Preference {
    /// Pass priority.
    Pass,
    /// Concede the game (available; rarely authored).
    Concede,
    /// Play a land chosen by `what`.
    Play {
        /// Which land to play.
        what: Selector,
    },
    /// Cast a spell chosen by `what`, optionally targeting via `target`.
    Cast {
        /// Which spell to cast.
        what: Selector,
        /// How to pick its target(s); `None` = no targets / engine default.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<Selector>,
    },
    /// Activate an ability chosen by `what`, optionally targeting via `target`.
    Activate {
        /// Which ability to activate.
        what: Selector,
        /// How to pick its target(s); `None` = no targets / engine default.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<Selector>,
    },
    /// Declare the attackers chosen by `what`.
    Attack {
        /// Which creatures attack.
        what: Selector,
    },
    /// Declare blocks per the given policy.
    Block(BlockPolicy),
    /// Discard an object chosen by `what`.
    Discard {
        /// Which card to discard.
        what: Selector,
    },
    /// A remembered `Preference` macro invocation (choose-a-play vocabulary).
    /// Serialized as the invocation, not the expanded variant.
    #[macro_ron(expanded)]
    Expanded(Expansion<Preference>),
}

/// One rule of a strategy: when `when` holds (and `prefer` resolves to a legal
/// play), the evaluator applies `prefer`. Rules are tried in order — the first
/// applicable one wins.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rule {
    /// The gating condition, in core's `Condition` vocabulary (reused verbatim;
    /// boolean composition via `AllOf`/`OneOf`/`Not`).
    pub when: Condition,
    /// The play to prefer when this rule fires.
    pub prefer: Preference,
}

/// A complete play policy: a name and an ordered list of [`Rule`]s the
/// evaluator walks top-to-bottom. Authored as RON, a peer of [`Deck`].
///
/// [`Deck`]: crate::Deck
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Strategy {
    /// Human-facing name.
    pub name: String,
    /// Ordered rules; the first whose `when` holds and whose `prefer` is legal
    /// wins.
    pub rules: Vec<Rule>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read<T: serde::de::DeserializeOwned>(src: &str) -> T {
        deckmaste_core::ron::options().from_str(src).unwrap()
    }

    fn def(src: &str) -> crate::macros::MacroDef {
        deckmaste_core::ron::options().from_str(src).unwrap()
    }

    #[test]
    fn extremum_reads_variants() {
        assert_eq!(read::<Extremum>("Min"), Extremum::Min);
        assert_eq!(read::<Extremum>("Max"), Extremum::Max);
        assert_eq!(read::<Extremum>("First"), Extremum::First);
    }

    /// The central thesis: a strategy type embeds core's `Count` and `Filter`
    /// verbatim — the "sensing" vocabulary is reused, not re-invented.
    #[test]
    fn selector_embeds_core_count_and_filter() {
        use deckmaste_core::Count;
        use deckmaste_core::Filter;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;
        let s: Selector = read("(pick: Max, by: StatOf(This, Power), among: Any)");
        assert_eq!(s.pick, Extremum::Max);
        assert_eq!(s.by, Count::StatOf(Reference::This, Stat::Power));
        assert_eq!(s.among, Some(Filter::Any));
    }

    /// `among` is optional — omitting it means "the whole legal set".
    #[test]
    fn selector_among_defaults_to_none_when_omitted() {
        let s: Selector = read("(pick: First, by: Literal(1))");
        assert_eq!(s.among, None);
    }

    #[test]
    fn preference_nullary_variants_read() {
        assert_eq!(read::<Preference>("Pass"), Preference::Pass);
        assert_eq!(read::<Preference>("Concede"), Preference::Concede);
    }

    /// The object-picking preferences each carry a `Selector` to choose which
    /// object to act on; `Cast`/`Activate` also carry an optional target.
    #[test]
    fn preference_object_picking_variants_read() {
        let cast: Preference = read("Cast(what: (pick: Min, by: StatOf(This, ManaValue)))");
        let Preference::Cast { what, target } = cast else {
            panic!("expected Cast");
        };
        assert_eq!(what.pick, Extremum::Min);
        assert_eq!(target, None);

        let activate: Preference = read(
            "Activate(what: (pick: First, by: Literal(1)), target: (pick: Max, by: StatOf(This, Power)))",
        );
        let Preference::Activate { target, .. } = activate else {
            panic!("expected Activate");
        };
        assert_eq!(target.expect("target present").pick, Extremum::Max);

        assert!(matches!(
            read::<Preference>("Play(what: (pick: First, by: Literal(1)))"),
            Preference::Play { .. }
        ));
        assert!(matches!(
            read::<Preference>("Attack(what: (pick: Max, by: StatOf(This, Power)))"),
            Preference::Attack { .. }
        ));
        assert!(matches!(
            read::<Preference>("Discard(what: (pick: Min, by: StatOf(This, ManaValue)))"),
            Preference::Discard { .. }
        ));
    }

    #[test]
    fn preference_block_reads_a_block_policy() {
        assert_eq!(
            read::<Preference>("Block(BlockAll)"),
            Preference::Block(BlockPolicy::BlockAll),
        );
        assert_eq!(read::<BlockPolicy>("NoBlocks"), BlockPolicy::NoBlocks);
        assert_eq!(
            read::<BlockPolicy>("ChumpBiggest"),
            BlockPolicy::ChumpBiggest
        );
    }

    /// A whole strategy file: a name and an ordered list of rules, each pairing
    /// a core `Condition` (`when`) with a `Preference` (`prefer`).
    #[test]
    fn strategy_reads_full_with_ordered_rules() {
        use deckmaste_core::Condition;
        let s: Strategy = read(
            r#"(
                name: "Test Aggro",
                rules: [
                    (when: YourTurn, prefer: Cast(what: (pick: Min, by: StatOf(This, ManaValue)))),
                    (when: AllOf([]), prefer: Pass),
                ],
            )"#,
        );
        assert_eq!(s.name, "Test Aggro");
        assert_eq!(s.rules.len(), 2);
        assert_eq!(s.rules[0].when, Condition::YourTurn);
        assert!(matches!(s.rules[0].prefer, Preference::Cast { .. }));
        assert_eq!(s.rules[1].when, Condition::AllOf(vec![]));
        assert_eq!(s.rules[1].prefer, Preference::Pass);
    }

    /// Serialize → parse is identity: an authored strategy survives a
    /// round-trip through RON unchanged (the render side of the contract).
    #[test]
    fn strategy_round_trips_through_ron() {
        let s: Strategy = read(
            r#"(
                name: "Round Trip",
                rules: [
                    (when: YourTurn, prefer: Cast(
                        what: (pick: Min, by: StatOf(This, ManaValue), among: Any),
                        target: (pick: Max, by: StatOf(This, Power)),
                    )),
                    (when: AllOf([]), prefer: Block(BlockAll)),
                    (when: AllOf([]), prefer: Pass),
                ],
            )"#,
        );
        let written = deckmaste_core::ron::options().to_string(&s).unwrap();
        let again: Strategy = read(&written);
        assert_eq!(s, again);
    }

    /// The reuse linchpin: a `Condition` macro at the `when:` position expands
    /// through the macro reader even though it sits deep inside plain-serde
    /// `Strategy`/`Rule` — sensing positions are macro-aware for free, because
    /// core's `Condition` does the fall-through. This is what lets strategy-
    /// guide vocabulary (`Always`, `BehindOnBoard`, …) be authored as macros.
    #[test]
    fn sensing_position_expands_a_condition_macro_through_the_reader() {
        use deckmaste_core::Condition;
        let mut macros = crate::macros::macro_set();
        macros
            .insert(&def(
                r#"(name: "Always", kinds: [Condition], body: AllOf([]))"#,
            ))
            .unwrap();
        let s: Strategy = macros
            .read_str(r#"(name: "M", rules: [(when: Always, prefer: Pass)])"#)
            .unwrap();
        let Condition::Expanded(exp) = &s.rules[0].when else {
            panic!("expected expanded condition, got {:?}", s.rules[0].when);
        };
        assert_eq!(exp.name, "Always");
    }

    /// `Preference` is itself a macroable kind, so the choose-a-play vocabulary
    /// (`AttackAll`, `Mulligan`, …) can be authored as macros that expand to
    /// literal preference variants.
    #[test]
    fn prefer_position_expands_a_preference_macro() {
        let mut macros = crate::macros::macro_set();
        macros
            .insert(&def(
                r#"(name: "AttackAll", kinds: [Preference], body: Attack(what: (pick: First, by: Literal(1))))"#,
            ))
            .unwrap();
        let s: Strategy = macros
            .read_str(r#"(name: "M", rules: [(when: YourTurn, prefer: AttackAll)])"#)
            .unwrap();
        let Preference::Expanded(exp) = &s.rules[0].prefer else {
            panic!("expected expanded preference, got {:?}", s.rules[0].prefer);
        };
        assert_eq!(exp.name, "AttackAll");
    }
}
