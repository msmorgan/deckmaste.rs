---
needs: []
---
**Delete the clause-order witnesses: `Continuously` and `Conditionally` fix
static-first, the word-order macros re-thread the other order, and the bench
carries no implicit handles at all.** Fresh workbench review 2026-09-03, F13,
resolved by ruling.

**Ruling (settled 2026-09-03): clause order is not a slot.** A static clause
and its condition are threaded in one fixed order —
`Effect.StaticEffect.Conditionally` and `Effect.Continuously` thread
static-first — and the printed orders that put the condition first are spelled
by the word-order macros (`throughout`, `onlyWhile`), which re-thread on the
way in. `Effect.StaticThreads` (`Effect.idr:320–323`: `CondFirstDone`,
`StaticFirstDone`), `Effect.SpanStaticThreads` and their `%hint`/`[noHints]`
pairs go. `Effect.OnlyIf`/`Effect.If` are already the constructor mechanism
and stay as they are; it is the witness mechanism that is being removed.

The bench pays for the witnesses in visible handles: `{ts = StaticFirstDone}`
appears 127 times across twelve `Cards/*.idr` families (Damage 24, Choice 19,
Deontic 18, Static 15, Cost 15, Mana 13, Faces 8, Keyword 5, Turn 4, Counters
3, Trigger 2, Copy 1). All 127 collapse to the bare constructor row.

**The braces lint (ruling 11).** With `{ts}` gone, finish the job: the 35
`{bs = …}` annotations disappear once the witness types they disambiguate
carry concrete indices, and the remaining one-offs — `{gm}` 6, `{wf}` 3,
`{nz}` 3, `{st}` 2, `{k}` 1 — go through wrapping macros per
`docs/decisions/card-authoring-binds-no-implicits.md`. Then add a one-line
lint to `idris/scripts/build` that refuses any `{name = ` in
`src/Experimental/Cards/*.idr`, the way that ADR's `grep -cE` already binds
`Cards.idr`. `Proofs*.idr` are exempt — a pin's whole subject is the proof
term it refuses.

Size: S–M for the witness removal; the bench edit is mechanical and large.

Done when: `StaticThreads`, `SpanStaticThreads` and every `%hint`/`[noHints]`
pair for them are gone from the tree; `grep -E '\{[a-zA-Z_]+ *= '
idris/src/Experimental/Cards/*.idr` is empty and the build script fails when
it is not; a condition-first printed sentence (`onlyWhile`) and a
static-first one are both typechecking bench witnesses; the `ProofsDeontic`
pins that bound `{ts = StaticFirstDone}` are re-spelled against the bare row
and still refute, probed non-vacuous; the build is 44/44 with 0 errors and 0
warnings. Standard constraints apply, plus the RON-shaped constraint: a core
constructor is admissible only if the RON re-emitter can produce it from a RON
node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- **Witness deletion.** `Effect.Static.StaticThreads` (`CondFirstDone`,
  `StaticFirstDone`) and its `%hint conditionFirstThreads` are gone, as are
  `Effect.SpanStaticThreads` (`StaticFirstDone`, `SpanFirstDone`) and its
  `%hint continuousStaticFirstThreads`; both `[noHints]` data declarations went
  with them. `Conditionally` and `Continuously` now thread static-first by
  construction:
  `Conditionally (se : StaticSpec bs) (c : Condition (staticIntro se)) (marking)`
  and `Continuously (se : StaticSpec bs) (span : Maybe (Duration (staticIntro se)))`.
  `Conditionally`'s positional order swapped to match its threading (the
  dependent argument cannot precede the one it reads), and
  `staticIntro (Conditionally se c _)` is now `condIntro c` — the last-threaded
  clause, as before.
