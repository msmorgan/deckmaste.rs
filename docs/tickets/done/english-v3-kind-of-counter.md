---
needs: []
---
# Read *kind of* + bare noun: Double the number of each kind of counter on target creature.

## Why

*kind* reads with a demonstrative and no Complement ("Put a counter of that
kind on each other creature you control." 2; "Put a counter of each kind on
it." 1) but not with its *of* Complement. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): hidden under `at:creature .` 3,
`at:control .` 5, `at:permanent .`, `at:it .`. `cargo xtask english-v3 grep
'kind of counter'` finds **27** faces, **25** unread; 25 failing lines match,
**21** of them the only failing line of their face. Sole lines by shape: *double the number of each kind of counter on …* 9; *for
each kind of counter on / among …* 9; *choose a kind of counter …* 2; *the
number and kind of counters* 1. The 2 covered faces are *the same number and
kind of counters* (Captain Marvel, Apex Avenger; Bold Plagiarist), a
coordination whose plural Complement is an ordinary NP.

## Goal

*kind of* + a bare singular count noun reads as an NP headed by *kind* with
an *of* Complement (*each kind of counter*, *a kind of counter*), taking the
postmodifiers a counter NP takes (*on target creature*, *among permanents you
control*).

## Analysis

No CGEL passage is relied on: the bare singular Complement of *kind of* is
treated here as a project ruling. Glossary: **Bare Noun Phrase**,
**Countability**.

Existing declarations: `lexicon/core.ron`, `lexeme:CommonNoun/Kind`
(`countability: [Count]`, `features: {}`, no frames). *of* PPs take an NP;
*counter* is a count noun and has no bare singular NP. `declarations.rs` has
the bare-Complement licences `BareNominalComplement { No, Location,
SpatialEdge }` on Prepositions and `NominalBareClass` on nouns, and
`AttributePremodifierHead` on *type*; none covers a taxonomic noun.

Orchestrator rulings 2026-10-10:

- KC1. The licence is a declared feature of the head noun *kind* (a
  taxonomic-Complement permission), read by the nominal *of*-Complement
  route, which then admits a bare singular count Nominal as *of*'s Complement.
  Refine that route; no free-standing "kind of" construction and no multiword
  lexeme *kind of*.
- KC2. No test on *kind* or *counter*. *type(s) of counter* is unattested (0
  faces): do not give *type* the feature.
- KC3. One analysis per string. In *each kind of counter on target creature*
  the *on* PP has the attachments it has in *each counter on target creature*
  under the new head, no more; *counter* must not also read as a verb
  (*counter on* …) or head a separate NP.
- KC4. *the number and kind of counters that were on that creature* (Tawnos's
  Coffin): the shape that already reads on the 2 covered faces; its blocker is
  elsewhere. Out of scope.
- KC5. Newly covered faces are gains. Reading-count increases are possible on
  the 2 covered `kind of counter` faces and on covered faces with *of that
  kind*; report each with its shape. None is expected elsewhere.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Ferrafor, Young Yew: "{T}: Double the number of each kind of counter on
  target creature." (sentence 0; *the number of counters on target creature*
  3)
- Contractual Safeguard: "Choose a kind of counter on a creature you
  control." (0; "Choose a kind of counter." 0)
- Ultimate Spider-Man: "Whenever you attack, double the number of each kind
  of counter on each Spider and legendary creature you control." (0)
- Gilder Bairn, Vorel of the Hull Clade: *Double the number of each kind of
  counter on target permanent / target artifact, creature, or land.* (not
  probed separately)

Controls that must not change: "Put a counter of that kind on each other
creature you control." 2; "Put a counter of each kind on it." 1; "Remove a
counter of each kind from it." 1; "Double the number of counters on target
creature." 3; "Double the number of +1/+1 counters on each creature you
control." 3.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bkinds?\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
KC5 for increases. Full `cargo test -p deckmaste_lexical_source` and `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (*kind* as head, the bare Nominal as its *of* Complement, the KC3
negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *a counter of each kind not on this creature* (0 today): *not* + PP
  postmodifier.
- *give that permanent or player another counter of that kind*, *put another
  counter of that kind on it or remove one from it*: second causes on the
  *for each kind of counter* lines (fused *one*, *give* frame); those faces
  may stay unread.
- *your choice of a … counter* (T-45 Power Armor).

## Landing record

1. **Census delta** (`wkooyuko`, before/after on claim parent `yllkynot`):
   bounded manifest of 344 faces: all 45 supported `\bkinds?\b` faces plus
   300 random covered controls, seed 20261008 (one overlap). Covered
   306 → 321; Readings 6,338 → 7,064; gained 15; lost / decreases /
   previously-covered increases / digest-only / issues: all zero. Lost faces:
   none. No wrong-analysis-only gains found. Evidence:
   `target/english-v3/english-v3-kind-of-counter/{manifest,before,after,diff}.json`;
   comparison uses `cargo xtask english-v3 diff --lines`.
2. **Witnesses** (`wkooyuko`): complete direct probes before → after:
   Ferrafor, Young Yew activation 0 → 3; Contractual Safeguard sentence
   0 → 3 (bare fragment 0 → 1); Ultimate Spider-Man trigger 0 → 22;
   Gilder Bairn instruction 0 → 3; Vorel of the Hull Clade instruction 0 → 6.
   None already read on the claim parent, so none dropped. Controls retain
   2 / 1 / 1 / 3 / 6 Readings in ticket order; the final control's original
   reported 3 was capped, whereas these probes enumerated all 6.
3. **Spot checks** (`wkooyuko`): ten newly covered faces' selected Readings
   all have noun head *kind*, marker *of*, and bare singular count Nominal
   *counter*, with Postmodifiers outside that Complement: Arcade Cabinet
   (double/number); Blitzball Stadium (quoted draw/for-each); Bribe Taker
   (preposed for-each); Contractual Safeguard (choose/Object); Deepglow Skate
   (triggered double/number); Ferrafor, Young Yew (activated double/number);
   Gilder Bairn (activated double/number); Maulfist Revolutionary
   (preposed for-each); Powerful Broker (activated for-each); Skyship
   Plunderer (triggered for-each). All ten sampled documents realize exactly;
   no unintended replacement analysis was observed.
4. **Deviations and STOPs** (`wkooyuko`): no new Construction or Frame and
   no superseded route to retire. Refined `PrepositionComplementNominal`
   with a guarded bare-Nominal form, preserving its existing NP form and
   the ordinary plural *kind of counters* analyses. KC3 requires an
   unmodified bare Complement so ordinary Postmodifiers attach under the
   new head without a duplicate inner attachment. Glossary gap filled:
   **Taxonomic Complement Use**, citing project rulings KC1–KC3 rather than
   CGEL or a game rule. No STOPs. Used the user's binding seed 20261008
   instead of the Method's 20261010.
5. **Tests** (`wkooyuko`): `cargo test -p deckmaste_english_v3` passed
   (901 tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed; `cargo test -p deckmaste_lexical_source` passed (84 tests);
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Assurance: restored 0 / re-spelled 0 / ignored 0 / added 5 / removed 0.
   The five role/attachment/licence/control/independent-roundtrip tests are
   registered once in `suite_5`. No tests were removed or weakened.
6. **Timing** (`wkooyuko`): bounded AFTER census 22.82 s wall time,
   6 workers, measured with `/usr/bin/time -v`; every selected face was
   fully enumerated with `--samples-per-face 0`. No full-corpus census run.
