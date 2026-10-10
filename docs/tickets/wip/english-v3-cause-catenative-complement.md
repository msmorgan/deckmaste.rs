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
