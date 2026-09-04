---
needs: []
---
**Close the two shapes fused-variants-3 stopped on that are not the look bodies.** Residue of
`workbench-fused-variants-3` (2026-09-04); each was a STOP with its evidence
in that ticket's landing record.

- **Skip-duration re-read.** `Effect.SkipsAllOf` stays because both bench
  cards (Empty City Ruse, False Peace) say "of their next turn", and
  `Duration.DuringNextTurnOf who` cannot be reached from `Continuously`'s
  span: the span is typed at `staticIntro se` and `staticIntro (Skips _ _) =
  bs`, so it cannot re-read the skipping player. Decide between
  `staticIntro (Skips who _) = nomIntro who` (moves every `Static (Skips …)`
  bench card) and a duration that re-reads the static's subject; then fold
  `SkipsAllOf` away.
- **Look bodies** — ruled 2026-09-04 and split out as `workbench-one-scry`.
- **Spell-level cast timing.** "Cast this spell only before the combat damage
  step" (Berserk, Blood Frenzy) needs a timing slot reachable from a spell:
  `Triggers.Timing` is reachable only from `Effect.AbilityAt.Activated`. Add
  the positional slot (RON-shaped), bench Berserk through `BeforePart
  CombatDamage`, pin a CR-meaningless timing on a spell.

Size: M. Done when: `SkipsAllOf` is gone or its retention is ruled; Berserk
is benched; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## As landed

