---
needs: []
---
**Engine: enter riders have grammar and validation but no execution.**

`core-action-riders-cost-modes` (done) landed the `EnterRider` shapes in
`deckmaste_core` with elaborator battlefield-gate validation, but never wired
engine execution. `copy::has_unbuilt_enter_rider` is
`riders.iter().any(|r| !matches!(r, EnterRider::AsCopy(_)))` — every rider but
`AsCopy` counts as unbuilt — so `Tapped`, `FaceDown`, `UnderControlOf`,
`UnderOwnersControl`, `Attacking`, and `WithCounters` each panic the moment a
card uses one, at `Action::Move`, `Action::MoveGroup`, and `Action::Create`
token minting.

**Why the owner was wrong.** That ticket self-scopes in its own header as "the
action-vocabulary half of the grammar reshape", and its Done section covers
only `deckmaste_core` shapes and serde tests, elaborator reject fixtures, and
mechanical canon/wizards edits. `engine-find-moved-object` corroborates
independently: it recorded that a canon blink card "is NOT added — its oracle
needs enter-rider rendering AND execution (the `Action::Move` riders `todo!`
seam, `core-action-riders-cost-modes`)", and used an inline fabricated card for
its e2e instead.

Apply each rider where the object lands in its destination zone, alongside the
existing static enters-tapped / enters-with-counters handling ([CR#603.6d]
makes that text a static ability whose effect happens as part of the entering
event; [CR#614.12] is the replacement-effect frame it sits in):

- `Tapped` / `FaceDown` — set the state flag at mint. Mechanical.
- `WithCounters(kind, count)` — place counters in the same batch, reusing the
  `Action::MoveCounters` plumbing. Mechanical.
- `UnderControlOf(Reference)` / `UnderOwnersControl` — resolve the reference
  and set controller at mint instead of defaulting.
- `Attacking(Option<Reference>)` — the large one. It has to fold the entering
  creature into the current combat's attacker set, and optionally its defender
  per the `Option<Reference>`. Scope this against the attacker-declaration
  machinery before committing to a design, and split it into its own ticket if
  that wiring proves non-trivial rather than letting it balloon this one.

Done when the mechanical riders each land under unit test, and a canon card
using a non-`AsCopy` rider graduates off `.ron.todo` — the blink/reanimation
family is the natural target, but re-check the current `.ron.todo` set rather
than trusting the older ticket's list.

Effort: **M**.
