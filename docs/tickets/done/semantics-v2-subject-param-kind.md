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
declaration to `Quality` proved the body and broke the retired parser's
`keyword_lines` suite, which parses the line through `Subject`.

Decide what `Subject` denotes on both sides, from Lean: `KeywordParam` in
`lean/Semantics/Abilities.lean` is the spec for what the printed argument
IS; the macro-parameter kind must be the same type, and english_v3's
frame for the keyword line follows. Then Champion, Enchant, and every
keyword the record lists under this STOP get their bodies. Standard
constraints apply; closure includes english_v3.

## Resolution

Lean is the spec: `KeywordParam.subject` carries a `Predicate`, a
description printed as a nominal ("Enchant creature you control",
"Champion a Kithkin"). The declaration kind `Subject` is a `NounPhrase`, a
referent (§7 ruling 2026-10-06 names it a referent parameter), and stays
one. So a description keyword declares `Predicate`, which the loader
forwards as `KeywordParam::Subject`; `Subject` is no longer a keyword
argument type anywhere in the chain:

- `deckmaste_semantics_v2::keywords::forwarded_keyword_params`:
  `Predicate` → `Subject(Param(i))`; a `Subject` parameter is refused.
- `deckmaste_construction_core::macro_def::ParameterType` gains `Predicate`
  and drops `Subject` (now `Other`); `[Predicate]` normalizes to the
  `Subject` keyword-line class, so english_v3's `SubjectKeyword` frame
  (head + `Nominal`) is unchanged.
- xtask facts: `["Predicate"]` → `Shape::Subject` / `.subject`.
- champion and enchant declare `params: [Predicate]`; enchant forwards its
  argument (its definition stays settled empty); champion gets its two
  linked triggered abilities [CR#702.72a,702.72b].
- Lean and Rust doc comments on `KeywordParam.quality`/`.subject` record
  which declaration kind writes each.

## Landing record

1. **Census delta** (at `xwuuqwvw`): no grammar, lexicon or construction
   source changed; the keyword-line class of enchant and champion is
   `Subject` before and after. Subsets: `--text-contains "Enchant "` 1240
   faces, 1106 read (437 one, 669 multiple), 134 unread, 0 issues;
   `--text-contains "Champion "` 13 faces, 0 read, before and after (the
   missing article is routed to `english-v3-champion-keyword-line`, minted
   here). Lost faces: none.
2. **Witnesses**: Supreme Exemplar (canon, re-spelled from
   `keywordSubject(label: champion, …)` to `champion(hasSubtype(elemental))`),
   now carrying champion's body, proves `Card.check = []`; English Readings
   0 (routed above). The enchant canon cards keep `keywordSubject` and prove
   unchanged.
3. **Spot checks**: no newly covered English faces. Semantic gain: the
   champion definition, expanded in `lean/Semantics/Macros/Declared.lean`:
   "When this permanent enters, sacrifice it unless you exile another
   [object] you control" as `triggered(enters(thisPermanent),
   doUnless(sacrifice(thisPermanent), perform(exile(a(and([Param(0),
   youControl, otherThan(thisPermanent)]))))))`, and "When this permanent
   leaves the battlefield, return the exiled card to the battlefield under
   its owner's control" as `triggered(leavesZone(thisPermanent,
   battlefield), returnToBattlefield(linkedObject(ByDeed(exile), exile,
   this), [Under(ownerOf(that(Card)))]))`. Facts: champion's `definition`
   is `[triggered, triggered]`.
4. **Deviations and STOPs**: the ticket's STOP (the kind mismatch) cleared
   by declaring the description `Predicate`, not by retyping
   `KeywordParam.subject` or renaming the referent kind `Subject`. Enchant's
   `keyword_params: []` override removed so its argument is forwarded. No
   other keyword declared `Subject`. English: "Champion an [object]" has no
   Reading and the spelling `champion <Param(0)>` drops the article; no
   article feature exists, and adding one is grammar work outside this
   ticket, so it is routed to `english-v3-champion-keyword-line`. No
   glossary gap.
5. **Tests** (at `xwuuqwvw`/`lqwrpooq`): `cargo test -p
   deckmaste_semantics_v2` ok; `cargo test -p deckmaste_construction_core`
   ok; `cargo test -p xtask` ok (407 + 4 + 14, 1 pre-existing ignore);
   `cargo test -p deckmaste_lexical_source` ok; `cargo test -p
   deckmaste_english_v3` ok (731); `cargo clippy -p <crate> --tests -- -D
   warnings` clean for semantics_v2, construction_core and xtask; `lake
   build --wfail` ok; `cargo xtask lean check` 240/240 cards; `cargo xtask
   lean definitions` 711/711; `lean macros check` and `facts check` up to
   date; `cite check` 0 stale, 0 noncompliant. Counts: restored 0;
   re-spelled 3 (`keywords::a_keyword_ability_forwards_its_signature`
   champion case to `Predicate`; `builtin_v2_keyword_abilities` champion and
   enchant signature to `["Predicate"]`; `plugins_v2_declarations` override
   pin moved from champion to prototype, with champion's and enchant's
   forwarding asserted); ignored 0; added 0 test functions (new assertions:
   enchant forwards `Subject(Param(0))`, a `Subject` keyword parameter is
   refused, `Predicate` is a registered param type, `Predicate` sample in
   `keyword_bodies`); removed 0.
6. **Timing**: no full census (grammar untouched); subset census
   `--text-contains "Enchant "` 38.9 s corpus wall, 1 worker; `lean check`
   238,700 ms.