- **Macro re-threading.** `Macros.onlyWhile` / `onlyUnless` / `onlyIfSo` and
  `Macros.throughout` are the surviving word-order macros; all four lost their
  threading handles and now permute only argument order into the fixed core row.
  `Macros.asLongAs` and `Macros.unlessSo` are deleted: with the threading fixed
  static-first, a condition-first *argument* order is not expressible (the
  condition's type mentions the static clause), so they had become exact
  aliases of `onlyWhile` / `onlyUnless`. Their 36 bench sites are re-spelled
  through the survivors.
- **Brace counts, before → after** (`grep -E '\{[a-zA-Z_]+ *= ' src/Experimental/Cards/*.idr`),
  by family: `{ts}` 127 → 0, `{bs}` 35 → 0, `{gm}` 6 → 0, `{wf}` 3 → 0,
  `{nz}` 3 → 0, `{st}` 2 → 0, `{k}` 1 → 0. Total 177 → 0.
  - The 127 `{ts = StaticFirstDone}` collapsed to the bare row.
  - The 2 `{st = Static.CondFirstDone}` sites (Tymaret's knight line in
    `Choice.idr`, Darkblade Agent in `Keyword.idr`) are re-spelled as bare
    static-first `Conditionally` rows and are the ticket's static-first bench
    witnesses; `Macros.onlyWhile` at 38 sites is the condition-first one.
  - The 35 `{bs = …}` and the `{k = Object}` one-off went by giving each
    witness a concrete type: 26 new named declarations across `Anaphora`,
    `Description`, `Cost`, `Choice`, `Piles`, `Trigger` and `Turn` (e.g.
    `Anaphora.nekrataalEntry : GameEvent []`, `Anaphora.scapeshiftSearch :
    Instruction Anaphora.scapeshiftSacrificed`, `Turn.finalFortuneExtraTurn :
    Instruction []`), so the index is read off the declared type instead of a
    handle.
  - The 6 `{gm}` / 3 `{nz}` / 3 `{wf}` went through macros: three
    `SomeOf (CountedSlice …) Nothing` sites now call the existing
    `Macros.someOf`, and the three `SomeOf WholeSlice (Just p)` sites call a
    new `Macros.allAmong descr grp`. The `{k = Object}` site calls a new
    `Macros.everyObject`.
- **The lint.** `idris/scripts/build` now runs
  `grep -nE '\{[a-zA-Z_]+ *= ' src/Experimental/Cards.idr src/Experimental/Cards/*.idr`
  before `idris2 --build` and exits 1 with `bench binds an implicit handle: use
  a macro` on any hit, printing the offending lines. `Proofs*.idr` are not in
  the glob. Probed non-vacuous: a planted `-- {probe = 1}` in `Cards/Turn.idr`
  made the gate exit 1 and name the line; removing it restored exit 0.

## Landing record

- Build gate: `cd idris && ./scripts/build` — clean build (`build/` removed),
  `44/44: Building Cards (src/Cards.idr)`, exit 0, 0 Error lines, 0 Warning
  lines, lint green.
- Witness grep: `grep -c 'StaticFirstDone\|CondFirstDone\|SpanFirstDone'
  idris/src/Experimental/*.idr idris/src/Experimental/Cards/*.idr` — 0 in all
  38 files.
- Brace grep: `grep -E '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards/*.idr`
  — empty.
- Citation gates: `cargo xtask cite check --list-noncompliant` — the 2
  non-compliant strings both sit in `docs/tickets/done/ability-kind-taxonomy.md`,
  untouched by this round; 0 in this diff. `cargo xtask cite check` —
  `checked 18218 citations against cr.txt (eff. 2026-08-07); 0 stale`.
  `cargo xtask cite audit --diff < /tmp/coc.diff` —
  `audited 0 citation site(s) — nothing selected` (this round adds and removes
  no citations).
- Pin probes (mis-stated once, message changed, then restored):
  `ProofsStatic.badPtDefinitionClause` (`ClauseStatic` through `Continuously`),
  `ProofsStatic.badUnlessConjunction` and `ProofsDeontic.badUnlessOnPositive`
  (`MarkingOk` through `Conditionally`), `ProofsDeontic.badCantBeAttacked`
  (`DeedFits` through `Continuously`), and
  `ProofsKeyword.badEmptyKeywordList` (over a `Conditionally`) each reported
  `… is not a valid impossible case.` when the obligation was made satisfiable.
  The re-spelled `ProofsStatic.badThatCreatureIsCondSubject` was probed through
  its twin: swapping `okGetsBattlefieldSubject`'s subject to
  `Macros.That (TypeW Creature) OneOf` fails with
  `Can't find an implementation for countReach (Word (TypeW Creature)) OneOf [] = 1`,
  locating the refusal at the anaphor.
- Assurance: restored 0; re-spelled 39 bench sites + 12 pin/witness sites;
  ignored 0; added 0 tests (3 macros and 26 typed witnesses, no assertions);
  removed 0 tests or pins.
  - 36 of the 39 bench re-spellings are pure argument-order swaps through
    `onlyWhile` / `onlyUnless`; 2 are the bare `Conditionally` rows; 1 is
    Gideon Jura through `throughout`.
  - Pin/witness re-spellings: `ProofsStatic.okGetsBattlefieldSubject` and
    `badThatCreatureIsCondSubject`, `okUnlessOverNegatedCondition`,
    `badUnlessConjunction`; `ProofsDeontic.okUnlessOnNegated`,
    `badUnlessOnPositive`; the four `ProofsKeyword` `AlsoForKeywords` terms;
    `ProofsAnaphora.onlyWhileThreadsPrefix`; `ProofsPiles.nestedStaticConditionals`
    (its five brace handles collapsed to a bare nested pair).
- Deviations and additions:
  - **Deleted `Macros.asLongAs` and `Macros.unlessSo`** (not named by the
    ruling, which names only the survivors). Under fixed static-first threading
    the condition's type mentions the static clause, so no macro can take the
    condition first; both became type- and body-identical to `onlyWhile` /
    `onlyUnless`, which is the duplicate-row shape `workbench-order-twin-macros`
    already ruled out. The condition-first printed order is now spelled by
    `onlyWhile` / `onlyUnless`.
  - **Seven cards lost an anaphoric spelling** to the fixed order; each is
    re-spelled with the same asserted outcome by naming the antecedent
    directly, because under static-first nothing precedes the static clause
    that could bind it:
    Adanto Vanguard, Frodo Baggins, Enkira Hostile Scavenger and Chillerpillar
    (`Macros.It OneOf` → `Macros.thisCreature`); Scrounged Scythe
    (`Macros.It OneOf` → `AttachHost Equipped (TypeW Creature)`); Conspicuous
    Snoop and Skill Borrower (`Macros.That CardW OneOf` →
    `Macros.topSlice (Lit 1)`).
    Gideon Jura's +2 moved its target announcement out of the duration and into
    the noun phrase — `deontic (allOf (And [creature, HasPossessor ControllerAx
    (target Opponent)])) …` with span `DuringNextTurnOf (That PlayerW OneOf)` —
    the only bench use of the deleted span-first threading. This is the ruling's
    cost, recorded here rather than resolved: a condition or duration can no
    longer bind an antecedent its clause reads.
  - **Three new macros** in `Macros.idr`: `allAmong descr grp`
    (`SomeOf WholeSlice (Just descr) grp`, filling `{gm}`), `everyObject`
    (`allOf (And [])` at `Noun bs Object`, fixing the `{k}` index), and the
    reimplemented `throughout se span` (now `Continuously se (Just span)`; it
    keeps the printed word order in its name and saves the `Just`, but no
    longer re-threads bindings — the ruling removed the threading it permuted).
  - **The lint covers `Cards.idr` as well as `Cards/*.idr`.** The ticket asks
    only for the directory; `card-authoring-binds-no-implicits` already binds
    the re-export file at 0 with no enforcement, so it is in the same glob.
  - 26 concretely-typed witness declarations added to the bench so the `{bs}`
    handles could go; they carry no new assertions.
- STOP: none taken. The seven refused anaphoric spellings above were judged
  applications of ruling §4 rather than a contradiction with it — the ruling
  removes the threading choice knowingly ("the sentence gives one order"), and
  every affected card re-spells with the same meaning and still typechecks. No
  card became unspellable.
