---
needs: []
---
# Read a list of bare option words after a choice verb: Choose odd or even.

## Why

A choice among named options is written as a bare coordination of the option
words: *Choose odd or even.*, *Choose artifact, creature, or land.*, *choose
friend or foe*, *Choose left or right.*, *choose first strike, vigilance, or
lifelink*, *votes for death or taxes*. It reads only when every member happens
to be a bare-capable noun ("Choose heads or tails." 2; "Each player votes for
time or money." 4). Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): hidden under `at:even .`,
`at:foe .`, `at:land .`, `at:sorcery .`, `at:lifelink .`, `at:random .` and
about thirty one-face `word:` keys. `cargo xtask english-v3 grep -i
'\b(chooses?|votes? for) [a-z-]+( [a-z]+)?(, [a-z -]+)*,? or [a-z-]+( [a-z]+)?[.:]'`
finds **116** faces, **76** unread; 72 failing lines match, **57** the only
failing line of their face (the 40 covered are mostly *choose target X or Y*
and *choose among one or two targets*, not this). A hand-cut of the sole lines
gives **67**: **39** whose option words are all declared Lexemes (*odd or
even* 8, card types 10, *friend or foe* 5, *left or right* 6, keyword lists
5, numerals 4, colours 2, *land or nonland* 3) and **28** with an undeclared
label word (*grace or condemnation*, *war or peace*; 22 are *votes for*).
`english-v3-hygiene-3` listed "each player votes for X or Y 27" as unrouted.

## Goal

A coordination of bare option words reads as the Complement of a choice verb
(*choose*, *vote for*), with byte-exact realisation, for every option word
that is a declared Lexeme. The 28 undeclared-label faces are counted and
routed, not read here (OW5).

## Analysis

No CGEL passage is relied on: the options are mentioned, not used (*odd* is
not predicated of anything; *creature* has no Determiner although it is a
singular count noun). The analysis is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): *choose*
(`core-verb:Choose`) takes an NP Object through `SelectedPredicate`; the
coordination schemas build `Coordination` over NPs; `BareNounPhrase` admits
plural and mass nouns, which is the only reason *heads or tails* and *time or
money* read. The covered Siege faces read by accident: "As this enchantment
enters, choose Khans or Dragons." reads 1 as `ProperNameNounPhrase`
(`lexeme:Name/Khan`) *or* `BareNounPhrase` (*Dragons*), and "Choose Island or
Swamp." reads through `catalog:card-names.txt/Island`. Nearest declared
neighbour for an open label is the lexical Category `FlavorWord`
(`FlavorWordHead`).

Orchestrator rulings 2026-10-11:

- OW1. One Option List route: a coordination (pair or comma series) whose
  coordinates are bare option words, selected as Complement by a frame the
  choice verb declares. Refine the existing coordination schemas with one
  member projection; no parallel coordination family. If a new Construction is
  unavoidable it is ONE member projection (R-budget): say so before building,
  with the faces it buys.
- OW2 (R-class). The licence is class-level: the verb's declared option frame
  (every existing verb of the choice class gets it: *choose*, *vote*; not a
  row per attested option list), and the member is any declared Lexeme of the
  mentionable lexical Categories (Noun, Adjective, Keyword, Numeral, colour
  word) in its citation form. No list of option words, no test on *odd*,
  *friend*, *left*, and no card test. Mint no Lexeme for an unattested word.
- OW3 (R-one, R-wrong). The twin to avoid is the bare-NP coordination that
  reads today by accident. Before designing, record the trees of "Choose heads
  or tails." (2), "Each player votes for time or money." (4), "choose Khans or
  Dragons" (Monastery Siege) and "Choose Island or Swamp." (Roots of Life). If
  the Option List would read beside them, STOP with both trees: which analysis
  those faces keep is a ruling (R-prune permits retiring the accidental one;
  the card-name and `Name/Khan` Readings are wrong analyses, not Readings to
  protect).
