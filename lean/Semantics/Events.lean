import Semantics.Words

/-!
# Semantics.Events

Shared event parameters and the markings and modes clauses carry. Event patterns themselves
live in the recursive phrase model; their facts are checked in `Check/Events`.
-/

namespace Semantics

inductive PaymentOutcome where
  | paid | unpaid
  deriving DecidableEq, Repr

/-- Whether a lookback tally counts event occurrences ("the number of times…") or sums the
quantity acted on ("the number of cards…", "the amount of life…"). -/
inductive TallyMode where
  | events
  | quantity
  deriving DecidableEq, Repr

inductive ReplUse where
  | repeatedly | nextTimeOnly
  deriving DecidableEq, Repr

/-- The explicit qualifier on damage: "combat damage" [CR#120.2a] or "noncombat damage"
[CR#120.2b]. A `kind` slot holds `Option DamageKind`, and `none` is unqualified "damage": in an
instruction it is the effect's own damage, noncombat by rule [CR#120.2b,701.14d], except in a
replacement's `instead` over a damage event, which is that event modified and keeps its kind
("it deals double that damage instead") [CR#614.6]; in an event, replacement, deontic or
lookback it matches all damage. -/
inductive DamageKind where
  | combat | noncombat
  deriving DecidableEq, Repr

/-- How a static ability's condition holds it in force: "as long as" it holds, or "if" it does.
"Unless C" is "if not C" (`onlyUnless`); whether a negated condition is said with "unless" is
read from the condition, not stored. -/
inductive CondMarking where
  | asLongAs | ifSo
  deriving DecidableEq, Repr

/-- How "a …" chooses. -/
inductive ChoiceMode where
  | unmarked
  | theirChoice
  | atRandom
  | yourChoice
  deriving DecidableEq, Repr

end Semantics
