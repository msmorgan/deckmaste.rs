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
(see below).

**Facts about the input.** The supported corpus is the Vintage-playable cards
(legal or restricted), excluding Scryfall `set_type: funny`. Balanced
parenthesised reminder text is removed before analysis; the original Oracle text
is kept as source metadata. Oracle separates the keywords of a keyword line with
semicolons only because one of them carries reminder text, so on a line that
loses reminder text and holds no sentence, those semicolons become commas. A
card's Shortened Name is in scope only on that card's own text [CR#201.5c]; full
names are in scope everywhere. The census enumerates, validates, counts and
digests per unit below the face, and a face's reported count is a sum over its
units.

## 2. What a Reading is

A Reading is one whole tree enumerated from the packed forest. Two Readings are
the same only if they are the same tree.

Multiple Readings are successful parsing. `Production Cost` may rank Readings
for presentation; it never decides admission and never removes one.

**The grammar avoids producing distinct trees for the same grammatical analysis
of the text.** Four canonicalizations are approved:

- the order in which commuting pre- and post-modifiers are applied to one
  nominal head (from a stated, reviewed list; anything unlisted is a barrier);
- the grouping within a chain joined by the same coordinator;
- clause versus verb-phrase coordination of bare imperatives, where nothing
  distinguishes them;
- **same-head complement priority:** when a head's selected subcategorization
  frame licenses a preposition phrase as a complement, that same head instance
  does not also attach the same preposition phrase as an unselected adjunct.

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
- A multiword game term that a plugin or catalog declares as one term is atomic.
- A chain of bare imperatives has one analysis: coordinated imperative clauses.
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
newly admitted wrong Reading is a defect, never a gain. No ruling accepts a
known-wrong analysis as coverage.

**Morphological and valence completeness vs. controlled-register syntax.** Every
declared verb and noun receives its full English inflectional paradigm, and
every declared complement frame automatically receives its active, passive,
gapped, and shared-complement derivations whether or not cards exercise every
combination. Syntactic schemas and unselected host categories, by contrast,
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
subordinators, auxiliaries, pronouns, negators) may be referenced directly, by
lexeme id, never by a string. A direct reference is used when exactly one
lexeme can fill a slot; a typed class is used when two or more can. A class
with one member, or a feature with one holder, is reported as a finding.

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
