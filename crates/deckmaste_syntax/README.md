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

- `Valence { head, phrase, passive }`: for each frame of each head lexeme
  (verb, preposition, noun, adjective, determinative), a phrase of its head and
  frame slots, recording the Selected Frame. For verbs it also derives every
  object-Gap variant and, when `passive` names the `by` Closed Word, passive
  phrases with and without the Internalised Complement. A preposition with an NP
  complement also strands; an Intransitive Preposition heads its phrase alone.
  Amount and Cost slots (*scry 2*, *waterbend {3}*) are Complements, never
  objects, so they derive no passive or Gap.
- `Adjunct { host, adjunct, side, host_kind }`: unselected attachment, limited to
  Preposition Phrases whose (preposition, complement frame) entry admits that Host
  Category. Same-head complement priority is derived from the frames: a PP may not
  attach as a Predicate adjunct to a head that has a longer frame selecting it.
- `Coordination { category, coordinators }`: binary and Oxford-comma serial
  coordination, one tree per same-coordinator chain, across-the-board Gaps, and
  Right Node Raising (a coordination with a shared NP Gap, filled once after it).

Gaps pass from a child to its mother only through children the Production marks
`transparent`; auxiliaries mark their verb-phrase complement transparent, so
extraction works across finite, plain, progressive, perfect and passive forms with
no per-form Productions.

## Lexical classes and notation

`typed_classes` names classes whose membership comes from the lexical
environment's declared properties, including Catalog entries. Empty and
singleton classes are reported. The grammar never enumerates their members.
`Parameters` compiles the catalog's ordered parameter patterns around a head:
slots select grammar Categories; markers, bound stems and head identities stay
in the catalog. Noun notation parameters support indexed headers by the same
mechanism as keyword parameters. `ModifiedWords` builds a restricted word set with recursive
premodification inside its projected category.

`Mark` specifies structural punctuation and its attachment at either edge;
`Attached` attaches a constituent at its outer boundaries, for adjacent symbol
sequences. These bindings are applied by both realization and the packed-forest
law checker. Paragraph boundaries are explicit terminals, never skipped as
interword whitespace. `Numeral` selects numeral notation; `FreeLex` excludes bound stems
from independent word positions. Bare valence phrases likewise require free words.
`Sign`, when declared, projects a numeral’s Negative, Zero, or Positive value
onto its lexical leaf. Grammars can constrain signed notation without inspecting
source spelling or changing lexical analysis.
Flavor Word leaves use the lexical layer's registered-label exclusion and realization.

## Reading Normal Form

`normal_form(reading)` identifies only the approved canonicalizations the
engine derives: every grouping of a chain joined by one coordinator becomes one
node listing its conjuncts and then the coordinator, and a Preposition Phrase
attached as a Predicate adjunct to a head with a longer frame selecting it
becomes that frame's complement. It endorses nothing; it lets two sets of
Readings be compared under exactly those equivalences.

Category aliases retain a grammatical category in Readings while allowing
separate internal production domains. Declared lexical features participate in
the same equations as inflectional features. Multiword noun terms remain atomic
in both parsing and Admission; measured compounds retain their internal measure.
Construction boundaries enforce the spacing used by realization.

The Valence Schema can interleave PP Adjuncts before selected complements and
postpose an Object. Same-head complement priority still applies through these
projections. Right Node Raising requires the shared Gap at each coordinate's
right edge; ordinary extraction is not subject to that restriction. Adjunct
Schemas can declare a punctuation delimiter for preposed or delimited PPs.
