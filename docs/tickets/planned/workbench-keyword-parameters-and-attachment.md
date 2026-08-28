---
needs: []
---
# Finish the keyword row's parameters, its grants and the attachment relation

Everything the keyword row still owes: the typed parameter slot and its
remaining payload sorts, the keyword-CLASS quantifier the extension anaphor
needs, the grant residues, the Aura carve-out rider, the equip line's compound
parameter and cost-reduction axis, and the attachment participle's own cells.
These are one claimable unit because they all move the same three files — the
`Keyword` catalog and its parameter shapes in `Words.idr`, the grant and
extension constructors in `Experimental.idr`, and the attachment reads that sit
beside them — and because several of the residues below are blocked on each
other (affinity's class parameter and annihilator's number parameter want one
slot; `fear` and `shroud` land only with the class term; the equipped-permanent
cell flips only with the negated type setting).

## Standing scope fence

Adding keywords that will eventually become macros is OUT of scope until the
verifier transition — the new Idris model is intended to mirror the macro shape
more accurately (string labels for composites), and Flying and Haste are
deliberate existing passes. Already-implemented keywords MAY be used where they
unlock better `Cards.idr` witnesses.

From the v1 comparison (2026-08-24): the closed-enum-vs-declared-atom question this fence defers is the same one [workbench-effect-basis-realign](workbench-effect-basis-realign.md) asks about `VerbName`, and the two should get the same answer or an explicit difference.

## AFFINITY's typed parameter — 5 lines, every one parameterised

"affinity for artifacts" (Mycosynth Golem; Sami, Wildcat Captain; Tezzeret),
"for Auras" (Pearl-Ear), "for creatures" (Witherbloom). The `Keyword` row has no slot,
and the parameter is a described CLASS rather than a number, so this is a
different shape from the numeric keyword parameters (annihilator X, mobilize X).
Schedule them together and mint the slot once.

## The remaining parameterized [CR#702] keywords

The slot is minted (`KeywordParamShape`/`KeywordParam`) with ward (cost) and
protection (quality) in it, and there is NO SHAPE LEFT TO MINT: cost, quality,
subject and number are all built and carry rows. A row is a `Keyword`
constructor, a `keywordParamShape` cell, and its four attestation cells — the
mechanism does not move again.

