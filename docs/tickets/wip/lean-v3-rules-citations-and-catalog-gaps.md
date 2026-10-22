---
needs: []
---
# Lean docstrings and catalogs that misstate their rule, and the catalog entries the CR adds

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#59, #64, #65, #67, #69; the mtg-rules review's state.md corrections 17-18 and its casting.md audit
note; style-guide finding A3's Lean half (`docs/evidence/mtg-rules-review-2026-10-21/style-guide-fixes.md`
#3; the guide half is [oracle-style-guide-mtg-rules-port](oracle-style-guide-mtg-rules-port.md)).
Verified against head 2026-10-21. Mostly prose; items 2, 5 and 9 also change behaviour or add pins.

## 1. Missing rules-defined subtypes

`Definitions/Subtypes.lean` calls itself "The rules-defined subtypes" but lacks:
- **Heartwood**, an artifact type [CR#205.3g] (added in the 2026-09-25 rules update). Add `semantic_definition heartwood
  : Subtype := .of .artifact "Heartwood"` and the registry declaration
  `plugins_v2/builtin/macros/subtypes/artifact/heartwood.ron`. The v4 catalogs already have it
  (`crates/deckmaste_lexical_source_v4/catalogs/subtypes.ron:17`). The Heartwood predefined token
  [CR#111.10x] is a separate question (the Lean and builtin token catalogs are partial), not part
  of this item.
- **Undercity**, the one dungeon type [CR#205.3p]. The `dungeon` card type exists
  (`Words.lean:24-28`) and v4 has it (`subtypes.ron:364`).

The other subtype classes [CR#205.3g..205.3q] match the CR at head (Attraction is Un-set and deferred).

## 2. `positiveZones?`'s docstring, and its two callers disagree

`Location.positiveZones?` (`Abilities.lean:1248-1250`) says "A move from a positive list of zones,
observed on its departure, is read with its objects' last known information [CR#603.6c,603.10a]",
as if every positive-list move looked back. Only some zone-change triggers look back
[CR#603.10a]. Reword: "The positive locations `loc` names, as a list of its disjuncts, or none for
a complement or anywhere; a 'from anywhere' move is never a leaves-the-battlefield trigger
[CR#603.6c]. Callers decide whether a move is observed on departure [CR#603.10a]."

The callers decide differently on a mixed origin list: `Check/Phrase.lean:3117-3119`
(`historicalOrigin`) treats the move as battlefield-historical if *any* origin is the battlefield,
while `Templates.lean:1065-1070` (`zoneChange`'s `before`) classes it as departure-observed only
if *all* origins are in the allowed set. On `[battlefield, hand]` the patient is read historically
while the move is classed `.arrival`. Make them agree (one shared classifier), and pin the mixed
case. [lean-v3-authoring-surface](lean-v3-authoring-surface.md) says the function drives both but
does not record the split.

## 3. The cost-reduction floor and its spelling

`CostShift.generic`'s docstring (`Abilities.lean:600-602`) cites [CR#601.2f] for "to a minimum of
{1}"; that rule gives only the {0} rules floor. Replace with: "`floor`: a card-text floor on the
mana left in the cost after this reduction ('This effect can't reduce the mana in that cost to
less than one mana'; *Training Grounds*, *Power Artifact*). It counts all mana in the cost, colored
included, not only the generic part this shift lowers: under *Training Grounds* {2}{G} becomes
{G} and {2} becomes {1} (*Training Grounds* ruling, 2025-10-02). With no floor, only the rules
floor of {0} applies [CR#601.2f]." The `less` Template's spelling (`Templates.lean:2300-2302`,
"costs <amount> less [to a floor of <floor>]") is unattested; Oracle prints "costs <amount> less[.
This effect can't reduce the mana in that cost to less than <floor> mana]". Users:
`Cards/Cost.lean:552,563`.

## 4. Designations: sector is deferred; battles have a protector

`Definitions/Designations.lean:5` says "The rules-defined designation labels". Sector
[CR#702.158b] is only on an Unfinity card, deferred with the Un-sets (`crates/xtask/src/facts.rs:1667-1685`
exempts it as out of scope): say so in the docstring rather than adding it. The battle protector
[CR#310.9] ("Each battle has a player designated as its protector") is in scope and missing: add a
protector relation valued in players, or record it as deferred until battles are on the bench. The
engine side is [engine-battles](engine-battles.md).

## 5. `thisWay` collects every event the ability caused

`lean/CONTRACTS.md:227` glosses `thisWay` as "the event the enclosing action caused", one event of
one action. `:299` (and the checker) read it as "referents the current ability stamped", every
event of the named kind the spell or ability caused. Oracle uses the wider reading (*Sinister
Concierge*: "exile it and put three time counters on it. If you do, exile up to one target creature
and put three time counters on it. Each card exiled this way …"; *Rally the Horde*; *Thieves'
Auction*). Rewrite `:227` to match `:299`, including events from paying the ability's costs, and
add a *Sinister Concierge* pin that "each card exiled this way" reads both exiled cards.

**Owner ruling (2026-10-22), option B.** At head the checker dropped everything a `may`
(`withContinuation`) body introduced or stamped, so the pin was refused. Objects an action stamped
inside an optional body or arm escape it, but only to Participial Descriptions ("… this way", "the
exiled card"); pronoun scope is unchanged. This is discourse scope: "this way" ranges over the
events that happened. Pin both the Concierge read and the contrast (a pronoun after the body still
cannot reach its object).

Out of scope, not accepted: exclusive paths are not yet reasoned about within one ability. "Sacrifice
a creature. If you don't, sacrifice an artifact", then a singular "the permanent sacrificed this
way", finds two candidates though exactly one sacrifice happens, and is refused. Fixing it needs
path-exclusivity reasoning for the current ability's own arms (as `linkArms` gives linked
abilities). No corpus card prints this.

## 6. `LinkedReadback` miscites the self-link rule

`Proofs/LinkedReadback.lean:139-140` cites [CR#607.1c] for "the card exiled this way" not reaching
a linked ability's deed. [CR#607.1c] self-links an ability that acts and refers back (*Mimic Vat*
reads earlier resolutions), so it can't ground a current-deed restriction. The result holds because
"exiled this way" is no linking phrase under [CR#607.2a]. Cite [CR#607.1,607.2a]; keep [CR#607.1c] for
self-linked readbacks.

## 7. A keyword counter's bearer gains the keyword

`Proofs/Rules.lean:109-113` (`flyingCounter`) and the `Conferral.property` docstring
(`Rules.lean:64-66`) say a keyword counter's grant "states a quality of the bearer rather than
giving it an ability of its own [CR#113.12]". A keyword counter "causes that object to gain that
keyword" [CR#122.1b]; a gained ability "can be removed by another effect" [CR#113.12]; keyword
counters apply in layer 6 [CR#613.1f]. Rewrite both: the counter is not an ability, but its bearer
gains the keyword in layer 6, and ability-removing effects remove it by timestamp. Keep [CR#113.12]'s
quality reading for real quality statements (`Rules.lean:190`, Combatant).

## 8. N keyword counters are N instances

`Rules.lean:86-92` says N keyword counters "grant it N times, which is having it once". Multiplicity
is each keyword's own rule [CR#113.2c]: flying's instances are redundant [CR#702.9c], exalted's and
decayed's are not (*Emissary of Soulfire* ruling, 2024-06-07). Reword: "…so N of them grant it N
times [CR#122.1b,113.2c]; that is having it once only where the keyword's own rule makes instances
redundant (flying [CR#702.9c]); each exalted or decayed instance triggers separately
[CR#702.83a,702.147a]." The exalted and decayed counters are `Definitions/Counters.lean:57,76`, their
conferrals `exaltedCounter.ron:10`, `decayedCounter.ron:10`. Consumers must count instances per
keyword.

## 9. "Doesn't untap" as `cant` is an engine choice, not CR-mandated

`docs/decisions/semantics-v3.md:454` writes `cant(any, untap(this))` for "doesn't untap" with
`[CR#614.17,502.3]`, and [idris-mirror-enum-gaps](idris-mirror-enum-gaps.md):7-9 says it "is `cant`
… not a replacement effect ([CR#614.17,502.3])". The CR does not settle this: [CR#614.17] covers
effects that "state that something can't happen", which "doesn't untap" never says, and
[CR#502.3]'s "keep … from untapping" is the wording [CR#122.1d] uses for the stun counter's
replacement. It is observable: under `cant` the untap can't happen, so the stun counter's
replacement can't apply [CR#614.17c] and the counter stays; as a replacement, the player would
order the two [CR#616.1]. Keep `cant` if wanted (the Lean Definition, `Definitions/Actions.lean:549-552`,
already cites only [CR#502.3,701.26b]), but record it as a UD entry in `docs/engine-adrs.md`, drop
the CR-mandated claim from the ADR row and the idris ticket, and pin the stun-counter case. *Winter
Orb* and *Static Orb*, which print "can't untap", are unaffected.

## Verification

`./lean/scripts/build`; `cargo xtask lean check` (new subtype, RON); `cargo test -p xtask -p
deckmaste_semantics_v2`; the CR citation gates.
