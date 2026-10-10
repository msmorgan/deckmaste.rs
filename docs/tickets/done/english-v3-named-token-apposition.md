---
needs: []
---
# Read a name with an ascriptive NP supplement: create Angelo, a legendary 1/1 green and white Dog creature token.

## Why

A token introduced by its name, with its description set off by a comma, does
not read: "Create Angelo." 1, "Create a legendary 1/1 green and white Dog
creature token named Angelo." 3, but the Oracle wording 0. Blocker report at
trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): hidden under
`at:token .` 6, `at:" <end>` 5, `at:haste .`, `at:indestructible .` and under
`word:` keys. `cargo xtask english-v3 grep "[Cc]reate [A-Z][A-Za-z' ]+, a
legendary"` finds **40** faces, all unread; 40 failing lines match, **33** of
them the only failing line of their face. On **20** of the 33 every word is
known, so the construction is the sole cause; on **13** the line also has an unknown
word, nearly always the name itself (*Voja*, *Mowu*, *Tiny*, *Ragavan*,
*Smaug*, *Avacyn* …).

## Goal

A proper-name NP followed by a comma and an indefinite NP reads as an anchor
with an ascriptive NP Supplement, as the Object of *create*, with whatever
postmodifiers the description NP has today (*with trample and haste*, *with
"…"*), byte-exact.

## Analysis

CGEL Ch. 15 §5.2(b), p. 1357, [19iib]: "*Kim Jones, a quite outstanding
student, won a scholarship to MIT*", labelled [ascriptive]; p. 1358, [22i]:
"*Her father, a die-hard conservative, refused to even consider the
proposal*", under "Ascriptive NP supplements". Those back a proper-name
anchor with an indefinite ascriptive NP supplement. P. 1353–1354: the anchor
and its supplement "form a construction – a supplementation construction".
Restricting it to the positions below is a project ruling. Glossary:
**Supplement**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`ProperNameNounPhrase` (`form [head: Name]`) reads the name; schema
`Supplementation` (`[anchor: node, ", ", supplement: PrepositionPhrase, "."]`,
instances for `Sentence`/`Clause` and `Ability`/`KeywordPhrase`) is the only
comma-supplement schema and takes a PP supplement on a clause anchor.
`NamedNominal` (`[head: Nominal, " ", modifier: NamePredicate]`) reads *token
named Angelo*.

Orchestrator rulings 2026-10-10:

- NA1. One new NP construction, or an NP instance of the supplementation
  schema if its shape allows: anchor `ProperNameNounPhrase`, `", "`,
  supplement an indefinite `NounPhrase`. The result has the anchor's number
  and is an NP wherever the name is.
- NA2. The licence reads declared features only: the anchor is a `Name`, the
  supplement's `Definiteness` is indefinite. No test on *create*, *legendary*
  or *token*. It is therefore not limited to *create*; that is accepted.
- NA3. One analysis per string. The description is a Supplement to the name.
  It must not also read as a second coordinate (asyndetic NP coordination
  *Angelo, a … token*), nor as two Objects. Where a clause-final comma
  supplement and this NP supplement could both attach, report the pair.
- NA4. A medial supplement closed by a second comma (*Kim, a legendary
  creature, attacks*) is unattested here: do not build it, and add no check to
  block it.
- NA5. Names: this ticket adds no name vocabulary. The 13 faces whose token
  name is an unknown word stay unread until the name inventory admits names
  introduced by a face's own text (`english-v3-name-lexeme-scope` is the
  precedent); list them in the landing as routed, not lost.
- NA6. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with a card name followed by a comma and an indefinite NP
  (name-initial lists); report each with its shape. None is expected from the
  40-face pattern, which has no covered face.

- NA7 (orchestrator, 2026-10-10), applying W9-A: “one analysis per
  string” means one route for this Construction. Preserve the independent
  ambiguity of each control formed by removing the name and comma. The
  witness must have exactly the control's Reading count and corresponding
  shapes, including a VP-Adjunct PP if the control has one. The Supplement's
  Postmodifiers attach inside its description NP; supplementation adds no
  outer Postmodifier attachment site. Export the same closed-to-postmodifier
  feature as a bare proper-name NP. Report any remaining extra shapes. Tests
  assert the Supplement role and witness-equals-control relation, not a
  literal count.

