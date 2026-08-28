---
needs: []
---
# Write the coordinations the grammar still refuses

**SPLIT 2026-08-27 into two sub-tickets — claim those, not this.** This file
is the umbrella and stays authoritative:
[1 condition disjunction](workbench-coordination-1-condition-disjunction.md),
[2 noun and phrase coordinations](workbench-coordination-2-noun-and-phrase.md).
Order: 1 first (its hand count sizes the family's biggest row); 2 after.

Split from `workbench-conditional-and-coordination` on 2026-08-22 after its
conditional half landed as round 1; the per-section scoping (NOT cross-kind
union questions) stands.

## The condition disjunction "if X or if Y"

**LANDED 2026-08-27** as `Condition.OrCond` — see sub-ticket 1's As-landed
section for the hand count (386 occurrences / 259 texts / 461 cards sorted into
four buckets; TRUE count 109 occurrences over 123 supported cards, against the
11 doubly-marked lower bound), the row, the four benches and the ledger
decisions. The section below is the pre-round record.

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

  **RULED (user, 2026-08-27): yes.** A disjunct may be a conjunction — the
  shape is `or [on the bf, and [in your gy, you have a giant]]` (the user's
  tree), i.e. disjunction over arms that are atoms or flat conjunctions, one
  level, nothing deeper. The printed scope marker is the reduplicated "if":
  each "if" opens a disjunct, so the AND binds inside its own disjunct.
  `FlatConjuncts` gets a companion predicate for the OR row (arms atomic or
  flat-conjunction), not a copy.
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

## Acceptance

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

## Ledger from round 1 review

**DISCHARGED 2026-08-27** by sub-ticket 1; every item decided there, two of the
consumerless-macro premises found wrong (`ifThen` has ten consumers;
`whereLetterStatic` does not exist in the tree). Do not re-buy.

- `otherwiseCtx` is orientation-asymmetric: under leading `If` the otherwise arm
  can read a target written inside the condition; under `OnlyIf` it cannot
  ([CR#601.2c] announces targets at casting whichever arm runs). Decide one
  answer for both.
- `condDelta` announces a target inside `CompareAmt` but not inside `Exists`;
  docstring at `Experimental.idr` ~2027 states the general principle, the table
  implements one row. Narrow the docstring or extend the rows.
- `Macros.ifThen`, `ifThenElse`, `onlyIf`, `onlyIfNot`, `whereLetterStatic` have
  no consumer; `whereLetterStatic` restores an order nothing writes.
- `nn : NotConditional se` still refuses a statement conditioned twice after
  `badDoubleConditional` retired; nothing attested is refused (8 `as long as …
  as long as` lines are independent statements), but the gate is asserted by
  nothing.

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **The non-`You` player half.** `kindJoin` takes a `Predicate bs Player` and
  the only two are `AnyPlayer` and `Opponent`, while "you" is a `Noun`, so the
  attested "attacks you or a planeswalker you control" has no player predicate
  (`docs/tickets/done/workbench-attackable-defender-join.md`). Its distributive
  twin is measured: 14 printed lines coordinate a non-"you" player half with a
  distributive object half ("deals N damage to target player and each creature
  that player controls"), which a declaration hard-wiring "you" refuses —
  `docs/tickets/done/workbench-union-gate-spelling-rehome.md`.
- **The reciprocal fight node** — [CR#701.14a]'s "two creatures fight each
  other", eleven printed lines (consolidated open gap 6) —
  `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`.
- **Static re-orientation, left by round 1.** desperateCastaways, bombur,
  brightspearZealot, deepwayNavigator, urborgScavengers, thunderstaff and
  martyrsOfKorlis are untouched; the last two are visibly fronted authorings of
  postposed print ("… as long as it's untapped") —
  `docs/tickets/done/workbench-conditional-and-coordination.md`.

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

Standard constraints apply.

- **Routed from workbench-split-determiner-union-read (close, 2026-08-25):** Concussive Bolt writes a 26th, differently-shaped split read — "by that player or by that planeswalker's controller", the preposition repeated over a noun disjunction rather than a bare `EitherOf` — outside that ticket's 25 and unbenched; it is a coordination-spelling question and lands here.

- **Carrier from workbench-randomness-residues (close, 2026-08-26):** Mana Clash — "You and target opponent each flip a coin" — is the player-plus-player coordination this ticket already owns, now with a distributive verb over the pair; add it to the witness set.

- **Cross-check from the workbench-event-zone-and-cast-provenance split (2026-08-26):** Doc Aurlock's zone DISJUNCTION on a cast's origin ("from your graveyard or from exile") is this ticket's CROSS-ZONE disjunction gap, already named above. If `workbench-event-zone-4-zone-catalog-and-reader-payload`'s zone-coordination pin (ordinary coordination at the zone sort) lands, Doc Aurlock may fall out for free — in which case tell this ticket so its cross-zone disjunction section is re-checked against the landed mechanism before this ticket's own round mints a second one.

- **Routed from workbench-chosen-counter-kind (close, 2026-08-26):** Inspirit — menu arms carrying DIFFERENT amounts ("two +1/+1 counters or four +1/+0 counters") is a coordination of counter clauses, not a kind menu; it lands here.

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** disjunction arms sharing a SUBJECT ("Whenever a player draws THEIR second card each turn …" — Trouble in Pairs); a count over a coordination's occurrences across arms ("if you've done all four this turn" — Avatar Aang, whose four bending verbs are also each one `verbFacts` row); both coordination reads, so they land here.

- **Routed from workbench-ability-kind-heads (close, 2026-08-27):** a discourse JOIN for DISAGREEING coordination arms — Repeated Reverberation's body wants "that spell or ability" over three arms whose after-discourses differ, but `sharedCtx` is whole-agreement-or-bare by the seat round's settled verdict; a join would be a deliberate re-decision of that verdict, made here or refused here with the rule. Supersedes the earlier "coordinated anaphor" line (its grammar half landed: `AbilityJoinW`, Shimmering Glasskite whole). Also: a join half carries no ZONE (`joinHalfPayload` drops it), so `halfReaches PermanentW` reads the spell half of a "spell or ability" mention as "that permanent" — tolerated overgeneration, newly reachable, same union-read machinery.

- **Routed from workbench-shuffle-into-library (close, 2026-08-27):** the coordinated MASS object — "shuffles their hand and graveyard into their library" (21 sentences). `Both (AllOf (InZone …)) (AllOf (InZone …))` may already write it — probe before building; bench either way. A coordination surface, so it lands here.

- **Routed from workbench-choice-b-sorts-and-reads (close, 2026-08-27):**
  Talion's three-way characteristic disjunction — `Or [Compare ManaValue…,
  Compare Power…, Compare Toughness…]` is refused by `parallelDisjuncts`'
  `seedsUniform` because `seedType (Compare c _ _) = comparedType c` differs
  per arm; the honest shape is a characteristic LIST on `Compare`, a
  coordination decision. 1 supported line.

- **Routed from workbench-choice-d-ascription-payloads (close, 2026-08-27):**
  the UNION SUBJECT — the Lace cycle's 6 lines are blocked by
  `parallelDisjuncts [spell, Permanent]` having no implementation (a spell-or-
  permanent subject); Mycosynth Lattice and Painter's Servant want the
  everywhere subject of the same family. Disjunct machinery, so it lands here.
