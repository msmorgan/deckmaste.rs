# Oracle Text Style Guide

This document is an attempt to faithfully reverse-engineer the style guide used
internally at Wizards of the Coast to author and edit Magic card Oracle text.

Provenance: synthesized from the Vintage card corpus and the
Comprehensive Rules via the mtg-rules skill, with assistance from that skill's
preexisting CR/corpus-derived grammar reference. It is independent of this
repository's implementation and project documentation (transcript-audited: no
code, design docs, or tickets were read; project instructions contributed the
citation format). The card corpus is authoritative on conflict: amend this
guide only with corpus evidence, such as a round-trip failure. Corpus evidence
is read through the project's input normalization (canonical quotes and range
dashes): a glyph the data channels transcode differently is judged by its
canonical, print-verified form, not by any one export's bytes.

## Scope and authority

This manual governs English Oracle rules text. Use it to write and edit card
text after the design's rules intent has been settled. It does not determine
card balance or repair an effect that cannot function under the rules. Before
copyediting, confirm the intended objects, zones, choices, targets, timing,
duration, and event sequence.

Oracle text includes operative rules text and reminder text. Flavor text,
artist credits, collector information, and printing history are outside this
manual's scope. Type-line style is addressed only where type-line terms are
used in rules text.

The Comprehensive Rules govern rules meaning. This manual governs the English
surface form used to express that meaning. Rules-defined keyword, symbol, and
frame text is fixed: use its approved Oracle form exactly, including spelling,
capitalization, punctuation, parameters, and reminder text. Do not reconstruct
fixed text from a nickname or a summary of its rules meaning.

In examples, bracketed terms such as `[cost]` and `[effect]` are drafting
metavariables, not literal Oracle punctuation. Use literal square brackets only
when a rules-defined template calls for them.

## Contents

