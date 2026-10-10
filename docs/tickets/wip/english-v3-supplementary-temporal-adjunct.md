---
needs: []
---
# Read a comma-set *before* supplement after an Adjunct: Activate only during your turn, before attackers are declared.

## Why

The Portal Three Kingdoms timing line fails at the end of the document (key
`at:. <end>`, 26 of its 31 sole-blocker faces). Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.json`). `cargo xtask english-v3
grep ', before (attackers|blockers) are declared'` finds **28** faces, all
unread; 28 failing lines match, **27** of them the only failing line of their
face. All 28 are *Activate only during your turn, before attackers are
declared.* Without the comma the same words read: "Activate only during your
turn before attackers are declared." 1; so do the parts: "Activate only
during your turn." 1, "Activate only before attackers are declared." 1.

## Goal

*, before attackers are declared* reads as a Supplement to the clause
*Activate only during your turn*, with byte-exact realisation including the
comma.

## Analysis

CGEL Ch. 15 §5.2(h), p. 1360, heading "PPs and AdvPs", [30ii]: "*In my
opinion, the idea isn't worth pursuing*": a PP as supplement to a clause
anchor. §5, p. 1350: supplements are "elements which occupy a position in
linear sequence without being integrated into the syntactic structure of the
sentence". Which Prepositions
Oracle English licenses as supplements is a project ruling. Glossary:
**Supplement**, **Preposition Function Licence**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`Supplementation` (`form [anchor: node, ", ", supplement: PrepositionPhrase,
"."]; require licence_supplement(supplement.PrepositionFunctionLicence) =
Yes`), instance `(Sentence, Clause, CompleteSentence)`. In `lexicon/core.ron`
only *where* declares `PrepositionFunctionLicence: "Supplement"`; *before* has
`"Adjunct|PreposedAdjunct"` with `FiniteClauseComplement: Yes`.

Orchestrator rulings 2026-10-10:

- ST1. Data ticket: add `Supplement` to *before*'s
  `PrepositionFunctionLicence`. No new Construction; `Supplementation` reads
  it. If the schema then still fails (for instance because the anchor carries
  a focused Adjunct), correct that in the schema and report it as a deviation.
- ST2. The licence is the declared function licence. Nothing names *before*,
  *attackers* or the anchor's verb. Other Prepositions get the licence only
  with their own attested witness; none is added here.
- ST3. One analysis per string. With the comma, the *before* PP is a
  Supplement to the clause. It must not also read as an integrated Adjunct
  across the comma, nor as a second focus of *only* (the focus of *only* is
  *during your turn*). Without the comma, "Activate only during your turn
  before attackers are declared." keeps exactly 1.
- ST4. A supplementary *before* PP is licensed sentence-finally only (the
  schema's shape). Medial use is `MedialSupplementation`'s business and is
  unattested for *before*.
- ST5. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with *, before* + clause or NP (*…, before damage is dealt*,
  *, before the beginning of …*): each is reported with its shape, and is
  legitimate only if the comma previously read through some other route that
  ST3 would call a wrong analysis; otherwise it is a duplicate and a defect.

## Witnesses

Each is the last sentence of the face's only failing line at `ymqwzour`; the
sentence is identical on all 28 faces. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first.

- Talas Researcher, King's Assassin, Stern Marshal, Lady Sun, Cao Cao, Lord
  of Wei: "Activate only during your turn, before attackers are declared." (0)

Also 0 today (unattested variants, for localisation only): "Activate only
during your turn, before blockers are declared."; "Activate only during
combat, before blockers are declared." Controls that must not change:
"Activate only during your turn." 1; "Activate only before attackers are
declared." 1; "Activate only during your turn before attackers are declared."
1; "Cast this spell only during combat before blockers are declared." 1;
"Cast this spell only before attackers are declared." 1; "Attackers are
declared." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bbefore\b` case-insensitively (the lexeme whose licence widens)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
ST5 for increases. Full `cargo test -p deckmaste_lexical_source` and `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. Lexicon and plugin `grammar:` data load at runtime, so a data-only change
needs no rebuild; a grammar edit costs 2–3 minutes and the six test binaries
about 75 seconds.
Tests assert the role (the *before* PP as Supplement to the clause anchor, *only* focusing
the *during* PP alone, the ST3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *Activate only during your turn, and only once each turn* (unattested with
  a comma; `english-v3-coordinated-focused-adjuncts`).
- The other five `at:. <end>` sole faces: *where X is …* closing a chapter
  line (The Weatherseed Treaty), *Cumulative upkeep—An opponent
  gains 1 life.* (Wall of Shards), *Equip—Sacrifice another nonland
  permanent. Activate only once each turn.* (2), a lower-case sentence start
  after a colon (Necratog).

## Landing record

1. **Census delta** — Claim-parent grammar (`ulwqwoxx`; BEFORE census stamped
   `mpvousxzotns`) → `mpvousxzotns`: covered faces **1,225 → 1,251**, total
   Readings **41,330 → 41,367**. **26 gains; zero lost, decreased, increased or
   digest-only faces; zero wrong-analysis gains**. Bounded selector: all 62
   supported faces matching `\bbefore\b` and all 1,119 matching `\bwhere\b`,
   case-insensitively, plus 300 covered controls selected by the official diff
   manifest with seed 20261008; overlaps deduplicated to **1,472 faces**.
   Evidence: `target/english-v3/english-v3-supplementary-temporal-adjunct/`
   (`manifest-final.json`, `before-final.json`, `after.json`, `diff.json`).
2. **Witnesses (`mpvousxzotns`)** — Talas Researcher, King's Assassin, Stern
   Marshal, Lady Sun, and Cao Cao, Lord of Wei: each sentence **0 → 1**.
   Both localization variants also **0 → 1**. All six controls remain **1**.
   No witness was already covered on the claim parent.
3. **Spot checks (`mpvousxzotns`)** — Alaborn Veteran; Apprentice Sorcerer;
   Cao Cao, Lord of Wei; Capricious Sorcerer; Coastal Wizard; Cunning Advisor;
   Diaochan, Artful Beauty; Fire Bowman; Goblin Firestarter; Hua Tuo, Honored
   Physician. Every inspected whole-face Reading places the final *before*
   PP as a Supplement to the imperative Clause, with *only* focusing *during
   your turn* alone. Other ability ambiguities remain intact.
4. **Deviations and STOPs** — ST4 required a schema correction beyond ST1's
   data-only route: `MedialSupplementation` now reads a separate
   `MedialSupplement` member of the declared function licence; *where* retains
   both supplementary licences, *before* only the sentence-final licence.
   No Construction or Frame added or deleted. The existing licence glossary
   entry now records ST4; the selector includes *where* to verify preservation.
   The seed follows the user's 20261008 direction. The medial localization test
   rejects the medial attachment while preserving the valid terminal
   Supplement whose finite Complement contains coordinated predicates; its
   initial blanket no-Reading assertion was too broad. No STOPs or unresolved
   glossary gaps.
5. **Tests (`mpvousxzotns`)** — `cargo test -p deckmaste_english_v3`: initial
   run failed the medial negative; after the correction, full suites passed
   twice (900 tests each), including the final independently constructed
   negative fixture. `cargo test -p deckmaste_lexical_source`: passed (84).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: passed.
   `cargo build -p xtask`: passed. Existing tests restored / re-spelled /
   ignored / removed: **0 / 0 / 0 / 0**; added: **4**, registered once in
   `tests/main.rs`; no removals to justify. Citation checks: zero noncompliant,
   zero stale; diff audit selected zero sites, with no new CR citations.
6. **Timing (`mpvousxzotns`)** — Final complete bounded AFTER census:
   **97.57 seconds**, **6 workers**, `--samples-per-face 0`, measured by
   `/usr/bin/time -v`. No full-corpus census was run.
