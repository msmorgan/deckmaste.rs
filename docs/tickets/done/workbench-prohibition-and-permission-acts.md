---
needs: []
---
# Pay what the prohibition, permission and outcome-gate acts still owe

**CLOSED 2026-08-28: ALL THREE SUB-ROUNDS DONE.** Survivors live in workbench-prohibition-tail; the mana restriction is decided into mana-family; the Crew/Saddle deed rows ride keyword-parameters. Originally split as: This
file is the umbrella and stays authoritative:
[1 unified carrier](workbench-prohibition-1-unified-carrier.md),
[2 qualified acts](workbench-prohibition-2-qualified-acts.md),
[3 permissions and gates](workbench-prohibition-3-permissions-and-gates.md).
Order: 1 first (the carrier both rulings define); 2 and 3 after it, in
parallel if their regions stay disjoint.

One region: the acts a subject may not do, the acts a subject may do, and the
gates that end the game. They share carriers (`StaticEffect.PlayerCant`,
`StaticEffect.ObjectCant`, `Compulsion`), they share the spanless-clause and
coordination questions, and the corpus writes the prohibition and the permission
about the same sentences one card apart. Splitting them re-measures the same
neighbourhood five times.

## Ability-class prohibition residues

`Kind.Ability` landed and `ObjectCant` and `CostsToCast` widened over it. The
whole family is 98 lines; these are the counted residues, plus one catalog
name-straddle recorded so it is not re-read as a gap. Note before scoping: an
earlier count of this family (74 lines, a 50/15 split) was wrong in every figure
— the raw grep included 12 detain REMINDER lines and the real split is 43/19
(finding 970), and the 28 cost-REDUCTION lines were never counted at all.

- **The triggered-ability subject**, 2 lines: The Master, Multiplied's "can't
  cause" and Nowhere to Run's "don't trigger". Both want a verb neither carrier
  has; the referent itself is now built.
- **The floor rider**, 6 lines: "This effect can't reduce the mana in that cost
  to less than one mana", the second sentence on Training Grounds, Heartstone,
  Biomancer's Familiar, Forensic Gadgeteer, Convergence of Dominion, Power
  Artifact.
- **The "that target …" restrictor on an ability class**, 7 lines: Bladegraft
  Aspirant, Cloud, Planet's Champion, Dwarven Mauler, Fervent Champion,
  Helitrooper, Kopala, Strong Back.
- **Five KEYWORD CATALOG rows** — Boast, Cycling, Ninjutsu, Exhaust, Power-up, 6
  lines between them. Data rather than grammar, since the class word takes the
  `Keyword` parameter.
