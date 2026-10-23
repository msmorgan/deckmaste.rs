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
4. `Parse::best_readings(k)` returns the `k` best Readings by Ranking Key
   (`RankingKey`, `docs/decisions/english-v4.md` §4.3): Attachment Class,
   then Right Association words, then summed Production Cost, compared in that
   order, with the Reading's own order as the final tie-break, so the best
   Reading is unique. Every component is a sum over the Reading's Productions,
   so each forest node keeps only its `k` best trees and the walk never builds
   every attachment tree. Right Association is computed from spans for every
   unselected dependent (a child named `adjunct`, `modifier` or `supplement`
   after its phrase's `head`): the words of its host, unless the host is a
   coordination, plus the words between the host and it; the last conjunct of
   a coordination adds the difference between its length and its first
   conjunct's. A dependent whose Attachment Class attaches high counts that
   distance reversed: a Supplement (`supplement`), and a dependent the
   grammar's `high_attachment` declares by Category, feature and value (the
   English grammar declares a distributive Preposition Phrase, *for each …*).
   The Attachment Class counts a coordination's
   last conjunct that ends in a dependent its first conjunct, which ends in its
   head, lacks: that dependent belongs to the whole coordination. It also
   counts the second of two dependents before their head (`adjunct` or
   `modifier`): it belongs inside the first where the first can hold it
   (*At the beginning of each combat this turn, …*). The chart
   carries the first conjunct's `head_final` across the coordinator so that
   the last conjunct's item records it. `best_ranked(k)` returns the keys too,
   the tree-side `CompiledGrammar::ranking_key` (feature
   `tree-ranking-key`, enabled for tests only) computes a given Reading's key to
   check the forest's, and `cheapest_tier()` counts the Derivations sharing the least key without
   building trees. `derivation_count()` counts trees without building
   them. `readings()` builds them all and is for tests. A Derivation never
   contains its own Constituent, so a unary cycle adds none of its own and
   hides none of its finite siblings.
   The forest is readable from outside through `PackedForest`'s accessors
   (`roots`, `node_span`, `node_items`, `item_families`, `leaf_span`),
   `Parse::count_node` and the `walk` memo; `deckmaste_english_v4`'s census
   builds its Segments on them.
