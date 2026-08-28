---
needs: [english-v2-stage-5-grammar-buildout-11-10]
---
Reduce single-member verb frame keys and MTG-named sentence-grammar codecs
(contract-landing-review findings 2 and 3). The verb dissolution moved
lexeme closure into frame-key closure: 22 of 42 verb codecs use role atoms
outside the plugin tail set and 24 of 29 core frame keys hold exactly one
verb. Collapse toward shared general frames: make "life" an ordinary mass
noun, unify the four encodings of "damage", route the GainLifeVerb /
LookAtVerb / Deal*Damage* class through general transitive/measure frames
with ordinary PP complements (~9 codec collapse estimated by the review).
Naming rule: codecs are named for their linguistic shape, never a lexeme
or mechanic. Coverage must not drop; ties during collapse follow the
dissolve-in-same-commit rule; genuine tie = STOP. Standard constraints
apply.