- **The `Phyrexian` name-straddle**, 1 bench line and no grammar cost: the
  creature type cannot enter the subtype catalog because the word is already the
  Phyrexian MANA SYMBOL constructor [CR#107.4f], so PHYREXIAN REVOKER's type line
  is benched as "Horror" alone (finding 978). Any fix is a naming decision about
  one of the two catalogs, not a construction — recorded so the elision is not
  read as a gap.

### Do not repeat the plural-possessor claim

`possessorOk` has admitted `PlayerGroup YourOpponents` since it was widened for
the relative clause: "activated abilities of creatures your opponents control"
was written expecting a refusal and got a GREEN build (finding 980), and LINVALA
KEEPER OF SILENCE and EIDOLON OF OBSTRUCTION landed whole on the correction.
Any surviving note claiming "creatures your opponents control is 386 lines and
stays unwritable" is stale — probe before believing it.

### What the class still owes, by card

KOPALA WARDEN OF WAVES (the "that target a Merfolk you control" restrictor);
TITHE TAKER (Afterlife, plus a during-your-turn condition over a coordination of
two cost statements at two sorts); SHARKEY TYRANT OF THE SHIRE, DRANA AND
LINVALA and KARN THE GREAT CREATOR (each blocked on an ability-grant or loyalty
line, not on this one); CARTH THE LION ("planeswalkers' loyalty abilities you
activate cost an additional [+1] to activate" — an ADDITIONAL cost with a
loyalty-symbol payload, which `CostsToCast` excludes by measurement). VOIDSTONE
GARGOYLE's fourth line is an ability-class subject and waits here with Phyrexian
Revoker. SANCTUM PRELATE has this row now and wants only the chosen-NUMBER read,
which is the chosen-value sort vocabulary's round and not this one.

## The prohibition acts' qualified complements

The prohibition family is 2,683 supported sentences over 2,528 cards (finding
787's table). Attacking and blocking (1,818), damage prevention (31), the untap
cap (9) and the game outcomes (10, `OutcomeGate`) were already written;
`StaticEffect.PlayerCant` over `PlayerAct` added 41 more (gain life 21, play
lands 7, cast spells 7, search libraries 4, draw cards 2 — all five BARE, one
row, no new `StaticKind`, no span-table cell), and `StaticEffect.ObjectCant`
over `ObjectAct` landed the object-subject half. What is left is the acts that
carry a description, plus the spanless cell the neighbourhood shares.

- **The QUALIFIED cast lines**, 74 sentences, and the reason
  `PlayerAct.CastsSpells` spells only the bare form. Five qualifier families,
  each its own subsystem: a spell TYPE ("noncreature spells", "creature spells",
  "spells of the chosen type"), a NAME ("with the chosen name", "with the same
  name as the exiled card"), a ZONE ("from graveyards", "from anywhere other
  than their hands"), a TIMING ("during combat", "during your turn") and a COUNT
  CAP ("more than one spell each turn", 9 lines, which is `CantUntapMoreThan`'s
  shape at a second verb — the cheapest of the five and the only one with a row
  already shaped like it). The object half is paid, which settles what these 74
  are NOT: a player prohibition with a qualified complement is a different row
  from an object prohibition with a described subject, and the corpus writes
  both about the same sentences ("Your opponents can't cast spells with the
  chosen name" against "Spells with the chosen name can't be cast", one card
  apart). The NAME qualifier's DESCRIPTION is landed (`Named ChosenName`), so
  its five sentences — Council of the Absolute, Gideon's Intervention,
  Alhammarret, Failure // Comply, Academic Probation — want only what all 74
  want: an act that carries an object description instead of a bare `PlayerAct`
  value.
- **The TARGETING prohibition**, 39 real sentences — and the count IS the
  correction: a naive sweep returns 114, of which 75 are the reminder text
  printed under hexproof and shroud, which is not a card's own line. It is the
  object-subject sibling ("this creature can't be the target of spells or
  abilities your opponents control") and it carries a complement the player
  prohibitions never do — WHOSE spells and abilities.
- **The remaining acts, each its own question**: the MANA restriction (48,
  "can't be spent to cast spells with mana value 3 or greater"), the ACTIVATION
  prohibition (35, "can't be activated" / "can't activate abilities" — re-probe
  first, since `ObjectCant` and `CostsToCast` were later widened over
  `Kind.Ability`), the amount FLOOR (18, "X can't be 0", "this effect can't
  reduce the mana in that cost to less than one mana" — the odd one, since it
  prohibits nothing an agent does and belongs with the amount vocabulary), and
  the long tail: sacrifice 6, destruction 5, drawing an object's cards 5,
  copying 4, exile 1.
- **The SPANLESS clause**, about nine lines, and NOT the act catalog's question:
  `absentOk DeedRestriction` is False and the neighbouring `Prevention` refuses
  the identical cell — Banefire writes both halves in one sentence ("this spell
  can't be countered and the damage can't be prevented"). Re-measure `absentOk`
  across the whole prohibition neighbourhood at once. It costs VEXING SHUSHER's
  second line ("{R/G}: Target spell can't be countered"; its first line
  elaborates today), the four mana-permission riders (Boseiju, Cavern of Souls,
  Delighted Halfling, Savage Summoning, which want the spend-only permission as
  well) and the "next spell you cast this turn" clauses (Insist, Overmaster,
  Mistrise Village), which additionally want a NEXT-object noun.
- **GADDOCK TEEG's second line**, 1 line: "Noncreature spells with {X} in their
  mana costs can't be cast" wants a predicate over a mana cost's SYMBOLS. Its
  first line elaborates today. AETHER STORM's second line wants an any-player
  activation and the regeneration rider. Both were once named as free payers for
  this row and both were wrong (finding 881).

### Measured zeros and recorded non-gaps

- "Can't lose life" is 2 lines ([CR#119.8]'s sentence) and both are CONJUNCTS of
  a larger coordination rather than sentences of their own (Courageous Resolve,
  Everybody Lives!); "can't pay life" is 3, all three qualified by a purpose
  clause ("to cast spells or to activate abilities"). Either becomes a catalog
  value the day a witness writes it alone — not before.
- "Must be countered" and "can't be countered unless [payment]" are ZERO lines
  apiece, so two of `Deontic`'s three polarities have nothing to spell here and
  no verb has a second voice.
- The conditional carriers are 4 lines (Banefire, Demonfire, Dragonlord's
  Prerogative, Exquisite Firecraft): "If X is 5 or more, this spell can't be
  countered" is this row under `Conditionally`, already composable, with each of
  the four trailing a second conjunct or an ability word of its own. Recorded,
  not a gap.

## The creature permission ("this creature can …")

`Compulsion` has `Forbid`, `Require` and `GatedBy` and no PERMISSION row, so
"this creature can attack" is unwritable before any counterfactual is reached.
That single gap blocks 118 measured sentences in six cells. Build the
permission first; the "as though" is a slot on it afterwards.

### The blocker

One blocker for all six cells, and it is NOT the counterfactual. The round is
the permissive twin of the deed restriction; the counterfactual slot follows it.

### The six cells — 118 sentences

- **Attack despite defender, 52** — the largest, and it wants nothing else. 45
  of the 52 are a bare "This creature can attack as though it didn't have
  defender" under a condition or an activated cost.
- **Assign combat damage as though unblocked, 19** — Thorn Elemental's family,
  always "You may have this creature …": a permission whose subject is the
  PLAYER and whose actor is the creature, so it may want the other carrier.
- **Crew or saddle at a greater power, 18** — all one sentence, "as though its
  power were 2 greater", 14 of them inside a quoted token ability.
- **Blocking despite evasion, 17** — landwalk 11, shadow 4, "those abilities" 2,
  reach 1, and "as though they were untapped" 1. A NEGATED-KEYWORD payload: a
  second counterfactual vocabulary, and the one place this family wants a
  general one.
- **Attack despite summoning sickness, 7.**
- **Be targeted despite hexproof or shroud, 5.**

### Surface context this round inherits

[CR#609.4] bipartitions the whole "as though" surface in its own words — "a
player MAY do something 'as though' … or A CREATURE CAN do something 'as
though' …". The surface measures 264 supported sentences over 261 cards in
eight cells; a naive line grep returns 314, and the 50-card difference is
phasing REMINDER text ("treated as though it doesn't exist", [CR#702.26b]),
which no card writes as its own line. The player-may side already landed as
`PlayAsThough = HadFlash` on `StaticEffect.MayPlay` (88 sentences).

Counts here are a prior session's measurements; re-measure before building.

## The play permission's remaining subjects, riders and complements

The cast permission's spell-sorted complement and the top-of-library visibility
riders both landed; what is left of the family is seven narrow cells, each with
its own named blocker and most with a whole card behind it.

### What already landed, for orientation

- The complement rule derives from the table that was already there:
  `complementLocates z = playableFrom z` — a complement's zone locates its
  object exactly when that zone could have been the source, because the two
  zones `playableFrom` refuses are refused for being places nothing can be
  played FROM. That opened the LAND arm as well as the spell arm: 233
  spell-worded and 17 land-worded sentences over a source phrase, 250 in all.
- The visibility riders landed as one row, not an axis: `ExposeVerb = LookAt |
  Reveal` already carried [CR#701.20a]/[CR#701.20e]'s "one operation, two
  audiences"; [CR#401.5] names the two surfaces in one sentence and gives them
  one timing regime, which is [CR#604.6]'s move at the play permission the
  riders gate. The "may" and the possessive both fell out as construction-owned
  spelling, so the complement is a closed two-row tag `VisibleThing =
  TopOfLibrary | WholeHand` rather than the exposure clause's general `Exposed`.
- All four permission pins still refuse. They must still refuse afterwards.

### The cells

- **The group possessor** — 3 reveal cards, 5 lines. Field of Dreams, Lantern
  of Insight and Wizened Snitches write "the top card of their libraries"; the
  plural zone word `badSliceOfGroupPossessor` refuses on the positioned slice's
  own possessor question. A LibrarySlice question and not a visibility one — the
  riders take those cards' subjects happily. Field of Dreams and Revelation
  additionally want the World supertype (2 cards).
- **`Casts` has no source-zone slot** — Melek, Izzet Paragon's only blocker. Its
  copy trigger probed green in full without the phrase, so what is missing is
  exactly "whenever you cast … FROM YOUR LIBRARY". Count it against the
  source-phrase family the permission opened.
- **The face-down look-at rider** — 3 lines plus one coordinated. "You may look
  at face-down creatures you don't control any time" (Keeper of the Lens, Found
  Footage, Lumbering Laundry), and Lens of Clarity's coordination of it with the
  library form. The same construction over an OBJECT DESCRIPTION rather than a
  place, overriding [CR#708.5] where the two landed rows override [CR#401.2] and
  [CR#402.3]; it wants a face-down predicate this grammar has no word for.
  Keeper of the Lens is a one-line whole card the day it lands.
- **The non-agreeing look-at possessive** — 1 sentence. Xanathar, Guild
  Kingpin's "you may look at the top card of THEIR library any time", the only
  one of the family's 77 sentences whose possessive does not agree with its
  subject. Its card additionally wants a play-the-top-card permission over
  another player's library and the mana half, so the possessor slot alone lands
  nothing.
- **The bare window on a permission** — 2 sentences. "During each of your turns,
  you may play a land and cast a permanent spell of each permanent type from
  your graveyard" (Muldrotha, the Gravetide) and Coram, the Undertaker. These
  permit REPEATEDLY inside a window where `PlayLimit` permits once, so they are
  a different axis and were deliberately kept out of it. Muldrotha additionally
  wants "of each permanent type", a distributive quantifier over card types.
- **The subtype-narrowed spell complement** — blocked one level down. `And
  [spell, Or [HasSubtype Aura, HasSubtype Equipment]]` is refused by
  `ZoneCoherent`, because a permanent-only subtype word answers the battlefield
  inside a conjunction whose head answers the stack. DANITHA, NEW BENALIA'S
  LIGHT is otherwise whole (three keyword lines and a limited graveyard
  permission) and this is its only blocker. It is the zone-coherence question
  asked INSIDE the predicate vocabulary rather than at the permission, and it
  should be answered there the same way or explicitly differently — a
  creature-type subtype does not trigger it (Gisa and Geralf's "Zombie creature
  spell" elaborates), so the cell is narrow and well-defined.
- **The self-permission's exclusion clause** — 1 sentence. Haakon, Stromgald
  Scourge's "You may cast this card from your graveyard, but not from anywhere
  else". Its second line (a conditional over a subtype-narrowed graveyard cast)
  probed green; the exclusion is the only blocker, and it is a negative
  co-ordinate on a permission that nothing else writes.

### Companion blockers, recorded and NOT this round's

- **"without paying its mana cost"** — the alternative-cost family's, with its
  own entry, and the single largest companion blocker in the whole family: it
  appears on a clear majority of the permission cards whose other lines are
  otherwise writable (Omniscience is one line and blocked on nothing else;
  Omnispell Adept, Maelstrom Archangel, Jace's Mindseeker, Etali, Villainous
  Wealth, Epic Experiment and dozens more).
- Routed to the entries that own them: Vizier of the Menagerie (the mana half),
  Conspicuous Snoop and Skill Borrower (ability borrowing), Vampire Nocturnus
  (the coordinated subject — its top-card condition probed green), Crown of
  Convergence and Mul Daya Channelers, Bolas's Citadel, Experimental Frenzy,
  Augur of Autumn and Cemetery Illuminator (an alternative cost, a source-phrase
  prohibition, a different-powers count, a shares-a-card-type relation).

Counts here are a prior session's measurements; re-measure before building.

## The game-outcome gates

Three measured entries meet at the game-outcome gate: a count over players in a
standing outcome state, two gate kinds sharing one subject, and an immunity that
carves out a single state-based cause instead of the whole outcome. All three
are small, all three are blocked on vocabulary rather than on evidence, and the
coordination question is deliberately wider than this site.

### The player-set count — 2 supported lines

"for each player who has lost the game" (Rampant Frogantua) and "if two or more
players have lost the game" (Hot Pursuit). A count over **players in a standing
state**, measured and found *not* to be the lookback query it looks like. Small,
and this is the only reader those two lines want.

### Coordination under one subject — 2 cards

"players can't lose the game or win the game this turn" (Everybody Lives!, and
Platinum Persecutor without the span): two gate **kinds** sharing one subject,
which the one-clause-one-gate row deliberately cannot say. It is the same shape
as the already-landed coordinated **event** and coordinated **deed** ("can't
attack or block", 6 lines) at a third site, so **design it once across all three
rather than per family** — a fix that only reaches the gate is the wrong size.

### Partial-cause outcome immunity — 9 lines

"You don't lose the game for having 0 or less life" (Phyrexian Unlife's family).
It carves out **one** state-based cause ([CR#104.3b,104.3c,104.3d]) and leaves
[CR#104.3e] intact, so it is not the whole-gate refusal already written but a
narrower thing. It wants an SBA-cause vocabulary the grammar has none of; naming
the causes is the substance of the entry.

## From the v1 comparison (2026-08-24)

[The v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md)
grades deontics (axis 14) **WORSE** against the crate's single modal algebra:
`Deontic { May, Cant, Must, Gate(_, [CostComponent]), Expanded }`
(`deontic.rs:325`) over `DeonticAction` (`deontic.rs:186`) with ten deed kinds
— `Attack`, `Block { count }`, `Target { by: DeedAgent, on }`, `Attach`,
`Cast { what, by, from, window, cost, tag }`, `Play`, `Activate`, `Regenerate`,
`Counter`, `Untap` — plus a general counterfactual,
`AsThough::Counterfactual { premise: Predicate, then: Arc<Deontic> }`
(`deontic.rs:49`).

**One claim in that report is wrong and is corrected here so it is not
re-derived.** It says v2 has "no `Must` or `Gate`". Both exist:
`Compulsion` is `Forbid | Require | GatedBy (c : Cost bs)`
(`Experimental.idr:2822-2825`), which is Cant/Must/Gate. The real narrowing is
**reach**, not polarity — `Compulsion` is only available through
`StaticEffect.Deontic` (`Experimental.idr:2679`), whose `Deed` is
`Attack | Block` (`Events.idr:285`), so Must and Gate cover two deed kinds of
ten and the other modal carriers (`PlayerCant` over `PlayerAct`, `ObjectCant`
over `ObjectAct`) are Cant-only with no Must, Gate or May at all. The permission
row this ticket already schedules is the May half of that same narrowing.

What the report adds beyond what is scheduled above:

- **The counterfactual is one hardcoded case**, `PlayAsThough = HadFlash`
  (`Events.idr:386`). The section above already owns deciding whether the
  negated-keyword payload gets a general counterfactual vocabulary; the report's
  contribution is that the crate's premise slot is an arbitrary `Predicate`, so
  "general" has a concrete shape to be argued for or against.
- **"Can't be the target of" needs a source/spell distinction the act
  vocabularies have no room for.** `ObjectAct` is
  `Countered | Cast | Played | Copied | Activated` (`Words.idr:2717`) with no
  targeting member; the workbench's `CantBe : (e : Effect bs) -> (act :
  ObjectAct) -> (what : Noun (preIntro e) k)` (`Experimental.idr:3229`) is a
  rider on an effect, a different construction from a standing prohibition. The
  crate models it as `DeonticAction::Target { by: DeedAgent { stack_object,
  source }, on }` (`deontic.rs:212,142`) — the distinction "hexproof from"
  needs. Take that as the shape to answer when the 39 targeting sentences above
  are written; the count is this ticket's, the source/spell split is the report's
  addition to it.
- **The eight unreached deed kinds** — `Attach`, `Cast`, `Play`, `Activate`,
  `Regenerate`, `Counter`, `Untap` and the counted `Block` — are where the
  well-built `Deed`/`Role`/`DeonticPatient` machinery does not reach. Whether
  they become `Deed` rows or stay distributed across the act vocabularies is a
  decision this region owes; the qualified-cast and activation-prohibition
  entries above are two of them arriving one at a time.

  **RULED (user, 2026-08-27): unify — and the deed slot is a LABEL, not an
  enum row.** One deontic carrier holds the full modal algebra
  (May/Can't/Must/Gate) for every deed; but the deeds do NOT become rows of a
  closed `Deed` enum — that would mint a second act vocabulary parallel to the
  open `VerbLabel` one (the user's challenge, and the shape the verb/subtype/
  keyword conversions retired). The deed slot is an open `VerbLabel` behind the
  fail-closed `KnownVerb` gate; per-deed parameter shapes (Block's count,
  Cast's what/from/window, Target's by-source/by-spell agent) are label-keyed
  facts/slots in the `verbFacts` mold, never constructor arguments on a closed
  enum. The existing `Deed = Attack | Block` retires into labels;
  `PlayerCant`/`ObjectCant` retire into (or respell over) the unified carrier.
  Exhaustiveness is traded for the gate, the established price of every open
  vocabulary here.

  **RULED (user, 2026-08-27): the as-though counterfactual is a premise SLOT
  on the deontic row, never a node above it.** [CR#609.4]'s own grammar is
  "may do something as though some condition were true — this applies only to
  the stated effect": the permission is the head, the counterfactual a scoped
  rider. v1's `AsThough(Counterfactual(premise, then: Deontic))` inverts that
  and admits pairings the rule cannot express (a free-floating May or Cant
  under an unrelated premise) — do NOT copy it. Shape: modality x deed label
  x `asThough : Maybe premise`, the premise's SORT depending on the deed —
  object/condition predicate at target/cast/play (Glaring Spotlight =
  `May Target ... asThough (Not (Has Hexproof))`; the hardcoded
  `PlayAsThough = HadFlash` collapses into `May Cast ... asThough HasFlash`),
  the mana-symbol matcher at the spend deed ([CR#609.4b] — v1's separate
  `SpendAsThough` row folds in too; cross-referenced in
  workbench-mana-family-residues). Stacked as-thoughs ([CR#609.4a]) are two
  permissions each carrying its own rider. The slot opens on May only; the
  other modalities' zero is measured, not assumed.

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **The cast-timing restriction node.** `Timing` attaches to `Activated` only,
  so Necrologia's cast-window restriction has no carrier (consolidated open gap
  4) — `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`.

## Consumption boundary

`idris/src/Experimental.idr` (`ObjectCant`, `CostsToCast`, `possessorOk`, the
class subject's restrictor slot; `PlayerCant`, `PlayerAct`, `ObjectAct`,
`absentOk`, `Prevention`, `CantUntapMoreThan`; `Compulsion`, the permission row
and the counterfactual slot; `Casts`, `PlayLimit`, `ZoneCoherent`, the
permission and rider rows; the outcome gates, the one-clause-one-gate row and
its coordination, the player count's reader),
`idris/src/Experimental/Words.idr` (`Kind.Ability`, the `Keyword` catalog rows,
the subtype catalog; the act's object description and the targeting complement;
the negated-keyword payload's vocabulary; the face-down predicate, the possessor
slot; the SBA causes if they land as a catalog),
`idris/src/Experimental/Events.idr` (`PlayLimit`'s reader), the pin modules
`idris/src/Experimental/Proofs*.idr` (`badSliceOfGroupPossessor`, the four
permission pins), and the evidence bench `idris/src/Experimental/Cards.idr`. No
Rust crate.

## Acceptance

- The 2 triggered-subject lines, 6 floor riders, 7 restrictor lines and 6
  keyword-catalog lines write; Kopala and Tithe Taker bench or their remaining
  blockers are named exactly.
- `CostsToCast`'s exclusion of a loyalty-symbol additional cost is either
  measured open or left refusing with its reason restated.
- The 74 qualified cast lines write, the five landed-description cards among
  them benching on the new act shape.
- `absentOk` is re-measured across the whole prohibition neighbourhood in one
  pass, and Vexing Shusher's second line writes.
- Every measured zero above still refuses afterwards, pinned rather than silent.
- `Compulsion` gains a permission row that is the deed restriction's twin, and
  "this creature can attack" is writable with no counterfactual present.
- The counterfactual attaches as a slot on that row rather than as a second
  construction.
- The player-subject/creature-actor carrier question raised by the 19-sentence
  cell is answered explicitly, not left to whichever carrier is convenient.
- Whether the negated-keyword payload gets a general counterfactual vocabulary
  is decided here and recorded.
- The four permission pins still refuse after the round.
- The group-possessor cell is answered as a LibrarySlice question, not by
  loosening the riders.
- The `ZoneCoherent` refusal is answered inside the predicate vocabulary the
  same way the permission answered it, or the difference is written down.
- Keeper of the Lens and Danitha, New Benalia's Light bench whole; each cell
  that lands nothing on its own says which companion blocker holds it.
- The coordination lands once and covers the event, the deed and the gate; the
  deed's 6 lines and the gate's 2 cards both elaborate through it.
- The immunity names the cause it carves out and does not become the whole-gate
  refusal.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-amount-ceiling-read (close, 2026-08-26):** ceilinged PERMISSIONS — "play up to N additional lands" (3 printed carriers) and "can block up to N additional creatures" (1): `MayPlayAdditionalLands`'s literal-bound gate carries the core's own "widening waits on a printed line" note, and the printed lines are now measured. Permission seats, so they land here.

- **Routed from workbench-anaphora-e-partitive-surfaces (close, 2026-08-27):** the play-permission "from among" SOURCE — 97 occurrences name a MENTION as what may be played ("play cards from among them"), and `MayPlay`'s `from` is a `ZoneExpr`. A play-permission seat, so it lands here.

- **Routed from workbench-choice-e-subtype-words-and-linkage (close, 2026-08-27):**
  an activation restriction naming WHO may activate — "only the player who
  controls Soul Ransom may activate" shape; Soul Ransom's whole card waits on
  it. A deontic agent restriction, this region's machinery.

- **Routed from workbench-counted-anaphora-narrowings (close, 2026-08-27):**
  a bare `It` inside `DeonticCounterpart` can still resolve to the Deontic's
  own subject — a rules-impossible self-block ([CR#509.1a]) that wants a pin;
  nothing on the bench writes it today. Deontic machinery, so it lands here.

## Routed from sub-round 1 (unified carrier, 2026-08-28)

The carrier landed (see that ticket's "As landed"). Four lines of future
work it could not pay, routed here so they stay live:

- **A bare `Blocked` predicate.** `Blocking`, `BlockedBy m` and
  `CouldBeBlockedBy m` all name a counterpart, so "as though it weren't
  blocked" has no premise word and the 19-sentence assign-damage cell
  cannot bench. Its CARRIER question is answered and recorded
  (`Compulsion.Permit`); only the word is missing. One `Predicate bs
  Object` row.
- **"Abilities you control."** `ControlledBy` is `Object`-kinded and
  `ActivatedBy` is a different relation [CR#602.2], so 4 of the 5
  hexproof lines (Glaring Spotlight, Detection Tower, Kaya Bane of the
  Dead, Autumn Willow) still do not bench; Nowhere to Run's bare "spells
  and abilities" is the one that does.
- **`"Crew"`, `"Saddle"` and `"AssignCombatDamage"` deed rows** for the
  18-line greater-power cell and the 19-line assignment cell. A row each
  in `deedFacts` and nothing else; not written for want of the premise
  words above.
- **Corrections to this file's own numbers**, measured 2026-08-28: the
  targeting prohibition is **30** real sentences, not 39 (a naive sweep
  returns 216 and 186 of those sit inside hexproof/shroud reminder text);
  "can't attack or block" is **109** lines, not 6; and the one-subject
  two-kind outcome coordination is **1** card, Everybody Lives!, not 2 —
  "Platinum Persecutor" is not a card, and Abyssal Persecutor / Platinum
  Angel / Angel's Grace / Herald of Eternal Dawn write two SUBJECTS,
  which is a plain conjunction and already writable.

## Routed from sub-round 2 (qualified acts, 2026-08-28)

The qualified complements landed (see that ticket's "As landed", which
carries the full re-measured table and the three recorded decisions —
the mana restriction's home, the amount floor's home, the ZONE family's
home). Seven lines of future work it could not pay, routed here so they
stay live:

- **The ZONE qualifier family**, 7 lines, is deliberately sub-round 3's:
  "can't cast spells from graveyards" names the Cast deed's SOURCE, the
  same slot `MayPlay.from` holds in the permissive voice. Build it once,
  in the `MayPlay` fold, for both voices.
- **The mana restriction**, 9 real lines (not 48 — 31 of the 40 naive
  hits are Powerstone/Vibranium token reminder text, and the umbrella's
  quoted example sentence is ZERO lines): the mana vocabulary's, since
  `Kind` has no mana row for the deontic subject. Its complement should
  read `DeonticCounterpart`'s noun rather than mint one.
- **The amount floor** "X can't be 0", 19 lines over 17 cards: the
  amount vocabulary's, beside the letter's announcement [CR#107.3].
- **The ordinal turn-of-game window**, 3 lines ("during your first,
  second, or third turns of the game"): `TurnPart` names parts of a
  turn, so `OnlyDuring` refuses it correctly and a turn-ordinal
  vocabulary is a separate ask.
- **The Aura coordination's anaphoric subject**, 43 of the 60 real
  activation-prohibition lines: "Enchanted creature can't attack or
  block, and ITS activated abilities can't be activated". Both halves
  write; the possessive read across the conjunction does not. An
  anaphora question.
- **The causative deed** (The Master, Multiplied, 1 line) and the
  **destruction prohibition** (Ogre Enforcer, 1 line): one wants a deed
  whose complement is a clause, the other a by-clause and an
  unless-clause on one statement. One line each.
- **Necrologia's additional cost** and **Alhammarret's reveal-then-choose
  as-enters**: the two named blockers left on cards whose deontic lines
  landed.
