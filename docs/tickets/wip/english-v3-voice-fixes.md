---
needs: []
---
# Fix passive selection and duplicate ellipsis readings

Audit findings: Voice travels through 16 category signatures and 121 exports,
with only two admission requirements. Finite/infinitival summaries are discarded.
Six empty ellipsis constructors independently choose Active/Passive; Skyfisher
Spider's two do omissions account for a fourfold 288/72 duplication. The current
summary also admits bare passive Destroy as perfect Have's complement and the
malformed auxiliary chain `be been destroyed`.

Replace coarse passive selection with precise declared complement structure;
retain genuine perfect/passive chains. Represent an unresolved ellipsis once per
selected form, without inventing its voice. Remove unused Voice propagation and
preserve actual passive, adjectival and mixed-coordinate structures. No blanket
English-wide do implies Active constraint. CGEL: pp. 1430–1431 (bare/expanded
passive), 1521–1524 (selected form and recoverability), 1327 (mixed coordination).
Use authentic Oracle constituents for positive independent AST tests; malformed
chains are negative admission tests. Preserve both roundtrip laws and compare
complete 24-worker corpus identities/readings against the type-macro baseline.

The standalone audit has been removed at the user's direction. Scratch evidence:
`/tmp/english-v3-voice-audit.md`; lasting findings belong in tickets. No additional
standalone documentation is needed for this implementation.

## Landing record

### PROVE

Voice has been removed from all 16 category signatures and 121 exports. Local
ParticipialUse is carried by only SecondaryVerbPhrase and its two series, with
47 exports. Ordinary lexical/auxiliary predicates are distinct from bare passive
participial structure; adjuncts preserve that distinction and coordination keeps
uniform or mixed uses. Passive complement selection requires a bare passive;
perfect selection requires ordinary participial structure. No lexical owner,
construction identity or card-naming guard is introduced.

Six empty omission constructors are replaced by three form-selected unresolved
ellipses. Their missing voice/antecedent is not invented. Selected Plain,
GerundParticiple and PastParticiple forms and passive/perfect hosts remain
distinct. This fixes duplicated readings without imposing do implies Active.
Antecedent resolution is an existing broader residual, not implemented here.

Independent AST tests preserve authentic Thieves' Auction and Struggle for Sanity
perfect passives and Sneaky Homunculus ordinary/passive coordination. Parser and
independent admission reject malformed repeated passive auxiliaries and bare
passive selection by perfect Have. The existing attested `you do` tree is
re-spelled against OmittedPlain and now requires exactly one Reading.

All 32,828 source identities reconcile with the inherited complete type-macro
census: 6,186 covered faces retained, zero losses/gains; unique 4,182 → 4,246 and
multiple 2,004 → 1,940. Complete Readings 29,653 → 28,799, all 28,799 byte-exact
and validated; zero issues, failures, limited enumerations, duplicates, cycles
or internal errors. Exactly 132 faces reduce their Reading counts, none increase;
each changed face previously had empty omission alternatives. Spider 288 → 72.
The remaining 6,054 covered faces retain their counts. Detailed per-face Reading
changes are scratch `/tmp/voice-fixes-reading-changes.json`. No Type Line grammar
or lexical source changes; lexical inventory hash is unchanged.

### DISCLOSE

The 854-Reading reduction concerns unsupported empty-voice alternatives. The
confirmed auxiliary-chain admission defects are fixed by grammatical selected-use
constraints; no previously covered face is lost. Preserve all actual passive,
adjectival and coordinate AST structure and both roundtrip laws. The three new
tests do not make invented nonsense positive fixtures.

Tests: restored 0, re-spelled 1, ignored 0, added 3, removed 0. Retired omission
constructors: OmittedPlainActive/Passive, OmittedGerundParticipleActive/Passive,
OmittedPastParticipleActive/Passive; replacements OmittedPlain,
OmittedGerundParticiple, OmittedPastParticiple. All six old constructor names
retire with their redundant voice distinction; no other construction is removed.
Glossary additions define Bare Passive and local project term Participial Use
with CGEL sources; existing Voice and Ellipsis rulings remain valid.

Documentation/history: at the user's direction the original audit change was
rewritten to contain only ticket evidence. It never adds the standalone audit
document; no add-then-remove document changes remain. Detailed audit scratch is
`/tmp/english-v3-voice-audit.md`. No standalone doc added by this feature.

### REPORT

Measured change `oooowkkqlpstpvzksvttnulmxylzyltz`, covered 6,186, 24 workers,
unlimited Readings, one retained sample per face. Source SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`; lexical SHA-256
`3d56de49f2e3203130fc710231516aa3dfa67247a944ef330c1f79395c28cae1`.
Corpus `/tmp/voice-fixes-after.json`; baseline `/tmp/type-macros-after.json`,
measured baseline change `opopkxsxzqlntxxkxpopnzuktvlksrsq`, covered 6,186.
The census exercises 170 construction kinds; the declaration contains 462 constructions.
Lexical homograph and form-literal/vocabulary overlap inventories are unchanged
from the stamped type-macro baseline: no lexical source or morphology changes.
No legacy coverage lock or selection-arbitration claim applies to this v3 census.

Performance advisory: corpus wall 54.969 seconds, above the 16.26-second quiet-host
reference; checked-text thread CPU 911,050 ns/B, host load
[22.95556640625,11.49755859375,6.5439453125], 24 workers. Package compilation/tests
overlapped the census; this is provenance, not a fitted performance gate.

Focused tests: 29 passed, 0 failed. Changed-path gate derives
`cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
Formatting check passes (stable rustfmt warns about nightly-only configuration).
Citation listing: 0 noncompliant; 15,844 citations checked, 0 stale. No CR citation
was added or changed. Declaration review found no missing selected-use exports
or unintended outward propagation.

The gate retains the existing ignored xtask test
`macros::templates::tests::macro_schema_census_count_matches_21`, whose documented
reason is an on-demand live-corpus census cross-check. This feature adds no
ignore attributes and changes none. Shared-be coordination with unlike outer
participial forms remains an existing breadth residual; complete ordinary/passive
coordinate predicates retain their independently checked structures.

Final gate exits 0: 522 tests pass across 44 suites, no failures;
1 existing ignored test as disclosed above. Lean integration checks and
doc tests pass. The final formatting check exits 0.
