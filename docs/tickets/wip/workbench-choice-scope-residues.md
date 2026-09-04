---
needs: []
---
**Close the two choice-scope gaps agent-scoped-choice left.** Residue of
`workbench-agent-scoped-choice` (2026-09-04), which typed `Choose`'s noun at
its chooser so "each player chooses … they control" reads [CR#700.8d].

- **Distributive spend of a distributive choice.** `Macros.sacrifice (each
  AnyPlayer) (theRest Object)` is refused by `EnactKeepsOuter` because
  `TheRest` spends the shared group, yet when the spent parts are the ones
  the same distributive chooser introduced, each player spends only their
  own partition. Let that case stand (a per-agent rest, or a `KeepsOuterEach`
  case that recognises the chooser's own delta), keep the guard for a shared
  group, pin both sides. Unblocks Stick Together's printed shape (its last
  blocker) and Disciple of Caelus Nin.
- **"Not chosen this way" after several standing choices.** `NotChosen`'s
  `ChoiceInScope` is counted uniqueness over one choice; cards write several
  choices under one "this way" (Sculpted Sunburst, two choices; Raiding
  Party, "by any player"). Decide the read (all standing choices of the same
  chooser, or a choice-group binding) and bench one of the two. Celestial
  Judgment additionally needs `ForEachKindOf` to publish its in-loop choice
  outward (`instrIntro (ForEachKindOf …) = bs` today); record whether that
  is a loop-delta gap or a separate shape.

Size: M. Done when: Stick Together is benched in its printed shape; one of
Sculpted Sunburst or Raiding Party is benched; pins probed non-vacuous; build
at its module count. Standard constraints apply, including the RON-shaped
constraint.

## As landed

- **Distributive spend of a distributive choice — closed.** `Effect.KeepsOuterEach
  ManyOf` is now the named witness `Effect.EachStackOk`, with two constructors:
  `EachOnlyAdds` (the old `KeepsOuterOf`, unchanged) and `EachClosesOwnParts`,
  which admits a deed whose output is `Words.partsClosed outer` — the agent
  stack with every standing partitive re-determined from `PartD` to `TheD` and
  **nothing dropped** — provided `Words.partsDistributed outer` (every standing
  partitive is plural, the shape `chosenIntroBy ManyOf`'s `pluralizeDelta`
  stamps on a distributive chooser's own delta). So "each player … sacrifices
  the rest" stands exactly when the parts it closes are the ones a distributive
  chooser published; a shared group binding would be dropped by `groupSpent`,
  which breaks the equation, and a singular chooser's part is `OneOf`, which
  breaks `partsDistributed`. `Effect.doesInstrIntro` gains one clause,
  `Move (TheRest _ _)` under a `ManyOf` agent, publishing
  `afterMoveTo to (partsClosed (nomIntro s))`: without it `distributedDelta`
  would take 0 and republish the un-spent stack, leaving the rest spellable a
  second time. `ProofsZone.badDistributedZoneMoveRead` still refutes (re-spelled
  to two impossible clauses, one per constructor, and re-probed).
  Bench: **Stick Together** (`Cards/Choice.idr`, `stickTogether`) is re-spelled
  to its printed distributive shape — four flat `Macros.chooses (Macros.each
  AnyPlayer) (Macros.counted (Macros.upTo 1) (… HasPossessor ControllerAx
  They))` roles and `Macros.sacrifice (Macros.each AnyPlayer) (Macros.theRest
  Object)`, no `ForEachOf`. Twin `ProofsChoice.okDistributedRestOfOwnChoice`;
  pins `badDistributedRestOfSharedGroup`, `badDistributedRestOfSingularChoice`,
  `badDistributedRestDisposedTwice`.
- **Not done: Disciple of Caelus Nin is not benched.** It is supported and in
  the corpus, but its printed first clause is "**starting with you**, each
  player chooses up to five permanents they control", and "starting with" is a
  slot only `Effect.Vote` has (`first`); `Choose` has none. Its second line
  ("Permanents can't phase in") is a further static. Dropping either would make
  the witness read a different card, so it is left alone. Phasing itself is
  supported (`SetStatus PhasedOut`), so the ordering slot is the only new shape.
- **"Not chosen this way" over several standing choices — read decided and
  landed.** `Phrase.ChoiceInScope`'s single constructor is renamed
  `OneChoiceStands` → `ChoicesStand` and its gate goes from counted uniqueness
  (`countParts k bs = 1`) to "at least one stands"
  (`So (not (countParts k bs == Z))`): **"this way" names the manner, so it
  reads back every standing choice of a compatible kind as one exclusion.**
  `Phrase.Predicate.NotChosen` is untouched — same positional shape, same single
  erased `cs` obligation. Evidence for the read, from the 24 supported cards
  whose text says "chosen this way": [CR#101.4]'s own example reads several
  players' choices back under one "chosen this way"; [CR#700.8d] makes Stick
  Together's one "choose a party" instruction four choices read back by one
  phrase; Call to the Void (two choices, one chooser) and Grenzo's Rebuttal
  (three) both read every standing choice back under one "chosen this way"; and
  no printed card writes the phrase meaning only one of several standing
  choices. A choice-group binding would therefore be a construct with no printed
  evidence.
  Witnesses: `ProofsChoice.okNotChosenAfterTwoChoices` (the re-spelled
  `badNotChosenAfterTwoChoices`, same sentence, now admitted) and
  `okNotChosenAcrossChoosers` (Sculpted Sunburst's exclusion, two choosers).
  Pin: `badNotChosenWithoutAChoice` (no choice stands — the CR-meaningless
  exclusion) re-spelled to `ChoicesStand` and re-probed.
- **Not done: neither Sculpted Sunburst nor Raiding Party is benched.** STOP
  below.
- **Celestial Judgment's `ForEachKindOf` gap is a loop-delta gap, not a separate
  shape.** `ForEachKindOf` already carries the same `{auto 0 ko : KeepsOuter
  body}` obligation `ForEachOf` does, and `instrDelta` is stated over the body's
  own index, so `instrIntro (ForEachKindOf …) = pluralizeDelta (instrDelta body)
  ++ bs` is `ForEachOf`'s rule applied verbatim — one line, same premise, no new
  vocabulary. Not widened here, per the ticket. Note that Celestial Judgment
  then needs nothing from this ticket's second bullet either: its loop publishes
  exactly one choice, which the old counted-uniqueness gate already admitted.

## Landing record

Measured on change `qvouonmt`, against parent `yrzkuxmu`
(`kata: claim workbench-choice-scope-residues`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructor rows (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`; lines matching `^ {4,}[A-Z]\w* :`) | 536 | 538 |
| `Unspellable` pin declarations (`Proofs*.idr`) | 626 | 628 |
| `: Card` bench witnesses (`Cards/*.idr`) | 809 | 809 |
| `Eq` instances (`check-eq-indexes`) | 43 | 43 |
| diffstat (`idris/`, ticket excluded) | — | 6 files, 125 insertions, 31 deletions |

The two new constructor rows are `EachStackOk`'s `EachOnlyAdds` and
`EachClosesOwnParts`; `ChoicesStand` replaces `OneChoiceStands` one for one. No
noun, verb, predicate or instruction row was added, renamed or deleted, and no
macro changed its surface.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, **1m39.802s** wall on a clean tree (a second clean
  run mid-round measured 2m04.155s; the workspace's first build, on the claim
  commit with a cold cache, was 3m06.362s — the spread is host load, not this
  change); `grep -cE 'Error|Warning'` over the log → `0`; bench brace lint
  green.
- `idris/scripts/check-eq-indexes` → `Eq instances checked: 43 index-based or
  derived`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14222 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` — **not run**: all three rules this round cites
  ([CR#101.4], [CR#700.8d], [CR#701.21a]) are already in `cr-citations.lock`
  (verified by name) and `cite check` reports 0 stale. `cr-citations.lock` is
  unmodified.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff <
  /tmp/round.diff` → `audited 8 citation site(s)`. [CR#700.8d] ("each player
  chooses up to one creature **they control** of that type") is what makes the
  four roles a per-player partition, so it is the rule the per-agent rest rests
  on and the rule under which one instruction is several choices. [CR#701.21a]
  ("A player can't sacrifice … something that's a permanent they don't
  control") is exactly why a distributive sacrifice may not spend a shared group
  or a singular chooser's leftover — it points the direction both pins cite it
  for. [CR#101.4]'s example ("Each player sacrifices a creature. … Then all
  creatures **chosen this way** are sacrificed simultaneously") is the CR
  reading one "this way" back over every player's choice, which is the read this
  round lands.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 on the claim commit).
- re-spelled: 3. `ProofsZone.badDistributedZoneMoveRead` — the obligation it
  refutes is now a two-constructor data type, so it has two impossible clauses;
  same sentence, same asserted outcome. `ProofsChoice.badNotChosenWithoutAChoice`
  — `OneChoiceStands` → `ChoicesStand`; same sentence, same asserted outcome.
  `ProofsChoice.badNotChosenAfterTwoChoices` → `okNotChosenAfterTwoChoices` —
  the ticket deliberately retires this refusal, so the same sentence is
  re-spelled as the positive witness of the shape that replaces it, not deleted.
- ignored with a blocker: 0.
- added: 3 pins (`badDistributedRestOfSharedGroup`,
  `badDistributedRestOfSingularChoice`, `badDistributedRestDisposedTwice`),
  2 witnesses (`okDistributedRestOfOwnChoice`, `okNotChosenAcrossChoosers`),
  1 stack helper (`afterDistributedRestSacrificed`). No bench card added;
  `stickTogether` was re-spelled in place.
- removed: 0.
- pin non-vacuity probes: **5 of 5**, each mis-stated once and restored.
  `badDistributedRestOfSharedGroup` with the `tap (allOf creature)` clause
  dropped → `badDistributedRestOfSharedGroup EachClosesOwnParts is not a valid
  impossible case.`; `badDistributedRestOfSingularChoice` with the chooser made
  distributive → `… EachClosesOwnParts is not a valid impossible case.`;
  `badNotChosenWithoutAChoice` given one standing choice → `… ChoicesStand is
  not a valid impossible case.`; `badDistributedZoneMoveRead` with `Macros.a
  Macros.creature` for `It OneOf` → `… EachOnlyAdds is not a valid impossible
  case.`; `badDistributedRestDisposedTwice` pointed at the stack **before** the
  sacrifice → `… Oh is not a valid impossible case.`

**Probes (scratch module under `src/Experimental/`, deleted after each run)**

- Stick Together's printed shape (four flat roles + the distributive rest)
  typechecks in 2.8 s; before the change it fails at
  `EnactKeepsOuter` with `Mismatch between: PartD and TheD` — the whole refusal
  was the determiner change, lengths already equal.
- "Tap all creatures. Each player chooses … then sacrifices the rest." and
  "Choose up to one creature. Each player sacrifices the rest." both refuse at
  `Can't find an implementation for EachStackOk …`.
- Sacrificing the distributive rest twice refuses at
  `theRestFits`/`countParts … == 0` — the parts are closed, so no rest stands.
- Sculpted Sunburst's exclusion over two choosers ("Choose a creature you
  control, then each opponent chooses a creature they control. Destroy each
  creature not chosen this way.") typechecks; under the old
  `countParts k bs = 1` gate it is the shape `badNotChosenAfterTwoChoices`
  pinned.

### Deviations and additions

1. **`Words.partsClosed` and `Words.partsDistributed`** are new projections
   beside `groupSpent`. `partsClosed` is the drop-nothing half of `groupSpent`,
   stated independently so the admissible spend can be named without a kind;
   `partsDistributed` is the "the chooser's own delta" test. Neither is a
   constructor.
2. **`Effect.KeepsOuterEach ManyOf` is a named data witness, not an equality.**
   VERIFY.md prefers a named witness where the refusal message carries the
   meaning, and the alternative (`Either` of two equations) reads worse and
   messages worse. The cost is that a pin refuting the obligation needs one
   impossible clause per constructor — `badDistributedZoneMoveRead` is the only
   such pin in the tree.
3. **`doesInstrIntro` gains a `TheRest` clause.** Admitting the shape at the
   type level is not enough on its own: `distributedDelta` takes
   `length out - length (agentIntro s)` = 0 for a rest (the spend is
   length-preserving), so the un-spent stack would be republished and "the rest"
   would stay spellable — exactly where `badChoiceRestDisposedTwice` refuses it
   undistributed. `badDistributedRestDisposedTwice` is the pin that holds the
   fix in place. The sibling projections (`doesPreIntro`, `doesRiderIntro`,
   `doesAnnIntro`) are deliberately left alone: `pre` is the stack before the
   deed, and `ann` is what a simultaneous sibling sees, neither of which the
   spend has reached.
4. **`ChoiceInScope`'s constructor is renamed**, not added to. "One choice
   stands" is no longer what the witness says, and a stale name in a refusal
   message is worse than a re-spell; the two uses are both in `ProofsChoice`.
5. **Two additions beyond the ticket's letter.**
   `badDistributedRestDisposedTwice` (deviation 3 above — the new publishing
   rule needs a pin or it is unwitnessed) and `okNotChosenAcrossChoosers` (the
   house fallback for the bench the STOP below could not deliver: a
   rules-meaningful synthetic sentence in the matching `Proofs*.idr`, spelling
   Sculpted Sunburst's actual exclusion).
6. **No macro added or changed.** `Macros.sacrifice` and the other eight
   `EnactKeepsOuter`-threading macros keep their surfaces verbatim; the
   obligation's type changed underneath them and no call site in the 46-module
   tree needed an edit.
7. **`stickTogether` stays an `Instruction []`**, as `workbench-party-and-outlaw`
   landed it; only its shape changed.

**STOP taken**

**Neither Sculpted Sunburst nor Raiding Party is benched.** The gate this
bullet opens is landed and witnessed, but the card the ticket asks for cannot be
written without a shape this ticket does not name.

- Of the 24 supported cards whose text says "chosen this way", exactly three
  put more than one choice in scope of one such phrase: Sculpted Sunburst (two,
  negative), Call to the Void (two, positive) and Grenzo's Rebuttal (three,
  positive). Only Sculpted Sunburst writes the **negative** form `NotChosen`
  spells; the other two need a positive "chosen this way" predicate, which does
  not exist and which this ticket does not name.
- **Sculpted Sunburst** is blocked twice over. Its third sentence begins "**If
  you chose a creature this way**, …", and no `Phrase.Condition` reads back
  whether a choice was made: `Exists` demands a bare-plural or counted
  existential description (`existentialMention`), not a pronoun read of a
  chosen part; `Happened` needs an `Events.EventName`, and there is no choosing
  event (`Choose` is not an `Enact` and stamps no `VerbedAct` label); `Matches`
  would assert something else ("if the thing you chose is a creature"). Its
  second clause, "a creature they control **with equal or lesser power**",
  additionally needs a `CompareOver` whose measure is the domain's own
  `StatOf Power` read — writable in principle (the `Own Bare OneOf` idiom
  `Macros.dealsDamageOwnPower` uses) but with no macro, and the tree benches no
  card that compares a stat against another object's.
- **Raiding Party** is not evidence for this bullet at all: its choice sits
  inside "For each creature tapped this way, that player chooses up to two
  Plains", and `ForEachOf` publishes `pluralizeDelta (instrDelta body) ++ bs` —
  **one** pluralized part — so its "weren't chosen this way by any player" reads
  exactly one standing choice and the old counted-uniqueness gate already
  admitted it. What blocks Raiding Party is elsewhere: an agent'd "may tap any
  number of untapped white creatures they control" (`Macros.tap` takes no
  agent) and the "can't be the target of white spells or abilities from white
  sources" static.
- Resolution: the read is landed and probed, Sculpted Sunburst's exclusion is
  witnessed by `okNotChosenAcrossChoosers` in `ProofsChoice` (the house bench
  fallback), and the two remaining shapes — a condition for "if you chose … this
  way", and a `Choose` with a "starting with you" ordering slot (Disciple of
  Caelus Nin, Grenzo's Rebuttal, Druid of Purification, The Horus Heresy,
  Rejoin the Fight all print it) — are follow-ups, not silently widened here.

**Follow-up for a live ticket**

- A `Condition` that reads back whether a choice was made ("If you chose a
  creature this way"). Blocks Sculpted Sunburst, the one printed card whose
  negative "this way" spans several standing choices.
- A "starting with you" ordering slot on `Effect.Choose`, the slot `Vote`
  already has as `first`. Blocks Disciple of Caelus Nin, Grenzo's Rebuttal,
  Druid of Purification, The Horus Heresy III and Rejoin the Fight.
- `ForEachKindOf` publishing its loop delta (`pluralizeDelta (instrDelta body)
  ++ bs`, `ForEachOf`'s rule verbatim under the identical `KeepsOuter`
  obligation). Blocks Celestial Judgment.
- A positive "chosen this way" predicate. Blocks Call to the Void, Grenzo's
  Rebuttal, Druid of Purification, The Horus Heresy III, Disorienting Choice,
  Unstable Glyphbridge, Harsh Mercy, Patriarch's Bidding.
