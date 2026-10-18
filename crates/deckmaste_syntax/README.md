# deckmaste_syntax

The unversioned, data-driven syntax engine of English v4
([`docs/decisions/english-v4.md`](../../docs/decisions/english-v4.md),
[`docs/decisions/english-lexical-analysis.md`](../../docs/decisions/english-lexical-analysis.md)).
It has no proc macros and no generated code: a grammar is data.

## Data flow

1. `Grammar::from_ron_str` reads a grammar data file of Productions and Schemas.
2. `CompiledGrammar::bind(grammar, lexicon, frames)` binds it to a
   `deckmaste_lexical::Lexicon` and a `FrameCatalog` of Subcategorization Frames.
   Binding validates every name, rejects policing feature names (`…Use`,
   `…Licence`, `…License`, `…Present`, `…Restriction`), and rejects any Closed Word
   or explicitly named Lexical Class member that is not a core closed-class lexeme, so catalog
   vocabulary such as keyword actions can never appear in a grammar.
3. `parse(lexicon, text)` runs an Earley chart over the lexical analysis of the
   text and returns a `Parse`: the Packed Forest, its `Metrics`, and vocabulary
   gaps. Constituents are packed by (Category, span, summary); a summary holds the
   Category's features, its Gap, and the engine-derived facts a parent may check.
4. `Parse::best_readings(k)` returns the `k` cheapest Readings by total
   Production Cost, keeping only `k` trees per forest node, so it never builds
   every attachment tree; `derivation_count()` counts trees without building
   them. `readings()` builds them all and is for tests. A Derivation never
   contains its own Constituent, so a unary cycle adds none of its own and
   hides none of its finite siblings.
5. `admit(reading, lexicon)` re-checks a Reading without parsing, through the same
   step function the chart uses. `realize(reading, lexicon)` writes it back out.
   Both round-trip laws are tested on every Reading the tests produce.
