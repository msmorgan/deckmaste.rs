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

- Measured tree: change `woulvwsttuos`, coverage-lock `covered` count 16,824.
  The schema-9 lock is byte-unchanged
  (`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`).
- Folded three possessor-number pairs: singular
  `genitive_determiner_singular_reference` with
  `plural_genitive_determiner_singular_reference` into the pinned
  `genitive_determiner_reference`; plural
  `genitive_determiner_plural_reference` with
  `plural_genitive_determiner_plural_reference` into
  `genitive_determiner_plural_reference`; and mass
  `genitive_determiner_mass_reference` with
  `plural_genitive_determiner_mass_reference` into
  `genitive_determiner_mass_reference`. In each pair the deleted
  `possessor.number` requirements were the exhaustive, derive-inert
  Singular/Plural partition; all derives and every nominal requirement remain.
- Did not fold the three surviving genitive branches with one another: their
  `nominal_form` requirements distinguish singular, plural, and mass Nominal
  forms. Did not fold `possessed_singular_reference`,
  `possessed_plural_reference`, or `possessed_mass_reference`: their remaining
  `nominal.number` requirements feed the derived Number where present, and
  their `nominal_form` requirements preserve the singular/plural/mass Nominal
  distinction. None is an inert possessor-number partition.
- Construction count: 394 before / 391 after, exactly the three folds above.
- Coverage: 16,824 selected and covered / 15,817 parse failures before and
  after; 0 selected-uncovered, unresolved ties, internal failures,
  exception resolutions or uses, round-trip mismatches, ownership failures,
  traversal failures, gap spans, overlap spans, synthetic claims, or
  provenance-plan mismatches. Licensing checkers remain 25 permitted and 0
  forbidden. No identities became newly covered.
- Selection census: 11,527 unique / 5,297 specificity-resolved before and
  after, with 0 unresolved ties. The complete before/after `ambiguity --json`
  comparison has 0 gained selections, 0 lost selections, and 0 changed
  resolution modes. Its 1,406 raw path spelling changes are exactly the three
  folded construction-name substitutions; normalizing those substitutions
  gives 0 changed selected analyses.
- Positive gates (rerun after refresh): `cargo fmt --all`; strict all-target Clippy for
  `deckmaste_english_v2`; and `cargo test -p deckmaste_english_v2 -p xtask`
  passed. Representative artifacts: `test result: ok. 143 passed; 0 failed`,
  `test result: ok. 49 passed; 0 failed`, and `test result: ok. 444 passed; 0
  failed; 1 ignored` for xtask. `coverage --check` passed with the coverage
  figures above; `ambiguity --require-resolved` passed with 16,824 selected
  and 0 unresolved ties; the accepted-set byte law reported 16,824
  parse-accepted / 16,824 clean / 0 mismatched. `cite check` reported 0 stale
  citations and the noncompliant list was empty.
- Performance advisory: all corpus commands used 8 workers. Coverage took
  52.222 s at 111,587 ns/B (host load 18.01/16.90/18.55); resolved ambiguity
  took 54.534 s at 108,872 ns/B (11.84/15.15/17.70); roundtrip took 53.965 s
  at 109,502 ns/B (13.90/14.97/17.45). Each exceeds the 16.26-second
  quiet-host ceiling under concurrent load; concurrent-process count is not
  visible from this sandbox and is for reviewer stamping.
- Assurance counts: restored 0; re-spelled 1 visitor contract against
  `GenitiveDeterminerReference`; ignored with blockers 0; added 0; removed 0.
- Deviations and additions: none. STOP: none. glossary gap: none.
