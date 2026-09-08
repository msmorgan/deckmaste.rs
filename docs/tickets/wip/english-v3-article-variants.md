---
needs: [english-v3-lexical-measures]
---
# Admit indefinite articles with onset and capitalization constraints

Replace the blanket indefinite-determiner exclusion in v3
`DeterminedNounPhrase` with general countability, Number and realized-onset
constraints. Complete grammatical capitalization admission using the lexical
values' retained surface variants. The `kkmxslkn` baseline has 19,516 failed
faces containing article-shaped text; this is an overlapping observation,
not a count of failures caused solely by articles.

Pinned shape: derive onset from the first pronounced constituent, including
premodifiers, numerals, names and bound/multiword forms. Normalize declared
pronunciation/defaults and overrides at the lexical realization boundary and
carry the effective value through grammatical summaries. Initial orthographic
letters alone are not onset authority. Preserve the chosen `a`/`an` and casing
variants in generated values; checked construction and parsing enforce the
same compatibility relation. Retain correlated onset/variant alternatives
through packing and any deferred admission. Update the Oracle English Onset
glossary entry's v2-specific wording to the established v3 ownership.

Capitalization depends on the actual sentence, keyword-line, quote and bound
word position. Preserve exact names and lexical identities; do not lowercase
the input or infer quote casing solely from closing punctuation. A bound
quality and keyword are cased as one word. Keep lexical alternatives available
independently before grammar checks their distribution.

Witnesses: Opt for a clause-final indefinite count noun; Krark-Clan Ironworks
for a vocalic article in an action cost; Viridian Joiner for an indefinite
measure noun; Animate Dead for a keyword subject at top level and inside a
quote; Ogre Marauder and Takklemaggot for contrasting quoted starts; Secret
Tunnel, Prisoner Zero and Three Dog for numeral/name casing. Fetch supported
Oracle text and retain each relevant constituent as a focused test even when
another part of the card still fails.

Acceptance pins positive and negative pairs for `a creature`/`an artifact`,
wrong article/onset combinations, plural/mass misuse and the first pronounced
modifier changing the required article. Include declared pronunciation
overrides that disagree with first-letter spelling. Test sentence starts,
ordinary interiors, keyword lists and both quote cases, preserving valid
homographs. Test independently constructed article/variant values, byte-exact
rendering, analysis identity and total traversal. The baseline synthetic
`Draw a card.` fails while `Draw the card.` and lowercase `draw the card.`
parse; use these as separate article and positional-casing discriminators.

Apply [Lexical analysis](../../decisions/english-lexical-analysis.md#lexical-analysis),
[Source and roundtripping](../../decisions/english-lexical-analysis.md#source-and-roundtripping)
and [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity):
keep Earley parsing, packed partial/complete derivations and declaration-driven
checked constructors, renderer and traversal. Both roundtrip laws and all
grammatical Readings are required. Update Lean before changing its grammatical
judgments. STOP and report rather than remove the exclusion without its
replacement constraints, name words in admission, echo source, construct eager
AST products or discard valid Readings. V2 remains untouched; unrelated
document/quote gaps route to `english-v3-systemic-residuals`. Standard
constraints apply.
