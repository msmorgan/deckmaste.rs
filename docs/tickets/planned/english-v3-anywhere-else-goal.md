---
needs: []
---
# Read *anywhere else* as a locative Complement: If that creature would leave the battlefield, exile it instead of putting it anywhere else.

## Why

*anywhere* reads only after *from* ("You may cast this card from anywhere."
3, from `docs/tickets/done/english-v3-from-anywhere.md`). As the goal of
*put* it does not, with or without *else*: "Put it anywhere." 0, "Put it
anywhere else." 0, "Exile it instead of putting it anywhere." 0, beside "Put
it there." 1, "Exile it instead of putting it there." 1 and "Exile it instead
of putting it into your graveyard." 1. *else* is not a word of the lexicon.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
`word:else` 16 sole, 16 touched. `cargo xtask english-v3 grep -i '\banywhere
else\b'` finds **16** faces, all unread; **16** have that line as their only
failing line. **15** are the replacement tail *exile it / them instead of
putting it / them anywhere else* (`'\b(put|putting) (it|them) anywhere
else'`); 1 is *but not from anywhere else* (Haakon, Stromgald Scourge).

## Goal

*anywhere else* is an NP (*anywhere* with its post-head dependent *else*)
functioning as the locative Complement of *put*: "… exile it instead of
putting it anywhere else.", with byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.6, p. 423: the compound determinatives include [41iv]
"*everywhere*, *somewhere*, *anywhere*, *nowhere*"; "The analysis of the
locatives as heads of NPs is motivated by their use as object in
constructions like *Have you got anywhere to spend the night?*"; and, under
"*Else* and *more*": "The compound determinatives share with the
determiner-heads *all*, *much*, and *little* the property of taking *else* …
as immediate post-head dependent: *everything else*". Those back *anywhere*
as head of an NP and *else* as its immediate post-head dependent. That such
an NP functions as a goal Complement of *put* with no Preposition is the
project ruling (the from-anywhere ticket's R6a re-categorised *anywhere* as a
Determinative in fused determiner-head use and asked for any attested
adverbial use to be reported: this is it).

Existing declarations: `vocab:CompoundDeterminative/Anywhere`
(`FusedHeadUse: Yes`, `DeterminerUse: Singular`, `ComparativePostmodifierUse:
Yes`; `lexicon/core.ron`), read by `FusedDeterminerHeadNounPhrase` (`[head:
lexical(Determinative)]`, `head.FusedHeadUse = Yes`); `core-verb:Put`'s frame
`Predicate([ObjectNounPhrase, Role("LocativeComplement")])`
(`lexicon/verbs.ron`); `LocativeComplement` (`[phrase: PrepositionPhrase]`,
`phrase.LocativeUse = Yes`), which is how *there* reads.

Orchestrator rulings 2026-10-10:

- AE1. *else* is a post-head dependent of the fused head: one added form on
  `FusedDeterminerHeadNounPhrase` (head + " " + the dependent), licensed by a
  feature the compound determinative declares, with *else* declared in
  `lexicon/core.ron` in the category the glossary gives a post-head
  dependent of this kind (amend the glossary in the landing if it has none).
  *else* gets no other function: 0 faces have it outside *anywhere else*
  (`'\belse\b'` 16 faces, the same 16).
- AE2. The locative function: one added form on `LocativeComplement` taking
  an NP whose head declares locative use (`LocativeUse` on the compound
  determinative, data), beside the PP form. No Preposition homograph of
  *anywhere*, and the NP does not become a general Object of *put* (the
  frame's first slot stays the thing moved).
- AE3. One analysis per string. *from anywhere* keeps exactly its Readings
  (136 covered lines); *putting it anywhere else* has one structure: Object
  *it*, locative Complement [*anywhere else*]. The twins to avoid:
  *anywhere* as a second Object, and *else* attached to the clause.
- AE4. Bare *put it anywhere* (no *else*) is unattested; it will read as a
  consequence of AE2. Do not add a check whose only job is to block it;
  report any covered face it changes.
- AE5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected; the 136 covered *from anywhere* lines and the locative
  *there* lines must keep their counts. Report any increase with its shape.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Gruesome Encore: "Put target creature card from an opponent's graveyard
  onto the battlefield under your control. It gains haste. Exile it at the
  beginning of the next end step. If that creature would leave the
  battlefield, exile it instead of putting it anywhere else."
- Nephalia Academy: "If a spell or ability an opponent controls causes you to
  discard a card, you may reveal that card and put it on top of your library
  instead of putting it anywhere else."
- Storm Herald: "When this creature enters, return any number of Aura cards
  from your graveyard to the battlefield attached to creatures you control.
  Exile those Auras at the beginning of your next end step. If those Auras
  would leave the battlefield, exile them instead of putting them anywhere
  else."
- From the Catacombs: "Put target creature card from a graveyard onto the
  battlefield under your control with a corpse counter on it. You take the
  initiative. If that creature would leave the battlefield, exile it instead
  of putting it anywhere else."

Probe the fragment first on the claim parent: "If that creature would leave
the battlefield, exile it instead of putting it anywhere else." Controls
that must not change: "Exile it instead of putting it into your graveyard."
1; "Exile it instead of putting it there." 1; "Put it there." 1; Dissipate
"Counter target spell. If that spell is countered this way, exile it instead
of putting it into its owner's graveyard." 2 (census); Luminous Phantom "If
Luminous Phantom would be put into a graveyard from anywhere, exile it
instead." 4 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(anywhere|else|there)\b|\binstead of putting\b` (the
first alternative alone is 624 faces; record the total) plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; AE5 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`, once, at
the end; a rebuild after a grammar edit takes about 2–3 minutes, the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
role (*else* as post-head dependent of the fused head; the NP as locative
Complement of *put*; the AE3 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Haakon, Stromgald Scourge: "… but not from anywhere else." (the *but not
  from* PP coordination is a separate blocker: "You may cast this card from
  your graveyard, but not from exile." probed 0).
- Faces whose only remaining blocker is elsewhere on the line: The Eighth
  Doctor, Kheru Lich Lord (*at random from your graveyard*,
  `english-v3-at-random-before-source`), Realmbreaker, Geth, Elemental
  Expressionist, Llanowar Greenwidow, Spirit-Sister's Call (quoted grants).
- *something else*, *someone else* (0 faces).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
