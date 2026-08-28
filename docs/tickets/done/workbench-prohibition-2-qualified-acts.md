# prohibition-2: the qualified acts and complements

Sub-round 2 of [workbench-prohibition-and-permission-acts](workbench-prohibition-and-permission-acts.md)
(the umbrella — authoritative). Runs AFTER sub-round 1's carrier. Owns the
umbrella's sections "Ability-class prohibition residues" (triggered-ability
subject 2, floor rider 6, restrictor 7, five keyword catalog rows, the
Phyrexian name-straddle stays recorded; do NOT repeat the plural-possessor
claim — probe first) and "The prohibition acts' qualified complements" (the
74 qualified cast lines — an act carrying an object description, five
qualifier families, count cap cheapest; the mana restriction 48; the
activation prohibition 35 — re-probe first; the amount floor 18 → belongs
with the amount vocabulary, decide and record; the long tail; the SPANLESS
clause — re-measure `absentOk` across the whole neighbourhood in ONE pass,
Vexing Shusher's second line writes; Gaddock Teeg's mana-cost-symbols
predicate), plus routed: the cast-timing restriction node (Necrologia) and
the who-may-activate agent restriction (Soul Ransom whole).

Pins: qualified acts land on sub-round 1's carrier (label-keyed complements),
never as new closed vocabularies; measured zeros stay pinned; declined cells
carry counts. Standard constraints apply.

## As landed (2026-08-28)

### The finding the round turned on

