# English grammar: lexical analysis, Readings, and how the grammar is kept small

Replaced 2026-10-12. This is the one governing record for `deckmaste_english_v3`.
It has no amendment log: when a rule changes, this text changes. English is an
independent parsing project with no interaction with Semantics.

## 1. The engine contract

**Lexical analysis is independent of the grammar.** Input is analysed without
construction-directed scanning. Each alternative identifies a declared Lexeme,
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
realization derives it from structure.

**Facts about the input.** The supported corpus is the Vintage-playable cards
(legal or restricted), excluding Scryfall `set_type: funny`. Balanced
parenthesised reminder text is removed before analysis; the original Oracle text
is kept as source metadata. A card's Shortened Name is in scope only on that
card's own text [CR#201.5c]; full names are in scope everywhere. The census
enumerates, validates, counts and digests per unit below the face, and a face's
reported count is a sum over its units.

## 2. What a Reading is

A Reading is one whole tree enumerated from the packed forest. Two Readings are
the same only if they are the same tree.

Multiple Readings are successful parsing. A cost may rank Readings for
presentation; it never decides admission and never removes one.

**The grammar may stop producing a tree that is not a different grammatical
analysis of the text.** Three cases are approved:

- the order in which modifiers are applied to one head;
- the grouping within a chain joined by the same coordinator;
- clause versus verb-phrase coordination of bare imperatives, where nothing
  distinguishes them.

The gate for such a change is the analysis key: per unit, the set of keys is
unchanged while the number of trees falls. Not decided, and therefore not
permitted: dropping the adjunct or postmodifier analysis of a preposition
phrase that the verb's frame also selects.

Settled analyses:

- *Be* plus a participle keeps both the adjectival (state) and the verbal
  passive (action) Reading. Both are attested.
- A multiword game term that a plugin declares as one term is atomic.
- A chain of bare imperatives has one analysis: coordinated imperative clauses.

**Wrong analyses.** A face is read when a correct analysis is among its
Readings; wrong Readings beside a correct one are tolerated, because not every
one can be eliminated. A face whose Readings are all known to be wrong is not
covered, and is reported separately from read and unread. A newly admitted wrong
Reading is a defect, never a gain. No ruling accepts a known-wrong analysis as
coverage.

**Attestation is provenance, never a filter.** A declaration admits its whole
linguistic class whether or not cards exercise every member. A productive
pattern is licensed for its class, not for the word that witnessed it.

## 3. How words reach the grammar

Content words enter only through lexemes: plugin terms, catalog entries, card
names and open-class core words reach the grammar through typed lexical classes
and frames, never by name.

Core closed-class words (determinatives, prepositions, coordinators,
subordinators, auxiliaries, pronouns, negators) may be referenced directly, by
lexeme id, never by a string. A direct reference is used when exactly one
lexeme can fill a slot; a typed class is used when two or more can. A class
with one member, or a feature with one holder, is reported as a finding.

Unknown ids, class names and values are load errors.

## 4. The authoring rule

An author states what a construction changes; the compiler derives the rest.
Complete feature summaries are the compiler's output, not the author's input.
A new feature, a new coordinable category or a new lexical distinction that
costs dozens of lines is a defect in the authoring layer, to be fixed there.

## 5. The gate

- A refactor keeps every reading digest identical.
- A linguistic change loses no face, and is checked against the judgment set
  and the analysis-key rule in section 2.
- The census check mode enforces both by failing; a report is not a gate.
- The judgment set is the only tracked evidence: hand-written, one line per
  entry, no tree dumps. Nothing a tool emits is tracked.
- Whether an analysis is right is decided by two independent judges citing
  CGEL or `docs/oracle-style-guide.md` verbatim; the owner sees disagreements.

## 6. Size

The aim is a clean, concise, correct grammar. Line count is a reasonable
measurement of that and is not a target: nothing enforces it, and no landing
is required to lower or record it.

About 2,400 grammar lines at 98% coverage has existed, so a grammar several
times that size is a defect. It is fixed by better abstractions (section 4),
not by counting. The rule for resuming coverage work is decided when that
step is reached.
