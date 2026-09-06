# Whole English grammar design review

Review pass 2026-09-05, change `nqzqyuro`, on the integrated composition,
document and selection work. The [design decision](decisions/english-lean-design-workbench.md)
and [intended scope](english-grammar-design.md) remain authoritative.

**Conclusion: useful formal foundation; not yet ready to define the production
migration.** The previous tickets delivered their bounded fragments and laws.
They did not finish the grammar described by the scope map. This review found
concrete composition holes and duplicate representations, in addition to the
already disclosed assumptions. More isolated positive examples would not
resolve those architectural gaps.

The sound parts are reusable: one recursive Syntax, independent grammatical
and surface judgments, declared lexical frames, explicit agreement and voice,
and the distinction between admissibility, preference and retained readings.
The checked conditional selection/packing theorems remain valid. The issue is
which grammatical facts justify their inputs and whether the grammar closes
under its intended combinations.

## Checked interaction challenges

| Challenge | Evidence and disposition |
|---|---|
| Nested quotation inside another sentence | A fully grammatical `Written` probe previously produced `Gain "Gain 'Attack.'".`. `Surface.finishSentence` recognized only one closing quote after a period. Fixed by traversing the terminal quote suffix. `Interactions.Quotation.deep_quote_written` now proves the exact text `Gain "Gain 'Attack.'"`; all earlier quotation assertions survive. Source direction: style guide §3, “Quotation marks and apostrophes.” |
| A mobile containing another mobile's varying scope region | `Interactions.NestedScope` derives four pairwise-distinct trees for synthetic *attack and attack during creatures and artifacts with turns*. The outer PP contains the inner scope alternation. All four are admitted and survive the neutral policy; `all_four_packed` retains exactly that finite candidate class. A fixed whole-mobile-subtree-identity condition would split legitimate alternatives here. |
| Scope plus document embedding | `quoted_admitted`, `quoted_distinction` and `quoted_packing_retains_all` lift those readings into a common quoted Document. Their trees remain distinct and all are retained. This closes the earlier absence of an actual nested-mobile grammatical witness; it does not implement a general scope classifier or preference lifting law. |
| Homographs plus arbitrary preference | A compiler-checked scratch probe supplies a policy that assigns a frame-role claim to one of two admitted homographic nouns and thereby suppresses the other. The existing consistency and soundness laws still hold: they cannot establish that the supplied claim is justified. |
| Keys too coarse and too fine | Compiler-checked probes show a constant key exactly packs the two homographic lexical identities, while key=`id` prevents the legitimate modifier-scope pair from sharing a package. Exact preservation of a supplied class is neither soundness nor completeness of grammatical classification. |
| Document category plus generic coordination | A compiler-checked `Written` probe admits `Attack. and Attack.` as a complete Document through generic `Production.coordinate`. The rule currently licenses every Category, including Document and notation/header categories. Whether a constituent can coordinate needs a declared grammatical relation; the existing “licensed category” prose overstates the current rule. |
| Modal list extension | A compiler-checked exclusion shows `modeJoin` cannot accept `[modes, mode]`. Its only document schema is `[mode, mode]`, and there is no extension schema. Generic coordination would insert a coordinator rather than the required bullet/newline structure. Three-mode document shape therefore remains missing. Source direction: style guide §10, “Modal choices.” |
| Trigger plus subsequent sentence | `triggered` directly yields Ability from a subordinate clause and one clause; `bodyJoin` takes Body plus Sentence. There is no composition path for the guide's single-paragraph trigger followed by another sentence. Source direction: style guide §1, “Paragraphs and ability boundaries,” and §8, “Triggered abilities.” |
| Singleton and sequence normal forms | A compiler-checked probe derives two unequal singleton-supertype trees with the exact text `Legendary`: `supertype [legendary]` and `supertypes [noSupertypes [], legendary]`. This is representation duplication, not a scope distinction. Binary document/keyword/cost joins also admit alternate association trees. Source direction: style guide §7, “Capitalization and type-line contrast,” and §15's structural groups. |
| Mixed agreement | A compiler-checked exclusion confirms the generic coordination production cannot combine singular and plural NP categories. The existing homogeneous-category rule and invariant-modal tests do not settle mixed-person/number coordination. This is a grammatical feature-propagation gap, not vocabulary population. |
| Ellipsis and contextual admissibility | A compiler-checked probe derives two unequal trees, `ellipsis attack` and `ellipsis (ellipsis attack)`, both with empty surface. The judgment accepts supplied antecedent trees without requiring accessibility in an enclosing context. Category-only gap lists also do not state extraction domains or antecedent identity. These are explicit model limits; the earlier finite-candidate theorem does not claim the grammar's candidate set is finite. |

The negative/adverse probes above were checked with Lean LSP scratch code,
not installed as tests requiring undesirable grammar to stay accepted. They
are recorded as findings to resolve. The positive interaction witnesses and
quotation regression assertion are tracked in `lean/English/Interactions.lean`.
All wording is synthetic; no named card transcription or corpus census is
being presented as proof of these claims.

## Residual design boundaries

### 1. Grammatical evidence for preference and scope

`Policy` currently accepts a function assigning two Boolean claims and a
comparison region to an entire analysis. `FirstEligible` describes traversal
of a separately supplied abstract Host. Neither constructs grammatical
preference evidence. `Anchors` preserves abstract group topology but does not
encode coordinator, lexical/mobile identity, or enclosing Document context.

