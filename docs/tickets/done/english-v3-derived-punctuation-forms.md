---
needs: ["english-v3-licence-hygiene"]
---
# Derive punctuation from structure: collapse separator-only form pairs and drop dead curly variants

Priority: medium.

## Why

Owner ruling (2026-10-08): a construction may use a comma to decide between
analyses, but a Reading must not store whether punctuation is present for
the sake of roundtrip. Realisation derives punctuation from structure
wherever Oracle text is consistent.

An audit the same day found that the Reading stores no punctuation leaves.
Punctuation is stored in only two places:

- the per-node `form` index (`Reading::X { form: usize, .. }`, emitted at
  `crates/deckmaste_construction_v3_core/src/emit.rs:485`), in constructions
  whose forms differ only by a separator literal;
- lexical spelling variants (`LexicalValue.variant`,
  `crates/deckmaste_lexical/src/analysis.rs:232–248`).

This ticket removes the stored punctuation that is derivable or dead. It
keeps the punctuation that is true variation in the source text. Expected
coverage gain: 0. The deliverable is fewer constructions and fewer
declaration lines, with the same Reading count on every face.

`needs: english-v3-licence-hygiene` puts the edits to `declarations.rs` in
order. That ticket also changes the *except* separator, and must land first.

Line numbers are those of the claim parent this ticket was minted on
(`twrksupy`).

## Items

### 1. Drop dead curly variants

Curly quotes and curly apostrophes never occur in Oracle text. Checked on
`data/scryfall/oracle-cards.jsonl`:

```sh
jq -r '.. | objects | .oracle_text? // empty' data/scryfall/oracle-cards.jsonl > oracle.txt
for p in "‘" "’" "“" "”"; do rg -o -F "$p" oracle.txt | wc -l; done   # 0 0 0 0
```

For comparison, the same text has 14,992 straight `'` and 5,155 straight
`"`. Under the ADR pruning rule (`docs/decisions/english-lexical-analysis.md`,
"Pruning rule for licensed functions and attachments"), a variant with zero
attested uses is removed. The full census must still lose zero faces.

Remove these:

- `declarations.rs`: the curly forms of `QuotedText` (`:1587`),
  `QuotedClause` (`:1594`) and `QuotedKeyword` (`:1602–1603`).
