---
needs: []
---
**Add the combat steps and cleanup to `Words.TurnPart` [CR#506.1,514.1].**
Fresh workbench review 2026-09-03, F9.

`Words.TurnPart` has no `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`
or `Cleanup`, so a whole family of printed trigger headers has no spelling:
"at the beginning of the declare blockers step" (34 supported cards, e.g.
Dazzling Beauty), "declare attackers step" (26), "combat damage step" (3),
"cleanup step" (15).

The four names are already RON: `plugins/builtin_v2/macros/stubs/turn_parts/`
holds `DeclareAttackersStep.ron`, `DeclareBlockersStep.ron`,
`CombatDamageStep.ron` and `CleanupStep.ron`, so the constructors are
re-emittable the day they land.

Fix: four constructors plus their rows in `Words.turnPartIx`,
`Words.partTriggerOk` and `Words.partAddable`. The gates are already there;
this is rows, not mechanism. `Words.PartQuant` (`workbench-possessor-residues`)
applies to the new parts unchanged — a turn can hold more than one combat
phase, so "each combat damage step" is spellable without a new slot.

Size: S.

Done when: a "beginning of the declare blockers step" trigger (Dazzling
Beauty) and a cleanup-step trigger are typechecking bench witnesses; the four
new parts each have their `partTriggerOk`/`partAddable` rows; a pin refuses a
part that the CR makes untriggerable, probed non-vacuous; the build is 44/44
with 0 errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Words.TurnPart` gains `DeclareAttackers | DeclareBlockers | CombatDamage |
  Cleanup` (indices 11–14, appended after `BeginningPhase` so no existing
  index moves) with a docstring citing `[CR#508.1,509.1,510.1,514.1]`, the
  turn-based-action rule that names each step. `Events.partTriggerOk` and
  `Events.partAddable` already classify every non-`Turn` part as `True` via
  their wildcard clause, so the four new constructors are covered with no
  code change there — the ticket's "rows" were already total.
- Bench witnesses (`Experimental.Cards.Turn`): `teleport` (DeclareAttackers,
  full printed card — "Cast this spell only during the declare attackers
  step. Target creature can't be blocked this turn.", reusing the exact
  `OnlyDuring`/`Macros.deontic` shape `Cards.Deontic.festival` already uses
  for a possessor-bearing sibling, and `Macros.cantBeBlocked` from
  `Cards.Deontic.infiltrate`); `dazzlingBeautyCastRestriction`
  (DeclareBlockers, a fragment — see Deviations); `thawingGlaciers`
  (Cleanup, full printed card — search-a-basic/put-tapped/shuffle plus
  `Macros.delayed (BeginningOf ThePart Cleanup NoPossessor) (Macros.move
  Macros.thisLand Macros.handZ)`, the same delayed-`BeginningOf` shape
  `selfSacrificeThenExile`/`turnToMist` already use for `EndStep`).
- No pin added. Every step from `CombatPhase` and `CleanupStep` has a
  well-defined "beginning" per general turn structure, and all four parts
  can recur within one turn (extra combat phases; two combat damage steps
  on first/double strike [CR#506.1]; a fresh cleanup step whenever
  514.3a's exception fires), so `PartQuant.EachPart` is meaningful for all
  four exactly as it already is for the existing parts — nothing in
  [CR#506,508,509,510,514] singles one of the four out as untriggerable.
  `HeaderPossessor.ByPlayer` is deliberately gateless per
  `workbench-possessor-noun`'s F2 landing, so a possessor-based refusal
  would contradict that ruling rather than extend it. Re-probed the
  existing turn-part pins (`ProofsTurn.idr`'s `badPluralPartWindow`,
  `badDurationEndEachPlayers`, `badDurationEndAnOpponent`,
  `badDeicticTurnWithoutIntroducer`, and the rest) by rebuilding clean —
  all still refuse for their recorded reasons since no existing
  `turnPartIx` value moved.
- CombatDamage residue: the entire corpus's only use of "combat damage
  step" (Angus Mackenzie, Berserk, Blood Frenzy — all 3 supported cards
  mentioning it) is "only **before** the combat damage step," which needs
  a generic before-a-point `Timing` constructor; today only the
  special-cased `Timing.BeforeAttackersDeclared` exists. No supported card
  uses "during"/"beginning of" the combat damage step. Building a generic
  before-point constructor is mechanism, not rows, so `CombatDamage` has
  no bench witness — the row is in `Words.turnPartIx` and typechecks via
  the same `windowOk`/`partTriggerOk` path as the other three, but nothing
  in the bench exercises it yet.

## Landing record

- Construction count: `Words.TurnPart` 11 → 15 constructors;
  `Words.turnPartIx` 11 → 15 rows; `Events.partTriggerOk`/`partAddable`
  unchanged (already total over the new constructors); `Cards.Turn` +3
  bench entries (`teleport`, `dazzlingBeautyCastRestriction`,
  `thawingGlaciers`).
- Gates (all foreground, from `idris/`): clean `rm -rf build &&
  ./scripts/build`, last line `44/44: Building Cards (src/Cards.idr)`, 0
  `Error` and 0 `Warning` lines (re-ran foreground per the coordinator's
  resume instruction and confirmed the same log). From the workspace root:
  `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`; `cargo xtask cite check` → `checked 18185
  citations against cr.txt (eff. 2026-08-07); 0 stale`; `cargo xtask cite
  audit --diff` → `audited 3 citation site(s) — read each rule text
  against its claim`, all three ([CR#508.1,509.1], [CR#510.1], [CR#514.1])
  read against their claims and on-topic. No `bless` needed — all four
  rule numbers were already registered in `cr-citations.lock`.
- Assurance: restored 0; re-spelled 0; ignored 0; added 3
  (`teleport`, `dazzlingBeautyCastRestriction`, `thawingGlaciers`); removed
  0.
- Deviations and additions:
  - `dazzlingBeautyCastRestriction` spells only Dazzling Beauty's cast
    restriction, not the full printed card. Its second sentence ("Target
    unblocked attacking creature becomes blocked") needs an instruction
    that makes an attacker *become blocked* — the bench only has
    `Triggers.BecomesBlocked` (the trigger event), not the effect — and
    its third sentence needs a "next turn's" `TurnRef` noun that does not
    exist (`Words.TurnRef` only has the anaphoric `ThatTurn` read). Both
    are out of this ticket's rows-only scope; recorded here rather than
    silently dropped. This follows the file's existing convention of
    standalone `Ability` fragments for cards whose other clauses aren't
    yet spellable (`hammerOfBogardan`, `staffOfNin`, etc. are the same
    shape).
  - `CombatDamage` has no bench witness (see As landed) — a deliberate
    scope line, not an oversight; a follow-up ticket to generalize
    `Timing.BeforeAttackersDeclared` into a before-any-part constructor
    would close it.
  - No pin added (see As landed for the CR reasoning).
- STOPs: none. The clean build was left running in the background when
  the round's turn ended before commit; the coordinator's resume message
  asked for a foreground re-run before finishing, which is the gate log
  recorded above.
