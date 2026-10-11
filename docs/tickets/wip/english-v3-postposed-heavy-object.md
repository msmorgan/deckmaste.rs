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

Further owner rulings:

- Owner principle (2026-10-10) supersedes HO3: license the natural class of
  declared frames whose Object is directly followed by a marked Complement,
  on by default, with no per-verb declaration.
- HO8 (orchestrator, 2026-10-11) amends HO2 and replaces HO7: require the
  Object’s declared internal dependent structure or NP coordination. Export
  the Nominal’s existing dependent-presence feature generally through NP
  if needed; never test words or count tokens. A light attested witness or
  a remaining wrong covered-face increase requires a STOP.
- W9-A: one route for the new construction, preserving independent
  ambiguity elsewhere in the sentence.

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
300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared with
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

1. **Census delta** — Claim-parent semantics `pmvvruok` (BEFORE measured in workspace `kxslwtpn`) → `kxslwtpn`: 3,463 → 3,489 covered faces; 162,919 → 164,533 Readings. Lost none; gained 26; decreases 0; increases 0; digest-only 0 (no classes or lines); unchanged 3,817. Gains: Engulf the Shore; Faith's Reward; Fell Shepherd; Garna, the Bloodflame; Gerrard's Hourglass Pendant; Gerrard, Weatherlight Hero; Gleancrawler; Magus of the Jar; Memory Jar; Moira, Urborg Haunt; Myojin of Grim Betrayal; No Rest for the Wicked; Noetic Scales; Parallax Nexus; Parallax Tide; Parallax Wave; Profaner of the Dead; Remove Enchantments; Scrap Trawler; Second Sunrise; Shadow of the Grave; Squirming Emergence; Storrev, Devkarin Lich; Supper for Spiders; Thrilling Encore; Twilight Shepherd. Same 3,843-identity manifest for BEFORE/AFTER: declared morphological forms of attach, deal, distribute, divide, exchange, find, move, pair, put, remove, return, search, separate, share and shuffle followed by marked Complements, all Return faces and the ticket’s Exile reach, plus 300 covered random faces (seed 20261008). `english-v3 diff` evidence: `target/english-v3/english-v3-postposed-heavy-object/diff-ho8-optimized.json`. HO8 explicitly retained this manifest for the general NP exports.
2. **Witnesses** — All were 0 on claim parent `pmvvruok`; on `kxslwtpn`, witness/control counts and qualifying Object structure: Faith’s Reward 18/9 (graveyard PP + relative); Shadow of the Grave 6/4 (graveyard PP + relative); Engulf the Shore 5/5 (with-PP); Moira, Urborg Haunt 117/45 (graveyard PP + relative); Twilight Shepherd 6/3 (graveyard PP + relative); simple Return 3/2 (graveyard PP); Noetic Scales 11/14 (with-PP); Gerrard’s Hourglass Pendant constituent 36/18 (graveyard PP + relative; tapped remains Depictive); Put constituent 46/30 (graveyard PP + relative). The two source-bearing default Return controls remain 1 each. Independent ambiguity is preserved under W9-A; counts are not test assertions. No attested witness is light under HO8.
3. **Spot checks** — On `kxslwtpn`, Faith’s Reward, Shadow of the Grave, Engulf the Shore, Moira, Twilight Shepherd, Gerrard’s Hourglass Pendant, Noetic Scales and Second Sunrise select Return’s frame with its Object postposed after the destination; Thrilling Encore and Supper for Spiders select Put’s frame with its Object after the battlefield/control Complement. All ten retained probes have zero issues and the intended postposed Object. All 26 provisional gains remain gains; none read only through the retired light-Object defect.
4. **Deviations and STOPs** — Owner principle supersedes HO3: the alternation licenses the declared frame shape, with no verb list. HO8 amends HO2 and replaces HO7. Initial STOP: Agitator Ant, Fangs of Kalonia, Melira, the Living Cure, Ravenous Squirrel, Ripples of Undeath, Spectacular Pileup and Volcanic Spray gained wrong manner/temporal Objects. The representative active route was Have(Preterite; Object=a counter; Complement=BarePredicate(Put(Plain; Object=this way; LocativeComplement=on it))). HO7 could not prune the active causative/preterite routes; HO8 resolves this through declared structural requirements. CGEL Ch. 16 §4, p. 1382: “there is a tendency or preference for heavy constituents to appear late in the clause”; p. 1383 also permits some light discourse-new NPs. The stricter structural proxy is HO8’s project ruling. Added compiler directive `frame_postposing`, canonical-slot `FrameValue::PostposedObject`, and `HeavyObject` frame-category requirement. General `NominalDependentPresent` exports now reach NP, its series/coordination and AccusativePhrase; nominal/NP wrappers project it, and partitive/supplement/coordination structure exports Yes. No existing Construction or frame was deleted, no default route was retired, and no claim-parent Reading was removed. Return’s existing tapped role is Depictive, retained rather than relabelled Resultative. Exile declares no source-bearing frame, so no new Exile frame was added. The added frame variants exposed a genuine chart resource regression in the existing recipient-cluster test. Prediction now checks the same declared first lexical transition as scanning before allocating an impossible start; the resource bound and complete Reading-set assertion remain unchanged. Glossary gaps filled: Heavy Object and Postposed Object.
5. **Tests** — All passed on `kxslwtpn`: `cargo test -p deckmaste_english_v3` (1,091 passed, one inherited ignore); `cargo test -p deckmaste_construction_v3_core` (40 passed); `cargo test -p deckmaste_lexical_source`; `cargo test -p deckmaste_construction_v3`. The targeted `cargo test -p deckmaste_english_v3 --test suite_5 number_transparency::` and `cargo test -p deckmaste_english_v3 --test suite_6 selected_complement_clusters::` passed after their fixes. Clippy passed: `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`; `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`; `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`; `cargo clippy -p deckmaste_construction_v3 --tests -- -D warnings`. Added 7 English integration tests in `postposed_heavy_object`, registered once in `suite_3`, and 1 compiler validation test. Restored 0; re-spelled 2; newly ignored 0; added 8; removed 0. Re-spelled `number_transparency::attested_ordinary_number_constituents_keep_singular_concord` and `number_transparency::transparent_quantification_rejects_singular_obliques_and_preserves_other_heads`: independent expected controls now have of-Complements, so their complete Summaries expect `NominalDependentPresent = Yes`; full value equality, the original card constituents, and singular-concord/rejected-plural assertions remain. Corrected the new Shuffle fixture to its declared keyword-action owner. Independent composition checks both roundtrip laws and the existing SourceFreeObject admission requirement; passive fragments from Agitator Ant and Melira compare complete Reading sets by value; a light active postposed pronoun has no Reading. The one inherited ignored test, `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`, retains its explicit `english-v3-mana-symbol-verbless-subject` MS4 blocker (governed Mana Phrase Subject route). No Lean/plugin changes or Lean checks. Citation checks: 0 noncompliant, 0 stale; diff audit selected 0 added CR citation sites, so no new rule registrations required blessing.
6. **Timing** — `kxslwtpn`: bounded census 60.25 s under `/usr/bin/time -v`, six workers, zero samples per face.
