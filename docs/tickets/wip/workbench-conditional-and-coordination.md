---
needs: []
---
# Type both conditional orientations and write the coordinations the grammar still refuses

The conditional containers and the coordination family in one claimable unit. Both
halves are about joining two things at one site: a condition to its consequent in
either orientation, and two nouns, phrases, zones or players under one head. They
share `idris/src/Experimental.idr`'s coordination region and the same evidence
bench, and several of the coordination sub-areas exist only because they are
explicitly NOT cross-kind union questions — that scoping is recorded per section
below and must not be re-opened.

## Conditional orientations and containers

One design across both conditional containers, plus the negation boundary found
inside the settled family. The orientation question has grown a second payer
since chapter 58 and is the whole blocker on three cards.

- **The STATIC orientation.** About 70 lines pronominalise the STATEMENT's
  subject inside the condition ("X has hexproof as long as IT's untapped"),
  where chapter 50's fronted flow pronominalises the CONDITION's subject in the
  body.
- **The ONE-SHOT orientation.** `Effect.If` types the trailing orientation only
  — its condition sits at `preIntro e`, which is what `badTrailingPostStateZone`
  refuses a post-state zone at — while SPELLING both. So a fronted condition
  cannot contribute anything to its own consequent, which costs Balance of
  Power, Vraska's [−9] and Iymrith their gap read. Balance of Power's condition
  half compiles today, so the orientation is that card's whole blocker.
- **One constructor cannot type both arguments in each other's context**
  (finding 362). Both halves are therefore SECOND ORIENTATIONS — carried by the
  marking or by the order, whichever the corpus supports — and not a second
  `condIntro`. The independent-condition trailing forms are writable today and
  are NOT part of this.
- **The counted-condition "unless"** (finding 354), small: the conditional
  static's `Unless` marking demands a NEGATABLE condition and `condNegatable`
  answers False for `CompareAmt`, so "unless you control an artifact" composes
  and "unless you control four or more artifacts" (Gadrak) does not. It belongs
  to the comparison's negation gap, not the deontic's. Measure before
  scheduling.

## The condition disjunction "if X or if Y"

The condition CONJUNCTION landed and this was deliberately left beside it because
the number is not known. The round that takes it owes a hand count before a row:
the doubly-marked lines are a lower bound and the sweep cannot separate the rest
from coordinations that are already spelled.

### What is measured

- 11 supported lines mark BOTH halves: "Activate only if this land entered this
  turn or if you control a basic land" — Dark Fortress, Gathering Place, Gleaming
  Bastion, Hidden Lair and Training Compound are one reprint cycle; plus Armored
  Kincaller, Bonecache Overseer, Dragon's Disciple, Mythos of Nethroi,
  Quakebringer, Reptilian Recruiter.
- Those 11 are a LOWER BOUND: a singly-marked disjunction is indistinguishable by
  sweep from the coordinations already spelled.
- The raw `if…or…` population is 1,274, still 360 after the comparator idiom
  ("two or more", "4 or greater") is stripped, and it is dominated by the same
  one-condition coordinations (`Predicate.Or`) that were the conjunction round's
  largest false-positive bucket.

### The hand count comes first

360 lines, sorted the way the conjunction round sorted its 182 — and only then
the row. A row minted against the 11 alone is sized against a lower bound.

### Settled going in

- The row sits beside `AndCond` on `Condition` and reaches the same five carriers
  for free.
- The arity and flatness demands are already written (`atLeastTwoCs`,
  `flatConjuncts`) and want one decision: may a disjunct be a conjunction?
  Quakebringer's "if Quakebringer is on the battlefield or if Quakebringer is in
  your graveyard and you control a Giant" is an OR of an AND, so the answer is
  probably yes — and `FlatConjuncts` would then need a COMPANION rather than a
  copy.
- The ACTIVATION GUARD is this family's biggest carrier (6 of the 11) where it
  was the conjunction's empty one.

## Coordinations the derived coordinator does not cover

Chapter sixty-five derived the word this row already writes — "or" under a
singular determiner or negative polarity, "and" under an affirmative plural
class head. Four coordinations sit outside that derivation, each with its own
truth condition or its own refused projection. They live in one region, so a
round that opens one has to answer the rest.

### The coordinated-destination move

