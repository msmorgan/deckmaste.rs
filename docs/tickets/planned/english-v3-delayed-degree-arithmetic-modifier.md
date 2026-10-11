---
needs: []
---
# Read the delayed arithmetic Modifier after a degree-quantified noun

Read *that many cards plus one*: the addition/subtraction PP follows the
head noun but modifies the degree quantity. Preserve the noun's quantification
and plural agreement, with one route for this construction and both roundtrip
laws. DQ1 already reads *that many plus one +1/+1 counters*.

The parent ticket's DQ2 project ruling requires the delayed PP to be licensed
by the degree Determiner's exported features, rather than admitted as a general
NP, verb or Clause Modifier. Its fallback permits a separate ticket if a new
Construction is necessary. That is the current compiler boundary: alternate
forms must retain identical named fields and cardinalities, so
QuantifiedNounPhrase cannot add a mandatory delayed field in a second form;
optional fields also have no single feature value for cross-field licensing.

Smallest proposed form: one new Noun Phrase Construction with
`[quantity: QuantitativeDeterminer, " ", head: Nominal,
arithmetic_modifier: ArithmeticModifier]`. Reuse DQ1's PP, require a declared
degree-derived licence exported by `quantity`, and retain QuantifiedNounPhrase's
nominal licensing, Agreement and feature equations. The PP is mandatory here,
so the ordinary NP retains its existing route. No word-naming guards. This is
a proposal for the follow-up, not an already implemented Construction.

Evidence: `target/english-v3/english-v3-degree-quantity-arithmetic/` contains
`before-{kenessos,colossus,water,draw}.json` and corresponding `after-*.json`
probes: each delayed witness remains 0; controls are respectively 1, 3, 1, 1.
`delayed.json` records the 21 unread faces found with
`cargo xtask english-v3 grep 'that many [a-z0-9/+ ]+ (plus|minus) (one|two|[a-z]+)\b'
-i --census target/english-v3/english-v3-degree-quantity-arithmetic/after.json
--unread`. This is the measured inventory; other gaps and the parent's stated
out-of-scope shapes mean it is not a promise of 21 gains:

- Heartwarming Redemption
- Colossus of the Blood Age
- Chandra, Flamecaller
- Evolving Door
- Twinning Staff
- Kenessos, Priest of Thassa
- Wyll, Blade of Frontiers
- Quantum Riddler
- Into the Night
- Barbarian Class
- Sycorax Commander
- Ichor Elixir
- Celes, Rune Knight
- Brass's Tunnel-Grinder
- Pixie Guide
- Path of the Pyromancer
- Khorvath's Fury
- Dark Deal
- Into the Fire
- The Water Crystal
- Valakut Awakening

Standard constraints apply. Re-probe witnesses and controls on the claim
parent. Assert the PP's delayed dependent role and its degree licence, the
quantified plural Nominal and both roundtrip laws. Preserve internal DQ1,
scalar arithmetic and multiplier controls. Use the bounded surface selector
`(?i)\bthat (many|much)\b|\b(plus|minus)\b` plus 300 random covered controls,
seed 20261008, and the official census diff; run the full changed-crate suites
and clippy at the end.
