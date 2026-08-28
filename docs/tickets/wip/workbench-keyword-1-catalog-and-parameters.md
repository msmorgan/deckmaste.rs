# keyword-1: the catalog, the parameters and the class term

Sub-round 1 of [workbench-keyword-parameters-and-attachment](workbench-keyword-parameters-and-attachment.md)
(the umbrella — authoritative for measurements, the standing scope fence, and
acceptance). Owns: the typed parameter cells (AFFINITY's described class — 5
lines — and ANNIHILATOR's number — 21 lines; the slot exists, NO fifth shape);
the keyword-CLASS term (7 cards; serves BOTH the extension list and
`HasKeyword`; `fear`/`shroud` bought only alongside it; the
`AlsoForKeywords` constraints survive un-relaxed); FLASH's grant (Amphibian
Downpour the witness, or the deferral restated); the remaining payload sorts
(protection from a PLAYER — 2 cards; from a CARD TYPE — Serra's Emissary;
Runed Halo's player-subject blocker recorded; the emblem chosen read
recorded); ward WITH a cost (Iymrith); and the routed keyword catalog work —
Saddle, Suspend, Partner with, Changeling (each a data row holding a whole
card off the bench), Emerge + Craft (the concurrent act arm's two shapes —
Craft also wants an ability-described-by-keyword `AbilityClass` cell), the
Crew/Saddle DEED rows for the deontic carrier (the 18-line as-though cell
waits on them), the landwalk family (Magnigoth doubly blocked), and the
five subtype-word vocabularies (Crew, Station, Class ladder, Case pair, Room
doors).

THE SCOPE FENCE GOVERNS: keywords that will become macros stay out until the
verifier transition. Apply it per item: a `keywordFacts`/catalog DATA row
that a benched witness pays for is in scope (Saddle, Suspend, Partner with,
Changeling, Crew-the-word); a keyword's SUB-MACHINERY (the Class level
ladder, the Case to-solve/solved pair, the Room door vocabulary, Station's
charge mechanics, Crew's full crewing) is a design — build it ONLY if it
reduces to existing machinery cheaply; otherwise record precisely what it
needs and decline with the fence cited. The nine core-unresolvable keywords
stay unwritten with the reason recorded. Standard constraints apply.

## As landed (2026-08-28)

Every count below re-measured against `data/derived/cards.jsonl` under
`jq 'select(.supported)'`. Gates: `idris/scripts/build` 23/23, 0 errors,
0 warnings; `cite check --list-noncompliant` 0; `cite check` 0 stale over
19,096 citations; `cite audit --diff` read at all 72 changed sites.

### The parameter cells — both landed, no fifth shape

