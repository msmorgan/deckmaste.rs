---
needs: [macro-first-wave]
---
**The no-dead-grammar sweep: every core grammar node carries ≥ 1 acceptance
card AND ≥ 1 checker reject fixture, enforced by CI.** A node nobody can
exercise is a liability, not a feature — this closes the historical
dead-connective tier (nodes minted speculatively, never used, silently
drifting) and makes the rule permanent: from here on, a new core node lands
WITH its fixture pair in the same change, or CI is red.

Related: [[core-copy-grammar]] (if copy-except shapes land there, its fixture
pair lands there and this sweep only audits it).

## The coverage check

- A CI test walks the core grammar's node inventory (enum variants across
  effects/actions/statics/events/counts/conditions/references) and asserts
  each appears in at least one loading acceptance card (canon/testing/builtin)
  and at least one reject fixture under `crates/deckmaste_plugin/tests/reject/`
  (nodes with no illegal configuration document why, in an explicit allowlist
  with a reason string — the allowlist is reviewed, not a dumping ground).
- The check is mechanical (derive/inventory-driven), so future nodes are
  covered automatically the moment they exist.

## Backfill (the known former dead tier and the named hard cases)

- `Vote` → Tyrant's Choice.
- `SeparatePiles`/`ChoosePile` → Fact or Fiction; a Liliana −6-shaped 2-pile
  split; Whims of the Fates (noted per-player piles, all players separate
  before any player sacrifices).
- `ChooseValue` → Three Tree City.
- `AdditionalCost` → Fling.
- Modal cost riders → Collective Defiance (escalate + per-mode targets +
  amount anaphora).
- Fact-backed product groups → a Blood-Money-shaped "destroyed this way"
  card.
- Copy-except → an accept (Vesuvan-Doppelganger-style `except:` on copiable
  characteristics) and a reject (a `SetController` inside `except:` —
  controller is not a copiable characteristic, `E-COPY-EXCEPT`)
  [CR#707.2,707.3,109.3].
- The delayed-trigger signature pair (exile-and-return: `That(Card)` accept /
  `That(Creature)` reject [CR#603.7c]).

## Done

- Coverage test lands and is green: zero uncovered nodes, allowlist reviewed.
- All backfill fixtures above load, elaborate, and (where engine support
  exists) execute in tests; rejects fail with their exact codes.

## Verification

- `cargo test --workspace` green, including the new coverage test.
- `cargo xtask validate` clean; `cargo xtask fidelity` green on canon.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
