---
needs: []
---
# Read restrictive *only* focusing a Noun Phrase: This creature can block only creatures with flying.

## Why

Focusing *only* reads before a PP (*only during your turn*), an Adverb
(*only once*) and a purpose infinitival, but not before an NP. Blocker report
at trunk `uoqyuxtn` (`target/english-v3/blockers-wave7.txt`): keys
`at:only creatures` 44 and `at:only a` 13, plus `at:reduces only`,
`at:only this`, `at:has only`.
`cargo xtask english-v3 grep '\b[a-z]+ only (a|an|one|the|this|that|it|creatures|cards|permanents|[A-Z][a-z]+)\b'`
finds **89** faces, all unread; **84** have that line as their only failing
line. By verb, sole-line faces: *block only creatures with …* 41; *targets
only a single … / this creature / it / <name>* 23; *reduces only the amount
of …*, *has only one target*, *contains only cards …*, *attack only the
nearest …* 15; *spend only <colour> mana …* 8.

The 84 is an upper bound: "Spend black mana on X." reads 0 without *only*,
so the 8 *spend only* lines have a second cause.

## Goal

*only* reads as a restrictive focusing Modifier of the Noun Phrase it
precedes, where that NP is the Object or a Complement of a verb, with
byte-exact realisation.

## Analysis

CGEL Ch. 6 §7.3.1, p. 587, [41]: *only* is among the "Adverbs that function
as focusing modifiers of the restrictive type"; [42i]: "*He loves* [*only his
work*]" labelled [NP], under "they modify a wide range of constructions".
P. 589, [48ii]: "*Only Kim preferred the ORIGINAL version*", "the focus of
*only* is *Kim*". Those back *only* + NP as a constituent, in Object and in
Subject function. Which positions Oracle English licenses is a project
ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`FocusedPhrase` (`[modifier: lexical(Adverb), " ", head: node]`, `require
modifier.FocusingModifierUse = Restrictive`, `require head.FocusedUse = No`)
with instances for `PurposeInfinitival`, `PrepositionPhrase` and
`AdverbPhrase`; its comment cites only [42ii, iv]. *only* is
`vocab:FocusAdverb/Only` with `FocusingModifierUse: Restrictive`. `FocusedUse`
is not a feature of `NounPhrase` today.

Orchestrator rulings 2026-10-10:

- FO1. Add a Noun Phrase instance to `FocusedPhrase`; no new Construction.
  The policy exports the host NP's features unchanged plus `FocusedUse = Yes`
  (the feature joins `NounPhrase` or the case phrase that carries it; choose
  the level that keeps the instance single).
- FO2. The licence is the Adverb's declared `FocusingModifierUse`; no word
  test. A focused NP is admitted where an Object or verb Complement NP is
  admitted; whether a position accepts it is read from `FocusedUse`, as for
  PPs.
- FO3. Subject position (*Only your opponents may activate this ability.*,
  *Only the chosen creatures can attack …*, 12 sole-line faces) is attested
  and CGEL-backed: it is in scope if FO1 reads it with no further
  declaration; if it needs its own position licence, list it with the count
  and leave it. Do not add a check whose only job is to block it.
- FO4. One analysis per string. *block only creatures with flying* has the
  focused NP *creatures with flying* (with whatever attachments *block
  creatures with flying* has today, no more); it must not also read with
  *only* modifying the bare Nominal or the verb. *Activate only during your
  turn.*, *Activate only as a sorcery.* and *can attack only alone* keep
  exactly their Readings (no NP Reading of the PP or Adverb focus).
- FO5. No double focus (`head.FocusedUse = No` already says so); *only one
  target* is the same instance, not a determiner-internal *only*.
