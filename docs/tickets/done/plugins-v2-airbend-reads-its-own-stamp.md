---
needs: []
---
**Airbend's body reads the airbending stamp its own wrapper has not yet
made.** Found 2026-10-05 at `plugins-v2-keyword-helper-additions`. Standard
constraints apply.

The loader wraps a keyword action's body as `Enact(Action("Airbend"), body,
actor)`, and `keyword_actions/airbend.ron`'s body reads its objects back as
`themVerbed(Action("Airbend"))` three times (the owner, the permission's
counterpart, and the "for as long as" condition). Inside the wrapper that
stamp does not exist yet, so the Lean card check refuses every reading,
`[anaphor (stamped (action "Airbend")) many 0]` three times, for the body as
it stands today and for the handoff spelling alike. No canon card airbends,
so nothing caught it. The probe "Airbend target creature." (an instant) was
used to find this; it is kept outside the repo.

The handoff spelling the owner approved is ready once the body has a
referent: `act(ownerOf(<the exiled objects>), establish(mayCastFrom(<them>,
exileZone, PayingInstead(mana([2]))), forAsLongAs(…)))` [CR#701.65a]. The fix
is to read "them" as what the body itself exiled (for example an exile that
stamps, read back by its own deed), not to change the checker; "each exiled
card's owner" may also need the group handoff to distribute per card.

Proof: a card or probe that airbends proves under `cargo xtask lean check`.

## Landing record

Measured on change `xnxwytkuyuus` (the claim's work, after `kata refresh`,
2026-10-08), over the claim `vmxputomllmo`. No coverage lock or grammar
construction is in play; no English or Lean source changed.

### PROVE

- **No silent loss.** `cargo xtask lean check`: canon 138/138, testing 7 → 8/8
  (the Airbend Probe is the eighth). `cargo xtask expansions` `diff -r`,
  claim vs tip: builtin 1519 declarations both, only
  `keyword_actions/airbend.expanded` differs; the testing plugin's two macro
  expansions are identical (the probe is a card, not a macro).
- **Structural laws.** `cargo test -p deckmaste_semantics_v2 -p xtask`: 490
  passed, 0 failed, 1 ignored (pre-existing). `cargo clippy -p
  deckmaste_semantics_v2 -p xtask --tests -- -D warnings`: clean. Lexicon
  export (`cargo xtask lexical --card-name "Black Lotus" --export`) is
  byte-identical with and without this change. `kata kanban check`: OK.
- **No word-naming.** No checker rule and no guard added.
- **Cites.** `cite check --list-noncompliant` empty; `cite check` 0 stale
  (16,391 citations); the audit of the diff from the claim read
  [CR#701.65a] against the probe's claim; the rule was already registered.

### DISCLOSE

- **Body.** `sequentially([exile(Param(0)), act(ownerOf(Param(0)),
  establish(mayCastFrom(Param(0), exileZone, PayingInstead(mana([2]))),
  forAsLongAs(matches(Param(0), inZone(exileZone)))))])`. The exile is the
  body's own deed-stamping `exile` keyword action; the loader writes its
  `Param(0)` as `firstMention("airbend.0", …)` and the three later ones as
  `laterMention`, and Lean's `moveIntro` records the binding the exile MOVED
  under that identity (`recordMoved`, `lean/Semantics/Check/Phrase.lean`), so
  each later mention is the exiled object, read with the parameter phrase's
  own number (§12.2).
- **Why not `theVerbed(exile, Many)`.** Tried first: the stamp resolves after
  the body's own exile, but a stamped pronoun fixes its number and `reaches`
  requires it to agree with the binding's. [CR#701.65a] airbends "one or more"
  objects: `theVerbed(exile, Many)` proved for "airbend up to two target
  creatures" and was refused `[anaphor (verbed (action "exile") none
  attributive) many 0]` ×3 for "airbend target creature"; `One` fails the
  other way. The identity read proves both (checked with both probes; the
  singular one is kept).
- **Probe** (`plugins_v2/testing/cards/Airbend Probe.ron`): {1}{U} instant,
  "Airbend target creature."
- **Deviations and additions.** The later mentions read by identity, not by
  the stamped pronoun the brief named (above). One testing card added; no
  helper, no test added or removed (restored 0, re-spelled 0, ignored 0,
  added 0, removed 0).
- **STOP (routed).** "For each card exiled this way … its owner may cast it"
  distributes per card. `act(ownerOf(Param(0)), …)` hands the permission to
  the owner of the exiled objects as one performer, which is exact for one
  object and for several with one owner, but not for several with different
  owners. The group handoff the brief named, `act(each(ownerOf(…)), …)`, does
  not exist: `each` takes a Predicate, and `eachOf` refuses a singular group,
  so no number-agnostic distributive performer is available. Routed to
  `semantics-v2-per-member-handoff` (planned); airbend.ron carries a STOP
  comment at the `ownerOf(Param(0))` line naming it.
