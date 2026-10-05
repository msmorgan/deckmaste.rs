---
needs: [english-v3-article-variants]
---
# Connect declared keyword parameters and italic ability heads

Extend the v3 keyword declaration consumers beyond Nullary/Amount/Cost to
Quality, Subject, AmountCost and QualityCost. Replace supported uses of legacy
`Unsupported(Ability)`, `Unsupported(Condition)` and
`Unsupported(CostPowerToughness)` metadata with positive typed payloads for an
Ability, grammatical condition Clause, or cost-plus-slash-pair respectively.
Use declaration-backed free/bound surfaces, prepositions and separators; do
not introduce a generic Unsupported leaf. The baseline's 1,783 matching failed
faces overlap other groups, and its declaration list includes out-of-scope
entries: only Vintage-supported witnesses establish implementation scope.

Pinned shape: consume ordinary nominal, measure, clause, cost and Ability
Categories in generated bidirectional Productions. Keyword parameter shape
and surface requirements are lexical declaration data; common feature/frame
constraints govern admission. Costs retain symbolic and action components and
their written separators. Preserve quality coordination, selected prepositions,
bound keyword surfaces and keyword-line/running-text/quoted positions.

Connect `Lexicon::analyze_italic_head` to generated Ability labels. Ability
Words are declared vocabulary; Flavor Words are a bounded open label at an
italic ability head, not a guessed word class or enumerated vocabulary. Audit
the helper's current classification by any Keyword-category match: only a
declared Ability Word warrants that label. Establish the raw-source head
boundary at the start of an ability line. Preserve the label verbatim, and parse
the body normally. User ruling (2026-10-05): lexical analysis emits an additional
FlavorWord for arbitrary line-head text followed by an em dash. Its consuming
construction costs 100. Declared Ability Words keep both readings and their
cheaper declared analysis; no candidate is suppressed merely because the head
also has a declared or layout interpretation. This supersedes the earlier
requirement to exclude such lexical candidates.

| Witness | Required structure |
|---|---|
| Myr Enforcer | Quality plus selected preposition. |
| Pacifism | Keyword Subject, retaining its ordinary nominal structure. |
| Rift Bolt | AmountCost with exact dash and symbolic cost. |
| Kodama's Might | QualityCost with selected preposition and cost. |
| Rust Goliath | CostPowerToughness with two ordered scalar components. |
| Usher of the Fallen | Ability payload, distinct from an italic label. |
| Lutri, the Spellchaser | Condition payload, distinct from an italic label. |
| Aberrant; Acolyte Hybrid | Open FlavorWord label over a parsed Ability. |
| Jeweled Spirit; Giver of Runes | Quality alternatives retain their coordinator and attachment. |

Acceptance fetches the supported text and asserts these structures at both
focused roots and connected hosts. Reject payloads on declared nullary syntax,
missing required parameters, ordinary nouns acquiring keyword-only costs, and
an unknown ability body accepted merely because a flavor label precedes it. Add an unseen
synthetic FlavorWord label so the test cannot pass through an inventory.
Reconcile all 196 inherited flavor identities individually: a parsed head
does not discharge a failing body; route remaining body causes to the final
systemic reconciliation. Preserve valid alternative nominal/PP analyses.

