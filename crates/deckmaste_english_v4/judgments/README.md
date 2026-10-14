# Judgment set

The hand-written linguistic judgments of `deckmaste_english_v4`
([`docs/decisions/english-lexical-analysis.md`](../../../docs/decisions/english-lexical-analysis.md) §5).
It is the only tracked linguistic test evidence: one line per entry, no tree
dumps, nothing a tool emitted. Each `*.txt` file here holds the entries for one
topic. `seed.txt` is evaluated against the seed lexicon of
`tests/seed/`; every other file against the full lexical environment.

An entry asserts what is right about an expression, adjudicated against the
controlled Oracle English register (`docs/oracle-style-guide.md` and
`docs/decisions/english-v4.md`, with CGEL terminology and constituent
structure) by two independent judges. It never names a Production, a form
index or any other internal detail of a Reading.

## When an entry is enforced

An entry's Complexity Stratum is the Stratum the profiler assigns to its text
(`deckmaste_english_v4::profile`). `tests/judgments.rs` fails on any entry at
or below the completed Stratum (`deckmaste_english_v4::COMPLETED_STRATUM`) that
does not hold, and reports the others as pending. `seed.txt` is always
enforced. Every file is always parsed, so a malformed entry fails at once.

The census uses the same entries: a corpus unit with an entry whose text it
equals, none of whose Readings satisfies that entry, is `known-wrong-only`.

## Syntax

```
# a comment
<mark> [<card=NAME>] <text> [=> <assertions>]
```

- **Mark.** `+` the text is grammatical; `*` it is not (no Reading at all).
  The text is parsed as the grammar's root unless the mark names a Category,
  as in `+NounPhrase` or `*PrepositionPhrase`.
- **Card.** `<card=Seedborn Muse>` puts that card's names in scope, for a
  Shortened Name or a self-reference ([CR#201.5c]).
- **Text.** The exact expression, with its final punctuation; it runs to the
  end of the line or to `=>`.
- **Assertions** follow `=>`, separated by `;`. `||` separates alternative
  groups: each group must be satisfied by some one Reading. An Ambiguity
  witness is an entry with two or more groups.

An assertion is one of:

| Form | Holds when |
|---|---|
| `Category "surface"` | a Constituent of that Category spans exactly that surface, at any of its occurrences |
| `Category "surface"#2` | the same, at the second occurrence of the surface as whole words in the text |
| `... head LEMMA` | and its head word is LEMMA |
| `... as FUNCTION of LEMMA` | and it bears FUNCTION in a phrase whose head word is LEMMA |
| `... gap "surface"` | and it contains a Gap whose Antecedent spans that surface |
| `!<assertion>` | no Reading at all satisfies the assertion |
| `readings N` | the expression has exactly N Readings |

`LEMMA` is a lemma (`destroy`), a quoted multiword lemma (`"mana value"`) or,
to tell homographs apart, a lexeme id (`action:counter`). Inside a quoted
surface or lemma, `\"` is a literal quotation mark. `FUNCTION` is one of `Subject`, `Object`,
`IndirectObject`, `Complement`, `PredicativeComplement`, `Adjunct`,
`Modifier`, `Determiner` (CGEL functions: a dependent of a clause or verb
phrase that its head does not select is an `Adjunct`, of a nominal a
`Modifier`). `!` and `readings` apply to the entry, wherever they are written.

Assert a count only when the count is the claim: exactly one Reading, or
exactly the set of analyses each of which is endorsed.

## Conventions

These keep entries consistent where CGEL allows more than one presentation.
Each was settled while adjudicating the migrated judgments.

- A Gap's Antecedent is the whole Nominal the relative clause modifies,
  without its determiner: `gap "permanents"` in *all permanents you control*,
  `gap "first time"` in *the first time this ability has resolved*,
  `gap "mana of any type"` when the relative clause modifies that Nominal.
- A Preposition Phrase its head's frame does not select is an `Adjunct` (or a
  nominal `Modifier`), never a `Complement`: *return ... from your graveyard*,
  *play with your hand revealed*.
- *Up to* with a number is a `PrepositionPhrase` functioning as Determiner
  (`PrepositionPhrase "up to two" as Determiner of card`); an independent
  quantity is a fused-head `NounPhrase` (`NounPhrase "one" as Object of
  choose` in *Choose one*).
- Comparative and superlative forms are named by their lemma
  (`as Complement of great` for *greater*).
- A subjectless non-finite clause is asserted as `Clause` when infinitival
  *to* introduces it (`Clause "to cast this spell"`), and otherwise as its
  `VerbPhrase`: a gerund-participial or past-participial complement, adjunct
  or postmodifier (`VerbPhrase "paying its mana cost"`,
  `VerbPhrase "Starting with you"`, `VerbPhrase "put into your hand"`).
- An attributive participle is the Participle of its verb, a `VerbPhrase`
  (`VerbPhrase "tapped" as Modifier of creature`). After *be*, a participle
  keeps both the adjectival (`AdjectivePhrase`) and the verbal passive
  (`VerbPhrase`) reading.
- A noun premodifier is a `Nominal`, including *non-* with a noun (`Nominal
  "nonland"`, `Nominal "noncombat"`); *non-* with an adjective is an
  `AdjectivePhrase` (`nonblack`). A Modifier attaches to the nearest nominal
  it modifies: *Elf* modifies *permanent card*, whose head is *card*.
- A conjunct bears no function of its own; assert the function on the whole
  coordination. Coordinated nominals under a shared postmodifier are a
  `Nominal` (`Nominal "power and toughness"`).
- A multiword term declared as one lexeme is atomic and is named by its
  quoted lemma (`as Modifier of "combat damage"`).
- Full card names are in scope everywhere; `<card=...>` is needed only for a
  Shortened Name or a self-reference by name.
- `*` holds when the text has no Reading for any reason, an undeclared word
  included: an off-card Shortened Name is a valid negative.

## Examples

```
+ Destroy target creature. => NounPhrase "target creature" as Object of destroy
+ Search your library for a card. => PrepositionPhrase "for a card" as Complement of search; !PrepositionPhrase "for a card" as Modifier of library; readings 1
+ Untap all permanents you control during each other player's untap step. => Clause "you control" gap "permanents"; PrepositionPhrase "during each other player's untap step" as Adjunct of untap || PrepositionPhrase "during each other player's untap step" as Adjunct of control
* Destroy creature target.
+NounPhrase target creature with flying => PrepositionPhrase "with flying" as Modifier of creature
```
