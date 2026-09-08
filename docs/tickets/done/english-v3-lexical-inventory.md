---
needs: [english-v3-lexical-model]
---
# Complete the declared lexical input for whole-grammar activation

Consolidate the independent lexical census and source recipes into the v3
lexical engine before the whole grammar is activated. Use existing core and
plugin declarations, open catalogs, keyword catalogs and handwritten numeral
codecs through one analysis interface. Preserve every licensed homograph,
including noun/determinative/numeric `one`, and keep morphology prospective:
grammar will reject contextually inapplicable alternatives.

Apply one documented default per applicable inflection and explicit replacing
irregular overrides. Archived v2 work is optional declaration evidence, not a
scheduling dependency. Map
the supported-corpus remainder into declared morphology/vocabulary,
catalog/notation, structural separators, or named unresolved source gaps.
Unknown words and speculative parts of speech remain errors.

Declare each predicate Lexeme's licensed Frame values through the shared
lexical model. The inventory owns which frames a Lexeme offers; grammar owns
how a frame's slots combine with constituents and whether a Reading is
admissible. No frame may be inferred from a following phrase or from corpus
success.

Consolidate declaration provenance, capitalization, contractions, genitives,
bound and overlapping multiword forms, symbols and opaque names without moving
grammar into lexical analysis. Ordinary-word recipes reject embedded separator
or quote boundaries; catalogs keep their declared atomicity. Analysis and
realization share the same defaults, overrides and surface variants.

Acceptance reruns the supported-corpus lexical inventory, accounts for every
remainder class and exercises all indexed values, including adjective and
Frame inventories, through realization and reanalysis. A short remainder list
is not a grammar-coverage claim. The ticket may leave explicitly named lexical
gaps for the machine-on report, but common closed-class words already required
by the complete grammar must be declared. Standard constraints apply.

Note (2026-09-07): `stubs/flavor_words/` is not lexical input. A flavor
word is an open slot (`flavor-words-are-vocabulary`); the inventory
excludes that family.

Correction (2026-09-08): the user confirmed that the old v2 WIP was archived
under `archive-<old-slug>` and this ticket should not have depended on its
landing. The lexical inventory lands independently of that retired work.


## Landing record

Measured change: `rmkrnxmotrtn`, lock `covered = 20,254`, 2026-09-08.
The accepted independent-lexical-analysis decision governs this landing:
all licensed alternatives survive; no destructive preference or grammar
coverage target is introduced.

### PROVE

The independent source interface now assembles core/plugin declarations,
catalogs, keywords, native closed-class declarations and numeral codecs.
`lexicon/core.ron` declares applicable auxiliary and pronoun bundles, bound
forms and frame additions; `lexicon/overrides.ron` owns replacing irregular
morphology. The lexical README records the single prospective default for
each inflection and the exact boundary, capitalization and variant policies.
Frame markers resolve declared owners and spellings at load time; unresolved
references, empty slots and duplicate additions fail loading. Frames remain
lexical properties, with composition owned by grammar.

The full supported corpus checks byte-exact realization of every lexical
occurrence and realization/reanalysis of all **38,529** independently indexed
values, including adjective and frame owners. Frame declarations also survive
complete serialization equality. Numeral property tests retain independently
generated values. The successful corpus run reports zero internal law failures.
The packed-chart tests retain lexical leaf and derivation identity; this work
adds no grammatical production or admission guard.

No silent loss: matching every original face ID, raw-text digest and word's
byte range/spelling found **zero formerly covered word occurrences becoming
unknown**, and **152,527** newly covered occurrences. The complete named
occurrence delta is `/tmp/english-v3-lexical-inventory-word-delta.json`.
Comparison of complete `(LexicalValue, realized spelling)` sets found **68
retired values**, all classified as wrong/incomplete analyses replaced here:

- **16** values of `lexeme:keyword_action/faceAVillainousChoice`: six finite
  preterite bundles, one past participle and one gerund-participle, each in
  declared and initial case. `face a villainous choiced` and
  `face a villainous choicing` retire in favor of `faced a villainous choice`
  and `facing a villainous choice` (and their initial-capital variants).