- `crates/deckmaste_lexical_source/lexicon/core.ron`: the curly second
  spelling in every `surfaces: Some(["…'…", "…’…"])` pair. They are on lines
  1133–1138 (*don't/doesn't*), 1147–1157 (*be* negatives), 1166–1171 (*'m/'re/'s*),
  1182–1193 (*'ve/'s/'d*), 1201 (genitive *'s*), 1208 (genitive *'*),
  1306–1317 (*can't*) and 1358–1363 (*didn't*): 55 lines in all. A
  single-element list stays a list (`Some(["don't"])`). Do not replace it
  with `None`: these forms have overrides.
- `crates/deckmaste_construction_v3_core/src/runtime.rs.txt:105`: the `'”'`
  arm of `SurfaceSummary::literal`. Once the curly quote forms are gone,
  nothing can produce that character.

After this removal, `LexicalValue.variant` still has a multi-variant surface
in `core.ron` in two places: `a`/`an` (`:844`) and `non`/`non-` (`:1215`).
Participial adjectives built in
`crates/deckmaste_lexical_source/src/native.rs:154–165` can also have
several surfaces. All of these are legitimate. Keep the mechanism. This
ticket does not remove it.

`crates/deckmaste_lexical_source/lexicon/verbs.ron:53` contains curly
quotes only inside a comment. Leave it alone.

### 2. Collapse derivable separator-only form pairs

#### Mechanism: a feature-selected literal (construction-core extension)

The DSL cannot express this today. A form piece is a fixed `Literal` or a
field (`Piece`, `ir.rs:92–96`; parsed at `parse.rs:1091`). A literal is scanned as
`Symbol::Literal` (`ir.rs:651`) and written verbatim by `form_pieces`
(`emit.rs:364–367`).

The existing "structure-derived punctuation" is not a realisation-time
choice. `terminal_punctuation` is a built-in summary domain (`ir.rs:214–218`,
exempt from the interface check at `ir.rs:910`). The runtime computes it from
the literals a child scanned (`runtime.rs.txt:100–110`, and
`emit.rs:408` for repeated fields). `Sentence` (`declarations.rs:1884–1890`)
still declares two forms, `[clause, "."]` and `[clause]`, and picks one with
mutually exclusive `require`s on that summary. So the Reading stores a form
index that the child summary already fixes. That is the same redundancy this
ticket removes.

Forms whose pieces and requirements are equal already share one index
(`ir.rs:642–645`). Forms that differ by a literal never share one.

The minimal extension is a literal that a feature-table lookup selects. It
is bounded to the following.

- **Declaration.** A table whose result is a string literal, for example:

  ```
  separator keyword_separator(KeywordSeparator) {
      (Space) => " ", (Dash) => "—", (SpacedDash) => " — ",
  }
  ```

  A form uses it as a piece:
  `form [head: lexical(Keyword), keyword_separator(head.KeywordSeparator), body: …]`.
  Arguments use the same reference grammar as table requirements. Use a new
  keyword or a result type, whichever fits `parse_table` (`parse.rs:282`)
  more cleanly. Use the name *separator* or *selected literal*, whichever
  the glossary supports (see Method).
- **IR and scanning.** The piece expands to one `Rule` per table row. Each
  rule holds that row's text as `Symbol::Literal`, plus a table requirement
  that the lookup equals that row. All of those rules build the same
  `Form`, which has one index. Canonicalisation (`ir.rs:642`) compares the
  selected piece by table and arguments, not by text. A row whose text is
  empty contributes no symbol. Normalisation already drops empty literals
  (`normalize_form`, `ir.rs:802–813`).
- **Admission and realisation.** `form_pieces` (`emit.rs:354`) resolves the
  lookup from the argument fields' summaries, or from the lexical value for a
  `lexical(...)` field. It then feeds the resulting text to
  `surface.literal` for the summary and pushes it in `write`. A failed
  lookup is an `Error::Invalid`, the same as a failed table requirement.
- **Excluded:** new xtask tooling, flags and fixtures. A change to the
  construction core is in scope, because it is grammar infrastructure. Add
  `deckmaste_construction_v3_core` tests in `src/tests.rs`. Model them on
  `surface_requirements_distinguish_forms_without_defeating_normalization`
  (`tests.rs:34`): one index for a selected-literal form, scanning each row,
  and realising each row.

The byte-exact roundtrip law must still hold. On every covered face, the
separator realised from features must equal the separator that was scanned.
Each collapse is valid only if no lexeme or summary admits both separators.
Check that per pair, as shown below.

#### (a) `Coordination` (`declarations.rs:1226–1232`)

The two forms, `" "` (`:1227`, `require coordinator.BareCoordination = Yes`)
and `", "` (`:1229`, `require coordinator.CommaCoordination = Yes`), become
one form with a selected separator over the coordinator's declared feature.
The two features are complementary on every coordinator. The defaults are
`BareCoordination = Yes` and `CommaCoordination = No` (`:227–228`). The only
override is `vocab:Coordinator/Then` (`core.ron:850`: `Bare = No`,
`Comma = Yes`). So *then* realises `", "`, and *and*, *or* and *but* realise
`" "`. Key the lookup on `CommaCoordination`. Keep `BareCoordination`,
because `ClauseCoordination` (`:2006`) still reads it.

#### (b) The four keyword constructions' `KeywordSeparator` child

`ClausalKeyword` (`:1848–1853`) agrees `head.KeywordSeparator =
separator.KeywordSeparator`. `AmountCostKeyword` (`:1818–1826`),
`QualityCostKeyword` (`:1827–1837`) and `CostPowerToughnessKeyword`
(`:1854–1861`) require `parameter_separator(head.KeywordParameterSeparator,
separator.KeywordSeparator) = Yes`. That table (`:1739–1741`) is the
identity. So in every case the head lexeme's declared feature fixes the
separator. Replace the `separator: KeywordSeparator` field with a selected
separator over that feature: `KeywordSeparator` for `ClausalKeyword`, and
`KeywordParameterSeparator` for the other three.

Then delete the following, which nothing else uses:

- `SpaceKeywordSeparator`, `DashKeywordSeparator` and
  `SpacedDashKeywordSeparator` (`:1782–1793`);
- `category KeywordSeparator(KeywordSeparator)` (`:328`);
- `table parameter_separator` (`:1739–1741`);
- the `agree` and `require` lines that read it.

The feature declarations at `:191–192` stay, because lexemes declare them.
The Reading of these four constructions loses a child node. Expect a changed
digest on every face that any of the four covers.

#### (c) Other pairs already fixed by a declared feature or child summary

A scan of every construction and schema with two or more forms found these:

- **`Sentence`** (`:1884–1890`). `[clause, "."]` requires
  `clause.terminal_punctuation = None`, and `[clause]` requires
  `QuotedFullStop`. Collapse it to one form with a selected terminal over
  `clause.terminal_punctuation`: `None` gives `"."` and `QuotedFullStop`
  gives `""`. This is in scope.
- **`LicensedMatrixComplementPredicate`** (`:1311–1318`). `" "` goes with
  `MatrixComplementRequirement = Nominal` and `", "` with `Content`. This is
  the *except* separator. Licence-hygiene item 4 widens it, so it is out of
  scope here. Once that ticket lands, check whether the separator is still
  a function of the requirement. If it is not, it is source variation and
  stays.
- **`CoordinatedClauseComplementPreposition`** (`:4033–4036`). Its two
  forms have the same surface and differ only in their requirements. That is
  not punctuation, so leave it.

The implementer re-runs the scan on the post-hygiene tree. Any newly found
pair is collapsed only if a declared feature or child summary fixes its
separator on every admitted value. Otherwise it goes on the list in item 3.

### 3. Pairs that stay as stored bits

Oracle text attests both spellings of these pairs over identical structure.
They are source data, and a form index is the sanctioned place to record
them.

- **`KeywordContinuation`** `", "` / `"; "` (`:1705–1708`). Which one
  appears depends on the card's era. The same keyword list appears both ways.
- **`InitialAdverb`** `"Otherwise, "` / `"Otherwise "` (`:3847–3852`). Both
  are attested with the same adverb and the same clause (222 vs 3).
- **`InitialPreposition`** `", "` / `" "` (`:1974–1983`). Both are attested
  for *instead* with the same PP. `ClauseInitialAdjunct = Yes` licenses the
  bare form, but it does not fix it.
- **`ClauseCoordination`** `", and"` / `" and"` (`:2003–2009`). Both are
  attested with the same coordinator.
- **`QuotedKeyword`** `"…"` / `"….\""` (`:1600–1601`, straight quotes). Whether a quoted
  keyword ends in a full stop is source variation, and the quoted keyword
  phrase is the same. Confirm with rg that both are attested. If only one
  is, it is dead and goes under item 1.
- **`Supplementation`** (`:1895–1900`) has only the `", "` form. The single
  bare `" where"` (Devoted Sultai) stays unread. Do not add a form for it.

## Method

Standard constraints apply. Deltas:

1. Verification budget (binding): one full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature. Compare it face by face
   (per-face `readings_sha256` and counts) against the claim parent's
   census. Iterate on selector subsets, never on `--all`.
   - Tests: `cargo test -p deckmaste_english_v3`, `-p
     deckmaste_lexical_source` and `-p deckmaste_construction_v3_core`, with
     clippy on each.
   - No xtask tests and no Lean checks.
   - No re-census after refresh unless the refresh touched the grammar,
     the lexicon or the construction core.
   - New tests go under `tests/suite/` and are registered in
     `tests/main.rs`. Evidence goes in the workspace's ignored
     `target/english-v3/`.
2. Proof. The census must show:
   - zero lost faces;
   - an identical Reading count on every face;
   - no changed digest except on faces that a construction from item 2(b)
     covers, or that have a node whose stored form index changes because
     forms merged (`Coordination`, `Sentence`).

   Name the constructions whose faces changed digest, and give a count for
   each. Any other digest change is a defect.
3. Tests:
   - one roundtrip test for each collapsed pair, covering both separators
     where both exist (*then* and *and*; Space, Dash and SpacedDash; `"."`
     and a quoted full stop);
   - re-spell every test that names a removed form, a
     `*KeywordSeparator` node or a curly variant. Known ones are
     `keyword_payloads.rs:222,325`, `granted_ability_quotes.rs`,
     `lexical_compositions.rs`, `grammar.rs`, `counter_compounds.rs` and
     `keyword_payload_features.rs` in the lexical-source suite, and
     `deckmaste_lexical/tests/analysis.rs`;
   - never delete a test.
4. No guard may name a lexeme, construction, verb, noun, preposition or
   card. Lookups read declared features or summaries only.
5. Glossary: if `docs/contexts/oracle-english/CONTEXT.md` has no term for
   the feature-selected literal, add one through the `domain-modeling`
   skill.
6. No CR citations. A project ruling is cited as a ruling.

## Landing record

1. **Census delta.** Claim parent `oxvkxppy`: 23,460 covered faces and
   1,605,559 Readings; finished feature `xtxrnswz`: 23,460 and 1,605,559.
   Lost faces: none. Removed Readings: none; every face retains its count.
   The parent baseline reconciles the signed-numeral-cost full census with
   licence-hygiene's refreshed subset; its grammar, compiler and lexical
   sources match `oxvkxppy`. At `xtxrnswz`, 1,981 faces change digest, with
   AmountCostKeyword 74, ClausalKeyword 111, Coordination 702, CostPowerToughnessKeyword 13, ExistentialClause 7, FiniteClause 368, ObjectRelativeClause 0, PronounSubjectRelative 0, QualityCostKeyword 21, ScalarLocation 0, Sentence 568, SubjectRelativeClause 215, ZeroObjectRelativeClause 0 (all measured at `xtxrnswz`). Counts overlap on faces with several changed nodes. Refresh brought no
   grammar, lexicon or construction-core changes; these results remain
   applicable.
   No unexplained digest changes, internal failures, roundtrip failures or
   traversal failures (`xtxrnswz`). Evidence: ignored
   `target/english-v3/english-v3-derived-punctuation-forms/`.
2. **Witnesses.** After counts (`xtxrnswz`): Stitcher's Supplier 1;
   Takklemaggot 0; Winter, Cursed Rider 0; Dance of the Dead 4,800;
   Animate Dead 84. Both straight-quoted keyword spellings remain attested:
   Dance of the Dead and Animate Dead have the unpunctuated quoted enchant
   phrase and the quoted enchant clause ending in a full stop; Mind Bend
   has quoted “islandwalk.”. The curly variants have zero corpus attestations
   (`xtxrnswz`); no bare supplementation route was added.
3. **Spot checks.** No newly covered faces (`xtxrnswz`), as expected for
   this zero-gain ticket. Ten affected faces inspected at `xtxrnswz`:
   Rift Bolt (1), AmountCostKeyword; Kodama's Might (2), QualityCostKeyword;
   Rust Goliath (1), CostPowerToughnessKeyword; Goldmaw Champion (2),
   ClausalKeyword; Obsessive Stitcher (2), SecondaryVerbPhrase Coordination;
   Verdant Haven (9), Sentence containing a quoted ability; Llanowar Mentor
   (6), quoted activated ability with an outer Sentence; Cogwork Assembler
   (4), SubjectRelativeClause with a clitic predicate; Mistform Wall (18),
   FiniteClause with a clitic predicate; Bounty Agent (5),
   SubjectRelativeClause with a clitic predicate. Each collapsed node uses
   form 0 (`xtxrnswz`); selected analyses are retained in `spotchecks.json`.
4. **Deviations and STOPs.** The ticket's post-hygiene scan additionally
   collapsed seven feature-fixed space/empty pairs: FiniteClause,
   ScalarLocation, ExistentialClause, PronounSubjectRelative,
   SubjectRelativeClause, ObjectRelativeClause and ZeroObjectRelativeClause
   (`xtxrnswz`). These read CliticHost or Clitic, preserving the declared
   contraction conditions. Identical table outputs share a scan production
   to avoid duplicate derivations. No new constructions or frames. Removed
   three keyword separator constructions (216 → 213), four curly quotation
   forms, 56 curly lexical spellings, and the obsolete parameter_separator
   and joined_clitic tables; added six selected-literal tables (`xtxrnswz`).
   `declarations.rs`: 4,480 lines (`oxvkxppy`) → 4,448 (`xtxrnswz`), net
   32 removed; `core.ron`: 1,368 → 1,368, net 0 removed, with lexical
   overrides retained. The except separator and all other attested source
   variations remain. The Selected Literal glossary gap was filled using
   the ticket's owner ruling, with no CGEL claim. STOPs: none.
5. **Tests.** At `xtxrnswz`, `cargo test -p deckmaste_english_v3` passes
   498 tests; `cargo test -p deckmaste_lexical_source` passes 57;
   `cargo test -p deckmaste_construction_v3_core` passes 27.
   `cargo clippy -p deckmaste_construction_v3_core -p deckmaste_english_v3
   -p deckmaste_lexical_source --tests -- -D warnings` passes; the final
   lexical-source test adjustment also passes
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`.
   Changed Rust files pass rustfmt checks. Assurance counts (`xtxrnswz`):
   restored 0, re-spelled 7, ignored-with-blocker 0, added 5, removed 0.
   The 24-face iteration subset and the single full census pass; a
   changed-face selector diagnostic accounts for the merged forms
   (`xtxrnswz`). No xtask tests or Lean checks were run.
6. **Timing.** The one full census at `xtxrnswz` took 205.919 seconds
   with 12 workers and zero samples per face.

## Out of scope

- The *except* separator (licence-hygiene item 4).
- Normalising the source Oracle text.
- Removing the `LexicalValue.variant` mechanism.