6. `parse_analyzed(analyzed)` parses a text already analyzed (for example with a
   card's names in scope); `parse_category(analyzed, category)` parses it as any
   Category, and `Parse::admit` admits at the Category the Parse was made for.
7. Both round-trip laws are checked without enumerating trees:
   `Parse::verify_realization(lexicon)` proves law 1 for every Reading in the
   forest by checking each reachable leaf's spelling and the spacing of every
   pair of atoms adjacent in some Derivation; `Parse::contains(reading)` decides
   law 2's membership by walking the forest along the Reading.
8. `constituents(reading, lexicon)` lists a Reading's Constituents in preorder
   with their spans in its realization, their mother, their head lexeme (through
   `head` children) and their name in the mother's Production. A child's name
   is its grammatical function: `subject`, `head`, `object`, `complement`,
   `predicative_complement` (from the Frame Slot's relation in derived valence
   phrases), `modifier` (an unselected dependent of a nominal) or `adjunct` (of
   a predicate).

## Schemas

Authors write active Subcategorization Frames once; the engine derives the rest.

- `Valence { head, phrase, passive, adjectival }`: for each frame of each head lexeme
  (verb, preposition, noun, adjective, determinative), a phrase of its head and
  frame slots, recording the Selected Frame. For verbs it also derives every
  object-Gap variant, a predicative-Gap variant of a `Copular` frame only (a
  subjective predicative; never the objective one of *turn it face up*) and,
  when `passive` names the `by` Closed Word, passive
  phrases with and without the Internalised Complement, a `by` Preposition
  Phrase; a non-auxiliary `Prepositional` frame's passive strands its
  Preposition Phrase (*each card looked at*). Only a verb's phrases are `Voice: Active`; another head's phrase
  takes its Voice from the head (a declared Adjectival Passive is Passive). When `adjectival` names a Category, a frame whose passive leaves no
  Complement also derives Adjectival Passives of it (the participle, with or
  without its `by` phrase), `Voice: Passive` and `AdjectivePosition:
  Predicative`; a verb marked `AdjectivalPassive: Lexical` (a declared Adjective
  spells it) or `VerbKind: Auxiliary` derives none. A preposition with an NP
  complement also strands; an Intransitive Preposition heads its phrase alone.
  Where the phrase's Category carries `ComplementKind`, a phrase bears its
  Complement's Slot Kind (`NP`, `AdjP`, ...); a non-verbal phrase without one
  is `Intransitive`. `agree` pairs make a phrase's feature agree with that
  feature of each Frame Slot child that carries it (a Preposition Phrase takes
  its Complement's Number; a determinative its oblique's).
  Amount and Cost slots (*scry 2*, *waterbend {3}*) are Complements, never
  objects, so they derive no passive or Gap. An `Idiom` frame fixes a
  preposition among its Complements (*come up heads*).
- `Adjunct { host, adjunct, side, host_kind, equations }`: unselected attachment, limited to
  Preposition Phrases whose (preposition, complement frame) entry admits that Host
  Category: a Nominal (as `modifier`), a Predicate (as `adjunct`), or a Clause
  or line that a comma-set Supplement is anchored to (as `supplement`, *, where
  X is its power*). Same-head complement priority is derived from the frames: a PP, or an
  adjunct of a Frame Slot's Category (a purpose *to*-infinitival), may not
  attach as a Predicate adjunct to a head that has a longer frame selecting it
  (*choose to attack* has no purpose reading). Likewise a frame's predicative
  Complement is not a Preposition Phrase that a rival frame of the same head
  selects in its place (locative *on the battlefield* after *be*). Optional
  `equations` constrain the `head` (host) and the adjunct (a depictive
  gerund-participial of a lexical verb only).
- `Coordination { category, coordinators, unshared }`: binary and Oxford-comma serial
  coordination, one tree per same-coordinator chain, across-the-board Gaps, and
  Right Node Raising (a coordination with a shared NP Gap, filled once after it).
  Conjuncts agree on every feature but the `unshared` ones, which the
  coordination leaves unvalued.

Gaps pass from a child to its mother only through children the Production marks
`transparent`; auxiliaries mark their verb-phrase complement transparent, so
extraction works across finite, plain, progressive, perfect and passive forms with
no per-form Productions.

A clause Frame Slot is finite (*when this creature enters*), verbless (*if
able*) or interrogative (*choose which creatures attack*), each filled by its
own Category.

A Gap is an `NP` (an extracted Object or Complement of a preposition), a `Pred`
(an extracted predicative Complement, *whose turn it is*) or an `Adjunct` (*the
next time you cast a spell*). Valence introduces `NP` and `Pred` Gaps. The
extracting verb is a free word, since a reduced clitic such as *'s* never
stands before a Gap (CGEL, Ch. 18 §6.2.1, p. 1615), and a `Pred` Gap's verb is
finite (never *can't be \_\_*). An authored Production introduces one with `Require("$.Gap", ..)` (a subject Gap).
`Require("child.Gap", ..)` fills the child's Gap within the mother, unless the
child is also `transparent`: then the mother takes that Gap over (a purpose
adjunct whose object a relative clause binds).

## Lexical classes and notation

`typed_classes` names classes whose membership comes from the lexical
environment's declared properties, including Catalog entries. Empty and
singleton classes are reported. The grammar never enumerates their members.
`Parameters` compiles the catalog's ordered parameter patterns around a head:
slots select grammar Categories; markers, bound stems and head identities stay
in the catalog. Noun notation parameters support indexed headers by the same
mechanism as keyword parameters. `ModifiedWords` builds a restricted word set with recursive
premodification inside its projected category.

`Selected` names a Preposition Phrase child headed by one of the listed
Closed Words, as a `PP([...])` Frame Slot selects one (*up to two*, a partitive
*of* oblique). An `AdvP` Frame Slot is an adverbial Complement (*more than
twice*).

`Mark` specifies structural punctuation and its attachment at either edge;
`Attached` attaches a constituent at its outer boundaries, for adjacent symbol
sequences; `Free` requires a constituent whose first atom does not glue to the
one before (a Verb Phrase that does not begin with a bound clitic). These bindings are applied by both realization and the packed-forest
law checker. Paragraph boundaries are explicit terminals, never skipped as
interword whitespace. `Numeral` selects numeral notation; `FreeLex` excludes bound stems
from independent word positions. Bare valence phrases likewise require free words.
`Sign`, when declared, projects a numeral’s Negative, Zero, or Positive value
onto its lexical leaf. Grammars can constrain signed notation without inspecting
source spelling or changing lexical analysis.
Flavor Word leaves use the lexical layer's registered-label exclusion and realization.

## Function Categories and frameless auxiliaries

A Category declared `function: true` names a grammatical function rather than a
constituent, as `Predicative` names the predicative Complement: in
`constituents`, its head child bears the same function in the same mother. A
Production can require that a Preposition Phrase child admit a Host Category
(`Hosted`), as a predicative PP must be able to modify a nominal. It can also
require a child to be `light`, one lexical leaf alone or projected, or `heavy`,
more than one, as a single postpositive participle differs from a participial
clause. A verb without
a Subcategorization Frame binds when the grammar names it as a Closed Word or a
Lexical Class member (perfect *have* and its forms).

## Reading Normal Form

`normal_form(reading)` identifies only the approved canonicalizations the
engine derives: every grouping of a chain joined by one coordinator becomes one
node listing its conjuncts and then the coordinator, and a Preposition Phrase
attached as a Predicate adjunct to a head with a longer frame selecting it
becomes that frame's complement. It endorses nothing; it lets two sets of
Readings be compared under exactly those equivalences.

Category aliases retain a grammatical category in Readings while allowing
separate internal production domains. Declared lexical features participate in
the same equations as inflectional features. `Require` values an unvalued
feature; `Carry` requires the child to carry the value itself. Multiword noun terms remain atomic
in both parsing and Admission; measured compounds retain their internal measure.
Construction boundaries enforce the spacing used by realization.

The Valence Schema can interleave PP Adjuncts before selected complements and
postpose an Object. Same-head complement priority still applies through these
projections. Right Node Raising requires the shared Gap at each coordinate's
right edge; ordinary extraction is not subject to that restriction. Adjunct
Schemas can declare a punctuation delimiter for preposed or delimited PPs.
