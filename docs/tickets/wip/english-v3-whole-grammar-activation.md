---
needs: [english-v3-generated-roundtrip-slice, english-v3-lexical-inventory]
---
# Declare the complete interconnected grammar and turn it on

Translate the whole intended Oracle English grammar from the reviewed source
map and corrected Lean model into v3 construction declarations, then run that
grammar against the complete supported corpus. This is deliberately one
top-down activation. Do not schedule or declare victory family by family before
their interactions exist.

Every planned family must have real general Productions and constraints:
documents; sentences and clauses; predicates, lexical frames and agreement;
nominals, determiners and adjectives; subordination; prepositions; relatives
and extraction; coordination, sharing and scope; measures; anaphoric forms and
locally licensed ellipsis; Type Lines; and Target Verb/Targeting Marker
homography. Each family must interact with at least one other family. Missing
lexemes or uncommon variants may remain explicit residuals; permissive
catch-alls, unused declarations and opaque source leaves do not count as
grammar.

Port useful v2 declarations and regression witnesses by meaning, never by
compatibility shape. The grammar consumes the independent lexical layer and
fresh compiler exclusively. Preserve all grammatical Readings, distinguish
duplicate derivations, and enforce every grammatical constraint during
admission or as an explicit deferred constraint before a Reading is exposed.
One declaration continues to drive parsing, checked construction, rendering
and traversal.

The first complete run is an experiment, not a coverage ratchet. Report no,
one and multiple Readings; lexical gaps; invalid-reading findings; byte-exact
and constructed-value roundtrip failures; chart/forest metrics; compiler/build
cost; and corpus runtime. Compute the Reading census only after every deferred
constraint has run; a forest whose candidate families all fail deferral counts
as no Reading. Sample successes and failures across every family and group
failures by shared grammatical, lexical, compiler or parser cause. The full
inherited obligation register is acceptance evidence for the activation; no
named witness may disappear because its former family ticket was retired.

Acceptance is a compiled whole grammar, an actual v3 supported-corpus command,
a reproducible baseline, and a cause-grouped residual report. It does not
require full corpus success. Standard constraints apply.