- **52** incomplete-feature values, declared and initial variants of these
  26 `vocab:` owners: `ObjectPronoun/{Her,Him,It,Them,You}`;
  `PossessiveAbsolutePronoun/{Hers,His,Theirs,Yours}`;
  `PossessiveDeterminerPronoun/{Her,His,Its,Their,Your}`;
  `ReflexivePronoun/{Herself,Himself,Itself,Themself,Themselves,Yourself,Yourselves}`;
  `SubjectPronoun/{He,It,She,They,You}`. Explicit correlated Case, Person and
  grammatical Number replace missing dimensions. Singular-reference `they`
  retains plural concord. No source spelling disappears.

There are no unexplained losses or re-coverage obligations. The complete
68-value retirement and 579-value addition lists, including identity, Form,
features, variant, capitalization and spelling, are retained in
`/tmp/english-v3-lexical-inventory-value-delta.json`. The baseline export was
migrated only for the new surface-boundary metadata before comparison; no
baseline identities, forms, bundles or spellings were rewritten for this audit.

The existing production no-word-naming checker test passes, with forbidden
checkers at zero. New lexical validation reads declared structure, categories,
binding, features and reference tables; no card/word-specific admission guard
was added. Grammar environment loading remains exercised by the derived gate.

### DISCLOSE

The **579 added values across 140 owners** are named with their full lexical
analyses in the value-delta artifact above. No representative is selected:
all alternatives are preserved. The final report's `lexeme_inventory`,
`lexeme_sources`, `matched_surfaces` and per-face lexical occurrences connect
those values to declared properties, provenance and source spans. Representative
witnesses include independent noun/determinative/numeral `one`, adjective and
verb participles, genitive/auxiliary `'s`, and contracted/finite agreement.

The nine existing frame owners whose signatures changed are
`core-verb:{Cause,Choose,Do,Have,Put,Turn}` and
`vocab:ScalarDegree/{Equal,Greater,Lesser}`. Cause/Choose/Put/Turn reconcile
literal markers to declared identities; Do/Have and the three degree owners
receive explicitly declared complements. Their complete before/after frames
are in `changed_frames`. New owners' frames are in the final inventory.
The two One owners retain their identities while provenance moves from the v2
supplement into the lexical-source override file.

Every remaining spelling and occurrence is named in the final report.
Remainder accounting is exhaustive and does not license these as grammar:

| Class | Distinct spellings | Occurrences | Disposition |
|---|---:|---:|---|
| Unresolved words | 2,804 | 15,946 | Named source gaps for whole-grammar activation |
| Structural separators | 15 | 1,019,607 | Lossless source material; grammar owns admission |
| Notation | 14 | 93,814 | Source material; unknown symbol payloads remain gaps |
| Unresolved nonwords | 1 (`&`) | 14 | Named notation gap for activation |

The report retains 230 source/policy notes: construction literals 120,
plugins 42, vocabulary 34, construction affixes 18, and retired nonattestation
policy 16. Leading unresolved words include `defending` (501), `addition`
(396), `named` (345), `spent` (328), `game` (309), `amount` (290), `spend`
(250), `later` (228), `triggers` (190), `effects` (177), `half` (158), `help`
(158), `form` (150), `door` (145), and `excess` (141). The full lists, including
unmapped declaration identities, route to
[whole-grammar activation](../planned/english-v3-whole-grammar-activation.md)
and then [systemic residuals](../planned/english-v3-systemic-residuals.md).
Those tickets decide grammar applicability; this ticket guesses no missing POS.

Deviations and additions:

- Removed the obsolete v2 prerequisite and corrected the wayfinder following
  the user's explicit clarification that old WIP is archived under
  `archive-<old-slug>`. Archived declarations are optional evidence.
- Added the missing ordinary-word boundary validation and host-boundary
  propagation required by the ticket's bound/overlapping-form contract.
- Salvaged the multiword head-inflection correction individually; the archived
  grammar stack was not transplanted.
- Added Affix and Clitic to the shared category vocabulary and owning glossary.
  These close the glossary gaps needed for explicit negative/genitive input.
- Assurance bookkeeping: **9 tests added, 3 existing tests re-spelled, 0
  restored, 0 newly ignored, 0 removed**. Re-spellings declare the existing
  multiword fixtures' explicit structure in two lexical tests and the packed
  chart's overlap test; their original assertions remain. The gate retains one
  preexisting ignored test. No construction was added or deleted.
