---
needs: []
---
**Give `Effect.Modal` a positional per-mode `Maybe (Cost bs)` so Spree's
per-mode additional costs have a spelling [CR#702.172a].** Fresh workbench
review 2026-09-03, R3, resolved by ruling.

**Ruling (settled 2026-09-03): Spree is effect-level, not a new card shape.**
`Effect.Modal (q : Quantity bs) (modes : List (Effect bs))`
(`Effect.idr:1307`) carries no cost per mode, so a Spree spell's "+ [cost] —
[effect]" modes cannot be written. The mode list becomes a list of
`(Maybe (Cost bs), Effect bs)` pairs — one positional slot, `Nothing` for
every ordinary modal spell, so no separate `SpreeModal` constructor and no
second modal mechanism.

21 supported cards print Spree modes (e.g. Caught in the Crossfire).

The slot is positional and required at the constructor, with the ordinary
modal spelling supplied by a wrapping macro that passes `Nothing`, per
`docs/decisions/card-authoring-binds-no-implicits.md`. Existing modal bench
sites go through that macro and do not change shape.

The mode-count gate is unaffected: P10 (`chooseModes (exactly 3) [two modes]`)
is refused today and stays refused.

Size: M.

Done when: Caught in the Crossfire is a typechecking bench witness with a cost
on each mode; an ordinary modal spell still reads through the unchanged macro;
P10's refusal is still pinned and probed non-vacuous; a pin refuses a Spree
mode list in which no mode carries a cost, or the CR-meaningless shape the
round identifies, probed non-vacuous; the build is 44/44 with 0 errors and 0
warnings. Standard constraints apply, plus the RON-shaped constraint: a core
constructor is admissible only if the RON re-emitter can produce it from a RON
node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Effect.Modal`'s `modes` field is now `List (Maybe (Cost bs), Instruction
  bs)` — one positional slot per mode, required at the constructor.
  `modeCount` and the cost-action fold (`costActionOk`/`costModesOk`, the
  renamed `costActionsOk`) retyped to the pair list; both project the
  `Instruction` half and ignore the mode's own cost.
- `Macros.noCosts` (new) maps a plain `List (Instruction bs)` to the
  all-`Nothing` pair list; `Macros.chooseModes` (unchanged spelling at every
  call site) now builds that pair list internally before handing it to
  `Modal` — the ordinary modal wrapping macro the ticket calls for.
  `Cards.Keyword.borrowedMalevolence` (Escalate), the only bench site that
  called `Modal` directly instead of through the macro, now goes through
  `Macros.chooseModes` too, so no card constructs `Modal` outside a macro.
- `Macros.spree` (new): "choose one or more modes", each carrying its own
  cost [CR#702.172a,700.2h]. Fixed quantity (`atLeast 1`), positional
  `List (Maybe (Cost bs), Instruction bs)`, plus a new `AllCosted` obligation
  (`Effect.allCosted`/`AllCosted`) requiring every mode's cost slot to be
  `Just` — the CR-meaningless shape identified for the round: a Spree mode
  with no cost, since [CR#702.172a] itself defines Spree as paying "the costs
  associated with those modes" (every mode has one).
- Elaboration note: a bare `Macros.atLeast 1` (or any freshly-applied
  `Quantity`-producing macro) inside `spree`'s own obligation types created
  an unconstrained, never-resolved `bs` metavariable distinct from the
  caller-supplied `q` variable `chooseModes` threads — `Modal q modes`
  works because `q` is already tied to the function's own `bs`; a literal
  built inside the callee is not. Fixed by ascribing it explicitly —
  `the (Quantity bs) (Macros.atLeast 1)` — so its `bs` unifies with `spree`'s
  own rather than floating free.
- Bench (`Cards.Choice`): `caughtInTheCrossfire` ({R}{R} instant, two
  {1}-costed modes, "each outlaw creature" / "each non-outlaw creature" via
  `Or`/`Not` over the five outlaw creature types [CR#700.12]),
  `requisitionRaid` ({W} sorcery, three {1}-costed modes), `rustlerRampage`
  ({W} instant, two {1}-costed modes) — all three spelled with
  `Macros.spree`, no `{…}` braces.
- Pins (`ProofsChoice`): `badModalOverreach` (P10, re-spelled to the pair
  shape, unchanged obligation) still refuses `chooseModes (exactly 3) [two
  modes]`; `okSpreeBothCosted`/`badSpreeMissingCost` are the new twin pair —
  the positive twin has every mode costed, the pin drops one mode's cost and
  binds `{ac = ok}`. Both P10 and the new pin were probed non-vacuous by
  mis-stating them (widening the quantity / costing the missing mode) and
  confirming the error changes from "not a valid impossible case" to nothing
  (i.e. the mis-stated term now typechecks) — reverted after the probe.

## Landing record

- Gates: `cd idris && ./scripts/build` — clean rebuild (`rm -rf build`),
  44/44 modules, 0 Error lines, 0 Warning lines, exit 0.
  `cargo xtask cite check --list-noncompliant` — 0 in this diff (3
  pre-existing non-compliant strings remain in an untouched ticket file,
  `docs/tickets/done/xtask-corpus-workers-env-cap.md`, outside this diff's
  files). `cargo xtask cite check` — 18359 citations checked, 0 stale.
  `cargo xtask cite bless` — registered nothing new (`[CR#702.172a]` and
  `[CR#700.2h]` were already present in `cr-citations.lock` from other
  citation sites); `cr-citations.lock` is unchanged in this diff.
  `jj diff --git | cargo xtask cite audit --diff` — 4 citation sites audited,
  each rule's text read against its claim, all on-topic.
- Assurance: restored 0, re-spelled 4 (the four pre-existing `Modal`-shaped
  pins in `ProofsChoice`, moved to the pair-list shape with unchanged
  obligations), ignored-with-blocker 0, added 6 (`okSpreeBothCosted`,
  `badSpreeMissingCost`, `caughtInTheCrossfire`, `requisitionRaid`,
  `rustlerRampage`, plus the `Macros.chooseModes` re-spelling of
  `Cards.Keyword.borrowedMalevolence`'s Escalate modes, counted as a
  re-spell of that existing card rather than an addition), removed 0.
- Deviations and additions: (1) `Cards.Keyword.borrowedMalevolence` was
  changed from calling `Modal` directly to `Macros.chooseModes` — required by
  `Modal`'s new positional shape and by the ticket's "existing modal bench
  sites go through that macro" requirement; that card was not itself in the
  ticket's letter but its direct `Modal` use would not otherwise typecheck.
  (2) The pin targeting "a Spree mode list in which no mode carries a cost"
  is spelled as a new `AllCosted` obligation on `Macros.spree` rather than as
  a bare obligation on `Modal` itself — an all-`Nothing` mode list at the
  `Modal` level is the ordinary (non-Spree) "choose one or more" modal shape
  already witnessed by `rainOfThorns`, so refusing it there would refuse a
  real, CR-meaningful sentence; the meaninglessness is specific to the Spree
  labelling, so the gate lives on the macro that spells Spree.
- No STOP taken.