- ANNIHILATOR — [CR#702.86a] "Annihilator N", 21 supported lines. A
  `keywordParamShape` cell of `NumberParam` plus four attestation cells; waits on
  a consumer alone.
- AFFINITY — "affinity for artifacts", 5 lines. A quality; waits only on a
  witness.

Equip, ascend, storied and renown have LANDED — do not re-file them here.

## FLASH's grant — 1 line, and that line unsupported

Deferred on evidence, not on cost; it is a cheap word whenever a supported line
wants it. Chapter eighty-four's re-measurement narrows the deferral to the GRANT:
as a printed keyword LINE flash is 500 supported cards, and one of them is a
migrated witness waiting on the word alone (Amphibian Downpour, the Frogify
sentence under Flash and Storm).

## What chapter sixty-six landed — context, not scope

Chapter sixty-six landed the keyword grant (`GrantSubject`, coupling the granted
keyword to the subject's place, and `keywordStackRegime`, deciding which relation
may describe a spell class) together with convoke, improvise, storm and lifelink.
What stayed out is the parameter gap and the evidence-deferred word above,
alongside the blockers recorded below.

## Recorded, not rows for this round

- **The nine keywords core cannot resolve** — cascade, replicate, emerge,
  conspire, rebound, prowl, freerunning, demonstrate, sticker kicker. No macro
  under `plugins/builtin/macros/keyword/` and zero built-card use, so a grammar
  row would name a word the layer below has nothing for. This is a CORE-side gap:
  The First Sliver's own printed cascade resolves to nothing today, so the card is
  unbuildable whatever the grammar says about its second line; Cast Through Time
  (rebound) and Flamekin Herald (cascade) wait with it.
- **The next-spell and first-spell grants are not this row's** — [CR#611.2f]'s
  regime, wanting the ordinal binder in
  [workbench-event-zone-and-cast-provenance](workbench-event-zone-and-cast-provenance.md).
  The as-though-flash family is a play PERMISSION and not a grant at all.
- **Two cards blocked past the keyword row.** The premise behind them closed with
  chapter seventy-seven, which also landed one of the three: the phrase was never
  the explicit tie sentence but chapter sixty-eight's FOLDED clause stating the
  superlative's denotation ("the player with the most life or tied for most
  life"), and with chapter seventy-six's defender slot Undercover Butler is
  benched whole. Seraphic Greatsword writes the same phrase off an equipped
  creature. What remains is PREACHER OF THE SCHISM, blocked on its third line's
  header-internal "while" clause (the ledgered family), and SCOURGE OF THE THRONE,
  blocked on three at once — the dethrone KEYWORD as a printed line, the "for the
  first time each turn" rider, and the additional combat phase.

## The keyword-CLASS entry — 7 cards

The keyword-list extension landed as `AlsoForKeywords ab ks` on `AbilityAt`.
Seven of its fifteen lists are unwritable because their trailer names a CLASS of
keywords rather than keywords — a quantifier over words, which the row's
paramless demand refuses and which `HasKeyword` refuses one position over, so
the base sentence cannot be written for them either.

- "protection" bare — Cairn Wanderer, Concerted Effort, Death-Mask Duplicant,
  Eater of Virtue, Rayami, Wretched Bonemass
- "protection from any color" — Escaped Shapeshifter
- "landwalk" — Cairn Wanderer, Concerted Effort, Death-Mask Duplicant

Not a word but a QUANTIFIER over words: every landwalk keyword, every protection
quality. Attested, so counted rather than pinned. It wants a keyword-class term
this grammar has at NO site. Eight of the fifteen lists are clean; seven are
not.

### Indominus Rex, Alpha

Its list quantifies over which COUNTER to place, not over which keyword to
grant — a different base, routed here because it shares the anaphor. Also the
family's only three-sentence line (the other 14 of 15 are two sentences with the
trailer second).

### `fear` (3 lines) and `shroud` (1 line) — deliberately not bought

Paramless and mintable, but their only carriers are the class-blocked seven.
They land when the class term does, not before.

### The unbenched clean carriers — each one line away, none blocked on this row

Animus of Predation (a draft-time read), Soulflayer (delve's linkage phrase,
"exiled with this creature's delve ability"), Majestic Myriarch (a doubling
CDA), Thunderous Orator (wants `Subtype.Kor`).

### Ward of Bones' card-type extension — ONE card, nothing to design from yet

"Each opponent who controls more creatures than you can't cast creature spells.
The same is true for artifacts and enchantments." The third construction sharing
this anaphor (finding 846): it extends a PROHIBITION across CARD TYPES, where
one sibling extends across zones and the other across keywords. The count is
confirmed at one, and nothing is designable until a second carrier appears.
Recorded here so the count is not re-derived and so this card is not
double-counted as a keyword-list carrier — it carries no keyword list at all.

### What already landed on the extension — do not re-buy

`AlsoForKeywords ab ks` sits on `AbilityAt`, NOT on the statement, because the
family repeats a STATIC line on nine cards and a TRIGGERED one on five, and the
unit the trailer never crosses is the line. The list is measured nonempty
(3..12), distinct, paramless, and disjoint from the base word 15/15 (ODRIC,
whose base is first strike and whose list holds flying, is what makes the last
constraint visible). The base was never this row's: the keyword-conditional
grant elaborated before the round began, both carriers, and the corpus writes it
with no trailer at all (Mutual Destruction, Manor Gargoyle) — the row carries
only the list. `Hexproof`, `Menace` and `Skulk` are bought, each spent by a
benched card; Odric Lunarch Marshal, Bleeding Effect and Urborg Scavengers are
benched whole. The family is FIFTEEN lines, not sixteen.

