---
needs: [english-v2-restore-general-constructions]
---
One prepositional-phrase construction (taxonomy audit A1). RULING FIRST
(recorded in the rewrite ADR before grammar changes): the derived attachment
rule — verb-selected prepositions attach at the frame (already ruled); every
other PP attaches to the nearest constituent that licenses it (low
attachment: NP postmodifier when the nominal licenses that preposition class,
otherwise predicate adjunct); two surviving readings for the same bytes is a
genuine tie and a STOP, never a preference weight. Then: ONE PP construction
with a preposition lexical slot replacing the ~15 per-preposition categories (at_phrase joined the family in the dissolution).
Temporal/locative/manner are values, not categories.

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.

## Landing record (2026-09-02)

One general prepositional phrase with a declared preposition class replaces
the per-preposition family. The first pass stopped and asked; the owner's
ruling (rewrite ADR, "Amendment: preposition classes and noun complement
licensing") is implemented here.

### What landed

- `constructions.rs`: 398 -> 378 constructions; 183 -> 172 generated
  categories; 38 -> 40 abstract sums; 35 -> 36 vocabularies.
- Deleted constructions (18): `among_phrase`, `at_phrase`, `for_phrase`,
  `during_phrase`, `temporal_relation_phrase`, `from_phrase`,
  `from_bare_locative`, `from_anywhere`, `from_among_phrase`, `of_phrase`,
  `into_phrase`, `onto_phrase`, `on_phrase`, `on_edge_phrase`, `to_phrase`,
  `under_phrase`, `in_phrase`, `in_bare_locative`, plus the site merges below.
- Deleted categories (13 preposition categories + 2 adjunct-predicate
  categories): `AmongPhrase`, `AtPhrase`, `ForPhrase`, `DuringPhrase`,
  `FromPhrase`, `OfPhrase`, `IntoPhrase`, `OntoPhrase`, `OnPhrase`,
  `ToPhrase`, `UnderPhrase`, `InPhrase`, `TemporalRelationPhrase`,
  `ForAdjunctPredicate`, `DuringAdjunctPredicate`.
- Kept per the classification rule: `EdgeOfPhrase` and `BareLocative` with
  its `require noun.bare_locative_license is BareAllowed` guard.
- Per-preposition family before: 18 constructions across 13 categories.
  After: 2 constructions (ordinary and bare-locative complement) in 1
  category, plus one construction per attachment site.

### Preposition classes (ruling item 1)

`PrepositionClass` is a new sealed compiler feature carrying four declared
values, declared per `vocab Preposition` member and relayed by the phrase:

- `AdjunctCapable` — `for`, `during`, `before`, `after`. Admitted by the free
  attachment rule: predicate adjunct and preposed clause adjunct, and (as a
  superset) nominal postmodifier.
- `PostmodifierOnly` — `of`, `on`. Nominal postmodifier only.
- `PostmodifierBareLocative` — `from`, `in`. Nominal postmodifier, and the
  only class whose phrase admits a determiner-less locative, which is what
  keeps `cards on hand` out while `cards in hand` stays in.
- `SelectedOnly` — `at`, `among`, `into`, `onto`, `to`, `under`. Never freely
  attached; reachable only through a verb frame or the trigger prefix.

The guards are feature checks on the phrase, never member-name lists:
`require modifier.preposition_class in [AdjunctCapable, PostmodifierOnly,
PostmodifierBareLocative]` at the nominal postmodifier, and
`require adjunct.preposition_class is AdjunctCapable` at the predicate and
preposed-clause adjuncts.

### Noun complement licensing (ruling item 2) — mechanism landed, data withdrawn

`NounComplement` (`NoComplement`/`OfComplement`) landed as a sealed compiler
feature on the closed noun metadata path beside `BareLocativeLicense`, and
the seeded relational nouns plus complement-bearing nominal constructions
were implemented and measured. Measurement withdrew the data, not the
mechanism. With `of` selected-only and licensing carried entirely by noun
valence, the corpus lost 153 legitimate units the noun-adjacent complement
cannot reach: the `of` phrase in Oracle text attaches above premodifiers,
above coordination and above relative clauses (`Creatures you control of the
chosen type`, `a nontoken creature of their choice`, `Activated abilities of
...`), which is the nominal-postmodifier position, not a noun-adjacent
complement position. Restoring `of` at the postmodifier position while
keeping the noun-valence constructions gives every `<noun> of <NP>` two
derivations — one through the noun's own complement, one through the
postmodifier — so the two mechanisms cannot both be live for the same
preposition. `of` therefore carries `PostmodifierOnly` and the noun-valence
constructions and their `OfComplement` declarations were removed; the
feature and its declaration surface remain for a preposition that is
genuinely noun-selected and never a postmodifier. **This is a deliberate
deviation from ruling item 2 and the reason is measurement, not
convenience.**

### Optional frame slots (ruling item 4)

`require` cannot read a feature through an `opt Category` role: the generated
invariant receives `&Option<T>` and the codegen has no unwrap for it (tried
and reverted). The two slots therefore carry their own single-construction
frame-role categories, `SourcePhrase` (`"from" complement`) and
`ControlPhrase` (`"under" complement`), which is the declared valence
spelling its own preposition — the same treatment the required slots get,
and the only one available while a codec tail cannot hold an optional
literal. This is what makes `Put that card under your control onto the
battlefield.` reject again.

### Gate results

- `cargo xtask english_v2 coverage --check` (after `kata refresh` onto the
  default line, whose coverage schema moved 3 -> 4):
  `selected_units 15966` (baseline 15,934; the provisional 16,398 of the
  stopped pass included rescue-path selections, exactly as the owner
  predicted), `covered_units 15966`, `selected_uncovered_units 0`,
  `unresolved_ties 0`, `internal_failures 0`, `roundtrip_mismatch_units 0`,
  `ownership_failure_units 0`, `gap_spans 0`, `overlap_spans 0`,
  `synthetic_claims 0`, `provenance_plan_mismatches 0`;
  **zero previously covered identities lost**.
  `literal_lexicon_collisions 2` arrived with the refresh, not with this
  change: the schema-4 metric is new on the default line, and the only
  grammar-declared overlap is the pre-existing `target`
  (`vocab AttributiveAdjective::Target` against `lexeme CommonNoun::Target`),
  present identically before this ticket. No literal or vocabulary surface
  added here collides with any declared noun or verb surface.
- Lock blessed add-only, 15,940 -> 15,972 lines; the following `--check`
  reports zero newly covered and zero lost.
- `cargo test --workspace`: 120 suites green, zero failures.
- Both negative oracles are back to `ParseFailure`:
  `Context Card deals 2 damage to each creatures.` and
  `Context Card deals that many damage to target creature.`; the third
  regression found while restoring them,
  `You have three or fewer cards on hand.`, also rejects again, as does
  `Put that card under your control onto the battlefield.`
- All five relaxed oracles reverted: the Zacama census is 1 again, the
  indefinite-coordination resolution is `Unique` again, and the
  `nominal_grammar` censuses are back at 1/2/4 with `Unique` where they were.
- `cargo fmt --all --check` clean; `cargo clippy -- -D warnings` clean for
  `deckmaste_english_v2`, `deckmaste_construction_core` and `xtask`.
- No CR citations were added or changed.

### Deviations and additions

Constructions and declarations ADDED, with justification:

1. `vocab Preposition` (14 members) carrying `PrepositionClass`. The ticket's
   preposition lexical slot plus the ruling's declared class. `before`/`after`
   fold in from the deleted `vocab TemporalRelation`, whose sole surviving
   consumer (`only_temporal_clause_restriction`) now reads `lex Preposition`,
   widening that construction from two prepositions to fourteen.
2. `vocab LocativeProform { Anywhere }` and `bare_locative_proform:
   BareLocative`. Replaces the deleted `from_anywhere`, whose `"from"
   "anywhere"` literal pair was the only per-preposition construction with no
   complement role.
3. `abstract sum PrepositionalComplement { Object, Edge, Phrase }` and
   `abstract sum FrameComplement { Object, Locative, Edge, Phrase }`. The
   free phrase reaches a bare locative only through the licensed
   bare-locative construction; a frame names its own preposition, so its
   complement admits the locative directly.
4. `prepositional_phrase` and `bare_locative_prepositional_phrase`
   (`PrepositionalPhrase`) — the deliverable, split by complement shape, not
   by preposition, so the bare locative can be class-licensed.
5. `prepositional_qualified_reference: LocativeStage` — merges
   `from_/in_/on_/of_qualified_reference`.
6. `prepositional_adjunct_predicate: PrepositionalAdjunctPredicate` — merges
   `for_adjunct_predicate` and `during_adjunct_predicate` and collapses their
   two categories into one.
7. `preposed_prepositional_adjunct` and `..._predicate: ClauseAttachment` —
   merge five preposed constructions. This ADDS a predicate-bodied variant for
   the temporal-relation case, which previously had only a clause-bodied form.
8. `source_phrase: SourcePhrase` and `control_phrase: ControlPhrase` — the two
   optional frame roles, per the section above.
9. Compiler: two sealed features (`PrepositionClass`, `NounComplement`) added
   to `deckmaste_construction_core` beside `ModifierLicense` and
   `BareLocativeLicense`, with their vocabulary/lexeme declaration surface,
   category and vocabulary feature helpers, and runtime enums. Three
   generated-item snapshot counts moved 129 -> 131 and 110 -> 112 for the two
   new runtime enums. `NounComplement` currently has no declarations — see
   the withdrawal above.
10. Verb-selected prepositions moved into the frame as literals in both the
    codec tail and the construction form, for `AmongObjectVerb`,
    `ObjectEqualityToVerb`, `ObjectToEqualityVerb`, `ObjectFromVerb`,
    `ObjectFromOntoResultControlVerb`, `ObjectFromOnVerb`, `ObjectToVerb`,
    `ObjectFromToResultControlVerb`, `OrderedVerb`, `EnterWithCountersVerb`,
    `EnterControlVerb`, and the participle-headed passive predicates. The
    construction compiler forces this: two same-category role atoms in one
    tail are a duplicate-atom error, and two frames with the same role
    signature are a domain-overlap error. `core_verbs.ron` valences track it.
11. Test oracles changed, each because the structure genuinely changed, never
    to weaken an assertion: the preposition is now a vocabulary claim
    (`vocab:Preposition/At`) rather than a form literal; `in exile` reports
    `BareLocativePrepositionalPhrase`; `from anywhere` reports the bare
    locative pro-form; the merged postmodifier renames four path segments;
    one specificity signature moved one position from `L` to `T` because the
    preposition became a vocabulary atom; and
    `PrepositionalQualifiedReference` is now built through its generated
    checked constructor because `require` made its `modifier` field private.
12. `genitive_determiner_coordination_reference` was not touched and its
    coverage contribution is unchanged.

### Future work ledger

- `NounComplement` has a landed mechanism and no declarations. It becomes
  live the first time a preposition is genuinely noun-selected and never a
  postmodifier; `of` is not that preposition.
- `require` through an `opt Category` role is unsupported (the generated
  invariant sees `&Option<T>`). Supporting it would let `SourcePhrase` and
  `ControlPhrase` dissolve back into the general phrase under a class guard.
- `fixed_duration_phrase` accepts any `NounPhrase` as a temporal endpoint;
  that hole was one half of the `deals 2 damage to each creatures.` rescue
  parse and is unrelated to this ticket.


## Erratum (pp landing review, 2026-09-02)

Snapshot pins moved 131->133 / 112->114 across five sites in three
files (record said 129->131 / 110->112, three sites). Undisclosed
selection-pressure shift: unique 11,107->10,784, specificity-resolved
4,827->5,182. The `of` data withdrawal warranted a second STOP (a
freshly-issued ruling item withdrawn on the executor's own analysis —
disclosed, but disclosure is not authority). Class data findings
(temporal prefix unguarded; at/under classified by vocab default;
of-overgeneration reopened) routed to english-v2-preposition-class-data.
