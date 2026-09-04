---
needs: [english-v2-number-feature-unification]
---
**Collapse the six `genitive_determiner_*` constructions, whose discriminating
guards are inert.** `genitive_determiner_{singular,plural,mass}_reference` and
their `plural_genitive_determiner_*` twins share the form `possessor nominal`
and byte-identical `derive` blocks; each pair differs only by
`require possessor.number is Singular` versus `… is Plural`. `possessor.number`
feeds no `derive`, and `{Singular, Plural}` is the whole `Number` domain, so
each pair is an exhaustive partition with no observable consequence: a
possessive of any number followed by a singular nominal parses once either
way.

Pinned shape: after number unification there is one
`genitive_determiner_reference: UnqualifiedReference { possessor: Possessive,
nominal: Nominal }` with the existing derives, plus
`genitive_determiner_coordination_reference` unchanged. If this lands before
the unification, collapse to three (singular / plural / mass nominal) by
deleting the `possessor.number` requires; never re-encode possessor number as
an AST product distinction. The `'s`/`'` choice stays in `construction
possessive`. Audit `possessed_{singular,plural,mass}_reference` (form
`lex(possessor) nominal`) for the same fold under the same test: a guard that
reads a feature no derive consumes and partitions its whole domain is inert.

Fence: a possessor-number distinction retained "for the downstream layer" is a
STOP; the downstream layer reads `possessor.number` from the `Possessive` node.

Acceptance: construction count down by the folds, selection census unchanged
for every possessive identity, byte-exact laws green. Standard constraints
apply.

## Landing record

- Measured tree: change `woulvwsttuos` (code) / `zzuzpnsvvsol` (this record),
  coverage-lock `covered` count 16,824. All figures below were re-measured by
  the landing reviewer after `kata refresh` picked up the concurrently
  integrated `english-v2-frame-selected-prepositions` landing. The coverage
  lock is byte-unchanged: schema-4 lock file
  (`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`), read by
  the schema-9 coverage report.
- Folded three possessor-number pairs: singular
  `genitive_determiner_singular_reference` with
  `plural_genitive_determiner_singular_reference` into
  `genitive_determiner_reference`; plural
  `genitive_determiner_plural_reference` with
  `plural_genitive_determiner_plural_reference` into
  `genitive_determiner_plural_reference`; and mass
  `genitive_determiner_mass_reference` with
  `plural_genitive_determiner_mass_reference` into
  `genitive_determiner_mass_reference`. In each pair the deleted
  `possessor.number` requirements were the exhaustive, derive-inert
  Singular/Plural partition; all derives and every nominal requirement remain.
- Inertness evidence for the deleted requirements: no `derive` and no
  `checked by` in any of the six constructions read `possessor.number` — every
  derive reads `nominal.*` or `Values::Consonant`. `Feature::Number`'s domain
  is exactly `{Singular, Plural}`
  (`crates/deckmaste_construction_core/src/feature.rs:208`), and the sole
  `Possessive` construction derives `number = owner.number` unconditionally, so
  each pair's two exact-value requirements covered the whole domain and their
  union is no requirement at all. The `'s`/`'` choice remains three guarded
  forms inside `construction possessive`, per the ADR's possessive-ending
  paragraph; no possessor-number distinction was retained for a downstream
  layer.
- Pinned shape versus delivered shape. The ticket pins one
  `genitive_determiner_reference` after number unification; this landing
  delivers three genitive branches, which is the ticket's own pre-unification
  fallback ("collapse to three (singular / plural / mass nominal)"). The reason
  the unified single construction is not reachable is the
  `english-v2-number-feature-unification` landing: it deleted the narrow
  `SingularNominal`/`PluralNominal` Categories and re-spelled their membership
  as `require nominal.nominal_form in [...]` on the genitive and possessed
  references, per the ADR's *Plan 08 unified-membership clarification
  (2026-09-04)* — "re-spelling former Category membership as a `nominal_form`
  requirement is the unified shape's replacement for the deleted Category, not
  a narrowing". Those requirements are load-bearing, not inert, so the three
  branches stay distinct: `genitive_determiner_reference` requires
  `nominal.number is Singular` and `nominal.nominal_form in [BareSingularNoun,
  ModifiedSingularNoun]`; `genitive_determiner_plural_reference` requires
  `nominal.number is Plural` and `nominal.nominal_form in [BarePluralNoun,
  ModifiedPluralNoun]`; `genitive_determiner_mass_reference` requires
  `nominal.nominal_form is MassNoun`. These are byte-identical to the
  requirement sets the number-feature landing's review narrowed them to; no
  coordination `nominal_form` was re-admitted. `genitive_determiner_coordination_reference`
  is untouched.
