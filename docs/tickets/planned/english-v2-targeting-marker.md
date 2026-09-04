---
needs: [english-v2-np-postmodifiers, english-v2-closed-class-single-owner]
---
**Represent targeting-sense prenominal `target` as one lexical Targeting
Marker with two thin syntactic projections.** Use the Oracle English
[`Targeting Marker`](../../contexts/oracle-english/CONTEXT.md) and Game Model
[`Target`](../../contexts/game-model/CONTEXT.md) definitions. The marker is invariant and has
one semantic contribution, but English distribution requires a determinative
projection for bare singular *target creature* and a nominal-modifier
projection under *another*, numerals, *up to N*, and similar determiners.

Replace the duplicate `DeterminativeHead::Target` and
`AttributiveAdjective::Target` lexemes with that one marker after the nominal
and closed-class ownership WIP lands. Retain the ordinary count noun
*target/targets* for phrases such as *choose new targets* and *the target*, and
retain the verb paradigm *target/targets/targeted/targeting* for clauses such
as *a spell that targets*. These are legitimate homographs, not projections of
the prenominal marker.

Update the scanner, grammar, AST, category inventories, diagnostics, and tests.
Append a dated amendment to the English-v2 rewrite decision that supersedes its
categorical lexical claims that *target* is or remains a determiner while
retaining the singular determinative projection, quantified modifier
projection, downstream semantic projection, and noun/verb homographs.

Acceptance covers *target creature*, *target artifacts*, *target tapped
creature*, *two target creatures*, *another target artifact*, *choose new
targets*, and *a spell that targets*, while rejecting adjective-like grading or
predicative use.

Ruling 2026-09-03: the user authorized superseding the rewrite decision's
target-as-determiner claim (including its 2026-08-27 target-as-noun amendment);
the amendment step above is not a STOP.
