# choice-E: the subtype words, the pronoun, and linkage

Sub-round E of [workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md)
(the umbrella). Owns its section "The ascription's last five subtype words,
its self-antecedent pronoun, and linkage", plus the routed backlog of subtype
rows named by closed rounds: `Fungus`, `Kor`, `Wolf`, `Arlinn`, `Drake`,
`Garruk`.

Pins:
- Every subtype word is paid for by a benched card writing it; two cells per
  word (`ascribesAsSubtype` True + `subtypeType`) are the WHOLE cost; the
  ascription itself is not reshaped. Same for the backlog rows: witness card
  or the row waits.
- The self-reference mention question (Soul Ransom's "sacrifices IT") is
  answered explicitly — announced or refused — with Soul Ransom benched or
  the refusal recorded with its rule.
- The linkage cell (`SortedSelfLinked`) opens only for subtypes whose rows
  exist; the Saga row's six lines become writable; the zero cells for rowed
  subtypes that never write it STAY zero.

Acceptance: the umbrella's subtype-word, self-reference and linkage lines.
`idris/scripts/build` PASS. Standard constraints apply.

## As landed

### The premise the round found stale: a subtype word costs nothing

The umbrella prices every subtype word at "one `ascribesAsSubtype` True cell
plus one `subtypeType` card type". Neither cell exists any more. The
`workbench-subtype-labels` round (done, 2026-08-26) replaced the subtype
catalog with `MkSubtype : (host : CardType) -> (label : String) -> Subtype`
and the class helpers (`creatureType "Zombie"`, `artifactType "Vehicle"`, …),
and `ascriptionOk t (Just s) = ascribesAsType t && subtypeType s == t` reads
the host straight off the word. There is no per-word table left to extend and
no `ascribesAsSubtype` census to widen: naming a subtype IS writing it, which
is what `rulings/subtypes-are-labels.md` asked for.

So the round's whole cost is the pin's other half — a BENCHED card writing
each word — and that is what landed. The closure grid's second table
(`ascribesAsSubtype`'s nine-self-naming-words census) is a measurement of a
gate that no longer exists; the first (the `Subtype` catalog's rows) is now a
census of the bench, not of the grammar.

### The five subtype words, each paid for

Every whole card here is blocked by keyword machinery this round does not
build (`KnownKeyword` has no `Crew` and no `Station`, and there is no level,
solve or door vocabulary), so each word is paid for by the printed line that
writes it, benched as an ability or an effect.

| word | witness benched | whole card waits on |
| --- | --- | --- |
| `Vehicle` | `debrisBeetleTrigger`, `nautiloidShipTrigger` | crew |
| `Class` | `rogueClassLevelThree` | the level machinery |
| `Spacecraft` | `wurmwallSweeperTrigger` | station |
| `Case` | `caseOfTheCrimsonPulseTrigger` | the to-solve/solved clauses |
| `Room` | `glassworksTrigger` | the door machinery |

No cross-links, no design question, and the ascription itself is untouched.

### The routed backlog rows

`Fungus`, `Kor` and `Arlinn` were already discharged by the label conversion
and are written on the bench today (The Skullspore Nexus' token, Conqueror's
Pledge's token, the Arlinn back face's type line). The three that were not:

- `Wolf` — `ferociousPup`, whole card; the word on the type line and on the
  token it makes.
- `Drake` — `talrandsInvocation`, whole card.
- `Garruk` — `predatoryWurm`, whole card. The planeswalker type read as a
  DESCRIPTION [CR#109.2] rather than off a type line; Garruk Relentless and
  Arlinn, Voice of the Pack stay blocked on the transform verb.

### The self-antecedent pronoun: ANNOUNCED

Soul Ransom's "This Aura's controller sacrifices it, then draws two cards"
benches as `soulRansomRansom`. The decision and its reasoning:

A possessive has always announced whatever its base announces —
`nounDelta (ControllerOf n)` threaded `nounDelta n`, so "target creature's
controller … it" wrote before this round. The only asymmetry was that a
DEICTIC base has no `nounDelta` of its own. `selfSubjDelta`'s rows are exactly
the announcement a deictic makes instead, and they were read at one seat only
(a clause's subject, through `selfSubjIntro`). The possessive's base is the
other seat where a deictic is named in full, so the two relational rows now
thread it:

    nounDelta (ControllerOf n) =
      MkBinding TheD Player OneOf PlayerP :: (selfSubjDelta n ++ nounDelta n)

This is a mint, not a gate: no constructor reads anything new, the shape stays
`delta ++ bs`, and `ControllerOf n` derives its rows from its own single
argument, so clauses 1, 2 and 4 of
`decisions/oracle-text-is-forward-anaphoric.md` are untouched and
`ProofsAnaphora`'s prefix-split lemmas hold unchanged.

Refusing was the alternative, on "deixis is not anaphora". It is the wrong
reading of that clause: the ADR's deixis paragraph says a deictic READS no
context, not that it announces none — `selfSubjDelta` already announces one at
the subject seat. Refusing would have kept a described base and a deictic base
behaving differently for no rules reason.

The announcement is at `SelfD`, and `wordNow` returns False for `SelfD`, so
"it" reaches the named object and the demonstrative words still do not. Three
`Refl` measurements carry that: `possessiveDeicticIsReadableByIt` (1 candidate
at the sacrifice slot's permanent carrier [CR#109.2,701.21a]),
`possessiveDeicticIsNotADemonstrative` (0 for `TypeW Enchantment`), and
`possessiveDescribedBaseUnchanged` (a described base still announces exactly
one). The one pre-existing deictic possessive on the bench, Phyrexian
Infiltrator's `ControllerOf Macros.thisCreature` followed by
`That (TypeW Creature)`, is unaffected for the same reason.

The possessive read itself (`ControllerOf` over the ascription) needed nothing,
as measured. Soul Ransom's whole card waits on an activation restriction
naming who may activate ("Only your opponents may activate this ability").

### Linkage under a subtype word

`LinkSource`'s `SortedSelfLinked` hard-coded `Nothing` in the ascription's
subtype slot; it now takes the slot as an index, so "cards exiled with this
[subtype]" writes wherever the ascription does. Because subtypes are labels,
the cell cannot open per-word: it opens once, for every subtype word at once.
That is a deviation from the umbrella's "opens only for subtypes whose rows
exist" and it is forced by the label conversion, not chosen — with no catalog
there is no per-word cell to gate. The pin the umbrella was protecting is
still honoured on the bench side: no line is benched for a subtype that writes
none.

Measured census over supported cards (`exiled with this <word>`): Saga 6,
Vehicle 3, Class 1, Equipment 1; Aura 0, Curse 0, Siege 0. The zeros stay
zero — nothing was benched for them.

Landed:
- Saga — `summonEsperValigarmandaCast` (Summon: Esper Valigarmanda's
  II/III/IV, in part).
- Vehicle — `nautiloidShipTrigger`.
- Class — `rogueClassLevelThree`.
- Equipment — `drachNyen` UPGRADED. Choice-A wrote its read against
  `thisArtifact` and said in the docstring that `thisEquipment` wanted the
  shut cell. The card prints no linkage word at all, so the self-word is the
  bench's to pick, and the card's own enters trigger already writes
  `thisEquipment`; the card-type spelling was an inconsistency the shut cell
  forced. Upgraded.

`badExiledWithOtherSource`, `badExiledWithDescribedSource` and
`badExiledWithControlled` are unchanged and still hold.

### Remainders (ledger)

- **A possessed zone is not a move destination.** `DestOk` admits bare zones
  only, so "Put a card exiled with this Saga into its owner's hand" (Roads Go
  Ever, Ever On II/III) does not write. Not a linkage gap — a
  move-destination one, and it blocks the plainest of the six Saga lines.
- **"Mana of any type/colour can be spent to cast that spell"** has no
  carrier: `PlayAsThough` is `HadFlash` and nothing else. It is the clause
  beside the linkage read on Rogue Class, King Narfi's Betrayal and Summon:
  Esper Valigarmanda.
- **An activation restriction naming who may activate** ("Only your opponents
  may activate this ability") — Soul Ransom's whole card.
- **Crew, station, the Class level ladder, the Case to-solve/solved pair and
  the Room door vocabulary** — the five whole cards behind this round's five
  subtype words.
- **Assimilation Aegis**, the one printed "exiled with this Equipment" line,
  waits on the attach-triggered chooser (sub-round C's region), not on this
  cell.
