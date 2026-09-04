---
needs: []
---
**Let a chosen noun refer to its chooser.** Residue of
`workbench-party-and-outlaw` and `workbench-party-residues` (2026-09-04).
`Effect.Choose` takes both its chooser and its noun in `bs`, so beside a
distributive chooser (`Macros.each AnyPlayer`) the chosen noun has no
antecedent for "they": "each player chooses up to one creature they control"
[CR#700.8d] fails with `countReach (Word PlayerW) OneOf [] = 1`. Stick
Together is benched as a `ForEachOf` loop for that reason even though
`Predicate.NotChosen` now exists, and the same gap blocks four of the seven
vintage-legal "not chosen this way" cards (Raiding Party, Sculpted Sunburst,
Consuming Tide, Celestial Judgment).

Fix: type `Choose`'s noun at `agentIntro chooser` (the shape `Enact` already
uses for its deed), so a distributive chooser publishes its per-player
binding to the chosen noun; keep the `Choose (UpTo 1)`-per-role structure
and `EnactKeepsOuter` untouched. Re-spell Stick Together back to its printed
distributive shape with `NotChosen`, bench one of the four blocked cards, and
re-probe the choice pins.

Size: M. Done when: Stick Together reads its printed shape; the four cards
are spellable (one benched); every `Choose` bench site still typechecks;
build at its module count. Standard constraints apply, including the
RON-shaped constraint.

## As landed

- **`Effect.Choose`'s noun is typed at the chooser.** `Choose : {k : Kind} ->
  (by : Maybe (Noun bs Player)) -> (n : Noun (agentCtx by) k) ->
  (disc : Disclosure) -> {auto 0 ch : ChoiceClause by n} -> Instruction bs`.
  The chooser moves to the first slot because the noun's index depends on it —
  the shape `Effect.Vote` already uses (`voters : Noun (agentIntro first)
  Player`) and `Enact` uses for its deed (`e : Instruction (agentCtx subj)`).
  `Phrase.ChoiceClause` is re-indexed to match (`AgentChoice`'s
  `{0 n : Noun (agentIntro by) k}`); `agentChoosable` and `choosable` are
  unchanged, so which nouns a chooser may choose is exactly as before.
- **`instrIntro`/`instrProfile` over a choice.** Four `Phrase` functions carry
  the split: `chosenIntroBy`/`chosenAnnBy` dispatch on the chooser's
  plurality, `chooseIntro`/`chooseAnn` on `Maybe`. For `Nothing` and for a
  `OneOf` chooser (`You`, `target player`, `an opponent`) both reduce to the
  old expressions verbatim — `chosenIntro n` and `nounDelta by ++ chosenIntro
  n` — so those reads are unchanged by construction, and every existing
  `chooses`/`secretlyChooses` bench site typechecked without an edit. For a
  distributive (`ManyOf`) chooser the chosen delta is pluralized over the
  restored outer stack (`pluralizeDelta (chosenDelta n) ++ nomIntro by`), the
  `distributedDelta` shape `Enact` uses. `countParts` over a choice is
  therefore unchanged in count for every pin that reads a choice back
  (`NotChosen`, `theRest`, `ProofsChoice`), pluralized only in the
  distributive case.
- **One erased obligation threaded.** `Macros.chooses` and
  `Macros.secretlyChooses` take their noun at
  `Experimental.Phrase.agentIntro who` and keep their single erased `ch :
  ChoiceClause (Just who) n`; nothing else was added to their surface.
  `Macros.choose` is unchanged in signature (`agentCtx Nothing` reduces to
  `bs`), so its ~60 bench sites were untouched.
- **`EnactKeepsOuter` untouched**, and the `Choose (UpTo 1)`-per-role
  structure is untouched.
- **Bench: Consuming Tide** (`Cards/Choice.idr`, `consumingTide : Card`,
  `{2}{U}{U}` Sorcery) — the whole printed card: "Each player chooses a
  nonland permanent they control. Return all nonland permanents not chosen
  this way to their owners' hands. Then you draw a card for each opponent who
  has more cards in their hand than you." The first clause is the shape this
  ticket opens (a distributive chooser publishing "they" to the chosen noun);
  the second is `NotChosen` over exactly one standing choice; the third is a
  `ForEachOf` over a `CompareOver` hand-count predicate.
- **Pins (`ProofsChoice.idr`)**: `badUnchooseredTheyControl` ("Choose a
  creature they control" with no chooser) with twin `okAgentScopedChoice`
  ("Each player chooses a creature they control" [CR#700.8d]); and
  `badDistributedChoiceReadSingular` ("Each player chooses a creature. Exile
  it.") with twin `okDistributedChoiceReadsAsGroup` ("… Exile them."), which
  is what makes the new `ManyOf` pluralization branch non-vacuous.
- **Not done: Stick Together is still a `ForEachOf` loop.** STOP below.
- **Not done: only one of the four named cards became spellable.** Consuming
  Tide is. Sculpted Sunburst and Celestial Judgment are still refused, both at
  `ChoiceInScope Object` — a gate this ticket does not name (probed; messages
  in the landing record). Raiding Party was not probed. Sculpted Sunburst's
  agent half ("each opponent chooses a creature they control with equal or
  lesser power") does now typecheck; what refuses it is the exclusion's
  counted-uniqueness gate, not the agent scope.

## Landing record

Measured on change `rlrqkmnu`, against parent `ktwsvnlp`
(`kata: claim workbench-agent-scoped-choice`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructor rows (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`; lines matching `^ {4,}[A-Z]\w* :`) | 537 | 537 |
| `Unspellable` pin declarations (`Proofs*.idr`) | 622 | 624 |
| `: Card` bench witnesses (`Cards/*.idr`) | 806 | 807 |
| `Eq` instances (`check-eq-indexes`) | 43 | 43 |
| diffstat | — | 9 files, 122 insertions, 40 deletions |