5. `admit(reading, lexicon)` re-checks a Reading without parsing, through the same
   step function the chart uses. `realize(reading, lexicon)` writes it back out.
   Both round-trip laws are tested on every Reading the tests produce.
   A `Quoted(name, Category)` child is set in quotation marks the engine
   supplies, double and single by nesting depth; the chart pairs the marks of
   the text and reads it in logical order, where a period or comma written
   inside a closing mark follows it and the quotation's own final period is
   not written (`docs/oracle-style-guide.md` §3). Realization writes them back
   from structure, so no quotation punctuation is stored.
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
   is its grammatical function (`predicative_complement` comes from the Frame
   Slot's relation in derived valence phrases); the names the engine inspects
   are listed on `ProductionDecl` in `src/grammar.rs`. A phrase shared by Right Node Raising also bears, in each
   conjunct, the function of the Frame Slot whose Gap it fills
   (`Constituent::also`), as does the head of a function Category it is. A
   `modifier` after its phrase's `object` is postposed out of it, and also
   bears `modifier` in that Object.

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

- `Valence { head, phrase, complement_kind, active, passive, verbal_passive, prepositional_passive, internalised, adjectival, adjectival_passive, obligatory }`: for each frame of each head lexeme
  (verb, preposition, noun, adjective, determinative), a phrase of its head and
  frame slots, recording the Selected Frame. For verbs it also derives every
  object-Gap variant, a predicative-Gap variant of a `Copular` frame only (a
  subjective predicative; never the objective one of *turn it face up*) and,
  when `passive` names the `by` Closed Word, passive
  phrases with and without the Internalised Complement, a `by` Preposition
  Phrase, which strands its preposition like any other. `internalised` equations
  constrain that Preposition Phrase in every derived passive, as the child
  `internalised` (the English grammar admits only *by* + a Noun Phrase that is
  neither a fused-head quantity nor a braced symbol, so *reduced by {2}* is only its selected extent); a `Prepositional` frame's passive strands its
  Preposition Phrase (*each card looked at*). What each derived phrase carries
  is declared, as equations over `$` and its lexical `head`: `active` on every
  active phrase, `verbal_passive` and `prepositional_passive` on every passive
  (required with `passive`), and `adjectival_passive` on every Adjectival
  Passive (required with `adjectival`). The English grammar makes a verb's
  active phrases `Voice: Active` (another head's phrase takes its Voice from
  the head), its passives `Voice: Passive` past participles, and excludes an
  auxiliary from the prepositional passive. When `adjectival` names a Category, a frame whose passive leaves no
  Complement also derives Adjectival Passives of it (the participle, with or
  without its `by` phrase); the English grammar makes them `Voice: Passive`
  and `AdjectivePosition: Predicative`, and excludes a verb marked
  `AdjectivalPassive: Lexical` (a declared Adjective spells it) or `VerbKind:
  Auxiliary`. A preposition with an NP
  complement also strands; an Intransitive Preposition heads its phrase alone.
  With `complement_kind`, a feature its phrase must carry, a phrase bears its
  first Complement's Slot Kind (`NP`, `AdjP`, ...); a non-verbal phrase without
  one is `Intransitive`, and a stranded Preposition Phrase bears the Slot Kind
  of the Complement it strands. The grammar's `slot_equations` constrain every
  Frame Slot child of a kind in a derived phrase, as the child `slot` (the
  English grammar's Noun Phrase Complement is never nominative). `agree` pairs make a phrase's feature agree with that
  feature of each Frame Slot child that carries it (a Preposition Phrase takes
  its Complement's Number; a determinative its oblique's).
  `obligatory: true` keeps the bare phrase to non-verbal heads that declare
  no frame or an `Intransitive` one, so another head realizes its frame (a
  noun in a fossilised Preposition Phrase: *until end of turn*, *in excess
  of*, but *on top*). A non-verbal head's `Intransitive` frame derives no
  phrase of its own: the bare phrase is it.
  With `clusters`, a verb with two or more Frame Slots also heads a coordination
  of their sequences, its head shared on the left as the mirror of Right Node
  Raising (*put one of them into your hand and the rest into your graveyard*,
  *deals 2 damage to any target and 1 damage to you*); each copy of a Slot
  bears its function (right nonce-constituent coordination, CGEL, Ch. 15
  §4.3, p. 1341).
  A `MeasureNP` slot is a Monotransitive Object filled by the grammar's
  measure Noun Phrase Category, which no `NP` slot accepts (*gets +2/+2*, but
  not *draw 2/2*); like a keyword-phrase Object it derives no passive.
  Amount and Cost slots (*scry 2*, *waterbend {3}*) are Complements, never
  objects, so they derive no passive or Gap. An `Idiom` frame fixes a
  preposition among its Complements (*come up heads*). A frame marked
  `active_only` (an Active-only Frame: *cost*, lexical *have*) derives no
  passive of any kind: verbal, prepositional or adjectival. A verb whose every
  frame is Active-only has no past participle in a `Premodification` either. A
  `Premodification`'s `equations` hold of the modifier one of its `forms`
  heads, over its lexical `head` (the English grammar's past participle is
  not one a declared Adjective spells). A
  `Premodification` passes up its head's Gap, so a premodified conjunct shares
  its Complement (*the noted number and kind of counters*).
- `Adjunct { host, adjunct, side, host_kind, equations }`: unselected attachment, limited to
  Preposition Phrases whose (preposition, complement frame) entry admits that Host
  Category: a Nominal (as `modifier`), a Predicate (as `adjunct`), or a Clause
  or line that a comma-set Supplement is anchored to (as `supplement`, *, where
  X is its power*). Same-head complement priority is derived from the frames: a PP, or an
  adjunct of a Frame Slot's Category (a purpose *to*-infinitival), may not
  attach as a Predicate adjunct to a head that has a longer frame selecting it
  (*choose to attack* has no purpose reading). A noun's bare phrase omits the
  Complements its frames select, and a Preposition Phrase right after it headed
  by one of their prepositions is that noun's Complement, never a dependent of
  another host (*on top of your library*, *equal to the number of cards in
  their hand*): every constituent ending in a bare noun passes its omitted
  Complements on to the sibling that follows it.
  A delimited (comma-set) post-adjunct's host passes up no Gap, so it never
  closes a relative clause's Clause (CGEL, Ch. 12 §4, p. 1058).
  Likewise a frame's predicative
  Complement is not a Preposition Phrase that a rival frame of the same head
  selects in its place (locative *on the battlefield* after *be*). Optional
  `equations` constrain the `head` (host) and the adjunct (a depictive
  gerund-participial of a lexical verb only).
- `Coordination { category, coordinators, unshared, equations, proximate, resolve, delimiter, delimited, serial }`:
  binary (only with the delimiter when `delimited`) and Oxford-comma serial
  coordination (`serial` overrides the default: like conjuncts only; a plain serial ends with
  one of its own coordinators), a delimited polysyndetic chain
  (*A, or B, or C*), one tree per same-coordinator chain, across-the-board Gaps, and Right Node Raising (a coordination with a
  shared NP, keyword-phrase, *to*-infinitival, predicative or catenative bare-infinitival Gap, a
  preposition's Amount Complement, or a noun's single-preposition
  Preposition Phrase Complement headed by that preposition (*the top or bottom of their library*),
  or nouns' Amount Complements filled respectively by a slash Measure (*base power and toughness 4/4*)
  or all by one Amount (*power or toughness 2 or less*),
  filled once after it, derived for a Category's first
  coordination schema). Conjuncts agree on every feature but the `unshared`
  ones, which the coordination leaves unvalued, and the `proximate` ones, which
  it takes from its last conjunct (*one or more cards*). Every conjunct but
  the last shares a `resolve` feature, and the coordination takes that value
  with the last conjunct's, or the listed value when they differ (a
  coordination of Noun Phrases is determined only when every conjunct is). `equations` over `$`
  resolve the whole coordination: a feature they value is still shared by the
  conjuncts but not by the coordination (two singular nominals joined by *and*
  are plural); equations over `first` hold of every conjunct but the last, over
  `last` of the last (imperatives followed by a declarative); an
  `ExcludeTogether` equation over `first` and `last` relates the last conjunct
  to the one before it. A `delimiter` may
  also precede the coordinator of a binary coordination (*X, and Y*). The
  grammar's `correlatives` pair a marker with a coordinator (*both … and*);
  every schema whose coordinators admit the paired one also derives the
  coordination after the marker. A coordination of Preposition Phrases keeps
  up to two prepositions for selection, and admits only the Host Categories
  every conjunct admits. A coordination of a function may name its `conjunct`
  Category, and `unlike` a feature its two conjuncts must not share: unlike
  categories coordinated in one function (a depictive *tapped and attacking*),
  binary unless `serial` is set (an *and*-list of an Object with keywords). A constituent that begins with a dependent set off by
  a comma (*When …, …*) is never the initial conjunct of a coordination
  undelimited before its coordinator: the preposed dependent scopes over the
  whole coordination. The initial conjunct of a coordination delimited before
  its coordinator may have its own (*[If you do, …], and [if you don't, …]*),
  and so may a middle conjunct of a serial one, whose serial tail names it
  `first` too (*C1, [if …, C2], and C3*).
  Nor is such a constituent the host of a comma-set post-dependent (a head
  followed by a comma and its dependent): the fronted dependent attaches
  outermost, *Pre, [Clause, Post]*.
  A coordination delimited before its coordinator (*A, and B*) and the host of
  a comma-set post-dependent contain a comma that runs to the end of the
  sentence, and every phrase that contains one carries it up
  (`Summary::internal_comma`). A dependent set off before its head by a comma
  ends at that comma, so it never contains one: an intervening *if* follows
  the trigger condition rather than sitting inside it (*When [this creature
  dies], if …*). A dependent after its head may contain one (*unless [its
  controller pays {X}, where X is …]*, *except [A, and B]*).

Gaps pass from a child to its mother only through children the Production marks
`transparent`; auxiliaries mark their verb-phrase complement transparent, so
extraction works across finite, plain, progressive, perfect and passive forms with
no per-form Productions.

A clause Frame Slot is finite (*when this creature enters*), present-tense
finite (*whenever you cast a spell*), verbless (*if able*) or interrogative
(*choose which creatures attack*), each filled by its own Category.

A Gap is an `NP` (an extracted Object or Complement of a preposition), a `Pred`
(an extracted predicative Complement, *whose turn it is*) or an `Adjunct` (*the
next time you cast a spell*). Valence introduces `NP` and `Pred` Gaps. The
extracting verb is a free word, since a reduced clitic such as *'s* never
stands before a Gap (CGEL, Ch. 18 §6.2.1, p. 1615), and a `Pred` Gap's verb is
finite (never *can't be \_\_*). An authored Production introduces one with `Require("$.Gap", ..)` (a subject Gap).
`Require("child.Gap", ..)` fills the child's Gap within the mother, unless the
child is also `transparent`: then the mother takes that Gap over (a purpose
adjunct whose object a relative clause binds).

## The lexicon-to-syntax interface

The engine never names a grammar feature, value or Category
(`docs/decisions/english-v4.md` §4.4, principle 6), with one exception: the lexicon
feature-bundle mapping table, the single declared interface through which
lexical facts reach the grammar's features. It is `PROJECTED` in
`compile.rs` (the grammar features the engine projects onto leaves: `Number`,
`Person`, `Tense`, `VerbForm`, `Case`, `Inflection`, `Countability`, `Sign`),
`projected_values` and `inflection_name` in `step.rs` (the values each takes),
and `leaf_facts` in `step.rs`, which projects a word's inflection (its Word
Form onto `VerbForm` and `Inflection`), a lexeme's Countability, and a
numeral's `Number` and `Sign`. A grammar that declares one of those features
must include every value projected onto it. A lexeme's own feature bundle
reaches the grammar by the names it declares. Everything else a derivation
carries, the grammar declares: Schema parameters (a Valence schema's voice
and passive equations and `complement_kind`, a Premodification's
`equations`) and grammar-level roles (`slot_equations`, `high_attachment`).
A grammar without such a declaration gets no such derivation or a load
error, never a default. Purely structural `Summary` bits, the engine's own
Gap kinds and its reserved child names are not grammar names.

## Lexical classes and notation

`typed_classes` names classes whose membership comes from the lexical
environment's declared properties, including Catalog entries. Empty and
singleton classes are reported. The grammar never enumerates their members.
Binding reports features the same way (`english-lexical-analysis.md` §3,
`english-v4.md` §4.4, principle 3): a feature with no value, and a value with fewer than
two holders. A feature may declare one, privative, value, which is reported
only by its holders. A value's holders are the lexemes that carry it and those
that hold it through a Production setting it on its mother: the lexemes its
lexical `head` names (a Closed Word, a lifted Lexical Class, a keyword's
Parameter pattern, the lexemes declaring a Frame), or else the Production
itself. A value with neither is held by the Productions that require it of a
child. A value projected from inflection is held by every word form inflected
for it. Features equated with one another share their holders, and such a
value is reported once, naming them all. Each warning names its holders.
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
imperative Clauses). A child can be `unextended`: it does not end, through
its head line, in a Preposition Phrase dependent that its Production, or a
sibling with the same children up to and including it, selects as a `PP`
Complement right after it (an existential's Extension is a Complement of
*be*, never also the Displaced Subject's postmodifier). A verb without
a Subcategorization Frame binds when the grammar names it as a Closed Word or a
Lexical Class member (perfect *have* and its forms).

A Production whose last child is a function Category aliasing its own Category
builds an auxiliary phrase: that child is the participial Verb Phrase
Complement of perfect *have* or of passive or progressive *be*, whereas a
modal's bare infinitival is a Category of its own. When such a phrase is
`VerbKind: Auxiliary` and not a coordination, it is never the head of a
Production of its own Category whose later children are all `adjunct`s: a
post-verbal Adjunct attaches to the lexical participial Verb Phrase inside it
(*was [put there from the battlefield this turn]*, *have [lost this turn]*), so
the duplicate site above it is never built (`english-lexical-analysis.md` §2).
A coordination of auxiliary Verb Phrases still hosts an Adjunct its conjuncts
share.

## Reading Normal Form

`normal_form(reading)` identifies only the approved canonicalizations the
engine derives: every grouping of a chain joined by one coordinator becomes one
node listing its conjuncts and then the coordinator (with no features of its
own); a Production over a coordination of its `uncoordinated` child becomes
the coordination of that Production over each conjunct (clause versus
verb-phrase coordination of bare imperatives); an Adjunct Production over an
uncoordinated auxiliary Verb Phrase becomes the auxiliary over that Production
of its participial Complement's Verb Phrase (adjuncts below a non-modal
auxiliary); a Preposition Phrase attached as a Predicate adjunct to a head
with a longer frame selecting it becomes that frame's complement; and a
comma-set post-dependent of a Clause that begins with a fronted dependent
becomes a post-dependent of that Clause's head, *Pre, [Clause, Post]* for
*[Pre, Clause], Post* (fronted dependent outermost). It endorses
nothing; it lets two sets of Readings be compared under exactly those
equivalences.

Category aliases retain a grammatical category in Readings while allowing
separate internal production domains. Declared lexical features participate in
the same equations as inflectional features. `Require` values an unvalued
feature; `Carry` requires the child to carry the value itself; `ExcludeTogether`
rejects only a combination of child values, every one of them carried. An
equation on a child name that several children share (two preposed `adjunct`s)
constrains each of them that carries the feature. Multiword noun terms, measured compounds
(`+1/+1 counter`) among them, are one leaf in both parsing and Admission.
The ordinary words of one declared outside the core lexicon also reach the
chart as separate leaves, but only to straddle a constituent boundary, where
the smallest Constituent spanning the term has its first word in a child that
begins before the term and its last word in a child that ends after it.
Admission checks the same rule on the Reading's own Constituents, without
parsing. Construction boundaries enforce the spacing used by realization.

The Valence Schema can interleave PP Adjuncts before selected complements and
postpose an Object. With `postposed`, a phrase whose Object is followed only by
selected PP complements also ends in a `modifier` of that function Category,
postposed out of the Object past them and agreeing with it in each feature
both carry (*put a number of counters on it equal to …*; CGEL, Ch. 12 §4,
p. 1066); it is never interleaved. Same-head complement priority still applies through these
projections; a coordination whose conjuncts take the same Complements keeps
them as its extensions, so a Complement they share is never an Adjunct of the
coordination. Right Node Raising requires the shared Gap at each coordinate's
right edge; ordinary extraction is not subject to that restriction. A
gap-capable Category's `closed_gaps` names Gap kinds its Constituents never
contain: a non-finite Complement passes up no predicative Gap (*can't be __*),
so one is shared only inside it (*can't [be and become] legendary*). Adjunct
Schemas can declare a punctuation delimiter for preposed or delimited PPs.
