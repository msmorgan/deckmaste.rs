---
needs: []
---
# Settle what a later phrase may read back, and finish the mention surfaces

One family around a single question asked at many sites: which phrases a clause
introduces outward, and which of the phrases in scope a later mention — a bare
pronoun, a demonstrative, a definite description, a partitive — may land on.
The intro machinery (`effIntro`, `preIntro`, `condIntro`, `annIntro`,
`nomIntro`, `selfSubjIntro`), the mention vocabulary (`It`, `That`,
`TheVerbed`, `ThoseVerbed`, `stampedBy`) and the creation clause's replacement
side all move together, so they are one claimable unit.

## Intro threading and what a later phrase may read back

Four measured gaps ask one question in four places: which phrases a clause
introduces outward, and which of the phrases in scope a later mention lands on.
Two are about the conditional's intro direction (`effIntro`, `preIntro`,
`condIntro`); two are about what a pronoun or a definite description may point
at. They share the intro machinery.

### The conditioned clause's own target — 1 card, population unmeasured

Savage Swipe writes "Target creature you control gets +2/+2 until end of turn if
its power is 2. Then it fights target creature you don't control." The first
sentence is benched; the second is refused because `If` makes its clause a
**hole outward** (`effIntro`) and "it" finds nothing. The refusal is right for
`badConditionAntecedent`'s case, where the *condition* introduces the phrase and
may have been false — but a **target announced by the clause is chosen at cast
and survives the conditional either way**. One card found; **measure the
population before designing**.

### The condition-first conditional — 7 cards, design already done

All that is left of the explicit tie sentence. The condition is writable, the
draw consequent landed (Celestial Convergence), and the chooser slot landed.
What blocks the seven "you choose one of them" cards (Drop of Honey, Porphyry
Nodes, Purging Scythe, Desecrator Hag, Loxodon Peacekeeper, Juxtapose, Tariff) is
that `Effect.If` types its condition at `preIntro e` — the condition reads the
effect's phrase, never the reverse — so a condition cannot announce the group its
own consequent reads back. The two carriers that *do* thread `condIntro` forward
are the static `Conditionally` and a trigger's intervening clause, and neither is
this sentence.

**Do not re-derive the design.** The group the tie condition announces is the
counted domain's own denotation, minted exactly when that domain `uniquifies`
(picks an extreme), measured **7/7**: every line that reads "them" back off a
count comparison is a tie sentence, and no ordinary threshold does. The
`condIntro` clause was written, compiled, and backed out for want of a consumer;
it goes back in on the round that mints the carrier. The two remaining
consequents are Psychic Battle's (the targets are unchanged) and Timesifter's
(the tied players repeat the process), one card each.

### Cross-ability memory — a both-layer gap

