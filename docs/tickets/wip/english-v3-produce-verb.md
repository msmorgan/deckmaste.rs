---
needs: []
---
# Declare the verb *produce*: Add one mana of any type that a land you control could produce.

## Why

*produce* is not a word. On census `qsupksky` (32,828 supported faces, 25,474
covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket
"unspent / produce(d) mana" touches **80** unread faces and is the sole cause
on **60** (touched / sole as in the wave-5 recon; surface counts, not gain
forecasts); **52** of the 60 sole faces fail on *produce(s/d)*, 8 on
*unspent*. `cargo xtask english-v3 grep -i '\bproduc(e|es|ed|ing)\b'` finds
60 faces. Shapes: *any type / color that a land you control could produce*,
*any type that land produced*, *it produces colorless mana instead of …*,
*could produce {C}*, *mana produced by …*, *produces three times as much of
that mana*.

## Goal

*produce* reads as a transitive verb with the Object frames *add* already
has, in finite clauses, in object-gap relatives (*that a land could
produce*) and as a passive participle.

## Analysis

No CGEL citation applies beyond ordinary monotransitive complementation.

Existing route: `Add` (`crates/deckmaste_lexical_source/lexicon/verbs.ron:2–5`,
`[ObjectNounPhrase]` and `[Role("ManaPhrase")]`). Each witness control
substitutes *add* / *adds* / *added* for *produce* / *produces* / *produced*
and admits a Reading with `--readings 1` (a cap, not a complete count), so
the relative-gap, finite and mana-Object routes already
exist; only the lexeme is missing.

Orchestrator rulings 2026-10-09:

- R16a. Declare *produce* (bare, third person, preterite and participle
  *produced*) with Add's two frames; no new Role or Construction. Data only.
- R16b. Pruning rule: only attested forms and frames; *producing* is
  declared only if attested.
- R16c. The *mana produced by …* faces also fail on the *Spend only mana …*
  restriction ("Spend only mana from creatures to cast this spell." 0); do
  not work on that clause here, record how many of those faces read.
- R16d. *produces three times / twice as much of that mana* is a multiplier
  construction; out of scope unless it reads through existing routes.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0 (vocabulary gap
*produce* / *produces* / *produced*), control (the matching form of *add*)
1. (D) = `--category Document`.

- Incubation Druid (D): "{T}: Add one mana of any type that a land you
  control could produce."
- Break the Ice: "Destroy target land that is snow or could produce {C}."
- Dictate of Karametra: "Whenever a player taps a land for mana, that player
  adds one mana of any type that land produced."
- Pale Moon: "Until end of turn, if a player taps a nonbasic land for mana,
  it produces colorless mana instead of any other type."
- Mirari's Wake: "Whenever you tap a land for mana, add one mana of any type
  that land produced."

## Method

Standard constraints apply.
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bproduc(e|es|ed|ing)\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *produce* read with a frame *add* lacks, or *{C}* read as anything but the declared mana Role, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *unspent* (8 sole faces in the same recon bucket; a separate vocabulary
  gap).
- *Spend only mana … to cast this spell* and multiplier *three times as much
  of that mana*.

## Landing record

1. **Census delta** (`kztqlmwwqpxw`): 360-face bounded manifest; covered
   300 → 343, total per-line Readings 10,356 → 13,683. Gained 43; lost,
   decreases and increases 0; digest-only 93; unchanged 224; validation
   issues 0. Lost faces: none. Selector: supported analysed text matching
   case-insensitive `\bproduc(e|es|ed|ing)\b` (60 faces), plus 300 covered
   controls from the stock diff command, seed 20261008. Before is the
   claim-line baseline at `nlvnvwnwovxs`, assembled from full `yntwupzymxxl`
   and its landed *until* subset refresh for trunk `lztruvkmznnx`; no grammar
   or lexical changes intervened before claim `pvpozqvw`. The original
   Produce delta (`zwkyqqyvxool`) gained the same 43 faces with zero losses,
   count decreases or digest-only changes. Refresh `yppxtnmu` corrected
   Sizzling Soloist's lexical-*have* Object-gap / Bare Passive *resolved*
   analysis to a perfect adverbial-gap relative, at unchanged count 1,728
   ([project ruling R1](../done/english-v3-perfect-relative-adverbial-gap.md#landing-record)).
   Refresh `ouzqotlx` brought 124 digest-only changes versus the preceding
   recheck: its added absent cardinal-modifier field, documented in the
   [cardinal landing](../done/english-v3-at-least-cardinal.md#landing-record).
   Neither refresh changed any selected face's coverage or Reading count.
   Stock evidence under `target/english-v3/english-v3-produce-verb/`:
   `manifest.json`, `before.json`, `after.json`, `diff.json`,
   `after-refresh.json`, `refresh-diff.json`, `after-cardinal-refresh.json`,
   `cardinal-refresh-diff.json`, `cardinal-final-diff.json`.
2. **Witnesses**: before implementation (`zwkyqqyvxool`), each ticket passage
   probed at 0; each matching *add* control admitted a Reading with the
   one-Reading cap. Complete after counts, retained by the final suite
   (`kztqlmwwqpxw`): Incubation Druid 2; Break the Ice 1; Dictate of Karametra
   4; Pale Moon 12; Mirari's Wake 4. Existing helpers select the Object-gap
   Relative Clause, Finite Predicate or declared ManaPhrase analysis.
3. **Spot checks** (`kztqlmwwqpxw`): Exotic Orchard, Reflecting Pool,
   Quirion Explorer, Fellwar Stone, Naga Vitalist, Harvester Druid and
   Sylvok Explorer each have 2 Readings; Mana Flare, Heartbeat of Spring and
   Zhur-Taa Ancient each have 4. Selected analysis: Object-gap Relative
   Clause with bare *could produce* or preterite *land produced*, using
   Add's noun-phrase Object frame. One table-driven test checks all ten
   complete passages' counts and constituent spans.
4. **Deviations and STOPs** (`kztqlmwwqpxw`): one native `core.ron` declaration
   instead of `verbs.ron`, whose loader would generate the unattested gerund.
   *Producing* attestation 0; its form is omitted, and Add's two frames are
   copied exactly. No grammar, loader, frame-inventory or glossary changes
   in this ticket; no superseded route or ruling STOP. The extra lexical
   and ten-face tests enforce pruning and spot checks. The inherited
   schema-v7 binary's preliminary bounded baseline was discarded. Seed
   corrected to the user's 20261008. Refresh's test-registration conflict
   was resolved by retaining both modules; grammar/lexical refreshes required
   only bounded rechecks and checks of the two affected crates. R16c's six
   *mana produced by …* faces remain unread (0/6); R16d's three multiplier
   faces remain unread (0/3).
5. **Tests** (`kztqlmwwqpxw`): `cargo test -p deckmaste_english_v3` passed
   (676); `cargo test -p deckmaste_lexical_source` passed (64);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Zero failed/ignored. Earlier lexical clippy found `assert_is_empty` in
   the new test; replaced it with a value comparison and reran the full
   lexical-source suite successfully. Restored 0 / re-spelled 0 /
   ignored-with-blocker 0 / added 7 / removed 0: five witness tests, one
   ten-face test, one exact lexical paradigm/frame test. No removals.
   Citation checks: 0 noncompliant, 0 stale; diff audit selects no new sites.
6. **Timing** (`kztqlmwwqpxw`): final bounded census, 11.74 seconds wall time,
   six workers, zero samples per face, peak RSS 1,384,808 KiB under
   `/usr/bin/time -v`; prebuilt schema-v8 binary, compilation excluded.
