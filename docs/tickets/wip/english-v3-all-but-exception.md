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

1. **Census delta** (`vqyrupul`, before = empty child of claim parent
   `vozvszqm`, after = implemented tree): bounded manifest
   `target/english-v3/english-v3-all-but-exception/manifest.json`, the Method's
   grep union (**898 faces**) plus **300 covered random controls**, seed
   **20261008**, **1,190 distinct faces**. Covered **1,062 → 1,069**; total
   Readings **35,996 → 36,115**. **7 gains**, **0 lost**, **0 decreases**,
   **0 increases on previously covered faces**, **0 digest-only changes**
   (therefore **0 changed digest-only lines/classes**), **0 source changes**,
   **0 issues**. Wrong-analysis gains: **0**. Before and after were both
   measured on this workspace; `baseline-wave8.json` supplied only the seeded
   covered-control identities. All **300** controls were covered on the claim
   parent. Comparison: `cargo xtask english-v3 diff BEFORE.json AFTER.json
   --lines --json target/english-v3/english-v3-all-but-exception/diff.json`.
2. **Witnesses** (probes on `vqyrupul`; every witness re-probed before editing,
   none dropped): Soulgorger Orgg **0 → 1** (control **1**); Temple Altisaur
   **0 → 3** (control **3**); Choice of Damnations **0 → 1** (control **1**);
   Lily Bowen's quoted witness **0 → 18** (bare control **36**, Cardinal-route
   control **18**). Bare *one* also has a lexical Determinative analysis;
   AB1's exception PP selects the Cardinal quantity, preserving the Cardinal
   control's independent ambiguity under W9-A. Aladdin's Lamp full text
   **0 → 0** (full control **0**); its attested *put all but one of them …,
   then draw a card* Bare Predicate **0 → 1** (control **1**). Its remaining
   full-face blocker is independent of the exception construction. NP
   fragments *all but 1 life*, *all but one of them*, and *all but one +1/+1
   counter* each **0 → 1**. Prevention controls each remain **2**, *Destroy
   one of them* **1**, Torrent Elemental's attack line **1**, and Felidar
   Sovereign's upkeep line **1**.
3. **Spot checks** (`vqyrupul`): only **7** faces became covered, so all were
   checked rather than inventing ten gains. Soulgorger Orgg (**6** per-line
   Readings) has *all but 1* as the life NP's Determiner; Choice of Damnations
   (**1**) has the degree exception *all but that many* as Determiner; Lily
   Bowen (**21**) has the singular Cardinal exception as the counter NP's
   Determiner. Temple Altisaur (**3**), Hyperion, Supreme Hero (**6**), Ajani
   Steadfast (**18**), and Forcefield (**64**) have *all but 1* as the fused
   head of the damage partitive. Every enumerated full-document Reading of
   each gained face contains the intended exception phrase. Three additional
   control checks: Wrath of God retains ordinary *all creatures* determination
   (**1**); Torrent Elemental retains ordinary *all creatures* with its
   Subject-gap Relative Clause (attack line **1**); Felidar Sovereign retains
   its comparative life quantity (upkeep line **1**). Counts distinguish the
   census's per-line sums from whole-document probe products.
4. **Deviations and STOPs**: added **1** Construction,
   `ExceptionDeterminativePhrase`: its Determinative head with an exception PP
   dependent differs from degree premodification and from a PP itself
   functioning as Determiner (AB1; CGEL p. 433). Added **2** instances of the
   existing `QuantitativePrepositionPhrase` schema, with separate cardinal and
   degree result Categories as required by the schema compiler, and **2**
   complement policies. Existing schema forms, instance requirements and
   exports are unchanged. Lexicon declares only *all*'s exception permission
   and *but*'s quantity-complement use (AB2); coordinator *but* is retained.
   No superseded route, frame or test was retired. Added the glossary's
   **Quantitative Exception Use**, with its grammatical source and AB1–AB3
   ruling; no game-rule claim is introduced. Aladdin's Lamp's independent
   full-text failure remains disclosed. No STOPs.
5. **Tests** (`vqyrupul`): `cargo test -p deckmaste_english_v3` passed
   (**1,125** tests; **1** inherited ignored test:
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`,
   blocked by `english-v3-mana-symbol-verbless-subject` MS4's governed Mana
   Phrase Subject route). `cargo test -p deckmaste_lexical_source` passed
   (**44** tests). `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` both
   passed. Assurance changes: restored **0**, re-spelled **0**, newly ignored
   **0**, added **6**, removed **0**. New module `all_but_exception` is
   registered only in `suite_2`; it tests the declared roles, independent
   construction, both roundtrip laws, head selection, and exclusion of the
   coordinator/floated-quantifier twins. Citation checks: **0** noncompliant,
   **0** stale; bless changed no lockfile; diff audit selected **0** sites.
6. **Timing** (`vqyrupul`): the bounded after census took **21.19 s** wall
   time under `/usr/bin/time -v`, **6 workers**, `--samples-per-face 0`.
   No full corpus census was run.