- [The shortest useful checklist](#the-shortest-useful-checklist)
- [Part I. Editorial foundations](#part-i-editorial-foundations)
  - [1. Rules prose and paragraphs](#1-write-rules-instructions-not-conversational-prose)
  - [2. Capitalization](#2-capitalization)
  - [3. Punctuation and glyphs](#3-punctuation-and-glyphs)
  - [4. Numbers and quantities](#4-numbers-quantities-and-comparisons)
- [Part II. Names, objects, and type grammar](#part-ii-names-objects-and-type-grammar)
  - [5. Names and anaphora](#5-names-self-reference-pronouns-and-anaphora)
  - [6. Objects, players, and targets](#6-describing-objects-players-and-targets)
  - [7. Types, subtypes, and supertypes](#7-types-subtypes-and-supertypes-as-nouns-and-modifiers)
- [Part III. Ability and spell construction](#part-iii-ability-and-spell-construction)
  - [8. Ability architecture](#8-ability-architecture)
  - [9. Costs and casting](#9-spell-costs-casting-permissions-and-restrictions)
  - [10. Logic, choice, and coordination](#10-logic-choice-and-coordination)
  - [11. Timing and duration](#11-timing-and-duration)
- [Part IV. Standard effect templates](#part-iv-standard-effect-templates)
  - [12. Standard effect families](#12-standard-effect-families)
    - [Cards and zones](#cards-and-zones)
      - [Draw, discard, and hand size](#draw-discard-and-hand-size)
      - [Look, reveal, and the top of a library](#look-reveal-and-the-top-of-a-library)
      - [Search and shuffle](#search-and-shuffle)
      - [Moving cards and permanents](#moving-cards-and-permanents)
      - [Exile and permissions tied to exile](#exile-and-permissions-tied-to-exile)
    - [Objects and characteristics](#objects-and-characteristics)
      - [Tokens](#tokens)
      - [Copies](#copies)
      - [Counters](#counters)
      - [Power and toughness](#power-and-toughness)
      - [Granting and removing abilities](#granting-and-removing-abilities)
      - [Colors and types](#colors-and-types)
    - [Combat and resources](#combat-and-resources)
      - [Combat](#combat)
      - [Damage](#damage)
      - [Life](#life)
      - [Mana](#mana)
    - [Spells and game actions](#spells-and-game-actions)
      - [Cast, play, counter, and copy](#cast-play-counter-and-copy)
      - [Control, exchange, attach, and unattach](#control-exchange-attach-and-unattach)
      - [Sacrifice, destroy, exile, and regenerate](#sacrifice-destroy-exile-and-regenerate)
      - [Randomness, dice, coins, and voting](#randomness-dice-coins-and-voting)
  - [13. Continuous, replacement, and prevention templates](#13-continuous-replacement-and-prevention-templates)
- [Part V. Keywords, frames, and final review](#part-v-keywords-frames-and-final-review)
  - [14. Keywords, labels, and reminder text](#14-keywords-labels-and-reminder-text)
  - [15. Frame-dependent text](#15-frame-dependent-text)
  - [16. Final editorial review](#16-final-editorial-review)

## The shortest useful checklist

Before releasing new rules text, check all of the following:

1. Give each ordinary ability its own paragraph. Keep a modal header and all of
   its bullets together.
2. Write spell abilities and one-shot instructions as imperatives; write static
   abilities as present-tense declarations.
3. Write an activated ability as `[Cost]: [Effect.] [Activation instruction.]`.
4. Begin each ordinary standalone triggered ability with **When**, **Whenever**,
   or **At**, then put a comma between its event and effect.
5. Put `target` before the complete description of every choice that is meant
   to be a target. Use `up to one target` if zero must be legal.
6. Use `card` for an object in a nonbattlefield zone, `spell` for one on the
   stack, and a bare type word for a permanent on the battlefield.
7. Lowercase card types, supertypes, colors, zones, counters, keyword abilities,
   and keyword actions in running text. Capitalize subtypes and exact names.
8. Spell out fixed counts of discrete things, but use digits for life, damage,
   power, toughness, mana value, and keyword parameters whose definitions use
   numerals.
9. Use the serial comma in a coordinated list of three or more items.
10. Use `then` for sequence, `if you do` for dependency, and `When you do` only
    when a reflexive trigger is intended.
11. Use `gets +N/+M` for a power/toughness modifier, `gains` for an ability
    granted by an effect, and `has` for a continuously true ability.
12. End ordinary rules sentences and modal bullets with periods. Do not add a
    period to a bare keyword line, an `Enchant`/`Equip` line, or a structural
    header.

## Part I. Editorial foundations

### 1. Write rules instructions, not conversational prose

#### Voice and tense

Use the simple present. Oracle text describes what is true, what happens, or
what a player does. Do not use *will* unless it is part of a fixed template.

- Spell or resolving-ability instruction: `Draw two cards.`
- Static declaration: `Creatures you control have flying.`
- Trigger: `Whenever you cast a noncreature spell, draw a card.`
- Restriction: `This creature can't block.`
- Permission: `You may play lands from your graveyard.`

Use active voice when the actor matters: `Target player discards a card.` Use
passive voice when the affected object or the event is the useful subject:
`Whenever this creature is dealt damage, ...` and `if it was cast from your
hand`. Do not replace established game verbs with synonyms: cards are *cast*,
abilities are *activated* or *trigger*, permanents *enter*, creatures *die*,
and tokens are *created*.

Use contractions in ordinary rules text: `can't`, `don't`, `doesn't`, `isn't`,
`weren't`, `you've`. Use singular `they`, `them`, and `their` for a player of
unknown gender; do not use `he or she` or `his or her`.

#### Paragraphs and ability boundaries

Except for the structural forms listed below, give each ability its own
paragraph. A paragraph may contain several sentences when they are instructions
or consequences within that ability. A paragraph break creates another ability
[CR#113.2c].

Use one paragraph for this sequence:

> When this creature enters, draw a card. Then discard a card.

Use two paragraphs if the statements are independently functioning abilities:

> Flying  
> Whenever this creature attacks, draw a card.

The important exceptions are structural:

- several bare keyword abilities may share one line;
- a `Choose ... —` header and its bullet lines are one modal ability;
- Saga chapters, Class bars, leveler bands, Case labels, die-roll results,
  Station thresholds, and `∞`-labeled lines create frame-defined groupings; and
- reminder text may occupy its own line without becoming an ability.

Separate paragraphs with one line break. Do not insert an empty line.

#### Sentence scope

Prefer short sentences when a pronoun or modifier could attach to the wrong
noun. A period does not necessarily end an ability; use it to make references
clear:

> Exile target creature. Its controller creates a 1/1 white Spirit creature
> token with flying.

Use a single sentence when coordination is itself the template:

> Tap target creature, then put a stun counter on it.

Do not use prose headings, colons, or semicolons merely for visual variety.
Oracle punctuation carries stable structural signals described below.

### 2. Capitalization

#### General rule

Capitalize the first word of a sentence, bullet, ability paragraph, or labeled
mode. Otherwise, capitalize only exact names, proper nouns within exact names,
subtypes, and rules-defined designations with prescribed casing.

This produces contrasts such as:

- `Artifact creatures you control ...` at the start of a sentence, but
  `target artifact creature` in the middle;
- `Goblin` and `Merfolk`, because they are subtypes;
- `legendary`, `snow`, `basic`, and `world`, because supertypes are lowercase
  in running text;
- `white`, `blue`, `black`, `red`, `green`, `colorless`, and `multicolored`;
- `battlefield`, `graveyard`, `hand`, `library`, `stack`, `exile`, and
  `command zone`; and
- `+1/+1 counter`, `stun counter`, `lore counter`, and `loyalty counter`.

When all five colors are listed, use color-wheel order: `white, blue, black,
red, and green` (or final `or` as semantics require). Put `colorless` after the
five colors when it participates in the same catalog.

#### Category-specific capitalization

Apply capitalization at the point where each formal category is defined:

- for card types, supertypes, and subtypes, see [Types, subtypes, and
  supertypes](#7-types-subtypes-and-supertypes-as-nouns-and-modifiers); and
- for keyword abilities, keyword actions, ability words, flavor words, and
  rules-defined designations, see [Keywords and labels](#14-keywords-labels-and-reminder-text).

### 3. Punctuation and glyphs

#### Periods

End every complete rules sentence with a period, including every modal bullet.
A final period appears inside a quoted granted ability:

> This token has "When this token dies, draw a card."

Do not append a period to these stand-alone forms unless their fixed template
contains one:

- a bare or mana-parameter keyword line: `Flying`, `Ward {2}`, `Equip {3}`;
- an enchant restriction: `Enchant creature you control`;
- a modal header: `Choose one —`;
- a Class or leveler header;
- a power/toughness line in a leveler band; or
- a result or threshold label before `|`.

If an approved keyword template punctuates a verbal cost as a sentence, retain
that punctuation. Do not extrapolate the period to a keyword form such as `Ward
{2}`.

#### Commas and the serial comma

Use the Oxford (serial) comma before the final coordinator in a list of three
or more. This applies to nouns, verb phrases, characteristics, and keyword
abilities in prose:

> Search your library for an artifact, a creature, or a land card.  
> It gains flying, vigilance, and lifelink until end of turn.  
> Exile it, then return it to the battlefield under its owner's control.

There is no Oxford-comma question in a bare keyword line because that line
omits a conjunction entirely: `Flying, vigilance, lifelink`.

Do not place a comma between two coordinated items unless normal clause
grammar requires it: `artifact or enchantment`, `draw a card and lose 1 life`.
Use a comma before `and` or `but` when it joins independent clauses or when the
comma is needed to keep a long series legible.

Other standard comma positions include:

- after a trigger condition: `Whenever this creature attacks, draw a card.`;
- after an introductory condition: `If you do, draw a card.`;
- around a trailing variable definition: `..., where X is ...`;
- between cost components: `{2}, {T}, Sacrifice a creature:`; and
- before `then` within a sequence: `exile it, then return it`.

#### Colons

A top-level colon separates an activation cost from the ability's effect. Use
no space before it and one space after it:

> {2}, {T}: Draw a card.  
> Sacrifice another creature: You gain 2 life.

Colons inside quoted abilities or reminder text have the same meaning. Do not
use a colon to introduce modes, Saga chapters, flavor words, definitions, or
ordinary lists; those structures use an em dash, a vertical bar, or a complete
sentence.

#### Em dashes

Use U+2014 EM DASH, not two hyphens. Oracle has two visibly different dash
patterns.

Use spaces on both sides for labels and structural separators:

> Landfall — Whenever ...  
> Choose one —  
> III — Return target creature card ...  
> ∞ — At the beginning ...

Use no spaces around the dash where an approved keyword template joins the
keyword to a nonstandard or composite cost:

> Ward—Discard a card.

For keyword forms that take a mana-only cost separated by a space, use no dash:
`Kicker {1}{G}` and `Ward {2}`. Use the exact punctuation prescribed for each
keyword and parameter form.

#### Semicolons and expressive punctuation

Do not use a semicolon as a general connector between rules clauses. A
semicolon may occur in fixed keyword or reminder text; reproduce that text
verbatim. In ordinary functional prose, use a period, a comma plus a
coordinator, or a comma plus `then`.

Ordinary functional prose does not use expressive terminal punctuation. Use a
question mark, exclamation point, or ellipsis only when it is part of an exact
name, prescribed keyword spelling, flavor word, or fixed reminder. Do not use
an ellipsis in ordinary rules instructions.

#### Hyphens, minus signs, plus signs, and slashes

Use the ASCII hyphen-minus in ordinary compounds and power/toughness
reductions: `face-down card`, `two-sided`, `-1/-1`. Use U+2212 MINUS SIGN in a
negative loyalty symbol: `[−2]`, not `[-2]`. Positive loyalty symbols use `+`
and neutral symbols use `[0]`.

Write power/toughness pairs and changes with no spaces: `3/3`, `+2/+0`,
`-X/-X`. Use `and/or` only when either or both categories qualify; put no
spaces around the slash. Hybrid and Phyrexian mana slashes belong inside one
symbol token: `{W/U}`, `{2/B}`, `{U/P}`, `{G/U/P}`.

Use a plus after a number for an open-ended structural threshold: `LEVEL 4+`
or `12+ |`. Do not substitute `N or more` inside those headers.

#### Parentheses

Parentheses mark reminder text, which is italicized in print and has no game
function [CR#207.2,207.2a]. Put one space between an ability and its inline
reminder text. Put sentence punctuation inside the closing parenthesis:

> Trample (This creature can deal excess combat damage to the player or
> planeswalker it's attacking.)

A parenthetical may occupy its own paragraph when it explains a frame or
intrinsic characteristic, as on Sagas, Rooms, Sieges, legendary instants and
sorceries, or cards with unusual mana symbols. Do not put operative choices,
targets, costs, durations, or exceptions only in reminder text.

Do not nest parentheses. Write reminder text as a complete sentence with its
period inside `)` unless the approved reminder is a short inline gloss. For a
gloss, give its punctuation to the outer sentence:

> When this creature enters, you get {E}{E} (two energy counters).

#### Quotation marks and apostrophes

Use straight double quotation marks and straight apostrophes in Oracle source
copy.
Quote an entire granted ability, with its terminal period inside the quotation
marks:

> Equipped creature has "Whenever this creature attacks, draw a card."

Do not put card names, token names, colors, creature types, mode labels, or
named counters in quotation marks. Quotes are appropriate when rules text
mentions literal wording, as in `effects that say "destroy"`.

If quoted rules text itself quotes an ability, use straight single quotation
marks for the inner quotation:

> "Creatures you control have '{T}: Add {R}, {G}, or {W}.'"

Put a comma or period inside a closing quotation mark when it punctuates the
quoted wording. Omit it when the quoted material is only a term, as in
`replace "white" with "blue"`.

Use ordinary possessives: `player's`, `opponent's`, `owner's`, `players'`, and
`owners'`. `Its` is the possessive pronoun; `it's` means *it is* or *it has*.
Preserve apostrophes that are part of an exact name.

#### Bullets, vertical bars, and square brackets

Use U+2022 BULLET followed by one space for an ordinary mode. Every ordinary
modal bullet begins with a capital and ends with a period.

Use ` | `, one space on each side of the vertical bar, after a d20 result or a
Station threshold:

> 1–9 | Draw a card.  
> 10–20 | Draw two cards.  
> 8+ | Flying, trample

Use an unspaced U+2013 EN DASH for a closed numeric result range: `1–9`.
Database exports commonly transcode this character as an em dash or an ASCII
hyphen; the character on the printed card is the en dash. Use `N+` for an open
upper range and `N or less` when the lower end is open in ordinary English.

Use literal square brackets only when an approved keyword template marks text
that can be removed. They are not generic optional-text notation:

> Return target nonland permanent [you control] to its owner's hand.

#### Mana and other symbols

Represent each printed symbol as one brace token. Do not insert spaces between
adjacent symbols in a single mana amount:

> {2}{W}{U}  
> {T}: Add {G}.  
> Pay {E}{E}.  
> {Q}: Untap target creature.

Use only rules-defined symbols. They form a closed vocabulary rather than a set
of text abbreviations [CR#107.4,107.4g,107.5,107.6,107.14,107.17,107.18]. Keep
each symbol's internal characters inside one pair of braces; this includes
hybrid and Phyrexian mana symbols.

For one through five energy counters, repeat `{E}` once per counter:
`{E}{E}{E}`. For six or more, spell out the amount before one symbol: `six
{E}`. Use `X {E}` or `any amount of {E}` for a variable amount. `{TK}` is the
ticket-counter symbol. `{P}` is a pawprint weight, not mana or a resource.

#### Infinity notation

Use the literal U+221E INFINITY symbol followed by a spaced em dash:

> ∞ — At the beginning of your end step, exile target nonland permanent.

Do not spell the symbol as `Infinity`, enclose it in braces, or substitute an
ability-word label [CR#702.186a..702.186b].

### 4. Numbers, quantities, and comparisons

#### Fixed counts of things are words

Spell out a fixed number when it counts discrete objects, players, choices,
targets, actions, counters, cards, or turns:

> Draw two cards.  
> Mill three cards.  
> Create thirteen tapped 2/2 black Zombie creature tokens.  
> Put four +1/+1 counters on target creature.  
> Choose one.  
> Up to three target creatures.  
> Take an extra turn after this one.

This convention continues beyond ten. Use `a` or `an`, not `one`, for an
ordinary single object: `Draw a card`, `Create a token`. Use `one` where the
number participates in a choice, contrast, bound, or distribution: `choose
one`, `up to one target`, `one of them`, `one or more`.

#### Scalar values and notation are digits

Use digits for life, damage, power, toughness, mana value, die notation and
result labels, loyalty, and fixed numeric keyword parameters:

> You gain 3 life.  
> This creature deals 2 damage.  
> a creature with power 4 or greater  
> mana value 3 or less  
> Scry 2.  
> Ward {2}  
> Crew 3  
> Toxic 1  
> roll a d20  
> [+1]: ...

The noun being counted controls the choice. Compare `three or more creature
cards` with `a creature card with mana value 3 or greater`.

Spell out fixed hand- and deck-size amounts: `Your maximum hand size is seven`
and `twenty cards more than the minimum deck size`.

In a prose numeral of four or more digits, separate each group of three digits
with a comma: `1,000 or more time counters`, `each opponent loses 1,000 life`,
the ability word `10,000 Needles`. Power/toughness notation takes no
separator: `it gets +9999/+0 until end of turn`, `-0/-9999`.

Use Roman numerals only for Saga chapter symbols. Use Arabic digits for Class
levels, leveler bands, result tables, and Station thresholds.

#### Variables

Use uppercase `X`; use `Y` only when a second independent variable is needed.
Define a text-derived variable at the narrowest unambiguous scope:

> Target creature gets +X/+X until end of turn, where X is the number of
> creatures you control.

Use a separate sentence when the definition governs several later sentences
or cannot read naturally as a trailing clause:

> X is the number of cards in your hand.

All instances of a variable in the same ability should deliberately share a
value. If zero must be prohibited, state it: `X can't be 0.` Do not use an
asterisk as a prose variable; `*` belongs in a characteristic box when a
characteristic-defining ability supplies its value.

#### Number, amount, total, much, and many

Use **number of** for countable things: `the number of creatures`, `the number
of counters`, `the number of times`. Use **amount of** for continuous or scalar
quantities: `the amount of damage`, `the amount of mana`, `the amount of life`.
Use `total power`, `total toughness`, and `total mana value` for sums.

Refer back with `that many` for countable things and `that much` for scalar
quantities:

> Create that many tokens.  
> It deals that much damage.

Use `twice`, `three times`, `double`, or `triple` according to the prescribed
effect-family template. Do not use a multiplication glyph in ordinary rules
prose.

#### Comparisons and rounding

For a fixed scalar threshold, use `N or less`, `N or greater`, `N or more`, or
`N or fewer` according to the noun:

> mana value 3 or less  
> power 4 or greater  
> 5 or less life  
> three or more cards  
> two or fewer creatures

For a variable or expression, use `less than`, `greater than`, or `less than or
equal to`: `mana value less than or equal to X`. Use `the greatest power among
creatures you control` and `the lowest mana value among ...` for extrema. Put a
rounding instruction immediately after the calculation: `half their life,
rounded up`.

## Part II. Names, objects, and type grammar

### 5. Names, self-reference, pronouns, and anaphora

#### Never print drafting shorthand

Do not use `~`, `CARDNAME`, or `[this card]` as a source-name placeholder.
Substitute the actual card or face name, an approved shortened name, or the
correct `this` phrase before treating text as final.

#### Choose the self-reference that matches the object

Use the source's name when the name is the natural grammatical actor on a
spell, when a legendary character is speaking as a named subject, when an
ability functions where a type description could be false, when defining a
characteristic, or when a transform/name-sensitive instruction requires it:

> Lightning Bolt deals 3 damage to any target.  
> Tarmogoyf's power is equal to ...  
> Transform Aang.  
> cards named Seven Dwarves

For a nonlegendary permanent, use `this` plus the relevant kind:

> When this creature enters, ...  
> Sacrifice this artifact.  
> As this Saga enters, ...  
> Return this card from your graveyard to your hand.

Use `this spell` for the spell on the stack, especially in cost, casting,
copying, or countering text. Use `this card` for the physical card in a hand,
library, graveyard, or exile. Use `this permanent` when no narrower permanent
type applies. Use a self-name only for that particular object; use `cards named
...` for objects sharing a name [CR#201.5].

Do not write `this object`. Use the actual role noun: `this card`, `this spell`,
`this permanent`, `this token`, or the relevant type or subtype.

On a multifaced card, use the relevant face name, not the combined `Front //
Back` name. An unambiguous shortened form may replace a long legendary name;
do not invent a shortening that could denote something else.

For an ordinary self-reference to a legendary name containing a comma, use the
distinctive pre-comma portion: `Whenever Zoraline enters or attacks, ...`. Use
the full name when the words identify an actual card name or face—`a card named
...`, partner and meld text, transform text, copy exceptions, and
deck-construction text.

#### Pronouns

Use `it` and `its` for an object after a clear singular antecedent, and `they`,
`them`, and `their` for plural objects or for a player. Personal pronouns may
refer to an established named character (`he`, `him`, `his`, `she`, `her`).
Reserve that treatment for a specific character whose identity supports it;
use object pronouns for generic creatures and permanents.

Repeat a noun or name instead of stacking ambiguous pronouns. After a sentence
names a source, target, controller, and owner, replace an ambiguous `it` with
`that creature`, `that player`, `the exiled card`, or the source's short name.

#### This, that, those, and the chosen

Use `this` for the source or the current temporal context: `this creature`,
`this ability`, `this turn`. Use `that` or `those` for an antecedent established
by the preceding instruction:

> Exile target creature. Return that card to the battlefield ...  
> Tap up to two target creatures. Those creatures don't untap ...

Use `the chosen [quality]` after an explicit choice: `the chosen color`, `the
chosen type`, `the chosen name`. Use `the exiled card` only when exactly one
relevant card was exiled; otherwise use a restrictive phrase such as `a card
exiled with this artifact`.

Use `the rest` only for unselected members of the group established by the
preceding instruction. State their destination:

> Put one of them into your hand and the rest on the bottom of your library in
> a random order.

#### This way, that much, and linked events

Use `this way` to restrict a reference to the immediately described action or
event: `cards exiled this way`, `a creature dealt damage this way`. Do not use
it when several prior actions could supply the antecedent.

Use `if you do` when a later instruction is conditional on actually taking an
optional or attempted action. Use `When you do` when the second part must be a
new reflexive triggered ability; that wording is not stylistic variation
[CR#603.12].

> You may sacrifice another creature. If you do, draw two cards.

> You may sacrifice another creature. When you do, this creature deals damage
> equal to the sacrificed creature's power to any target.

### 6. Describing objects, players, and targets

#### The carrier-word rule

Choose the carrier word according to the object's zone or game role:

| Form | Use for | Example |
|---|---|---|
| bare type/subtype | a permanent on the battlefield | `target artifact`, `Goblins you control` |
| `[quality] card` plus a zone | a card in that zone | `creature card in your graveyard` |
| `[quality] spell` | a spell on the stack | `target instant or sorcery spell` |
| `[quality] source` | a source in any relevant zone | `a red source` |
| `[quality] permanent` | a battlefield permanent, explicitly | `nonland permanent` |

The carrier word determines the rules domain
[CR#109.2,109.2a,109.2b,109.2c,110.1]. Write `creature card in your graveyard`,
not `creature in your graveyard`; write `permanent` on the battlefield, not
`permanent card on the battlefield`.

Use `permanent card` in a nonbattlefield zone when any permanent type should
qualify. Use `nonland permanent` on the battlefield. A land is played, not
cast; do not refer to a `land spell`.

#### Determiners and modifier order

Use this selector order:

> quantity → other/another → target → intrinsic qualities → type phrase →
> controller/owner clause → zone or event clause → numeric qualification

For example:

> up to one other target nonland permanent you control  
> target tapped creature an opponent controls with mana value 3 or less  
> an instant or sorcery card in your graveyard  
> each creature card put into a graveyard this way

Use `another target creature`, never `target another creature`. Put ownership
and control after the full noun phrase: `target artifact you control`, not
`your target artifact`; in operative text, prefer `creatures you control` to
`your creatures`. If independent negative modifiers stack before one head,
separate them with commas: `target nonartifact, nonblack creature`.

This is a clarity rule, not a license to pile up modifiers indefinitely. If a
selector becomes difficult to follow, define a group in one sentence and refer
to `those cards` or `those creatures` in the next.

#### Target

Only `target` marks an explicit target in ordinary Oracle text. Put it directly
before the entire quality phrase:

> target creature  
> target artifact or enchantment  
> target nonland permanent  
> target creature card in a graveyard

Use `target` for a choice made as a spell or ability is put on the stack. Use
`choose`, `each`, `all`, or `of your choice` for a nontarget choice or
instruction performed during resolution [CR#115.1,608.2d].

Use `up to one target` when choosing zero must be legal. Do not use `You may
[verb] target ...` for that purpose: it still requires the target when the spell
or ability is put on the stack, and only the later action is optional
[CR#115.6].

Use `any target` only for the rules-defined damage target class. Do not use it
as a synonym for “any object.” Use `another target creature` to exclude the
source or first referent. Use `up to two target creatures` for a set; use
`target creature and target planeswalker` when two separately described target
roles are intended.

If a verb applies independently to every member of a target set, place `each`
where the distribution is clear:

> Up to two target creatures each get +1/+1 until end of turn.  
> Put a +1/+1 counter on each of up to two target creatures.

#### Each, all, any, and every

Use `each` for a distributive instruction or value: `each opponent loses 2
life`, `for each creature`. Use `all` for a set-level operation or quality:
`destroy all creatures`, `all cards exiled with this artifact`. Use `any
number of` when the player may choose zero or more, up to all available members
of the stated set.

Use `any` without `number of` for unrestricted choice within a stated class
(`mana of any color`, `any target`) and in negative or free-choice
constructions. Do not substitute `every` for an `each` template.

Use `each opponent` rather than `all opponents` for the normal distributive
life, damage, draw, discard, sacrifice, and token instructions. Use `each
player` when the controller is included.

#### Controller and owner

Use `you control` for a battlefield permanent or a spell/ability under the
player's control. Use `you own` when ownership matters across zones. Put the
relative clause immediately after the object phrase:

> a creature you control  
> a creature card you own in exile  
> target permanent an opponent controls

Return a card or permanent `to its owner's hand`. Put a returning permanent
onto the battlefield `under its owner's control` or `under your control` when
control is intentionally specified. For several owners, use `under their
owners' control`.

Use `control` for battlefield permanents, spells, and abilities; use `own`
when legal ownership or identity across zones matters. If both are required,
use `own and control`, in that order. Once an object is established, use
`its controller` or `its owner`; for a long coordinated selector, front the
player noun (`The owner of target spell, nonland permanent, or card in a
graveyard ...`) to avoid a dangling possessive.

Use `its owner's ...` for one object and `their owners' ...` when several
objects can have several owners. Use distributive singular `their owner's ...`
only when each object is explicitly paired with its own single owner.

Use `opponent` when the player must be an opponent and `player` when the
controller may qualify. Use `defending player` only in combat's defined role.
For controlled objects, write `a creature an opponent controls`, not `your
opponent's creature`; possessives remain appropriate for turn parts and
ownership.

### 7. Types, subtypes, and supertypes as nouns and modifiers

#### Closed categories

Use only card types [CR#205.2a], supertypes [CR#205.4a], and subtypes
[CR#205.3g..205.3q] recognized by the rules. Preserve each catalog term's
spelling and category.

Do not coin a new type or subtype as ordinary prose. If a design needs a
flavorful grouping without a rules catalog entry, use an ability word, flavor
word, counter name, token name, or defined quality instead.

#### Capitalization and type-line contrast

In a type line, the displayed words are title-cased by the frame:
`Legendary Artifact Creature — Robot`. In rules text:

- card types are lowercase: `artifact creature`, `instant or sorcery card`;
- supertypes are lowercase: `legendary permanent`, `basic land`, `snow
  source`; and
- subtypes retain capitals: `Robot`, `Aura`, `Forest`, `Jace`, `Time Lord`.

On the type line, supertypes precede card types without a conjunction, and
subtypes follow a spaced em dash: `Legendary Snow Artifact Creature —
Phyrexian Golem`. Preserve every subtype's catalog spelling, including spaces,
hyphens, and apostrophes.

Pluralization does not change capitalization: `Goblins`, `Auras`, `Forests`,
and `Time Lords`. Do not capitalize a batching term merely because its reminder
text expands to capitalized subtypes: `outlaw`, `historic`, `modified`, and
`party` are lowercase.

#### Card types are common nouns and attributive modifiers

A card type can stand alone as a noun:

> Destroy target artifact.  
> Return target creature to its owner's hand.  
> Whenever a land you control enters, ...

It can modify a carrier noun:

> artifact card  
> creature spell  
> permanent card

Do not add `permanent` after a positive permanent type. On the battlefield,
write `artifact`, `creature`, or `land`, not `artifact permanent`, `creature
permanent`, or `land permanent`. `Nonland permanent` is different because the
negative modifier needs a head. Use the rules-defined `permanent card` for a
card in a nonbattlefield zone and `permanent spell` for a spell that will
resolve as a permanent [CR#110.4a..110.4b].

Juxtapose adjacent card types to require their intersection:

> artifact creature  
> enchantment creature card  
> artifact land

Join alternatives with `or`: `artifact or creature card`. Use plural `and`
when naming separate collections: `artifact and creature cards`. These forms
are not stylistic alternatives.

Coordinate alternatives explicitly:

> artifact or enchantment  
> instant or sorcery card  
> artifact, creature, or planeswalker card

A final carrier noun may distribute over the whole coordinated phrase when
the grammar is unambiguous: `an instant or sorcery card`. Repeat the carrier
when the alternatives have different domains or modifiers.

#### Subtypes are capitalized nouns and modifiers

A subtype may be a bare noun:

> Goblins you control  
> sacrifice an Aura  
> search your library for a Forest

It may modify the word that fixes the object's domain:

> Goblin creature  
> Merfolk card  
> Arcane spell  
> Jace planeswalker  
> Saga permanent

Use `Goblins you control` when every permanent with the Goblin subtype should
qualify. Use `Goblin creatures you control` when the object must also have the
creature card type. Kindred noncreatures can have creature types, so add
`card`, `spell`, `creature`, or `permanent` whenever the intended domain is
narrower than the bare subtype supplies.

When spelling out a complete characteristic, put the subtype before the
lowercase type: `Aura enchantment`, `Equipment artifact`, `Forest land`, `Jace
planeswalker`. This running-text order is the reverse of the visual type-line
relationship after the dash.

When several creature subtypes form alternatives, capitalize each and use
ordinary coordination: `an Assassin, Pirate, or Vehicle card`; `Elf or Druid
creature`. When they jointly describe one object's subtype set in a token
description or type line, place them adjacently: `Human Wizard creature token`.

#### Supertypes are lowercase modifiers

Use supertypes as modifiers of a type or carrier noun:

> legendary creature  
> basic land card  
> snow permanent  
> world enchantment

`Legendary` does not name a card type and `legend` is not the corresponding
noun. Use `legendary permanent`, not `legend`; use `legendary cards`, not
`Legend cards`. The batching term `legendaries` is a separate rules-defined
word used in the historic reminder.

The five basic land subtypes are `Plains, Island, Swamp, Mountain, and Forest`.
Write `basic Forest card`, not the redundant `basic Forest land card`.
`Legendary` and `snow` can appear as predicates (`isn't legendary`, `becomes
snow`). Use `basic` as a modifier, not as a standalone predicate.

#### Negation with non-

Attach `non` directly to a lowercase common quality, card type, supertype,
color, or status:

> nonartifact  
> nonblack  
> noncreature  
> nonland  
> nonlegendary  
> nonsnow  
> nontoken  
> noncombat

Preserve a fixed term's approved spelling when it departs from the general
pattern, as in `non-outlaw`.

Use a hyphen before a capitalized subtype:

> non-Aura enchantment  
> non-Human creature  
> non-Merfolk creature  
> non-Saga permanent  
> non-Swamp land

The hyphen follows capitalization, not syllable count. Attach `non` directly to
a lowercase term; use a hyphen before a capitalized subtype. Do not write `non
land`, `non-creature`, or lowercase a subtype to avoid the hyphen.

#### Plurals and agreement

Inflect a subtype as an English count noun while preserving its capital:
`Goblins`, `Elves`, `Dwarves`, `Auras`, `Sagas`. Preserve approved invariant
and irregular forms, including `Merfolk`, `Treefolk`, `Equipment`, `Samurai`,
`Mouse`/`Mice`, and `Ox`/`Oxen`. Artifact subtypes are not uniformly mass
nouns: use `Foods`, `Clues`, and `Treasures`, but never `Equipments`. A plural
form is not a different subtype.

Use singular agreement after `each`: `Each creature gets ...`. Use plural
agreement after a bare plural selector: `Goblins you control get ...`. In a
token description, keep subtype words singular even when creating several
tokens: `Create two 1/1 red Goblin creature tokens`.

Choose `a` or `an` by the pronunciation of the next word, including a prefix:
`an artifact`, `an Aura`, `a creature`, and `a nonartifact permanent`. The final
head carries number when type words are modifiers: `Goblin creatures`,
`artifact cards`, `instant and sorcery spells`.

#### Choosing a type or name

Say what catalog the choice comes from:

> Choose a creature type.  
> Choose a basic land type.  
> Choose a card type.  
> Choose a card name.

Do not say merely `Choose a type`; write `Choose a card type` when that is
intended. One choice of `Merfolk Wizard` is not one creature type; it is two
subtype words. The rules require one existing subtype when an instruction asks
for a subtype [CR#205.3e].

After the choice, filter with `of the chosen type`: `creatures you control of
the chosen type`, `creature cards of the chosen type`, and `spells you cast of
the chosen type`. Use `of that type` for a nearer local antecedent. Reserve `the
chosen creature type` for an instruction that assigns that subtype. Do not
create a `chosen-type` adjective.

For a relationship, use `shares a creature type with`, `shares a card type
with`, or the corresponding approved phrase—not “has the same type as.” To
count categories represented by a collection, use `the number of card types
among ...`, not “the number of different card types.”

#### Type changes and retention

Use `in addition to its other types` when the previous types remain
[CR#205.1b]:

> Target permanent becomes an artifact in addition to its other types.

Write `becomes an artifact creature`, not `becomes an artifact and a creature`.
When a land retains that type, add `It's still a land.` Use each retention
formula only for its prescribed function.

Use `gains all creature types` and `loses all creature types` for additions or
removals. For a static characteristic, use `is every creature type`; the fixed
changeling reminder is `This card is every creature type.` Do not substitute
`has every creature type`.

## Part III. Ability and spell construction

### 8. Ability architecture

#### Spell abilities

Write resolving instructions on an instant or sorcery as imperatives:

> Destroy target artifact.  
> Draw two cards, then discard a card.  
> Target creature gets +3/+3 and gains trample until end of turn.

Use the card's name when the spell itself is a source or actor:

> Shock deals 2 damage to any target.

Put casting permissions, restrictions, additional costs, and alternative costs
before the main resolving instruction. Put each keyword line in the position
prescribed by its approved template.

An instant or sorcery may also have static, activated, or triggered abilities.
Do not force those into imperative spell prose: write a triggered ability in
trigger form and an activated ability in its cost-colon-effect form.

#### Activated abilities

Format every activated ability as:

> [Cost]: [Effect.] [Activation instructions, if any.]

Place every activation cost before the colon and begin the effect after the
colon. Put at the end any instruction governing who may activate the ability,
its activation timing, or its activation frequency [CR#602.1,602.1a].

##### Cost punctuation and order

Separate cost components with comma-space, not `and`. Put no comma immediately
before the colon:

> {2}, {T}, Sacrifice another creature: Draw two cards.  
> {U}, Discard a card: Return this creature to its owner's hand.  
> Pay 3 life: Add {B}{B}{B}.

Order cost components as mana, tap or untap symbol, then other action costs. A
verbal cost component begins with a capital even after a comma: `Sacrifice`,
`Discard`, `Exile`, `Remove`, `Reveal`, `Pay`, `Tap`. Use the exact game action
and state the object or resource being paid.

Do not put `target` in an activation cost. Costs identify what is paid without
targeting (`Sacrifice a creature`, `Discard a card`); every target belongs
after the colon in the effect.

Use `{T}` or `{Q}` alone for the source's tap or untap cost; do not write `{T}
this creature`. To tap a different object as an additional cost, write `Tap
another untapped creature you control`. Use `Sacrifice this artifact`, not
`Sacrifice ~`.

##### Effect

Begin the effect with a capital and write it as an instruction or declaration:

> {T}: Add {G}.  
> {1}: This creature gets +1/+0 until end of turn.  
> {2}, {T}: Target player draws a card.

An activated mana ability still uses the colon. Do not put reminder text or an
ability word between the colon and effect unless the fixed keyword template
does so.

##### Activation instructions

Put every activation instruction after all effect sentences:

> Activate only as a sorcery.  
> Activate only during your turn.  
> Activate only if a creature died this turn.  
> Activate only once each turn.  
> Activate no more than twice each turn.  
> Only an opponent may activate this ability.  
> Any player may activate this ability.

Combine independent restrictions with `and`. Repeat `only` when each
restriction needs independent scope:

> Activate only if you control three or more artifacts and only as a sorcery.

Use `as a sorcery`, not `at sorcery speed`; use `once each turn`, never `once
per turn`. Do not begin with `Use this ability`. If a keyword's reminder text
supplies the restriction, copy that fixed reminder exactly.

An instruction follows the effect even though it governs activation rather
than resolution. That placement is rules-significant [CR#602.1b].

##### Loyalty abilities

Use a bracketed loyalty symbol as the complete first cost component:

> [+1]: Draw a card.  
> [0]: Create a 1/1 white Soldier creature token.  
> [−3]: Destroy target creature.

Use U+2212 for the negative symbol. Do not put a mana symbol, word, or comma
inside the brackets. If an effect changes or copies a loyalty cost, preserve
the bracketed form.

##### Ability words before activated abilities

An ability word may label an activated ability. The em dash precedes the
entire cost-effect structure:

> Channel — {2}{G}, Discard this card: Draw a card.

The colon still marks the activation; the ability word is not part of its cost.

#### Triggered abilities

Use exactly this skeleton:

> [When/Whenever/At] [trigger event], [effect.] [Instructions, if any.]

The initial trigger word and comma define the ordinary surface form
[CR#603.1].

Use **When** for a discrete event that is naturally singular in the stated
context:

> When this creature enters, draw a card.  
> When you do, create a token.

Use **Whenever** for a repeatable event or event class:

> Whenever you cast a noncreature spell, scry 1.  
> Whenever one or more creatures you control attack, ...

Use **At** for a turn-structure point or other stated moment:

> At the beginning of your upkeep, ...  
> At the beginning of the next end step, ...  
> At end of combat, ...

Do not substitute `If`, `During`, `Each time`, or `After` for the trigger word
of an ordinary standalone triggered ability. Those words can appear inside a
trigger condition. Delayed and reflexive triggers can be embedded in resolving
instructions as described below.

##### Enters and dies

Use the shortened battlefield event for routine enters triggers:

> When this creature enters, ...  
> Whenever another artifact enters under your control, ...

Do not write `enters the battlefield` by habit. Retain the full phrase only
when the destination phrase is needed to express a nonstandard relation, such
as `enters the battlefield under your control`.

Use `dies` for a creature moving from the battlefield to a graveyard. Use the
long form when the subject can be a noncreature or when the exact origin and
destination matter:

> Whenever this creature dies, ...  
> Whenever an artifact or creature you control is put into a graveyard from the
> battlefield, ...

##### One or more

Use `one or more` when simultaneous events should produce one trigger:

> Whenever one or more creatures you control deal combat damage to a player,
> draw a card.

Use a bare singular event when each event should trigger separately. This is a
functional distinction, not stylistic polish.

##### Intervening if

To make a trigger conditional both when it would trigger and when it resolves,
put `if` immediately after the trigger event:

> At the beginning of your upkeep, if you control three or more artifacts,
> draw a card.

That exact position creates an intervening-if clause [CR#603.4]. An `if` later
in the effect has only its ordinary conditional meaning:

> Whenever this creature attacks, draw a card if you control an artifact.

Do not move `if` between those positions merely for rhythm.

##### Trigger instructions

Put instructions about the ability after the effect in a separate sentence:

> This ability triggers only once each turn.  
> This ability can't be countered.  
> This ability can target a permanent with hexproof as though it didn't have
> hexproof.

These sentences do not all function at the same time: target and countering
instructions apply to the ability on the stack [CR#603.1a], while a
trigger-frequency sentence limits whether it triggers. Do not fold either kind
into the trigger event.

##### Delayed and reflexive triggers

A resolving effect can create a delayed triggered ability with a timing clause
embedded in its instruction:

> Return that card to the battlefield at the beginning of the next end step.

Use `When you do` or `When [event happens] this way` for a reflexive trigger
created during resolution. Use `If you do` when no new trigger is intended.

#### Static abilities

Write a static ability as a declaration that is continuously true:

> Creatures you control get +1/+1.  
> This creature can't block.  
> You may play an additional land during each of your turns.  
> Spells your opponents cast cost {1} more to cast.

Static abilities have no activation colon and no trigger word. Introduce a
condition with `as long as`, `during`, `while`, or `if`:

> This creature has flying as long as you control an artifact.  
> During your turn, this creature has first strike.  
> You may cast this card from your graveyard if a creature died this turn.

Do not begin a continuous condition with `Whenever`; that creates a triggered
ability instead of a continuous condition.

#### Keyword abilities on a shared line

Put simple keyword abilities on one line, separated by comma-space, when each
is independently intelligible:

> Flying, vigilance, lifelink

Use a semicolon when a following keyword has reminder text and a comma would
make the reminder's attachment unclear:

> First strike; reach (This creature can block creatures with flying.)

Keep a parameter with its keyword: `flying, ward {2}`. A shared keyword line is
several abilities despite occupying one paragraph, an express exception to the
ordinary paragraph rule [CR#113.2c].

Do not join a keyword and a nonkeyword sentence in a comma list. Put the
sentence in its own paragraph.

### 9. Spell costs, casting permissions, and restrictions

#### Mana costs and mana amounts

Write a mana amount as adjacent symbols: `{2}{U}{U}`. In prose, distinguish
the printed **mana cost** from the calculated **mana value** and from an amount
of mana in a cost or pool.

> mana cost `{2}{U}`  
> mana value 3  
> Add {C}{C}.  
> Pay {3}.

Use `mana value`, not `converted mana cost`. Do not write generic mana as a bare
digit: `{3}` represents three generic mana in a mana amount or cost; `3` is a
scalar value.

#### Additional costs

Put a mandatory additional-cost sentence before a spell's resolving
instructions:

> As an additional cost to cast this spell, sacrifice a creature.

For an optional additional cost, make the permission explicit:

> You may discard a card as an additional cost to cast this spell.

Use `As an additional cost`, not `When you cast`, when payment is required
during casting. Coordinate alternatives inside the cost with `or`; coordinate
several required components with `and` or a list whose scope is unmistakable.
Use the approved template for a keyworded additional cost. Do not restate the
cost outside that template unless approved reminder text calls for it.

#### Alternative costs

Use one of these forms:

> You may pay [cost] rather than pay this spell's mana cost.  
> You may cast this spell for [cost] rather than its mana cost.  
> You may [take an action] rather than pay this spell's mana cost.

`Rather than` marks the replacement of one cost with another. Do not describe
an alternative cost as a reduction. A spell cast `without paying its mana cost`
also uses an alternative-cost template; do not say it is cast “for free.”

When an effect grants permission to cast another card without its mana cost,
name the card first and keep the cost phrase adjacent to `cast`:

> You may cast that card without paying its mana cost.

#### Cost increases and reductions

Use `cost [amount] more to cast` and `cost [amount] less to cast`:

> Spells your opponents cast cost {1} more to cast.  
> This spell costs {1} less to cast for each artifact you control.

State what kind of cost can be reduced if the effect is narrower than the total
cost. Put a floor or special limitation in its own sentence using the fixed
wording for that mechanic. Do not say a spell's mana cost *becomes cheaper*.

Use `the [keyword] cost is equal to its mana cost` when defining a keyword's
variable cost. Use `paying [cost] in addition to paying its other costs` when a
permission explicitly preserves other costs.

#### Casting restrictions

Put a restriction that applies to the spell before its effect:

> Cast this spell only during combat before blockers are declared.  
> Cast this spell only if you control a legendary creature.

Use `cast`, not `play`, for a spell. `Play` is deliberately broader and can
cover either playing a land or casting a spell. Use `play that card` when both
lands and nonlands are intended. For an identified nonland card outside the
stack, write `cast that card`; for a land, write `play that land`. Once casting
has begun and the object is on the stack, it is `that spell`.

#### Permissions from unusual zones

State the object, origin, duration, and cost treatment:

> You may cast creature spells from your graveyard.  
> Until end of turn, you may play that card.  
> You may cast that card for as long as it remains exiled, and mana of any type
> can be spent to cast that spell.

Use `from among cards exiled with this artifact` when the permission selects
from a linked group. A permission does not itself move the card out of its
zone; use the correct cast or play verb.

### 10. Logic, choice, and coordination

#### May, can, must, and can't

Use `may` for a player-facing optional action, including a granted cast or play
permission:

> You may draw a card.  
> Target player may sacrifice a creature.

Use `can` or `can't` for a standing capability, prohibition, or `as though`
statement:

> You can cast spells as though they had flash.  
> This creature can't block.  
> Damage can't be prevented.

Do not treat `may` and `can` as interchangeable. Use the prescribed
effect-family template.

Use `must` only when the rules need an explicit obligation not already created
by the imperative voice: `This creature must be blocked if able.` Ordinary
instructions such as `Discard a card` are mandatory without the word *must*.

Use `if able` after the required action when the game should require as much
compliance as possible: `This creature attacks each combat if able.` Use `if
able`, not `if possible`.

Use `only` immediately before the limiting condition or timing phrase. Repeat
it when two independently scoped restrictions could otherwise blur together:
`only if ... and only as a sorcery`.

#### Sequence: and, then, if you do, when you do

Use `and` for coordinated results without an emphasized ordering distinction:

> You draw a card and you lose 1 life.

Use `then` when the order is important or when the second instruction refers
to the result of the first:

> Draw a card, then discard a card.  
> Exile the top three cards of your library, then choose one of them.

`Then` orders instructions; it does not by itself make the second instruction
conditional on successful completion of the first. Use `if you do` for that
dependency:

> You may sacrifice a creature. If you do, draw two cards.

Use `When you do` only to create a reflexive trigger. Use `otherwise` for the
alternative branch of an explicit condition, and `instead` for a replacement,
not as synonyms for `else` in casual prose.

#### And, or, and and/or

Use `and` when every listed requirement or result applies. Use `or` for an
alternative. Use `and/or` when either category alone or both together qualify:

> It's both an artifact and a creature.  
> artifact or enchantment  
> instant and/or sorcery cards

An adjacent type phrase without a coordinator is an intersection: `artifact
creature`. A coordinated type phrase is an alternative or union: `artifact or
creature`.

In a list of three or more, put the serial comma before `and`, `or`, or
`and/or`:

> artifacts, creatures, and/or lands  
> flying, vigilance, and lifelink

Repeat a preposition when failing to repeat it could change the grouping:
`protection from black and from red`. Share it when the scope is plainly one
list: `a card from your hand or graveyard`.

#### Conditions

Use an initial `if` clause for a condition governing the whole instruction:

> If you control an artifact, draw a card.

Put a trailing `if` next to the result it alone governs:

> Draw a card if you control an artifact.

Use `unless` for an exception or a toll whose failure produces the stated
result:

> Counter that spell unless its controller pays {2}.  
> This land enters tapped unless you control a Forest.

Use `as long as` for a continuous condition, `for as long as` for a duration,
`while` for a state or interval, `during` for a turn or phase interval, and
`until` for an ending boundary. These are not interchangeable.

For a continuous condition shared by two coordinated predicates, a single
postpositive `as long as` clause may follow the coordination when no comma
separates its members:

> This creature gets +1/+1 and has trample as long as there are four or more
> card types among cards in your graveyard.

When one continuous condition governs three or more coordinated predicates,
front the condition rather than leave it after a serial-comma list:

> As long as there are four or more card types among cards in your graveyard,
> this creature gets +2/+2, has flying, and attacks each combat if able.

This rule concerns coordination of predicates, not a coordinated complement
under one predicate. One `has` predicate may take a serial-comma list followed
by one shared condition:

> This creature has trample, hexproof, and haste as long as an opponent
> controls a planeswalker.

When coordinated predicates have different conditions, repeat `as long as`
with each predicate it governs:

> This creature has flying as long as you control an Island, has first strike
> as long as you control a Mountain, and has trample as long as you control a
> Forest.

Apply these scope conventions within one rules sentence and one quotation
level. A condition inside a quoted granted ability does not govern text outside
that quotation.

#### Ordinary choices

Use `Choose a [quality]` when the choice is made by the instruction and is not a
target. State the chooser if it is not the effect's controller:

> Choose a color.  
> Target opponent chooses a creature they control.  
> Each player sacrifices a creature of their choice.

Use `of their choice`, not a target phrase, when a player affected during
resolution chooses their own object. Use `at random` after the object or action
being randomized: `discards a card at random`; use `in a random order` for a
group placed into a zone.

If a choice persists, name the stored quality consistently: `the chosen color`,
`the chosen creature`, `the named card`. Do not switch from *chosen* to
*selected*.

#### Modal choices

Use a modal header followed by U+2022 bullets:

> Choose one —  
> • Draw two cards.  
> • Destroy target artifact.

Use the header that states the permitted number of modes. Standard forms
include:

- `Choose one —`
- `Choose two —`
- `Choose one or both —`
- `Choose one or more —`
- `Choose up to one —`
- `An opponent chooses one —`
- a trigger or activated-effect prefix ending in `choose one —`

The header and bullets are one ability. Its controller chooses the mode or
modes as part of casting the spell or activating the ability, or as part of
putting a triggered ability on the stack [CR#700.2,700.2a,700.2b]. A target
appears only in the mode that needs it. Do not repeat the header in each bullet.
End every ordinary modal bullet with a period, even if it contains only a short
instruction.

Bullet groups also use complete-sentence headers when the choice count is
conditional, repeatable, randomized, or followed by targeting or activation
instructions:

> Choose three. You may choose the same mode more than once.  
> Choose one. If you control a commander as you cast this spell, you may choose
> both instead.

Use labeled modes when the card calls for names:

> • Khans — [Effect.]  
> • Dragons — [Effect.]

The bullet and label are both followed by spaces; the label's em dash is
spaced. State `You may choose the same mode more than once.` only when repetition
is intended.

A trigger or activated effect may introduce a dash-ended header on the same
line:

> At the beginning of your end step, choose one —  
> • Draw a card.  
> • Create a Treasure token.

The bullets remain modes of that one ability.

##### Pawprint modes

Use this variable header; spell out `[number]`:

> Choose up to [number] {P} worth of modes. You may choose the same mode more
> than once.

For example, a five-pawprint card says `Choose up to five {P} worth of modes.`

Each mode begins with one or more adjacent `{P}` symbols, then a spaced em dash:

> {P} — Draw a card.  
> {P}{P} — Exile target nonland permanent.

Pawprints are weights, not mana, counters, or a cost. The lines are modes of
the header ability.

#### For each and distributed results

Put `for each` next to the set that determines a repeated action or multiplier:

> Draw a card for each creature you control.  
> For each opponent, destroy up to one target creature that player controls.

Use `for each` plus an imperative when choices or targets are made separately
for members of the set. Use `equal to the number of` when one scalar value is
calculated from the set. Keep the referent available with `that player`, `that
creature`, or `those cards`.

### 11. Timing and duration

#### Turn-structure names

Lowercase turn, phase, and step names: `turn`, `beginning phase`, `first main
phase`, `combat phase`, `end step`, `cleanup step`, `upkeep`, `draw step`,
`declare attackers step`, `declare blockers step`.

Use `At the beginning of` for the start of a step, phase, or turn:

> At the beginning of your upkeep, ...  
> At the beginning of combat on your turn, ...  
> At the beginning of each opponent's end step, ...

Use `during` for an interval: `during your turn`, `during combat`. Use `on your
turn` in the fixed phrase `at the beginning of combat on your turn` and similar
constructions. Do not use `at your upkeep` or `on your upkeep`.

#### Until end of turn and delayed endpoints

The standard duration is `until end of turn`, with no *the*:

> Target creature gets +2/+2 until end of turn.

Use `until end of turn` for the ordinary current-turn duration. Use fuller
`until the end of [specified] turn` forms when the wording identifies a
particular turn, as in `until the end of your next turn`.

The parallel combat duration is `until end of combat`, also without *the*.
Keep it distinct from the trigger time `At end of combat`.

Use this delayed-trigger endpoint:

> At the beginning of the next end step, [effect].

Do not use `at end of turn` for that event. Reserve `At end of combat` for the
combat endpoint.

#### This, next, and each

Use `this turn` for the current turn, `your next turn` for the next turn of a
specified player, and `the next end step` for the next qualifying step
regardless of whose turn it is. Use `each turn` for a per-turn limit and `each
of your turns` when quantifying only one player's turns.

The standard frequency wording is:

> Activate only once each turn.  
> This ability triggers only once each turn.  
> Whenever you cast your second spell each turn, ...  
> Once during each of your turns, you may ...

Do not write `once per turn`. Use `the first time each turn` when the first
event, rather than the ability itself, is what matters.

#### Continuous and bounded durations

Use:

- `as long as [condition]` for a continuously rechecked condition;
- `for as long as [object remains/state persists]` for a duration created by
  an effect;
- `while [card] is in your graveyard` for a zone-bound static ability;
- `until [object] leaves the battlefield` for a linked endpoint;
- `for the rest of the game` for a game-long duration; and
- `the next time [event] this turn` for a one-use shield or modification.

Do not omit a duration from a temporary power/toughness change, ability grant,
control change, or color/type change unless the effect is intentionally
indefinite under the rules.

## Part IV. Standard effect templates

### 12. Standard effect families

This section gives standard surface templates. Replace bracketed content only
with a grammatically compatible selector or value.

#### Cards and zones

##### Draw, discard, and hand size

Use:

> Draw a card.  
> Draw two cards.  
> Target player discards a card.  
> Each opponent discards two cards.  
> Discard your hand, then draw that many cards.  
> Your maximum hand size is seven.  
> You have no maximum hand size.

Use `draw`, never *take*, for cards moving from library to hand by the draw
action. Use `put [card] into your hand` when the move is not a draw. A player
discards; an effect does not “make a card discard.” Use `at random` or `of their
choice` when the method matters.

##### Look, reveal, and the top of a library

Looking does not reveal. State exactly who receives the information:

> Look at the top three cards of your library.  
> You may look at the top card of your library any time.  
> Reveal the top card of your library.  
> Each player reveals their hand.

Use this selection template:

> Look at the top [number] cards of your library. You may reveal a [quality]
> card from among them and put it into your hand. Put the rest on the bottom of
> your library in a random order.

Use `from among them`, not *amongst*. Use `in any order` when the player chooses
the order and `in a random order` when they do not.

For reveal-until effects, identify both the kept cards and the rest:

> Reveal cards from the top of your library until you reveal a [quality] card.
> Put that card into your hand and the rest on the bottom of your library in a
> random order.

##### Search and shuffle

Use this single-player template:

> Search your library for a [quality] card, reveal it, put it into your hand,
> then shuffle.

Omit `reveal it` when the searched-for card is not restricted by a hidden
quality or the governing mechanic says not to reveal. If several cards are
found, use plural pronouns and state any split destinations before shuffling.

Use `then shuffle`, not `then shuffle your library`, after the ordinary search
template. Use `shuffle your library` when an effect shuffles without the
ordinary search antecedent. For several players, state who actually searched:

> Each player searches their library for a basic land card, puts it onto the
> battlefield tapped, then shuffles.

or, when participation varies:

> Then each player who searched a library this way shuffles.

The Vintage corpus uses `shuffle your deck` only on three Draft Matters cards,
all in this pregame template:

> Before you shuffle your deck to start the game, [effect].

Otherwise, the zone is a library, not a deck.

##### Moving cards and permanents

Use the zone as a destination:

> Put that card into your hand.  
> Return target creature to its owner's hand.  
> Return target creature card from your graveyard to the battlefield.  
> Put target card on top of its owner's library.  
> Put those cards on the bottom of your library in any order.

Use `return` when an object moves to a zone or state it occupied before or when
the approved effect family uses the return template. Use `put` for a direct
zone move without that implication. Do not use *send*, *bring back*, or *move*
as generic substitutes.

Use `enters tapped`, `enters with [counters] on it`, or `put it onto the
battlefield tapped`. Put `under [player's] control` after `battlefield`:

> Return that card to the battlefield tapped under its owner's control.

Use `leaves the battlefield` for any destination and `dies` only for a creature
going from battlefield to graveyard.

##### Exile and permissions tied to exile

Use `exile` as both verb and zone name:

> Exile target creature.  
> a card you own in exile  
> For as long as that card remains exiled, you may play it.

Use `exile it instead` in a replacement effect. Use `exile that token at the
beginning of the next end step` for a delayed cleanup. If cards are exiled face
down, say who may look at them and how the later text identifies the linked
pile.

`Face down` is an adverb; `face-down` is an adjective:

> Exile it face down.  
> a face-down card

#### Objects and characteristics

##### Tokens

The ordinary token description order is:

> quantity → tapped state → power/toughness → color → subtype(s) → additional
> card type(s) → `creature token` → name/ability/attack clauses

Examples:

> Create a 1/1 red Goblin creature token.  
> Create two 2/2 colorless Robot artifact creature tokens.  
> Create a 2/2 blue Bird enchantment creature token with flying.  
> Create a tapped 1/1 black Fungus Zombie creature token named Cordyceps
> Infected.

Use singular subtype words even after a plural quantity. For a normal described
token, `tapped` precedes its characteristic string: `Create a tapped 2/2 black
Zombie creature token.` Append `that's tapped and attacking` when a newly
created noncopy token enters combat. A copy uses the distinct fixed order `a
tapped and attacking token that's a copy of ...`.

For a predefined artifact token kind, capitalize the subtype and omit the
implied card type in the creation instruction: `Create a Treasure token`,
`Create two Food tokens`, `Create a tapped Powerstone token`. Include the card
type only when a fixed reminder supplies it (`It's an artifact with ...`). Role
enchantment tokens retain the Role head: `Create a Cursed Role token attached
to ...`.

For a named legendary token, lead with the name and apposition:

> Create Avacyn, a legendary 8/8 white Angel creature token with flying,
> vigilance, and indestructible.

Do not move a legendary token's name to a trailing `named` clause. Use the
appositive form required for a named token whose name precedes its other
characteristics [CR#111.9].

For a nonlegendary named token whose complete characteristics are given first,
append `named [Name]`. Do not put the token name in quotation marks.

Grant simple keyword abilities with `with`: `a 1/1 white Bird creature token
with flying`. Quote a full rules ability: `with "When this token dies, draw a
card."` If the ability description is long, end the creation sentence and use
`It has ...` or `That token has ...`.

An unspecified token name follows from its subtypes under the rules; a
specified proper name overrides that default [CR#111.4].

##### Copies

For a token copy, use:

> Create a token that's a copy of target creature.

Use `tokens that are copies of` for a plural quantity. Do not write `copy
token`. A keyword's fixed reminder text may prescribe a different internal
word order; do not generalize that wording to ordinary copy effects.

Introduce modifications with `except`:

> Create a token that's a copy of that card, except it's a 3/3 black Zombie in
> addition to its other types and it has menace.

Keep all copy exceptions inside the same sentence and use the serial comma when
there are three or more. Use `in addition to its other types` when types are
retained; use `with no mana cost` when that characteristic is deliberately
removed.

For a spell or ability copy, use:

> Copy target instant or sorcery spell. You may choose new targets for the
> copy.

Use `copy it` only after an unambiguous spell or ability antecedent. A copied
card and a token copy are not described by the same surface template.

##### Counters

Counter names are lowercase common modifiers:

> Put a +1/+1 counter on target creature.  
> Put two stun counters on it.  
> Remove a time counter from this card.  
> This creature enters with three +1/+1 counters on it.

Use `on` when putting or counting counters on an object or player and `from`
when removing them. Use `has` for testing and omit redundant `on it`: `if it
has a stun counter`. Use `one or more counters` when simultaneous placement
should be grouped.

A keyword counter takes the lowercased keyword as its modifier: `a flying
counter`, `an indestructible counter`. Use `each kind of counter` to range over
counter names; use `one of each of those kinds` when referring to a prior
catalog.

Use `double the number of each kind of counter on it`, not *double its
counters*, when preserving kinds matters. A player **gets** poison, energy, or
experience counters when the applicable template prescribes `gets`. **Put**
counters on an object.

##### Power and toughness

Use `gets` for a modifier:

> Target creature gets +3/+1 until end of turn.  
> Creatures you control get -1/-1.

Use `has base power and toughness` to set base values:

> This creature has base power and toughness 4/4.

Use `power and toughness are each equal to` for a characteristic-defining or
continuous equality:

> This creature's power and toughness are each equal to the number of lands
> you control.

If only one characteristic changes, name it: `base power becomes 1`; `toughness
is equal to ...`. Use `switch its power and toughness` for the prescribed
exchange. Put temporary durations after all coordinated changes.

Do not say a creature *gains +1/+1*. It gets a modifier or has a counter put on
it.

##### Granting and removing abilities

Use `gains` for an ability granted by a resolving effect. State a duration when
the grant is temporary:

> Target creature gains flying until end of turn.  
> Those creatures gain first strike, vigilance, and lifelink until end of turn.

Use `has` or `have` for a static continuous statement:

> Equipped creature has flying.  
> Creatures you control have vigilance.

Use `loses all abilities` or `loses flying` to remove abilities. Use `can't
have or gain [ability]` only when future grants must also be prohibited. Quote
a full nonkeyword ability; do not quote a bare keyword.

When one condition applies to several keywords, use `The same is true for ...`
followed by a serial-comma list only when every listed ability truly follows
the same condition and duration.

##### Colors and types

Use `is [color]` or `becomes [color]` for a color-setting effect and state
whether old colors remain if that matters. Use the exact retention templates
for types:

> It becomes a 4/4 red Dragon creature until end of turn.  
> It's still a land.  
> It's an artifact in addition to its other types.  
> It becomes a Zombie in addition to its other creature types.

`Still a land` and `in addition to its other types` carry different rules
instructions; do not exchange them for stylistic variety. Use `all colors`,
`colorless`, `the color or colors of your choice`, and `the chosen color` in
their defined senses.

#### Combat and resources

##### Combat

Use the defined verbs and participles:

> Whenever this creature attacks, ...  
> Whenever this creature blocks or becomes blocked, ...  
> target attacking creature  
> target creature blocking it  
> defending player  
> attacks each combat if able  
> can't attack or block  
> can't be blocked except by two or more creatures

For one trigger per attack declaration, use `Whenever you attack`. For one
trigger per attacking creature, use `Whenever a creature you control attacks`.
Use `attacks alone` or `a creature you control attacks alone` for the approved
single-attacker condition; do not paraphrase it as “is your only attacker.”

For a creature placed into combat without being declared as an attacker, use
`tapped and attacking`; do not use attack-trigger wording. Use `at end of
combat` for the combat cleanup endpoint when that fixed duration is intended.

##### Damage

Make the source the grammatical subject:

> This creature deals 3 damage to any target.  
> Shock deals 2 damage to any target.  
> It deals damage equal to its power to that player.

Use `combat damage` and `noncombat damage`, not *battle damage* or
`non-combat`. Use `damage dealt this way` to link a result to a particular
instruction. Use `that much damage` after a scalar antecedent.

For division, use:

> [Source] deals [amount] damage divided as you choose among one, two, or three
> targets.

Use `excess damage` only for the defined excess calculation. State whether a
source deals damage to a permanent, player, battle, or `any target`; do not use
*hit* or *hurt*.

##### Life

Use:

> You gain 3 life.  
> Target opponent loses 2 life.  
> Pay 4 life.  
> Your life total becomes 10.  
> Exchange your life total with target creature's power.  
> You gain life equal to the damage dealt this way.

Life is a scalar, so use digits and `amount of life`/`that much life`. A loss of
life is not damage; do not swap the verbs. A cost says `Pay N life`, not `Lose N
life`.

##### Mana

Use `Add [symbols]` for a fixed mana result:

> {T}: Add {G}.  
> Add {R}{R}{R}.

Use words for flexible production:

> Add one mana of any color.  
> Add two mana in any combination of colors.  
> Add an amount of {G} equal to this creature's power.

When all six mana types are cataloged in prose, use `white, blue, black, red,
green, and colorless`. Use `mana of any type` when colorless is included and
`mana of any color` when it is not.

Put a spending restriction in a following sentence:

> Spend this mana only to cast creature spells.  
> This mana can't be spent to cast a noncreature spell.

For retention, use `you don't lose this mana as steps and phases end`. Refer to
a `mana pool` only when the governing fixed template requires that term.

#### Spells and game actions

##### Cast, play, counter, and copy

Use `cast` for spells and `play` for lands or the deliberately combined
land/card permission. Use `counter target spell`, `counter target activated or
triggered ability`, and `counter it unless its controller pays [cost]`.

Use `You may choose new targets for the copy`, not *redirect the copy*. If an
effect casts a copy of a card, use the exact `copy that card. You may cast the
copy` sequence; a card copy outside the stack and a copied spell are different
objects.

Use `was cast` to test casting history and `When you cast this spell` for a cast
trigger. An enters trigger is not a substitute: a permanent can enter without
being cast.

##### Control, exchange, attach, and unattach

Use:

> Gain control of target creature until end of turn. Untap it. It gains haste
> until end of turn.  
> Exchange control of two target creatures.  
> Attach target Equipment you control to target creature you control.  
> Unattach that Equipment.

Use `gain control`, not *take control*. State a duration for a temporary control
effect. The standard temporary-control bundle puts untapping and haste in
separate sentences so each instruction and duration is clear.

Use `attached to`, `equipped`, and `enchanted` according to the object's kind.
Use the approved attachment-keyword line to define its legal object and cost.
Do not write a generic “equip to” instruction.

##### Sacrifice, destroy, exile, and regenerate

Use `sacrifice` only for a player sacrificing a permanent they control. State
the player when it is not the resolving effect's controller:

> Sacrifice a creature.  
> Each opponent sacrifices a creature of their choice.

Use `destroy target [permanent]` for destruction and `exile target [object]`
for exile. Do not use *remove from the game*. Use `It can't be regenerated`
only when the design specifically defeats regeneration.

##### Randomness, dice, coins, and voting

Use `at random` for selecting an object or player, `in a random order` for
ordering a group, and `choose ... at random` when the instruction itself makes
a random selection.

Use `roll a d20`, `roll two d20 and ignore the lower roll`, or `roll a
six-sided die` according to the die. Put d20 outcomes on separate result lines
using the [range-and-bar format](#bullets-vertical-bars-and-square-brackets).
Use `Flip a coin.` and the lowercase results `heads` and `tails`.

For votes, use `Starting with you, each player votes for [label] or [label].`
Then compare `more votes`; state what happens if the vote is tied. Mode-like
vote labels are lowercase common words unless they are exact names.

### 13. Continuous, replacement, and prevention templates

#### Choose the effect-family template

For a response that happens after an event, use a triggered ability beginning
with **When**, **Whenever**, or **At**. To modify an event before it happens,
use the appropriate `would ... instead`, `as ... enters`, or enters-modifier
replacement template [CR#614.1a..614.1e]. For an ongoing truth, use a static
declaration. Do not combine these templates.

- Do not use: `Whenever you would draw ...`
- Trigger after the event: `Whenever you draw a card, you gain 1 life.`
- Replace the event: `If you would draw a card, you may draw two cards instead.`
- Add an event at a turn point: `At the beginning of your draw step, draw an
  additional card.`

#### Would and instead

Use this replacement template:

> If [event] would happen, [modified event] instead.

or:

> If [subject] would [verb], it [verbs differently] instead.

Examples:

> If a card would be put into your graveyard from anywhere, exile it instead.  
> If a source would deal damage to this creature, it deals double that damage
> to this creature instead.  
> If an effect would create one or more tokens under your control, it creates
> twice that many of those tokens instead.

Place `instead` as close as practical to the replacement event, at the end of
its clause unless that placement would create ambiguity. Use `may ... instead`
for an optional replacement. Identify the original event completely enough to
define what is replaced.

`Instead` also appears in a conditional modification after an action:

> Put two +1/+1 counters on it instead if a creature died this turn.

Keep the condition adjacent to the replacement it controls. Do not use
`instead` merely to contrast modes or later sentences.

#### Enters replacements

Use these fixed shapes for how a permanent enters:

> This land enters tapped.  
> This creature enters with two +1/+1 counters on it.  
> As this artifact enters, choose a color.  
> This creature enters as a copy of any creature on the battlefield.

Use `As`, not `When`, when the choice or modification must exist as the object
enters. Use `When [it] enters` for an effect that triggers after entry. Write
`enters with`, not `enters the battlefield with`, for the ordinary modifier.

For a conditional tap clause, use:

> This land enters tapped unless you control a Forest.

For additional counters, keep `additional` next to the counter quantity:

> It enters with two additional +1/+1 counters on it.

#### Skip

Use `skip` for an event, step, phase, or turn replaced with nothing:

> Players skip their untap steps.  
> If a player would draw a card, that player skips that draw instead.

For a one-shot skipped turn, write `You skip your next turn.` State the subject
and exactly what is skipped.

#### Prevention is its own family

Use one of these forms:

> Prevent the next 3 damage that would be dealt to target creature this turn.  
> If damage would be dealt to this creature, prevent that damage.  
> Damage can't be prevented.

Prevention effects use `prevent` and `would be dealt`; a consumable shield uses
`the next [N] damage ... this turn`. Do not add `instead` to a prevention
template. Use `prevent all but [N] of that damage` when preserving part of one
damage event.

Include the source or damage quality if the shield is restricted. Prevention
is not life gain; do not template it as a replacement with life unless that is
the actual design.

State nonpreventability as `Damage can't be prevented` or `[quality] damage
can't be prevented`. Do not write *is unpreventable*.

#### Continuous characteristic changes

State continuously true effects in present tense:

> Other creatures you control get +1/+1.  
> This permanent is all colors.  
> Lands are 1/1 creatures that are still lands.  
> Each creature is a Frog in addition to its other types.

State one-shot changes with `becomes`, `gets`, `gains`, or `loses` and give a
duration if temporary. Use `base power and toughness` when setting the base,
and `gets` when modifying it. Use the explicit retention phrases `in addition
to its other types` and `It's still a land`; ordinary English *also* is not an
adequate replacement.

#### Restrictions, requirements, and exceptions

Use:

> [Subject] can't [action].  
> [Subject] can [action] only [condition].  
> [Subject] attacks each combat if able.  
> [Subject] can't [action] unless [condition].  
> [Subject] may [action] as though [counterfactual condition].  
> [Subject] can be blocked only by [quality].

Prefer the positive form when it is shorter and equally exact, but do not
reverse polarity if that changes interaction with a `can't` effect. Put `except`
after a prohibition when specifying the only escape: `can't be blocked except
by two or more creatures`.

## Part V. Keywords, frames, and final review

### 14. Keywords, labels, and reminder text

#### Keep the four systems separate

Use the formal category assigned by the Comprehensive Rules; do not infer the
category from typography alone.

| Formal category | Required editorial treatment |
|---|---|
| keyword ability | Use the approved keyword line or in-sentence form: `Flying`; `Ward {2}`; `has flying`. |
| keyword action | Conjugate the approved verb or verb phrase in ordinary sentence grammar: `Scry 2.`; `This creature explores.` |
| ability word | Set the approved catalog label in italics before a spaced em dash: `Landfall — ...`. |
| flavor word | Set the approved bespoke label in italics before a spaced em dash: `Heavy Power Hammer — ...`. |

The Comprehensive Rules catalogs keyword abilities [CR#702.1] and keyword
actions [CR#701.1]. Ability words label groups of functionally similar
abilities [CR#207.2c]; flavor words label one particular ability [CR#207.2d].
Neither kind of italic label supplies rules meaning.

Apply a spaced em dash only when the assigned formal category requires it. A
spaced dash does not establish a category; a rules-defined keyword signature
may also contain one.

#### Keyword abilities

Capitalize a keyword ability only when it begins its line or sentence:

> Flying  
> First strike  
> Ward {2}

Lowercase the same keyword in running text:

> Target creature gains flying and first strike until end of turn.  
> Creatures you control have ward {2}.

For a multiword keyword, capitalize only the first word at the start of a line:
`Double strike`, `Living weapon`, `Umbra armor`. Preserve internal proper names
and punctuation in fixed keywords, such as `For Mirrodin!`.

After a comma or semicolon in a shared keyword line, begin the next keyword in
lowercase unless it contains a proper name:

> Flying, first strike, vigilance  
> Protection from black; banding (...)

Do not add a terminal period to a keyword line unless the keyword's fixed
grammar includes one. Parameters and reminder text do not turn the keyword's
name into title case.

Use each keyword ability's approved spelling, parameter order, separators,
punctuation, and display form. The Comprehensive Rules supply the catalog and
each entry's signature. Never extrapolate one keyword's presentation from
another keyword with a superficially similar parameter.

When an effect refers to a keyword, lowercase its name as a noun-like game
term: `has flying`, `gains ward {2}`, `without flying`, `the same is true for
first strike`. Add *ability* only when needed to distinguish the reference from
another meaning.

#### Keyword actions in sentences

Keyword actions behave like ordinary verbs. Capitalize them at the beginning
of a sentence or cost component and lowercase them elsewhere. Use the
inflection prescribed by the action's rules entry:

> Scry 2, then draw a card.  
> Whenever you scry, ...  
> This creature explores.  
> It endures 2.  
> {2}, {T}, Sacrifice this artifact: Draw a card.

Do not capitalize an action merely to advertise that it is a game term. Do not
paraphrase a keyword action if it states the intended operation.

Give every action its defined arguments in the defined order:

> Scry 2.  
> This creature explores.  
> The Ring tempts you.  
> Target opponent faces a villainous choice — They discard their hand, or you
> draw three cards.

Use each approved keyword action as its complete verb or verb phrase. Preserve
its prescribed subject, arguments, word order, capitalization, agreement, and
inflection. For example, write `The Ring tempts you.` and `Target opponent
faces a villainous choice — ...` in their complete approved forms. Phrasal
actions do not form a separate category.

Other action names change form in running text: `this creature explores`, `it
endures 2`, `cards manifested this way`, `transform it`. Do not mechanically
append *-ed* or *-s* to a multiword catalog label.

A capitalized keyword action may begin an activation cost: `Sacrifice this
artifact:`. That capitalization does not make it a keyword ability.

#### Ability words and flavor words

An ability word is italicized in print, uses its catalog spelling, and appears
at the beginning of an ability followed by a spaced em dash:

> Landfall — Whenever a land you control enters, ...  
> Fateful hour — If you have 5 or less life, ...

Set an ability word in sentence case: `Fateful hour`, `Pack tactics`, and
`Council's dilemma`, not title case. Ability words have no rules meaning
[CR#207.2c]. Do not make later text refer to “the landfall ability” as though
the label supplied rules.

A flavor word occupies the same typographic position and also has no rules
meaning [CR#207.2d]. Give a flavor word the natural title casing appropriate to
its bespoke label:

> Heavy Power Hammer — Whenever this creature deals combat damage ...

Because a flavor word is bespoke, punctuation within it may be expressive.
That freedom stops at the em dash; the rules text after it follows ordinary
Oracle style.

#### Reminder text

Reminder text summarizes a rule; it does not create one. Put it immediately
after the relevant keyword unless it applies to the card frame or intrinsic
card property, in which case it may be a separate parenthetical paragraph.

Use the approved reminder template exactly, substituting only its prescribed
source noun, pronoun, and parameter slots. Write it as one or more complete
sentences unless the approved form is a short inline gloss:

> Ward {2} (Whenever this permanent becomes the target of a spell or ability an
> opponent controls, counter it unless that player pays {2}.)

If reminder text would be the only place a necessary restriction appears, the
design is not correctly expressed unless the keyword's rules inherently supply
that restriction. Ability-word explanatory text is rules text after the dash,
not reminder text for the label.

Do not paraphrase reminder text. Preserve distinctions such as `this spell`
versus `this card`, `may` versus an imperative, the named zone, and any
activation limit.

A short inline reminder may instead be a noun or condition fragment, such as
`(two energy counters)` or `(front face up)`. Its punctuation belongs to the
outer sentence. Never nest reminder parentheses.

#### Named and batch terms

Use the prescribed casing for rules-defined names and designations: `the
monarch`, `the initiative`, `the city's blessing`, `day`, `night`, `the Ring`,
and `Ring-bearer`. Card names and face names retain their exact spelling,
accents, apostrophes, and internal capitalization every time they are repeated.

Rules-defined terms, batching terms, designations, and statuses preserve their
approved spelling, capitalization, governing verb, and predicate form. For
example, write `historic`, `modified`, and `outlaw` in lowercase; write a
player `becomes the monarch`; and preserve the capital in `Ring-bearer`. Do not
coin a parallel term or substitute a near-synonym.

### 15. Frame-dependent text

Apply a frame-specific structure before applying the ordinary paragraph rules.
Its punctuation and line breaks are part of the approved template. A
frame-specific structure changes only the text it supplies; every ordinary
ability in the same text box still follows this manual.

#### Sagas

When a Saga uses its frame reminder, put that reminder in a separate
parenthetical paragraph before the chapter lines:

> (As this Saga enters and after your draw step, add a lore counter. Sacrifice
> after III.)  
> I — [Effect.]  
> II — [Effect.]  
> III — [Effect.]

Use uppercase Roman numerals. Combine chapters that share exactly the same
text with comma-space: `I, II — [Effect.]`. Follow the chapter symbol with a
spaced em dash [CR#107.15].

A chapter may have a flavor-word label after its structural dash:

> II, III — Brimstone — Add {R}{R}{R}{R}.

The first dash is the chapter separator; the second introduces the flavor
word.

#### Classes

A Class begins with its frame reminder and always-on top section, then level
bars and the abilities in those sections:

> (Gain the next level as a sorcery to add its ability.)  
> [Level-1 ability.]  
> {1}{U}: Level 2  
> [Level-2 ability.]  
> {3}{U}: Level 3  
> [Level-3 ability.]

Capitalize `Level`; use an Arabic digit and no terminal period. Text after a
Class bar belongs to that level section until the next bar [CR#107.16].

#### Leveler cards

Put the level-up keyword above the bands. Order each band as an uppercase
header, its power/toughness line, then zero or more ability lines before the
next header:

> Level up {2}{G} (...)  
> LEVEL 1-3  
> 4/4  
> LEVEL 4+  
> 6/6  
> Trample

Use an ASCII hyphen for a bounded level range and `+` for an open one. The
header has no colon, em dash, or period. Set `LEVEL` in all capitals
[CR#107.8].

#### Cases

A Case uses these fixed labels on separate lines:

> To solve — [Condition.] (If unsolved, solve at the beginning of your end
> step.)  
> Solved — [Ability.]

Use sentence casing and a spaced em dash. `To solve` states the solve condition;
`Solved` introduces the ability available in that state. A colon inside the
Solved text can still mark an activated ability:

> Solved — Sacrifice this Case: Draw two cards.

#### Rooms

Treat each Room door as an independent named text portion with its own mana
cost. Do not insert a combined `Left // Right` name into either door's rules
text. Use the approved standalone reminder:

> (You may cast either half. That door unlocks on the battlefield. As a
> sorcery, you may pay the mana cost of a locked door to unlock it.)

Use `When you unlock this door` for the door's unlock trigger. Use `this Room`
when the whole permanent is the source. Capitalize `Room` and the subtype names
because Room is an enchantment subtype.

#### Die-result tables

Put the roll instruction in an ordinary sentence, then one result line per
outcome range:

> Roll a d20.  
> 1–9 | [Effect.]  
> 10–19 | [Effect.]  
> 20 | [Effect.]

In a d20 table, use an unspaced en dash for a closed range and spaces around
`|`. A result line is subordinate to the roll instruction; do not make each
result an ability. Use `N+` or `N or less` when a modified roll can leave one
side unbounded.

Ordinary six-sided result rows use a spaced em dash instead of a vertical bar:

> Roll a six-sided die.  
> 1 or 2 — [Effect.]  
> 3 — [Effect.]

#### Station cards

A Station card prints its ordinary abilities, the `Station` keyword and
reminder, then striated threshold sections. A threshold line begins:

> 8+ | Flying, trample

or opens a section whose following lines share that threshold. Use an Arabic
number plus `+`, spaces around the vertical bar, and normal ability grammar to
the right [CR#721.1..721.4].

Do not write the threshold as `8 or more —`, and do not confuse a Station
threshold with a leveler band.

#### Multifaced, split, Adventure, aftermath, meld, and prepare layouts

Treat each face or half's rules text as its own text box. Do not insert a
combined-name ` // ` separator into a face's rules sentences.

- A transforming double-faced card uses the relevant face name and the action
  `transform`; a back face may have standalone `(Transforms from
  [Name].)` reminder text when the frame calls for it.
- A modal double-faced card has independent cast/play text for each face; do
  not refer to it as transforming unless an effect actually transforms it.
- A split or aftermath card gives each half its own spell text and name. Do not
  combine the halves into one paragraph.
- An adventurer card gives the permanent face and Adventure spell their own
  text. Use the Adventure spell's own name when it is the source.
- A meld component may carry `(Melds with [Name].)` as a standalone reminder.
- A prepare card's permanent and prepare spell have separate text. Use the
  designation forms `becomes prepared` and `unprepares it` exactly
  [CR#722.3a..722.3b].
- Flip, reversible, mutate, and prototype layouts do not license alternate
  punctuation outside their named fixed structures.

#### Sieges and other standalone frame reminders

When a Siege uses its frame reminder, put it in an independent parenthetical
paragraph:

> (As a Siege enters, choose an opponent to protect it. You and others can
> attack it. When it's defeated, exile it, then cast it transformed.)

Use a parenthetical-only reminder paragraph when the governing frame or symbol
requires one. Insert the approved reminder verbatim. Do not rewrite a frame or
symbol reminder as an ordinary ability.

### 16. Final editorial review

Perform separate architecture and surface-style passes after drafting.

#### Architecture pass

- Confirm whether each paragraph uses an ordinary ability form or an approved
  keyword- or frame-supplied form. For ordinary text, confirm the assigned
  spell, activated, triggered, or static category and apply its template.
- For each activation, verify every cost is before one colon, every target is
  after it, and every activation instruction is last.
- For each ordinary standalone trigger, verify an initial When/Whenever/At
  clause and its separating comma; place an intervening `if` intentionally.
  Check embedded delayed and reflexive triggers separately.
- Confirm whether each choice is a target, a nontarget choice on resolution, a
  mode, or a random choice, then apply that category's surface form.
- Check the zone and role of every object, then choose the bare type, `card`,
  `spell`, `permanent`, `token`, or `source` accordingly.
- Verify each pronoun, `this way`, `that many`, and `the chosen ...` has one
  unambiguous antecedent.
- Give each one-shot temporary change a duration and each linked permission a
  precise endpoint.
- Confirm the assigned effect family and apply its template: replacement
  (`would ... instead`), prevention (`prevent`), entry modification (`as ...
  enters` or `enters with`), or a post-entry trigger.

#### Surface-style pass

- Recheck type and subtype capitalization, `non` compounds, subtype plurals,
  exact names, and player pronouns.
- Spell out counts of objects; use digits for scalar values and keyword
  parameters; preserve exact symbol tokens.
- Use the serial comma in prose lists, no conjunction in a bare keyword line,
  and no general-purpose semicolon.
- Check colon spacing, em-dash spacing by structure, ASCII versus true minus,
  slash usage, straight quotes, and the location of terminal punctuation.
- For every rules-defined keyword, insert its approved parameter signature and
  reminder wording verbatim.
- Verify that bracketed text is grammatical and rules-complete both with and
  without the bracketed material. Verify each mode, face, door, chapter, level,
  and threshold in its structural context.
- Compare the complete sentence with the approved template for the same effect
  family. If no template applies, submit the construction for rules review; do
  not splice fragments from unrelated templates.
