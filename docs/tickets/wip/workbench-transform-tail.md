# workbench-transform-tail

The transform round's survivors (done 2026-09-02, recovered whole after the
crash; details in its As-landed section):

- **The intransitive transform voice + "into [name]" complement**
  [CR#701.27e] — 39 trigger faces, 37 with "into [name]".
- **Casting-"transformed"** [CR#712.11a] — 62 faces.
- **"Transformed permanent" as a description** (3 lines; the participle is a
  STATE by [CR#701.27g], which is why the verb rows record no participle).
- **State triggers** [CR#603.8] — declined at 1 supported loyalty-state face
  (Garruk Relentless's last blocker); revisit if a second carrier appears.
- **The plural anaphor over two singular antecedents** — "meld them into" /
  the melders' shared read now that the meld verb exists; recorded at
  `obeliskOfUndoing`.

Re-measure at claim.

## As landed

Corpus authority throughout: `data/derived/cards.jsonl` filtered
`jq 'select(.supported)'` (32,568 supported faces; 60,133 supported
lines), all counts re-measured 2026-09-02 with reminder text stripped
where the number is about printed rules text. CR text via `data/rules/`.

### 1. The intransitive voice and its "into [name]" complement — landed

Counts hold: **39** supported lines write the intransitive `transforms`,
**37** of them with the complement. (Of the 37, 35 write a printed name
and 2 write a bare characteristic — Cult of the Waxing Moon's "into a
non-Human creature" and Norn's Inquisitor's "into a Phyrexian".) The two
bare ones are Neglected Heirloom and Corruption of Towashi's first arm.
Head words: 16 `When`, 18 `Whenever`, 4 `As`, 1 ability-word `When`.

- `VerbedVoice` is now INDEXED BY THE LABEL and has three arms. This
  was forced, not stylistic: the actorless voice is spelled two ways and
  the label's own rule picks which, so a type over the two nouns alone
  could not tell them apart. `PassiveAct` gains `So (actNamesParticiple
  v)` and `IntransitiveAct` takes `So (actIntransitiveOf v)`.
- **A latent hole closed on the way.** Nothing had held the passive to a
  participle, so `VerbedEvent Nothing "Transform" …` type-checked and
  spelled "equipped creature is [nothing]". `neglectedHeirloom` was
  written that way and is corrected to `IntransitiveAct`; the pin
  `badPassiveWithoutParticiple` keeps `"Put"` out of that voice.
- `VerbFacts.actIntransitive` is the datum, True at `Transform` and
  `Convert` and False at the other seventeen. ONE field for the voice
  and the complement both, because [CR#701.27e] states the pair in one
  sentence and no printed line writes either without it.
- The complement is `VerbedEvent`'s fifth positional slot, a
  `Predicate` and not a noun: [CR#701.27e] says "an object with a
  specified characteristic", and [CR#109.3] puts a NAME among an
  object's characteristics, so one slot spells the 35 names and the 2
  subtypes alike. Gated by `VerbBecomes`; pin
  `badBecomesWithoutIntransitive`.
- Macros `transforms` / `transformsInto`. Bench: `neglectedHeirloom`
  (bare, corrected) and **Cult of the Waxing Moon whole**
  (`cultOfTheWaxingMoon`) for the complement.

### 2. Casting-"transformed" [CR#712.11a] — VERDICT: no row, at a measured zero

**The 62 was a reminder-text count and the ticket's premise is
corrected.** Re-measured: 62 supported lines contain "cast … transformed",
and with parenthesized reminder text stripped **0** remain. The
breakdown is 24 Disturb reminder lines ("You may cast this card from
your graveyard transformed for its disturb cost"), 35 Siege reminder
lines ("When it's defeated, exile it, then cast it transformed"), and 3
lines that the loose match caught but that are the transformed ARRIVAL
[CR#712.14a] and not a casting at all (Esper Origins, Grist, Ral,
Monsoon Mage — all "put/return onto the battlefield transformed", which
`TokenRider.EntersTransformed` already writes).

So this stands exactly where the Adventure exile-and-recast rider
stands: [CR#712.11a] states the whole reading as a rule, and every
printed occurrence is the reminder text of a keyword ([CR#702.146a]'s
disturb) or of an intrinsic ability ([CR#310.12b]'s Siege). No row, and
the zero is measured rather than assumed.

### 3. "Transformed permanent" as a description — landed

`Predicate.IsTransformed`, [CR#701.27g]. Count holds at **3** supported
lines: Invasion of Pyrulea // Gargantuan Slabhorn (static subject),
Mutagen Connoisseur (counted), Oculus Whelp (bare existence). Seeds the
battlefield, the rule naming that zone inside the description. Not a
fifth `StatusCat` — [CR#110.5] closes status at four categories — which
is the refusal `Effect.TurnOver` already records. Bench: **Mutagen
Connoisseur whole** (`mutagenConnoisseur`).

This row is why `Transform` records no participle, and the docstring now
says so from the other side: a participial lookback names what the act
was performed on, and this names what a permanent IS — two different
sets, since a transformed ARRIVAL [CR#712.14a] is one without the act
ever happening.

### 4. State triggers [CR#603.8] — the count MOVED, and it is landed

**The recorded verdict measured the wrong thing.** It was declined at
"1 supported loyalty-state face", which is the count of Garruk
Relentless's own phrasing — but the MACHINERY's carriers are the whole
state-trigger family, and that is **35 supported lines**:

| shape | lines |
| --- | --- |
| "When you control no [description]" | 21 |
| "When there are no [description]" | 5 |
| "When [n] has no [kind] counters on it" | 2 |
| "When [player] has 10 or less life" | 2 |
| "When a player has no cards in hand" | 1 |
| "When Garruk has two or fewer loyalty counters on him" | 1 |
| landhome subtotal ("no Islands") | 13 of the 21 |

All 35 write the word `When`; the zero at `Whenever` is recorded rather
than gated, [CR#603.8] naming no word of its own.

- `GameEvent.StateHolds : Condition bs -> GameEvent bs`, named
  `EventName.StateMatch`. A `GameEvent` row and not a second header
  kind, because [CR#603.2] already writes the slot as matching "a game
  event or game state" and [CR#603.1] spells one header for both — so
  the window, the joined header, the usage limit and the intervening
  clause all keep working over it unchanged.
- It is none of the three condition seats the header already had, and
  the row says which and why: not [CR#603.4]'s intervening clause (that
  rule reaches only an `if` following a trigger condition, and re-checks
  on resolution, where this condition IS the trigger condition), not
  `Concurrent.WhileTrue` (which narrows an event the header names), not
  a static guard.
- It announces nothing — a condition names no referent — so Garruk's
  "him" is the source's own deixis.
- Tables: `sameEventName`, `eventHasMagnitude` and `lookbackSubjectOk`
  each answer the new name negatively with [CR#603.8] as the reason.
  Pin `badStateMatchLookback`.
- Macro `whenState`, beside `whileState`. Bench: **Skeleton Ship whole**
  (`skeletonShip`) for the landhome majority, and
  `garrukRelentlessFlip` for the counter-count reading.
- `predatoryWurm`'s stale note ("blocked on a STATE TRIGGER") is
  corrected.

### 5. The plural anaphor over two singular antecedents — still recorded

Not built, and the count did not move: the seven ownership-condition
melders remain its only carriers. Unchanged from the transform round's
statement, and it is a `Them`/`AndCond` question rather than a transform
one.

### Honest remainder

Garruk Relentless is no longer blocked on the state trigger, but is not
yet benched WHOLE: its back face writes "search your library for a
creature card, reveal it, put it into your hand", which is the
`revealsIt` gap owned by workbench-search-locus-tail. `garrukRelentless`
lands there.
