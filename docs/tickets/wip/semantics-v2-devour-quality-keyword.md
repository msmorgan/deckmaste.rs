---
needs: []
---
**Declare `devourQuality`, the general "Devour [quality] N" [CR#702.82c].**
`semantics-v2-off-stack-statics` wrote devour's definition as the general
`devourQualityExpansion(quality, N)` (an Ability helper under
`plugins_v2/builtin/macros/abilities/`), bare devour N being it over creatures,
but could not land the keyword declaration the 2026-10-09 ruling names. A
`KeywordAbility` named `devourQuality` with `params: [Quality, Amount]` and
grammar surface "devour" is refused by four standing contracts, each needing
its own decision:

- `deckmaste_construction_core`'s keyword parameter classes have no
  quality-then-number class (`UnsupportedKeywordParameterSignature`);
- `builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`
  requires a keyword ability's name to derive from its grammar surface, and
  only bare gift to be unspelled;
- the English line codec has no construction for that class, so "Devour Food
  3" (Feasting Hobbit) has no Reading;
- xtask's facts `Shape` has no quality-then-number shape, and the frozen Idris
  facts table has no constructor for one.

Once declared, its body is `[devourQualityExpansion(Param(0), Param(1))]`, and
Feasting Hobbit is its canon witness.
