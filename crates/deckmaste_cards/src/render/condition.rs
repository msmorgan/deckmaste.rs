//! Rendering for `Condition` predicates — the intervening-if / "only if"
//! clauses around triggered and activated abilities ([CR#603.4,602.5b]).

use deckmaste_core::Condition;
use deckmaste_core::Filter;
use deckmaste_core::Reference;
use deckmaste_core::RelationFilter;

use super::Ctx;
use super::fragment::strip_expanded;

/// A `Condition` as a clause with no leading/trailing punctuation, sized to sit
/// after "if" / "only if": "it's your turn", "it's an opponent's turn".
/// Unhandled conditions yield an `[unrendered: …]` marker, never a panic.
///
/// `ctx` is threaded for the reference-bearing conditions (`Is`/`Exists` over a
/// `Target(i)`/`This`) that will render here next; the timing arms below don't
/// read it yet.
#[expect(
    clippy::only_used_in_recursion,
    reason = "ctx anchors the Target/This references reference-bearing conditions will render"
)]
pub(super) fn condition(c: &Condition, ctx: &Ctx) -> String {
    match c {
        // Look through a macro-provenance wrapper.
        Condition::Expanded(e) => condition(&e.value, ctx),
        // `YourTurn` is sugar for `TurnOf(Ref(You))`; both render the same.
        Condition::YourTurn => "it's your turn".to_string(),
        Condition::TurnOf(filter) => format!("it's {}", turn_owner(filter)),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The possessive turn-owner phrase for `TurnOf(<player predicate>)`:
/// `Ref(You)` → "your turn", `OpponentOf(Ref(You))` → "an opponent's turn",
/// `TeammateOf(Ref(You))` → "a teammate's turn".
fn turn_owner(filter: &Filter) -> String {
    match strip_expanded(filter) {
        Filter::Ref(Reference::You) => "your turn".to_string(),
        Filter::Relation(RelationFilter::OpponentOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
        {
            "an opponent's turn".to_string()
        }
        Filter::Relation(RelationFilter::TeammateOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
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
            condition(&Condition::TurnOf(Filter::Ref(Reference::You)), &ctx()),
            "it's your turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Filter::Relation(RelationFilter::OpponentOf(Box::new(
                    Filter::Ref(Reference::You)
                )))),
                &ctx()
            ),
            "it's an opponent's turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Filter::Relation(RelationFilter::TeammateOf(Box::new(
                    Filter::Ref(Reference::You)
                )))),
                &ctx()
            ),
            "it's a teammate's turn"
        );
    }
}