## Witnesses

Each is (the last clause of) the face's only failing line at `ymqwzour`, all
words known. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Rinoa Heartilly: "When Rinoa Heartilly enters, create Angelo, a legendary
  1/1 green and white Dog creature token." (sentence "Create Angelo, a
  legendary 1/1 green and white Dog creature token." 0)
- Minsc, Beloved Ranger: "Create Boo, a legendary 1/1 red Hamster creature
  token with trample and haste." (0)
- Stangg: "When Stangg enters, create Stangg Twin, a legendary 3/4 red and
  green Human Warrior creature token." (0)
- Tamiyo, Compleated Sage: "Create Tamiyo's Notebook, a legendary colorless
  Book artifact token." (0; the face's line continues *with "…" and "…"*)
- Tuktuk the Explorer: "When Tuktuk the Explorer dies, create Tuktuk the
  Returned, a legendary 5/5 colorless Goblin Golem artifact creature token."
  (not probed: the binary stopped loading plugin data when trunk moved)

Controls that must not change: "Create Angelo." 1; "Create a legendary 1/1
green and white Dog creature token named Angelo." 3; "Create a legendary 2/2
white and black Dog creature token." 1; "Create a token named Stangg Twin."
1; "Sacrifice Stangg when that token leaves the battlefield." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `[A-Z][A-Za-z' ]+, an? [a-z0-9]` (a capitalised word, comma, indefinite
article: the surface a name supplement can touch)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
NA6 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the name as anchor and head of the Object NP, the description as its
Supplement, the NA3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Token-name vocabulary (NA5), 13 sole-line faces.
- *Ragavan enters tapped and attacking.* and other later sentences that use
  the introduced name.
- Quoted abilities inside the description that fail for their own reason
  (*and "{T}: Draw a card."*).

## Claim-parent measurements

