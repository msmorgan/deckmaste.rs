---
needs: []
---
# Read *remove* with its Object and *from* source Complement

## Why

*Remove … from …* never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), *remove(s) …
counter(s) from* touches **547** unread faces and is the sole cause on **276**
(recon bucket "remove counters": 608 / 300). Counts are unread faces *touched*
(at least one localised failing unit matches) / *sole* (every failing unit
matches and no other recon STRONG bucket does). They are surface counts, not
gain forecasts.

Probes (admitted roots): "Remove a charge counter from this artifact." 0;
"Remove all counters from target creature." 0; "Remove a charge counter." 0;
"Put a charge counter on this artifact." 1. Remove's only frame in `verbs.ron`
is `Predicate([ObjectNounPhrase, Lex("Preposition", "From"),
Role("FrameComplement")])`; it is the `core-verb:Remove` row of the
unsupported-inventory table in the done `english-v3-generic-frame-consumption`
("unsupported slot category FrameComplement (Complement)"). This ticket takes
that row over from `english-v3-systemic-residuals`.

## Goal

*Remove NP from NP* reads with the theme as Object and the *from* PP as the
source Complement, in imperatives, costs (*Remove a +1/+1 counter from this
creature: …*), finite clauses and under *may*. The legacy `FrameComplement`
slot is replaced by a typed NP Complement of the selected *from*, which the
generic consumer admits. Whether the omissible-source frame (Red Ward: "This
effect doesn't remove this Aura.") is added is decided by attestation and
recorded.

## Analysis

*Remove* takes an Object (theme) and a source PP with *from*: *I removed leaves
from the pool*, which unlike *drain* has no locatum-object alternant (\**I
removed the pool of leaves*) (CGEL, Ch. 4, §8.3.1(c), p. 315, [60ii]). The
source Complement is omissible: *He removed the key from the table* vs *He
removed the key* (Ch. 4, §8.3.3, p. 319). The *from* PP is therefore a selected
Complement, not a free Adjunct.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Spitting Hydra: "{1}{R}, Remove a +1/+1 counter from this creature: It deals
  1 damage to target creature."
- Timberline Ridge: "At the beginning of your upkeep, remove a depletion
  counter from this land."
- Phyrexian Prowler: "Remove a fade counter from this creature: This creature
  gets +1/+1 until end of turn."
- Voracious Hatchling: "Whenever you cast a white spell, remove a -1/-1 counter
  from this creature." (and its black twin)
- Perfect Intimidation: "Remove all counters from target creature." (modal
  bullet)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-remove-from-complement-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-remove-from-complement-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A *from* PP read as a free Adjunct or as an NP postmodifier of the
   counter NP is a defect. A wrong analysis that parses is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Other source frames deferred in `english-v3-systemic-residuals` (*from-M*,
  *from-A*: Exile, Discard, Reveal, Choose, Cast, Enter, Put): not reopened
  here. Reuse whatever typed source Complement already exists; do not build a
  second one.
- *counters removed this way* (reduced passive postmodifier) beyond what the
  Remove frame gives for free; *move a counter from … onto …* (Move).

## Landing record

In addition to the standard record: the replacement Remove frame and the
retired `FrameComplement` slot; whether the bare-Object frame was added and its
attestation; before/after counts stamped with change ids; timings as integer ns
and ns/B with host load and worker count.