No core constructor was added, renamed or deleted: the change is a re-typing
of one existing row plus four `Phrase` projection functions
(`chosenIntroBy`, `chosenAnnBy`, `chooseIntro`, `chooseAnn`).

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, **91 s** wall on a clean tree (the
  `workbench-party-residues` clean run was 74 s and the
  `workbench-distributive-mutation-hole` one 116 s, so this sits between
  them); `grep -cE 'Error|Warning'` over the log → `0`; bench brace lint
  green.
- `idris/scripts/check-eq-indexes` → `Eq instances checked: 43 index-based or
  derived`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14190 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` — **not run**: the one rule this round cites,
  [CR#700.8d], is already in `cr-citations.lock` and `cite check` reports 0
  stale. `cr-citations.lock` is unmodified.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 1 citation site(s) — read each rule text
  against its claim`. [CR#700.8d] reads "…each player chooses up to one
  creature **they control** of that type", which is exactly the sentence
  `okAgentScopedChoice` admits and which the old typing refused; it points the
  direction it is cited for.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 at the start).
- re-spelled: 20 `Choose` sites for the new argument order — 9 of them pins
  (`badThemCounterRecipient`, `badGroupPower`, `badTargetColor`,
  `badZeroGroup`, `badThemAmbig`, `badAgentChooseTheRest`, `badChooseYou`,
  `badChooseSomeOf`, `badChooseDefinite`), the rest positive witnesses. No
  asserted outcome changed: every pin still refuses the same sentence with the
  same constructor, and every positive witness still admits its own.
- ignored with a blocker: 0.
- added: 2 pins (`badUnchooseredTheyControl`,
  `badDistributedChoiceReadSingular`), 2 twins (`okAgentScopedChoice`,
  `okDistributedChoiceReadsAsGroup`), 1 bench card (`consumingTide`).
- removed: 0.
- pin non-vacuity probes: **13 of 13**, each mis-stated once and restored.
  The 9 re-spelled pins above; the 2 new pins; plus the two `NotChosen` pins
  the `instrIntro` change could have hollowed out
  (`badNotChosenWithoutAChoice` given a standing choice,
  `badNotChosenAfterTwoChoices` reduced to one) — both still turn into `…
  OneChoiceStands is not a valid impossible case.` Mis-statements used:
  `EachOf` around the counter recipient; a single-target choice with `It
  OneOf`; `Macros.creature` for the colour target; `exactly 1` for `exactly
  0`; one of the two ambiguous choices dropped; `Macros.a Macros.creature` for
  `theRest`/`You`/`someOf`/`the`; a chooser added to
  `badUnchooseredTheyControl`; a `You` chooser for
  `badDistributedChoiceReadSingular`.

**Deviations and additions**

1. **`Choose`'s parameters are reordered** to `(by, n, disc)`. Unavoidable:
   `n`'s type now mentions `by`, so the chooser must bind first. This is the
   house shape (`Vote`, `Enact`), and the standing RON-shaped ruling makes
   clause order not a slot, so the row stays positional and re-emittable.
