//! Rendering for `Condition` predicates — the intervening-if / "only if"
//! clauses around triggered and activated abilities ([CR#603.4,602.5b]).

use deckmaste_core::Cmp;
use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;

use super::Ctx;
use super::fragment::strip_expanded;

/// A `Condition` as a clause with no leading/trailing punctuation, sized to sit
/// after "if" / "only if": "it's your turn", "it's an opponent's turn".
/// Unhandled conditions yield an `[unrendered: …]` marker, never a panic.
///
/// `ctx` anchors the reference-bearing conditions (`Compare(CounterCount(r,
/// …), …)`'s `r`, "this enchantment has …") — the caller threads a
/// self-type-noun subject through it for a triggered ability's
/// intervening-if ([CR#603.4], see `ability.rs::self_type_phrase`).
pub(super) fn condition(c: &Condition, ctx: &Ctx) -> String {
    match c {
        // Look through a macro-provenance wrapper.
        Condition::Expanded(e) => condition(&e.value, ctx),
        // `YourTurn` is sugar for `TurnOf(Ref(You))`; both render the same.
        Condition::YourTurn => "it's your turn".to_string(),
        Condition::TurnOf(filter) => format!("it's {}", turn_owner(filter)),
        // [CR#122.1,603.4]: "this enchantment has ten or more luck counters
        // on it" (Chance Encounter) — only the counter-count `AtLeast`
        // shape is recognized; any other `Compare` operand/comparator falls
        // through to the generic marker.
        Condition::Compare(Count::CounterCount(r, kind), Cmp::AtLeast, Count::Literal(n)) => {
            format!(
                "{} has {} or more {} counters on it",
                super::fragment::reference(r, ctx),
                super::fragment::number_word(*n).map_or_else(|| n.to_string(), str::to_string),
                super::fragment::counter_noun(kind.as_str()),
            )
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The possessive turn-owner phrase for `TurnOf(<player predicate>)`:
/// `Ref(You)` → "your turn", `OpponentOf(Ref(You))` → "an opponent's turn",
/// `TeammateOf(Ref(You))` → "a teammate's turn".
fn turn_owner(filter: &Predicate) -> String {
    match strip_expanded(filter) {
        Predicate::Ref(Reference::You) => "your turn".to_string(),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "an opponent's turn".to_string()
        }
        Predicate::Relation(RelationPredicate::TeammateOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "a teammate's turn".to_string()
        }
        other => format!("[unrendered: {other:?}] turn"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Ctx<'static> {
        Ctx {
            subject: "~",
            targets: &[],
            that: None,
        }
    }

    /// `YourTurn` and its `TurnOf` generalization render the timing clause; the
    /// two spellings of "your turn" agree, and an opponent's turn reads.
    #[test]
    fn renders_turn_of_and_your_turn() {
        assert_eq!(condition(&Condition::YourTurn, &ctx()), "it's your turn");
        assert_eq!(
            condition(&Condition::TurnOf(Predicate::Ref(Reference::You)), &ctx()),
            "it's your turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Predicate::Relation(RelationPredicate::OpponentOf(
                    Box::new(Predicate::Ref(Reference::You))
                ))),
                &ctx()
            ),
            "it's an opponent's turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Predicate::Relation(RelationPredicate::TeammateOf(
                    Box::new(Predicate::Ref(Reference::You))
                ))),
                &ctx()
            ),
            "it's a teammate's turn"
        );
    }
}