"Put that card into your graveyard and the rest into your hand" (Murmurs from
Beyond, the one elision that card's bench line names). One verb, two phrases,
two destinations, joined by "and" rather than by a sequence word. The grammar
writes it as two `Move` clauses under `Sequentially`, which says the same thing
and spells "then" where the card spells "and". Take the measurement before the
row: the coordination may be general across verbs rather than the move's own.

### "and/or" is a THIRD coordinator — 365 supported lines

Outside reminder text: "the number of tapped artifacts and/or creatures you
control", "search your graveyard, hand and/or library". Either category alone or
both together qualifies — a different truth condition, not an environment of the
derived word. The parser already keeps `Conjunction::AndOr` beside `And` and
`Or`. The big surfaces are a counted domain and a multi-zone search, so this
wants the rows it coordinates as much as it wants a constructor.

### The KIND-crossing disjunction — one kind, type crossed with subtype

These cross a card TYPE with an artifact SUBTYPE at ONE `Kind`; they are not the
player/object crossing and the cross-kind union work has nothing to say about
them.

Authority: [The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— cross-kind mentions are joins on the kind index and the marked union
constructions' tables are spelling-boundary knowledge, so do not model this
within-kind disjunction on a union head.

SUGAR COAT is the precisely-known whole card and has been down to this one thing
since chapter eighty-seven: its flash line is a keyword row, its "loses all other
card types and abilities" is a spelling (finding 630), and its quoted `{2}, {T},
Sacrifice this artifact:` payload became writable when `TokenChars` started
holding abilities. What is left is "Enchant creature or Food", which brings the
`Food` subtype row with it (finding 631). IN TOO DEEP and MINIMUS CONTAINMENT
are the same sentence with the same one blocker apiece ("creature, planeswalker,
or Clue"; a mana ability inside the quotation).

### The CROSS-ZONE disjunction — 17 lines

"an Equipment card from your hand or graveyard", a shared preposition over two
zones, which the single-valued zone projection refuses outright rather than
mis-place (`badCrossZoneDisjunction`). A zone-SET projection is the shape chapter
sixty-four's `zoneAdmit` took for a relation's domain.

DOC AURLOCK's origin disjunction ("from your graveyard or from exile") is this
same gap wearing the landed cast-origin row: `Predicate.CastFrom` hangs the
single-zone qualifier and PATRICIAN GEIST is benched as its witness, so the
disjunction is this entry's work and not that row's.

### Recorded, not a gap

The SUBTYPE union — "Other Ninja and Rogue creatures you control get +1/+1"
(Silver-Fur Master, the one supported line) — composes today as
`Or [HasSubtype …, HasSubtype …]` under the derived "and" and waits on two
catalog words. It records the head generalizing past card types.

## Two object descriptions in one subject and one restriction

Two measured families want the same thing: one statement over **two object
descriptions at the same kind**, once as a subject and once inside a targeting
restriction. Neither is a cross-kind union site — the joined-kind question is
someone else's — and both are blocked on the coordination alone.

Authority: [The kind index joins; union marking is
spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md) — its
consequence is that a conjoined noun is a noun-coordination question, not a
kind-unification one.

### The coordinated subject — 10 cards plus one

"Target creature and all other creatures with the same name as that creature":
Echoing Decay, Echoing Truth, Echoing Ruin, Echoing Calm, Echoing Return, Bile
Blight, Declaration in Stone, Deputy of Detention, Banishment, Cylian Sunsinger.
It is the co-referential name family's **commonest frame**, and the name half is
already spelled — what these want is a subject that coordinates two nouns.
Rukarumel writes the same construction from a different family ("Slivers you
control and nontoken creatures you control"). Landing it buys the whole cycle at
once.

### The protection-shaped ability source — 12 sentences

"Can't be the target of nongreen spells or abilities from nongreen sources"
(Gaea's Revenge, Thrun, Spellbane Centaur, Mercenary Informer, Raiding Party,
Rebel Informer, Suq'Ata Firewalker, Artifact Ward and kin). The source head
predicate is landed and nothing else about the source is missing; what these want
is the negated colour **distributed over both conjuncts** and a targeting
restriction that reaches two phrases at once. [CR#113.7] is the relation.

## The two noun coordinations the union round did not land

Two measured coordinations sit in the workbench's round queue behind the union
work, and **neither is a kind-unification question** — the settled direction's
own consequence is that a conjoined recipient is a noun-coordination question.
Both survive the direction call unchanged, and both are blocked on their own
mechanism rather than on the join.

Authority for that scoping:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).

### 1. The player-plus-player distributive — 39 sentences over 36 cards

"You and target opponent each draw a card"; "each opponent and you each create a
Treasure token". The trailing "each" is present in **all 39** and bare unmarked
joint player-plus-player is **0**, so the marking *is* the construction rather
than decoration on the mixed group's shape. It needs a distribution over two
referents, where the mixed group has a joint reading and a player half fixed at
"you" [CR#109.5]. Two more fold the distributivity into the second conjunct
(Model of Unity, Juxtapose). **Do not fold this into the group referent; it is
the opposite reading.**

### 2. The heterogeneous double target — 12 sentences over 12 cards

Churning Eddy, Fumarole, Goblin Grenadiers, Grip of Desolation, Hull Breach,
Legerdemain, Necron Deathmark, Plague Spores, Spiteful Blow, Stomp and Howl,
Sudden Substitution, Cruel Entertainment. **Two mentions, not one filter**, and
the rule's own example says so: a spell that says "Destroy target artifact and
target land" can target the same artifact land twice **because it uses the word
"target" in multiple places** [CR#601.2c]. No union head and no coordinator
reaches it — the word "target" is written once in a union phrase and twice here.
Ballroom Brawlers is a thirteenth of the same shape with a floating "both" over
it. None benched.

### Measured zeros — recorded so this round does not re-buy them

Fresh "either … or" as a coordinator **0** (all 9 are anaphoric "either of
them"); fresh "both X and Y" as a two-referent coordinator **0**; "neither … nor"
12, of which 10 are the "neither day nor night" state idiom. None is pinnable —
the refusal is by vocabulary, there being no such word in the grammar to be
`impossible` about.

Out of scope and named so they are not folded in: the and/or type-word bucket,
"instant and sorcery cards" (a spelling of disjunction on one mention), the
statement-level coordinations, the two-header trigger join, and modal "choose one
or both".

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.3.

`negatable`'s full-row census over every `Predicate` constructor
(`Experimental.idr:1024`). One row of it — `negatable` at `Permanent` — is
recorded as "not measured by this round's recon" and is
not counted here (the docstring-audit ticket was deleted 2026-08-21).

## Reading-order inversions this ticket owns (added 2026-08-21)

- **`Effect.If` is oriented to the postposed sentence.** `If (e) (c : Condition
  (preIntro e)) otherwise` lets the condition read the effect's mentions —
  "Counter target spell if it's red" — and that is reading order for that shape.
  The bench uses it for leading-if sentences authored backwards (Tezzeret,
  `Experimental/Cards.idr` ~592, ~601, ~2140), and a leading condition's own
  mentions are unreadable because `condDelta` is `[]` for every `Condition`.
  Ruling: **both orientations are core**, because they introduce mentions in
  different places and under forward authoring the introduction site is the
  binding structure. "Counter target spell if it's red" introduces the target
  in the effect and the condition reads it back — today's shape, renamed
  `OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) -> …`. "If you
  control three artifacts, draw two instead" is a new leading
  `If : (c : Condition bs) -> (e : Effect (condIntro c)) -> Maybe (Effect bs)
  -> Effect bs`, with `condDelta` carrying what a condition introduces. The
  postposed form is NOT a macro over the leading one: the expansion would have
  to relocate the target noun into the condition and pronominalize the hole,
  and the only single-constructor alternative introduces the target above both
  clauses — the prenex lift the workbench exists to remove — or a binder with
  cataphora (full endophora), rejected: oracle text is strictly anaphoric, so
  a forward-only binder is true to the CNL and cataphora would be a miserable
  refactor in service of nothing the corpus writes. Macros name the
  English over the two; the bench's leading-if cards move to `If`.
- **`WhereLetter` / `WhereLetterStatic`** take the definition first where
  English postposes it ("…, where X is the number of …"); eight bench sites
  author backwards and `DefinedLetter` is the largest family. Same treatment:
  reading-order shape in core, or a macro that restores it — record which.

## Consumption boundary

`idris/src/Experimental.idr` (`Effect.If`, `Conditionally`, `condIntro`,
`condNegatable`, `badTrailingPostStateZone`; `Condition`, `AndCond`,
`Predicate.Or`, the five condition carriers; `Or` and the derived coordinator,
the zone projection, `zoneAdmit`, `Predicate.CastFrom`; the noun coordination,
the distributive reading, `Predicate`, the targeting restriction),
`idris/src/Experimental/Words.idr` (the doubled `if` spelling, `Kind`, the
subtype catalog), `idris/src/Experimental/Events.idr` for the one-shot's context
threading and for `Move` destinations, the pin modules
`idris/src/Experimental/Proofs*.idr` (`atLeastTwoCs`, `flatConjuncts`) and
`idris/src/Experimental/ProofsB.idr` (`badCrossZoneDisjunction`), evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Both orientations type in both containers; Balance of Power benches, and
  Vraska's [−9] and Iymrith read their gaps correctly.
- `badTrailingPostStateZone` still refuses what it refuses today; finding 362's
  no-single-constructor result is respected, not worked around.
- Gadrak's counted "unless" composes without opening negation wider than the
  comparison it fixes.
- The 360-line population is hand-sorted and the true count reported before the
  row is designed.
- The disjunct-may-be-a-conjunction question is answered explicitly, with a
  companion demand rather than a copied `FlatConjuncts` if the answer is yes.
- The condition-disjunction row reaches all five carriers, and an
  activation-guard carrier benches.
- "and/or" lands as its own truth condition, never as an environment of the
  derived word.
- The move coordination is measured across verbs before any move-specific row.
- The kind-crossing disjunction is answered inside one `Kind`; Sugar Coat lands
  whole or its one remaining blocker (the `Food` subtype word) is named.
- `badCrossZoneDisjunction` either retires against a zone-set projection or keeps
  its refusal with the reason restated; Patrician Geist stays benched.
- One coordination serves both the subject and the restriction position; the
  modifier's distribution over both conjuncts is answered in the construction,
  not special-cased per card.
- The Echoing cycle and Rukarumel bench whole, and at least one of the twelve
  targeting-restriction carriers with them.
- The coordination stays same-kind: it does not become a second spelling of a
  cross-kind join.
- The player-plus-player distributive and the heterogeneous double target
  elaborate against their own measured counts, with the two kept distinct (a
  joint reading and a distributive reading are not one row).
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