- The initial baseline census implements the decision's early-census
  requirement. An initial full acceptance pass exposed omitted common
  closed-class declarations, which were completed before remeasurement.
  A subsequent default refresh added the independent counter declarations
  `lexeme:counter_kind/{decayedCounter,exaltedCounter}` (four case variants);
  the final census was rerun against that source environment. No unresolved
  STOP or contradiction remains.

### REPORT

All final numbers below describe `rmkrnxmotrtn` with lock count **20,254**.
Input: `data/mtgjson/AtomicCards.json`, SHA-256
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.
Scope is 32,641 Vintage-playable faces, 32,285 with raw Oracle text including
reminders; 914,801 word occurrences and 6,824 unique spellings. No normalization
was substituted for raw text.

| Lexical measure | Original baseline | Final |
|---|---:|---:|
| Declared lexemes | 34,447 | 34,560 |
| Independently checked values | 38,018 | 38,529 |
| Returned lexical readings | 1,726,873 | 2,126,929 |
| Unknown word occurrences | 168,473 | 15,946 |
| Unique unknown spellings | 3,148 | 2,804 |

The final inventory includes **168 declared frame signatures**. The complete
named lexical homograph inventory has **871 spellings**, in `homographs` of
the value-delta artifact. Lexical overlaps remain visible in the final report;
form-literal/vocabulary grammar overlaps, construction count, grammatical
no-reading/unique/multiple-reading census, permitted grammar-checker count and
optional-preference effects were **not remeasured**: this is the independent
lexical gate, and no construction or parsing algorithm changed. The parser
coverage lock remains unchanged at 20,254; these lexical figures claim no
additional grammatical coverage.

Final artifacts (session evidence, intentionally untracked):
`/tmp/english-v3-lexical-inventory-final.{json,ron,log}`,
`/tmp/english-v3-lexical-inventory-value-delta.json`,
`/tmp/english-v3-lexical-inventory-word-delta.json`.
Baseline: `/tmp/english-v3-lexical-inventory-before.{json,ron,log}`;
normalized declaration export SHA-256
`33acf88b6eca338eeb56baa500cef107ed0a173dcdbc194180996b09d29a3a49`.
Final export SHA-256
`cfa3120e24f1dea0223dc46c2ce01ed4d8dff86bdfe36d07bb5a174e949efe55`.
Reproduce the final inventory with:

```sh
cargo xtask lexical --output /tmp/lexical-inventory.json --export /tmp/lexemes.ron
```

Performance advisory: the single-worker lexical process took **18.110 seconds
wall**, 17.880 seconds user CPU and 0.184 seconds system CPU. Load/index/value
law/analysis-accounting were 0.377/0.168/0.278/13.895 seconds. Host one-minute
load was 6.54 before and 6.33 after, with the downstream test gate active.
These timings measure the lexical command, including JSON accounting, not the
parser coverage command. The parser's 16.26-second quiet-host ceiling and
accepted-thread nanoseconds per byte were not remeasured; comparing them with
lexical wall time would compare different workloads.

Validation is the reverse-dependency closure derived by
`cargo xtask gate --changed --from sxqosxqp --clippy --run`; the fixed own
claim avoids attributing concurrent default work to this feature:

```sh
cargo test -p deckmaste_english_v2 -p deckmaste_lexical_model -p deckmaste_lexical -p deckmaste_english_v3 -p deckmaste_lexical_source -p xtask
cargo clippy -p deckmaste_english_v2 -p deckmaste_lexical_model -p deckmaste_lexical -p deckmaste_english_v3 -p deckmaste_lexical_source -p xtask --all-targets -- -D warnings
```

The derived test gate passed **1,041 tests in 31 suites**, with zero failures
and one preexisting ignored test. Strict Clippy passed. Full log:
`/tmp/english-v3-inventory-landing-gate.log`. Changed Rust files were formatted
with the repository nightly formatter; no unrelated formatter changes remain.

After the last default refresh, regeneration on the 106-face development subset
again checked every indexed value and produced a byte-identical normalized
lexical export (`cmp` succeeded against the full-census export). The complete
measured lexical environment therefore survives that refresh unchanged.
The required post-refresh derived gate is recorded in
`/tmp/english-v3-inventory-post-refresh-gate.log`.
It also passed 1,041 tests across 31 suites, with zero failures and the same
one preexisting ignored test; strict Clippy passed.
