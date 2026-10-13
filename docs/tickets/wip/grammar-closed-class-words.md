---
needs: [grammar-generic-coordination]
---
# Closed-class words in the grammar crate, typed classes, checked references

Lexical distinctions reach syntax as untyped string flags today. 72 of 184 features are in no category interface, 37 of those are read at exactly one site, and 39 are written for exactly one holder in the lexicon, for example `InfinitivalMarker { Yes }` (held by *to*), `RelativeSubordinator { Yes }` (*that*), `FocusingModifierUse { Restrictive }` (*only*), `SelectedTailLicence` (*put*). `core.ron` carries about 192 per-lexeme patches of grammar-internal flag names, and an authored value outside a feature's domain silently becomes absent (`crates/deckmaste_construction_v3_core/src/emit.rs:317–331`).

## Change

- Closed-class words (determinatives, prepositions, coordinators, subordinators, auxiliaries, pronouns, negators) are declared in Rust in the grammar crate, with their class memberships, so a reference to anything else cannot compile. `crates/deckmaste_lexical_source/lexicon/vocabulary.ron` moves there and is deleted, taking its `provenance_path` with it. `verbs.ron` and the nouns in `core.ron` stay data.
- Inventory first. List every `vocabulary.ron` declaration by destination and by direct-reference eligibility. The file also holds `PredicativeAdjective` (line 24), `ColorWord` (149), `Supertype` (216) and the `CommonNoun` lexeme (224), which are open-class and are not eligible for direct reference; each gets a stated destination (typed class reachable through frames, or data that stays). Closed-class material that lives elsewhere (patches in `core.ron`) is listed too. Open-class members stay out of the directly referenceable Rust type.
- Loader boundary. The grammar crate depends on `deckmaste_lexical_source` only as a dev-dependency, and `deckmaste_lexical_source` does not depend on the grammar crate, so the Rust declarations cannot simply be read by the loader. The landing states who supplies them (for example the caller, `xtask` and the tests, passes them into the load; or a crate both depend on) with no dependency cycle, and keeps today's order: native declarations, then core, then plugins, then normalisation (`crates/deckmaste_lexical_source/src/lib.rs:39–50`). Callers, the lexical-source tests (`crates/deckmaste_lexical_source/tests/suite/errors.rs:53`) and the incremental-source accounting (`crates/xtask/src/english_v3/incremental.rs:15`) are updated.
- The per-lexeme feature patches in `core.ron` become class memberships.
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
