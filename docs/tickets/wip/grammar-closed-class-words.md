---
needs: [grammar-generic-coordination]
---
# Closed-word references, lexical classes, checked data membership

Lexical distinctions reach syntax as untyped string flags today. 72 of 184 features are in no category interface, 37 of those are read at exactly one site, and 39 are written for exactly one holder in the lexicon, for example `InfinitivalMarker { Yes }` (held by *to*), `RelativeSubordinator { Yes }` (*that*), `FocusingModifierUse { Restrictive }` (*only*), `SelectedTailLicence` (*put*). `core.ron` carries about 192 per-lexeme patches of grammar-internal flag names, and an authored value outside a feature's domain silently becomes absent (`crates/deckmaste_construction_v3_core/src/emit.rs:317–331`).

## Change

- Closed-word declarations in the grammar bind names to existing lexeme IDs and Categories; they do not define or synthesize lexemes. Every word has one home in lexicon data, including forms, properties and class membership. The review withdrew the earlier proposal to move `vocabulary.ron` into Rust and delete it. `vocabulary.ron`, `verbs.ron` and `core.ron` stay data. The loader checks reference eligibility from existing source/vocabulary evidence, Category agreement, unique names/IDs and declared membership classes.
- Inventory first. List every `vocabulary.ron` declaration by destination and by direct-reference eligibility. The file also holds `PredicativeAdjective` (line 24), `ColorWord` (149), `Supertype` (216) and the `CommonNoun` lexeme (224), which are open-class and are not eligible for direct reference; each gets a stated destination (typed class reachable through frames, or data that stays). Closed-class material that lives elsewhere (patches in `core.ron`) is listed too. Open-class members stay out of the directly referenceable Rust type.
- Loader boundary. The grammar crate depends on `deckmaste_lexical_source` only as a dev-dependency, and `deckmaste_lexical_source` does not depend on the grammar crate, so the Rust declarations cannot simply be read by the loader. The landing states who supplies them (for example the caller, `xtask` and the tests, passes them into the load; or a crate both depend on) with no dependency cycle, and keeps today's order: native declarations, then core, then plugins, then normalisation (`crates/deckmaste_lexical_source/src/lib.rs:39–50`). Callers, the lexical-source tests (`crates/deckmaste_lexical_source/tests/suite/errors.rs:53`) and the incremental-source accounting (`crates/xtask/src/english_v3/incremental.rs:15`) are updated.
- Eligible require-only membership patches become word references or class memberships. Table-fed properties, meaningful defaults, open-class singletons and permissions whose class spelling would not clarify the construction remain properties, as recorded in the final table.
- The grammar refers directly only to core closed-class lexemes, by lexeme id, never by string literal; a direct reference is used when exactly one lexeme can fill the slot, a typed class when two or more can. The compiler checks every reference. Everything else (plugin terms, catalog entries, card names, open-class core words) reaches the grammar only through typed classes and frames.
- The loader and compiler reject unknown ids, class names and values; anything newly rejected is fixed as its own change, not waved through. `unsupported_frames()` (`crates/deckmaste_construction_v3_core/src/selected_frame.rs.txt:275`, called only from tests) is called by the tool once and its result reported.
- Report, as findings, every class with one member and every feature with one holder, and what was done with each.

## Gate

Reading digests identical on every face, checked with the census check mode as an identical-digest landing. Corpus digests cannot protect lexemes no face uses, so a structured inventory comparison is also required: for every entry, lexeme id (`vocab:{inventory}/{member}`, `crates/deckmaste_lexical_source/src/legacy/core.rs:177`), forms, variants, lexical behaviour and frame order are identical before and after; the only differences allowed are identified provenance and membership-representation changes. The comparison is reported alongside the one change of the lexicon inventory hash (`crates/xtask/src/english_v3/mod.rs:131`). The landing updates `crates/deckmaste_english_v3/README.md`. Note: census delta (none); the judgment-set line does not apply yet.

## Phase 1: measured inventory and authoring-model STOP

The baseline on the claimed workspace has 32,828 faces, 30,794 read,
482,041 Readings, no incomplete faces and no issues. No grammar, lexicon,
compiler or loader changes have been made.

The expanded grammar declares 184 features; 73 occur in no Category interface
(the authored-only count is 74 because helpers carry CorrelativeCoordinator).
The loaded lexicon contains 35,603 entries. Of those 73 features, 32 have one
explicitly stored holder; across all declared features there are 51 such
findings. Explicit No values and category defaults count as stored holders,
not affirmative membership. BareCoordination, for example, has only Then=No
written and defaults to Yes.

