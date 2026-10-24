# English grammar: lexical analysis, Readings, and how the grammar is kept small

Updated 2026-10-14 for `deckmaste_syntax` and `deckmaste_english_v4` (paired with [`docs/decisions/english-v4.md`](./english-v4.md)). It has no amendment log: when a rule changes, this text changes. The syntax layer (`deckmaste_syntax` and `deckmaste_english_v4`) has no dependency on Game Model semantics; downstream CxG spelling and `deckmaste_semantics_v3` consume its packed parse forest.

## 1. The engine contract

**Lexical analysis is independent of the grammar.** Input is analysed without
production-directed scanning. Each alternative identifies a declared Lexeme,
its lexical Category, Word Form, correlated grammatical features and its
provenance. Analyses come only from declared vocabulary and its default or
overridden morphology; an unknown word is reported as a vocabulary gap, never
guessed. Core vocabulary, plugin declarations and catalogs feed one analysis
interface. Analysis and realization share the same inflection defaults and
overrides.

**Admission works on finite summaries.** Parsing is Earley-family chart
parsing. Chart completion operates on grammatical summaries, never on products
of full child values; a full tree is built only when a Reading is requested
from the forest. Packing preserves every distinction that can affect a
parent's admissibility and keeps correlated alternatives correlated, for
incomplete derivations as well as completed constituents. A deferred validity
check is part of admission: an unchecked forest path is not a Reading.

**Both round-trip laws hold**, with the same parse context and lexical
environment:

```
v is an admitted Reading of s                      =>  render(v) = s
v is independently constructed and grammatical     =>  v is a Reading of parse(render(v))
```