- **Skip-duration re-read** — took `staticIntro (Skips who _) = nomIntro who`.
  `Effect.SkipsAllOf` is gone (the constructor plus its `reflexEncloseUse`,
  `instrIntro` and `instrProfile` clauses; `Continuously`'s own clauses already
  compute the same intro and profile). Empty City Ruse and False Peace read
  `Macros.throughout (Skips <who> Combat) (DuringNextTurnOf (Macros.That
  PlayerW OneOf))` — "of their next turn" as an anaphoric re-read of the
  skipping player. This is the branch that keeps `Continuously se span`
  positional and RON-shaped with no second target announced: it adds no
  constructor at all, and reuses the `DuringNextTurnOf (Macros.That PlayerW
  OneOf)` span Gideon Jura already writes. The rejected branch — a `Duration`
  that re-reads the static's subject — would have added a `Duration`
  constructor with no RON node behind it whose only job is to name a subject
  the pronoun can already reach.
  The four `Static (Skips …)` bench sites (Eon Hub, Stasis, Yawgmoth's
  Bargain, `sandsOfTimeSkip`) needed no re-spelling: their subjects
  (`PlayerGroup AllPlayers`, `You`, `Macros.each AnyPlayer`) introduce nothing
  targeted, so `Static`'s `Untargeting` gate still holds over the wider intro.
  Pin `ProofsStatic.badSkipDuringUnboundNextTurn` ("You skip all combat phases
  of their next turn" — the duration names a player the static never bound),
  twin `okSkipDuringTheirNextTurn`.
- **Look bodies** — not this round (split out as `workbench-one-scry`).
- **Spell-level cast timing** — `Effect.AbilityAt.Spell` gains a positional
  `(window : Maybe (Timing bs))` ahead of its instruction; `Triggers.Timing`
  is unchanged, so the activated-ability reach is not widened, and the same
  `Timing` serves both because the combat-timing rules apply to a spell's
  "cast only" and an ability's "activate only" alike [CR#506.7g]. 209 existing
  `Spell` sites take `Nothing`. Bench: **Berserk** and **Blood Frenzy**
  (`Cards/Turn.idr`), both full printed cards, each through `BeforePart
  CombatDamage Nothing`. Pin `ProofsMana.badCastBeforeTheTurn` ("Cast this
  spell only before the turn"), twin `okCastBeforeCombatDamage`.

## Landing record

Measured on change `xnmwumto` (working copy at the time of writing), against
parent `mmvtnwvn` (`kata: claim workbench-fused-residues`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructors (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`) | 795 | 794 |
| `Unspellable` pins | 613 | 615 |
| `: Card` bench witnesses | 807 | 809 |
| diffstat | — | 22 files, 290 insertions, 226 deletions |

One core constructor net: `Effect.Instruction.SkipsAllOf` deleted, nothing
added — `AbilityAt.Spell` gains a slot, not a sibling.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0; 0 lines matching `^Error` or `warning`
  (case-insensitive).
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → 0 stale.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → every cited rule
  read against the claim citing it.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 at the start).
- re-spelled: 2 named, plus one mechanical sweep.
  - `Cards/Deontic.idr` `emptyCityRuse` and `falsePeace`: same cards, same
    asserted outcome, new spelling (`SkipsAllOf` → `throughout (Skips …)
    (DuringNextTurnOf …)`), which is what the fold retires.
  - 209 `Spell` sites take the new `Nothing` window (`Cards/*.idr`,
    `Proofs*.idr`); every subject is unchanged.
- ignored with a blocker: 0.
- added: 2 pins (`badSkipDuringUnboundNextTurn`, `badCastBeforeTheTurn`), 2
  positive twins (`okSkipDuringTheirNextTurn`, `okCastBeforeCombatDamage`), 2
  bench cards (Berserk, Blood Frenzy).
- removed: 0.
- pin non-vacuity probes run (mis-state, watch the message change): 2 of 2.
  `badSkipDuringUnboundNextTurn` (`Skips You` → `Skips (Macros.target
  Opponent)`) and `badCastBeforeTheTurn` (`Turn` → `Upkeep`) each turn into
  `… is not a valid impossible case.`

**Deviations and additions**

1. **`Spell`'s window is positional-first, not appended.** `Activated` carries
   its window after the instruction because "Activate only …" is printed last;
   the cast restriction is printed as the spell's first sentence (Berserk,
   Blood Frenzy, Teleport, Festival), so the slot precedes the instruction.
   Every `AbilityAt` constructor's argument order follows its printed order.
2. **No spell-specific gate on the new window.** The slot routes through
   `Timing`'s existing gates (`beforePartOk`, `WindowOk`, `PointWindowOk`) and
   adds none of its own: nothing in the Comprehensive Rules makes `AsSorcery`,
   `AsInstant` or `DuringPart` meaningless as a cast restriction, and a gate
   that refused them would be a house preference rather than a rules refusal.
   The pin is therefore an inherited-gate refusal at the new slot, which is
   what it is evidence for — that the slot is not a hole around `Timing`.
3. **Berserk reads "its power" as `Macros.It OneOf`,** not a second
   `Macros.ownSubject`: inside `Gets Adds (ownSubject …) pow tou` the power
   slot sits at `selfSubjIntro` of an `Own` noun, whose own delta is empty, so
   an `Own`-style re-read counts 0 reaches (`Can't find an implementation for
   0 = 1`). The `Bare` self-subject binding the `Gets` subject introduces is
   what "its" reads.
4. **No macro added.** Both new bench cards reach the core through existing
   macros (`throughout`, `sharedSubject`, `gets`, `delayed`, `destroy`,
   `happened`, `keyword`, `It`, `That`, `ownSubject`).

**STOPs taken**

None.

**Residue for a live ticket**

- The `Spell` window admits `DuringPart p w`, which spells the same sentence
  as `Static (OnlyDuring p w (Macros.deontic This Permit ["Cast"] Patient
  NoDeonticPatient))` — the shape `teleport`, `festival` and
  `dazzlingBeautyCastRestriction` already use. That is a fused pair a later
  round should collapse; folding it here would have retired the deontic
  `staticOnSpellCardOk` clause and left `dazzlingBeautyCastRestriction` (a
  bare cast-restriction fragment with no instruction to hang a window on)
  unspellable, neither of which this ticket names.
