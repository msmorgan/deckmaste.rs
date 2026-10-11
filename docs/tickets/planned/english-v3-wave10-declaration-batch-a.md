---
needs: []
---
# Wave-10 declaration batch A: hexproof from, plural card names, bare-Object *return*, reciprocal *each other*

## Why

Four end-of-sentence clusters of the wave-9 blocker report (trunk `qzprwuxz`,
`target/english-v3/blockers-wave9.json`) are declarations an existing route
reads. No construction work is expected; each item says what to do if that
turns out false. Together about **50** sole-blocker faces. Counts are
`cargo xtask english-v3 grep` faces and sole failing lines.

| Item | Pattern | Faces (unread) | Sole |
|---|---|---|---|
| A. *hexproof from* Quality | `(?i)hexproof from` | 19 (15) | 12 |
| B. plural card names | own name + plural verb (hand-cut) | 26 failing faces | 19 |
| C. *returned this way* | `(?i)returned this way` | 10 (10) | 10 |
| D. *fight each other* | `(?i)fights? each other` | 11 (11) | 11 |

## Items and rulings

Orchestrator rulings 2026-10-11 (DA0 applies to all): each licence reads a
declared feature or frame; no guard names a word, construction or card
(R-class: declare the class, follow English's real paradigm, mint no Lexeme
for an unattested word). An item that needs a Construction or a compiler
change is RETURNED with its probes, not built here (R-budget); the others
land without it.

### A. *Hexproof from* + Quality (plugin data, R-plugin)

Document "Hexproof from black" 0, "Hexproof from monocolored" 0, "Flying,
hexproof from activated and triggered abilities" 0; "Hexproof" 1; "Protection
from black" 1. `plugins_v2/builtin/macros/keyword_abilities/hexproof.ron`
declares `FixedKeyword(surface: "hexproof")` with `params: []`;
`protection.ron` declares `parameter: Quality(preposition: ("Preposition",
"From"))`, read by `QualityKeyword`.

- DA1. Add a separate definition for the Quality form (`hexproofFrom.ron`,
  `params: [Quality]`, the protection parameter row), grammar-only and
  bodiless if it has no body yet (owner ruling PK1), with its facts
  registration or exemption. Bare *hexproof* keeps its definition. Do not
  give one Lexeme two parameter classes.