## The keyword parameter's remaining payloads and the Aura carve-out

The keyword parameter landed indexed, with its payload riding the grant ([CR#607.1a]
makes a granted ability count as printed text) and exactly one of its four rows
using the index; the other three record measured zeros — ward cost, enchant
restriction, renown count, **0 chosen reads apiece**. What it did not reach is
three payload sorts and one trailing rider, all on the same carriers.

Authority: [The kind index joins; union marking is
spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md) — the
player-payload question below is settled per round on the payload's algebra, not
by an appeal to which surfaces mark a player.

### Protection from a player — 2 cards, and a third at the other end

True-Name Nemesis and Guardian Archon want a `Predicate bs Player` payload, which
the quality payload does not take. Runed Halo is the same blocker from the other
side: its phrase elaborates today (probed — the chosen-name parameter writes), so
what blocks the card is that its **subject** is a player where `Gains` takes an
object. One line.

### Protection from a card type — 1 card

Serra's Emissary wants a `QualitySort` value that does not exist. (An earlier
reading paired it with protection from a card *name*; the probe above corrects
that — the card-name cell writes.)

### The Aura carve-out rider — 15 supported lines in four spellings

The seven Aura carriers of the granted keyword each trail a rider with no row —
Cho-Manno's Blessing, Flickering Ward, Pentarch Ward, Ward of Lights, Benevolent
Blessing, Floating Shield among them. 12 lines write "This effect doesn't remove
this Aura"; the other three name Auras and Equipment already attached. All are
same-line trailers like the keyword extension but point the **opposite way** —
they carve an object out of a statement's scope where the extension widens it.
Its own construction, and the cheapest six-card win in the area.

### The emblem's chosen read — 1 card

One emblem reads a chosen value through `GetsEmblem`'s bs-free list (Oko, Lorwyn
Liege). One card, one index, otherwise unscheduled.

### Keyword catalog holes standing behind whole cards

Each is one named blocker away and none of them is this construction; recorded so
the next keyword round can sweep them:

- Bloodline Pretender — `Changeling`, a missing `Keyword` catalog entry, and
  nothing else (its triggered header probed green and the same shape is benched
  on Prism Ring).
- Disruptor Flute — Flash, beside two already writable lines.
- Skyseer's Chariot — Vehicle and Crew.
- Petrified Hamlet — a quoted mana ability granted to a class.
- Sorcerous Spyglass, Anointed Peacekeeper — "look at an opponent's hand" inside
  the entry choice.

## The status event's SECOND READER

