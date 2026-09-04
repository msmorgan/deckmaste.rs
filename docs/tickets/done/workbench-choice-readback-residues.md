---
needs: []
---
**Close the three readback gaps the choice-scope round left.** Residue of
`workbench-choice-scope-residues` (2026-09-04): Stick Together now reads
its printed distributive shape and "this way" excludes every standing
choice, but two printed cards stayed unspellable.

- **"Starting with you."** Disciple of Caelus Nin orders a distributive
  choice; only `Effect.Vote` has a `first` slot, `Choose` has none. Add the
  positional slot (RON-shaped, `Nothing` for the unordered case, every
  existing site takes `Nothing`), bench Disciple of Caelus Nin, pin an
  order on a singular chooser [CR#101.4].
- **Whether a choice was made.** No `Phrase.Condition` reads back that a
  choice happened (`Exists` wants a description, `Happened` wants an event
  and `Choose` stamps no `VerbedAct`). Decide whether `Choose` stamps a
  `VerbedAct` so `Happened` reads it, or a condition over the standing
  choice count; keep it positional.
- **Own-stat comparison.** "with equal or lesser power" needs a
  `CompareOver` read of the compared object's own stat with no macro;
  add the macro (one lemma) and bench Sculpted Sunburst once the second
  bullet lands.

Size: M. Done when: Disciple of Caelus Nin and Sculpted Sunburst are
benched in full; pins probed non-vacuous; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## As landed

- **"Starting with you" — closed.** `Effect.Choose` gains a positional first
  slot, `first : Maybe (Noun bs Player)`, mirroring `Effect.Vote`'s `first`,
  gated by the new named witness `Phrase.ChoiceOrder first by`: `Unordered`
  (any chooser, `first = Nothing`) and `RoundStartsWith` (`nounPlur by =
  ManyOf`). All 25 existing `Choose` sites take `Nothing` (scripted sweep);
  `Macros.choose`, `chooses`, `secretlyChooses` and `proliferate` keep their
  surfaces and discharge `Unordered` by search. Bench: **Disciple of Caelus
  Nin** (`Cards/Choice.idr`, `discipleOfCaelusNin`), the whole printed card —
  ordered distributive choice, the `NotChosen` phase-out, and the
  "Permanents can't phase in" static. Twin `ProofsChoice.okChoiceStartingWithYou`;
  pin `badOrderedSingularChooser` ("starting with you, target player chooses a
  creature" — one chooser, so no order to fix [CR#101.4]).
- **Whether a choice was made — read decided and landed as a standing-choice
  condition, not a `VerbedAct` stamp.** `Phrase.Condition.ChoseThisWay who p`
  gates on the same `ChoiceInScope k bs` (`ChoicesStand`) that
  `Predicate.NotChosen` gates on. The evidence against the event route: (a)
  `Happened`'s `LookbackClause` carries no obligation over `bs` at all — its
  two gates (`LookbackSubject`, `ComplementWritten`) are facts about the event
  vocabulary — so a `VerbedAct "Choose"` reading would typecheck with no
  choice standing, which is exactly the sentence this bullet must refuse
  (`ProofsAnaphora.okShuffleLocusAtLibrary` typechecks with no shuffle in
  scope, and is the shape of that hole); (b) Sculpted Sunburst writes the
  positive and the negative "this way" in consecutive clauses, so they must
  read the same manner, and `NotChosen` already reads the standing choices;
  (c) `Choose` is not a deed — it stamps no `Verbed` reach and is not an
  `Enact` — so the event route would first have to make "Choose" an act label
  and thereby open `happened`, `eventCount`, `eventCountInvolving` and the
  `Verbed`/`Stamped` reaches on a verb no printed card reads back as an event.
  Twin `ProofsChoice.okChoseThisWayAfterChoice`; pin
  `badChoseThisWayWithoutAChoice` (the readback with no choice in scope).
- **Own-stat comparison — closed.** One macro, `Macros.comparesOwnStat c dom r
  bound`, spells the `CompareOver` whose measure is the compared object's own
  `StatOf c` read (`Own Bare OneOf [bindFor TheD OneOf ph dom] (predDelta dom
  ++ bs)` — the `Own Bare OneOf` idiom `Macros.dealsDamageOwnPower` uses,
  moved onto `CompareOver`'s domain binding), carrying the two erased
  obligations (`countReach Bare OneOf` over that binding, and
  `statHeadTysOk`). Bench: **Sculpted Sunburst** (`Cards/Choice.idr`,
  `sculptedSunburst`), the whole printed card — the two choosers, "with equal
  or lesser power" through the new macro, and "if you chose a creature this
  way" through `ChoseThisWay`.
- **Nothing left undone.** Both cards are benched in full; no clause of either
  is dropped or paraphrased.

## Landing record

Measured on change `wwzussxs`, against parent `mqwluwrs`
(`kata: claim workbench-choice-readback-residues`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructor rows (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`; lines matching `^ {4,}[A-Z]\w* :`) | 538 | 541 |
| pin declarations (`^\w+ : Unspellable` in `Proofs*.idr`) | 617 | 619 |
| `: Card` bench witnesses (`Cards/*.idr`) | 809 | 811 |
| act labels (`MkActFacts "` in `Words.idr`) | 87 | 88 |
| `Eq` instances (`check-eq-indexes`) | 43 | 43 |
| diffstat (`idris/`, ticket excluded) | — | 10 files, 148 insertions, 32 deletions |

The three new constructor rows are `ChoiceOrder`'s `Unordered` and
`RoundStartsWith` and `Condition.ChoseThisWay`. No noun, verb, predicate or
instruction row was renamed or deleted; `Choose` gained a slot and one erased
obligation, and no other core row changed.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, **1m40.912s** wall on a clean tree (the claim
  commit's baseline clean run was 2m01.282s on the same host);
  `grep -cE 'Error|Warning'` over the log → `0`; bench brace lint green.
- `idris/scripts/check-eq-indexes` → `Eq instances checked: 43 index-based or
  derived`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14227 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` — **not run**: the only rule this round cites,
  [CR#101.4], is already registered in `cr-citations.lock` by name and
  `cite check` reports 0 stale. `cr-citations.lock` is unmodified.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 5 citation site(s)`. All five cite [CR#101.4]
  and all five read with it: the rule conditions on "if multiple players would
  make choices" and then fixes the turn order they choose in, which is what
  "starting with you" names (Disciple's docstring, `ChoiceOrder`'s docstring,
  `okChoiceStartingWithYou`) and what a single chooser has none of
  (`badOrderedSingularChooser` — the rule does not apply, so the order is
  vacuous, and the cite points at the refusal). The fifth is Sculpted
  Sunburst's docstring, where [CR#101.4]'s own example ("Then all creatures **chosen
  this way** are sacrificed simultaneously") is the CR reading one "this way"
  back over several players' choices — the same cite `Phrase.ChoiceInScope`
  already carries for the negative form.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 on the claim commit).
- re-spelled: 0. No pin's subject was retired; the 25 `Choose` sites took a new
  positional `Nothing` and kept their sentences, spellings and outcomes.
- ignored with a blocker: 0.
- added: 2 pins (`badOrderedSingularChooser`, `badChoseThisWayWithoutAChoice`),
  2 twins (`okChoiceStartingWithYou`, `okChoseThisWayAfterChoice`), 2 bench
  cards (`discipleOfCaelusNin`, `sculptedSunburst`), 1 macro
  (`Macros.comparesOwnStat`), 1 named witness (`Phrase.ChoiceOrder`), 1
  `Condition` row (`ChoseThisWay`), 1 act-facts row (`"Phase In"`).
- removed: 0.
- pin non-vacuity probes: **2 of 2**, each mis-stated once and restored.
  `badOrderedSingularChooser` with the chooser made distributive
  (`Macros.each AnyPlayer`) → `badOrderedSingularChooser Refl is not a valid
  impossible case.`; `badChoseThisWayWithoutAChoice` with one choice put in
  scope ahead of the condition → `badChoseThisWayWithoutAChoice ChoicesStand is
  not a valid impossible case.`

**Probes (scratch module under `src/Experimental/`, deleted after each run)**

- Disciple's trigger body, its full card, and the "Permanents can't phase in"
  static each typecheck standing alone, with no implicit handle at any slot —
  `RoundStartsWith` and the seven `Deontic` obligations are all found by
  search.
- Sculpted Sunburst typechecks with the comparison written two ways — the
  own-stat `CompareOver` as one conjunct beside "creature they control", and
  (the spelling landed) with "creature they control" as the `CompareOver`
  domain. The landed one is the printed reading.
- The first shape tried for the `first` slot re-indexed the chooser at
  `agentCtx first`, mirroring `Vote`'s `voters : Noun (agentIntro first)
  Player`. It builds, but at a bench site the chosen noun's context stops
  reducing: "each player chooses up to five permanents **they** control" fails
  with `Can't find an implementation for countReach (Word PlayerW) OneOf
  (agentCtx (Just (each AnyPlayer))) = 1`, and supplying `{od}` by hand then
  fails at `Can't solve constraint between: ?bs and agentDelta ?first ++ ?bs`.
  See deviation 1.

### Deviations and additions

1. **`first` does not re-index the chooser** (`by : Maybe (Noun bs Player)`,
   unchanged), where `Vote`'s `voters` sits at `agentIntro first`. The probe
   above is the mechanical reason; the reading reason is that no printed
   "starting with" card describes its choosers relative to the ordering player
   — "starting with you, each player chooses" names an APNAP seat, not an
   antecedent. Keeping `by` at `bs` also leaves `chooseIntro`, `chooseAnn`,
   `chosenIntroBy`, `chosenAnnBy` and `ChoiceClause` untouched, so every
   existing choice read is unchanged by construction rather than by argument.
2. **One new act label, `"Phase In"`** (`Words.actFacts`), an intransitive
   battlefield deed with the permanent as its bare agent — what Disciple's
   second line, "Permanents can't phase in", forbids [CR#702.26c]. It is a
   vocabulary row, not a guard, and `Experimental.Words` feeds no emitted
   table (`EmitTables` reads `Semantics.idr`), so nothing outside `idris/`
   moves. Without it the card cannot be benched in full, which the ticket
   requires; the alternative was to bench a card the printed text does not
   read.
3. **`ChoiceOrder` is a named `data` witness**, not a `So (…)` gate: VERIFY.md
   prefers one where the refusal message carries the meaning, and
   `RoundStartsWith`/`Unordered` name the two readings in the message.
4. **No macro for the ordered choice.** `Macros.vote` covers only `Vote`'s
   unordered form and `ProofsPiles.voteStartingWithSpecifiedPlayer` writes the
   ordered one with the raw constructor; the same is done here, which avoids
   a `chooses`/`choosesStartingWith` arity pair. Both bench sites carry no
   implicit handle, so the lint is satisfied.
5. **Disciple's static is a raw `Deontic … Agent`.** `Macros.objectCant` is
   Patient-role and `Macros.playerCant` is Player-role; a third role macro
   would be the pair the house rules refuse, and raw `Deontic` at a bench site
   is the established idiom (`Cards/Deontic.idr` writes nine of them).
6. **`condDelta (ChoseThisWay who _) = selfSubjDelta who`**, matching
   `Happened`, its nearest sibling that also names a subject; it is `[]` for
   every Player noun but "enchanted player".
7. **One citation dropped, not added.** `okChoseThisWayAfterChoice` first
   carried `[CR#101.4]`; it is a single-chooser witness and that rule conditions on
   several players choosing, so the cite pointed away from its claim and was
   removed rather than restated.

**STOP taken**

None.
