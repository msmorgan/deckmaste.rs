---
needs: []
---
# Read *all but* + quantity as one quantifier: you lose all but 1 life / prevent all but 1 of that damage

## Why

*all* with a *but* exception reads nowhere: NP "all but 1 life" 0, "all but
one of them" 0, "all but one +1/+1 counter" 0; "If a source would deal damage
to you, prevent all but 1 of that damage." 0. Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:all but` 11,
failing token `but` 11. `cargo xtask english-v3 grep -i '\ball but\b'` finds
**13** faces, all unread; **11** have that line as their only failing line.
By shape:

- (a) *all but* + quantity, `\ball but (\d+|one|that many|X)\b`: **9** faces,
  9 sole. Before a noun 3 (*all but 1 life*, *all but one +1/+1 counter*,
  *all but that many permanents*); heading a partitive 6 (*all but 1 of that
  damage* 4, *all but one of them / those counters* 2).
  The 6 partitive faces also need `english-v3-partitive-quantity-phrases`
  (land after it; alone, this ticket gains the 3 pre-nominal faces).
- (b) *all but the* + NP, `\ball but the\b`: 4 faces, 2 sole (*all but the
  bottom card of target player's library*). Out of scope.

## Goal

*all but* + a quantity is a Quantitative Determiner: before a nominal (*lose
all but 1 life*, *sacrifices all but that many permanents*) and as the head
of a partitive (*prevent all but 1 of that damage*, *put all but one of them
on the bottom of your library*), with byte-exact realisation.

## Analysis

CGEL Ch. 5 §11(f), p. 433, "PPs: condition, exception, proportion, and
addition/subtraction", [8ii] "[*All but/except a few*] *helicopters have
crashed*"; "The exception construction is found only with *all*: with
*every*, *no*, *any* the PP can only occur after the head noun, as in *Every
helicopter but one has crashed*." Those back *all* + exception PP as one
determinative phrase in Determiner function, and its restriction to *all*.
Its use as a partitive head and with a non-count noun (*life*) are project
rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): the
`QuantitativeDeterminer` feeders `DegreeDeterminativePhrase` (`[modifier:
lexical(Determinative), " ", head: lexical(Determinative)]`, *that many*)
and `PrepositionDeterminer` (over `QuantitativePrepositionPhrase`, *up to
two*); the schema `QuantitativePrepositionPhrase` (`[head:
lexical(Preposition), " ", complement: …]`, licensed by the Preposition's
declared `QuantitativeComplement`); consumers `QuantifiedNounPhrase` and,
after `english-v3-partitive-quantity-phrases`, `PartitiveNounPhrase`.
`vocab:FloatedQuantifier/All` declares `DeterminerUse: PluralOrMass`. *but*
is declared as a Coordinator; whether a Preposition *but* lexeme exists is to
be checked in `lexicon/core.ron` (none found by `"vocab:Preposition/But"`).

Orchestrator rulings 2026-10-10:

- AB1. One added `QuantitativeDeterminer` form, in the family of
  `DegreeDeterminativePhrase` / `PrepositionDeterminer`: a Determinative head
  that declares the exception use, followed by an exception PP whose
  Complement is a quantity (`Cardinal`, `QuantityConjunct` for *1*, or the
  degree phrase *that many*). Prefer a refinement of one of those two
  Constructions; a new Construction needs the grammatical distinction stated
  under Deviations.
- AB2. Licences are declared: the head declares the exception use (*all*
  only, per CGEL p. 433) and the exception marker declares its
  `QuantitativeComplement` class. If *but* needs a Preposition lexeme, it is
  declared in `lexicon/core.ron` with that one function; it does not become
  an NP postmodifier or an Adjunct (no attested use; pruning rule).
- AB3. Number and countability follow the exception quantity as they do for
  the bare quantity today: *all but 1 life* is as *1 life*; *all but one
  +1/+1 counter* selects a singular head (Lily Bowen), like *more than one*
  (CGEL p. 432).
- AB4. One analysis per string. The twins to avoid: *all* as Determiner of a
  coordination [*all*] *but* [*1 life*] (Coordinator *but*), and *all* as
  floated quantifier with *but 1 of that damage* attached to the verb. The
  whole *all but 1* is one constituent.
- AB5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *all but*, and Coordinator *but*
  lines (*but not*, *but only*) must keep their Readings. Report any increase
  with its shape before landing.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Soulgorger Orgg: "When this creature enters, you lose all but 1 life."
- Temple Altisaur: "If a source would deal damage to another Dinosaur you
  control, prevent all but 1 of that damage." (also needs
  `english-v3-partitive-quantity-phrases`)
- Choice of Damnations: "Target opponent chooses a number. You may have that
  player lose that much life. If you don't, that player sacrifices all but
  that many permanents of their choice."
- Lily Bowen, Raging Grandma: "At the beginning of your upkeep, double the
  number of +1/+1 counters on Lily Bowen if its power is 16 or less.
  Otherwise, remove all but one +1/+1 counter from it, then you gain 1 life
  for each +1/+1 counter removed this way."
- Aladdin's Lamp: "{X}, {T}: The next time you would draw a card this turn,
  instead look at the top X cards of your library, put all but one of them on
  the bottom of your library in a random order, then draw a card. X can't be
  0."

Controls that must not change: "If a source would deal damage to you, prevent
that damage." 2; "If a source would deal damage to you, prevent 1 damage." 2;
NP "one of them" (in "Destroy one of them." 1); Torrent Elemental "Whenever
this creature attacks, tap all creatures defending player controls." 1
(census); Felidar Sovereign "At the beginning of your upkeep, if you have 40
or more life, you win the game." 1 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b[Aa]ll (but|of|the|other|damage|creatures|cards|permanents|lands)\b|\bbut (not|only|one|\d+|that)\b

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent (size
the selector with `cargo xtask english-v3 grep --count` and record it; the
first alternative alone is 886 faces). Zero lost; AB5 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds. Tests assert the role (*all* as head, the exception PP as
its dependent, the whole as Determiner or partitive head; the AB4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only.

## Out of scope

- (b) *all but the bottom card / the bottom six cards / the highest roll* (4
  faces, 2 sole): *all* as fused head with an NP exception. Mint separately
  if wanted.
- *Every helicopter but one* post-head exception (0 faces).
- Deification's *all loyalty counters on it being removed* (a second blocker
  on that line); Ajani Steadfast's emblem quote if the quote route blocks it.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