2. **Four `Phrase` functions, not one.** `instrIntro` and `instrProfile`
   already disagreed for a chooser'd choice (the profile never announced the
   chooser). Keeping both old values verbatim on the `OneOf` path — the
   ticket's "reads exactly as before" — needs a pair of plurality-dispatching
   helpers rather than one.
3. **`Consuming Tide` chosen over the other three named cards**, and benched
   in full rather than as a fragment: it is the only one of the four whose
   sole blocker was the agent-scoped-choice gap, and its remaining two
   sentences were already spellable, so the whole card costs nothing extra.
   Its "more cards in their hand than you" predicate is written inline
   (a `CompareOver` mirroring `Cards.Anaphora.opponentWithMoreLands`) because
   the `Macros.They` inside it only resolves at a concrete stack.
4. **`badDistributedChoiceReadSingular` reads through `Macros.exile`**, not
   `destroy`/`SetStatus`/`DealDamage`: those three carry a second obligation
   (`zoneIsB … Battlefield`, `DamageRecipient`) that also fails when the
   singular read finds nothing, which would have made the pin refuse two
   things at once. `exile` leaves exactly the `It OneOf` read open. Same
   idiom as `ProofsAnaphora.badDistributedDiscardSingular`.
5. **No macro added.** `chooses`/`secretlyChooses` already name the row.

**STOP taken**

**Stick Together is not re-spelled to the printed distributive shape.** The
ticket's premise — that the agent-scope gap is what blocks it — is only half
right, and closing that gap is not sufficient.

- The printed card is "Each player chooses a party from among creatures they
  control, then **sacrifices the rest**." (reminder text: "To choose a party,
  choose up to one each of Cleric, Rogue, Warrior, and Wizard."), i.e.
  `Noun.TheRest`, not `Predicate.NotChosen` — the brief's suggested
  `NotChosen` spelling would make the witness read a card that isn't printed.
- The four role choices **do** now work flat and distributively: four
  `Macros.chooses (Macros.each AnyPlayer) (Macros.counted (Macros.upTo 1)
  (And [creature, HasSubtype …, HasPossessor ControllerAx They]))` clauses in
  a `Sequentially` typecheck (probed, 2 s). That is the half this ticket
  opened.
- The sacrifice half is refused, and the refusal is `EnactKeepsOuter`, which
  the ticket says to leave untouched. `Macros.sacrifice (Macros.each
  AnyPlayer) (Macros.theRest Object)` after one such choice fails with
  `Can't find an implementation for MkBinding TheD Player OneOf PlayerP ::
  groupSpent Object (instrIntro (chooses (each AnyPlayer) …)) = take (minus …)
  …` — the group spend the distributive deed may not make, exactly the guard
  `workbench-distributive-mutation-hole` landed.
- Three alternatives were probed and all refused, so this is not a spelling
  I overlooked: (a) flat choices with the sacrifice alone in a `ForEachOf`
  fails at the loop's own `KeepsOuter` with the same `groupSpent` equation;
  (b) one `Macros.chooses (each AnyPlayer) (Macros.partyOf They)` fails at
  `ChoiceClause (Just (Described EachDet AnyPlayer)) (OneEachOf …)` —
  `agentChoosable` is False for the computed party group, which is right
  ([CR#700.8a]: the game computes a party, players don't declare one); (c)
  the brief's `NotChosen` spelling is refused at `ChoiceInScope`, because four
  choices stand and the predicate's gate is counted uniqueness
  (`countParts k bs = 1`) — the gate `badNotChosenAfterTwoChoices` pins.
- Resolution: `stickTogether` is left exactly as `workbench-party-and-outlaw`
  landed it — the `ForEachOf (Macros.each AnyPlayer)` loop, still reading its
  printed card in full including "they control" on all four roles. Nothing was
  widened. Closing the printed shape needs a distributive deed to be allowed
  to spend the choices its own agent phrase published — an `EnactKeepsOuter`
  question, not a `Choose` typing one.

**Follow-up for a live ticket**

- Distributive spend of a distributive choice: let `Macros.sacrifice (each
  AnyPlayer) (theRest Object)` stand when the spent parts are the ones the
  same distributive chooser introduced. Blocks Stick Together's printed shape
  (its last blocker) and Disciple of Caelus Nin.
- "Not chosen this way" after more than one standing choice: [CR#700.8d]-style
  cards write several choices under one "this way". Blocks Sculpted Sunburst
  (two choices) and Raiding Party ("by any player"). Celestial Judgment needs
  a third thing — `ForEachKindOf` publishes nothing outward
  (`instrIntro (ForEachKindOf _ _ _ _) = bs`), so its in-loop choice cannot be
  read back at all.
