---
needs: [english-v3-generic-frame-consumption]
---
# Compose maximum quantities with existing cardinal phrases

Refine quantity/Determiner composition to admit “up to” with a cardinal expression
and feed the existing Noun Phrase/Nominal pipeline. Do not add a bespoke Noun Phrase
recipe for each noun or target combination, or treat recognition of “up to” as
permission to broaden Oracle English beyond its attested quantity forms.

Pinned witness: Sanguine Indulgence's “up to two target creature cards”. Exact
Noun Phrase probing gives zero Readings, as does Cardinal probing of “up to two”;
“two target creature cards” has one. All words have Lexical Analyses. Reasonable
Doubt supplies “up to one target creature”, also currently unrecognized. Counted
targets and targeted genitives otherwise already compose: preserve that structure.

Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered
13,226). The raw supported-Oracle scan found 966 overlapping up-to-target surface
occurrences, including reminder text; this is not a gain forecast or sole-cause
classification. Fetch supported current card text for implementation witnesses.

Acceptance: independently construct authentic maximum-quantity constituents,
assert the intended quantity structure and appropriate Number/Agreement, preserve
lexical ownership, targeting and both roundtrip laws. Reuse existing schemas where
possible. Report complete corpus identity-level gains/losses and remaining causes;
recognition alone does not establish the intended quantity analysis. Scratch
/tmp/english-v3-np-probes is optional diagnostic evidence. Standard constraints apply.

## Sequencing

The `needs:` edge is sequencing, not a technical dependency: these grammar
tickets touch overlapping structures and are worked one at a time. Write the
pinned witnesses as tests before implementing.
The landing compares corpus Reading identities before and after on its own tree.

## Pinned analysis

Quantitative *up to two* is a Preposition Phrase in determiner function, not a
Cardinal head. CGEL Ch. 5, p. 357, example [6] lists *up to twenty minutes*
beside *around ten thousand copies* and *under ten new drugs* as PPs in that
position, and the glossary's Quantity entry already counts *up to two* among
Determiners. The counted Noun Phrase therefore keeps its noun head and takes
either a Cardinal or this PP as its determiner; no second Noun Phrase family is
introduced.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It reached the analysis pinned above. Read it for ideas if
useful. Do not rebase onto it, and treat every claim in its ticket notes as
unverified.
