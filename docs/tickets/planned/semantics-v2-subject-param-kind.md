---
needs: []
---
**`KeywordParam::Subject` and the `Subject` macro-parameter kind disagree.**
Found by `semantics-v2-macro-bodies-keyword-abilities` round two
(2026-09-07): a keyword whose printed argument is a description ("Champion
a Kithkin", "Enchant creature you control") carries it as
`KeywordParam::Subject(Predicate)` in the syntax, while the declaration's
`params: [Subject]` maps to the `NounPhrase` kind in
`deckmaste_semantics_v2::ron`, so the definition body cannot forward
`Param(0)` into a predicate position and the keyword STOPs. Retyping the
declaration to `Quality` proved the body and broke english_v2's
`keyword_lines` suite, which parses the line through `Subject`.

Decide what `Subject` denotes on both sides, from Lean: `KeywordParam` in
`lean/Semantics/Abilities.lean` is the spec for what the printed argument
IS; the macro-parameter kind must be the same type, and english_v2's
frame for the keyword line follows. Then Champion, Enchant, and every
keyword the record lists under this STOP get their bodies. Standard
constraints apply; closure includes english_v2.