Note (2026-09-07): `flavor-words-are-vocabulary` deleted the enumerated
flavor-word declarations, and with them english_v2's flavor-word codec and
its mode-marker and label-term constructions. The 196 corpus identities that
stopped being covered are listed in that ticket's landing record and are
this activation's to re-cover: the italic head is one construction over an
ability word [CR#207.2c] or a flavor word [CR#207.2d], and the flavor word's
label is the run itself, analysed by `deckmaste_lexical`.

## Implementation progress

The activation remains incomplete. `english_v3::grammar` is now the connected
production candidate, alongside the retained bounded `english_v3::slice`
compiler/runtime witness. No supported-corpus grammar baseline has been claimed
or measured.

Compiler prerequisite, change `yoqwmwru` (2026-09-08): declarations can export
typed constants and apply finite feature tables to correlated child features.
These supply derived agreement and dependency summaries without handwritten
admission hooks. Missing table rows reject during chart completion and checked
construction, before any Reading is exposed. Table inputs remain in the finite
register state until completion. The compiler rejects duplicate tuples, wrong
domains/arity, inaccessible features and conflicting Category exports.

Validation: `cargo xtask gate --changed --clippy --run` derived
`cargo test -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_english_v3`
and the same package closure for strict all-target clippy; both passed.
The 51 tests include independent constructed roundtrips, all nine mixed-person
pairs, agreement negatives and missing-row rejection before materialization.
Assurance delta so far: 5 added, 0 restored, 0 re-spelled, 0 ignored, 0 removed.
These are compiler witnesses, not whole-grammar family acceptance evidence.

Remaining work retains this ticket's entire scope: translate the connected
source map and corrected Lean judgments, declare the lexical distribution data
needed by those Productions, preserve and exercise the full obligation register,
add the actual v3 supported-corpus command, and establish the complete baseline
and cause-grouped report before review and integration. In particular, the
initial lexical inventory had no determiner Countability licensing, and most
determiners have no Number bundle; absence cannot become a grammar wildcard.
The article's two spelling variants also need an applicable onset constraint.
These source/interface findings must be resolved or explicitly measured as
residuals; permissive admission does not discharge them.

Lexical distribution checkpoint, change `yxntkrys` (2026-09-08): 26
determinatives now declare their nominal Number/Countability selection as
`DeterminerUse`. The generic source adapter rejects unresolved owners and
duplicate or empty feature additions, and attaches per-feature provenance.
No inflectional bundle or spelling has changed. These declarations await their
whole-grammar consumers; they do not themselves establish grammar coverage.
The expanded derived test/clippy gate passed across
`deckmaste_construction_v3_core`, `deckmaste_lexical_source`, `xtask`,
`deckmaste_construction_v3` and `deckmaste_english_v3`: 576 tests passed,
1 preexisting ignored, strict all-target clippy clean. Cumulative assurance
delta: 6 added tests, none removed or weakened. The user's follow-up request
for typed library errors is recorded in `library-typed-errors`.

Ellipsis scope correction (2026-09-08): the user identified earlier-sentence
antecedent availability as discourse interpretation, outside the English
parser. Activation recognizes the elliptical construction, preserves its local
form/voice constraints, and leaves the antecedent unresolved. No earlier-VP
collection or paragraph context fold is required for acceptance. The Lean
workbench now reflects that boundary, while keeping extraction gap closure
and an overt predicate head for imperatives. This corrects the scope of the
ellipsis family; it does not remove the family or its interaction obligations.

Validation of this correction, change `rumowyrp`: `english/scripts/axioms`
passed the complete 46-job workbench build with warnings treated as errors and
checked 2,631 compiled theorems with zero disallowed axiom uses. The source
census is 575 theorems in 43 modules. Ten witness statements were replaced for
the corrected scope, three witnesses added, none ignored; the retired context
helper and all replaced statements are accounted for in `english/DECLARATIONS.md`.
Existing exact sentence/parenthetical realization witnesses still pass, and
both existing imperative admission witnesses prove the added local condition.
`jj fix -s rumowyrp` made no formatting changes. Rust and lexical inputs are
unchanged in this checkpoint, so their earlier gate remains the applicable
result; no new corpus or whole-grammar acceptance claim is made.

Connected declaration checkpoint, change `swqkxsvu`: `english_v3::grammar`
contains 78 Construction declarations over 29 public Categories. Documents and paragraphs keep
flat sequences; finite and imperative clauses use nominal agreement and Case;
verb and auxiliary heads select complete lexical frames; prepositions, relative
Subjects/Objects and shared relative Objects compose with those phrases.
Binary NP coordination derives mixed-person agreement and checks common Case.
Auxiliary ellipsis retains its selected form/voice and realizes without a
separator or lexical leaf. The source inventory now declares ordinary versus
indefinite determiners, coordinator distributions, finite subordinators and
nominal preposition complements for these consumers. Integration tests load
that real inventory rather than maintaining a second bounded vocabulary.

The three initial Rust tests pass: 18 positive cross-family examples, 15
admission negatives, and an independently constructed `you do` Reading with
exact realization and lexical traversal identity. Those are bounded interaction
witnesses, not the corpus or full obligation-register evidence. The direct
fixture initially used `Plain` for finite `do`; inspection of the lexical
paradigm corrected it to the declared `Present` form without changing grammar.

Remaining implementation includes article/onset and capitalization constraints,
measures/numerals, lexical framing for adjective complements, the Target Verb
entry, flat serial coordination, richer extraction and frame patterns, and
keyword/editorial document forms. Adverbial, content and relative subordinator
licenses must permit a lexical owner to serve more than one role. The actual
supported-corpus command, whole-family audit and baseline remain outstanding.
None of these missing general mechanisms is a long-tail residual or a completed
family acceptance claim.

Validation for `swqkxsvu`: `cargo xtask gate --changed --clippy --run`
passed 1,611 tests with zero failures and one preexisting ignore; strict
all-target clippy passed. Its derived package closure was
`deckmaste_construction_core`, `deckmaste_construction`,
`deckmaste_construction_v3_core`, `deckmaste_english_v2`,
`deckmaste_lexical_source`, `deckmaste_semantics_v2`, `xtask`,
`deckmaste_construction_v3`, and `deckmaste_english_v3`. The default comparison
had widened because the coordinator advanced since this feature's claim; no
sibling sources were changed by this work. Subsequent development gates can
use `--from ookvvtoz` to derive the closure from this claim's changes, with the
refreshed default comparison restored for final landing validation.
`jj fix -s swqkxsvu` formatted the new test file. Rust assurance delta for this
checkpoint: 3 added, 0 restored, 0 re-spelled, 0 ignored, 0 removed (9 added
cumulatively). No full-corpus run occurred.