The inherited frame-complement-pair challenge also remains structurally
unmodeled: `JudgeFrame` validates an ordered sequence of arguments and fixed
markers, while ordinary coordination combines a single Category. There is no
coordinated partial-frame schema to which the positive alternate-anchor
packing obligation can attach. The abstract flat-versus-nested exclusion does
not discharge that positive obligation.

**Recommendation, high confidence:** require grammatical evidence for
comparisons and a scope relation justified over actual Syntax. Show both
preservation of non-scope distinctions and inclusion of the modeled scope
alternatives. Retain all four nested-mobile readings above; the earlier
abstract correlated-pair example is not a license to remove two of them.

**Less certain:** replace whole-analysis claim summaries with local competing
derivations and context-preserving lifting. This appears necessary for two
independent competitions in one document, but the exact relation should be
chosen against that interaction witness, not assumed from the present rank.

### 2. Canonical groups and document/phrase distinctions

The same recursive carrier remains useful, but membership in it must not
license every combination. A keyword phrase used as a verb complement is not
the same structural job as its printed keyword line. Whole documents, headers,
modes, costs, and type groups need explicit composition and cardinality rules.
Triggered and activated heads must connect to multi-sentence bodies without
introducing duplicate sentence/ability wrappers.

**Recommendation, high confidence:** use canonical ordered collections for
editorial sequences, reserve grouping nodes for actual grammatical grouping,
and license coordination by grammatical distribution. Consolidate the
singleton/empty-group overlap. Challenge three-element sequences and nested
embeddings, with forbidden cross-category twins, before calling this closed.

**Less certain:** exact placement of the common body boundary and which
collections need a nonempty versus at-least-two constraint. These should be
chosen together; adding independent special cases would reproduce the current
wrapper and arity problems.

### 3. Context and feature propagation

The source map still names unrepresented feature decisions: countability,
attributive modifiers/Targeting Marker ordering, genitives, mixed agreement,
bare temporal NPs, richer relative and complement clauses, extraction domains,
and antecedent accessibility. These are ordinary combinatorial grammar, not
the post-migration long tail. The eleven passive-temporal obligations and the
named gap/relative obligations remain owned by the existing wayfinder.

**Recommendation, high confidence:** make grammatical contextual admissibility
explicit before selection. English should check whether an antecedent or gap
is structurally available; Semantics should determine what it denotes. Keep
lexical distributions and feature propagation declarative.

**Less certain:** the best context representation: indexed occurrences,
separate binding witnesses, or an environment carried by judgments. Do not
turn every combination into another Category. The decisive challenges are
nested dependencies, an inaccessible antecedent, and coordination with
independent bindings.

### 4. Surface contracts and layout validity

`Atom.word`, symbol and notation payloads are arbitrary strings supplied by
lexical relations. The model assumes they respect boundary ownership. Exact
`Spells` theorems therefore apply under those supplied surfaces; they do not
validate lexical transcription or source normalization. `Faces.layout` is an
unconstrained tag alongside documents. Range and chapter headers are lexical
notation. A level band requires a nonempty Document where the style guide
allows zero ability lines after its statistics (§15, “Leveler cards”).

**Recommendation:** assign typography, textual group cardinalities and source
boundary checks to English/document validation; keep game legality and
interpretation in Semantics. Give normalization and lexical payload validity
an explicit contract rather than inferring correctness from exact spelling
of fixture data. Frame metadata may remain external if its required
relationship to text boxes has a named validator/owner.

**Less certain:** precisely which layout checks belong inside English's
admissibility judgment versus a neighboring document-validation relation.
This should not block the basic phrase carrier, but it must be settled before
claims about complete frame/document validation or migration correspondence.

## Scope-map disposition

All sixteen guide sections remain assigned in the design document. Reviewing
those assignments yields these closure units, rather than a card-count gate:

- §§1–3 and §§8,14–15: consolidate document composition, phrase/document
  distinctions, canonical collections, and explicit surface/layout contracts.
- §§4–7 and §§9–13: finish lexical distributions, feature propagation,
  contextual dependencies and the ordinary clause/phrase combinations above.
  Populating attested lexical/template data is distinct from missing schemas.
- §10 across those groups: define actual partial-frame coordination, local
  preference evidence, and scope equivalence with preserved context.
- §16: validate the resulting finite set of interactions and exclusions;
  report remaining lexical/editorial population work separately.

No corpus-completeness proof, global uniqueness result, executable parser or
Lean-to-Rust generator has been added as a requirement. Existing Rust
machinery is still a reuse candidate. Its migration boundaries wait on this
design closure as the accepted decision requires.

## Validation and status

The full `lean/scripts/build` passes all 68 jobs with warnings treated as
errors. All prior 117 theorem declarations remain unchanged. Added: 12 named
interaction assertions, removed/re-spelled/ignored/restored: zero. Lean LSP
checked all 12 declarations for axioms and source warnings: only subsets of
`propext`, `Quot.sound`, `Classical.choice`; no placeholders or custom axioms.

This review pass and the bounded quotation fix are ready for inspection.
The review ticket remains WIP: the four residual boundaries above need the
user's design decisions and subsequent closure work. The recommendations are
proposals, not newly accepted rulings; migration is not unblocked by this
report or by the green build.