- `possessed_*` audit (the ticket's second obligation), per construction:
  none of `possessed_singular_reference`, `possessed_plural_reference`, or
  `possessed_mass_reference` ever carried a `possessor.number` requirement —
  their possessor is a `lex PossessiveDeterminerPronoun`, and the only feature
  any of them reads off it is `onset`, which `derive onset = possessor.onset`
  consumes. So there is no inert possessor-number partition to fold in any of
  the three. What distinguishes them is not inert either:
  `possessed_singular_reference` requires `nominal.number is Singular` (read by
  `derive number = nominal.number`) plus `nominal.nominal_form in
  [BareSingularNoun, ModifiedSingularNoun]`;
  `possessed_plural_reference` requires `nominal.number is Plural` (same
  derive) plus `nominal.nominal_form in [BarePluralNoun, ModifiedPluralNoun]`;
  `possessed_mass_reference` requires `nominal.nominal_form is MassNoun`, the
  unified-membership spelling of the deleted mass Category. No fold.
- Out-of-scope observation, no obligation: `demonstrative_possessive_reference`
  still carries `require possessor.number is Singular`. It is *not* inert — it
  is a single construction, so the requirement excludes plural possessors
  rather than partitioning the domain across a twin. Whether that exclusion is
  right is a grammar-coverage question outside this ticket, and no ticket is
  minted for it.
- Construction count: 394 before / 391 after, exactly the three folds above.
- Coverage (`coverage --check`, this tree): 32,641 total units, 16,824 selected
  and covered, 15,817 parse failures, 0 selected-uncovered, 0 unresolved ties,
  0 internal failures, 0 exception resolutions or uses, 0 round-trip
  mismatches, 0 ownership failures, 0 traversal failures, 0 leaf-traversal
  failures, 0 gap spans, 0 overlap spans, 0 synthetic claims, 0
  provenance-plan mismatches. Licensing checkers 25 permitted / 0 forbidden.
  No identities became newly covered; no identity was retired.
  (`form_literal_vocab_overlaps` reads 5 and visited leaves 244,945 on this
  tree against 25 and 237,017 before the refresh — that shift belongs to the
  `english-v2-frame-selected-prepositions` landing this refresh absorbed, not
  to this one.)
- Selection census: 11,527 unique / 5,297 specificity-resolved before and
  after, 0 unresolved ties. The reviewer re-ran the complete `ambiguity --json`
  census on the refreshed claim base and on this tree and compared all 32,641
  rows by id: 0 status changes, 0 gained or lost selections, 0 changed
  resolution modes, 0 changed candidate counts. 1,406 rows changed a raw
  construction-path spelling; after substituting the three folded construction
  names the diff is 0 changed selected analyses. The only path segments that
  differ across the two runs are the three removed names
  (`UnqualifiedReferenceGenitiveDeterminerSingularReference`,
  `UnqualifiedReferencePluralGenitiveDeterminerPluralReference`,
  `UnqualifiedReferencePluralGenitiveDeterminerMassReference`) and the one new
  name (`UnqualifiedReferenceGenitiveDeterminerReference`);
  `PluralGenitiveDeterminerSingularReference` was never selected on the corpus.
- Positive gates (all foreground, rerun after refresh): `cargo fmt --all` clean
  (`jj st` reports no changes); strict all-target Clippy for
  `deckmaste_english_v2` finished with no warnings; `cargo test -p
  deckmaste_english_v2 -p xtask` green — `test result: ok. 143 passed; 0
  failed; 0 ignored`, `test result: ok. 49 passed; 0 failed; 0 ignored`, `test
  result: ok. 100 passed; 0 failed; 0 ignored`, `test result: ok. 444 passed; 0
  failed; 1 ignored` (the single xtask ignore is pre-existing; no xtask file is
  touched by this landing). `coverage --check` passed with the figures above;
  `ambiguity --require-resolved` passed with `summary selected=16824`, `summary
  unique=11527`, `summary specificity_resolved=5297`, `summary
  unresolved_ties=0`; `roundtrip` reported `parse accepted 16824 / clean 16824
  / mismatched 0`. `cite check --list-noncompliant` printed `0 non-compliant
  citation-looking string(s)` and `cite check` printed `checked 14226 citations
  against cr.txt (eff. 2026-08-07); 0 stale`. No citation changed in this
  landing.
- Performance advisory (reviewer's re-measurement, 8 workers on every corpus
  command): coverage 102 s at 130,006 ns/B (host load 10.46/14.42/14.82);
  `ambiguity --require-resolved` 90.675 s at 115,135 ns/B (8.02/12.39/14.04);
  roundtrip 91.842 s at 110,437 ns/B (7.05/10.67/13.10). Each is far past the
  16.26 s quiet-host ceiling, and the per-byte thread-CPU figures are ~25-30%
  above the implementer's own run for the same tree. Contention stamp supplied
  by the coordinator: 1 concurrent codex executor and 1-2 concurrent landing
  reviewers on this host for the whole measurement window, plus this review's
  own base-tree corpus runs. The wall times are a contention artifact, not a
  parse-time regression from this landing.
- Assurance counts: restored 0; re-spelled 1 (the `ability_logic.rs` visitor
  contract, `visit_genitive_determiner_singular_reference` /
  `GenitiveDeterminerSingularReference` becomes
  `visit_genitive_determiner_reference` / `GenitiveDeterminerReference`; same
  card, same asserted traversal sequence, still a full value comparison of the
  visited-name vector); ignored with blockers 0; added 0; removed 0.
- Deviations and additions: one, disclosed above — three surviving genitive
  branches rather than the ticket's pinned single
  `genitive_determiner_reference`, taken under the ticket's own three-branch
  fallback because the number-feature landing's `nominal_form` requirements are
  load-bearing. No construction or test was added or deleted beyond the
  ticket's letter. STOP: none taken; none owed (the retained distinction is a
  nominal-form distinction, not the possessor-number distinction the ticket's
  fence forbids). glossary gap: none.

### Review corrections

- MEDIUM — the record did not say why the delivered shape is three
  constructions where the ticket pins one. Added the *Pinned shape versus
  delivered shape* entry, citing the `english-v2-number-feature-unification`
  landing and the ADR's Plan 08 unified-membership clarification, and listed
  the three survivors' exact requirement sets.
- MEDIUM — the `possessed_*` audit gave a single collective reason. Replaced it
  with a per-construction finding, and corrected the framing: none of the three
  ever had a `possessor.number` requirement at all, because their possessor is
  a `lex PossessiveDeterminerPronoun`.
- LOW — the record called the lock "schema-9"; the lock file is
  `schema_version` 4 and 9 is the coverage *report* schema. Both are now named
  correctly.
- LOW — the code commit was described `english_v2:`; the house prefix is
  `english-v2:`. Re-described.
- LOW — the implementer could not see host contention from its sandbox. The
  coordinator's contention stamp is now in the performance advisory, alongside
  the reviewer's own re-measurement on the refreshed tree.
- LOW — added the inertness evidence (the `Feature::Number` domain and the sole
  `Possessive` construction's total `number` derive) that makes the deletion
  provably selection-neutral rather than asserted, and the out-of-scope
  observation about `demonstrative_possessive_reference`.
