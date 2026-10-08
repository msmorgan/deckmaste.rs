---
needs: []
---
**Residue of the 2026-10-04 keyword-body survey: what is deferred, what stays
as it is on purpose, and what is still unwritten.** Split on 2026-10-05; the
actionable items moved to their own tickets (table below). What remains here
is either a recorded decision not to change something, or a backlog with no
card asking for it yet. Standard constraints apply.

## Where the other items went

| Item (old wording) | Now in |
|---|---|
| `gift` cannot be invoked | `plugins-v2-gift-variants` |
| The bodyless `exchange` shadows the `Exchange` alias; `auraSwap` keeps a constructor | `plugins-v2-exchange-helpers` |
| Scope: `assemble`, `hiddenAgenda`, `spaceSculptor`, `visit` | `plugins-v2-out-of-scope-keywords` (`assemble` stays, blank) |
| Lean has bodies for fight, scry and surveil | `plugins-v2-scry-surveil-fight-bodies` |
| Helpers proposed, not built (`mayOrElse`, "+X/+X", token copy, Servo token, cast permission, menace's bound) | `plugins-v2-keyword-helper-additions` |
| Named calls forced by parameter order | `plugins-v2-keyword-helper-additions` |
| `returnToBattlefieldWithCounters` omits the return wrapper | `plugins-v2-keyword-helper-additions` (delete it; see below) |
| Storm, split second and surge flatten "another spell" | `plugins-v2-keyword-helper-additions` |
| Bushido, exalted, flanking, melee, prowess and rampage repeat the subject in the toughness half | `semantics-v2-gets-both-deltas` |
| `isCard` written inconsistently | `plugins-v2-keyword-helper-additions` |
| `amass` has no `Enact` wrapper | `semantics-v2-actor-handoff` |
| The `exile` keyword action has no agent | `plugins-v2-implicit-actor-spelling` (`exileBy` retires) |

## Corrections to the old text (2026-10-05)

- "Eight bodies write the raw wrapper" for exile is false now: eight files call
  the `exileBy` helper (ingest, scavenge, myriad, embalm, eternalize, recover,
  unearth, forage), which the handoff turns into `exile(…)` under the actor.
- Old text quoting `aAtRandom` now reads `aRandom`.
- `returnToBattlefieldWithCounters`: "one of the two is wrong" was mistaken.
  The RON helper has no callers, and persist and undying already go through
  `returnToBattlefield`; the helper is deleted.

## Stays as it is, on purpose

- **Sequenced costs stay refused** (`Instruction.costActionOk` in
  `lean/Semantics/Check/Abilities.lean`, pin `badSequentialCost` in
  `Proofs/Deontic.lean`). The reason, to be recorded beside the rule: costs are
  paid "in any order" [CR#601.2h], which a cost written as an ordered
  sequence would contradict, and no Vintage-legal card has "then" inside a
  cost. The rule's other inconsistencies (it
  accepts a sequence inside a fixed repeat, and accepts conditionals and loops)
  wait until costs are next touched.
- **Deferred, unchanged:** `champion`, `enchant`, `companion`, `foretell`
  declare a parameter the keyword term drops and carry `keyword_params: []`.
  Owner: they "may need new mechanisms that allow different parts of the game
  state to be addressed that currently cannot be". `enchant` is marked
  "Settled empty" and still drops its subject.
- **`prototype`** forwards nothing by design: its cost, power and toughness are
  the inset frame's alternative characteristics [CR#718.1,718.2], modelled as
  `Card::Prototype`. A frame fact; listed so nobody "fixes" it.
- **`demonstrate`** stays on the raw allowlist with `Pro(Word(Copy), One,
  top(1))` for "that copy" (the chosen player's copy, not the caster's). No
  `latest` helper is built: one use does not justify a name.

## Backlog, waiting for a card or a pass

- **21 keyword actions have no body** and wait for a card that needs one:
  activate, assemble (blank on purpose), cast, cloak, collectEvidence,
  connive, discover, double, exchange (bodyless on purpose, for parsing),
  fateseal, incubate, learn, manifest, manifestDread, play, regenerate,
  support, theRingTemptsYou, triple, ventureIntoTheDungeon, waterbend.
- **The counter and type declarations still write raw conferrals.** `types/`
  is skipped by `cargo xtask expansions`, so give it coverage before
  re-spelling. (The counter declarations are rewritten by
  `semantics-v2-counter-kind-is-a-name`; do this after it.)
  Unblocked 2026-10-06: `semantics-v2-counter-kind-is-a-name` is done, so the
  counter and type raw-conferral re-spells this item was holding can go
  ahead. They are Sonnet-mechanical.
- **Helper macros' own bodies** were left for a later pass.

## Landing record

The series: `qqowuxzv` (routing: the type half and the helper bodies) and
this record. No RON, Rust or Lean file changed: every re-spell this ticket
held had already landed or is blocked. Measured on the tree at `qqowuxzv`
over claim `szzporps`; `kata refresh` was a no-op.

**Survey (2026-10-08).** `keyword_actions/` and `keyword_abilities/` grepped
for `Printed(`, `Boost(`, `Conferral`, `CounterKind(`, `Named(`,
`Action("`, `keyword("`, `PtModification`, `AbilityGrant`, `Property(` and
`TypeDef` outside comments: 0 sites. Every keyword body is already over
helpers or bodyless.

- **(a) Counter conferrals: 0 to re-spell.** The 73 `counter_kinds/`
  declarations already write the general helper forms
  (`semantics-v2-counter-kind-is-a-name`, `pxvxllnp`): 2
  `boost(up(1), up(1))` / `boost(down(1), down(1))`, 15
  `grants(<keyword>)`, 56 confer nothing. 0 raw.
- **(b) Type conferrals: STOP, routed.** `creature`, `land` and `instant`
  write raw v1-shaped conferrals inside `TypeDef(…)`. `types/` has no
  expansion coverage: `cargo xtask expansions` skips all ten ("no kind of
  [Type] is a v2 syntax type, and the body (`TypeDef`) is not a registry
  node"), and coverage cannot be added in xtask alone, because v2 does not
  read a `Type` body at all (a loader tag, `ron.rs` `DECLARATION_KINDS`;
  Lean `Definition` has no type constructor). Giving it coverage is a
  Lean-first model change, so the half is routed whole to the new
  `semantics-v2-type-definitions`. No helper was invented.
- **(c) `returnToBattlefieldWithCounters`: already done**
  (`plugins-v2-keyword-helper-additions`, S2). The Lean bench macro of the
  same name stays by that ticket's decision (two bench uses in
  `lean/Semantics/Cards/Counters.lean`).

**Stays as it is, on purpose** (the section above, unchanged): sequenced
costs refused; `champion`, `enchant`, `companion`, `foretell` deferred;
`prototype` forwards nothing; `demonstrate` raw with no `latest` helper.

**Routed residue.**
- The type half: `semantics-v2-type-definitions` (new, planned).
- The 21 bodyless keyword actions keep their STOP comments; their live owner
  is `semantics-v2-keyword-action-residues`.
- Helper macros' own bodies: appended to `plugins-v2-bodies-through-helpers`,
  with the one instance found (`Printed(Param(kind))` in
  `entersWithCounters`, `entersWithAdditionalCounters`,
  `entersWithFewerCounters`, `exileWithCounters`).

**Proof.**
- `cargo xtask expansions` on the claim and on `qqowuxzv`: 1509
  declarations, 1467 printed, 42 skipped, 0 failed; `diff -r` identical.
- `cargo xtask lean check`: canon 134/134, testing 7/7 (2.0s).
- `cargo test -p deckmaste_construction_core -p deckmaste_semantics_v2 -p
  xtask --no-fail-fast`: 24 binaries, 952 passed, 0 failed, 1 ignored.
  Clippy on the same crates `--tests -- -D warnings`: clean.
- `cargo xtask cite check`: 0 stale; `--list-noncompliant`: 0; the audit of
  the diff from the claim read 3 sites (`semantics-v2-type-definitions`),
  each checked against its rule text.
- `kata kanban check`: OK.
- `cargo xtask lexical --card-name "Black Lotus" --export` on the claim and
  on `qqowuxzv`: byte-identical.

**Tests.** Restored 0, re-spelled 0, ignored 0, added 0, removed 0.

**Deviations and additions.** New ticket `semantics-v2-type-definitions`;
a paragraph appended to `plugins-v2-bodies-through-helpers`. No commit for
stages (a) and (c): nothing was left to change.

**STOPs.** The type half (above): coverage needs a v2 type Definition.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed.
