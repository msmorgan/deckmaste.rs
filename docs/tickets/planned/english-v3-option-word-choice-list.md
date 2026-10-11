---
needs: []
---
# Read option words as mentions after a choice verb: Choose odd or even.

## Why

A choice among named options is written as a bare coordination of the option
words: *Choose odd or even.*, *Choose artifact, creature, or land.*, *choose
friend or foe*, *Choose left or right.*, *choose first strike, vigilance, or
lifelink*, *votes for death or taxes*, *choose Khans or Dragons*. The words
are mentioned, not used (*odd* is predicated of nothing; *creature* has no
Determiner although it is a singular count noun), and today a list reads only
when every member happens to be a bare-capable noun or a Catalog name.

Counts from the findings below (supplied `baseline-wave9.json`, stamp
`mqrlzszkuszn`): **71** distinct unread faces carry an option list: **21**
whose members are all declared Noun heads, **22** with a member of another
declared Category (odd/even 8, left/right 5, Keyword/state lists 4, Color
Words 2, Arabic Numeral lists 3), **2** with a plural or Catalog-shaped
member, and **26** with at least one undeclared label. **5** covered faces
(the Khans/Dragons Sieges) read an option pair through a mixed proper-name /
bare-NP Coordination. These are faces, not predicted gains: Expropriate's
option clause already reads and its face is held elsewhere. The five *votes
for planeswalk or chaos* faces were excluded from the 71 as out-of-scope
wording; the scope ruling of 2026-10-11 (`english-v3-planechase-conspiracy-wording`)
brings them back as card-local labels, so the label count is 26 + 5.

## Owner ruling 2026-10-11

Verbatim: "own mention category probably. Khans/Dragons is another of these.
these should be declared as macros somehow. odd or even may be slightly
different since they're existing words."

This resolves OW7's hold and the STOP under OW3. It decides design **A** of
the findings; design B (nominalisation under ordinary bare NPs) is not
pursued.

## Goal

An option word reads as a member of its own mention Category, selected as
the Complement of a choice verb through that verb's declared frame, with
byte-exact realisation: (i) for every option word that is an existing
Lexeme; (ii) for every card-local label a plugin macro definition declares.
The five covered Sieges move to this analysis.

## Analysis

No CGEL passage is relied on; the analysis is a project ruling under the
owner ruling above. Existing routes
(`crates/deckmaste_english_v3/src/declarations.rs`): *choose*
(`core-verb:Choose`) takes an NP Object through `SelectedPredicate`; *vote*
is declared by `plugins_v2/builtin/macros/keyword_actions/vote.ron` with a
`Custom` frame set (`[]`, `[ObjectNounPhrase]`, `[Lex("Preposition", "For"),
ObjectNounPhrase]`); the coordination schemas build `Coordination` over NPs;
`BareNounPhrase` admits plural and mass nouns.

Rulings (orchestrator 2026-10-11, rewritten after the owner ruling):

- OW1 (owner). Option words form their own mention **Category**. A member is
  a mentioned label, not a referential NP: it keeps its lexical identity and
  printed Word Form and takes no Determiner. One route: a coordination (pair
  or comma series) of members, built by the existing coordination schemas
  with one member projection, selected as Complement. Budget: **1** new
  member Construction, **0** new coordination families; say so before
  building, with the faces it buys.
- OW2 (R-class). The licence is the choice verbs' declared frame: every verb
  of the choice class (*choose*, *vote for*) declares an option-list frame as
  data, *vote*'s in its plugin definition. No row per attested list, no list
  of option words, no test on *odd*, *friend*, *left*, and no card test.
- OW3 (owner: "existing words"). An existing word used as an option (*odd*,
  *even*, *left*, *right*, *heads*, *tails*, card types, keywords, colour
  words, numerals) is a mention of the existing Lexeme and needs **no new
  declaration**: the member projection reads any declared Lexeme of the
  mentionable lexical Categories. Two consequences to carry, not to guard:
  the findings show *left* analysed only as a form of the verb *leave*, so
  the direction word needs an ordinary vocabulary declaration (a Lexeme in
  its own right, not an option-label declaration); and a printed plural
  (*taxes*, *heads*) is the Word Form mentioned, so no citation-form filter
  is applied unless a probe shows it creates a twin.
- OW4 (owner: "declared as macros"). Card-local labels (*Khans* / *Dragons*,
  *war* / *peace*, the council-vote labels *grace* / *condemnation*,
  *silence* / *snitch*, *planeswalk* / *chaos*) are declared through plugin
  macro definitions. They are never minted as core Nouns and never read as
  Catalog names. **The declaration shape is NOT yet decided.** Two concrete
  shapes are proposed below; the choice between them is **ORCHESTRATOR RULING
  NEEDED BEFORE CLAIM**.
- OW5 (pruning rule). The five covered Sieges that read *Khans or Dragons* as
  `ProperNameNounPhrase(Name/Khan)` *or* `BareNounPhrase(Dragons)` move to
  the mention analysis. The accidental Readings are retired: `Name/Khan` and
  a creature-subtype plural are wrong analyses of a label, not Readings to
  protect. The same applies to the pure proper-name coordinations recorded in
  the findings (clan-pair Sieges, Roots of Life, Travel Through Caradhras)
  when their labels are declared. Under the choice frames the bare-NP
  coordination of the same words does not survive beside the mention
  (R-one); ordinary NP Objects (*a color*, *target artifact or enchantment*)
  and the PP-Adjunct controls keep their Readings. The local exclusion reads
  a declared feature of the frame, never the words.