**The qualified complement was already paid for.** Sub-round 1's seam
note expected a new complement slot; the carrier already had one.
`DeonticPatient.DeonticCounterpart` reads the deed's other end through
`counterRole`, so `Deontic who Forbid ["Cast"] Agent (DeonticCounterpart
<spell noun>)` type-checks and is gated correctly — [CR#601.2] seats a
cast spell on the stack, which is exactly where `deedFacts` puts a
`Cast` patient, and a land noun there is refused by the patient's own
type list [CR#305.1]. What landed is a widened doc, `Macros.cantDoTo`,
and a generalisation over `Kind` (below); no new arm.

### Grammar changes

| change | why | pays |
| --- | --- | --- |
| `DeonticCounterpart` widened over `Kind` | casting takes an object [CR#601.2], activating takes an ability [CR#602.2,109.1]; a line coordinating them names two kinds | the qualified cast AND the qualified activation in the player voice |
| `Macros.cantDoTo` | the spelling of the above | 74-line family |
| `CantUntapMoreThan` → `CantMoreThan who deed k p` | a cap bounds how many times a DEED is done; one row over the deed vocabulary, not one per verb | 9 untap + 12 cast + 3 draw = 24 lines |
| `deedFacts` += `Untap` [CR#502.3], `Sacrifice` [CR#701.21a], `Trigger` [CR#603.2]; `DrawCard` gains a patient [CR#121.1] | each deed's own rule states both ends; a cap needs a role for what it counts | the cap, the long tail, the ward line |
| `StaticEffect.OnlyDuring p w se` | the WINDOW a static statement is confined to — `Conditionally`'s twin at the turn structure [CR#500.1]. A duration says when a statement ends [CR#611.2a], a condition says what must hold, a window says which part of every turn it is in force during; none is the others | 10 TIMING cast lines, 47 cast-window lines (routed gap), Tithe Taker |
| `Card.staticOnSpellCardOk` admits the windowed cast permission | [CR#113.6e] functions an object's own cast restriction from the zones it could be cast from and from the stack | the 47 |
| `Predicate.ManaCostHasX` | a description by a printed SYMBOL [CR#202.1], not by the value X took [CR#107.3a] | Gaddock Teeg's second line (the only one) |
| `CostShift.CostLess` gains `floor : Maybe (Amount bs)` | the rider bounds this effect's own amount and forbids no agent anything; [CR#601.2f]'s {0} is a different, lower bound | 10 floor-rider cards |
| `AbilityAt.Activated` gains `activator : Maybe (Noun bs Player)` | [CR#602.2]'s own "unless the object specifically says otherwise" — beside the window and the usage limit, not a second statement | Soul Ransom (routed) |
| `keywordFacts` += `Boast`, `Exhaust`, `PowerUp` under a new `AbilityParam` shape; += `Afterlife` | [CR#702.142a,702.177a,702.193a] each read "a keyword that adds additional rules to the activated ability that follows it"; no `KeywordParam` produces `AbilityParam`, so none is writable as a keyword LINE while `KeywordClass` can name it | the 6 keyword-class lines; Tithe Taker's second line |

### Re-measured counts (2026-08-28, `jq 'select(.supported)'`, line-level)

| claim | ticket | measured | note |
| --- | --- | --- | --- |
| qualified cast lines | 74 | **101 real** (130 naive − 29 split-second/epic reminder) | families below |
| — TYPE | — | **23** | writes |
| — NAME | — | **15** | writes |
| — COUNT CAP | 9 | **14** (9 bare "more than one spell each turn") | writes |
| — TIMING | — | **14** (6 your turn, 3 combat, 2 next-turn duration, 3 ordinal turn-of-game) | 9 write; 3 ordinal declined |
| — ZONE | — | **7** (4 graveyards, 2 anywhere-other-than-hands, 1 graveyards-or-exile) | declined, see below |
| — mana value / colour / chosen type | — | **7** | writes |
| — BARE | — | **12** | already wrote |
| activation prohibition | 35 | **60 real** (74 naive − 13 detain reminder − 1 "only once") | 17 standalone class sentences already benched; 43 are conjuncts of an Aura coordination |
| mana restriction | 48 | **9 real** (40 naive − 31 Powerstone/Vibranium token reminder) | declined, see below |
| amount floor "X can't be 0" | 18 | **19 lines / 17 cards** | declined, see below |
| floor rider | 6 | **10 cards** | LANDED |
| "that target …" restrictor | 7 | **7** | writes; 5 at `KeywordClass "Equip"` |
| keyword catalog rows | 5 | **3 missing** (Cycling and Ninjutsu already had rows) | LANDED |
| triggered-ability subject | 2 | **1 real** (13 naive − 11 read-ahead reminder [CR#702.155a] − 1 other) | Nowhere to Run LANDED; The Master, Multiplied is a causative, declined |
| sacrifice | 6 | **1 standalone sentence** (8 naive; 7 are conjuncts or quoted token abilities) | LANDED |
| destruction | 5 | **1 real** (7 naive; 6 are indestructible reminder text) | declined — Ogre Enforcer carries a by-clause and an unless-clause |
| copying | 4 | **3 real** ("This spell can't be copied") | LANDED |
| exile | 1 | **0** | MEASURED ZERO — the one hit was "Exile X blue cards" |
| drawing prohibition | 5 | **6** (2 bare, 3 capped, 1 coordinated) | all write |
| cast-window (routed, Necrologia) | — | **47** | LANDED |

### The SPANLESS clause — the premise was stale

`absentOk` **does not exist**. `SpanOk` is
`SpanUnstated : SpanOk k Nothing`, unconditional in the `StaticKind`, so
an unstated duration is admitted at every static kind ([CR#611.2a] gives
one the end of the game). Whatever table the umbrella's
"`absentOk DeedRestriction` is False" described was retired before this
round. Vexing Shusher's second line therefore wanted nothing but its own
noun, and **Vexing Shusher benches whole**. The four mana-permission
riders (Boseiju, Cavern of Souls, Delighted Halfling, Savage Summoning)
and the next-spell clauses (Insist, Overmaster, Mistrise Village) keep
their own second blockers, unchanged and unchased.

### Decisions recorded

- **The mana restriction (9 real lines) belongs to the MANA vocabulary,
  not this carrier.** Its subject is MANA in a pool, and `Kind` has no
  mana row — the deontic carrier's subject is a `Noun bs k` and there is
  no `k` for it. The restriction rides the mana-adding effect ("Add {C}.
  This mana can't be spent to cast a nonartifact spell"), which is where
  the slot goes; its complement is the same spell description
  `DeonticCounterpart` now carries, so the mana family should read that
  noun rather than mint one. Cross-reference: the spend deed named in
  sub-round 1's seams, and [CR#609.4b]'s as-though arm.
  **Correction:** the umbrella's quoted example, "can't be spent to cast
  spells with mana value 3 or greater", is **0 supported lines**. The
  real complements are "a nonartifact spell", "spells from your hand",
  "spells", and "generic mana costs".
- **The amount floor "X can't be 0" (19 lines / 17 cards) belongs to the
  AMOUNT vocabulary.** [CR#107.3] makes X a placeholder whose value the
  controller chooses; the sentence bounds that choice's domain. It
  prohibits nothing an agent does and has no deed, so no honest carrier
  exists here. Not built: the bound belongs beside the letter's
  announcement, which is the amount round's seat.
  The *other* half of the umbrella's "18" — the cost-reduction floor
  rider — is a different question with a different home and **did**
  land, on `CostLess` (10 cards, not 6).
- **The ZONE qualifier family (7 lines) is the Cast/Play deed's SOURCE
  slot, and that slot is sub-round 3's.** "Players can't cast spells
  from graveyards" names where the spell was cast FROM, not where the
  noun is; the noun is on the stack either way. `MayPlay.from` is the
  same slot in the permissive voice, and sub-round 1's seam already
  schedules `MayPlay`'s fold into the carrier for sub-round 3. Building
  a second `from` here would mint the rival slot that fold has to
  reconcile. Declined deliberately, with the count.

### Benched

Whole cards: **Council of the Absolute**, **Gideon's Intervention**,
**Grand Abolisher**, **Festival**, **Rule of Law**, **Spirit of the
Labyrinth**, **Fluctuator**, **Kopala, Warden of Waves**, **Tithe
Taker**, **Gaddock Teeg**, **Vexing Shusher**, **Soul Ransom**,
**Training Grounds**, **Power Artifact**.

Clauses: Failure // Comply's `complyNameLock`, Academic Probation's
`academicProbationNameMode`, Boom Scholar's and Hulk's keyword-class
discounts, Kang the Conqueror's `kangPowerUpLock`, Fervent Champion's
equip discount, Nowhere to Run's `nowhereToRunWardLine`, Hithlain
Rope's `hithlainRopeSacrificeLock`, Display of Power's
`displayOfPowerCopyLock`, Mornsong Aria's two-deed coordination.

**Voidstone Gargoyle already benched whole in sub-round 1** (all four
lines); the umbrella's expectation that its fourth line waited here was
already satisfied. The `Phyrexian` name-straddle stays recorded, as a
naming decision between two catalogs and not a construction.

### Pins

No new pins: nothing measured here is rules-impossible. Every existing
pin still refuses — the full build is the evidence, since a pin that
stopped refusing fails elaboration. The four permission pins and
`ProofsD.badUntapCapGraveyardSet` (re-expressed onto `CantMoreThan`)
included.

### Remainders (not this round)

- **The ordinal turn-of-game window**, 3 lines: "You can't cast Serra
  Avenger during your first, second, or third turns of the game"
  (also Jace Reawakened, Spider-Man 2099). `TurnPart` names parts of a
  turn, not turns counted from the game's start; `OnlyDuring` refuses it
  correctly and a turn-ordinal vocabulary is a separate ask.
- **The Aura coordination's anaphoric subject**, 43 of the 60 activation
  lines: "Enchanted creature can't attack or block, and ITS activated
  abilities can't be activated". Both halves write; what is missing is
  the possessive read of the first clause's subject inside the second.
  An anaphora question, not a deontic one.
- **The causative deed**, 1 line: The Master, Multiplied's "Triggered
  abilities you control can't cause you to sacrifice or exile creature
  tokens you control" — a deed whose complement is a whole clause. No
  other line writes one.
- **The destruction prohibition**, 1 line: Ogre Enforcer's "can't be
  destroyed by lethal damage unless lethal damage dealt by a single
  source is marked on it" wants a by-clause and an unless-clause on the
  same statement. A `"Destroy"` deed row alone pays nothing.
- **`Necrologia`'s additional cost.** Its cast window landed; "As an
  additional cost to cast this spell, pay X life" has no row (`AltCost`
  is the ALTERNATIVE cost). Named exactly, not chased.
- **Alhammarret, High Arbiter.** Its prohibition line is byte-identical
  in shape to Council of the Absolute's and writes; its remaining
  blocker is the as-enters "each opponent reveals their hand. You choose
  the name of a nonland card revealed this way" — a choice whose domain
  is a prior reveal.
- **Academic Probation's second mode.** "Choose target nonland
  permanent. Until your next turn, it can't attack or block" is refused
  because [CR#506.3] admits only a creature at the Attack agent and the
  phrase names a nonland permanent. Correct per the rule as stated; a
  line that expects the permanent to become a creature is a layers
  question and not this carrier's.