- FO6. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is reported with its shape before landing; none is
  expected, because no covered face matches the attestation pattern, so an
  increase means an NP focus was built where a PP or Adverb focus already
  reads (a defect under FO4).

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category sentence --readings 3`): witness 0;
control = the same sentence without *only* (3+ = enumeration capped).
Re-probe on the claim parent first.

- Welkin Tern: "This creature can block only creatures with flying." (control
  3+)
- Muck Drubb: "When this creature enters, change the target of target spell
  that targets only a single creature to this creature." (control 3+)
- Reflective Golem: "Whenever you cast an instant or sorcery spell that
  targets only this creature, you may pay {2}." (control 3+)
- Ragemonger: "This effect reduces only the amount of colored mana you pay."
  (control 3+)

Also 0 today: "If target spell has only one target and that target is a
creature, change that spell's target to another creature." (Meddle); "Only
your opponents may activate this ability." (control without *only* 1).
Controls that must not change: "Activate only during your turn." 1;
"Activate only as a sorcery." 1; "This creature can block only during your
turn." 2; "This creature can attack only alone." 2; "Activate only once each
turn." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bonly\b` (1,473 faces, case-insensitive) plus 300 random
covered controls (`cargo xtask english-v3 diff --manifest … --random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; FO6 for
increases. Full `cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings` (and `deckmaste_lexical_source`
if a declaration changes), once, at the end: a rebuild after a grammar edit
takes about 3 minutes and the test binary about 7, so batch edits. Tests
assert the role (the Focus Adverb as Modifier, the NP as its head and as the
verb's Object, the FO4 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: **Focusing Modifier** and **Focused Use** name PPs and Adverb
Phrases only; amend both in the landing.

## Out of scope

- *Spend only black mana on X.* (8 sole-line faces): *spend* + mana NP + *on
  X* reads 0 without *only*; a frame gap, not focus.
- *only any time they could cast a sorcery* (2 faces): focus on a temporal
  NP Adjunct.
- *it's only the color its controller chose*, *instead only you draw a card*
  (2 faces).
- *the only creature card* (1 face): adjectival *only*.
- *only X and only Y* coordination beyond what ordinary coordination gives.

## Landing record

1. **Census delta** — `mqrlzszkuszn` → refreshed `ouktkvpyzvym`
   (implementation `zqnqzmnumpyr`): bounded covered faces **1,367 → 1,457**,
   Readings **20,228 → 27,329**; **90 gains**: **88** from this ticket and
   Rivaz of the Claw (**17** Readings) from `lvpxmkkl`'s preposed-frequency
   landing. Its *Once during each of your turns* is an initial Adjunct.
   The other trunk gain is Zevlor, Elturel Exile (**1,393** Readings), from
   `zunolrnv`'s preverbal-next landing; its *next* is a central Adverb
   Adjunct in *when you next cast*.
   Lost **none**, decreases **0**, increases **0**, digest-only
   **0**, unintended gains **0**. Selector: case-insensitive
   `cargo xtask english-v3 grep '\bonly\b'` identities (1,473 faces), union
   300 covered controls selected by `diff --random 300 --seed 20261008`:
   **1,759 distinct faces**. The pre-edit subset at `zqnqzmnumpyr` (claim
   parent `rmvovpnx`, before refresh) exactly matches the `mqrlzszkuszn`
   census on this set.
   Evidence: `target/english-v3/english-v3-focus-only-object/{manifest.json,
   before.json,after.json,diff.json,diff-integration-refresh.log,claim-parent-diff.json,
   refresh-diff.json,integration-refresh-diff.json,refresh-stat.log,
   integration-refresh-stat.log}`.
2. **Witnesses** — sentence probes before refresh, at pre-edit/post-edit
   `zqnqzmnumpyr`:
   Welkin Tern **0 → 3** (control 3); Muck Drubb **0 → 7** (control 7);
   Reflective Golem **0 → 10** (control 10); Ragemonger **0 → 3** (control 3);
   Meddle **0 → 2** (control 2); *Only your opponents may activate this
   ability.* **0 → 1** (control 1). No witness already read on the claim
   parent. Existing controls retain **1 / 1 / 2 / 2 / 1** for *Activate only
   during your turn.*, *Activate only as a sorcery.*, *This creature can
   block only during your turn.*, *This creature can attack only alone.*,
   and *Activate only once each turn.* Complete enumerations, cap 100;
   `before-probes.json` and `after-probes.json` in the evidence directory.
3. **Spot checks** — `zqnqzmnumpyr`, `spot-probes.json`: Air Bladder — focus
   on the Object *creatures*, with *with flying* at an existing predicate
   attachment in the selected Reading; Chef's Kiss — focused coordinated
   Object *a single permanent or player*; Edgewalker — focused Object *the
   amount of colored mana you pay*, retaining the relative attachments;
   Imperiosaur — focused Object *mana produced by basic lands* plus purpose
   infinitival; Meddle — focused Object *one target*; Merseine — focused
   Subject *the controller of the enchanted creature*; Retether — focused
   Subject *creatures* of the passive; Selective Obliteration — focused
   copular Complement *the color its controller chose*; Kumena's Awakening —
   focused Subject *you* under *instead*; Throne of Eldraine — focused mana
   Object in the second ability, with its existing purpose focus retained
   in the first. All ten selected analyses are intended.
4. **Deviations and STOPs** — `zqnqzmnumpyr`: one Noun Phrase instance of
   `FocusedPhrase`, no new Construction or frame. Explicit unfocused exports
   on ordinary NPs and their coordination summaries are required by the
   compiler. Subjects read through the same instance (FO3); copular
   Complement and *instead only you* coverage follows FO2–FO3 automatically,
   including the two passages listed as out of scope for targeted work.
   No additional position licence, lexicon route, or loader change; no
   superseded grammar/lexicon route. No STOPs. Extended **Focusing Modifier**
   and **Focused Use** in the glossary; no remaining glossary gap.
5. **Tests** — before refresh, `zqnqzmnumpyr`:
   `cargo test -p deckmaste_english_v3` **passed (794 tests)** and
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` **passed**.
   After refresh, `ukqoxwzxtwpq`: `cargo build -p xtask` **passed**; full
   `cargo test -p deckmaste_english_v3` **passed (797 tests)**;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` **passed**;
   full `cargo test -p deckmaste_lexical_source` **passed (82 tests)**;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` **passed**.
   Final refresh, `ouktkvpyzvym`, incorporates `zunolrnv`'s lexical data
   and test registrations; the registration conflict was resolved by retaining
   both suite modules exactly once. `cargo build -p xtask` **passed**;
   full `cargo test -p deckmaste_english_v3` **passed (804 tests)**;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` **passed**;
   full `cargo test -p deckmaste_lexical_source` **passed (82 tests)**;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` **passed**.
   Final logs are `build-integration-refresh.log` and the four test/clippy
   `*-integration-refresh.log` files in the evidence directory.
   Zero failures or ignored tests in the successful full runs. Logs:
   `tests-verified.log`, `clippy-verified.log`, and the four test/clippy
   `*-refresh.log` files in the evidence directory.
   Assurance counts: restored **0**, re-spelled **1**, ignored **0**, added
   **4**, removed **0**. Re-spelled
   `focusing_only::focus_does_not_supply_missing_constituents_or_other_functions`:
   FO2–FO3 retire its blanket Object/Subject prohibition. The same two inputs
   still reject unintended focus analyses; every other negative stays.
   The first full run found that obsolete prohibition and exited 101; it was
   corrected before the rerun. That rerun found an incomplete expected
   attachment list in the new Muck Drubb test; the test now compares full
   NP values with its unfocused control, preserving the existing *to this
   creature* attachment rather than enumerating assumed surfaces. No invented
   positive Oracle witness
   or test deletion. Four added tests cover attested Object roles, Subject
   role, existing Adjunct roles/double focus, and an independent NP roundtrip.
6. **Timing** — `ouktkvpyzvym`: the final refreshed bounded after-census took **36.92 seconds**
   wall time, **6 workers**, `--samples-per-face 0`, prebuilt binary under
   `/usr/bin/time -v`. No full-corpus census or Lean check was run.