- OW6. *at random* after the list (3 faces) is the existing Adjunct, in scope
  only if it falls out. A trailing colon (Phyrexian Splicer's cost) is the
  existing cost position.
- OW7. Newly covered faces are gains. Decreases are expected only as OW5's
  retirements and are listed by face; any other change on a covered face is a
  defect.
- OW8 (follow-up, not this landing). Later mentions of the same labels: *for
  each death vote*, *For each Redhorn Pass vote*, *players who chose war*,
  *each player who chose money*, *If grace gets more votes*, *the number of
  truth votes*. Findings (d): 15 faces / 27 matches. They need the label as
  Premodifier of *vote*, as Complement in a Relative Clause and as Subject,
  all referring to the same declared label. Mint the follow-up ticket when
  this lands; the declaration shape chosen under OW4 must not preclude it.

### Proposed declaration shapes for card-local labels (OW4)

How `plugins_v2/builtin/macros/` declares comparable vocabulary today:

- **Ability Words** are a closed inventory, one definition file per word
  under `ability_words/` through the `AbilityWord` meta-macro, e.g.
  `councilsDilemma.ron`: `AbilityWord(name: "councilsDilemma", spelling:
  "Council's dilemma", grammar: FixedTerm(surface: "Council's dilemma"))`.
- **Flavor Words** are an open slot: one definition,
  `abilities/flavorWord.ron`, with a `FlavorWordLabel` parameter; the label is
  the italic run captured verbatim, and nothing enumerates the labels.
- **Anchor words** (the Siege labels as the rules call them) have no
  definition under `plugins_v2/` today.
- **Shortened Names** are scoped to a card: a generated table keyed by the
  full card name (`deckmaste_catalogs`, `card-name-nicknames.json`), so a
  shortened spelling is vocabulary only on that card's own faces.

**Shape 1: one declared definition per label, scoped to the cards that print
it** (Ability Word model for the file, Shortened Name model for the scope).
A new meta-macro; a label is ordinary declared vocabulary on the named cards'
faces and unknown elsewhere. Closed, auditable, one file per label; the
follow-up mentions (OW8) find the same identity. Cost: one small file per
label (two or three per label face, shared labels once), and each new
printing needs a declaration.

```ron
// plugins_v2/builtin/macros/option_labels/khans.ron (sketch)
OptionLabel(
    name: "khans",
    spelling: "Khans",
    grammar: OptionWord(surface: "Khans"),
    scope: Cards(["Monastery Siege", "Citadel Siege", "Frontier Siege",
                  "Palace Siege", "Outpost Siege"]),
)
```

**Shape 2: one open-slot definition; the choice sentence introduces its own
labels** (Flavor Word model). A single definition with a label parameter; on
a face, the members printed in the option position of a choice frame become
that face's labels, and a later mention must match one of them. No per-label
file and no card list. Cost: the label source is the face's own text, so the
introducing position must be bounded by a declared feature of the frame
(unbounded unknown-word capture is not acceptable, per the findings), and
multiword labels (*Redhorn Pass*, *Mines of Moria*) need a declared extent.

```ron
// plugins_v2/builtin/macros/abilities/optionLabel.ron (sketch)
(
    name: "optionLabel",
    kinds: [OptionWord],
    params: {"label": OptionLabelText},
    grammar: OptionWord(source: IntroducedBy(ChoiceFrame), scope: Face),
)
```

Both sketches are illustrations: neither meta-macro, `OptionWord` row nor
scope field exists. Either shape is a plugin-schema change (bodiless,
grammar-only definitions are allowed), so the Method's plugin checks apply.

## Findings (2026-10-11, recorded before the owner ruling)

Kept as evidence. Where a paragraph below says the owner must still decide
between A and B, the owner ruling above has decided it (A); the label
declaration shape (OW4) is the one question still open, and it is the
orchestrator's.
OW numbers inside the findings are the pre-ruling numbering: old OW3
is the twin STOP (now OW5), old OW5 the undeclared labels (now OW4), old OW7
the hold for the owner (resolved).

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
declared); "Choose 1, 2, or 3 at random." Label witnesses once OW4 is ruled:
Archangel of Strife (*war or peace*), Magister of Worth (*grace or
condemnation*). Moved under OW5 (1 → 1, analysis changes): Monastery Siege "As
this enchantment enters, choose Khans or Dragons." Controls that must not
change: "Choose one or both." 1; "Choose a color." 1; "Choose a creature
type." 1; "Each player votes for a creature." 2; "Each player votes." 1.
"Choose heads or tails." (2; 0 supported faces) and "Each player votes for
time or money." (4) are expected to change under OW5 and are reported with
their trees before and after.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(choose|chooses|chose|chosen|vote|votes|voted)\b` plus
300 random covered controls (`--random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; OW7 for changes. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings` (and `-p deckmaste_lexical_source` if the lexicon changes). Because
`plugins_v2/` changes (*vote*'s frame; the OW4 definitions): the FULL `cargo
test -p xtask`, `-p deckmaste_semantics_v2`, `-p deckmaste_construction_core`,
`cargo xtask lean check`, and a facts registration or exemption for each new
bodiless definition. No existing keyword's declared signature and no v1 /
Idris facts row changes. Tests assert the role (each option a member of the
list, the list the verb's Complement, the OW5 retirements as negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only. Glossary: the option-word mention Category and the
card-local label term are added to `docs/contexts/oracle-english/CONTEXT.md`
by this landing.

## Out of scope

- Later mentions (OW8): follow-up ticket.
- *Choose a creature type other than Wall.* / *a card type other than
  creature* (6 sole faces): a bare type word after *other than*; related
  mention use, separate shape.
- *Choose a number between 0 and 10.* (4 faces); *your choice of 5/1 or 1/5*.
- Quoted options (Raging River, *choose "left" or "right."*).
- The two expanded nominal lists (Teferi's Realm, Sarpadian Empires, Vol.
  VII): whether a label has internal structure is not decided.

## Landing record
