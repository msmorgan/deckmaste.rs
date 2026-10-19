---
needs: []
---
# One Noun Phrase coordination schema per coordinator

Stratum 6 splits each Noun Phrase coordination schema by its conjuncts'
Determination and Countability, so an overtly determined first conjunct never
coordinates with a bare mass last one (*[target spell or permanent that's red]
or [green]*; Lush Growth's *[a Mountain], [Forest], and [Plains]*). The owner
accepted that as an exception: it takes the schemas from 9 to 17 in
`crates/deckmaste_english_v4/grammar/english.ron`.

## Change

- Express the restriction once, as a general mechanism, so each coordinator
  needs one schema for untargeted conjuncts and one for a targeted first
  conjunct, as before the split.
- A lexicon fix cannot do it: colour nouns stand bare legitimately (*choose
  red*, *protection from red*).

## Done when

- The Noun Phrase coordination schemas are back to 9, or fewer.
- No unit gains a derivation and no face is lost against the census before
  the change; Lush Growth stays unread (deferred) until a correct reading
  exists.

Standard constraints apply.
