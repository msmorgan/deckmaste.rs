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
   a predicate). A phrase shared by Right Node Raising also bears, in each
   conjunct, the function of the Frame Slot whose Gap it fills
   (`Constituent::also`), as does the head of a function Category it is.

## Chart and realization costs

Binding computes transitive left-corner terminal masks for Categories and
Productions, including recursive Categories. Parsing builds one active terminal
mask per chart position from the independent lexical analysis and punctuation;
only Productions whose first terminals can match are predicted. Whitespace
marks use the unskipped position, preserving structural line boundaries.

Prediction stamps and waiting-parent lists use flat indexes. Item deduplication
is partitioned by end position and hashes internal integer identities with a
fixed mixer. State and Summary feature slots occupy one byte each; binding
rejects feature domains larger than 255 values, reserving one byte value for an
unbound slot. These representations retain the same grammatical distinctions.

Packed realization verification stores its four possible boundary behaviours
in a bitmask. Each reachable leaf is checked once per verification, including
when unary cycles prevent memoizing a surrounding node. Successful spelling
checks compare characters without allocating another source string.

## Schemas

Authors write active Subcategorization Frames once; the engine derives the rest.

- `Valence { head, phrase, passive, adjectival }`: for each frame of each head lexeme
  (verb, preposition, noun, adjective, determinative), a phrase of its head and
  frame slots, recording the Selected Frame. For verbs it also derives every
  object-Gap variant, a predicative-Gap variant of a `Copular` frame only (a
  subjective predicative; never the objective one of *turn it face up*) and,
  when `passive` names the `by` Closed Word, passive
  phrases with and without the Internalised Complement, a `by` Preposition
  Phrase, which strands its preposition like any other; a non-auxiliary `Prepositional` frame's passive strands its
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
  With `clusters`, a verb with two or more Frame Slots also heads a coordination
  of their sequences, its head shared on the left as the mirror of Right Node
  Raising (*put one of them into your hand and the rest into your graveyard*,
  *deals 2 damage to any target and 1 damage to you*); each copy of a Slot
  bears its function (right nonce-constituent coordination, CGEL, Ch. 15
  §4.3, p. 1341).
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
  (*choose to attack* has no purpose reading).
  Likewise a frame's predicative
  Complement is not a Preposition Phrase that a rival frame of the same head
  selects in its place (locative *on the battlefield* after *be*). Optional
  `equations` constrain the `head` (host) and the adjunct (a depictive
  gerund-participial of a lexical verb only).
- `Coordination { category, coordinators, unshared, equations, proximate, delimiter }`:
  binary and Oxford-comma serial coordination, a delimited polysyndetic chain
  (*A, or B, or C*), one tree per same-coordinator chain, across-the-board Gaps, and Right Node Raising (a coordination with a
  shared NP, keyword-phrase, *to*-infinitival or predicative Gap, filled once after it, derived for a Category's first
  coordination schema). Conjuncts agree on every feature but the `unshared`
  ones, which the coordination leaves unvalued, and the `proximate` ones, which
  it takes from its last conjunct (*one or more cards*). `equations` over `$`
  resolve the whole coordination: a feature they value is still shared by the
  conjuncts but not by the coordination (two singular nominals joined by *and*
  are plural); equations over `first` hold of every conjunct but the last, over
  `last` of the last (imperatives followed by a declarative). A `delimiter` may
  also precede the coordinator of a binary coordination (*X, and Y*). The
  grammar's `correlatives` pair a marker with a coordinator (*both … and*);
  every schema whose coordinators admit the paired one also derives the
  coordination after the marker. A coordination of Preposition Phrases keeps
  up to two prepositions for selection, and admits only the Host Categories
  every conjunct admits. A coordination of a function may name its `conjunct`
  Category, and `unlike` a feature its two conjuncts must not share: unlike
  categories coordinated in one function (a depictive *tapped and attacking*),
  binary only. A constituent that begins with a dependent set off by
  a comma (*When …, …*) is never a conjunct other than the last: the preposed
  dependent scopes over the whole coordination.

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
in the catalog. With `coordinators`, a pattern's final marker and slot also
coordinate, binary and serial, the marker repeated (*protection from black and
from red*). Noun notation parameters support indexed headers by the same
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
clause, or `uncoordinated`, not itself a coordination, where the coordination
one level up is the same analysis (a chain of bare imperatives is coordinated
imperative Clauses). A verb without
a Subcategorization Frame binds when the grammar names it as a Closed Word or a
Lexical Class member (perfect *have* and its forms).

## Reading Normal Form

`normal_form(reading)` identifies only the approved canonicalizations the
engine derives: every grouping of a chain joined by one coordinator becomes one
node listing its conjuncts and then the coordinator (with no features of its
own); a Production over a coordination of its `uncoordinated` child becomes
the coordination of that Production over each conjunct (clause versus
verb-phrase coordination of bare imperatives); and a Preposition Phrase
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
right edge; ordinary extraction is not subject to that restriction. A
gap-capable Category's `closed_gaps` names Gap kinds its Constituents never
contain: a non-finite Complement passes up no predicative Gap (*can't be __*),
so one is shared only inside it (*can't [be and become] legendary*). Adjunct
Schemas can declare a punctuation delimiter for preposed or delimited PPs.
