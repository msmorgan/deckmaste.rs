---
needs: []
---
**Close the two gaps the party round left in its witnesses.** Residue of
`workbench-party-and-outlaw` (2026-09-04):

- **Crime.** At Knifepoint is benched as its first ability only; its second
  ("Whenever you commit a crime, …") needs a game event for committing a crime
  [CR#700.13] — a player casting a spell, activating an ability, or putting a
  triggered ability on the stack that targets an opponent or their permanent,
  spell, ability, or graveyard card. Add the event to `Triggers` as one
  positional row (RON-shaped), bench At Knifepoint's second ability and one
  more vintage-legal crime card, pin a CR-meaningless target (a crime with no
  opponent-owned object).
- **"That weren't chosen this way."** Stick Together's closing line is
  spelled as a `ForEachOf (each AnyPlayer)` loop holding the four
  `Choose (UpTo 1)` roles and `sacrifice They theRest`, because a
  distributive deed cannot host a group spend (`EnactKeepsOuter`) and no
  predicate reads "not chosen this way". Decide whether an exclusion
  predicate over a prior choice (`NotChosenBy`-style, counted-uniqueness
  gated) is a real vocabulary gap or whether the loop reading is the settled
  shape for "each player … then each player sacrifices … that weren't
  chosen"; if the former, add it and re-spell the witness back to the printed
  distributive shape.

Size: S–M. Done when: both witnesses read their printed card in full; pins
probed non-vacuous; build at its module count. Standard constraints apply,
including the RON-shaped constraint.

## As landed

- **Crime.** `Triggers.GameEvent.CommitsCrime (who : Noun bs Player)` — one
  positional row; the crime's victim is not a slot because no printed card
  names it and the RON node carries only the subject. Gated by the named
  witness `Triggers.CrimeSubject`/`OneCriminal` (`So (isOne (nounPlur who))`):
  a crime targets an opponent, something an opponent controls, or a card in an
  opponent's graveyard [CR#700.13], so it is one player's action and the whole
  player set has no opponent to victimise. `Events.EventName.CrimeCommission`
  with `eventIx 41` and an `eventFactsOf` row (`[Player]` subject, no
  complement, interceptable and spannable — the `GameLoss` shape).
  Bench: **At Knifepoint** is now the whole card (`Cards/Description.idr`
  `atKnifepoint`, `{1}{B}{R}` Enchantment) over the kept `atKnifepointOutlaws`
  plus the new `atKnifepointCrime` — "Whenever you commit a crime, create a
  1/1 red Mercenary creature token with '{T}: Target creature you control gets
  +1/+0 until end of turn. Activate only as a sorcery.' This ability triggers
  only once each turn." — and **Patrolling Peacemaker**
  (`Cards/Counters.idr`), which reads the other subject, "Whenever **an
  opponent** commits a crime, proliferate". Pin
  `ProofsTrigger.badCrimeByAllPlayers` ("Whenever all players commit a crime")
  with twin `okCrimeBySinglePlayer`, probed non-vacuous (`Macros.a AnyPlayer`
  in the same slot turns it into "not a valid impossible case").
- **"That weren't chosen this way."** Decided on the evidence: **the exclusion
  predicate is a real vocabulary gap**, so it is added — but it does not
  unblock Stick Together, which keeps its loop (STOP below).
  `Phrase.Predicate.NotChosen`, gated by the named witness
  `Phrase.ChoiceInScope`/`OneChoiceStands` (`countParts k bs = 1` — counted
  uniqueness, so "this way" has exactly one standing choice to read back).
  Being a predicate it introduces and spends nothing, so `EnactKeepsOuter` is
  untouched, exactly as the brief requires.
  Evidence for the gap: seven vintage-legal cards write "not chosen this way",
  and five of them no loop-plus-`TheRest` reading can spell — Raiding Party
  and Sculpted Sunburst say "by any player" (a cross-agent exclusion the loop
  cannot see), while Consuming Tide, Celestial Judgment, Thunderwave and
  Disciple of Caelus Nin exclude from an independently described *global*
  group ("all nonland permanents", "each creature", "all permanents other than
  this creature"), not from a partitive leftover of the chooser's own group,
  which is all `Noun.TheRest` can denote.
  Bench: **Thunderwave** in full (`Cards/Damage.idr`), whose 10—19 row is
  "You may choose a creature. Thunderwave deals 3 damage to each creature not
  chosen this way" — the one crime-free card in the seven whose chooser is the
  deictic `You`, so nothing else blocks it. Pins (`ProofsChoice.idr`):
  `badNotChosenWithoutAChoice` (no choice stands) and
  `badNotChosenAfterTwoChoices` (two stand, so no unique antecedent), with the
  twin `okNotChosenAfterOneChoice`; both probed non-vacuous (each mis-stated
  to one standing choice turns into "not a valid impossible case").
  **STOP — Stick Together is not re-spelled distributively.** The brief pairs
  "add the predicate" with "re-spell Stick Together back to its printed
  distributive shape", and the two cannot both hold. `Effect.Choose` takes its
  noun and its chooser both in `bs`, so beside a distributive `each AnyPlayer`
  chooser the chosen noun cannot say "creatures **they** control": the shape
  is refused at `Can't find an implementation for countReach (Word PlayerW)
  OneOf [] = 1`. Dropping "they control" from the four `Choose (UpTo 1)` roles
  does typecheck — the sacrifice half with `NotChosen` is fine, so the
  exclusion gap really was one of the two blockers — but it makes the witness
  read a different card than the printed "chooses a party from among creatures
  **they control**" [CR#700.8d]. The remaining blocker is the agent-scoped-choice
  gap the party round already recorded as Choice-family and out of region, not
  the exclusion gap this ticket closes. `stickTogether` is therefore unchanged
  and still reads its printed card in full, in loop shape.

## Landing record

Measured on change `ttmyvrkk`, against parent `qnlstsvm`
(`kata: claim workbench-party-residues`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructors (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`) | 791 | 795 |
| `Unspellable` pins | 608 | 611 |
| `: Card` bench witnesses | 803 | 806 |
| `Eq` instances (`check-eq-indexes`) | 43 | 43 |
| diffstat | — | 9 files, 201 insertions, 0 deletions |

Four constructors added: `Triggers.GameEvent.CommitsCrime`,
`Triggers.CrimeSubject.OneCriminal`, `Phrase.Predicate.NotChosen`,
`Phrase.ChoiceInScope.OneChoiceStands`, plus the `Events.EventName`
row `CrimeCommission`. Nothing was deleted or renamed, so no re-spelling
sweep was needed and the diff is additive only.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 1 m 14 s wall; a second run greps 0 lines matching
  `warning` or `error`; the bench brace lint passes.
- `idris/scripts/check-eq-indexes` → `Eq instances checked: 43 index-based or
  derived`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14191 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` — **not run**: both rules cited by this round,
  [CR#700.13] and [CR#700.8d], are already in `cr-citations.lock` (the party
  round registered them) and `cite check` reports 0 stale. `cr-citations.lock`
  is unmodified.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 4 citation site(s) — read each rule text
  against its claim`. Each read: [CR#700.13] states the crime definition and
  the opponent-owned-object requirement the `CrimeSubject` gate rests on;
  [CR#700.8d] names Stick Together and its "each player chooses up to one
  creature **they control** of that type", which is exactly what the STOP
  says the distributive shape would lose.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 at the start).
- re-spelled: 0. Nothing was deleted or renamed; `atKnifepointOutlaws` and
  `stickTogether` keep their names, bodies and asserted outcomes.
- ignored with a blocker: 0.
- added: 3 pins (`badCrimeByAllPlayers`, `badNotChosenWithoutAChoice`,
  `badNotChosenAfterTwoChoices`), 2 positive twins (`okCrimeBySinglePlayer`,
  `okNotChosenAfterOneChoice`), 3 bench cards (`atKnifepoint`,
  `patrollingPeacemaker`, `thunderwave`) and 1 bench ability
  (`atKnifepointCrime`, spliced into `atKnifepoint`).
- removed: 0.
- pin non-vacuity probes: 3 of 3, each mis-stated once and restored.
  `badCrimeByAllPlayers` with `Macros.a AnyPlayer`,
  `badNotChosenWithoutAChoice` with a choice prepended, and
  `badNotChosenAfterTwoChoices` with one of the two choices dropped, each turn
  into `Error: … is not a valid impossible case.`

**Deviations and additions**

1. **At Knifepoint became a whole card.** The ticket asks only for "At
   Knifepoint's second ability", but the Done-when asks the witness to read
   its printed card in full, so `atKnifepointOutlaws` (unchanged) and the new
   `atKnifepointCrime` are now spliced into `atKnifepoint : Card`. Its printed
   cost is `{1}{B}{R}`.
2. **The card lives in `Cards/Description.idr`** beside the outlaw witness it
   extends, not in `Cards/Trigger.idr`, so the card stays in one place; the
   family dependency order is unaffected (Description is first).
3. **`Patrolling Peacemaker` reads the other subject.** The ticket asks for
   "one more vintage-legal crime card"; this one was chosen over the simpler
   `You` cards because it is the only printed subject variant — "Whenever
   **an opponent** commits a crime" — so both readings of the new row have a
   bench witness rather than only a pin twin.
4. **`Thunderwave` is benched in full**, not as the 10—19 row alone: the row
   is the `NotChosen` witness, and the surrounding `ResultsTable` was already
   spellable, so the whole card costs nothing extra. It is the first `d20`
   table card on the bench.
5. **`ChoiceInScope` is a named `data` witness, not a bare `So`**, so a
   missing choice and an ambiguous one fail with the constructor named rather
   than as `So False`.
6. **No macro for either row.** `CommitsCrime You` and `NotChosen` are
   single-token spellings with no implicit handles, so a macro would only
   rename them; the bench reads the constructors directly, as it does for
   `Dies`, `Draws` and `Other`.
7. **Region respected**: `crates/xtask/src/facts.rs`, `FactsGen.idr`,
   `Words.idr` and `Designation` are untouched, so the `workbench-facts-residues`
   sibling merges cleanly. The new event's facts row is in `Events.idr`
   (`eventFactsOf`), which is outside that sibling's region.

**STOP taken**

**Stick Together is not re-spelled to the printed distributive shape**, though
the predicate the brief conditions that on was added. The two cannot both
hold: `Effect.Choose` takes both its noun and its chooser in `bs`, so beside a
distributive `each AnyPlayer` chooser the chosen noun has no antecedent for
"they" — the shape is refused at `Can't find an implementation for countReach
(Word PlayerW) OneOf [] = 1`. Dropping "they control" from the four
`Choose (UpTo 1)` roles typechecks (probed: the `NotChosen` sacrifice half is
fine on its own, so the exclusion gap really was one of the two blockers the
party round named), but it would make the witness read a card other than the
printed "chooses a party from among creatures they control" [CR#700.8d].
Resolution: `NotChosen` landed and benched on Thunderwave, which needs no
agent-scoped choice; `stickTogether` left unchanged. Closing the shape needs
the agent-scoped-choice gap — a `Choose` whose noun is typed at
`agentIntro by` — which the party round already routed as Choice-family and
which this ticket does not name.

**Follow-up for a live ticket**

- Agent-scoped choice: `Effect.Choose`'s noun at `agentIntro by`, so "each
  player chooses … they control" is spellable outside a loop. Blocks Stick
  Together's printed shape, and also Consuming Tide, Disciple of Caelus Nin,
  Raiding Party and Sculpted Sunburst — four of the seven "not chosen this
  way" cards.
