---
needs: []
---
# Read a postposed heavy Object after the destination: Return to your hand all creature cards in your graveyard that were put there from the battlefield this turn.

## Why

*Return X to your hand* reads; with a long Object the card prints the
destination first, and that order reads nowhere. Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:return to` 17
sole-blocker faces. `cargo xtask english-v3 grep -i`:
`\breturn(s)? to (your|their|its) (owner.?s.? )?hands? (all|each|target|…)\b`
finds **16** faces and `\breturn(s)? to the battlefield (tapped )?(under
[a-z]+ control )?(all|each|target|…)\b` **8**, all 24 unread, each line the
only failing line of its face (an upper bound: Scrap Trawler's *with lesser
mana value* and a few long Objects have a second cause). The postposed
Object is *all … cards in your graveyard that were put there from the
battlefield this turn* or similar in most; *target … card in your graveyard
that/with …* in at least 4; *each creature that player controls with power greater
than …* in 1. The same order after *put onto the battlefield* has 3 unread
faces.

## Goal

A verb's Object reads after its *to* destination Complement when the frame
declares both (*return to your hand <Object>*, *return to the battlefield
<Object>*), as a postposed Object of the same frame, with byte-exact
realisation.

## Analysis

CGEL Ch. 4 §4.1(d), p. 247, [4ii]: "a. *I returned the books to Jo.* b. *I
returned to Jo all the books I'd borrowed.*"; "In [iib] an O that is heavy
is placed in end position … in [iia], by contrast, the O is not heavy enough
and must precede the PP". Ch. 16 §4, pp. 1382–1383: "An element is said to
be postposed when it appears to the right of its basic position"; the
postposed elements include "direct object"; "there is a tendency or
preference for heavy constituents to appear late in the clause". Those back
the order as postposing of the Object over a PP Complement, and the Object
as still the Object. How the grammar licenses it is a project ruling.

Existing declarations: `Return`'s frames in
`crates/deckmaste_lexical_source/lexicon/verbs.ron` (`Predicate([Argument(
Object: SourceFreeObject), Lex(Preposition, To), ObjectNounPhrase, …])` and
its source-bearing and *under … control* variants), consumed in order by
`selected_frame(head, Predicate)` in the verb-phrase Constructions
(`crates/deckmaste_english_v3/src/declarations.rs`). No frame or
Construction places the Object after the *to* phrase.

Orchestrator rulings 2026-10-10:

- HO1. The postposed order is an alternant of the declared frame, not a new
  verb sense and not a new clause Construction. Preferred: a general
  ordering alternation in the frame consumer that lets a frame's Object be
  realized after its directly following marked Complement, licensed by a
  feature the frame (or verb) declares in data. Permitted fallback if the
  consumer cannot reorder: declare the postposed alternant frames for
  *return* in `verbs.ron`, marked with the same declared feature, so the
  Object keeps its relation.
- HO2. The licence reads the declared frame feature. No test on *return*,
  *to*, *hand* or *battlefield*, and no weight measure: CGEL's heaviness
  condition is a preference, and no check is added whose only job is to
  block a short postposed Object (*Return to your hand it* is unattested and
  need not be blocked).
- HO3. Declare the alternant only where attested: *return* (24 faces). *put
  onto the battlefield <Object>* (3 faces) is in scope if the same
  declaration reads it; otherwise list it and leave it.
- HO4. The resultative and control dependents keep their frame positions:
  *Return to the battlefield tapped all …* (Gerrard's Hourglass Pendant) and
  *return to its owner's hand each creature …* (Noetic Scales) are witnesses
  for the alternant composing with what the frame already allows.
- HO5. One analysis per string. The twins to avoid: (a) the postposed Object
  read as a second Complement of *to* or as an apposition to *your hand*;
  (b) *Return target creature card from your graveyard to the battlefield.*
  and every other default-order sentence keep exactly their Readings (the
  alternant adds no Reading where the Object precedes); (c) an intransitive
  *return to your hand* followed by a stranded NP.
- HO6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: in default order the alternant cannot apply. Report any
  increase with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category sentence --readings 3`): witness 0; control = the same sentence
in default order (3+ = enumeration capped). Re-probe on the claim parent
first.

- Faith's Reward: "Return to the battlefield all permanent cards in your
  graveyard that were put there from the battlefield this turn." (control
  3+)
- Shadow of the Grave: "Return to your hand all cards in your graveyard that
  you cycled or discarded this turn." (control 3+)
- Engulf the Shore: "Return to their owners' hands all creatures with
  toughness less than or equal to the number of Islands you control."
  (control 3+)
- Moira, Urborg Haunt: "Whenever Moira deals combat damage to a player,
  return to the battlefield target creature card in your graveyard that was
  put there from the battlefield this turn." (main clause in default order
  3+)
- Twilight Shepherd: "When this creature enters, return to your hand all
  cards in your graveyard that were put there from the battlefield this
  turn."

Also 0 today: "Return to your hand all creature cards in your graveyard."
(default order 2). Controls that must not change: "Return target creature
card from your graveyard to the battlefield." 1; "Return this card from your
graveyard to your hand." 1; "Return all creature cards in your graveyard to
your hand." 2; "Put all creature cards in all graveyards that were put there
from the battlefield this turn onto the battlefield under your control." 3+.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(return|put|exile)s? (to|onto|into|from|on)\b` (563
faces) plus every face with *return* if HO1's general alternation is chosen
(count it on the claim parent), plus 300 random covered controls (`--random
300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared with
`cargo xtask english-v3 diff` against the claim parent. Zero lost; HO6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, once,
at the end: a rebuild after a grammar edit takes about 2–3 minutes and the
six test binaries about 75 seconds, so batch edits; lexicon data loads at
runtime. Tests assert the role (the postposed NP as the frame's Object, the
*to* phrase as its destination Complement, the HO5 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only. Glossary: add a term for the postposed Object (CGEL
Ch. 16 §4, p. 1382) in the landing.

## Out of scope

- *with lesser mana value* (Scrap Trawler):
  `english-v3-attributive-scalar-comparative`.
- Postposing of Subjects or predicative Complements; preposing.
- Remove Enchantments' three-way Object coordination beyond what
  coordination already reads.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
