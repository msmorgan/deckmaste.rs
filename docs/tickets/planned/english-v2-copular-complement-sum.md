---
needs: []
---
2026-09-04: inherits re-coverage owed for `Target land becomes a 3/3 creature
until end of turn.` from `english-v2-clause-level-duration`.

# One copular complement: the whole PredicativeComplement sum, plus scalar equality

**R4 — Group R.**

Defect, two halves.

1. `copular_subject_gap_relative_clause`
   (`crates/deckmaste_english_v2/src/constructions.rs:4348`) narrows its
   `complement` role to `PredicativeAdjectiveComplement` rather than the
   `PredicativeComplement` sum that already carries Color, Nominal, Status,
   Ability, Orientation, PowerToughness and Scalar arms. With
   `vocab PredicativeAdjective` holding one member (`Legendary = "legendary"`),
   `Destroy target creature that is legendary.` selects while
   `… that is red.`, `… that is a Goblin.`, `If that land was nonbasic, …` and
   `The same is true for creature spells you control…` all fail.
2. `abstract sum PredicativeComplement` (`:1212`) has no arm for a scalar
   *equality* complement. `Scalar: PredicativeScalarComplement` is
   `predicative_scalar { value: CardinalQuantity }` (`:1762`) — a bare cardinal,
   not `is equal to ‹count›`. So `Syr Elenora's power is equal to the number of
   cards in your hand.`, `Awakened Amalgam's power and toughness are each equal
   to the number of…` and `Yavimaya Kavu's power is equal to…` fail. The
   `ScalarMeasureValue` category already exists as a `PrepositionalComplement`
   arm (`:1094`, added by `english-v2-with-preposition`) and is the value to
   reuse.

Pinned shape. Widen every copular complement site to the `PredicativeComplement`
sum, and give that sum a scalar-equality arm built on the existing
`ScalarMeasureValue` rather than a new sealed atom. All arms are present
regardless of witness counts; the complement-shape census is provenance recorded
in the landing record, exactly as `english-v2-with-preposition` recorded its own.
The adjective inventory itself is `english-v2-adjective-inventory`'s
(`vocab PredicativeAdjective` / `AttributiveAdjective` are transitional homes for
content words); `docs/tickets/fog.md` §"Predicative complement as one copular
frame (A7)" owns the deeper question of the sum's game-carved partition. Do not
duplicate either — reference them and stop at the complement widening.

Fences. Adding a second narrowed copular construction beside the widened one. A
per-complement-kind `checked by`. Adding arms only where a census shows them.

Glossary: Predicative Complement, Copula, Complement, Subject-Gap Relative
Clause, Scalar. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.