- OW4. *at random* after the list (*Choose 1, 2, or 3 at random.*, 3 faces)
  is the existing Adjunct; in scope only if it falls out. A trailing colon
  (Phyrexian Splicer's cost) is the existing cost position.
- OW5. Undeclared label words (28 sole faces: council votes, *war or peace*,
  *Believe or Doubt*) are card-local labels. They are NOT read by declaring
  each word as a Noun. Out of scope here; report the count and the candidate
  mechanism (an open label slot like `FlavorWord`, or per-face declarations)
  for an owner decision. Later mentions (*for each death vote*, *each player
  who chose war*) go with that decision.
- OW6. Newly covered faces are gains. Reading-count changes are expected only
  on covered faces matching the selector, and only as OW3's shape; under OW3
  that is a STOP, not a result to report afterwards. Any other change is a
  defect.

## Findings for the owner decision

OW7 (orchestrator, 2026-10-11): do not implement. The owner decides what an
option word is, including OW5's card-local labels, the existing NP Readings,
plural/Catalog membership and later mentions, as one design decision. This
section records evidence and alternatives; it does not settle that decision.

### Recorded trees and claim-parent probes

Measured on empty working change `nomvqwun`, claim parent `pxxrtuss`
(default ancestor `pllsstmm`). Every probe used `cargo xtask english-v3
probe --category sentence --readings 8 --json --text ...`; the two named
card controls also used `--as-card`. Enumeration completed in every case,
with no internal issues. Raw probe trees are session evidence under
`target/english-v3/english-v3-option-word-choice-list/*-before.json`.

All seven witnesses remain unread: Extinction Event 0; Turnabout 0; Pir's
Whim 0; Order of Succession 0; Angelic Skirmisher 0; Capital Punishment 0;
random numerals 0. No witness was dropped. Controls: heads or tails 2;
one or both 1; a color 1; a creature type 1; votes for a creature 2;
votes 1; time or money 4; Monastery Siege 1; Roots of Life 1.

Existing trees, abbreviated without their sentence and case wrappers:

- Heads/tails: `SelectedPredicate(Choose, Object(BareNounPhrase(
  Coordination<Nominal>(Noun(Head/Plural), or, Noun(Tail/Plural)))))`,
  and `SelectedPredicate(Choose, Object(Coordination<NounPhrase>(
  BareNounPhrase(Noun(Head/Plural)), or, BareNounPhrase(Noun(Tail/Plural)))))`.
- Time/money: `SelectedPredicate(vote/frame 2, for, Object(NP))`, or
  `PrepositionPredicate(SelectedPredicate(vote/frame 0),
  PrepositionPhrase(for, NP))`. Each has both NP shapes above, using the
  mass singular nouns Time and Money: four Readings. The second pair treats
  *for time or money* as an Adjunct rather than a selected Complement.
- Monastery Siege: `SelectedPredicate(Choose, Object(
  Coordination<NounPhrase>(ProperNameNounPhrase(CatalogName(Name/Khan)),
  or, BareNounPhrase(Noun(creature_subtype/dragon, Plural)))))`.
- Roots of Life: `SelectedPredicate(Choose, Object(
  Coordination<NounPhrase>(ProperNameNounPhrase(CatalogName(card-names/Island)),
  or, ProperNameNounPhrase(CatalogName(card-names/Swamp)))))`.

OW1/OW2's required alternative is, schematically,
`SelectedPredicate(choice verb, Complement(Coordination<OptionList>(
option-word member, or, option-word member)))` (with *for* in vote's frame).
This is a required prospective structure, not an implemented/probed tree.
For Time/Money the words are already eligible singular Nouns; adding the
option frame and member projection alone cannot retire any of its four
existing trees. OW3 caused the pre-implementation STOP; OW7 resolves that
STOP by returning this ticket for the owner's combined decision, rather
than authorizing a grammar change.

### (a) Covered faces and existing routes

Coverage here means the **whole face** has a Reading in the supplied
`target/english-v3/baseline-wave9.json`, not merely that its option sentence
can be probed. The baseline and the default workspace's
`target/english-v3/blockers-wave9.json` both actually stamp `mqrlzszkuszn`
(the filenames and the earlier ticket's `qzprwuxz` description are not the
stamp). All new corpus counts below use those supplied files; additional
route probes use claim parent `pxxrtuss` via working change `nomvqwun`.

| Route for an option list | Covered faces | Examples / scope |
| --- | ---: | --- |
| Object of *choose*, NP Coordination containing a BareNounPhrase | 5 | Monastery Siege, Citadel Siege, Frontier Siege; also Palace Siege and Outpost Siege. All five coordinate the Catalog proper name *Khans* with the plural bare NP *Dragons*. |
| Selected Complement of *vote for*, bare-NP option Coordination | 0 | No covered-face examples exist. |
| PP Adjunct of *vote*, bare-NP option Coordination inside *for* | 0 | No covered-face examples exist. |

If "bare-NP coordination" requires **every** member to be bare, the first
row is also **0**: the five Sieges are mixed proper-name/bare-NP
Coordination, not two bare NPs. Six further covered option-list faces are
pure proper-name NP Coordination (five clan-pair Sieges and Roots of Life),
so are not counted in the bare-NP row. The remaining 29 covered matches of
the ticket's original grep are target descriptions or *among* target
quantities, not bare option words.

Three covered examples cannot be supplied for either zero vote row. The
attested contrast examples for **both** vote routes are Custodi Squire
(*an artifact, creature, or enchantment card in your graveyard*), Council's
Judgment (*a nonland permanent you don't control*) and Trap the Trespassers
(*a creature you don't control*): these are ordinary determined NP
Complements, not option words, and their whole faces are unread in the
supplied baseline. Their isolated clauses admit both the selected
Complement and PP Adjunct routes. They are controls to preserve, not
covered option-pair examples. Custodi Squire's probe inspected eight
Readings without exhausting enumeration; that cap is not a total count.
Council's Judgment and Trap the Trespassers exhausted enumeration at four
and two respectively.

For the actual option wording, Expropriate's isolated *Each player votes
for time or money.* has **4** Readings (two selected and two PP Adjunct),
but the whole face is unread: its blocker is later `at:one .`, not the
option pair. Travel Through Caradhras's isolated *Each player votes for
Redhorn Pass or Mines of Moria.* has **2** Readings (one selected and one PP
Adjunct), both proper-name NP Coordination, not bare NPs; its whole face
is also unread. *Choose heads or tails.* has the two recorded probe trees,
but `cargo xtask english-v3 grep -i '\bheads or tails\b'` finds **0**
supported faces. It is a diagnostic supplied by the ticket, not a corpus
face that can be offered as an attested positive witness.

### (b) Unread faces by member type

These are **71 distinct unread faces**, including an unread face whose
option clause already reads (Expropriate). They are not a prediction of
71 coverage gains. Count a face once; an undeclared member takes precedence,
then a plural/Catalog-only member, then another declared lexical Category,
then an all-Noun-headed list. The two expanded Noun-headed lists below are
explicitly distinguished from single Lexemes.

| Member type | Unread faces | Examples / breakdown |
| --- | ---: | --- |
| Declared Noun heads | 21 | Pir's Whim (*friend/foe*), Turnabout (card types), Fatespinner (declared multiword turn-part nouns). Includes Expropriate, and two expanded nominal lists: Teferi's Realm (*non-Aura enchantment*) and Sarpadian Empires, Vol. VII (*white Citizen*, etc.). The latter two are not bare single-Lexeme members. |
| Adjective / other declared Category | 22 | Odd/even 8; left/right 5; Keyword/state lists 4; Color Words 2; Arabic Numeral lists 3. |
| Plural / Catalog form | 2 | Capital Punishment (*taxes*: Tax/Plural); Travel Through Caradhras (*Redhorn Pass*: Name/Catalog, *Mines of Moria*: card-name Catalog). |
| At least one undeclared label member | 26 | Magister of Worth (*grace/condemnation*), Archangel of Strife (*war/peace*), Prisoner's Dilemma (*silence/snitch*). Includes Master of Ceremonies (*secrets* undeclared), despite its other declared/plural members. |

The "other declared" row does **not** claim the printed form is a licensed
citation-form mention. In particular *left* currently analyses only as the
preterite/past participle of **Leave**; *right* is a count Noun. There is no
declared Adjective analysis of *left* in the claim-parent probe. This
matters to both candidate designs: recognition of a spelling by some
Lexeme is not recognition of the intended option label or its citation
form. Likewise a Catalog analysis of a label is not proof that it denotes
the corresponding card. Khans/Dragons are in the covered mixed-route row,
not in the unread plural/Catalog count.

Reproduction uses only the supported, reminder-stripped text through
`cargo xtask english-v3 grep`, with `--json` exports under
`target/english-v3/english-v3-option-word-choice-list/`:

- The Method's original regex gives **116 faces / 117 matches**;
  `--census target/english-v3/baseline-wave9.json --unread` gives **76
  faces / 77 matches**, and `--covered` gives **40 / 40**. Filtering the
  inspected matches to untargeted option words leaves **61 unread faces**.
  Target/among descriptions, modal *one or more/both* and permanently
  excluded wording are not counted as this construction's option members.
- `\b(chooses?|votes? for) [0-9]+(, [0-9]+)*,? or [0-9]+\b` adds **3**
  disjoint unread faces: Tibalt's Trickery, Haktos the Unscarred and
  Expert-Level Safe.
- A broader attestation search,
  `\b(chooses?|votes? for)\b[^.\n:]*\bor\b`, followed by inspection,
  adds **7** disjoint unread faces missed by the original sentence-final
  pattern: Storage Matrix, Travel Through Caradhras, Truth or Consequences,
  Elrond of the White Council, Blitzwing's Adaptive Assailant face,
  Abundance and Prisoner's Dilemma. The exact confirming grep is
  `\b(chooses?|votes? for) (artifact, creature, or land during|Redhorn Pass or Mines of Moria|truth or consequences|fellowship or aid|flying or indestructible at random|land or nonland and|silence or snitch)\b`;
  `--census ... --unread --count` gives **7 / 7**.

Unknown-member classification uses only unknown-word spans **inside the
matched option list**, from the supplied blockers file. A later unknown
word such as *tied* or *proceeding* does not turn a declared option member
into an undeclared label. The expanded selector is why these counts do not
repeat the historical hand-cut estimate of 39 declared / 28 undeclared
sole-line faces. Quoted options and parameterized keyword members such as
*rampage 3* are separate shapes, not silently included as bare Lexemes.
The identity/type accounting is preserved in `owner-member-types.json`.

### (c) Candidate designs and Construction cost

**A — Option List as a mention Category.** Option members retain their
lexical identities and printed Word Forms but function as mentioned labels,
not referential NPs. The declared choice class supplies the selecting frame
for both *choose* and *vote for*. Reuse the existing pair and comma-series
Coordination schemas with a single member projection. Initial-list budget:
**1 new member Construction**, **0 new coordination families**; category
and schema instances are counted separately. A declaration-scoped open
label source can feed the same member projection without another
Construction, but its source, boundaries, identity and scope need the
owner's decision; unbounded unknown-word capture is not a proposed shortcut.

A would replace both old nominal/NP coordination shapes **locally under
the choice frames**, preserve normal NP Objects such as *a color* and
*target artifact or enchantment*, and replace the wrong Catalog-name
analyses of labels if those labels are admitted. The PP-Adjunct route around
intransitive *vote* is an additional bypass: leaving it available for the
same labels would retain the twin route. The owner must decide its local
exclusion while preserving ordinary referential PP controls and genuine
independent ambiguity under W9-A. Plural and Catalog-shaped labels need an
explicit member policy; "citation forms only" cannot automatically admit
*heads*, *Dragons* or *left*. No global bare-NP or *for*-Adjunct removal is
proposed. The two expanded nominal lists require a separate decision on
whether a label can contain internal structure.

**B — Keep ordinary bare NPs; declare mention/nominalisation.** Retain the
existing NP and Nominal Coordination and the choice verbs' NP frames.
Declare a class-level nominal use of citation forms for eligible non-noun
Lexemes, without inventing duplicate Noun Lexemes or pretending an Adjective
changed its lexical identity. A single nominal projection can cover the
class: budget **1 new nominalisation Construction**, or **0** if the
existing `Noun` projection is refined to express that distinction;
**0 new coordination families**. The zero-Construction alternative still
changes an existing Reading declaration and its census/test contract; it
is not a data-only promise.

B keeps the two heads/tails NP shapes and the four time/money Readings.
The nominalisation must not create a second nominal route for nouns that
already have a bare use. Singular count-noun option words also need a
class-level mention licence: adding a route only for non-nouns would still
leave *friend or foe* and card types unread. A retained licence must have
an explicit distribution, so it does not automatically make a mentioned
adjective into a referential Object of every verb. Catalog-label Readings
known to be wrong still need replacement with declared label use; keeping
bare NPs does not validate Name/Khan or card-name identities. Undeclared
labels still need the same scoped lexical-source decision as A; B does not
solve them by minting a Noun per spelling.

These are initial-list Construction budgets, not implementation promises
for the whole label system. Both reuse the existing selected-predicate
machinery and must preserve byte-exact realization and independent
construction/parse roundtripping. A additionally needs a label projection
into nominal-modifier function for later *death vote* shapes: **at most
one further shared projection** if the existing Premodifier schema cannot
express it. B can reuse nominal Premodifier composition only if its declared
nominalisation includes that function; ordinary nominal membership does
not imply the licence. The owner should settle these distributions before
any Construction or lexical Category is added.

### (d) Later mentions governed by the same decision

`cargo xtask english-v3 grep -i '\bfor each [a-z -]+ vote\b|\b(each player|players) who chose [a-z -]+\b|\bwho chose (war|peace|money|friends|secrets|fame|fortune)\b'`
finds **15 faces / 27 matches** (includes Wheel of Misfortune's ordinary
*that number* control). Relevant attested shapes include:

- Capital Punishment: *for each death vote* and *for each taxes vote*;
  Expropriate: *For each time vote* / *For each money vote*. The label
  premodifies the Noun *vote*; no choice verb is present to license it.
- Travel Through Caradhras: *For each Redhorn Pass vote* / *For each Mines
  of Moria vote*. These preserve multiword labels and cannot be justified
  by interpreting the labels as references to the card-name Catalog.
- Archangel of Strife: *Creatures controlled by players who chose war* /
  *players who chose peace*. Master of Ceremonies: *For each player who
  chose money*, *friends*, or *secrets*. The label is selected in a Relative
  Clause and is no longer a coordinate of an option pair. The requested
  *each player who chose war* is a schematic combination of these attested
  shapes, not an exact quote from Archangel of Strife.
- Council text also uses *If grace gets more votes*, *for each sprout vote*,
  *the number of truth votes*, and *If each opponent chose silence*.
  Initial choice, later Complement, noun Premodifier and Subject-label
  distribution must refer to the same scoped label identity.

A therefore cannot restrict the entire mention system to two-or-more-member
lists: later single labels and noun Premodifiers must share it. B must
carry the same identity and distribution through its nominalisation.
FlavorWord shows that the lexical environment has an open label source,
but its current italic ability-head role is not authority for any of these
new roles. Whether to use a scoped label source or explicit per-face
label declarations, and whether noun-like labels keep ordinary nominal
ambiguity, remains the one owner decision under OW7.

No implementation, census rerun, tests, clippy, grammar refresh or integration
was performed for these findings. Only this ticket is changed; the source
probes, grep exports and member-type accounting remain untracked evidence,
preserved for return under the default workspace's
`target/english-v3/english-v3-option-word-choice-list/owner-findings-nomvqwun/`.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Extinction Event: "Choose odd or even."
- Turnabout: "Choose artifact, creature, or land."
- Pir's Whim: "For each player, choose friend or foe."
- Order of Succession: "Choose left or right."
- Angelic Skirmisher: "At the beginning of each combat, choose first strike,
  vigilance, or lifelink."

Also 0: "Each player votes for death or taxes." (Capital Punishment; all words
declared); "Choose 1, 2, or 3 at random." Controls that must not change
except under OW3: "Choose heads or tails." 2; "Choose one or both." 1; "Choose
a color." 1; "Choose a creature type." 1; "Each player votes for a creature."
2; "Each player votes." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(choose|chooses|chose|chosen|vote|votes|voted)\b` plus
300 random covered controls (`--random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; OW6 for changes. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings` (and `-p deckmaste_lexical_source` if the lexicon changes) once, at
the end: a rebuild after a grammar edit takes 2–3 minutes and the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
role (each option a coordinate of the list, the list the verb's Complement,
the OW3 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- OW5's undeclared labels (28 sole faces). *votes for planeswalk or chaos*
  (5 faces, Planechase wording): listed, never ticketed.
- *Choose a creature type other than Wall.* / *a card type other than
  creature* (6 sole faces): a bare type word after *other than*; related
  mention use, separate shape.
- *Choose a number between 0 and 10.* (4 faces); *your choice of 5/1 or 1/5*.
- Quoted options (Raging River, *choose "left" or "right."*).

## Landing record