Re-probed on the claim-parent tree `tlpwnxqn` (parent `lznsqwuy`): all five
witnesses have 0 Readings, with complete enumeration. The five controls have
1 / 6 / 1 / 1 / 1 Readings, respectively; the earlier 3 for the named-Angelo
control was the probe's enumeration cap. The bounded selector is the Method's
pattern plus 300 covered controls chosen with seed 20261008 (user direction,
superseding the Method's seed): 77 surface faces, 377 faces in the union.
The claim-parent census has 323 covered faces and 6,784 Readings; no issues.

## STOP resolution

The NA3 attachment STOP was resolved by NA7 / W9-A (orchestrator,
2026-10-10): independent control ambiguity stays; one route does not mean a
unique Reading. No keyword-PP licence is retired. The supplemented NP already
exports the proper-name anchor's `ComparativePostmodifierUse = No`, closing
the outer NP to the existing NP Postmodifier route; nominal Postmodifiers
remain inside the indefinite Supplement.

On `tlpwnxqn`, each name-and-comma-removed control completely enumerates
1 / 7 / 1 / 1 / 1 Readings (Rinoa / Minsc / Stangg / Tamiyo / Tuktuk), exactly
matching its witness. The four single-Reading controls have the full token
description as Object NP. Boo's control has six *with trample and haste*
Postmodifier heights inside the Object Nominal: *legendary 1/1 red Hamster
creature token*, *1/1 red Hamster creature token*, *red Hamster creature token*,
*Hamster creature token*, *creature token*, and *token*. Its seventh Reading
attaches the PP to *create* as a VP Adjunct. No extra shape remains. These are
control measurements under NA7 / W9-A, not a CGEL ruling on the scopes.

## Landing record

1. **Census delta** (before `tlpwnxqn`, claim-parent tree `lznsqwuy`;
   final refreshed after `strszsqr`): bounded
   manifest 377 faces (77 matching `[A-Z][A-Za-z' ]+, an? [a-z0-9]` plus
   300 random covered faces, seed 20261008). Covered 323 → 342; Readings
   6,784 → 8,329. Built-in `diff --lines`: gained 19, lost none, decreases 0,
   increases on previously covered faces 0, digest-only 0, unchanged 358.
   Evidence: `target/english-v3/english-v3-named-token-apposition/`;
   `before.json`, `after-integrate.json`, `manifest.json`, `diff-integrate.json`.
   Refresh brought `roomopmr` phase-in/out data and tests; the built-in
   `diff-incoming.json` comparison to pre-refresh `tlpwnxqn` reports all
   377 faces unchanged, 8,329 → 8,329 Readings.
2. **Witnesses** (`tlpwnxqn`): Rinoa Heartilly 1; Minsc, Beloved Ranger 7;
   Stangg 1; Tamiyo's Notebook fragment 1; Tuktuk the Explorer 1. Each was 0
   before implementation. NA7 controls removing the name and comma have
   exactly the same counts and corresponding shapes; Boo's six nominal PP
   heights and one VP Adjunct are inherited, with no extra attachment site.
   Original controls remain 1 / 6 / 1 / 1 / 1.
3. **Spot checks** (`tlpwnxqn`): Dark Depths → Marit Lage; Falcon, Winged
   Wonder → Redwing; Helm of Kaldra → Kaldra; Mabel, Heir to Cragflame →
   Cragflame; Marit Lage's Slumber → Marit Lage; Rinoa Heartilly → Angelo;
   Rufus Shinra → Darkstar; Stangg → Stangg Twin; Tomb of Urami → Urami;
   Tuktuk the Explorer → Tuktuk the Returned. Each selected Reading has one
   name-anchored Object NP with an indefinite description NP in its Supplement
   role. No wrong-analysis gain found in these ten checks.
4. **Deviations and STOPs**: one new `AscriptiveNounPhraseSupplement`
   Construction and declared `NounPhraseStructure` feature; no retired route,
   new frame, or name vocabulary. Glossary gap filled with **Noun Phrase
   Structure** under NA1–NA2. Attachment STOP resolved by NA7 / W9-A:
   preserve control ambiguity and close the supplemented NP to outer
   Postmodifiers through its anchor's declared feature. Four tests cover roles,
   control equivalence, licensing negatives, and both roundtrip laws.
   NA5 name-inventory residuals remain routed, not lost: Battle at the Helvault;
   Tolsimir, Midnight's Light; Venser, Corpse Puppet; Drizzt Do'Urden;
   Tolsimir Wolfblood; Baral and Kari Zev; Jiang Yanggu, Never Alone;
   Kiora, the Rising Tide; Kari Zev, Skyship Raider; Volo, Itinerant Scholar;
   Setzer, Wandering Gambler; Jiang Yanggu; Soundwave, Superior Captain;
   Verix Bladewing; The Spear of Leonidas; Inquisitor Eisenhorn;
   There and Back Again; The Coming of Galactus; White Tiger, Ava Ayala;
   Sophia, Dogged Detective; Bonny Pall, Clearcutter. This current
   `tlpwnxqn` list is from the ticket's 40-face grep selector, rather than the
   older 13 sole-line blockers; some have other blockers too. Name admission
   follows the `english-v3-name-lexeme-scope` precedent, outside NA5's scope.
5. **Tests** (`tlpwnxqn`): `cargo test -p deckmaste_english_v3` passed
   all 900 tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. Earlier full/module runs exposed test expectations corrected under
   NA7; `cargo test -p deckmaste_english_v3 --test suite_5 named_token_apposition::`
   was the diagnostic module run. Final full suite includes all four new
   tests. Restored 0 / re-spelled 0 / ignored 0 / added 4 / removed 0.
   Citation checks: 0 noncompliant, 0 stale; diff audit selects no new CR sites.
   After refresh, on `volqukqn`: `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source` passed
   all 905 English tests and 84 lexical-source tests;
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`
   passed. The five incoming English tests belong to `roomopmr`; this ticket
   still adds four. Accepted review followed by the final refresh (`strszsqr`)
   brought cause-catenative data and tests. The same build, full-suite and
   clippy commands passed again: 909 English tests and 84 lexical-source
   tests. Final bounded diff remains 19 gains and zero other changes.
6. **Timing** (`strszsqr`, final refresh): bounded landing census wall time **45.98 s**,
   **6 workers**, under `/usr/bin/time -v`; no full-corpus census run,
   per the binding verification budget.