- **AFFINITY** — `QualityParam`, [CR#702.41a]. Re-measured: 30 distinct
  printed "Affinity for [text]" spellings over 74 supported cards, and 8
  further lines that GRANT it (5 static, 1 loyalty, 2 triggered) — the
  umbrella's "5 lines" is that static-grant subset, not the printed line.
  The described class IS the quality payload, so no shape was minted.
  Witness: **Frogmite**, whole.
- **ANNIHILATOR** — `NumberParam`, regime `Nothing`, [CR#702.86a]. 21
  supported lines mention the word (umbrella's count confirmed), 13 print
  it as a keyword line. Cheapest consumer: **Ulamog's Crusher**, whole —
  its other line is `berserkersOfBloodRidge`'s deontic verbatim.
- The three measured zeros survive: `ParamCost`, `ParamSubject` and
  `ParamNumber` still take bindingless payloads, so only `ParamQuality`
  uses the index.

### The keyword-CLASS term — landed, serving both seats

`KeywordFamily` (a word plus an optional `QualitySort` narrowing) and
`KeywordTerm` (`TheKeyword` | `AnyKeywordIn`) in `Words.idr`.
`HasKeyword`'s slot and `AlsoForKeywords`' list element are now that ONE
type. The quantification is over the word's PARAMETER: [CR#702.16a]
writes every protection ability as "Protection from [quality]" and
[CR#702.14a] makes landwalk "a generic term … as '[type]walk'", so a bare
"protection"/"landwalk" is the word with its parameter left open, and
"protection from any color" is the same word narrowed to [CR#105.1]'s
sort.

- The constraints survive UN-RELAXED. `keywordTermBare` still asks
  `keywordParamless` of a WORD arm; the class arm is admitted by
  `keywordFamilyOk`, its own rule. Three new pins in `ProofsG.idr` prove
  it: `badParameterisedKeywordInList` (ward still refused in a list),
  `badClassOfParamlessKeyword` (flying has no parameter to quantify),
  `badSortedClassOnNumberKeyword` (a sort narrows a quality, not a
  number). The pre-existing `badKeywordListOnPlainLine`,
  `badEmptyKeywordList` and `badKeywordListRepeatingBase` still hold.
- Re-measured: 15 extension carriers, 7 naming a class — "protection"
  bare on 6 (Wretched Bonemass, Cairn Wanderer, Concerted Effort,
  Death-Mask Duplicant, Eater of Virtue, Rayami), "landwalk" on 3 of
  those, "protection from any color" on Escaped Shapeshifter. `fear` 3
  lines, `shroud` 1 — both bought only here, as required.
- Benched WHOLE, 4 of the 7: **Cairn Wanderer** (both classes, fear,
  shroud, changeling — the longest list in the family), **Concerted
  Effort**, **Death-Mask Duplicant**, **Escaped Shapeshifter** (the
  narrowed term's only carrier).
- The remaining 3 are blocked elsewhere and not on this term: Eater of
  Virtue (an Equipment — sub-round 2's lane), Rayami (a death-replacement
  that exiles with a counter), Wretched Bonemass (a craft-transformed
  back face). Indominus Rex, Alpha stays recorded as a different base.

### The payload sorts

- **Protection from a PLAYER — landed.** `ParamQuality` is now
  kind-indexed by `qualityParamKind` (Object or Player, and nothing
  else). Decided on the payload's algebra per the cited authority:
  [CR#702.16k] makes "protection from [a player]" a VARIANT whose slot
  names a player outright and [CR#109.3] makes a player no characteristic
  of anything, so the phrase names its referent. NOT a join —
  [CR#702.16g] splits "from [A] and from [B]" into two abilities, so no
  printed slot names both kinds and there is no union site to mark.
  Witness: **True-Name Nemesis**, whole. Guardian Archon stays off on its
  own line's mixed-group subject ("You and target permanent you control
  each gain …").
- **Protection from a CARD TYPE — `CardTypeQ` already serves**, and it
  was never Serra's Emissary's blocker: the payload writes as
  `OfChosen CardTypeQ`. What refuses the card is its SUBJECT, the same
  mixed group Guardian Archon writes. Recorded at `qualityParamKind`.
- **Runed Halo** recorded there too: its parameter writes; its subject is
  a player where `Gains` takes an object. One line.
- **The emblem's chosen read** stays unscheduled at 1 card (re-measured:
  one supported emblem line reads a chosen value, Oko, Lorwyn Liege's).

### Ward WITH a cost — landed

**Iymrith, Desert Doom**, whole: `keywordCosting "Ward" (Mana [generic 4])`
under the postposed static, [CR#702.21a]. The `iymrithGapDraw` fragment
is retired into the card; `dragonlordOjutaiHexproof`'s docstring now
carries the pairing.

### The routed catalog rows — fence applied per item

Landed, each with a benched witness:

| word | shape | rule | witness |
| --- | --- | --- | --- |
| Fear | `NoParam` | [CR#702.36a,702.36b] | Cairn Wanderer |
| Shroud | `NoParam` | [CR#702.18a] | Cairn Wanderer |
| Landwalk | `QualityParam` | [CR#702.14a,702.14c] | the class term |
| Changeling | `NoParam` | [CR#702.73a] | Cairn Wanderer, Bloodline Pretender |
| Crew | `NumberParam` | [CR#702.122a] | Debris Beetle |
| Saddle | `NumberParam` | [CR#702.171a] | Autarch Mammoth |
| PartnerWith | `QualityParam` | [CR#702.124j] | Pir, Imaginative Rascal |
| Emerge | `CostParam` | [CR#702.119a] | `foulEmissaryLine` |
| Craft | `CostParam` | [CR#702.167a] | Market Gnome |

Newly whole besides those: **Autarch Mammoth** (`autarchMammothLine` plus
"Saddle 5"), **Bloodline Pretender**, **Pir, Imaginative Rascal**,
**Debris Beetle**, **Market Gnome**. `foulEmissaryLine` is a fragment —
Foul Emissary's other line looks at four, reveals one from among them and
bottoms the rest.

DECLINED, fence cited, with precisely what each needs:

- **SUSPEND.** [CR#702.62a] writes "Suspend N—[cost]": a count of TIME
  COUNTERS beside a cost, and a counter count is no component of one, so
  the parameter is genuinely COMPOUND and `KeywordParamShape` has no
  compound arm. Minting one is the same decision the restricted equip
  line's "[quality] [cost]" [CR#702.6c] asks, which sub-round 2 owns —
  one decision for both, and this ticket's pin forbids a fifth shape.
  **Veiling Oddity stays at `veilingOddityLine`**, its docstring updated
  to the moved blocker. *This is a deviation from the ticket's
  expectation that Suspend land with its witness.*
- **CRAFT is NOT the same case** and did land: [CR#702.167a] expands
  "Craft with [materials] [cost]" into ONE activation cost —
  "[Cost], Exile this permanent, Exile [materials] …" — so its two
  printed slots are two components of one cost [CR#118.1], which
  `Cost`'s `Compound`/`Do` arms already express. Recorded beside the
  suspend note so the equip round has the distinction.
- **STATION.** [CR#702.184a]'s word is bare and would be a one-line row,
  but every station card also writes the station SYMBOLS [CR#702.184b],
  themselves keyword abilities on a nonstandard layout, so no card comes
  whole with the word alone and the row would have no consumer. The
  symbol ladder is `ChapterMark`'s question again — sub-machinery.
- **The Class level ladder, the Case to-solve/solved pair, the Room door
  vocabulary.** None is a keyword vocabulary at all; each is a printed
  frame with its own rules section, and each holds exactly one routed
  card. Same fence as Station: not catalog rows, and nothing in them
  reduces to existing machinery.
- **Crew's full crewing** — the tap-a-set-by-total-power cost — is the
  keyword's own expansion [CR#702.122a] and belongs to the macro, not to
  this grammar. Crew-the-WORD is what the fence admits, and it landed.
- **The `"Crew"` and `"Saddle"` DEED rows.** Measured: NO witness benches
  through them. The 18 as-though lines (re-measured at 18) write "as
  though its power were 2 greater", and `AsThoughOf` takes a
  `Predicate bs Object` — no predicate states a characteristic SHIFTED by
  an amount, so those lines wait on a counterfactual-VALUE premise and
  not on the deed rows at all. The only prohibition carriers (Revoke
  Privileges, Bound in Gold, Intercessor's Arrest — "can't attack, block,
  or crew Vehicles") need a PER-DEED complement: `deonticPatientOk`
  checks one shared `DeonticPatient` against every coordinated deed, and
  [CR#506.3] gives attacking only planeswalkers and battles at the
  patient, so a Vehicle complement fails for the attack and block arms.
  Two named blockers, neither of them a deed row. *Deviation from
  prohibition-1's routing note, which assumed the as-though cell waited
  on the rows.*
- **The landwalk family.** The row landed for the class term alone. An
  INDIVIDUAL landwalk still does not write — its parameter is a land
  type and the land-subtype quality sort is routed to
  workbench-quality-sort-gaps — and **Magnigoth Treefolk stays doubly
  blocked** past it: "Domain — For each basic land type among lands you
  control, this creature has landwalk of that type" also wants the
  distributive over a counted type axis. Recorded at the `Landwalk` row.

### FLASH's grant — deferral restated

Nothing was owed: the `"Flash"` row and **Amphibian Downpour** (whole,
under Flash and Storm) had both already landed. Re-measured, the printed
keyword LINE is **598** supported lines, not 500. The GRANT stays
deferred on evidence: one supported line grants the word to an object off
the stack ("Creature cards you own that aren't on the battlefield have
flash"), which `grantSubjectFits` refuses because flash's regime is
`AtCasting` — [CR#113.6e]'s rule, not a gap. The six "This spell has
flash as long as …" lines are stack-side self-statics, a different
construction.

### The nine core-unresolvable keywords — now EIGHT

Recorded in `keywordFacts`' own docstring, where the vocabulary is
defined. Cascade, replicate, conspire, rebound, prowl, freerunning,
demonstrate and sticker kicker stay unwritten for the recorded reason.
**EMERGE left the list**: the premise was "no macro and zero built-card
use", and a DESCRIBING read is a use the count had not seen —
`HasKeyword "Emerge"` on the spell Foul Emissary's trigger watches. Its
row buys no keyword line the bench writes. *Deviation flagged: the
umbrella's acceptance said nine stay unwritten; the ticket's routed item
said to add what the witness pays for, and the witness benched.*

### Not touched (sub-round 2's lane)

`AttachHost`, `attachHeadOk`, `attachedCheckOk`, `CostParam`'s equip
side, `statusEventOk`, `eventSpan`.
