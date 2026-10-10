---
needs: []
---
# Read a perfect auxiliary in a bare object relative: X is the number of spells you've cast this turn.

## Why

*you've cast a spell* reads; *spells you've cast* does not. The 2026-10-10
triage (`docs/memory/english-v3-blocker-triage-2026-10-10.md`, blocker report
at trunk `vwlkmvns`) ranks this sixth, about 70 sole-blocker faces.
`cargo xtask english-v3 grep "\b(you|they)'ve\b"` finds **218** faces; about
140 are full clauses (*if you've cast …*), which read in the controls below. A noun directly before
the clitic (`\b(spells?|cards?|lands?|creatures?|permanents?|life|…) (you|they)'ve\b`)
has **59** faces. 76 faces have a single failing line containing a word +
*you've* / *they've*; 13 of those are *for each time you've cast your
commander …*, a different gap.

The triage read this as a clitic problem. It is wider: there is no perfect
object gap at all. "the life you have lost this turn" has exactly 1 Reading,
and it is `ZeroObjectRelativeClause` over lexical *have* (*the life you
have*) with *lost this turn* as a passive participial postmodifier. The strong
form is covered only through that analysis; the clitic
(`core-verb:HaveContracted`, `PredicateFrameUse: No`, perfect auxiliary frame
only) cannot take it, so the contracted form reads 0. "… spells your opponents
have cast this turn" reads the same way (3) and is not part of the gap.

## Goal

A finite perfect auxiliary, strong or clitic, heads the Predicate of an
object-gap relative (*spells you've cast this turn*, *the life you've lost
this turn*, *each spell they've cast this turn*), with byte-exact
realisation.

## Analysis

CGEL Ch. 18 §6.2(a), p. 1615, [6]: the clitic forms of *am*, *are*, *have*
and *will* "attach only to a preceding subject pronoun" (*they've*). Ch. 12
§2.1, p. 1034: non-*wh* relatives are "subdivided into ***that*** relatives
and bare relatives depending on the presence or absence of *that*". Those back
the host restriction and the bare relative only.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`FiniteObjectGap` (`[head: lexical(Verb)]`, `head.frame = Transitive`,
`PredicateFrameUse = Yes`) and `AuxiliaryObjectGap` (`[head: lexical(Verb),
complement: BareObjectGap]`, `head.frame = BareAuxiliary`: *can cast*), both
Category `FiniteObjectGap`, which carries `CliticHost`.
`ObjectRelativeClause`, `ZeroObjectRelativeClause` and `FiniteGappedClause`
already place `clitic_separator(predicate.CliticHost)` and require
`clitic_subject(…)`. `AdverbialGapRelativeClause` takes a whole
`FiniteClause` and already reads *the first time you've cast your commander*
(3). `PerfectComplement` / `PerfectAuxiliaryPredicate` are the gap-free
perfect.

Orchestrator rulings 2026-10-10:

- CH1. Refine `AuxiliaryObjectGap` (a second form, or a schema with a bare and
  a perfect instance) so that a perfect auxiliary takes a past-participle
  object gap. No new relative-clause Construction; the three clause routes
  above consume the result unchanged.
- CH2. Licences read declared features: the auxiliary's declared perfect
  frame, the participle's form and its transitive frame. The host restriction
  stays where it is (`clitic_host` / `clitic_subject`); no word test, and no
  route specific to the clitic.
- CH3. The strong form gains the same analysis: *spells you have cast this
  turn* gets the perfect-gap Reading beside the existing lexical-*have* +
  participial-postmodifier Reading. Both are grammatical and both stay
  (retained-Reading contract). Do not prune the old one here.
- CH4. One analysis per string for the new part: the clitic form reads only
  through the perfect gap (the clitic has no lexical-*have* use), and the
  perfect gap is not also reachable through `AdverbialGapRelativeClause` on a
  head that declares no `RelativisableAdjunctClass`.
- CH5. Coordinated participles under one auxiliary (*each card you've cycled
  or discarded this turn*, Hollow One) are in scope only through the existing
  object-gap Coordination series; list them if they need more.
- CH6. Newly covered faces are gains. CH3 predicts increases on covered faces
  with strong *have* / *has* / *had* + participle in a bare relative: report
  each with its shape before landing; anything other than the one added
  perfect-gap analysis per relative is a defect.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--readings 3 --as-card`): witness 0; control = the same
unit with *you have* / *they have* (count shown; 3 means at least 3). Trunk
has since taken `nvotwtqp`; re-probe on the claim parent first.

- Aetherflux Reservoir: "Whenever you cast a spell, you gain 1 life for each
  spell you've cast this turn." (control 3)
- Kydele, Chosen of Kruphix (document): "{T}: Add {C} for each card you've
  drawn this turn." (control 2)
- Greven, Predator Captain: "Greven gets +X/+0, where X is the amount of life
  you've lost this turn." (control 3)
- Knowledge Is Power: "Creatures you control get +X/+X, where X is the number
  of cards you've drawn this turn." (control 3)
- Rug of Smothering: "Whenever a player casts a spell, they lose 1 life for
  each spell they've cast this turn." (control 2)

Controls that must not change: "Draw a card if you've cast a spell this
turn." 2; "You've cast a spell this turn." 2; NP "the first time you've cast
your commander" 3; NP "the second spell you cast this turn" 3; "X is the
number of spells your opponents have cast this turn." 3 (CH6: expected to
rise).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    ('ve|\b(have|has|had)) [a-z]+(ed|n|t)\b

(766 faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; CH6 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3`, once, at the end: a rebuild after
a grammar edit takes about 2–3 minutes and the test binary about 7, so batch
edits. Tests assert the role (the relative's Subject, the perfect auxiliary
with its lexeme and Clitic Host, the participle with its Object gap; for one
strong-form control, both Readings by structure), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *for each time you've cast your commander from the command zone this game*
  (13 lines): *for each time* + an Adjunct-gap relative.
- *creature … that you've put one or more +1/+1 counters on this turn* (Kid
  Loki): a stranded-preposition gap.
- *a spell you've cast is countered* (Multani's Presence) unless it falls out.

## Landing record

1. **Census delta** — BEFORE: the pre-refresh claim-parent `myrrznxk` tree
   at then-empty `pnrkopro`; initial AFTER: implemented `pnrkopro`; refreshed
   AFTER: then-empty `ovwmnzms` atop refreshed `pnrkopro`. Bounded manifest:
   851 surface faces (ticket grep `('ve|\b(have|has|had)) [a-z]+(ed|n|t)\b`, unioned with
   strong/clitic *have* followed by every declared past-participle spelling)
   plus 300 covered controls sampled by `english-v3 diff`, seed 20261008;
   five overlaps, 1,146 distinct faces. Initial covered 889 → 937; total
   Readings 37,074 → 41,804. After refresh: covered 889 → 942; Readings
   37,074 → 42,043. Final lost none; gained 53; decreases 0; increases 6;
   digest-only 0; internal issues 0. The initial 48 gains are this change;
   five additional gains (239 Readings) came from vocabulary batch 2,
   `nqnztuzo`: Fire Navy Trebuchet; Frontier Siege; Gor Muldrak,
   Amphinologist; Mordor Trebuchet; Scriv, the Obligator. The
   refresh-only diff has no losses, decreases, increases or digest-only
   changes. Rebuilt selector instrumentation finds no additional surface
   faces after refresh. All six increases below are unchanged by refresh
   and instantiate the one perfect-gap analysis per relative licensed by
   CH3/CH6, alongside retained analyses (other independent ambiguities
   multiply the line totals):

   Final accepted-review refresh, at then-empty `nzkyxpym` atop `nulkxpsk`:
   covered 889 → 944; Readings 37,074 → 42,095. Lost none; gained 55;
   decreases 0; increases 6; digest-only 0; internal issues 0. The two
   additional inherited gains (52 Readings), Jeska, Thrice Reborn;
   Uncivil Unrest, come from multiplier-predeterminer `qoswqlns`.
   Own gains remain 48; inherited gains total seven. The final refresh-only
   diff leaves every previously covered face and all six increases unchanged.

   | Face | Face Readings before → after | Added Reading shape |
   | --- | --- | --- |
   | Abaddon the Despoiler | 181 → 281 | *life your opponents have lost*: Subject *your opponents*, strong perfect *have*, transitive *lost* with Object gap |
   | Delightful Discovery | 7 → 12 | *spell your opponents have cast*: Subject *your opponents*, strong perfect *have*, transitive *cast* with Object gap |
   | Heliod, the Warped Eclipse | 22 → 42 | *card your opponents have drawn*: Subject *your opponents*, strong perfect *have*, transitive *drawn* with Object gap |
   | Molten Psyche | 229 → 349 | *cards that player has drawn*: Subject NP *that player*, strong perfect *has*, transitive *drawn* with Object gap |
   | Moraug, Fury of Akoum | 14 → 16 | *time it has attacked*: Subject *it*, strong perfect *has*, transitive *attacked* with Object gap; existing temporal-gap readings retained |
   | Thought Sponge | 146 → 210 | *cards an opponent has drawn*: Subject NP *an opponent*, strong perfect *has*, transitive *drawn* with Object gap |

2. **Witnesses** — Complete ticket-unit probes at the pre-refresh claim-parent tree
   (`myrrznxk`, empty `pnrkopro`) → implemented `pnrkopro`: Aetherflux
   Reservoir 0 → 4 (strong control 4 → 8); Kydele, Chosen of Kruphix
   0 → 2 (2 → 4); Greven, Predator Captain 0 → 4 (6 → 10);
   Knowledge Is Power 0 → 4 (6 → 10); Rug of Smothering 0 → 2 (2 → 4).
   No already-covered witness was retained as a failure. Controls:
   conditional perfect 2 → 2; finite perfect 2 → 2; *the first time you've
   cast your commander* 8 → 8; *the second spell you cast this turn* 8 → 8;
   opponents' strong-perfect sentence 6 → 10, expected by CH3/CH6.
   Hollow One's coordinated constituent 0 → 3. The refresh-only census
   diff changes none of these witness/control faces.

3. **Spot checks** — Newly covered faces on `pnrkopro`, each inspected with
   complete probes, exact realization and no issues: Aetherflux Reservoir
   (*spell you've cast*), Kydele (*card you've drawn*), Greven (*life you've
   lost*), Knowledge Is Power (*cards you've drawn*), Rug of Smothering
   (*spell they've cast*), Hollow One (*card you've cycled or discarded*),
   Children of Korlis (*life you've lost*), Fists of Flame (*card you've
   drawn*), Multani's Presence (*a spell you've cast*, modifying the matrix
   Subject), Voracious Wurm (*life you've gained*). Each selected Reading
   has a bare Object relative with its overt Subject and perfect auxiliary
   selecting a transitive past-participle Object gap; Hollow One coordinates
   the gaps under one auxiliary. The outer clauses and their other
   grammatical ambiguities remain intact.

4. **Deviations and STOPs** — `pnrkopro`: add the supporting
   `PastParticipleObjectGap` Construction/Category and its series Category;
   a transitive past-participle gap is distinct from both a plain gap and a
   Bare Passive. Instantiate the four existing ordinary Coordination schemas
   for that Category, as CH5 permits. Refine `AuxiliaryObjectGap` with a
   perfect second form under CH1–CH2. No relative-clause Construction, frame,
   lexical declaration, loader change or superseded route; CH3 explicitly
   retains the lexical-*have* route. Selector expanded by 85 attested faces
   to cover all declared participles and the *'s*/*'d* clitic forms; checks
   use `english-v3 grep`. Multani's Presence falls out without extra work,
   as permitted by the ticket. Five structural tests added. The initial
   Hollow One test wrongly required the shared perfect in every Reading;
   corrected it to assert both the
   shared perfect and the independently licensed finite coordination with
   preterite *discarded*, checking their lexical identities and frames.
   No grammar pruning. No glossary gap or unresolved STOP. Commit followed
   by successful `kata refresh`; bounded re-census and full checks follow
   the refreshed lexical/core changes. Standing user-directed STOP before
   integration honored; orchestrator review ACCEPTED, with integration
   and workspace drop authorized.
   Residual, not addressed: the perfect has no past-participle counterpart of
   `SelectedGapLocativePredicate`, so *each +1/+1 counter you've put on
   creatures …* (Iridescent Hornbeetle; `(you|they)'ve put` 4 faces) still
   reads 0 while preterite *you put on …* and *you can put on …* read.
   Follow-up ticket, not this landing.

5. **Tests** — `pnrkopro`: `cargo test -p deckmaste_english_v3`
   initially 752 passed / 1 new-test failure (the Hollow One assumption
   above); the corrected FULL run passed all 753, zero failed/ignored.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   After refresh, then-empty `ovwmnzms` atop `pnrkopro`:
   `cargo test -p deckmaste_english_v3` passed all 754;
   `cargo test -p deckmaste_lexical_source` passed 39 unit and 41 integration tests;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   `cargo test -p xtask english_v3` passed all 56 selected tests, zero
   failed/ignored (required because refresh changed construction core).
   Final accepted-review refresh, then-empty `nzkyxpym` atop `nulkxpsk`:
   `cargo test -p deckmaste_english_v3` passed all 764;
   `cargo test -p deckmaste_lexical_source` passed 39 unit and 41 integration
   tests; both full suites have zero failed/ignored.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Citation checks: no noncompliant strings or stale rules; no new CR
   registrations. Assurance counts: restored 0 / re-spelled 0 /
   ignored-with-blocker 0 / added 5 / removed 0; no removals.

6. **Timing** — `pnrkopro`: bounded AFTER census 49.87 s wall time under
   `/usr/bin/time -v`, six workers, zero samples per face. No `--all` run.
