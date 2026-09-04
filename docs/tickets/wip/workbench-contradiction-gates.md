---
needs: []
---
**Keep only the rules-defined contradiction clashes; delete `Phrase.predEq`
and its two gates.** Ruling 2026-09-04 on contradiction gates (cleanroom
review 3, finding U5 and D-Q3). The ruling is delete, not complete.

- `Phrase.predEq` answers `False` for `p` against itself on eight structural
  predicates — `Targets`, `ManaCostHas`, `CombatRel AttackerOf`, `AttachedTo`,
  `CompareOver`, `Or`, `OtherThan`, `Joined` and `CastBy _ (Just _)` — so
  `noNegatedPair` and `noRepeatedPair` never fire on them and
  `And [AttachedTo (target creature), Not (AttachedTo (target creature))]` is
  admitted. Delete `predEq`, `noNegatedPair` and `noRepeatedPair`.
- `Phrase.contradictionFree` keeps only the clashes the CR makes meaningless:
  token against card, permanent against spell, status, colour, and the emptied
  seed. Its other consumer `DistinctDisjuncts` either gets a replacement read
  stated over those clashes or goes with them; say which in the landing.
- Every pin that refuted through `noNegatedPair` or `noRepeatedPair` is
  re-spelled against the surviving clash it names, or retired together with
  its subject and named in the assurance counts. None is deleted silently.

Size: S. Done when: `grep` finds no `predEq`, `noNegatedPair` or
`noRepeatedPair` in the tree; each surviving clash has a pin and a same-module
twin, probed non-vacuous; the restored, re-spelled and removed counts are
reported; build at its module count. Standard constraints apply, including the
RON-shaped constraint.

## As landed

- `Phrase.predEq` (125 lines), `predEqAll`, `negates`, `anyNegates`,
  `noNegatedPair`, `anyPredEq`, `noRepeatedPair` and `DistinctDisjuncts` are
  deleted. `sameChoiceDomain` and `sameDomainOpt` went with them: `predEq`'s
  `QualityNoun`/`OfYourChoice` rows were their only consumers.
- `Phrase.contradictionFree` now reads five families, each carrying its rule:
  the emptied seed [CR#205.3c], status [CR#110.5], colour [CR#105.2c],
  card/token/emblem [CR#111.6,114.5], and permanent against a non-permanent
  type [CR#110.4]. No family was completed or widened.
- `DistinctDisjuncts` **goes with them** — no replacement read. A repeated
  disjunct is redundant, not CR-meaningless, and no surviving clash says
  anything about disjunct identity; `Or` loses its `dd` slot.
- `roleElem` (the party-role gate, `predEq`'s one consumer outside the
  contradiction lane) is re-based on a new `roleSubtype`: a party role is a
  creature subtype [CR#700.8], so roles repeat by subtype. `rolesDistinct`,
  `RolesOk` and both party pins are untouched and still refuse.
- Pins moved (9, none deleted). Re-spelled against a kept family (1):
  `ProofsZone.badNontokenToken` "Destroy target nontoken token"
  → `badCardTokenTarget` "Destroy target card token" (`And [IsToken, IsCard]`,
  card/token family), probed non-vacuous. Became positive twins (8) — the CR
  makes none of them meaningless, each is an empty but harmless description:
  `ProofsDescription.badKeywordContradiction` → `okKeywordSelfNegation`,
  `badBlockedAndUnblocked` → `okBlockedAndUnblocked`,
  `badRepeatedStructuredDisjunct` → `okRepeatedStructuredDisjunct`;
  `ProofsDeontic.badRepeatedComparisonDisjunct` → `okRepeatedComparisonDisjunct`;
  `ProofsCounters.badControlContradiction` → `okControlSelfNegation`;
  `ProofsZone.badQualityContradiction` → `okQualitySelfNegation`,
  `badNestedContradiction` → `okNestedSelfNegation`,
  `badRepeatedDisjunct` → `okRepeatedDisjunct`.
- Every surviving `cf` pin still refuses (the build re-checks each
  `Oh impossible`), and each module keeps a same-module positive twin — the
  converted twins serve that role in ProofsCounters and ProofsZone.

## Landing record

Change id `wtluknkr`; measured on this tree.

Numbers before/after:

- `idris/src/Experimental/Phrase.idr`: 3023 → 2849 lines.
- `contradictionFree` clause families: 6 → 5.
- `Or` obligations: 4 → 3 (`ne`, `pd`, `cd`).
- Pins standing on a `cf` or `dd` obligation: 20 → 12 (17 `cf` → 12, 3 `dd` → 0).
- Modules built: 46/46 → 46/46.

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`; 0 lines matching Warning/Error/FAILURE.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14349 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 7
  citation site(s)`; every rule text read against its claim. No `cite bless`
  was needed — all seven rules were already in `cr-citations.lock`.
- Pin probe: `badCardTokenTarget` with `IsCard` swapped for `Attacking` →
  `Error: badCardTokenTarget Oh is not a valid impossible case.`; restored.

Assurance counts: restored 0, re-spelled 9 (1 kept as a pin against the
card/token family, 8 became positive definitions), ignored with blockers 0,
added 0, removed 0.

Deviations and additions:

- Added `Phrase.roleSubtype` (3 lines) so the party-role gate survives
  `predEq`'s deletion without a whole-predicate equality. Roles that are not
  subtypes no longer collide; the CR's party is four creature subtypes
  [CR#700.8], so nothing rules-meaningful is lost.
- Deleted `sameChoiceDomain`/`sameDomainOpt` (dead after `predEq`).
- Attempted and reverted: a `seedTypeAlts (HasType t) = [t]` row, so that
  "creature that is a noncreature" would stay refused inside the kept
  emptied-seed family. It regressed a printed card — Caught in the Crossfire's
  `And [creature, Not outlaw]`, where `Not (Or [creature subtypes])` reports
  Creature as negated — so the row was reverted and that pin became a positive
  twin instead. The emptied-seed family is deliberately conservative; leaving
  it alone is what the ruling asks for.
- Not edited (outside this round's file scope): `docs/idris-workbench-closure-tables.md`
  still carries rows for `predEq`, `noRepeatedPair` and `DistinctDisjuncts`,
  and `docs/decisions/workbench-ron-shaped-and-label-rulings.md` names them as
  pending. Both are now stale and want a follow-up pass.

STOP taken: none.