Equality in the second law preserves grammatical structure, lexical identities,
features and retained spelling variants; computed source positions are not
identity. No source-buffer echo or opaque catch-all leaf counts as realization.
Punctuation may decide an analysis, but its presence is not stored in a Reading;
realization derives it from structure. This includes the separator in a
keyword-ability list, which is always a comma once reminder text is removed
(see below), and the punctuation of a quotation: its marks are double, and
single inside a double one, and the period or comma after it is written inside
its closing mark, where its own final period is not written
(`docs/oracle-style-guide.md` §3, Periods; Quotation marks and apostrophes),
and the period of a sentence that ends in a word whose spelling ends in
terminal punctuation, which is not written either (*Exile Blood for the Blood
God!*; OSG §3, "Semicolons and expressive punctuation").
The square brackets of Bracketed text, which a cleave spell removes
[CR#702.148a], are no Constituent and are not parsed at all: a line of a
cleave face is read as its Uncleaved Text, without the brackets, which are
kept beside the text and written back from there, and as its Cleaved Text,
without the Bracketed text, each by the ordinary grammar
(`docs/oracle-style-guide.md` §3, Bullets, vertical bars, and square
brackets). The grammar's only square brackets are loyalty costs.

**Facts about the input.** The supported corpus is the Vintage-playable cards
(legal or restricted), excluding Scryfall `set_type: funny`. Balanced
parenthesised reminder text is removed before analysis; the original Oracle text
is kept as source metadata. Oracle separates the keywords of a keyword line with
semicolons only because one of them carries reminder text, so on a line that
loses reminder text and holds no sentence, those semicolons become commas. A
face has a Shortened Name only where its own text refers to it by one
[CR#201.5c], never a shortening that spells a token name or an Introduced Name
in scope on that face (OSG §5, "do not invent a shortening that could denote
something else"). A face's Shortened Name is in scope only on that face's own
text [CR#201.5c,712.8d..712.8f]; full names are in scope everywhere. Where a face's
own name and another card's full name are spelled alike, both are Readings, and
the face's own name ranks first ([`english-v4.md`](./english-v4.md) §4.3). The
census enumerates, validates, counts and digests per unit below the face, and a
face's reported count is a sum over its units.

## 2. What a Reading is

A Reading is one whole tree enumerated from the packed forest. Two Readings are
the same only if they are the same tree.

Multiple Readings are successful parsing. The Ranking Key
([`english-v4.md`](./english-v4.md) §4.3) may rank Readings for presentation;
it never decides admission and never removes one.

**The grammar avoids producing distinct trees for the same grammatical analysis
of the text.** Twelve canonicalizations apply:

- the order in which commuting pre- and post-modifiers are applied to one
  nominal head (from a stated, reviewed list; anything unlisted is a barrier);
- the grouping within a chain joined by the same coordinator, or linked by
  the same connective (*A, then B, then C* is one chain, never *[A, then B],
  then C* nor *A, then [B, then C]*);
- clause versus verb-phrase coordination of bare imperatives, where nothing
  distinguishes them;
- **bare coordination:** a coordination of bare Noun Phrases with no
  determiner and no dependent to share (*creatures and lands*, *damage and
  life*, *devotion to [white and black]*, *choose [black or red]*) is a
  coordination of Noun Phrases, never also one bare Noun Phrase over a
  coordination of Nominals: the two differ in no meaning. A coordination is
  of Nominals only where something outside it is shared, as the determiner
  *her* is outside *her [son and younger daughter]* (CGEL, Ch. 15 §2.1,
  p. 1291): *all [creatures and lands]*, *[creatures and lands] you control*.
  The canonical site is built structurally: a bare Noun Phrase's Nominal is no
  coordination (BarePlural and BareMass, `uncoordinated`), though it may be a
  modified one;
- **adverbs before a bare imperative:** a bare imperative has no Subject, so
  its front and central positions coincide. An adverb before it is the front
  Adjunct of its Clause, outermost over its Verb Phrase and that phrase's
  dependents (*Also [put a +1/+1 counter on that creature if …]*, *Then [draw
  a card and discard a card]*), never also a central Adjunct of the Verb
  Phrase, below or above its post-verbal dependents: the sites differ in no
  meaning, and the front one is the one a coordination of bare imperatives,
  coordinated Clauses, also has. An adverb found there therefore has a front
  position (*secretly*, *simultaneously*). The canonical site is built
  structurally: a Verb Phrase that begins with a central Adjunct heads no
  imperative, and a coordination begins where its first conjunct does;
- **same-head complement priority:** when a head's selected subcategorization
  frame licenses a preposition phrase as a complement, that same head instance
  does not also attach the same preposition phrase as an unselected adjunct.
  It extends to any non-verbal head (noun, adjective, determinative) that
  stands bare with its Complement omitted: a preposition phrase right after it
  that one of its frames selects is that head's Complement, never a dependent
  of another host (*on top [of your library]*, *equal to the number [of cards
  in their hand]*; never *who controls more lands than you equal* with *[to the
  difference]* left to *opponent*; CGEL, Ch. 5 §14.1, pp. 439–441, and §15,
  p. 454, for nouns; Ch. 6 §3.1, pp. 542–545, for adjectives, whose PP
  Complements are likewise licensed by the head, which picks the preposition,
  and mostly optional). Its prepositions pass up the right edge of every
  constituent that ends in the bare phrase;
- **adjuncts below a non-modal auxiliary:** a post-verbal adjunct after the
  participial Verb Phrase complement of perfect *have* or of passive or
  progressive *be* attaches to that lexical participial Verb Phrase, never to
  the uncoordinated auxiliary Verb Phrase above it: *was [put there from the
  battlefield this turn]*, *was [spent to cast this spell]*, *have [lost this
  turn]*, *have [[cycled or discarded] this turn]*. The two sites differ in no
  meaning, and k trailing adjuncts would otherwise yield 2^k trees; the host
  head is the lexical verb (*put*, *spend*, *lose*). A coordinated auxiliary
  Verb Phrase still hosts an adjunct shared by its conjuncts. The canonical
  site is built structurally in the chart, not merged afterwards;
- **fronted dependent outermost:** when a Clause has both a fronted
  dependent, set off by a comma (*When this creature enters, …*, *Until end
  of turn, …*) or preposed without one (*instead create X of those tokens*),
  and a comma-set post-adjunct or Supplement (*…, except it has flying*,
  *…, where X is its power*), the fronted dependent attaches outermost:
  *Pre[,] [Clause, Post]*, never *[Pre[,] Clause], Post*. Both are
  dependents of the same Clause, and the two trees differ in no meaning. The
  canonical site is built structurally: the host of a comma-set
  post-dependent never begins with a fronted dependent;
- **added element versus coordination:** an *and only …* or *but only …*
  qualifying fragment after a Clause that ends in a Predicate Adjunct it could
  coordinate with (*Activate only as a sorcery and only once each turn*) is that
  Adjunct's coordinate, a basic coordination of Adjuncts, never also an added
  element of the Clause (CGEL, Ch. 15 §4.5, p. 1346, [29]). The carrier is the
  Clause's final Adjunct, not its mood: bare imperatives and finite clauses
  alike (*the player can activate mana abilities only if … and only if …*)
  take the coordinated Adjunct. The added element is what the fragment is
  where the Clause ends in no such Adjunct, so the declarative *…, but only …*
  and *Only your opponents may … and only …* keep it. The canonical site is
  built structurally: the Clause of an added element never ends, through its
  head and last complement, in a trailing Adjunct of a coordinable Category
  (the engine's `unadjoined` declaration);
- **frequency interval:** a temporal Noun Phrase after a Verb Phrase's final
  Adverb Phrase is a second Adjunct of the Verb Phrase (*[[activate only once]
  each turn]*), never also the adverb's interval (FrequencyInterval) in one
  Adverb Phrase (*[only once each turn]*): the two trees differ in no meaning
  (CGEL, Ch. 8 §9, p. 715: *each year is probably a separate adjunct rather than
  a postmodifier*). The interval is the adverb's only where no Verb Phrase
  follows it, as in the fronted *Once each turn, …*. The canonical site is built
  structurally: the end Adverbial of a Verb Phrase takes no Adverb Phrase that
  ends in a temporal Noun Phrase;
- **medial remnants:** a coordination may have a remnant for a conjunct, a
  phrase with an element at its beginning elided and recovered from the
  initial conjunct: a copular Verb Phrase's Predicative Complement without the
  copula (*is enchanted, [equipped], and has a counter on it*). It is at the
  level of the final conjunct, with the ellipsis in the medial ones (CGEL,
  Ch. 15 §3.6, pp. 1335–1336, [31ii]: *He was middle-aged, of sallow
  complexion and had penetrating blue eyes*), so a remnant is only ever a
  medial conjunct, never the initial or the last. A coordination whose later
  conjuncts were all remnants would be the coordination of their Predicatives
  under the one copula (*is [a creature, an artifact, and an enchantment]*;
  §3.2, pp. 1326–1327), and the two trees would differ in no meaning. The
  canonical site is built structurally: a remnant is the first conjunct of a
  serial tail only, never the last conjunct after its coordinator.
- **single Objects in Complement Clusters:** a Complement Cluster may be a
  single Slot with no Adjunct (*exiles [the top card of their library], [a
  card at random from their graveyard], and [a card at random from their
  hand]*) only where it alone breaks the coordination's parallelism: it is
  not the last Cluster, and at least two sibling Clusters end in Adjuncts,
  all in as many (CGEL, Ch. 15 §4.3, pp. 1341–1343). CGEL stars a single
  element coordinated with two (p. 1342, [17iia]) but grades breaches of
  parallelism (note 64); Oracle English prints this one. A coordination of bare single Objects (*Exile target
  creature and target land*; *has lifelink, deathtouch, and flying*) is the
  coordination of those Objects, the verb's one Object, never also a
  coordination of Clusters: the two trees differ in no meaning. The sibling
  Clusters must be Clusters of necessity: each holds an Adjunct no Nominal
  hosts (*at random*). Where every Adjunct could be its Object's Modifier,
  the coordination of Objects is parallel, and a bare Cluster beside them is
  the breach CGEL stars (p. 1341, [17iia]): *exile [this artifact], [all
  creatures …] [on it], and …* is not admitted. The canonical site is built
  structurally: the coordination of Clusters a verb heads licenses each bare
  Cluster it holds (`Summary::bare`).

Whenever a grammar change introduces or tightens a structural canonicalization,
its gate compares, per unit, the set of Reading Normal Forms before and after
under that exact equivalence and nothing wider: the set of analyses is
unchanged while the number of trees falls.

**Cross-head attachment ambiguity is retained in the packed forest.** While each
`(preposition, complement_frame)` pair declares its unselected host categories
(so unselected `for + NP` is a clause/predicate adjunct rather than a free
nominal postmodifier, and `with + KeywordPhrase` is a nominal postmodifier
only), when a phrase's declared host categories genuinely match more than one
head along the right spine, both attachments are retained:
- a `with + NP` phrase that can either postmodify an object nominal or modify
  the matrix predicate (`"Exile target creature with a +1/+1 counter on it"`);
- a right-spine temporal adjunct that can modify either a matrix predicate or
  an embedded relative-clause predicate (`[[Seedborn Muse]]`).

Settled analyses:

- *Be* plus a participle keeps both the adjectival (state) and the verbal
  passive (action) Reading. Both are attested.
- A multiword game term that a plugin or catalog declares as one term is one
  leaf. It does not suppress the single-word analyses of its parts when those
  parts are ordinary lexicon words that straddle a constituent boundary: in *if it's the first time counters have
  been put on that creature*, *time* ends the Nominal *first time* and
  *counters* begins the clause after it, so the plural of the catalog counter
  *time counter* [CR#122.1] does not hold them together. The parts straddle a
  boundary only when, in the smallest constituent containing the term, the
  part holding its first word begins before the term and the part holding its
  last word ends after it. Otherwise the parts rebuild the term's own
  analysis, alone (*[first] [strike]*) or with a dependent of one part
  (*mana [cost for permanent spells]*), which is a second tree beside the term.
  A core multiword term (*mana value*) never splits, nor does a term with a
  part that is not an ordinary word (the measure in *+1/+1 counter*). A
  multiword noun whose first word is also a preposition is no barrier at
  all: its words form a Preposition Phrase, never the noun's own analysis.
  *Outside the game* [CR#400.11] is a noun after *from* and as a conjunct
  beside zone nouns (*from outside the game*; *your library and/or outside
  the game*), and *outside* is the preposition where the noun has no place
  (*cards you own [outside the game]*, *if it's [outside the game]*). The
  noun is count and stands bare only in *from*'s Fixed Frame (CGEL, Ch. 5
  §8.5(b), pp. 409–410), never as a bare Noun Phrase elsewhere.
- A colour word is an adjective and a noun converted from it (CGEL, Ch. 19
  §3.3, p. 1642: *His tie is brown* (adj), *a dark shade of brown* (noun)),
  never a fused-head adjective for the colour itself: conversion is
  distinguished from the fused modifier-head (p. 1642), whose colour heads
  are elliptical, recovering a head from an antecedent (*Henrietta likes red
  shirts, and I like [blue]*; Ch. 5 §9.3, p. 417, [25i]). Each position has
  one of the two. Where only a Noun Phrase stands (a preposition's
  Complement, *devotion to blue*, *a color other than white*; a Subject or
  an Object, *green is the most common color*, *choose black or red*) it is
  the noun. A bare colour word after a copula is the adjective (*becomes
  [green]*; Ch. 6, p. 538, [29i]), never also the noun: ascriptive, the two
  would ascribe one property, and CGEL gives the adjective (p. 1642); the
  noun as an ascriptive Predicative takes a determiner (*a dark shade of
  brown*; Ch. 4 §5.4, p. 266, [43]: *an unusual colour* beside the colour
  adjectives). Specifying needs no noun either: an Adjective Phrase
  specifies too (*What you are … is [too big for your boots]*; §5.5.1,
  p. 267, [48ii]). The noun is `HeadOnly` and non-count, and such a noun is
  no bare Predicative; other bare non-count predicatives are English
  (*That's utter nonsense*, p. 536; *The sheet is cotton*, p. 538), so
  nothing excludes them, which would lose 76 faces. In a Quality
  (*protection from red*) the colour word is the noun alone: the adjective
  is a fused-head Quality only for a word with no noun of its own
  (*protection from multicolored*), which the lexicon declares
  (`fused_quality`), as a supertype word is one [CR#205.4a]. *Protection
  from red* has one Reading (Dominaria's Judgment, 88 → 5), *becomes red*
  one.
- A chain of bare imperatives has one analysis: coordinated imperative clauses.
- *Then* after a comma marks a Linked Coordination (Oracle English glossary):
  it is an Adjunct of its Linked Conjunct, never a Coordinator, and the
  Conjunct may be a coordination by a Coordinator (*then [shuffle and put
  that card on top]*; CGEL, Ch. 15 §2.10, pp. 1319–1320). A sole connective
  links instructions and main clauses or Verb Phrases, never subordinate
  clauses (`docs/oracle-style-guide.md` §10, Sequence; CGEL p. 1320 on
  *so*): *until [they exile a nonland card], then you may cast it*, never
  *until [they exile a nonland card, then you may cast it]*.
- A locative or temporal Preposition Phrase right after an existential's
  Displaced Subject is its Extension, a Complement of *be* (CGEL, Ch. 16
  §6.2.1, p. 1394), never also the Displaced Subject's postmodifier or an
  Adjunct: the two attachments are one existential claim, so this is not a
  retained cross-head ambiguity. A Preposition Phrase that cannot be an
  Extension (*of the requested type*) stays a postmodifier.

**Wrong analyses.** A face is read when a correct analysis is among its
Readings; wrong Readings beside a correct one are tolerated, because not every
one can be eliminated in syntax alone. A face whose Readings are all known to
be wrong is not covered, and is reported separately from read and unread. A
newly admitted ungrammatical Reading is a defect, never a gain; a grammatical Reading that English admits beside the intended one is disambiguated by semantics ([`english-v4.md`](./english-v4.md) §4.4, principle 1). A known-wrong
analysis is never accepted as coverage.

**Morphological and valence completeness vs. controlled-register syntax.** Every
declared verb and noun receives its full English inflectional paradigm, and
every declared complement frame automatically receives its active, passive,
gapped, and shared-complement derivations whether or not cards exercise every
combination; only an Active-only Frame (*cost*, *have*: CGEL, Ch. 16 §10.1.2,
p. 1432) derives no passive. Syntactic schemas and unselected host categories, by contrast,
model the controlled technical register of **Oracle English**
(`docs/oracle-style-guide.md`), not unrestricted open-domain English. *The
Cambridge Grammar of the English Language* (CGEL) supplies standard grammatical
terminology and constituent structure; it does not always contain the
Oracle-appropriate attachment rule and is never used to force open-domain
ambiguities (such as *"I found a book for my children"* licensing unselected
`for + NP` on arbitrary nouns, or *"I saw a man with a telescope"* licensing
`with + KeywordPhrase` as a verb adjunct) into the Oracle grammar.

## 3. How words reach the grammar

Content words and all Comprehensive Rules catalog items enter only through
lexemes: plugin terms, catalog entries (including all Vintage-legal keyword
actions [CR#701.1], keyword abilities [CR#702.1], ability words [CR#207.2c],
card types, supertypes and subtypes [CR#205.2a,205.3,205.4a], counters
[CR#122.1], turn parts [CR#500.1], and designations), card names [CR#201.5] and
open-class core words reach the grammar through typed lexical classes and
subcategorization frames, never by name in the grammar data file.

Core closed-class words (determinatives, prepositions, coordinators,
subordinators, auxiliaries, pronouns, negators, the interrogative adverb *how*;
CGEL, Ch. 10 §7, p. 902) may be referenced directly, by
lexeme id, never by a string. A direct reference is used when exactly one
lexeme can fill a slot; a typed class is used when two or more can. A class
with one member, or a feature value with one holder, is reported so it gets a
look; it is not a defect when the construction it marks really has one holder
(a one-of-one card in the long tail, or a word with no peer). A feature may
declare a single, privative, value: leaving the feature unvalued is the
unmarked default. A value's holders are counted as they are: the members of
a class a Production lifts, the lexemes that declare the Frame or Parameter
pattern that projects it, and once across features equated with one another.

Unknown ids, class names and values are load errors.

## 4. The authoring rule

An author states what a schema or production changes; the syntax engine
(`deckmaste_syntax`) derives the rest at load time from the grammar data file.
Complete feature summaries and valence alternations (active, passive,
slash-gap extraction, and right-node raising) are the engine's compiled
productions from declared complement frames, not the author's manual input. A
new feature, a new coordinable category or a new lexical distinction that costs
dozens of lines is a defect in the schema/compilation layer, to be fixed there.

## 5. The gate

- Completing Complexity Stratum $k$ (`0..=7` in [`docs/decisions/english-v4.md`](./english-v4.md))
  requires 100% coverage of all non-deferred corpus units in Strata $0..=k$
  with both round-trip laws holding and zero `known-wrong-only` units counted as
  read.
- The judgment set is the only tracked linguistic test evidence: hand-written,
  one line per entry, no tree dumps. Nothing a tool emits is tracked.
- Whether an analysis is right is decided against the controlled Oracle English
  register (`docs/oracle-style-guide.md` and [`docs/decisions/english-v4.md`](./english-v4.md),
  using CGEL for standard terminology and constituent structure) by two
  independent judges; the owner sees disagreements.

## 6. Size

The aim is a clean, concise, correct grammar. Line count is a review smell test
and is not a mechanical target: nothing enforces a hard cap, and no landing is
required to lower or record it.

A concise grammar data file (on the order of ~2,500 lines, ~40 features, ~60
categories at full corpus coverage) is the expected scale; a grammar several
times that size—or one that pushes equivalent complexity into `deckmaste_syntax`
or `deckmaste_lexical_source_v4`—is a design defect. It is fixed by better
schema abstractions (section 4), not by counting or by policing flags.
