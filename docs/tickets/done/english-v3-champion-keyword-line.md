---
needs: []
---
**"Champion an [object]" has no keyword-line Reading, and its spelling drops
the article.** Found by `semantics-v2-subject-param-kind` (2026-10-10).
Champion now declares its description `Predicate` and forwards it as
`KeywordParam::Subject`, with its definition body written [CR#702.72a], but
the English side still lacks the article the CR template prints:

- english_v3's `SubjectKeyword` reads `head " " subject: Nominal`, which
  admits "Enchant creature you control" but not "Champion an Elemental": the
  13 supported faces whose text contains "Champion " (Boggart Mob,
  Changeling Berserker, Changeling Hero, Changeling Titan, Lightning
  Crafter, Mistbind Clique, Nova Chaser, Silvar, Devourer of the Free,
  Supreme Exemplar, Thoughtweft Trio, Unstoppable Ash, Wanderwine Prophets,
  Wren's Run Packmaster) read 0 (measured at `xwuuqwvw`,
  `cargo xtask english-v3 --text-contains "Champion "`).
- `champion.ron`'s `spelling` is `"champion <Param(0)>"`, which realizes
  `champion(hasSubtype(elemental))` as "champion Elemental"; the spelling
  template has no indefinite-article modifier ("a"/"an" by the realized
  word).

The keyword's article must come from a declared feature of the declaration
(its `grammar` parameter recipe, as affinity's `Quality(preposition: …)`
declares "for"), never a guard naming the keyword. Prior attempts and the
declaration-only versus referable distinction are in
`docs/tickets/done/english-keyword-ability-parameters.md`. Standard
constraints apply; the census delta covers the 13 champion faces.


## Landing record

1. **Census delta**: bounded comparison at `qzrvqqol` against the exact claim-parent
   census `mqrlzszk` (`target/english-v3/baseline-wave8.json`, schema v8;
   no tree differences from the claim). Selector: the 13 supported faces matching
   `Champion `, all 1,176 faces using `SubjectKeyword` in that census, all 1,240 supported
   faces with an Enchant line (including unread documents), plus 300 random
   covered faces (seed 20261008), union 1,555 identities in
   `target/english-v3/english-v3-champion-keyword-line/manifest.json`.
   Covered 1,459 → 1,470; Readings 14,150 → 14,181. Lost faces: none;
   decreases/increases/digest-only: 0/0/0; gains: 11. After: 85 no-Reading,
   686 unique, 784 multiple, 0 undetermined, 0 issues. All existing
   `SubjectKeyword` face digests are unchanged. Official `english-v3 diff --lines`
   records 27 newly available lines and zero changed existing lines;
   `target/english-v3/english-v3-champion-keyword-line/diff.json` is the evidence.
2. **Witnesses** (`qzrvqqol`): each of the 13 documents was probed before implementation,
   with 0 Readings. After: Boggart Mob 4; Changeling Berserker 3;
   Changeling Hero 3; Changeling Titan 2; Lightning Crafter 2;
   Mistbind Clique 0; Nova Chaser 2; Silvar, Devourer of the Free 0;
   Supreme Exemplar 2; Thoughtweft Trio 3; Unstoppable Ash 3;
   Wanderwine Prophets 4; Wren's Run Packmaster 3. All twelve actual Champion
   keyword lines were separately probed: 0 before, 1 after. Mistbind's separate
   trigger has the vocabulary gap `championed`; Silvar matches the selector
   through the name *Trynn, Champion of Freedom* in its unread Partner line,
   and has no Champion keyword line. These unrelated blockers were not chased.
3. **Spot checks** (`qzrvqqol`): exact `SubjectKeyword` analyses selected in the
   after-probes: Boggart Mob — Goblin noun; Changeling Berserker — creature noun;
   Changeling Hero — creature noun; Changeling Titan — creature noun;
   Lightning Crafter — Nominal Coordination of Goblin or Shaman;
   Nova Chaser — Elemental noun; Supreme Exemplar — Elemental noun;
   Thoughtweft Trio — Kithkin noun; Unstoppable Ash — Nominal Coordination of
   Treefolk or Warrior; Wren's Run Packmaster — Elf noun. Each noun is the
   declared singular count noun in the Subject payload, and the article is
   selected by its Onset. Probe evidence is under the same target directory.
4. **Deviations and STOPs**: orchestrator rulings CH1–CH4 (2026-10-10) control
   this landing. `SubjectKeyword` is refined in place; no Construction or frame
   is added or deleted. The parameter recipe exports `KeywordIndefiniteArticle`;
   a Selected Literal realizes the article from that feature and the Nominal's
   Onset. The same declared feature retires the former bare Champion route on
   the lexical and grammar sides, while retaining bare Enchant subjects.
   Two existing emitter matches explicitly pass through Subject metadata without
   projecting it as Quality metadata, keeping the shared enum exhaustive without
   adding a legacy grammar route. Five regression tests are added, and the
   builtin keyword inventory expectation is re-spelled for Champion's new recipe.
   CH3 residual: `champion.ron` keeps `spelling: "champion <Param(0)>"`;
   checked `SpellingHint` supports only `plural` and `word`, so article spelling
   needs new realization machinery beyond this landing. The initial bounded
   selector used completed-document construction counts for Enchant; it was
   expanded through corpus attestation to include unread Enchant documents and
   rerun before commit. No STOPs or glossary gaps.
5. **Tests** (`qzrvqqol`): all final commands passed:
   `cargo test -p deckmaste_english_v3` (896 tests);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`;
   `cargo test -p deckmaste_construction_core` (474 tests);
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical_source` (84 tests);
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`;
   `cargo test -p deckmaste_semantics_v2` (105 tests);
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings`;
   `cargo test -p xtask english_v3` (56 tests);
   `cargo xtask lean check` (240 cards). Citation checks: 0 noncompliant,
   0 stale; diff audit: 0 newly cited sites. Assurance counts: restored 0,
   re-spelled 1 (`builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`),
   ignored 0, added 5, removed 0. An initial English test build rejected the
   new helper's owned String-to-Ident conversion; passing a string slice fixed
   it, and the complete suite was then run successfully.
6. **Timing**: bounded census at `qzrvqqol`: 9.998 s, 6 workers, 1,555 faces,
   zero issues. The user requires this bounded run instead of a full census;
   no `--all` run was performed.
