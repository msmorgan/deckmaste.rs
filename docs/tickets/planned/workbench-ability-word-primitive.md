---
needs: []
---
# Home the `AbilityWord` primitive, then transcribe the attested ability words

The shape is ruled — an ability word is `AbilityWord Name Ability`, a macro
over an ordinary ability, no engine change (`workbench-keyword-policy-audit-
and-ability-words`, which measured the inventory and stopped here). What is
not decided is **where the primitive lives**: no `AbilityWord` constructor
exists in `idris/src/**`, core `Ability` has no ability-word variant, and
`deckmaste_core::ron::kinds()` registers no such kind, so a `.ron` body
`AbilityWord(…)` cannot load. `aw-prefix-parsing` records the same gap. The
binder-unification log (ch. 36) gives it to the semantics-v2 Idris grammar;
`builtin-v2-macro-spelling-and-grammar` rules ability words out of
declaration-backed vocabulary. Decide the home, mint the primitive there, then
transcribe.

Measured inventory (supported corpus, lines/cards), all 61 [CR#207.2c] words
attested — top of the table: Landfall 129/180, Threshold 91/99, Delirium
69/73, Domain 51/54, Channel 38/37, Raid 36/45, Constellation 35/35, Heroic
34/44, Metalcraft 30/35, Morbid 28/29; the full table is in the done ticket's
"As landed" section. Attested but outside [CR#207.2c]: Corrupted 24/24, Will
of the Planeswalkers 1/5 — evidence for `ability-word-catalog-regen`'s
authority question. Scryfall labels with zero supported prints (do not add):
Covercast, Descend (bare), Hero's Reward, Kinfall, Landship, Legacy, Underdog;
"Start your engines!" is a keyword ability [CR#702.179], not an ability word.

## Acceptance

- The primitive's home is recorded (ADR or the rewrite ADR's deviation log),
  the primitive exists there, and every attested word is transcribed; none
  unattested is.
- Build/test gates of the chosen home PASS; `cargo xtask cite check` 0/0.

Standard constraints apply.
