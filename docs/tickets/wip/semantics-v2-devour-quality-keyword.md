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

## Landing record

Owner rulings 2026-10-10 (two §7 entries, `oxuompql`) resolve the four
contracts. Census measured on the tree of `pknkolqq` after the final refresh (claim
parent `ymtoyryz` at `335c60d9`); before is the claim parent's tree.

1. **Census delta** (bounded, `cargo xtask english-v3 --text-contains Devour`,
   30 faces): covered 19 → 22, Readings 742 → 786. Lost faces: none; no other
   face's reading digest changed.
2. **Witnesses**: "Devour Food 3" (Feasting Hobbit) 3 face Readings, 1 line
   Reading; "Devour land 3" (Famished Worldsire) 38 face Readings, 1 line
   Reading; "Devour artifact 1" (Caprichrome) 3 face Readings, 1 line Reading.
   Lean: `okDevour`, `okDevourQuality`, `okItDevouredCount` hold.
3. **Spot checks**: only three faces are newly covered, all three listed
   above; each line reads `QualityAmountKeyword(head: devourQuality, quality:
   NominalKeywordQuality(Food | land | artifact), amount: 3 | 3 | 1)`, byte-exact.
4. **Deviations and STOPs**: no STOP. Beyond the ticket's letter: (a) Lean
   `Ability.generalFormOf` in `keywordBodyFits` (mirrored in xtask
   `keyword_categories`): devour's body `devourQuality(creature, N)` is a
   same-label keyword term, whose components are read as devour's
   definition; any other keyword in a definition is still refused
   (`badOtherKeywordInDevourDefinition`). (b) A general form's signature
   reaches its keyword's facts row from the declaration
   (`KeywordDefinition.general_forms`), not an overlay `extra`. (c) Idris:
   the generator skips `Shape::QualityAmount` (`Shape::idris` returns
   `None`), the 2026-10-09 ruling extended to keyword rows; `FactsGen.idr`
   is unchanged and `facts check` passes. (d) `carried_keyword` decides by
   the body: one invocation of the label is a variant, a body invoking no
   label is a general form's own definition. (e) English
   `QualityAmountKeyword` reads the `QualityAmount` class feature and names
   no keyword. Glossary gap: "general form" (of a keyword) is in neither
   glossary. STOP (trunk, outside this ticket): after the final refresh,
   `cargo xtask facts check` refuses "ChooseABackground: keyword
   declaration has no gate-column overlay" and `cargo xtask lean macros
   check` reports `Declared.lean` stale; both come from the six grammar-only
   keyword declarations of `english-v3-phrasal-keyword-surfaces`
   (`zvoyutom`: chooseABackground, doctorsCompanion, partner*), whose
   regeneration adds only those six macros. Resolved on trunk by `kruuppqw`
   ("Register grammar-only keyword semantics debt"); not fixed here.
5. **Tests**: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings`
   for `deckmaste_construction_core`, `deckmaste_semantics_v2`,
   `deckmaste_lexical_source`, `deckmaste_english_v3`, `xtask`: all pass and
   clippy is clean before the final refresh; after it, all pass except
   `xtask --lib` (392 pass, 15 fail: the 14 `facts` tests and
   `lean_macros::…the_committed_module_matches_the_declarations` on the
   trunk STOP above); after refreshing onto `kruuppqw`, `cargo test -p
   xtask` passes (407 + 14 + 4, 1 ignored) and `facts check` and `lean macros
   check` pass. `lake build --wfail` passes after each refresh;
   `macros check` and `facts check` passed before the final refresh; `cite
   check` (0 non-compliant, 0 stale) passes; `cargo xtask lean
   check` 241/241 cards (233 canon incl. Feasting Hobbit, 8 testing), `cargo
   xtask lean definitions` 711/711. Restored 0.
   Re-spelled 3: the nursery test's name-from-surface assertion and its
   "only bare gift is unspelled" assertion retire by ruling 2 (the unspelled
   skip stays; the dead `declaration_name` helper goes with them); the
   refused variant case `[kicker(tapSymbol)]` becomes `[gift(draw(1)),
   kicker(tapSymbol)]`; `okDevourQualityExpansion` becomes `okDevourQuality`.
   Counts updated: parameter table 108 → 109, keyword templates 66+201 →
   66+202. Ignored 0. Added 3: `a_general_form_carries_its_own_body_under_the_label`,
   `badOtherKeywordInDevourDefinition`,
   `quality_amount_keywords_read_a_bare_quality_then_a_number`. Removed 0.
6. **Timing**: bounded census wall 4.26 s, 1 worker.