"the exiled creature card's power" — Phyrexian Ingester's static line reading the
card its enters-trigger imprint exiled; Drach'Nyen wants the same. The pieces for
a **same-ability** version exist (`ExiledWith` links a card to its source,
`TheVerbed Exile` reads a verbed mention back definitely), and both are
discourse-local by construction, the discourse being one ability wide. Core has
nothing either: no imprint primitive, and `Count::Noted(Ident)` is a number read
back from a slot ([CR#607.2] linked values), not an object reference. Last thing
between Phyrexian Ingester and the letter Y.

### Subject preference for the bare pronoun

English resolves a bare "it" by preferring the subject, where this grammar
refuses two object mentions outright. That was an over-generation and is now a
**blocker**: Bioplasm's "exile the top card of your library. If it's a creature
card, this creature gets +X/+Y, where X is the exiled creature card's power and Y
is its toughness" has the trigger's attacking subject and the exiled card both in
scope. Wants a **measured** preference rule; the definite description it would
otherwise use exists (`TheVerbed Exile`), which is why that card's two halves sit
in two entries — this one and cross-ability memory, one ambiguity inside an
ability and one reference across two.

## Verb provenance for the bare anaphor

The grammar cannot see WHICH verb stamped the mention a bare `It` reads. That is
one named overgeneration and three blocked cells, all the same missing query.
`TheVerbed` already carries `stampedBy`; `It` carries nothing.

- **The overgeneration** (finding 1010), 4 lines: the regeneration rider is
  spellable after a DAMAGE clause although the corpus spans those instead —
  "Engulfing Flames deals 1 damage to target creature. It can't be regenerated
  this turn.", Rage of Purphoros, and Carbonize and Disintegrate with their own
  conditional.
- **The participle read**, 10 lines: "Creatures destroyed this way can't be
  regenerated", blocked because the verb stamp lives in the post-context while
  the rider's subject is typed in the pre. A provenance query on the bare
  anaphor unblocks this cell too.
- **The demonstrative subject**, 4 lines (Nekrataal, Scorching Lava and kin):
  blocked because `That` cannot read a destroyed target at all — probed bare and
  refused the same way independently of the regeneration row.
- PLAGUE SPORES is the CR's own worked example for the family but is NOT this
  round's: it needs two separate targets under one plural anaphor, which the
  double-target work owns (see
  [workbench-conditional-and-coordination](workbench-conditional-and-coordination.md)).
- **Measure `It`'s other readers before widening** — it is the most-read noun in
  the grammar.
- Context, so the round is scoped right: the regeneration family landed whole
  (435 sentences), `ObjectAct` taking a sixth row read by two carriers —
  `StaticEffect.ObjectCant` for the spanned 18 and `Effect.CantBe` for the 138.
  The prohibition alone is 156 and reproduces exactly; the anaphoric subject is
  127 and the no-span count is 138, which is also the ATTACHED count — the two
  axes coincide, and the corpus writes neither a spanned rider nor a spanless
  standing clause even once. [CR#701.19c] is why the MECHANISM is not a deed
  denial and is NOT a reason the word cannot sit in a word catalog (finding
  1008); do not re-derive the opposite.
- One further payer, worth nothing on its own but free here: the marked-read row
  admits a marked read after a single non-repeating chooser because nothing
  represents a chooser's repeatability (finding 1030). Every one of its 12
  carriers has a chooser that can fire more than once and no card writes the
  marked read against a once-only chooser. Closing it wants the same kind of
  provenance.

## The statement that reads its own subject

The fourth container, at two positions with two different costs (finding 624),
plus the one compound the static coordination did not free. Both are the same
shape: a term inside a statement reading the statement's own subject.

- **The pump's AMOUNT**, 9 lines, and this half is cheap. "Enchanted creature
  gets +2/+2 for each Aura attached to it" (Auramancer's Guise; Golem-Skin
  Gauntlets, Luxior, Mantle of the Ancients, Strong Back, Thran Power Suit,
  Alpha Status, Stoneforge Masterwork). `Gets`' slots are typed at `nomIntro n`
  and would move to `selfSubjIntro n` exactly as `staticIntro` did — the same
  one-line change at a second site. Cheapest witness: Auramancer's Guise, whose
  other conjunct is a plain vigilance grant.
- **The "as long as" CONDITION**, 16 lines, and this half is not cheap.
  "Enchanted creature gets +2/+2 as long as it's a Human" (Bonds of Faith;
  Armament of Nyx, Burden of Proof, Clutch of Undeath, Favorable Destiny, Gift
  of Fangs …). `Conditionally : (c : Condition bs) -> (se : StaticEffect
  (condIntro c))` threads the condition INTO the statement, so an adverbial
  reading its own statement's subject is an argument-order question. Cheapest
  witness: Bonds of Faith.
- **Titania's Song and the self-reading definition** — the last of the four
  compounds and the only one the coordination did not free. "Each noncreature
  artifact loses all abilities and becomes an artifact creature with power and
  toughness each equal to its mana value": the first two parts compose today,
  and the third is `DefinesPt` reading the SUBJECT's own mana value from inside
  a coordinated part. It is also finding 604's one reverse crossing — the sole
  static ability line in the corpus writing the inchoative "becomes" — so
  landing it benches that crossing too. Small: this card and Karn, Silver
  Golem's animation are most of the family.

## The mention denoting the union over loop iterations

A loop body creates one thing per iteration; the corpus then reads them back
PLURAL. Nothing in the grammar denotes that union, and two separate families
are stuck on it. Build the mention, scope it against both readers, then the
loop's until-condition cell is a one-constructor follow-up. The demonstrative's
zone tension rides here because it is the same "what may a mention reach"
question.

### The mention itself — three lines

- "Those tokens gain haste" (Hate Mirage)
- "Exile those tokens at the beginning of the next end step" (Twinflame)
- "Those tokens have enchant creature and …" (Smoke Spirits' Aid)

Each reads the tokens the BODY created and reads them plural, where the body
creates one per iteration. Passing `effIntro body` outward is not the answer:
it would offer a singular mention no card writes, and still not spell these.
All three probed green up to their last sentence — the mention is the only
thing between them and the bench.

### The second and larger reader — the loop's until-condition, 9 cards

Eureka, Hypergenesis ("until no one puts a card onto the battlefield"), Plague
of Vermin, Mana Clash, Struggle for Sanity, Tainted Pact, Thieves' Auction,
Timesifter, Helm of Obedience. NOT an ordinary board condition, which is why
`Effect.Repeat` landed with no slot for it: all nine read the ITERATIONS' own
doing ("until no one pays life", "until all cards exiled this way have been
chosen"). Twelve cards in all across the two readers, one missing mention.

Blockers inside the nine, not this round's: Tainted Pact and Helm of Obedience
additionally want a disjunctive termination ("whichever comes first"), which is
the routed OR cell; Timesifter additionally waits on the tie condition queued
with extremal selection.

### The permanent word after a zone change — a recorded TENSION, not a gap

`wordReaches PermanentW` demands `onFieldZone`, correctly: [CR#110.1] makes a
permanent a card or token on the battlefield and it "stops being a permanent as
it's moved to another zone". The corpus writes the word anyway — Soul of
Emancipation's "destroy up to three other target nonland permanents. For each
of those permanents, its controller creates a 3/3 white Angel creature token
with flying" reads "those permanents" of things that are in graveyards by then.
Spelled with "them" the whole trigger elaborates (probed green), so the card is
one demonstrative away from the bench. This round owns the decision on whether
the word's zone demand tolerates the loose reading. Do NOT patch it from the
binder's side.

Counts here are a prior session's measurements; re-measure before building.

## The token anaphor's definition channel and the creation clause

The creation clause's remaining cells, all on the replacement side: an anaphor
that points at a definition rather than at an object, the causer and subject
shapes the clause has no rows for, and the once-per-turn word that caps it. The
definition channel is the one that lands cards; the others are single lines that
would each be too small alone.

### The definition channel — what `InsteadOf` withholds, 16 lines

16 supported lines write an anaphoric create specification inside a SELF-
replacement: "Create six 1/1 white Kor Soldier creature tokens. If this spell
was kicked, create twelve of those tokens instead" (Conqueror's Pledge), and the
same shape on Saproling Migration, Increasing Devotion, Gather the Townsfolk,
Prismari Pianist, Anax, Skeletal Swarming, Throne of Empires, Starnheim
Unleashed, Rite of Replication, Adipose Offspring, From Under the Floorboards,
Safana, Andúril, Runo Stromkirk and The Final Days.

The composition is fine — `TokenSpec.TokenAsThose` in an ordinary sequence and
under arrival riders both probed green. What refuses is `InsteadOf`, whose
replacement arm is typed at `annIntro replaced` and so cannot see the batch the
replaced clause made.

**That refusal is correct as far as it goes and must not simply be relaxed.**
[CR#614.6] says the replaced event never happened, so the tokens do not exist
and `annIntro` is right to withhold the OBJECT mention
(`badInsteadReadsReplacedSequenceOutcome` is the same discipline one step over).
What the anaphor actually points at is the CHARACTERISTIC DEFINITION the
replaced clause wrote — [CR#111.3]'s "this becomes the token's 'text'" — which
is written whether or not the creation happens. So the ask is **a channel for
DEFINITIONS beside the announcement channel for phrases**, where the binding
vocabulary today has one mention doing both jobs. Decide whether `Payload`'s
object arm grows a definition flag or the announcement channel gains a second
row.

The five NON-instead ordinary lines (Brood Birthing, Delina, Swarming Goblins
×2, Wurmquake) are blocked on dice, quoted abilities and mana instead, so this
cell lands no card by itself — **Prismari Pianist** is the cheapest one behind
it, single-blocked on exactly this.

### The described-ability causer — 3 lines, plus the singular anaphor

`Causer` is one row (`AnEffect`, [CR#614.16]'s bare word, 4 lines) and three
supported lines write a described ability in the same position: "if a spell or
ability would cause its controller to gain life" (Rain of Gore), "if a cycling
ability of another nonland card would cause you to draw a card" (Unpredictable
Cyclone), "if a modular triggered ability would put one or more +1/+1 counters
on a creature you control" (Zabaz, the Glimmerwasp). One line apiece, each
carrying its own restriction, and two of the three write the periphrastic verb
"cause … to" rather than the event's own — a second unbuilt thing, and the
reason these are not `Causer` rows.

Zabaz is ALSO the sole crossing of finding 582's voice covariance (an active
event clause with a passive body), so the line that lands it is the line that
turns `Intercepts`' derived body voice into a slot; the two facts belong to one
round.

The SINGULAR anaphor rides here too: "instead create that token and a Treasure
token" (Mr. House, President and CEO) is the one line writing a singular create
specification, and it is attested, so `TokenAsThose`'s plural demand is **not
pinned** (finding 596). It wants a coordination of two specifications and
[CR#111.10]'s predefined name besides.

### The creation clause's remaining subject shapes — two single lines

The effect subject is landed: it is `Causer` and [CR#614.16]'s word is a row.
The worry recorded with it did not survive measurement — a replacement body over
this event writes no subject phrase of its own in any of its three shapes
(finding 595), so the pronoun is the container's derivation exactly as the
passive voice is. What remains:

- **the ability subject** — Zabaz, the Glimmerwasp, the same line and the same
  voice-covariance crossing as above;
- **the relative-clause determiner** — Crafty Cutpurse's "each token that would
  be created under an opponent's control this turn is created under your control
  instead": one line, a determiner `TokenPhrase` has no constructor for, and a
  body that re-states the control phrase.

### The "first time … each turn" replacement word — 3 lines

`ReplUse` is [CR#614.3]'s two endings, "used up" and "duration expired", and
three supported lines write a third thing: "The first time you would create one
or more tokens during each of your turns" (Esix, Fractal Bloom), "the first time
you would create one or more tokens each turn" (Mirrormind Crown, Moonlit
Meditation). A once-per-turn CAP is neither ending, and it is the replacement-
side twin of the landed once-each-turn trigger rider (`UsageLimit`) — which is
the argument for reading that vocabulary here rather than minting a third
`ReplUse` value. All three bodies also want the anaphoric token spec above, so
the word alone lands no card, which is why it rides with the definition channel.

## Partitives over a verbed mention and the plural-group library slice

The verbed-mention machinery landed its participle halves; what is left are two
partitive/possessor SURFACES it does not spell, plus one standing prohibition
recorded so no successor re-proposes it. Same declarations.

### The mill's AMONG-restriction — 20+ lines across mill, reveal and exile

The participle halves landed with the tag ("the milled card" 19 lines, "milled
this way" 43, both through `TheVerbed`/`ThoseVerbed`). "From among the milled
cards" / "from among them" is a partitive over a verbed mention and a surface of
its own: it is not the tag's, and no `VerbName` row will produce it.

Its neighbour is `SomeOf`, the partitive this grammar has. THE DESIGN QUESTION:
whether "from among" is that determiner under a different spelling, or a second
construction.

### The plural-GROUP library slice — 5 supported lines

The surviving half of a retired pin, held today by `badSliceOfGroupPossessor`.
Five lines write a slice over a GROUP possessor and every one pluralises the ZONE
word: "the top card of their libraries" (Field of Dreams, Lantern of Insight),
"each of those opponents' libraries" (Breeches), "one of their opponents'
libraries" (Shared Fate). `LibrarySlice` spells one library and pluralises only
the card word, so the phrase is a second surface, not a cell — and two of the
five put a partitive over the possessor besides.

### RETURN is not a `VerbName` row — do not re-propose it

Recorded because the recon proposed it twice. Measured: "return" is absent from
[CR#701]'s seventy keyword actions and the CR defines it nowhere; "would be
returned" is 0 supported lines, "was returned to" 0, "the returned card" 0 —
against mill's 2, 12 and 19. The five "is returned to [a] hand" triggers are
zone-change triggers ([CR#603.6]) keyed on the DESTINATION, and "returned this
way" (10) is the verb-general deictic idiom, attaching to "removed", "prevented",
"moved", "died" and "enchanted" too.

What IS open here is smaller and different: a return's hand-destination `Move`
writes no stamp, so nothing reads a returned card back by participle. If a
witness ever wants "the returned card", it wants a STAMP WITHOUT A TAG — decide
that only against such a witness.

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **`It` has no antecedent after a discard cost.** Twinshot Sniper's `Channel —
  {1}{R}, Discard this card: It deals 2 damage to any target.` cannot be written:
  `Do (discards You This)` introduces no object binding, so `It`'s
  `countOnes Object bs = 1` fails and a `Channel` witness waits on it —
  `docs/tickets/done/workbench-ability-word-primitive.md`.
- **The plural read-back mention.** "Each player scries N" is unspellable: the
  anaphor needs a single `Player` mention and `Each` introduces a plural one.
  Same gap `MillB` has —
  `docs/tickets/done/workbench-closure-flip-risks.md`.
- **Replacement-event anaphora.** `eventIntro` mints no subject binding for the
  event's own noun, so Clergy's printed "it" has no antecedent (consolidated
  open gap 7) —
  `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`.
- **`Effect.Search` takes no `Quantity` and mints no mention**, so neither a
  counted search nor a group constraint on its object has anywhere to attach;
  13 of the 24 "with different names" lines write the constraint there —
  `docs/tickets/done/workbench-name-match-family.md`. Boreas Charger's second
  clause ("search your library for a number of Plains cards equal to the
  difference") is a payoff too, ledgered by
  `workbench-choice-frame-licensed-reads`.
- **The card's mana cost is unthreaded.** `Spell` effects are typed at `[]`, so
  Prosperity's `{X}` and its text X are one variable only in prose; the fix is
  `Card.text : AbilitySeq (costLetters cost)` —
  `docs/tickets/done/workbench-letters-introduce-then-define.md`.
- **Tayam's "from among" distributive removal** over a described group belongs
  with the mill's AMONG-restriction above —
  `docs/tickets/done/workbench-counter-family-residues.md`.

## Consumption boundary

`idris/src/Experimental.idr` (`effIntro`, `preIntro`, `condIntro`, `Effect.If`,
`Conditionally`, the mention scope and `TheVerbed`/`ExiledWith`; `Gets`,
`DefinesPt`, `nomIntro`, `selfSubjIntro`; the iteration-union mention row and
`Effect.Repeat`'s until-condition cell; `InsteadOf`, `annIntro`, `TokenSpec`,
`TokenPhrase`, `Causer`, `ReplUse`, `UsageLimit`, `Intercepts`; `ThoseVerbed`,
`SomeOf`, `LibrarySlice`, `Move`, `VerbName`),
`idris/src/Experimental/Macros.idr` (`effIntro`/`preIntro`/`condIntro` and
`InsteadOf`/`annIntro`'s spelling side), `idris/src/Experimental/Words.idr`
(`It`, `That`, `TheVerbed`, `stampedBy` and the mention's readers;
`wordReaches`, `PermanentW`, `onFieldZone`; `Payload` and the binding
vocabulary's mention channels; the partitive and possessor spelling),
`idris/src/Experimental/Events.idr` (where the verb stamp is set, pre- versus
post-context), the pin modules `idris/src/Experimental/Proofs*.idr` — including
`idris/src/Experimental/ProofsC.idr` and `idris/src/Experimental/ProofsD.idr`,
`badInsteadReadsReplacedSequenceOutcome` and `badSliceOfGroupPossessor` —
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- `badConditionAntecedent`'s case still refuses; only the announced-target case
  opens, and only against a measured population.
- The tie carrier mints the announced group off `uniquifies`, and the restored
  `condIntro` clause has a consumer; the seven cards elaborate.
- The pronoun preference is stated as a measured rule, not as a widening that
  admits both readings.
- The bare anaphor answers a provenance query; the 4 overgeneration lines are
  refused and pinned, and the 10 participle-read lines write.
- `It`'s existing readers are inventoried before the widening and none loses a
  reading.
- Auramancer's Guise and Bonds of Faith bench; the 9 amount lines and 16
  condition lines write.
- Titania's Song benches, and finding 604's reverse crossing is benched with it.
- The argument-order answer for `Conditionally` does not loosen what the
  condition may see beyond the statement's own subject.
- The mention denotes the union over iterations, and is scoped against BOTH
  readers before it is minted — the three token lines and the nine termination
  cards.
- The three token cards bench whole; the until-condition cell either lands on
  the new mention or records why it does not.
- The permanent word's zone demand is decided explicitly, with the decision
  carried by a gate or a witness either way.
- The definition channel carries the characteristic definition without relaxing
  `annIntro`'s withholding of the object mention;
  `badInsteadReadsReplacedSequenceOutcome` still refuses.
- Prismari Pianist benches whole; the five non-instead lines stay blocked on
  their own named reasons.
- Zabaz's ability subject lands together with the voice slot — `Intercepts`'
  body voice becomes a slot rather than a derivation.
- The once-per-turn cap reuses `UsageLimit`'s vocabulary, or records why it
  cannot; no third `ReplUse` ending is minted silently.
- `TokenAsThose`'s plural demand is re-pinned or dropped deliberately, with Mr.
  House recorded as the attested singular.
- "From among" is decided explicitly as either a `SomeOf` spelling or its own
  construction, with the mill/reveal/exile carriers counted before the choice.
- The group-possessor slice lands as a surface; `badSliceOfGroupPossessor` is
  retired only against the lines it was holding, and the zone-word plural is
  spelled.
- No `VerbName` row is minted for "return", and its four zeros stay measured.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-payment-events-and-replacement-disjunction (close, 2026-08-26):** Heart of Bogardan's BODY is blocked card-level on demonstrative uniqueness — the header announces the non-payer, so the split read ("that player or that planeswalker's controller") finds a second singular player mention and `countWord PlayerW bs = 1` fails. A mention/uniqueness question, so it lands here.

- **Routed from workbench-distinct-kind-count (close, 2026-08-26):** `Gains` introduces no readable object binding when the trigger already bound a subject — `It` finds two singular objects (Perrie, the Pulverizer). A mention/binding gap, so it lands here. Related: the counted-search line above also wants a DESTINATION slot on `Effect.Search` (Celebrate the Harvest searches "for … put onto the battlefield").

- **Routed from workbench-verb-label-residues (close, 2026-08-26):** `afterShuffle` is owner-blind — it drops every unstamped library mention whichever library was shuffled, but [CR#701.24b] scopes to the shuffled one; the binding payload records a zone, not an owner. A payload/mention question, so it lands here.

- **Routed from workbench-amount-ceiling-read (close, 2026-08-26):** "Discard up to two cards, then draw that many" (12 carriers) — a set ceiling plus a magnitude the discard clause must ANNOUNCE for "that many" to read; a mention/announcement gap, so it lands here.
