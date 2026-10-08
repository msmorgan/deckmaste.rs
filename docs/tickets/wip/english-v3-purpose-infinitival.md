---
needs: []
---
# Read the purpose *to*-infinitival Adjunct: mana spent to cast this spell

## Why

The ungoverned purpose infinitival never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), the recon buckets
"spend / spent" and "purpose infinitival (pay/spend/tap … to <verb>)" read
together touch **483** unread faces and are the sole cause on **287**: 154 of
those contain *spent to cast*, 108 *only to* (*Spend this mana only to cast
…*), 25 other. Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Pay {1} to draw a card." 0; "You may pay {2} to draw
a card." 0; "Sacrifice a creature to draw a card." 0; "Draw a card to gain 1
life." 0; "Spend this mana to cast a creature spell." 0, while "Spend this
mana." 1; "You gain life equal to the amount of mana spent to cast this
spell." 0, while "… equal to the amount of mana spent." 1.

## Goal

A *to*-infinitival without a governing preposition reads as a purpose Adjunct
of the clause (*Pay {1} to draw a card*) and of the reduced passive in an NP
Postmodifier (*the amount of mana spent to cast this spell*, *each color of
mana spent to cast it*), with its missing Subject left to the grammar's
existing control treatment. With `english-v3-focusing-only` landed, *Spend this
mana only to cast …* composes through its focusing Modifier; measure those
faces, and if focusing-only has not landed, record them as owed there.

## Analysis

The ungoverned infinitival is the most frequent type of purpose adjunct (*We
left early to miss the rush-hour*); ungoverned infinitivals are always
positive, a negative needing a governing preposition (CGEL, Ch. 8, §12.2,
p. 728, [9i]). Purpose elements are usually adjuncts (p. 728). The existing
`ToInfinitive` Construction is a Complement; whether the Adjunct reuses its
internal structure is for the landing to settle and record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Molten Note: "Molten Note deals damage to target creature equal to the amount
  of mana spent to cast this spell." (control without *to cast this spell* 1)
- Skyrider Elf: "Converge — This creature enters with a +1/+1 counter on it for
  each color of mana spent to cast it."
- Infuse with the Elements: "Converge — Put X +1/+1 counters on target creature,
  where X is the number of colors of mana spent to cast this spell."
- Scaled Nurturer: "When you spend this mana to cast a Dragon creature spell,
  you gain 2 life." (control "When you spend this mana, you gain 2 life." 1)
- Angel of Jubilation: "Players can't pay life or sacrifice creatures to cast
  spells or activate abilities." (control without the infinitival 1)

Composition check, not a gain claim before focusing-only: Piracy, "Spend this
mana only to cast spells."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The infinitival read as an Object or catenative Complement
   of a verb that does not select one, or as a relative Postmodifier of the
   noun, is a defect. A wrong analysis that parses is a defect, not a gain.
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

## Out of scope

- Finite passive *was spent* with a measured mass Subject (*If at least four
  mana was spent to cast it*): "If three or more mana was spent, draw a card."
  is 0 without any infinitival; note those faces.
- *Spend … on X* (*Spend only black mana on X.*), cost-additional quoted
  costs (Drought), and *in order to*.
- Focusing *only* itself: `english-v3-focusing-only`.

## Landing record

In addition to the standard record: the attachment of the purpose Adjunct per
host (clause, reduced passive Postmodifier) and its relation to
`ToInfinitive`; the *only to* faces measured with or without focusing-only on
the base; Reading counts of each witness; before/after counts stamped with
change ids; timings as integer ns and ns/B with host load and worker count.