- DA2 (R-wrong, R-one). Four covered faces read *gains hexproof from that
  color* today (Tam, Mindful First-Year; Skrelv, Defector Mite; Sungold
  Sentinel; Mondo Gecko) through a wrong analysis: `BareKeyword` *hexproof*
  plus a stray *from*-PP on the predicate ("It gains hexproof from that color
  until end of turn." 1). After DA1 they gain the keyword-with-Quality
  Reading. Report each as an increase of that shape; retiring the stray-PP
  Reading belongs to whichever restriction owns stray PPs after a keyword
  mention (name it in the landing record; do not add a guard on *hexproof*).
- Witnesses (whole line, sole failing line of the face): Knight of Grace
  "Hexproof from black"; Niv-Mizzet, Supreme "Flying, hexproof from
  monocolored"; Breaker of Creation "Hexproof from each color"; Volatile
  Stormdrake "Flying, hexproof from activated and triggered abilities".

### B. Plural card names (lexicon data)

As the face: "Whenever Raph & Leo attack, draw a card." 0 while "… Raph & Leo
attacks …" 1; "When Tom, Bert, and William die, draw a card." 0 / *dies* 1;
"When The Immortal Weapons enter, draw a card." 0 / *enters* 1. Route:
`CatalogName` exports `number = name_number(head.NameNumber)`; feature
`NameNumber { Singular, Plural } default Singular`; one declaration exists,
`"catalog:card-names.txt/Aang and Katara": {"NameNumber": "Plural"}`
(`lexicon/core.ron`), under the owner ruling of 2026-10-08 ("Oracle names
carry the grammatical agreement declared on their lexeme").

- DA3 (R-class). Declare `NameNumber: Plural` for every catalog name whose
  own Oracle text agrees plural with it, found by census (own name or short
  name + plain-form verb / *them* / *they*), not only the sole-blocker ones.
  The hand-cut finds 26 failing faces: the *X & Y* / *X and Y* team-ups
  (Raph & Leo, Bebop & Rocksteady, Aang and La, Cloak and Dagger, Lo and Li,
  Ran and Shaw, Tui and La …), *Tom, Bert, and William*, and plural-headed
  names (*The Immortal Weapons*, *The Notary Hobbits*, *The
  Sackville-Bagginses*, *The Neutrinos*, *The Fantastic Four*). A name is
  plural because its text says so, never because of its spelling: no rule
  keyed on *&* or *and*.
- DA4. Short names: the witnesses use the short name (*Raph & Leo* of *Raph &
  Leo, Sibling Rivals*). If the short-name Lexeme does not inherit the
  declaration from its catalog entry, RETURN the item with the probe: making
  short names inherit is a derivation change, not data.
- DA5 (R-reach). Flipping a name to Plural removes its singular agreement.
  The selector includes every face containing each flipped name; a covered
  line that agreed singular with a flipped name is a STOP (the name is then
  not consistently plural), not a loss to accept.
- Witnesses (probe with `--as-card`): Raph & Leo, Sibling Rivals "Whenever
  Raph & Leo attack, if it's the first combat phase of the turn, untap one or
  two target attacking creatures."; The Immortal Weapons "When The Immortal
  Weapons enter, return target instant or sorcery card from your graveyard to
  your hand."; Tom, Bert, and William "When Tom, Bert, and William die, if
  they were a creature, return them to the battlefield."; Aang and La,
  Ocean's Fury "Whenever Aang and La attack, put a +1/+1 counter on each
  tapped creature you control." Control: test
  `name_lexeme_scope` (*Aang and Katara enter.* reads, *enters* does not).

### C. *return* with a bare Object (lexicon data)

"You lose 3 life for each card returned this way." 0; "… each card exiled
this way." 2; "… each card returned to your hand this way." 2; "Return it."
0. *return* (`lexicon/verbs.ron`) declares only frames with a *to* goal, so
its bare passive participle has no frame. `english-v3-hygiene-3` listed
"returned this way 10" as unrouted.

- DA6. Declare the goal-less frame (`Predicate([Object])`, with the same
  `SourceFreeObject` Object the goal frames use). The participial
  postmodifier and "A card was returned this way." follow from the existing
  passive routes. Active *Return it.* becomes readable and is unattested: no
  check to block it.
- DA7 (R-one). The witness must read exactly as many ways as its *exiled*
  control (2). A covered *return … to …* line must not gain a Reading in
  which the bare frame takes the Object and the *to*-PP floats as an Adjunct:
  selector below; any such increase is a defect to fix by the frame's
  declared tail licence (`SelectedTailLicence`), not by a word test.
- Witnesses (sentence of the sole failing line): Revival Experiment "You
  lose 3 life for each card returned this way."; Plow Through Reito "Target
  creature gets +1/+1 until end of turn for each Plains returned this way.";
  Sink into Takenuma "Target player discards a card for each Swamp returned
  this way."; Morgue Burst "Morgue Burst deals damage to any target equal to
  the power of the card returned this way." (`--as-card`).

### D. Reciprocal *each other* (lexicon data)

"Those creatures fight each other." 0; "Then the chosen creatures fight each
other." 0; "Target creature fights another target creature." 1; "It fights
that creature." 1; NounPhrase "each other" 0. CGEL Ch. 5 §10.1.2, p. 428:
"There are two reciprocal pronouns, *each other* and *one another*"; Ch. 17
§4.2, p. 1502: "Reciprocals always function as complement". *fight*
(`plugins_v2/builtin/macros/keyword_actions/fight.ron`, `frame_set:
Transitive`) already takes an NP Object; the lexicon has no reciprocal
pronoun (accusative pronouns are `vocab:ObjectPronoun/…`, read by
`AccusativePronoun`).

- DA8 (R-class). Declare the reciprocal pronouns as multiword accusative
  pronoun Lexemes of the existing object-pronoun class: *each other*
  (attested, 11 faces) and *one another* (attested: `grep 'one another'` 2
  faces). No nominative form (CGEL p. 1502); the genitive *each other's* only
  if attested (count it; 0 expected).
- DA9 (R-one). The twin to avoid: *each* + fused *other* (*the other* is a
  fused head since `english-v3-fused-head-noun-phrases`; "Each of those
  creatures deals damage to the other." 2 must stay 2, and *each other
  creature* keeps reading as Determiner + *other* + noun: `grep 'each
  other'` finds 343 faces, 298 covered, nearly all that shape). If *fight
  each other* reads twice after the declaration, STOP with both trees.
- DA10. A plural antecedent requirement (CGEL Ch. 17, p. 1501) is not added:
  it would only block unattested strings. If the pronoun class cannot hold a
  multiword Lexeme, RETURN the item.
- Witnesses (sole failing line): Rivals' Duel "Those creatures fight each
  other."; Blizzard Brawl "Then those creatures fight each other."; Duel for
  Dominance "Then the chosen creatures fight each other."

## Method

Standard constraints apply. All witnesses probed 0 and all controls probed at
the stated counts on 2026-10-11 with the trunk binary; re-probe on the claim
parent. Bounded census, never `--all`: every supported face matching
`(?i)hexproof|protection from|\breturn(s|ed|ing)?\b|each other|one another|\bfights?\b`
plus every face containing a name flipped under DA3, plus 300 random covered
controls (`--random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent.
Zero lost. Expected Reading-count increases: DA2's four faces only; anything
else is reported with its shape and treated as a defect until explained.
Because `plugins_v2/` changes (item A): FULL `cargo test -p xtask`, `-p
deckmaste_semantics_v2`, `-p deckmaste_construction_core`, `-p
deckmaste_english_v3`, `-p deckmaste_lexical_source`, clippy `--tests -- -D
warnings` for each, and `cargo xtask lean check`, once, at the end (lexicon
and plugin `grammar:` data load at runtime; a grammar rebuild, if any, takes
2–3 minutes and the six test binaries about 75 seconds). Tests assert the
role per item (Quality as the keyword's parameter; plural agreement of the
Name Subject and the singular negative; the participle's bare frame; the
reciprocal as *fight*'s Object and the DA9 negative), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *maximum hand size* as a scalar Subject: declared in
  `english-v3-scalar-location-plain-be-and-become` (SL4).
- *Trample over planeswalkers* (1 face); *Partner with* + Name and the
  Quality-then-Cost keyword lines (own tickets).
- Unexpressed reciprocal *Those creatures fight.* (`grep '\bfight\.'`: 0
  faces).
- Plural-name faces whose line fails for a second reason (*meld them into
  …*, 7 sole faces; *they both own and control*).

## Landing record