`statusEventOk` is one table asked by the trigger header and, once, by a
DURATION ENDPOINT. Vesuvan Shapeshifter's "until this creature is turned face
down" is the clause's only supported occurrence anywhere, and the FaceDown cell
refuses it along with the header it correctly refuses (0 headers, against the up
cell's 125). One table, two readers, disagreeing on exactly one cell — the split
is cheap and was deferred only for want of room in the chapter that found it.

It travels with `eventSpan`, whose Unattested row for `StatusChange` is the same
question asked of the tap pair.

## The restricted equip compound — 21 lines, 20 distinct shapes

21 supported lines carry BOTH a quality and a cost: "Equip legendary creature
{1}" (2), "Equip commander {3}", "Equip Knight {1}", "Equip Soldier {W}" and the
rest — 20 distinct shapes over 21 lines.

[CR#702.6c] describes the quality as an additional restriction on the ability's
**TARGET**, not on what the Equipment may be attached to — a fact worth keeping,
since it means the quality does **not** compose with the attachment.
`CostParam` cannot hold both and neither shape alone can; core has no compound
precedent for equip either, only the reinforce keyword's `[Count, Cost]` proving
multi-parameter possible at all.

Deliberately not minted when the equip row landed: no witness demanded it, and a
shape with no consumer is what the open-catalog ruling refuses. **So this round
arrives with the witness or does not mint.** Neighbours [CR#702.6e]'s "Equip
planeswalker", a variant of the same shape.

## The equip cost-reduction statics — 9 + 7 lines, a different axis

Ghostfire Blade's line and its family, and **not** part of the equip keyword
line. 9 supported lines write "Equip abilities you activate … cost {N} less to
activate", plus 7 more that append a reduction sentence to a bare equip line
("Equip {4}. This ability costs {3} less to activate if you're the monarch.").
Cost modification over an ability CLASS is a different axis from the keyword row
entirely — the row landed and Ghostfire Blade still does not, on this line
alone.

## The attachment's own direction — 12 lines

"As long as this Equipment is attached to a creature", 12 lines: the THIRD
direction, naming the relation from the attachment's side, where the host's
participle names attachment→host and the presence reads name host→has-
attachment. Small, and it is the one place an attach relation would actually be
written.

## The attachment participle's two cells

Two attachment cells, one attested against the record and one attested against a
False table entry. Both are readings of the attachment participle, and the
second must not be scheduled alone.

- **The prenominal attachment predicate**, 18 occurrences over 18 cards:
  "equipped creatures you control have flying" (Dalakos Crafter of Wonders,
  Blacksmith's Talent, Kemba Kha Enduring, Stone Haven Outfitter, Greater
  Auramancy, A Tale for the Ages, Syr Armont, Zamriel, Firion, Resistance
  Reunited). Finding 369 minted the inverse direction as a PREDICATE and
  recorded that the corpus writes it "predicatively after the copula and never
  prenominally"; these lines are the prenominal writing, so the record is half
  wrong and the reader is `attachedCheckOk`'s, not `AttachHost`'s. **Check
  whether the participle is simply the predicate in attributive position
  (finding 275 on the copula) before minting anything.**
- **`attachHeadOk Equipped PermanentW`, one attested line against a False
  cell** (finding 625). Luxior, Giada's Gift writes "Equipped permanent isn't a
  planeswalker and is a creature in addition to its other types", where the cell
  says the equipped participle takes no permanent head. The cell is rules-backed
  ([CR#301.5]: an Equipment "can't legally be attached to anything that isn't a
  creature") and so is the exception ([CR#702.6e]: "equip planeswalker" attaches
  "as though that planeswalker were a creature"), which is why the card cannot
  write "equipped creature". **No pin stands on the cell.** Flipping it buys
  nothing until the line's OTHER half exists — a negated type setting, "isn't a
  planeswalker" — so schedule the two together or not at all.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.1.

`attachHeadOk`'s participle × head grid (`Words.idr:2211`). One of its cells —
`Equipped` × `PermanentW`, coded False beside a comment citing Luxior's printed
"equipped permanent" — is a live discrepancy carried by
the `Measured cell` ledger on `workbench-static-frame-and-ability-values` (the docstring-audit ticket was deleted 2026-08-21); settle it there, not here.

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **`attachHeadOk Enchanted _ = True` now covers the six command-zone types**,
  so "enchanted conspiracy" is admitted. The pre-existing catch-all already
  admitted "enchanted instant"; a rule tightening it belongs with the attachment
  relation — `docs/tickets/done/workbench-closure-flip-risks.md`.
- **Ward with a cost.** Iymrith, Desert Doom's "has ward {4} as long as it's
  untapped" was benched at Dragonlord Ojutai's cost-free spelling of the same
  sentence because the parameterised ward row is unbuilt —
  `docs/tickets/done/workbench-conditional-and-coordination.md`.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`Keyword`, `KeywordParamShape`, the keyword
catalog, a new keyword-class term, `QualitySort`, keyword spelling and
attestation cells, `CostParam`'s word side, the attachment participle and
presence reads, `attachedCheckOk`, `attachHeadOk`, `AttachHost`, the participle's
attributive position), `idris/src/Experimental.idr` (`KeywordParam`,
`KeywordParamFits`, `GrantSubject`, `keywordStackRegime`, `AlsoForKeywords`,
`HasKeyword`, `AbilityAt`, `ParamQuality`, `Gains`, `GetsEmblem`, the equip
keyword row, `CostParam`, `AttachHost`, the cost-modification statics, the
negated type setting), `idris/src/Experimental/Events.idr` (`statusEventOk`,
`eventSpan`), `idris/src/Experimental/Macros.idr` (the trailer's spelling and the
equip line's spelling), the pin modules `idris/src/Experimental/Proofs*.idr`
(including `ProofsE.idr` and `ProofsG.idr`), evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. The core macro directory
`plugins/builtin/macros/keyword/` is named only as the reason the nine keywords
stay unwritten; this round does not touch it.

## Acceptance

- One parameter slot serves both the described-class parameter and the numeric
  keyword parameters; affinity's 5 lines land.
- Each keyword row added is paid for by a benched consumer or witness; no fifth
  param shape is minted.
- Flash's grant lands with Amphibian Downpour as its witness, or the deferral is
  restated against the 500-line printed-line count.
- The nine unresolvable keywords remain unwritten, with the reason recorded where
  the keyword vocabulary is defined.
- A keyword-class term exists and serves BOTH positions — the extension list and
  `HasKeyword` — so the base sentence writes for the seven as well as the
  trailer.
- The paramless/distinct/disjoint constraints on `AlsoForKeywords` survive; the
  class term is not smuggled in by relaxing them.
- `fear` and `shroud` are bought only alongside the class term.
- Ward of Bones stays at one and is not counted as a keyword-list carrier.
- No new keyword is added merely because it is cheap — see the scope fence.
- The three measured zeros on the unused parameter rows survive the round as
  zeros.
- The carve-out is written as its own construction, not as a sign flip on the
  scope-widening extension.
- Whole cards benched from the carve-out's carriers, not fragments.
- The header reader keeps its FaceDown refusal (0 headers) while the duration
  endpoint admits Vesuvan Shapeshifter — the split is two tables, not a widened
  cell.
- `eventSpan`'s `StatusChange` row is answered in the same pass as the tap pair
  or explicitly left, not silently widened.
- The compound equip parameter is minted only with the card that demands it, and
  the quality lands as a target restriction that does not compose with the
  attachment ([CR#702.6c]); "Equip planeswalker" is covered by the same shape.
- The cost reductions land on the cost-modification axis, not as an equip row
  variant, and Ghostfire Blade benches.
- The attachment's own direction is a third direction, not a re-spelling of the
  participle or of the presence read.
- Finding 369's "never prenominally" record is corrected in place, with the
  attributive-position question answered explicitly rather than assumed.
- The 18 prenominal occurrences write.
- The `Equipped PermanentW` cell flips only together with the negated type
  setting, and Luxior, Giada's Gift benches on the pair.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-axis-distributive-reads (close, 2026-08-26):** landwalk is entirely absent from the grammar (Magnigoth Treefolk's "landwalk of that type" is doubly blocked: no landwalk keyword param family, and the land-subtype quality sort routed to workbench-quality-sort-gaps); Sunbird Effigy's domain needs craft machinery. Keyword-parameter families, so they land here.

- **Routed from workbench-choice-e-subtype-words-and-linkage (close, 2026-08-27):**
  the keyword vocabularies behind the five subtype words' whole cards — Crew
  (Debris Beetle, Nautiloid Ship), Station (Wurmwall Sweeper), the Class level
  ladder (Rogue Class), the Case to-solve/solved pair (Case of the Crimson
  Pulse), the Room door vocabulary (Glassworks // Industrial Advancement).
  `KnownKeyword` carries none of them.
