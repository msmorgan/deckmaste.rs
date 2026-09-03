---
needs: []
---
**Add `BareDet` as a determiner value that introduces a group binding, and
fold the four predicate/noun pairs it exists to work around.** Cleanroom
review 2026-09-03, R2, ruled: a bare plural *is* a determiner value.

`Amount.CountOf (p : Predicate)`, `Aggregate op ax (p : Predicate)`,
`AggregateOver`, `Condition.Exists (p : Predicate)` and `Happened`'s
complement take predicates rather than nouns because "creatures you control"
(no determiner) has no `DetPhrase` value — `DetPhrase = TargetDet | ADet |
EachDet | AllDet | TheDet | CountDet` (`Phrase.idr:1412–1418`). That gap is
why `CountOf p` / `CountOfGroup grp` and `Aggregate` / `AggregateOf` are
pairs.

## The ruling

`BareDet` is a determiner value: plural indefinite, plurality `ManyOf`. It
**introduces a group binding**, exactly as `AllDet` does — "creatures you
control … those creatures" reads back in printed text, so the binding is real
and the read must be available. This follows the settled "the determiner is a
slot" ruling; it is not an exception to it.

Then fold, each to the noun-taking member of the pair: `CountOf` /
`CountOfGroup`, `Aggregate` / `AggregateOf`, and `AggregateOver`. `Exists`
takes a `Noun`, and so does `Happened`'s complement.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `BareDet` is a
`DetPhrase` value with `ManyOf` plurality and a group binding in `detDelta`;
`CountOfGroup`, `AggregateOf` and `AggregateOver` are gone and their former
sites read through the surviving noun-taking constructors; `Exists` and
`Happened`'s complement take a `Noun`; a bench witness reads a bare plural
back through `those` in a later sentence; a pin refutes a singular read of a
`BareDet` group, and it is non-vacuous. Standard constraints apply.

## As landed

- `Phrase.DetPhrase` gains `BareDet` (no payload, `[CR#109.2]`); `detOf BareDet = BareD`,
  `detPlur BareDet = ManyOf`, `detOk` falls to the `()` row, and `detDelta` falls to the
  shared default clause, so a bare plural introduces `bindFor BareD ManyOf ph p` exactly
  as `AllDet` does. `Words.Determiner` gains `BareD` (with its `openLetter`, `sameDet`,
  `pluralizeBinding` rows).
- `Amount.CountOf` now takes `(grp : Noun bs k)` with `nounPlur grp = ManyOf` and a new
  `CountableGroup grp` (`nounDet n == Just BareD || groupMention n`, spelled after the
  existing `partitiveBase`); `CountOfGroup` is gone and its one site reads `CountOf`.
- `Amount.Aggregate` now takes `(grp : Noun bs k)` with `projScope ax = k` and
  `nounPlur grp = ManyOf`; `AggregateOf` is gone and its two sites read `Aggregate`.
- `Condition.Exists` now takes `(n : Noun bs k)` with a new `ExistentialMention n`
  (`nounDet n == Just BareD || countedExistential n`); `ExistsGroup` is gone and its two
  sites read `Exists`.
- `Happened`'s complement needed no change: `EventComplement.Involving` already takes a
  `Noun`, as does `Happened`'s `who`. Recorded as already satisfied.
- **Undone — `AggregateOver`.** Its `dom` is a *binder domain*, not a determiner gap: it
  scopes `bindFor TheD OneOf ph dom` over its `body`, and `Phrase.CompareOver` (which the
  ticket leaves alone) and `Predicate.Superlative` have the identical shape. A `Noun`
  domain cannot yield that per-element singular binding, and deleting the row makes
  Investigator's Journal, Cavern-Hoard Dragon and Thought Sponge unspellable. STOP taken
  and recorded below.
- Macros: `bare p = Described BareDet p`, plus `countOf` / `aggregate` / `exists` one per
  folded lemma (each `<core> (bare p …)`); `nForEach`/`forEach` gained the forwarded
  `{auto ph : Phrasal k}` and `{auto 0 pl}` those need. 150 `CountOf`/`Aggregate`/`Exists`
  sites across `Cards.idr` and `Proofs*.idr` re-spelled through them.
