# workbench-small-residues

The bucket round's surviving small items (2026-09-02; corrected counts —
full method and blockers in done/workbench-event-disjunction-tail.md and
Cards.idr's residue trailers):

- Ordinal-per-turn activation restrictor — 11 lines / 11 cards, two shapes.
- Entry-rider defender slot — 15/15 of 121 (needs the `TokenRider` module
  move above `Noun`/`AttackDefender`).
- Attachment-host word with no card type — 105 lines / 80 cards total; the
  prohibition subset is 31/31.
- Cross-kind you-or-description join — 42/41 (the "you or a player" class
  was a false positive, joins two players).
- Per-player partition — 6/6 of 28 (element scoping on `ForEachOf`).
- Marked player-sort read ("the last chosen player" — Beckoning
  Will-o'-Wisp / Triarch Stalker).
- Ordinal-player cluster (`NthPlayer`, `OpponentOf` sibling — probed
  blockers in done/workbench-each-player-binder.md).
- Face-down spell — 7 descriptions + 1 cast-instruction.
- Subtype retention — 2/2 (Cavernous Maw, Duplicant).
- `Choose` binder gap — 2/2 (Call to the Void, Malik).
- Singletons waiting on second carriers: MoveCounters partitive (1),
  Master Biomancer's rider (1), Temporal Anchor's step-event (1).
- The general event disjunction stays DECLINED with its live successor
  (the tail-aligned union) recorded in the bucket's close.

Re-measure at claim; several may fall out of each other.

- **Routed from mandatory-if-you-do (close, 2026-09-02):** the token
  SUPERTYPE slot (`TypeLine` has none, so "a legendary 20/20 black Avatar
  token" is unwritable — Dark Depths' real blocker, with Marit Lage's
  Slumber); the player-side COUNT comparison (`PlayerStat` is life-only —
  Cartographer's Hawk's "controls more lands than you"); Flameskull's
  two-exile batch read; and Breath of Fury's coordinated antecedent (split
  `EncNotOneAction` into agent-half and count-half if a second carrier
  appears).

