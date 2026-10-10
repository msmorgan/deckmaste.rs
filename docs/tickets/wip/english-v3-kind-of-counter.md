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