Proposed dispositions for the 73 rows: 24 direct-reference candidates (including
CorrelativeKind's three single-lexeme values), 10 closed-word classes, 15
lexicon-supplied classes, 19 retained lexical properties, and five notation or
nonlexical cases. These are candidates, not blanket conversions: a reader's
lexical Category, value and use in tables or selected literals still control
its conversion. No declared grammar feature currently has an out-of-domain
stored value. The string map also contains 33 other key names plus the
FeatureSource provenance family; these need explicit adapter/metadata treatment,
not a blanket exemption from unknown-name checks.

Vocabulary has 39 declarations and 203 authored members: 15 closed-word groups
(49 members), six open-class groups (91 members), three notation groups
(34 members), twelve currently unmapped groups (29 members), and three ignored
morphology recipes. Unmapped declarations must not gain entries on migration;
a few identities also have separate native declarations. There are eleven
one-member vocabulary groups and three one-member compound-noun recipe groups;
Affix also has one loaded member. Core has 241 native lexemes and 192 feature
patch targets; 62 native entries and 63 patch targets have closed-word lexical
Categories, before separately considering functional adverbs and auxiliaries.

The inventory tool also called unsupported_frames(): seven distinct unsupported
frames remain, owned by Deal (one), Enter (three), Put (two), and plugin exile
(one). They must not be activated by this refactor.

Detailed measured tables, every holder ID/value/provenance, reader equations,
original policy/schema sites, table and selected-literal readers, runtime
readers, vocabulary destinations and singleton findings are in the workspace's
ignored target/english-v3/. The structured before inventory preserves all
entries, forms, variants, lexical properties and ordered frames. No generated
evidence is tracked.

### Decision required before the prototype

Two materially different authoring models cover the simple slice. The shared
registry design is grammar-owned Rust declarations emitted by the grammar macro:
closed-word symbols denote fixed existing IDs, and lexical-class symbols denote
checked membership sets. Open-class and plugin members never enter the
closed-word symbol type.

**A: lexical selectors in forms.**

```rust
schema ToInfinitive {
    form [negator: optional(SecondaryNegator), marker: lexical_word(InfinitivalTo),
        " ", predicate: node];
}
// In GerundComplementPreposition:
form [head: lexical_class(GerundClausePreposition), " ", complement: SecondaryVerbPhrase];
```

A mixed-Category class also needs an intersection spelling, such as
lexical_class(SuperlativeQuantifier, Determinative) and
lexical_class(SuperlativeQuantifier, Adjective), to retain the two existing
SuperlativePremodifiedNominal forms and their indices. Shared policies that
restrict a lexical field would need a corresponding form-binding mechanism,
or would keep some flags.

**B: word and class requirements (recommended).**

```rust
schema ToInfinitive {
    form [negator: optional(SecondaryNegator), marker: lexical(Subordinator),
        " ", predicate: node];
    require marker is InfinitivalTo;
}
// In GerundComplementPreposition:
form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
require head in GerundClausePreposition;
```

B keeps lexical Categories and form indices explicit and unchanged, and works
inside existing policies. Neither model changes Word or Reading fields. These
are proposals, not implemented syntax.

The proposed registry spelling is the same in both models. For example, the
following excerpts replace InfinitivalMarker=Yes on infinitival To and
GerundClauseComplement=Yes on By; unshown forms, properties and frames are copied
unchanged, not defaulted away:

```rust
closed_word InfinitivalTo: Subordinator {
    id "vocab:InfinitivalMarker/To";
    lemma "to";
}
lexical_class GerundClausePreposition: Preposition;
closed_word By: Preposition {
    id "vocab:Preposition/By";
    lemma "by";
    classes [GerundClausePreposition];
}
```

These declarations live inside the Rust grammar macro and generate the closed
word/class symbol types and loader registry. The other three members are Of,
prepositional To and Without. Data-backed classes declare their allowed lexical
Categories in the same registry; data supplies membership IDs, checked at load.

For either model, callers pass the grammar's closed-word declarations and schema
registry into lexical_source loading; no dependency from lexical_source back to
the grammar is introduced. Supply words in the core-declaration stage before
plugins and normalization. The common lexical layer owns registry descriptors;
the grammar owns declarations. Plugin/catalog adapters join classes through
checked registry IDs and data membership, never grammar references to their
individual lexemes. Unknown word IDs, class names and property values reject.
Migration removes membership flags from the string map; genuine properties
remain schema-validated, while provenance/legacy adapter fields receive separate
explicit metadata handling. Existing lexical behavior and frame order remain
part of the structured comparison, not inferred from corpus coverage.

After the decision, prototype seven existing word identities: infinitival To,
relative That, Only, and the four GerundClausePreposition members By, Of,
prepositional To, Without. The two To identities remain distinct. Leave Than's
ExpandedComparisonMarker judgment and separator-driving properties explicit in
this slice. Then run compiler/loader regressions, full touched-crate tests and
clippy, structured inventory and contract/shape comparisons, and the full digest
gate before committing the prototype and pausing for Phase 2 review.

### Phase 1 decision and prototype

The owner selected the split model: `lexical_word(Word)` in a form for a single
closed word, `require field in Class` for a class, and `require field is Word`
when a policy owns the constraint. This supersedes the alternatives above.
No class selector or Category-intersection syntax was added to forms.

The prototype moves seven existing identities into Rust declarations: infinitival
To, relative That, Only, and prepositional By, Of, To and Without. The latter
four declare GerundClausePreposition membership. The four membership features
InfinitivalMarker, RelativeSubordinator, FocusingModifierUse and
GerundClauseComplement, together with their stored values/provenance, are removed.
Other properties and core patches are retained. The empty FocusAdverb vocabulary
group remains for the later whole-file migration.

Callers pass `grammar::lexical_registry()` into `load_workspace`. The common
lexical crate owns descriptors, with no production dependency from the loader to
the grammar. Lexical-source tests supply the grammar registry through a test-only
dependency. Words enter at the native declaration stage, before legacy core,
plugins and normalization. Source data can supply class memberships directly or
through `class_additions`, keyed by existing lexeme IDs, including plugin/catalog
IDs. The loader checks those IDs, classes, Categories and real feature domains.
The 33 measured adapter metadata keys remain explicitly recognized; their
provenance does not grant grammar membership. Incremental capture already covers
Rust grammar/compiler sources, so registry changes invalidate data-only reuse.

The compiler lowers word/class constraints to finite lexical admission checks;
no word identity is forwarded in constituent summaries. Single-word form
selectors retain the existing lexical Category, fields and form index. Shared
policies and mixed-Category classes use requirements. Direct optional-word
requirements still apply only when present. The invariant-word prototype cannot
yet declare inflected closed words or form overrides; those remain data-backed.
No additional families have been converted.

The structured comparison covers all 35,603 lexemes. IDs, forms, variants,
non-migrated properties and frame order agree exactly; only the seven identified
membership/provenance migrations differ. Each of the four new checks was also
compared extensionally against its old flag on every lexeme, including absent
flags and lexemes no corpus face uses. Unsupported frame signatures remain the
same seven reported above. Evidence lives only under target/english-v3/.

The first full census had identical Reading counts and line-level Reading
digests but changed all covered face digests. The face-context encoding includes
raw root Summary Debug, and the initial compiler unnecessarily allocated identity
slots for By, Of, PrepositionalTo and Without even though no construction named
them individually. Removing those unused checks restores the 200-slot layout;
registry membership remains loader data. A compiler regression checks that
unused registry members allocate no admission slots. Nissa, Worldsoul Speaker,
the first reported failure, then matched its baseline face digest exactly.
Neither the gate nor the context encoding was changed.

Final full-corpus gate: identical digests on all 32,828 faces, with 30,794 read
and 482,041 checked Readings; no incomplete faces or issues. The completed step
census was deleted after the gate, retaining the original baseline. The sorted
comparison also matches 927 rule contracts (including table/frame contracts,
after explicit membership aliases and internal register renumbering), 326
constructor records and 666 emitted struct/enum variant shapes. Fields, their
order, form indices and optional category fields are unchanged.

The lexical inventory hash changes once, from
`8224cc4de830fcb532d621327146d32626320b41a0156463390281ab254bee64` to
`bf8e0b103cb8519be4fbdb5544dc5d0b37a0e52715a6d3b12798490c8ad29f71`,
accounted for by the structured comparison's explicit seven-entry allowlist.
Authored declarations.rs changes from 6,093 to 6,122 lines; declared features
184 to 180, lexical-only features 73 to 69, with seven closed_word declarations
and one four-member lexical_class. Of the singleton findings, infinitival To,
relative That and Only now use direct references; the other findings remain for
Phase 2. No singleton lexical class was introduced.

Final verification: full cargo test for deckmaste_lexical,
deckmaste_lexical_source, deckmaste_construction_v3_core,
deckmaste_construction_v3, deckmaste_english_v3 and xtask: 1,899 passed,
zero failed, two pre-existing ignored. Full clippy --tests -- -D warnings passes
for the same six crates. Citation audit has zero changed citation sites,
format checking has zero noncompliant sites, and 17,780 citations have zero stale.

Assurance: 17 added tests (eight compiler, four generated-runtime, five loader),
one re-spelled (`gerund_complement_requires_declared_licence`, removing class
membership instead of the retired flag, with the same rejection assertions),
zero restored, zero newly ignored, zero removed. The existing ignored tests are
`cost_notation_is_the_subject_of_the_licensed_locative_small_clause` (the separate
MS4 governed Mana Phrase Subject route) and `macro_schema_census_count_matches_21`
(the on-demand live-corpus schema census). Neither was changed.

Phase 1 stops here for review. The remaining closed vocabulary, data-backed
classes, default-sensitive flags and real lexical properties have not been
converted; the measured inventory remains the Phase 2 worklist. Inflected
closed-word declarations need explicit form/override support before those
families can move. The ticket remains WIP and has no final landing note yet.

## Review follow-up: digest encoding

The review's declaration redesign is accepted: data owns each complete lexeme;
`closed_word Name: Category = "existing-id";` only binds a checked reference.
Membership stays on data members. The prototype's properties/lemma syntax and
lexeme synthesis will be removed. The remaining work also includes the real
word-selector AST, producer-based provenance validation, the four unread map
keys, a tested plugin membership route where the measured classes need it,
and the restricted Phase 2 conversions. Table/export-fed properties,
default-sensitive properties and open-class singletons stay properties. No
further authoring decision is pending.

The first follow-up changes only the census's surrounding encoding. Decomposition
encoding version 2 omits derived root-summary storage; unit Reading hashing
remains version 2 and the root contract remains context version 1. The fixed
surrounding constructors, lexical values, literal separators, unit boundaries
and root-contract hash remain encoded. Every reconstructed surrounding Reading
still passes admission and the root predicate. Summary slot layout is not a
Reading distinction.

An isolated audit executable under ignored target/english-v3 computes both
encodings from each parse. Across all 32,828 faces, its legacy hashes match the
saved census: 30,794 surrounding hashes, 75,412 unit digests and counts, 482,041
Readings, and every face's status/count are unchanged. Independently rebuilding
both face hashes accounts for every change solely through summary omission and
the decomposition version tag. There are still 2,034 unread faces and zero
incomplete faces or issues. Compact legacy contracts and the old census SHA-256
are retained in ignored evidence; the superseded 1.1 GB census is removed.

The sorted comparison still matches 927 rule contracts, 326 constructor records
and 666 emitted shapes. The structured comparison still matches all 35,603
lexemes under the prototype's seven explicitly recorded membership/provenance
migrations; this follow-up changes no lexeme, feature or grammar declaration.
The unsupported frame finding remains Deal ×1, Enter ×3, Put ×2 and plugin
exile ×1. No frame is fixed or activated here.

The slot-layout regression was reproduced in an isolated compiler build by
restoring the unused generated `__word_By` feature (201 slots instead of 200).
Its retirement changes every legacy surrounding hash on the 30,794 read faces,
but no version-2 payload or digest. The full identical-digests gate passes for
all 32,828 faces. This is a scratch-only retirement probe, not an additional
production declaration or a Phase 2 conversion.

Full tests for xtask and deckmaste_english_v3 pass: 1,605 passed, zero failed,
two existing ignored. Clippy --tests -- -D warnings passes for both crates.
Assurance for this step: one added test
(`surrounding_digest_records_structure_and_root_contract`), zero restored,
re-spelled, newly ignored or removed. Neither existing ignored test changed.
Citation audit reports zero changed sites, zero noncompliant strings and
17,780 checked citations with zero stale references.

This is a completed digest-step prefix. The declaration redesign and all
Phase 2 conversions remain pending; no additional property or data key has
been removed. declarations.rs remains 6,122 lines with 180 authored features.
The ticket remains WIP and has no final landing note yet.

The production executable also passes the full version-2 identical-digests
gate on all 32,828 faces and the legacy-unit/structure audit. Its report is the
new `target/english-v3/closed-class-baseline.json`; no step census remains.
The production lexical inventory hash remains
`bf8e0b103cb8519be4fbdb5544dc5d0b37a0e52715a6d3b12798490c8ad29f71`.
Standalone audit builds give the prototype's synthesized words different
`file!()` provenance paths; their reports are not retained as the baseline.
The production inventory comparison, not that incidental build provenance,
establishes lexicon equivalence.

## Review follow-up: reference-only declarations

The grammar now binds only three names to existing IDs:

```rust
closed_word InfinitivalTo: Subordinator = "vocab:InfinitivalMarker/To";
closed_word RelativeThat: Subordinator = "vocab:Subordinator/That";
closed_word Only: Adverb = "vocab:FocusAdverb/Only";
lexical_class GerundClausePreposition: Preposition;
```

All seven prototype lexemes return to their original data homes and provenance:
That, infinitival To, By and Without in core.ron; Only, Of and prepositional To
in vocabulary.ron. The four prepositions state membership on their data entries;
they need no individual grammar binding. Their existing properties and frame
patches remain data. The four unread map keys deliberately remain for the next
separate step. No vocabulary file is retired.

`lexical_word` has a real FieldType variant, and `is`/`in` have a separate
requirement variant. Lowering retains the same lexical fields and form indices;
word/class admission checks no longer masquerade as grammatical features.
The four hidden feature domains disappear (200 compiled slots become 196).
Optional lexical requirements check only present words. The compiler rejects
attempts to access the retired hidden feature names.

Callers still pass the generated registry to the loader. It contains names,
IDs, Categories and domains only, never lexemes. The loader validates reference
existence, Category, unique names/IDs and core closed-class eligibility. Existing
Core provenance and a closed lexical Category establish eligibility; Adverbs
must come from PredicateNegator, FrequencyAdverb or FocusAdverb inventories,
and Verbs must have an Auxiliary frame. Other open-class, plugin, catalog and
card-name lexemes cannot become named word symbols. No eligibility flag was
added. Existing data forms, including inflection and variants, require no new
grammar declaration syntax.

The compiler, grammar and loader READMEs describe the revised boundary; the
Oracle English glossary defines Closed Word and Lexical Class. Plugin/catalog
membership on the supplying declaration remains a later, need-dependent step:
the existing class_additions adapter is not claimed as that implementation.

The full identical-digests gate against the version-2 baseline passes on all
32,828 faces: 30,794 read, 482,041 Readings, zero lost/gained/count/digest-only
changes, incomplete faces or issues. Nissa's 20 Readings and unit/surrounding
contracts also match in the subset check. Retiring four production feature slots
now leaves every digest unchanged, confirming the earlier encoding repair.
The completed step census was removed; the baseline remains under target.

The sorted comparison matches all 927 rule contracts, 326 constructor records
and 666 emitted shapes, using the same compiler for both declaration sets and
explicit membership aliases. The private Rule/Summary representation now holds
separate lexical checks; public Reading shapes, fields and forms are unchanged.
The structured lexicon comparison matches all 35,603 entries against the original
inventory, allowing only the four already-approved membership replacements on
seven entries. Original provenance now matches exactly too. Each replacement
predicate agrees with its former flag on every lexeme. The inventory hash is
now 504777d9b027452b72c652ed61b9c01a51c258dc314ad377063dbc145ab1390e.

The full suite caught three selected-frame regressions in the new representation:
dynamic frame expansion rebuilt its existing per-symbol arrays but initially
left lexical_checks at the template length. Longer selected frames consequently
failed admission. Expansion now clears/truncates/appends lexical_checks with
those arrays, retaining constraints on the selecting head. The original three
selected-frame tests pass unchanged; no frame signatures or expectations were
altered. This is separate from the seven unsupported signatures, which remain
unchanged (Deal: MassNoun Complement; Enter: FrameComplement Complement and two
Object Complement signatures; Put: two Object Complement signatures; plugin
exile: ResultativeComplement Complement).

Full cargo test passes for deckmaste_construction_v3_core,
deckmaste_construction_v3, deckmaste_lexical, deckmaste_lexical_source,
deckmaste_english_v3 and xtask: 1,907 passed, zero failed, two existing ignored.
The compiler tests were also rerun after extracting register-release planning;
all 92 pass. Full clippy --tests -- -D warnings passes for those six crates.
Citation audit selects zero changed sites; formatting reports zero noncompliant
sites, and all 17,780 citations are current.

Assurance: seven added tests (two compiler, five loader), twelve re-spelled
(eight compiler fixtures and four runtime fixtures now use reference-only
bindings/data), zero removed and zero newly ignored. Three selected-frame
regressions were fixed with their original tests unchanged; no deleted tests
were restored. The two existing ignored subjects and their blockers are unchanged.
Authored declarations.rs is 6,089 lines (6,093 at ticket start, 6,122 after the
prototype), with 180 authored features (184 at ticket start), three closed-word
bindings and one four-member class. No additional lexical property was retired
in this step; only the four hidden compiler feature domains were removed.

This is a complete, gate-passing declaration-redesign prefix. Remaining, in the
owner's order: remove the four unread property keys and reduce metadata handling;
implement a real plugin/catalog membership route only for a measured Phase 2
need; convert the eligible families while retaining table/export-fed properties,
default-driven flags and open-class singletons; finish the remaining dispositions
and ticket note. The glossary and READMEs are current for the completed step.
The ticket remains WIP; the final two-line landing note belongs to completion.

## Review follow-up: unread keys and provenance metadata

The three preposition keys have no reader in the compiler, grammar, loaders or
other crates: their only production occurrences were the vocabulary data and
registry exemptions. Each occurred on 15 vocabulary prepositions. The string
map key Finiteness occurred only on infinitival To; form finiteness is a separate
typed field, not a reader of that key. Removed all 46 values and their four
metadata exemptions. Negative loader tests now reject these keys.

The fixed metadata allowlist shrank from 33 names to 25: four dead keys were
removed and four observed OverrideSource cell names were replaced by the
producer-owned namespace. Both OverrideSource and FeatureSource prefixes are
shared constants used by their writers and consumers. OverrideSource requires
a nonempty opaque cell suffix; FeatureSource must identify a declared property
or recognized metadata key. No provenance values or typed fields changed. The
loader README lists all 25 remaining adapter annotations and typed-field mirrors;
this step does not redesign their representation.

### Final candidate dispositions

All 49 original candidates are accounted for below: 21 converted and
28 retained properties; none pending. Final validation is recorded at the end.
The table-output-domain check adds VerblessPredicateUse to the retained list;
the initial inventory's table column had counted only inputs. No plugin/catalog
route is implemented by this cleanup step. BareSingularUse and IdentityUse are
the measured candidates that could justify it; table-fed plugin properties do
not justify a membership route on their own.

**Direct-reference candidates (24)**

| Original feature | Disposition | Reason / next work |
|---|---|---|
| QuantitativeExceptionUse | Converted | Word reference: All. |
| QuantificationalDeterminer | Retain property | Inputs to quantificational_number and quantificational_scalar_denotation. |
| QuantificationalAdjunctUse | Converted | Word reference: Each. |
| PrimaryNegation | Converted | Policy requirement: head is Not. |
| ObliqueMarker | Retain property | Inputs to oblique_number and whole_object_oblique_source exports. |
| AdverbComplementClass | Converted | Word reference: For. |
| FocusingModifierUse | Converted | Only word reference. |
| ImmediatePostheadDependentUse | Retain property | Input to licensed_posthead_dependent; optional admission judgment. |
| FusedHeadPostmodifierUse | Converted | Present-only optional requirement: dependent is Else. |
| QuantitativeConjunctUse | Converted | Word reference: Both. |
| MatrixComplementGovernor | Converted | Word reference: Except. |
| CompoundComplementMarker | Retain property | Input to selected_compound_complement. |
| FiniteObjectGapComplement | Converted | Word reference: As. |
| IrrealisClauseComplement | Converted | Word reference: AsThough. |
| ExtentMarker | Converted | Word reference: By. |
| RelativeSubordinator | Converted | RelativeThat word reference. |
| ProFormComplementUse | Retain property | So is a core Adverb in the generic Adverb inventory, which does not establish closed-word eligibility; no singleton class. |
| MatrixComparisonUse | Retain property | Both Yes and default No are read; ordinary Adverb admission uses No. |
| ExistentialUse | Retain property | Dummy pronouns require Yes; ordinary nominative/accusative pronouns require default No. |
| CardinalModifierUse | Retain property | Exactly is a core Adverb in the generic Adverb inventory, which does not establish closed-word eligibility; no singleton class. |
| InfinitivalMarker | Converted | InfinitivalTo word reference. |
| SecondaryNegation | Converted | Policy requirement: head is Not. |
| CorrelativeKind | Converted | Word references: Both, Either, Neither. |
| QuotedComplement | Converted | Word reference: With. |

**Closed-word class candidates (10)**

| Original feature | Disposition | Reason / next work |
|---|---|---|
| ComparativeDegreeModifierUse | Retain property | Any, Many, Much, No and Some have this degree-modifier permission; ComparativeDegreeDeterminative would restate that role without clarifying which words belong. |
| AuxiliaryUse | Retain property | Only Be, BeContracted and HaveNegative carry this permission; Auxiliary would overstate the class and PostmodifiedAuxiliary would merely rename the construction permission. |
| FusedHeadUse | Retain property | Anywhere and This license this particular fused-head path; FusedHeadDeterminative would imply a broader general class or merely rename the permission. |
| ComparativeQuantityUse | Converted | ComparativeQuantifier names fewer, less and more at both reader sites. |
| GerundClauseComplement | Converted | GerundClausePreposition class (By, Of, To, Without). |
| RelativeUse | Retain property | Both Yes and default No are read; ordinary pronoun admission depends on No. |
| BareCoordination | Retain property | Default Yes licenses words without a stored value; Then explicitly has No. |
| NoncorrelativeCoordination | Converted | NoncorrelativeCoordinator names the five ordinary joining words and excludes Nor; other joining requirements remain. |
| NominalLicense | Converted | DependentGenitivePronoun names the five possessors more precisely than AnyNominal; Genitive case is still required. |
| KeywordComplement | Retain property | With and Without license the keyword complement; KeywordPreposition would restate the same complement permission, not clarify the policy. |

**Lexicon-supplied class candidates (15)**

| Original feature | Disposition | Reason / next work |
|---|---|---|
| SuperlativeQuantifierUse | Converted | SuperlativeQuantifier names least, fewest and most across the two lexical Categories; form indices stay unchanged. |
| QuantitativeAdjectiveComplementUse | Retain property | Least and Fewest license this complement; QuantitativeComplementAdjective would only rename the role. Reusing SuperlativeQuantifier would also include Most and require Category-dependent comparison rather than a direct membership replacement. |
| ScalarComparativeUse | Retain property | Greater is an open-class adjective and the only holder; no singleton class. |
| FrequencyUnit | Retain property | Time is an open-class noun and the only holder; no singleton class. |
| VerblessPredicateUse | Retain property | Output domain of verbless_ordinary_frame_use, not merely a membership flag. |
| VerblessComplementOmissionUse | Retain property | Input to verbless_ordinary_frame_use; also singleton Able. |
| PastParticipialPremodifier | Retain property | Input to verbal_premodifier. |
| BareSingularUse | Retain property | Retained grammatical permission: modifier.BareSingularUse = Yes states what licenses the bare singular NP. in BareSingularModifier would mostly rename it, not clarify the construction; no plugin route added. |
| BoundKeyword | Retain property | Both Yes and No are read (bound suffix versus noun premodifier); positive membership does not replace both. |
| CreatureSubtypeStem | Retain property | Input to licensed_creature_stem. |
| CreatureSubtypeStemUse | Retain property | Input to licensed_creature_stem. |
| IdentityUse | Retain property | Retained name use: head.IdentityUse = Name already distinguishes name-bearing Catalog entries. in ProperName would mostly rename that condition; no catalog route added. |
| TaxonomicComplementUse | Retain property | Kind is an open-class noun and the only holder; no singleton class. |
| SelectedTailLicence | Retain property | Default ParallelFrames and explicit DeclaredFrames are read by selected-frame expansion; a real lexical property. |
| KeywordIndefiniteArticle | Retain property | Input to keyword_subject_article and keyword_subject_license; also singleton champion. |

The cleanup passes the full identical-digests gate against the version-2
baseline: 32,828 unchanged faces, zero lost/gained/count/digest-only changes,
30,794 read faces and 482,041 Readings. Nissa's 20 Readings, unit contracts and
surrounding structure also match in the subset. Sorted comparison again matches
927 rules, 326 constructor records and 666 emitted shapes. The structured
comparison checks all 35,603 lexemes, permitting only the four earlier
membership replacements on seven entries and the 46 explicitly retired map
values. Forms, variants, other properties, provenance and frame order match;
the four replacement predicates agree extensionally on every entry. All seven
unsupported frame signatures and all unmapped declarations remain unchanged.
The lexical inventory hash is now
c86d867f1b731356925fba56f899530a9968ffb052db21d0399c71e1b977429b.
The step census was deleted after the gate; only the baseline census remains.

No grammar declaration changed in this step: declarations.rs remains 6,089
lines, 180 authored features, three closed-word references and one lexical class.
The already-retired features are InfinitivalMarker, RelativeSubordinator,
FocusingModifierUse and GerundClauseComplement. This cleanup retires data keys,
not further grammar features.

Full cargo test for deckmaste_lexical_source, deckmaste_english_v3 and xtask
passes: 1,705 passed, zero failed, two existing ignored. Full clippy for those
three crates with --tests -- -D warnings passes. Assurance for this step:
two added loader tests, zero restored, re-spelled, removed or newly ignored.
The existing ignored mana-symbol Subject test and live-corpus macro census test
retain their stated blockers unchanged. Citation audit selects zero changed
sites; zero noncompliant sites and zero stale citations (17,780 checked).

This is a complete cleanup step. The ticket remains WIP: the need-dependent
plugin/catalog data route, 26 pending candidate reviews/conversions and final
landing note remain. The glossary already defines Closed Word and Lexical Class;
READMEs and the seven unsupported frame findings are current for this prefix.

## Phase 2: direct word references

Completed all 15 pending direct-reference decisions. Thirteen features are
replaced by 13 additional named words (16 bindings total); So and Exactly remain
properties under the existing closed-class eligibility check. They are native
core entries with Category Adverb and Source owners vocab:Adverb/So and
vocab:Adverb/Exactly, not members of the functional adverb inventories accepted
by that check. Neither an eligibility relaxation nor a singleton class is added.

The removed features are QuantitativeExceptionUse, QuantificationalAdjunctUse,
PrimaryNegation, SecondaryNegation, AdverbComplementClass,
FusedHeadPostmodifierUse, QuantitativeConjunctUse, MatrixComplementGovernor,
FiniteObjectGapComplement, IrrealisClauseComplement, ExtentMarker,
CorrelativeKind and QuotedComplement. Their values and FeatureSource records
leave the data; all other properties remain. Forms name their words directly;
the optional Else field and shared negator policies use existing is requirements.
The finite-gapped form alone names As; the ordinary finite-clause form keeps its
table judgment. Both correlative Both and quantitative Both bind the same ID.

Four tests are re-spelled: licences_and_positive_auxiliary_status_control_admission,
named_irrealis_word_controls_admission_and_parsing (formerly declared_irrealis_permission_controls_admission_and_parsing),
negation_requires_the_named_word (formerly negation_requires_the_declared_feature),
and correlative_markers_keep_determinative_identity_and_nor_is_distinct.
Identity perturbations retain spelling and other lexical data. The independently
built irrealis Reading points to the perturbed, realizable lexeme before its
admission is rejected; rejection is not merely a missing-word lookup. Original
texts, controls and asserted outcomes are retained. No tests are removed or
newly ignored, and no compiler capability is added.

The two potential plugin/catalog classes are deliberately retained as properties
under the owner's clarity rule. BareStatusNounPhrase currently says
`require modifier.BareSingularUse = Yes`; `require modifier in BareSingularModifier`
would rename the same grammatical permission without making its role plainer.
CatalogName says `require head.IdentityUse = Name`; `require head in ProperName`
would likewise rename an already explicit name-use condition. Neither warrants
new source-declaration plumbing here. The eligibility check is unchanged.

The first full census passed, but the independent optional-Else test found a
real admission regression: generated optional-word admission enforced feature
checks but omitted the separate lexical checks already enforced by parsing.
The emitter now checks those same word/class memberships when a word is present;
absence remains admissible. The original
anywhere_else_goal::optional_dependent_and_locative_use_require_declared_licenses
test is unchanged. The compiler runtime fixture additionally rejects independently
constructed optional homographs, and a new test checks optional class membership
with either separator position, including absence, parse and realization.
This fixes an existing capability; it introduces no authoring construct.

After the admission fix, the full identical-digests gate against the version-2
baseline passes again: 32,828 unchanged faces, 30,794 read, 482,041 Readings,
zero lost/gained/count/digest-only changes, incomplete faces or issues. The
subset retains Nissa's 20 Readings. The completed step census was removed.

Regenerated comparisons match 927 sorted rule contracts, 326 constructor records
and 666 emitted shapes. Retired feature constraints are compared as their
explicit word/class predicates. Their constant registers are checked to have
one lexical reader, no export/table consumers and no release before that reader;
only the subsequent lifetime of these retired registers is representation-specific.
All surviving register releases remain compared. The structured comparison
checks all 35,603 lexemes and all 19 replacement predicates extensionally,
including absent values. It permits only the 17 cumulative feature/provenance
retirements and the 46 previously approved dead values; 30 entries differ.
Forms, variants, other properties and ordered frames match. The seven unsupported
signatures and unmapped declarations are unchanged. The new inventory hash is
c84e961550a48d6697542b391b3a967409f50ce7c9769e35910da09ad3e85df5.

Authored declarations.rs is 6,071 lines (6,089 before this group; 6,093 at ticket
start), with 167 authored features (180 before this group; 184 at ticket start),
16 closed-word bindings and one four-member class. No production construction,
Category, constructor name, field, form index or optional Category field changed.

Full cargo test for deckmaste_construction_v3_core, deckmaste_construction_v3,
deckmaste_english_v3, deckmaste_lexical_source and xtask passes: 1,848 passed,
zero failed, two existing ignored. The compiler suites were rerun after extracting
the optional-word check emitter: 143 passed, zero failed or ignored. Full clippy
for all five crates with --tests -- -D warnings passes. Citation audit selects
zero changed sites; zero noncompliant sites and zero stale citations (17,780).
Assurance for this group: one added compiler runtime test, four re-spelled English
tests, one existing compiler runtime test strengthened, zero restored, removed or
newly ignored. The independent optional-Else regression test passes unchanged.

This is a complete direct-reference group, not ticket completion. The final table
currently has 17 converted candidates, 23 retained properties and nine pending
class conversions: ComparativeDegreeModifierUse, AuxiliaryUse, FusedHeadUse,
ComparativeQuantityUse, NoncorrelativeCoordination, NominalLicense,
KeywordComplement, SuperlativeQuantifierUse and QuantitativeAdjectiveComplementUse.
No plugin/catalog membership route is needed for those measured members. Finish
the two class groups with their gates, update the final table and append the
two-line landing note then. The glossary and READMEs are current for this prefix;
the ticket remains WIP.

## Completion: lexical classes and admission coverage

The final nine decisions complete the disposition table above. Converted
ComparativeQuantityUse, NoncorrelativeCoordination, NominalLicense and
SuperlativeQuantifierUse into ComparativeQuantifier, NoncorrelativeCoordinator,
DependentGenitivePronoun and SuperlativeQuantifier. Their 16 members state
membership on their vocabulary/native data declarations; the four flags and
FeatureSource records are retired. Nor's explicit No is removed as well: every
reader required Yes, and nonmembership preserves that exclusion.

Retained ComparativeDegreeModifierUse, AuxiliaryUse, FusedHeadUse,
KeywordComplement and QuantitativeAdjectiveComplementUse under the clarity
rule. These state permissions at their reader sites; the proposed class names
would largely restate the same roles. AuxiliaryUse's three holders do not define
all auxiliaries, and Anywhere/This do not define all fused-head determinatives.
The final total is 21 converted candidates and 28 retained properties. There
are no pending candidates and no singleton classes. BareSingularUse and
IdentityUse remain properties, so no plugin/catalog membership route is added.
The glossary's Closed Word and Lexical Class definitions remain current.

Three new compiler runtime tests independently construct accepted and rejected
Readings for required word selectors, direct is/in requirements, policies,
schema instances, policy use inside instances, optional is/in through policies
and instances, and form-local constraints. They check admission and realization,
plus parsing controls. Existing tests retain optional separator-before/after,
absence, mixed Categories, capitalization and spelling-variant assertions.
Optional lexical_word nesting is not a supported syntax: optional fields use
an explicit lexical Category with is/in. No compiler capability was added.

The earlier omission could have admitted and realized the wrong optional lexeme
or class member in any construction, including requirements inherited from a
policy/schema or confined to a particular form. A census of parsed Readings
could miss all such false admissions because parsing already enforced the
constraint. Required fields use Rule::advance, which checks lexical_checks;
optional fields have the separate emitter path now exercised above. Dynamic
selected-frame expansion maintains the same per-symbol array, covered by the
previously repaired expansion and the unchanged selected-frame regressions.
The added tests find no further regression.

One English test is re-spelled in this step:
correlative_markers_keep_determinative_identity_and_nor_is_distinct now checks
NoncorrelativeCoordinator membership while retaining its identity, Category and
CoordinationKind assertions. Assurance: three added tests, one re-spelled, zero
restored, removed or newly ignored. No assertion was weakened.

The seven unsupported frame signatures remain findings, not fixes: Deal's
MassNoun Complement; Enter's FrameComplement Complement and two Object Complement
signatures; Put's two Object Complement signatures; plugin exile's
ResultativeComplement Complement. No frame was activated or reordered.

Final full-corpus gate against the version-2 baseline: all 32,828 faces have
identical digests; 30,794 read faces and 482,041 Readings before and after;
zero lost, gained, count or digest-only changes, incomplete faces or issues.
The subset retained Nissa's 20 Readings. The completed 1.1 GB step census was
deleted after the gate; the baseline remains under target/english-v3.

The sorted comparison matches 927 rule contracts, 326 constructor records and
666 emitted shapes. The structured comparison checks all 35,603 lexemes and all
23 replacement predicates on every entry, including absence. Cumulatively,
only 21 identified flags/provenance and 46 dead map values are retired; 47
entries differ. IDs, forms, variants, other properties and ordered frames match.
All seven unsupported signatures and unmapped declarations are unchanged.
The inventory hash changes from the direct-reference prefix's
c84e961550a48d6697542b391b3a967409f50ce7c9769e35910da09ad3e85df5 to
3d72e2c503c8f8d7f31cb7c6a537f091ec592ecc257f64757b47ea2f566398e2.

Final authored declarations.rs: 6,071 lines (6,093 at ticket start), 163 features
(184 at start), 16 reference-only closed words and five classes with 20 total
memberships. This step adds four class declarations and removes four features,
so its line count remains 6,071. Vocabulary stays data; no lexeme is synthesized
by the grammar. The four deleted metadata keys remain BareLocativeComplement,
PrepositionAttachment, PrepositionComplementKind and the infinitival marker's
unread Finiteness map key; typed form finiteness is unchanged. The remaining
25 fixed metadata keys and producer-owned provenance namespaces are documented
in the lexical-source README.

Full cargo test for deckmaste_construction_v3_core, deckmaste_construction_v3,
deckmaste_lexical_source, deckmaste_english_v3 and xtask: 1,851 passed, zero
failed, two pre-existing ignored with their blockers unchanged. All eight
lexical-requirement runtime tests passed again after the test-only lint fix.
Full clippy --tests -- -D warnings passes for all five crates. Citation audit
selects zero changed sites; zero noncompliant strings and zero stale references
among 17,780 citations. READMEs and the glossary are current. All requested
candidate decisions and verification are complete; integration remains the
owner's action, so this ticket stays in WIP until kata integrates it.

Census delta (ryxnzzsu): faces read 30,794 → 30,794; Readings 482,041 → 482,041; lost none; identical digests on all 32,828 faces.
Judgment set (ryxnzzsu): not applicable yet.