- **Routed from room-halves (close, 2026-09-02):** the effect-level ACT
  DISJUNCTION over one door ("Lock or unlock a door of target Room", 2 lines
  + Ghostly Dancers — not `Modal`'s bulleted spell [CR#700.2]; brings
  [CR#709.5g]'s Lock label with it) and the PLURAL door count (3 reads —
  halves of a described group with distinct names per [CR#709.5]).

- **Routed from pile-contents-and-faces (close, 2026-09-02):** the
  EXILE-MADE pile + the shuffle's pile arm (10 + 7 lines — waits on
  cloak/manifest, fence territory; [CR#701.24a]'s gap recorded); Boneyard
  Parley's `theRestOk` two-standing-groups question; `Reveal`'s missing
  participle (Truth or Tale); the exclusive-permission arm ("only [n] can" —
  Fight or Flight, Stand or Fall); Sauron's Ransom's Ring-tempts
  INSTRUCTION; and a stale-note flag worth a bench attempt (Unesh / Sphinx
  of Clear Skies — daruWarchief/tribalFlames may already write both halves).

- **Routed from mixed-head-disjunction (close, 2026-09-02):** the
  all-headless PRESUPPOSITION disagreement (3 lines — Sonar Strike, Tetsuo
  Umezawa, Dire Downdraft; the Talion `Compare`-list precedent's own
  question) and the `historic` PREDICATE ([CR#700.6], 57 supported cards —
  its reminder is a substantive list, not a mixed coordination).

## As landed (closing sweep, 2026-09-02)

Every count below was re-measured this round over
`jq 'select(.supported)'` against `data/derived/cards.jsonl`, with
parenthesised reminder text stripped before matching. Where a figure
differs from the bullets above, the corrected one is what to carry
forward.

**Measurement warning, learned twice in this round and worth carrying.**
A narrow regex under-reports and reads like a finding. The token
supertype measured 16 lines under `create <word> ... legendary ...
token` and 47 under a lazy match that stops at the first "token"; the
ordinal-per-turn restrictor measured 2 under "each turn" and 11 once
"during each of your turns" was admitted. Both narrow figures would have
gone into this ledger as re-measurements. Measure the CONSTRUCTION with
a deliberately loose pattern and then read every hit.

### Headline — the bucket was mostly already built

Six of the items routed here with a named blocker were writable when the
round opened, and one more had landed in the same day's earlier round.
None of them was a close call: in every case the machinery existed under
another name, and what was missing was a bench. They are benched now,
because a bench is the only proof that a construction writes.

- **The cross-kind "you or [description]" join** — routed as "no joined
  head takes 'you'" and called "the biggest single unbuilt item in the
  bucket" at 42/41. Re-measured at **45 lines / 43 cards**, and
  `EitherJoined` has spelled it since the attacking-defender round: two
  arms of different kinds, no agreement gate between them. Benched whole
  on **Blood Reckoning**. The routed count was measuring a family that
  had already landed. (The family's other half — the 25 lines with a
  PLURAL right arm, "you or planeswalkers you control" — still refuses at
  `OneDefender`'s singular gate, correctly: `EitherJoined`'s number is
  `ManyOf` when its arms disagree, and [CR#508.1b] gives each attacker
  one defender. Those lines write at the deontic seat instead, where
  `archangelOfTithesAttackToll` already benches them.)
- **The player-side COUNT comparison** — routed as "`PlayerStat` is
  life-only". Re-measured at **59 lines / 54 cards**. `PlayerStat` needs
  no count arm and never did: the comparison is `CompareOver`'s
  member-relative measurement, whose docstring names "an opponent who
  controls more lands than you" as its own worked example and which
  `opponentWithMoreLands` has benched since the counted-search round.
  What was genuinely unwritten was a carrier at the CONDITION frame, and
  `Exists` over the same description is it. Benched whole on **Land Tax**.
- **The attachment-host word with no card type** — routed as the
  bucket's largest item at 105/80. `attachHeadOk` admits every noun word
  under `Enchanted` and `PermanentW` under `Equipped`, and `AttachHost
  Enchanted PermanentW` is the phrase. Benched whole on
  **Indestructibility**. The routed figure also **conflated two
  readings** of the same two words: re-measured, **93 lines / 71 cards**
  write the HOST word (an Aura's own text naming what it is attached to)
  and **12 lines / 12 cards** write the ADJECTIVE ("destroy target
  enchanted permanent"), which is `IsAttached` and a different row. Only
  one of the 93 writes "equipped permanent" (Luxior, Giada's Gift).
- **The marked PLAYER-sort read** — routed with two blockers and
  carrying neither. `ChosenPlayer`'s own docstring already spells it
  ("the last chosen player" behind a repeatable chooser) on
  `ChoiceStands`' existence gate, the same gate `OfLastChosen` carries;
  and `AttackerOf` writes "creatures attacking [player]" in exactly the
  reduced participle these two cards print. `abIntro` threads a triggered
  ability's choice delta to the abilities printed after it, so the static
  reads a choice the trigger makes each combat — which is the repetition
  [CR#607.2d] gives the marked spelling. Both carriers benched whole:
  **Beckoning Will-o'-Wisp** and **Triarch Stalker**.
- **The mandatory "if you do"** — listed as "the LARGEST unbuilt item in
  the bucket" at 96/95. `IfDone` landed in the mandatory-if-you-do round
  earlier the same day (94 lines / 93 cards there), and Charnel Troll and
  Garruk Relentless bench on it. Stale on arrival.
- **The mixed head/adjective disjunction** — `parallelDisjuncts` now
  admits an adjectival arm beside head-bearing ones, on the
  enclosing-head reading its own docstring records. Landed in the
  mixed-head round; nothing owed.
- **The flagged stale note (Unesh / Sphinx of Clear Skies)** — run, as
  the round was asked to. `daruWarchief` writes the subtype-scoped cost
  reduction and `tribalFlames` writes the domain count, so the note was
  stale. **Unesh, Criosphinx Sovereign benches whole**, and its trigger
  header needed nothing either: "Unesh or another Sphinx you control" is
  `EitherOf` over the self and an `OtherThan`-anchored description.
  Sphinx of Clear Skies does not land, and its real blocker is neither of
  the two the note named — see ledger item 13.

### Built — three rows, one counter kind, and one module move

**1. The token SUPERTYPE cell.** `TokenChars` gains `supers : List
Supertype`, and it sits BESIDE the type line rather than inside it, on
the card seat's own arrangement: `cardOf` takes `supers` and `line` as
two arguments and `SharedLineSplit` splits them apart again, because
[CR#205.4b] makes a supertype independent of the card types and subtypes
it is printed in front of. Putting it in `TypeLine` would also have cost
923 construction sites where the record costs six.
[CR#111.9] is the cell's own rule ("some effects instruct a player to
create a legendary token ... written 'create [name], a . . .'") and
[CR#111.3]'s example is what made it visible as missing (a bare
Saproling token has "no mana cost, supertypes, rules text, or
abilities"). **47 supported lines over 46 cards** write one: 45
legendary, every one of them [CR#111.9]'s named frame WITHOUT EXCEPTION
— the rule describes its whole corpus — and 2 snow (Replicating Ring,
Svella, Ice Shaper), which write the word in an ordinary bundle and name
their tokens with a trailing "named [name]".
`MkToken` stays the five-cell wrapper and defaults the list to empty;
`MkSupertypedToken` is the six-cell one. NO CLOSED SET is written:
`CardSupers` gates distinctness alone, [CR#205.4a] enumerates five
supertypes, and no rule refuses any of them to a token. "Ongoing" is
inert on one — [CR#205.4h] exempts an ongoing SCHEME CARD and [CR#111.6]
says a token isn't a card — but inert is not impossible, and a refusal by
what the corpus prints is what this workbench does not write.
The distinctness gate folds into `tokenCanonical` beside `colorsDistinct`
and `typesDistinct`, for the same reason [CR#205.4b] gives.
`supersDistinct` moved from `Card.idr` up to `Words.idr`, beside
`Supertype` itself, so both layers can reach it.

**2. The `Ice` counter kind.** An ordinary marker [CR#122.1] in `Luck`'s
and `Bloodstain`'s mold. **17 supported lines over 7 cards** name it:
Dark Depths' countdown and Iceberg's stored mana put and remove it,
Rimefeather Owl and Rimescale Dragon describe the permanents carrying it,
and Draugr Necromancer exiles a card with one on it.

**3. The `historic` PREDICATE.** [CR#700.6]'s defined term, "an object
that has the legendary supertype, the artifact card type, or the Saga
subtype". **59 supported lines over 55 cards**, heads spread across the
whole object vocabulary: 23 "historic spell", 13 "historic card", 12
permanent(s), 3 creature(s), 3 land(s).
It is a ROW AND NOT THE UNION IT ABBREVIATES, and the choice is a real
one rather than a forced one now that the mixed head/adjective
disjunction has landed: `Or [HasSupertype Legendary, HasType Artifact,
HasSubtype Saga]` is buildable. What decides it is that all 59 lines
print the single adjective and none prints the union — the union is the
REMINDER, which this workbench strips before it measures. Spelling the
word as its definition would put a three-armed coordination where the
English has one modifier, and would tie the description's identity to the
rule's current membership rather than to the printed word.
It is an ADJECTIVE (`hasHead` False) and seeds NO ZONE, unlike
`IsTransformed`: [CR#700.6] describes an object and [CR#109.1] makes a
card in any zone one, which is why the same word reaches a spell on the
stack, a card in a graveyard and a permanent on the battlefield.
Benched whole on **Artificer's Assistant** (the stack) and **Aya of
Alexandria** (the battlefield).

**4. The ENTRY-RIDER defender, and the `TokenRider` module move.**
[CR#508.4] gives the rider its content and its default in one sentence:
the controller of a permanent put onto the battlefield attacking "chooses
which defending player, planeswalker a defending player controls, or
battle a defending player protects it's attacking ... unless the effect
that put it onto the battlefield specifies what it's attacking". What
blocked the slot was LAYERING and never the rules — `TokenRider` was
declared in `Words.idr`, four modules above `Noun` and `AttackDefender`.
The move is made: `TokenRider` now sits in `Triggers.idr` directly below
`AttackDefender`, is `Bindings`-indexed like every other
phrase-carrying vocabulary, and `EntersAttacking` takes the same
`AttackDefender` the declaration event and `BecomesAttacking` take, so
[CR#506.3]'s closed set and [CR#508.1b]'s one-defender rule ride it
unchanged. `NoDefender` is a real reading and not a missing one: the rule
itself fills an unwritten defender.
The threading fell out with no design left over — `MoveRiders` was
already `Bindings`-indexed, `Create`'s rider list sits at `amtIntro
count`, and `EntersRider`'s at `selfSubjIntro n`. **121 supported lines**
write an attacking entry or creation; **15 over 15 cards** specify the
defender. Benched whole on **Seraphic Greatsword**, where both
`AttackDefender` seats meet in one sentence.

**Pin — 1, in `ProofsG`.** `badTokenDuplicateSupertype` (a token created
with the word "legendary" twice; [CR#205.4b] makes a supertype a property
an object has or lacks, so the word twice is one fact twice). Checked
non-vacuous by mis-stating it to a single `Legendary` and confirming the
build rejects `Oh impossible`.
**No pin for the entry-rider defender and none for `IsHistoric`**, and
that is the honest record rather than an omission: the rider reuses
`AttackDefender`'s existing gates and states no new refusal (the
creature-defender and plural-defender pins already stand at the event
seat), and `IsHistoric` states no gate at all — its whole content is the
word, as `IsCard`'s and `IsToken`'s are, and neither of those carries a
pin either.

**Benches — 11 whole cards.** Dark Depths, Tuktuk the Explorer,
Artificer's Assistant, Aya of Alexandria, Blood Reckoning, Land Tax,
Indestructibility, Beckoning Will-o'-Wisp, Triarch Stalker, Seraphic
Greatsword, Unesh Criosphinx Sovereign. Dark Depths is the round's
finale and the card the whole day was pointed at: its state trigger over
an emptied counter store, `IfDone` over the mandatory sacrifice, and the
supertype on Marit Lage all had to be there at once.

**Gate.** `idris/scripts/build` from a cleared `build/ttc`, **23/23, 0
errors, 0 warnings**. `cargo xtask cite check --list-noncompliant` empty;
`cite check` 0 stale over 20699 citations; `cite bless` registered no new
rule (every rule cited this round was already in the lock). The round's
58 citation sites were audited with `jj diff --git | cargo xtask cite
audit --diff` and **three were rewritten**: a [CR#105.2a] cite for
colorlessness that names monocoloured objects (→ [CR#105.2c], "a
colorless object has no color"), a [CR#508.4a] cite for "an arrival never
attacked" that is about a defender leaving the game (→ [CR#508.4], which
says it outright), and a [CR#301.5f] cite carrying an "enchanted
permanent" claim on a rule about "equipped creature" (→ [CR#303.4m]).

## THE CAMPAIGN'S CLOSING RESIDUE LEDGER

The authoritative remainder record for the whole workbench effort. Every
item the sweep did not build, with its re-measured count, its named
blocker, and where it should live. Counts are this round's unless marked
CARRIED, in which case the named done ticket holds the measurement.

**A. Items with a live count and a named blocker**

1. **The ordinal-per-turn activation restrictor — 11 lines / 11 cards,
   two shapes, confirming the routed figure.** The ORDINAL, "the first
   [X] ability you activate each turn / during each of your turns": 6
   (Advancing the Spirit, Bruenor Battlehammer, Forge Anew, Kíli the
   Resourceful, Professor Hojo, Tezzeret Betrayer of Flesh). The NEXT,
   "the next [X] you activate this turn": 5 (Dynaheir Invoker Adept,
   Jaya's Phoenix, Pit Automaton clean; Magus Lucea Kane and Repeated
   Reverberation govern a cast-or-activate disjunction and are
   borderline). Nahiri, Storm of Stone is NOT one — "equip abilities you
   activate cost {1} less" carries no ordinal. Blocker: the ordinal
   family at the ACTIVATION seat, where `NthOccurrence` sits at the
   event seat. Mint the ordinal and the "next" separately or say why
   they are one. **Home: a cost/ordinal round.**
2. **The per-player partition — 3 lines / 3 cards strict, 4 more near.**
   Strict ("each player/opponent separates ... into piles"): Bend or
   Break, Make an Example, Whims of the Fates. Near, and worth deciding
   together: Stand or Fall and Fight or Flight write an outer iteration
   ("for each defending player, separate ..."), and Camouflage and
   Raging River write "divides" rather than "separates". The routed
   figure of 6 folded the first two of those in. Blocker: element
   scoping on `ForEachOf`, not a slot on the partition row. **Home: the
   partition family.**
3. **The face-down SPELL description — 7 lines / 7 cards, plus 1
   instruction.** Describing: Dream Chisel, Goblin Maskmaker, Kadena
   Slinking Sorcerer, Obscuring Aether, Panoptic Projektor, Qarsi
   Deceiver, Tin Street Gossip. Instructing a cast face down: Illusionary
   Mask. Blocker: [CR#110.5d] gives status to permanents alone, so the
   description needs a carrier that is not `HasStatus`. **Home: the
   face-down family.**
4. **The three remaining "last chosen" sorts — 8 lines.** The player sort
   landed this round, which makes the others worth a fresh probe rather
   than a re-derivation: the CARD (4 — Forgotten Lore, Shrouded Lore,
   Koh the Face Stealer, Paleontologist's Pick-Axe; recorded as Koh's
   blocker at `kohChooser`), the DIRECTION (2 — Mystic Barrier, Teyo
   Geometric Tactician; `QualitySort` has no direction row at all), the
   NUMBER (1 — Shapeshifter, which reads it as an AMOUNT and so wants an
   amount seat, not a predicate), and the paired NAME-and-creature-type
   (1 — Psychic Paper, a choice-payload gap). **Home: the marked-read
   family. Probe the card sort first; it is the largest and the player
   sort's lesson is that the machinery may already be there.**
5. **The `Choose` binder gap — 2 lines / 2 cards**, isolated out of 11
   supported "secretly choose" lines. `Choose` types noun and agent at
   the OUTER context, so "each player secretly chooses a creature THEY
   control" is unwritable: Call to the Void and Malik, Grim Manipulator
   are the only two whose chosen object is scoped to the chooser. A
   structural gap, not the secret protocol's. **Home: the binder layer.**
6. **The subtype retention payload — 2 lines / 2 cards**, confirmed
   isolated. Cavernous Maw ("It's still a Cave land") and Duplicant
   ("It's still a Shapeshifter"); `SetsType.ret` is `Maybe CardType` and
   every other "still a/an X" line names a card type. **Home: a rider
   round, or drop at two carriers.**
7. **The Room pair.** The effect-level ACT DISJUNCTION over one door —
   **2 lines / 2 cards** (Keys to the House, Marina Vendrell), plus
   Ghostly Dancers; not `Modal`'s bulleted spell [CR#700.2], and it
   brings [CR#709.5g]'s Lock label with it. The PLURAL door count — **4
   lines / 3 cards** (Central Elevator, Rampaging Soulrager, Smoky
   Lounge twice); halves of a described group with distinct names per
   [CR#709.5]. **Home: the Room family.**
8. **The all-headless PRESUPPOSITION disagreement — 3 lines** (Sonar
   Strike, Tetsuo Umezawa, Dire Downdraft). CARRIED from
   done/workbench-mixed-head-disjunction; it carries the Talion
   `Compare`-list precedent's own question. **Home: the disjunction
   family.**
9. **The pile family's five.** The EXILE-MADE pile plus the shuffle's
   pile arm (waits on cloak/manifest; [CR#701.24a]'s gap recorded);
   Boneyard Parley's `theRestOk` two-standing-groups question; `Reveal`'s
   missing participle (Truth or Tale); the exclusive-permission arm
   ("only [n] can" — Fight or Flight, Stand or Fall); and Sauron's
   Ransom's Ring-tempts INSTRUCTION. CARRIED from
   done/workbench-pile-contents-and-faces. **Home: the pile family.**
10. **The ordinal-player cluster.** `NthPlayer` (6 cards, all blocked
    behind `CompareOver`'s member unnameable past an outer singular and
    `Opponent`'s missing possessor — `OpponentOf` the cheaper shape) and
    the SELF-DESCRIBING description ("each player who searched their
    library this way", 9 carriers). CARRIED from
    done/workbench-each-player-binder, which holds the probes.
11. **The attacking-defender round's small residues** — `Other` outside a
    targeted context, the player-kind partitive, the condition-gated
    deontic, two reads of one amount, the combat-damage/block assignment
    decision. CARRIED; counts in
    done/workbench-attacking-defender-attributive.md.
12. **The disjunct-head type read.** `nounTy` collapses a coordinated
    head, so an excluded disjunct is no longer refused. CARRIED from
    done/workbench-prohibition-tail; a gate defect, not a count. **This
    is the ledger's one correctness item and should outrank the rest.**

**B. Blockers this round newly diagnosed**

13. **`LibrarySlice`'s size slot drops its amount's bindings.** The row
    takes `amt : Amount bs` and returns a noun at `bs`, so an amount
    written there contributes nothing to the discourse and a LETTER
    written there is never opened — `Define X` after it fails at
    `anyOpenLetter X`, because `openLetter` wants an indefinite singular
    letter mention and the slice introduced none. Every other letter seat
    threads its amount (`DealDamage`, `Create`, `Search`), which is why
    Tribal Flames and Krenko write "where X is" and **Sphinx of Clear
    Skies** cannot. 1 known carrier. Whether the slice should thread
    `amtIntro` is a question about that row. **Home: the amount layer.**
14. **A second plural object mention breaks `Them` at the pile
    separator.** Sphinx of Clear Skies again: the domain count's own
    plural domain ("lands you control") collides with the revealed
    cards', and `SeparateIntoPiles ... Them` fails at `countManys Object
    = 1`. The printed spelling is the fix and not a gap — the card writes
    "separates THOSE CARDS", which is `Those CardW` — so this is a note
    for whoever benches the card, not an item. Recorded so the next
    attempt does not re-derive it.

**C. Singletons held for a second carrier**

15. **The `MoveCounters` partitive source — 1 line / 1 card** (Slippery
    Bogbonder). Confirmed a third time: 16 supported lines write
    "counters from among" and all but one are REMOVE, a different row
    that already writes.
16. **Master Biomancer's entry-time type ascription — 1 line / 1 card.**
    Sole carrier; no other line pairs an entry with a type ascription.
17. **The Temporal Anchor step-event — 1 line / 1 card.** Confirmed sole
    carrier: an event naming a step INSIDE a keyword action, which no
    event row names.
18. **Flameskull's two-exile batch read** and **Breath of Fury's
    coordinated antecedent** (split `EncNotOneAction` into agent-half and
    count-half if a second carrier appears). CARRIED from
    done/workbench-mandatory-if-you-do; not re-measured.

**D. Declined, with live successors — the campaign's other remainder**

These are not this ticket's bullets; they are
done/workbench-event-disjunction-tail's own remainder list, recorded here
so the ledger is the whole board.

19. **The general event disjunction: DECLINED**, with the TAIL-ALIGNED
    UNION as its live successor. Align two announcement lists on their
    shared END in `unionBindings` instead of requiring equal length. It
    would move `badThreeArmHeaderReadback` and would also move
    `badAltHeaderMixedReadback` onto a determinate but WRONG referent, so
    the question to settle first is whether a union may do that. Shape
    and argument recorded at `sharedCtx` in `Triggers.idr`.
20. **The `Causer`-as-noun widening** — `Words.Causer`'s nullary
    `AnEffect` at `CreationVoice`/`CausedBy`/`TokensCreated`/
    `CounterEvent`, plus `badCausedCounterWithAgent`. Three payers, each
    carrying a second blocker of its own (Zabaz, Karmic Justice, Cobra
    Trap); Crafty Cutpurse rides with them.
21. **Negated causation — 17 lines / 17 cards**, three shapes (the
    entry-trigger denial, the source-side prohibition, the Clockwork
    ceiling). Wants the causation at a DEED.
22. **The coordinated causer — 2 lines** (Veyran, Gandalf the White).
    `Causing` takes one event; the arm list is the header seat's.

### What the ledger says about the board

Twelve items in section A, two diagnostics in B, four singletons in C,
four declined-with-successors in D. Of the twelve in A, **eleven are
family-shaped items waiting on a design decision and one (12, the
disjunct-head type read) is a correctness defect in a gate that
currently admits phrases it used to refuse.** That one should be
scheduled on its own and ahead of the rest; nothing else here is
urgent, and several (5, 6, 15, 16, 17) are honestly two-carrier or
one-carrier items that the workbench's own standing rule says to leave
unbuilt until a second carrier appears.

The round's real lesson is section-headline-sized: **six of this
bucket's routed items were already writable, and one had landed hours
earlier.** A routed blocker is a hypothesis recorded at the moment a
round closed, and the grammar moves under it. Before building anything
off a routed blocker, try the bench.