- `tiedDelta` deleted (and dropped from `condDelta (CompareAmt …)`): it existed to publish
  the group that a predicate-taking `CountOf` could not, and the bare plural's own binding
  is now that group — keeping both made Purging Scythe's "one of them" ambiguous.
- `countGroups` and `countedGroupSize` skip `BareD` alongside `PartD`: a bare plural is
  not a partitive base (`partitiveBase` already refuses it), so it cannot host "the rest"
  or "the other". Without this Boreas Charger's `TheRest` is refused by the land count
  inside "an opponent who controls more lands than you".
- Witness: `Cards.jeskaiAscendancyPump` — Jeskai Ascendancy's first trigger, "creatures you
  control get +1/+1 until end of turn. Untap those creatures", the bare plural read back
  through `Those (TypeW Creature)`.
- Pin: `ProofsAnaphora.badSingularReadOfBarePlural` — the same sentence with a singular
  `It` read of the bare-plural group, refused.
- Re-spelled witness: `Cards.lightmineField`'s measure is `CountOf (Those (TypeW Creature))`
  ("the number of those creatures") rather than a second bare description of the same set,
  which the printed sentence's own `each of those creatures` would otherwise have two
  antecedents for.

## Landing record

- Construction count: `Phrase.DetPhrase` 6→7, `Words.Determiner` 8→9, `Phrase.Amount`
  35→33, `Phrase.Condition` 18→17. Helper defs: `tiedDelta` removed; `countableGroup`/
  `CountableGroup`, `existentialMention`/`ExistentialMention` added; `Macros` +4
  (`bare`, `countOf`, `aggregate`, `exists`).
- Coverage and lock state: bench coverage unchanged plus one card (Jeskai Ascendancy's
  first trigger); no witness removed. `cr-citations.lock` unchanged — `[CR#109.2]` was
  already registered, so no `bless` was run.
- Positive gate artifacts: `cd idris && ./scripts/build` (clean `rm -rf build`) ends
  `23/23: Building Cards (src/Cards.idr)`, 23 modules, 0 `Error` and 0 `Warning` lines;
  `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking
  string(s)`; `cargo xtask cite check` → `checked 17724 citations against cr.txt
  (eff. 2026-08-07); 0 stale`; `cargo xtask cite audit --diff` → `audited 1 citation
  site(s)` (`[CR#109.2]` on `BareDet`, read against its claim).
- Assurance: restored 0; re-spelled 150 macro call sites + 5 constructor-level witnesses
  (`CountOfGroup`→`CountOf` ×1, `AggregateOf`→`Aggregate` ×2, `ExistsGroup`→`Exists` ×2),
  4 pins re-spelled (`badSingularAggregateOf`→`badSingularAggregate`,
  `badAggregateOfWrongSort`→`badAggregateWrongSort`, `badPowerAmongPlayers`,
  `badLifeTotalAmongObjects` — all four re-probed non-vacuous, each refusing "not a valid
  impossible case" when mis-stated once), 1 card measure re-spelled (`lightmineField`);
  ignored 0; added 1 witness + 1 pin (`badSingularReadOfBarePlural`, probed non-vacuous by
  swapping `Macros.bare` for `Macros.a`); removed 0.
- Deviations and additions: (1) `AggregateOver` left in place — see the STOP; (2)
  `tiedDelta` deleted as redundant with the bare plural's own binding; (3) `countGroups`
  and `countedGroupSize` skip `BareD`; (4) `lightmineField`'s measure re-spelled as a group
  mention; (5) two new gates (`CountableGroup`, `ExistentialMention`) rather than reusing
  `GroupMention`/`CountedExistential`, which exclude `BareD` by construction and would have
  refused every re-spelled site; (6) three convenience macros beyond `bare`, and two
  forwarded auto gates on the pre-existing `nForEach`/`forEach`.
- STOP: the ticket's "Done when" requires `AggregateOver` to be gone, but `AggregateOver`
  is a binder (`op`, domain, *body*), structurally identical to `CompareOver`, which the
  ticket does not touch; its predicate is a lambda domain, not the missing `DetPhrase`
  value, and no noun-taking form supplies the per-element `TheD OneOf` binding its body
  reads through `They`. Deleting it makes three printed cards unspellable, which the
  workbench's representability constraint forbids. Resolution: the row is left as it is and
  the deviation is recorded here; the fold of the other four rows is complete.
