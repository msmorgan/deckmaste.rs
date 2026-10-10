---
needs: []
---
# Read *cause* with an Object and a *to*-infinitival: Creatures entering don't cause abilities to trigger.

## Why

*cause* reads nowhere. Its declared frame has no consumer. Blocker report at
trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): the failures hide
under end-of-sentence keys (`at:, that` 14, `at:trigger .` 6, `at:graveyard .`
5, `at:life .` 4, `at:permanents .`, `at:hand .`).
`cargo xtask english-v3 grep '\bcauses?\b'` finds **66** faces, all unread; 66
failing lines match, **62** of them the only failing line of their face. By
shape (sole lines): *cause(s) … to trigger* 27 (Torpor Orb, Panharmonicon);
*causes you to discard this card* 18 (Loxodon Smiter, Quagnoth); the rest
*cause you to lose life / sacrifice permanents / search their library*.

The 62 is an upper bound: every probe of a shortened variant fails at *cause*
itself, so a second cause behind it cannot be excluded line by line.

## Goal

*cause* + Object NP + *to*-infinitival reads as a complex Catenative
Construction in every finite and plain form attested (*causes*, *don't cause*,
*can't cause*, *would cause*), with byte-exact realisation.

## Analysis

CGEL Ch. 14 §5.4, p. 1233, [34ii]: *cause* is listed with *allow*, *enable*
and *permit* under "3Aii: plain-complex with raised object". That backs
*cause* + NP + *to*-infinitival as a complex catenative. The frame spelling
and the Subject analysis below are project rulings. Glossary: **Catenative
Construction**, **Catenative Complement**.

Existing declarations: `deckmaste_lexical_source/lexicon/verbs.ron`, `Cause`:
`Predicate([Role("Object"), Literal("to"), Role("VerbPhrase")])`. No
construction consumes `Literal("to")` + `Role("VerbPhrase")` (the only use in
the file). The nearest reading route is causative `Have`:
`Predicate([Argument((relation: Object, category: "NounPhrase")),
Argument((relation: Complement, category: "BarePredicate"))])`, read by the
generic frame consumer (`english-v3-generic-frame-consumption`,
`english-v3-causative-have`). `InfinitiveComplement` is the category of
*to*-infinitivals (`frame InfinitiveSelection`, used by `Choose`).

Orchestrator rulings 2026-10-10:

- CC1. Re-spell `Cause`'s frame after `Have`'s: Object `NounPhrase` Argument
  plus a Complement Argument of category `InfinitiveComplement`. No new
  Construction if the generic frame consumer reads it; this may be a data-only
  landing. Delete the unread `Literal`/`Role("VerbPhrase")` spelling (retire
  the superseded route on the lexicon side).
- CC2. The licence is the verb's declared frame. No test on the word *cause*,
  on *trigger*, or on the Object.
- CC3. Subject: *Creatures entering don't cause …*, *a creature dying causes
  …*, *a player losing unspent mana causes …* keep the Subject analysis they
  have today, an NP with a gerund-participial postmodifier ("Creatures
  entering have haste." 1, "A creature dying has haste." 1). Agreement is with
  the noun (*Creatures entering don't* / *a creature dying causes*). Do not add
  a clausal-Subject construction; if a witness needs one, STOP with the count.
- CC4. One analysis per string. The *to*-infinitival is the Catenative
  Complement of *cause*; it must not also read as a purpose infinitival
  Adjunct on a monotransitive *cause* (there is no `[Obj]` frame; do not add
  one), nor as a postmodifier of the Object NP.
- CC5. Passive (*is caused to*) and *cause* + NP alone are unattested; do not
  add them, and add no check to block them.
- CC6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *cause(s)*. Any increase means the
  frame change leaked into another verb and is a defect.

- W9-A (orchestrator, 2026-10-10). CC4 requires one catenative route
  for *cause* + NP + *to*-infinitival, preserving independent NP ambiguities.
  The witness/NP-only control counts are Sigarda 3/3, Quagnoth 2/2, and
  Teysa 5/5. This interpretation applies to every "one analysis per string"
  ruling.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the
`ymqwzour` binary (`--category sentence --readings 3`): all 0. Re-probe on the
claim parent first.

- Torpor Orb: "Creatures entering don't cause abilities to trigger."
- Sigarda, Host of Herons: "Spells and abilities your opponents control can't
  cause you to sacrifice permanents."
- Quagnoth: "When a spell or ability an opponent controls causes you to
  discard this card, return it to your hand."
- Yurlok of Scorch Thrash: "A player losing unspent mana causes that player to
  lose that much life."
- Teysa Karlov: "If a creature dying causes a triggered ability of a permanent
  you control to trigger, that ability triggers an additional time."

Also 0 today: "A creature causes an ability to trigger."; "Creatures don't
cause abilities to trigger."; "It can't cause you to sacrifice permanents."
Controls that must not change: "You may have it gain flying until end of
turn." 3+; "You may have that player discard a card." 1; "That ability
triggers an additional time." 1; "Target opponent chooses to sacrifice a
creature." 1; "Creatures entering have haste." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bcauses?\b` (66) and, because CC1 retires a frame spelling,
every face matching `\b(have|has) [^.]* (gain|get|discard|sacrifice|put|lose)\b`
(308 faces, the causative-*have* consumers), plus 300 random covered controls (`cargo
xtask english-v3 diff --manifest … --random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; CC6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings` (and `deckmaste_lexical_source`), once, at the end: a rebuild after a
grammar edit takes 2–3 minutes and the six test binaries about 75 seconds;
lexicon data loads at runtime. Tests assert the role (the NP as Object of
*cause*, the infinitival as its Catenative Complement, the CC4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *instead of putting it into your graveyard* (Loxodon Smiter, 5 faces): reads
  today ("Put it onto the battlefield instead of putting it into your
  graveyard." 1); those faces fail on *causes* alone.
- *for each 1 life …* and other second causes on the four non-sole faces.
- A clausal gerund-participial Subject (CC3).

## Landing record

1. **Census delta** (`nzmrrqkysxto`, pre-edit claim parent `tzpunrwm` →
   frame replacement): 671 faces; covered 550 → 605; Readings 10,252 →
   10,727. 55 gains; lost faces none; decreases, increases on covered faces,
   and digest-only changes all zero; zero issues. Selector is the Method's
   two grep patterns (373 distinct faces), union 300 random covered controls
   selected by `english-v3 diff --manifest --random 300 --seed 20261008`
   (671 after overlap). Claim-parent BEFORE was measured on this manifest;
   `english-v3 diff before.json after.json --json diff.json --lines` is the
   comparison authority. Evidence is under
   `target/english-v3/english-v3-cause-catenative-complement/`.
   Post-refresh `uppvxtrvrrvn` imported phase-in/out `roomopmr`: the same
   671-face bounded census has zero changed faces or lines against the first
   AFTER; final delta remains 55 gains, 550 → 605 covered and 10,252 →
   10,727 Readings (`final-diff.json`).
2. **Witnesses** (`nzmrrqkysxto`): sentence probes before → after: Torpor Orb
   0 → 1; Sigarda, Host of Herons 0 → 3; Quagnoth 0 → 2; Yurlok of Scorch
   Thrash 0 → 1; Teysa Karlov 0 → 5. The three shortened diagnostics each
   0 → 1. All enumerations completed. Controls retain 3/1/1/1/1 in ticket
   order. NP-only controls retain Sigarda 3, Quagnoth 2, Teysa 5; these
   independent ambiguities are preserved under W9-A, with one catenative
   route per Reading.
3. **Spot checks** (`nzmrrqkysxto`): Torpor Orb, Hushwing Gryff, Tocatli Honor
   Guard and Doorkeeper Thrull select *abilities* as Object and *to trigger*
   as Catenative Complement. Panharmonicon, Ancient Greenwarden and Drivnod,
   Carnage Dominus select the triggered-ability NP as Object and *to trigger*
   as Catenative Complement. Loxodon Smiter and Quagnoth select *you* and
   *to discard this card*; Sigarda, Host of Herons selects *you* and *to
   sacrifice permanents*. All ten selected analyses use the intended route;
   wrong-analysis-only gains found: zero.
4. **Deviations and STOPs**: retired the unread Cause frame spelling and
   replaced it with the two declared Argument slots under CC1. There was no
   grammar-side consumer of the retired spelling; no Construction, loader
   code, or additional verb frame was added. Four tests were added for roles,
   retained NP ambiguity, frame licensing/controls/CC4 negatives and an
   independently constructed Torpor Orb constituent's two roundtrip laws.
   STOP on CC4's literal uniqueness resolved by W9-A above. Seed 20261008
   follows the user's binding brief rather than the Method's 20261010.
   Glossary gaps: none. Concurrent Kata snapshot refusals were retried;
   the modal-header owner resolved its divergent working copy. A behind-default
   integration refusal was resolved by refresh and the required rechecks.
5. **Tests** (`nzmrrqkysxto`): `cargo test -p deckmaste_english_v3` passed
   (900 tests); `cargo test -p deckmaste_lexical_source` passed (84 tests).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` both
   passed. Assurance: restored 0 / re-spelled 0 / ignored 0 / added 4 /
   removed 0; no removals to justify. Citation checks: zero noncompliant,
   zero stale, no changed citation sites. Post-refresh `uppvxtrvrrvn`:
   `cargo build -p xtask` passed; the same four full test/clippy commands were
   repeated and passed (English 905 tests, lexical-source 84; the five extra
   English tests belong to imported `roomopmr`, not this ticket).
6. **Timing** (`uppvxtrvrrvn`): final bounded AFTER census 28.97 s wall under
   `/usr/bin/time -v`, six workers, zero samples per face; no `--all` run.