Apply [Lexical analysis](../../decisions/english-lexical-analysis.md#lexical-analysis),
[Source and roundtripping](../../decisions/english-lexical-analysis.md#source-and-roundtripping)
and [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity):
Earley parsing, correlated packed partial/complete derivations, generated
checked construction/rendering/total traversal, both roundtrip laws and all
valid Readings. The retired English Lean prerequisites are superseded by the lexical-analysis
decision; Rust tests and corpus validation govern this landing. STOP and report
rather than use keyword/card-named guards, arbitrary body payloads (the open flavor label is expressly authorized), eager AST
products, destructive preference or game-semantic admission. V2 remains
untouched. Standard constraints apply.


## Landing record

Measured change `xpspnuqpqmzvpvmkyxwvmvxrzukqsxxq`, lock covered count 11,394. The prior
verified corpus is systemic-hosts-final, 11,249 covered. Input SHA-256
49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb and lexical-inventory SHA-256
d261c3ea60c7b9f32c8b8b8b8972f3a1a15663cf831670e3d1270dd7a31cdd6e are unchanged. Lexical engine and grammar
changes are identified by the measured change, not by the inventory hash.

The open lexical FlavorWord value retains its label and realizes the exact
space/em-dash suffix. Analysis emits it only at text/line start, independently
of grammar; a declared Ability Word also retains its ordinary lexical reading.
FlavorWordHead costs 100 versus the ordinary head's default 1. Both parse the
body normally, with a single label per ability. Nested labels and labels inside
keyword Ability payloads are excluded so independently supplied values remain
reparsable under the same lexical boundary rule. The helper now checks declared
LabelKind, rather than treating every Keyword Lexeme as an Ability Word.

Astra's NP audit corroborates the determined quality in Jeweled Spirit and
Giver of Runes: a determiner-bearing NP cannot be called a Nominal. The existing
Nominal quality readings remain; a separate NP quality retains common/accusative
case and excludes targeting. Existing Targeting summaries propagate through
NP constructors and ordinary/correlative coordination using logical OR. This
prevents targeted qualities, including mixed coordinands, without discarding
ordinary mixed NP coordination. Myr Enforcer explicitly has both its retained
Nominal and additional bare-plural NP quality values.

The complete, uncapped 24-worker census covers all 32,828 supported faces:
21,434 no, 5,768 one, 5,626 multiple. Exactly 145 new covered identities,
zero lost identities; 645 identities change reading count. All 120,836 Readings
pass declaration admission, lexical validation, exact realization and both
construction/word traversal checks. Zero issues, failed or limited enumerations,
duplicates or internal failures. Costs rank samples only; no readings are pruned.
Each gained identity has its selected sample/cost/construction fingerprint in
`data/reports/english-v3/keyword-labels-xpspnuqp/reconciliation.json`, alongside
`census.json`. No change to declared vocabulary means the declared homograph
and literal/vocabulary overlap inventories are unchanged; FlavorWord is an open
lexical class, not an enumerated vocabulary. No word-naming licensing checker
is introduced.

The inherited 196 identities reconcile individually in `flavor-identities.json`
and are reproducible with `flavor-selection.json` in that report directory.
59 regain whole-face Readings. All remaining 137 explicitly route to
[systemic residuals](../planned/english-v3-systemic-residuals.md): 112 have at
least one probed ability body without a Reading; 25 have readable probed bodies
but unresolved connected hosts/other abilities. The 260 body probes distinguish
body parsing from head consumption; this is not a sole-cause classification.
Companion/Condition witnesses, including Lutri and Zirda, retain missing ordinary
body constituents (starting-deck, quantified/card-modified NP and activated-ability
phrases); the typed Condition consumer is present. Landwalk's declared bound
suffix retains its properties but still needs a declared stem class and bounded
consumer; that obligation also routes to systemic residuals. These explicit
body/stem residuals are not claimed as discharged by label support.

Deviations and additions: two named constructions are added, FlavorWordHead
(open lexical label over an ordinary Ability) and NounPhraseKeywordQuality
(the genuine NP/Nominal distinction). The compiler rejects mixed child Categories
in one constructor's surface alternatives, so the second cannot honestly be
another form of NominalKeywordQuality. The grammar has 164 ordinary constructions
and 53 schemas (217 named), 318 typed instances, 534 chart productions and
2,552 declaration lines, maximum width 100. No new glossary term is needed.
Two English test files are added: italic heads (four tests) and NP qualities
(two tests). Two lexical tests are added. Existing Myr expected values are
extended to assert the exact two-Reading set; no fixture is removed or ignored.
Existing fixture adapters exhaustively handle the new lexical variant.

Observed STOP/resolution: trying to reuse NominalKeywordQuality with an NP form
failed the compiler's child-category identity contract. A distinct grammatically
justified constructor resolves it. Its valid extra Myr NP reading then exposed
the old singleton assertion; both independent expected values are retained.
A constructed nested label would violate the reparse law; LabelKind summaries
and independent negative AST tests resolve that path. No unexplained coverage
loss or unresolved validation failure remains.

Performance advisory: complete corpus wall 212.398 s versus the
16.26 s advisory, 972,512 ns/B checked-text thread CPU,
24 workers, host load 2.96/5.86/4.69. The existing attachment/enumeration
cost remains a systemic obligation. Reproduce: cargo xtask english-v3 --all
--workers 24 --samples-per-face 1 --output REPORT.json.

Validation: cargo xtask gate --changed --run exits 0: 690 tests pass across
76 suites, one existing on-demand live-corpus test ignored. This is the derived
reverse-dependency gate for the changed lexical model, lexical engine, grammar
compiler/runtime, English parser and corpus report adapter. All six new English
tests and both new lexical tests pass; no existing test is removed or newly
ignored. cargo fmt --all --check passes; cargo xtask cite check checks 15,871
citations with zero stale. The previous keyword payload tests and their connected
hosts remain covered by this gate; unresolved Condition bodies are explicitly
routed rather than replaced with invented positive fixtures.
