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

---

## Done

Five of six riders landed: `Tapped`, `WithCounters`, `UnderControlOf`,
`UnderOwnersControl`, `Attacking`. `FaceDown` deferred to `engine-face-down`
(no face-down permanent state exists yet to set) — `has_unbuilt_enter_rider`
now checks only that variant.

**Mechanism**: `crate::copy::enter_status_from_riders` folds a rider list into
the SAME `EnterStatus` the `engine-enters-replacement-compose` self-fold
populates (`replace.rs::as_enters_status`) — both sources write into the one
struct `apply_zone_will_change`/`apply_token_created` merges (union for
`tapped`, extend for `counters`, new `controller`/`attacking` fields are
rider-only). This is convergence onto the existing mechanism, not a new
parallel one — `TokenCreated` gained an `enters: Option<EnterStatus>` field to
carry it through `Action::Create` too. `WithCounters` reuses `EnterStatus`'s
existing atomic-at-mint counter slot rather than `Action::MoveCounters` (the
ticket's suggested plumbing) — that slot already exists for exactly this
no-counterless-window guarantee and predates this ticket.

`Attacking` was NOT split off: `CombatState::declare_attacker` already existed
and this engine is hard-coded 2-player, so `Attacking(None)`'s "controller
chooses" ([CR#508.4]) degenerates to a forced pick (the sole opponent) with no
new decision-surfacing needed. It mutates `CombatState` directly WITHOUT
firing `GameEvent::Attacking` — that fact also drives "whenever ~ attacks"
triggers and the [CR#508.1f] declaration-tap, both of which [CR#508.4]
explicitly excludes for an enters-attacking creature.

Cloudshift added to `plugins/canon/cards/` (hand-written — the extraction
parser doesn't handle "under your control" phrasing yet) as the first real
card exercising `EnterRider::UnderControlOf`; `no_dead_grammar.rs`'s deferred
entry for it removed. Found and fixed a pre-existing `deckmaste_legacy_render`
bug the new card's fidelity check exposed: `UnderControlOf(You)` rendered
"under you's control" instead of "under your control" (no possessive form for
`Reference::You` in that position). `plugins/canon/idris-check-baseline.ron`
blessed with Cloudshift's one known emitter gap (EnterRider lists beyond a
lone `Attacking(Some(_))` aren't in the Idris mirror yet — the same
pre-existing gap Otherworldly Journey already carries).

Tests: 8 rider unit tests (`resolve/player_action.rs`, one per rider plus a
`MoveGroup` per-member check), 1 semantic e2e (`tests/stack.rs`,
`cloudshift_returns_the_exiled_creature_under_the_casters_control`, through
the real cast/target/resolve path), `has_unbuilt_enter_rider`'s test updated
for the new FaceDown-only seam.
