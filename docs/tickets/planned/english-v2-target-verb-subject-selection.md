---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Integrate the Target Verb without game-semantic Subject filtering

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

The retained ticket identity owns the withheld Target Verb and Infectious Curse
re-coverage. Its former Spell/Ability Subject whitelist is withdrawn under the
NLP-only scope: Subject is a grammatical relation, and game-reference
resolution is not available to English. The Target Noun and the two projections
of the single invariant Targeting Marker remain distinct lexical homographs.

Declare the ordinary verb forms through the shared frame/morphology inventory
and apply the same case, agreement, dependency and evidenced-selection rules
as other verbs. Preserve the declared imperative restriction only if it has
independent grammatical distribution evidence; do not substitute a guard naming
the verb, its game meaning or a construction. The reviewed scope relation does
not automatically pack a marker-versus-verb lexical-category rivalry.

Acceptance probes: `a spell that targets`, `spells that target`, `it targets`,
`can't target you`; and `Two target creatures or planeswalkers gain 2 life.`
with its intended marker/NP bracketing retained. Recheck the marker-subject
inventory from the original 5,805-identity audit against current supported
input; that historical count is not an acceptance target. Infectious Curse
must expose the object-gap relative in `Spells you cast`, the subject-gap
relative `that target enchanted player`, and the shared cost-comparison frame.
Its full identity is in the register.

Known design risk: both marker and finite-verb readings may remain grammatical
after the general replacements. If they survive, STOP with the two checked
ASTs and the exact missing decision; seek a grammatical/ambiguity ruling rather
than restoring the semantic whitelist, adding a dominance edge, treating a
non-scope rivalry as scope, or declaring a wrong selection covered. This ticket
cannot close merely on unchanged coverage while the Target Verb is withheld.
It is a separate integration gate so that risk neither hides nor restarts the
whole grammar review. Source evidence: style-guide §6 “Describing objects,
players, and targets”; the glossary owns the three lexical distinctions.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
