---
needs: [english-v3-number-transparent-concord]
---
# Audit and remove redundant bare-target projection routes

Determine whether the direct TargetNounPhrase and
BarePlural(TargetedNominal) routes for the attested constituent “target creatures”
encode the same grammatical targeting function. The exact probe yields two Noun
Phrase Readings with identical words, Plural/Third Agreement and targeting; the
Nominal probe has one TargetedNominal. No scope-bearing material intervenes here.
Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered 13,226).

The glossary deliberately permits determinative and nominal-modifier projections
of the Targeting Marker. Do not remove one legitimate function globally or change
that ruling merely to lower counts. Establish an explicit structural equivalence
for the local overlap; if both routes intend one function, prevent its duplicate
derivation at source through the shared NP pipeline. If they represent genuinely
different functions, document that result and preserve them.

Preserve attested composition in Blinding Beam's “two target creatures”, Sanguine
Indulgence's “two target creature cards”, Bojuka Bog's “target player's graveyard”,
and Dwarven Thaumaturgist's “target creature's power and toughness”. Audit modifiers
above/below targeting, singular/plural, quantifiers, genitives and coordination;
do not collapse those scopes by analogy with the bare case. Avoid a blanket
post-parse semantic quotient or card/word-named guards.

Acceptance: independent expected structural sets with both roundtrip laws;
classify each removed route as redundant versus invalid versus retained-distinct.
Report actual corpus Reading-count changes and identity-level coverage losses,
not a presumed global twofold reduction. Optional scratch: /tmp/english-v3-np-probes.
Standard constraints apply.

## Sequencing

The `needs:` edge is sequencing, not a technical dependency: these grammar
tickets touch overlapping structures and are worked one at a time. Write the
pinned witnesses as tests before implementing.
The landing compares corpus Reading identities before and after on its own tree.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It adds a `targeting_projection.rs` test file. Read it for ideas if useful. Do not rebase onto
it, and treat every claim in its ticket notes as unverified.
