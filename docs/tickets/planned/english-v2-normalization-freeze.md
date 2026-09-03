---
needs: [english-v2-retire-manifest-precondition]
---
Freeze normalization over the whole corpus (line-initial landing review
M1). The coverage lock hashes only covered units' normalized text; since
the fossil normalizer test was (correctly) retired, ~16,675 parse-failure
units' normalization is pinned by nothing — a normalization change that
alters only failing units' text is invisible until one of them starts
selecting under a changed identity. Add a corpus-wide normalization digest
to the lock header (hash over all 32,641 normalized texts in unit order),
checked by `--check` like the existing corpus fingerprint and updated only
by `--bless`; a normalization change therefore always surfaces as lock
drift to be blessed with a landing-record note. Also: rename the three
`*_mid_line_*` tests to what they now test, and decide line-final "()"
pass-through explicitly. Zero identity change; standard constraints apply.
