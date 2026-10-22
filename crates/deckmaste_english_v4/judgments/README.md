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
does not hold, and reports the others as pending, as it does an entry whose
text is a unit named in [`deferred.txt`](../deferred.txt) while that unit stays
deferred. `seed.txt` is always enforced. Every file is always parsed, so a malformed entry fails at once.

The census uses the same entries: a corpus unit with an entry whose text it
equals, none of whose Readings satisfies that entry, is `known-wrong-only`.

## Syntax

```
# a comment
<mark> [<card=NAME>] <text> [=> <assertions>]
```

- **Mark.** `+` the text is grammatical; `*` it has no Reading at all (see
  [What a negative entry may claim](#what-a-negative-entry-may-claim)).
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
| `... gap "surface"` | and it contains or binds a Gap whose Antecedent spans that surface |
| `!<assertion>` | no Reading at all satisfies the assertion |
| `readings N` | the expression has exactly N Readings |

`LEMMA` is a lemma (`destroy`), a quoted multiword lemma (`"mana value"`) or,
to tell homographs apart, a lexeme id (`action:counter`). Inside a quoted
surface or lemma, `\"` is a literal quotation mark. `FUNCTION` is one of `Subject`, `Object`,
`IndirectObject`, `Complement`, `PredicativeComplement`, `Adjunct`,
`Modifier`, `Determiner` (CGEL functions: a dependent of a clause or verb
phrase that its head does not select is an `Adjunct`, of a nominal a
`Modifier`). `!` and `readings` apply to the entry, wherever they are
written.

A judgment asserts grammaticality, never rank. A `+` entry claims that its
endorsed constituent structure is admitted, and a `!` assertion or a `*` entry
that a form is not. No entry fails because a well-formed alternative ties
with the endorsed Reading or ranks ahead of it: the Ranking Key is a heuristic
traversal and presentation order, not a correctness gate
(`docs/decisions/english-v4.md` §4.4, round 9). Assert a count only when the
count is the claim: exactly one Reading, or exactly the set of analyses each
of which is endorsed. Where English grammar admits a dispreferred analysis
beside the intended one, assert the intended analysis, not that the other
analysis is absent (`docs/decisions/english-v4.md` §4.4).

## What a negative entry may claim

The grammar parses Oracle text into Readings that keep the legitimate
Ambiguity and no more; it does not validate templating. Text that Oracle
would template differently (digits for a spelled number, a capital, a glyph,
a missing comma, *if possible* for *if able*) may parse incidentally, and
that is fine. The grammar does not police strings that no card contains. A `*` entry
therefore claims only the rejection of a wrong inflectional form, which
generation needs to choose the right one: agreement in Person, Number or
Countability (*a creatures*, *You draws cards.*), Case (*Draw cards from
they.*) and verb form (*choose not untap this creature*). A synthetic word
combination that no card contains (*Destroy creature target.*, *up two target
creatures*) gets no entry. A constraint that keeps a spurious Reading off real
Oracle text is witnessed by a `+` entry on that text, with a `!` assertion or
a Reading count. A rule, feature or production that would exist only to reject
some text is never added, and no entry asks for one.

Punctuation that the construction determines is never stored in a Reading.
The separator between the keywords of a keyword line is always a comma: Oracle's
semicolons there exist only beside reminder text, and become commas when it is
removed.

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
- Oracle determinatives are explicit lexical entries: *target*, *enchanted*,
  and *equipped* project a `DeterminerPhrase` in determiner function. Their
  homographs retain their own categories in other uses; an arbitrary participle
  does not acquire a determinative reading. This is an Oracle-register rule.
- An attributive verbal participle is the Participle of its verb, a `VerbPhrase`
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
- An auxiliary verb heads its Verb Phrase and Clause: a Clause's Subject and
  its preposed Adjuncts belong to the auxiliary (`NounPhrase "X" as Subject of
  can` in *X can't be 0*; `as Adjunct of may` in *During your turn, each
  player may pay ...*). A contracted or negative auxiliary is named by its
  verb's lemma (*'s*, *isn't* as *be*; *can't* as *can*).
- A locative after *be* or *remain* is a `Complement`; another Preposition
  Phrase after them is a `PredicativeComplement` (`face down`, `of the chosen
  type`).
- An *as* phrase is a Complement only of a head whose frame selects it
  (*count it as*); otherwise it is a predicative Adjunct, as in *activate only
  as a sorcery*, *enter as a copy of*, *put it onto the stack as a copy*
  (CGEL Ch. 4 §5.3, p. 263).
- A name with a comma-set description (*create Angelo, a legendary ...
  token*) is the name plus a supplement anchored to it: the name bears the
  function, and the supplement is a constituent with no function of its own
  (CGEL Ch. 15 §5, pp. 1350–1351).
- A multiword term declared as one lexeme is one leaf, named by its quoted
  lemma (`as Modifier of "combat damage"`), except where its words straddle a
  constituent boundary (*the first time [counters have been put ...]*).
- Full card names are in scope everywhere; `<card=...>` is needed only for a
  Shortened Name or a self-reference by name.
- `*` holds when the text has no Reading for any reason, an undeclared word
  included: an off-card Shortened Name is a free negative.

## Examples

```
+ Destroy target creature. => NounPhrase "target creature" as Object of destroy
+ Search your library for a card. => PrepositionPhrase "for a card" as Complement of search; !PrepositionPhrase "for a card" as Modifier of library; readings 1
+ Untap all permanents you control during each other player's untap step. => Clause "you control" gap "permanents"; PrepositionPhrase "during each other player's untap step" as Adjunct of untap || PrepositionPhrase "during each other player's untap step" as Adjunct of control
* You draws cards.
+NounPhrase target creature with flying => PrepositionPhrase "with flying" as Modifier of creature
```

The state adjectives `tapped` and `untapped` coexist with the verbal participles.
The premodifier judgments require both analyses; the depictive `enters tapped`
requires the adjective. Transform also explicitly declares the state adjective
`transformed`; the Garland and Startled Awake judgments require its depictive
Adjunct reading. Short focused Objects may be postposed after a PP.
Temporal/manner Objects are not globally forbidden by their game meaning.
The scalar complement in `base power 0` attaches to `power`, with `base`
modifying that headed nominal. `until counter spells` retains the noun-premodifier
reading of `counter spells`, while rejecting a subjectless finite Clause.

Independent genitives agree with the possessed referent, not the possessor.
Quantity nominals use general determination, including `an additional two
creatures`, `another two creatures`, and `every two turns`. Articles select modified quantities; `a two creatures` remains rejected.
Distributive quantities have singular external agreement, while
`another two creatures` retains plural agreement. `each two turns` and
`every two creatures` now assert the quantity nominal instead of rejection.
The Targeting Marker is `det:target`, the last word of its Determiner
(`DeterminerPhrase "target" as Determiner of creature` in `target creature`,
`DeterminerPhrase "two target" as Determiner of creature` in `two target
creatures`); the Target Noun has no attributive noun use. Bare numeric arguments select Amount
complements rather than becoming arbitrary recipient or object Noun Phrases;
numeric subjects and predicatives retain their Noun Phrase analyses.
The `vocabulary_batch_1` judgment for `different kinds of counters` requires
the noun-headed nominal and its selected `of` Complement, without pinning a
separate intermediate `kinds` projection.
