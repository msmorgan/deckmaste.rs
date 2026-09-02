# workbench-pile-partitions

The pile bundle, declined whole by the turn-structure round (2026-09-02)
rather than sliced — 41 supported cards. Its close-out characterized the
remaining cost precisely: the partition machinery ALREADY EXISTS
(`theRestOk`/`partsTaken`/`countedGroupSize`/`groupSpent`; `SomeOf` writes
"one of those piles"; `TheOther` writes "return the other" verbatim). What
remains is a `PileP` payload and a `PileW` word — so "that card" cannot read
a pile — (~100 mechanical clauses) plus the partition row itself. The naming
ruling (done/workbench-named-memory-channels) governs: piles are not named
cross-ability. Death or Glory and Fact or Fiction are the marquee benches.
Re-measure at claim.

## As landed (round of 2026-09-02)

Every count below was re-measured this round over `jq 'select(.supported)'`
against `data/derived/cards.jsonl`; where it differs from the brief above,
the corrected figure is the one to carry forward.

### Premise corrected

**38 supported cards write "pile", not 41**, over 96 sentences. The
family divides into the PARTITION (26 lines: "separate/divide [group]
into [n] piles") and the SINGLE PILE the exile idiom makes (7 cards:
"exile them in a face-down pile, shuffle that pile, then cloak those
cards"), which is not a partition at all — one pile, made by the exile,
with no choice after it.

### Rows minted

- **`Payload.PileP`** (zone, size) — the pile mention. [CR#700.3b] is
  one sentence with two halves and both are load-bearing: "the pile is
  not an object" is why the payload is its own, so every object word
  answers False at it and "that card" cannot read a pile; "each object
  in a pile is still an individual object" is why it sits at the
  `Object` KIND anyway, so `Move` takes it as it takes any group of
  cards and the landed partition machinery
  (`objGroup`/`countParts`/`theRestOk`/`groupSpent`) reads it unwidened.
  Zone is carried because [CR#700.3c] leaves the members where they
  were — Death or Glory's piles are in the GRAVEYARD, so this is not the
  library-search gap; size is counted in PILES, which is what makes "the
  other" writable.
- **`NounWord.PileW`** — the pile demonstrative, the one word that
  reaches a `PileP` and reaches nothing else. "that pile" / "those
  piles" / "the chosen pile".
- **`Effect.SeparateIntoPiles`** (separator, group, pile count) — the
  partition [CR#700.3], 26 supported lines. The SEPARATOR is a slot and
  not an ellipsis: 14 lines write it and 12 leave it to the clause's
  agent, and wherever a card writes both roles the separator and the
  chooser are different players. It SPENDS the group it partitioned
  ([CR#700.3a] puts each object into exactly one pile, so no undivided
  group is left to name), which is what lets "the other" count against
  the piles rather than the cards. The count is a bare `Nat` with no
  lower bound — [CR#700.3d] admits an empty pile, so no printed count is
  rules-impossible and the row refuses none.
- **`Noun.PileOf`** (slice count, chooser) — the pile partitive, 11
  lines at "chooses one of those piles", 8 at the bare "one pile", 6
  with the chooser inside the phrase ("the pile of an opponent's
  choice"). A SECOND ROW beside `SomeOf` rather than a payload switch
  inside it: `SomeOf` mints an `ObjectP` outright and has to, because a
  partitive that chose its payload CONSTRUCTOR by a test on its base
  stops reducing wherever that base is abstract. It takes no base
  mention — the piles carry no name (the naming ruling), so the pile
  word is the whole gate.
  The chooser is a slot on the ROW and not a `ChoiceMode`: `ChoiceMode`
  has no player position (`TheirChoice` reads a chooser the prefix
  already holds) and these lines introduce their own. It is not on
  `SliceCount` either — threading the chooser's delta from
  `sliceCountDelta` adds a `nounDelta` edge that costs the whole
  `Phrase` mutual block its termination proof, measured.

### Reused unchanged

`TheOther` writes "and the other into your graveyard" verbatim over a
partition of two, and `TheRest`, `theRestOk`, `theOtherOk`,
`countedGroupSize`, `partsTaken` and `groupSpent` were not touched.
`Effect.Choose`'s agent slot takes the pile partitive because
`agentChoosable` admits it where `choosable` refuses it — no printed
line writes the bare "choose one of those piles" — and `chosenDelta`
falls through to `nounDelta`, so the choice leaves the `PartD` pile
mention that `That PileW` reads.

### Benches — 4 whole cards

**FACT OR FICTION** (the marquee, and [CR#700.3c]'s own worked example),
**DEATH OR GLORY** (the graveyard partition and the chooser-in-the-phrase),
**STEAM AUGURY** (the choice as its own sentence, the commonest spelling),
**SPHINX OF UTHUUN** (the same three sentences inside a trigger, so the
row is not spell-only). The three spellings of one procedure and two
carriers.

### Pins — 3, all in `ProofsG`

`badCardWordReadsPiles` ("Put those cards into your hand" after a
separation), `badPileWordWithoutAPartition` ("Put those piles into your
hand" after a bare reveal) and `badPilePartitiveWithoutAPartition`. The
first two are [CR#700.3b]'s two halves held apart; none refuses a count.

### Declined, with counts and blockers

- **The pile-CONTENTS read, 13 lines over 12 cards** — the largest
  single blocker, and why Do or Die, Liliana of the Veil's ultimate,
  Boneyard Parley, Fight or Flight, Truth or Tale and Phyrexian Portal
  do not bench whole. Wants a `Predicate bs Object` testing pile
  membership; [CR#700.3b] leaves nothing for a containment predicate to
  hold and [CR#700.3c] keeps the members in their own zone, so `InZone`
  cannot say it.
- **The face marking on a pile, 17 lines over 16 cards** — the
  face-down/face-up PAIR (6) and the single face-down pile of the exile
  idiom (7 cards). `FaceDown` is a battlefield-permanent status,
  `OnBattlefield`-gated at both `HasStatus` and `SetStatus`; these mark
  cards in exile and in the library. Shuffling a pile (7 lines) is a
  second gap in the same cards, and the rule is on the cards' side —
  [CR#701.24a] shuffles "a library or a face-down pile of cards" in one
  sentence, so it is this grammar's shuffle that is library-typed.
- **The per-player partition, 6 lines over 5 cards** (Bend or Break,
  Make an Example, Raging River, Stand or Fall, Whims of the Fates).
  `ForEachOf` at the player kind is landed and is where this goes, but
  the piles a pass makes must be scoped to the body where the row mints
  one mention for the clause — the element-scoping question, not a slot
  here. Whims wants "at random" on the pick besides; Bend or Break "one
  of their opponents of their choice" as the chooser.
- **The labeled and variable partition, 4 lines over 2 cards**
  (Camouflage, Raging River). Three widenings — a pile LABEL, a variable
  pile count, and an assignment of piles to objects — and both cards are
  blocked on their combat lines independently.
- **The empty-pile reminder, 5 lines.** Reminder text restating
  [CR#700.3d], which the row already admits. Nothing to build.

The full residue list with the same counts is in
`idris/src/Experimental/Cards.idr` under "THE PILE PARTITION'S RESIDUES".

Gate: `idris/scripts/build` 23/23, 0 errors, 0 warnings.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 0 stale
over 20328 citations; `cite bless` registered no new rule; the round's
34 citation sites were read against their rule texts and two were
rewritten — one [CR#701.24a] claim that pointed backwards (the rule
defines shuffling a pile) and one [CR#700.3a] cite doing no work.
