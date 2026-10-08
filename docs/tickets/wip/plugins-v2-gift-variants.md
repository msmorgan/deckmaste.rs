---
needs: [semantics-v2-actor-handoff, plugins-v2-implicit-actor-spelling, semantics-v2-keyword-definition-by-card-class, lean-keyword-definition-regimes, semantics-v2-linked-choice-readback]
---
**Gift becomes one unspelled `gift(effect)` declaration plus six spelled
variants.** Split from `plugins-v2-keyword-body-defects` on 2026-10-05.
Standard constraints apply.

## Why

`keyword_abilities/gift.ron` declares `params: [Subject]` and carries a STOP:
"Gift a [something]" is a fixed additional cost plus a second ability whose
effect "is defined by the [something] listed" [CR#702.174a,702.174b], and a
noun-phrase parameter cannot supply that effect. So `gift` cannot be invoked.

## What is decided

Owner, 2026-10-05: "bare gift declaration must exist, I think, for the "gift
was promised" part and the keyword/nominal declarations for parsing. giftACard
= gift(draw(1)) with the "Gift a card" spelling on the macro or something like
that. bare `gift` is "private" by way of not having its own spelling and
therefore being unparseable".

- **Bare `gift(effect)` stays, unspelled.** It takes the effect as an
  instruction and writes both abilities: the optional additional cost "you may
  choose an opponent" [CR#702.174a], and the second ability, which hands the
  effect to the chosen player — `act(the(chosenPlayer), effect)` — under "if
  its gift cost was paid" on a permanent's enters trigger or an instant or
  sorcery's spell ability [CR#702.174b]. With no spelling it cannot be parsed.
  It is what "the gift was promised" [CR#702.174k] and "gives a gift"
  [CR#702.174c] name.
- **Six spelled variants**, one declaration each, each `gift(<effect>)` with
  its own spelling [CR#702.174d..702.174i]:

  | Macro | Spelling | Effect (performed by the chosen player) |
  |---|---|---|
  | `giftAFood` | "Gift a Food" | creates a Food token |
  | `giftACard` | "Gift a card" | draws a card: `gift(draw(1))` |
  | `giftATappedFish` | "Gift a tapped Fish" | creates a tapped 1/1 blue Fish creature token |
  | `giftAnExtraTurn` | "Gift an extra turn" | takes an extra turn after this one |
  | `giftATreasure` | "Gift a Treasure" | creates a Treasure token |
  | `giftAnOctopus` | "Gift an Octopus" | creates an 8/8 blue Octopus creature token |

- **A variant carries the "Gift" label** through a pass-through field, so its
  body IS `gift(draw(1))` under that label with its own spelling. Owner,
  2026-10-05: "yes to 2". (Without it the loader labels `giftACard` a keyword
  of its own, "GiftACard", with no facts row — see F4.)
- **Decided 2026-10-06:** gift's permanent and spell forms are one
  definition holding both halves, each guarded by the card's class with the
  existing conditional forms, not a new selector device
  (`semantics-v2-keyword-definition-by-card-class`).
- **Three model gaps, each its own ticket, in this order** (decided
  2026-10-06): these are three gaps in the model that gift exposed, each its
  own ticket.
  1. `semantics-v2-keyword-definition-by-card-class`: the class guard, and a
     spell ability [CR#113.3a] admitted in a keyword definition.
  2. `semantics-v2-linked-choice-readback`: the chosen player read back
     through a paid-cost facet.
  3. `lean-keyword-definition-regimes`: each part checked against its own
     regime.

  Then gift. No stub lands before them: a stub body would be a negative
  oracle.
- **Excluded:** "Gift a Rhystic Study" (Archival Whorl) is not Vintage-legal.

## Stopped 2026-10-05

The first attempt wrote the faithful body and ran the checker on probe cards;
it committed nothing and the ticket returned to `planned/` (owner: "yes to
3"). Findings:

- **F1** — a keyword definition is one fixed list with no per-class part, and
  gift's spell form is a spell ability, which `Ability.category`,
  `AbilityCategory` and the facts generator all refuse [CR#702.174b,113.3a].
  → `semantics-v2-keyword-definition-by-card-class`.
- **F2** — the permanent form's enters trigger fails the keyword-regime law
  (gift's row is `atCasting`; an enters event has no regime), the law already
  blocking Offspring and Squad. → `lean-keyword-definition-regimes`.
- **F3** — "the chosen player" cannot be read back: the opponent chosen as the
  additional cost is out of the second ability's scope (`choiceRef theChoice
  player 0`), and `PaidFacet` reads no player [CR#607.2d].
  → `semantics-v2-linked-choice-readback`.
- **F4** — a variant `giftACard` is labelled "GiftACard" by the loader
  (`keyword_ability_label`, `crates/deckmaste_semantics_v2/src/keywords.rs`)
  with no facts row; writing `body: [gift(draw(1))]` nests a keyword inside a
  keyword, which has no category and is refused. Answered by the decided
  pass-through label (step 2).
- **F5** — `Shape::from_params` (`crates/xtask/src/facts.rs` ~L102) has no case
  for an `Instruction` parameter, and `forwarded_keyword_params` rejects one
  unless the file writes `keyword_params: []`. Work step 1.

## The work

Blocked steps name their gap ticket; the rest can start once the existing
needs land.

1. **Let a keyword declaration omit `spelling`.** Decided 2026-10-05: the
   keyword declaration format allows a missing `spelling` for a declaration
   that is deliberately unparseable (the bare `gift`). Owner: "yes". Make
   `spelling` optional in the `KeywordAbility` meta-macro
   (`macros/meta/KeywordAbility.ron`) and the declaration schema, keeping the
   keyword and nominal declarations the parser needs. Then retype `gift`'s
   parameter to an instruction and remove its `spelling`. F5 belongs here:
   the generator maps gift's `Instruction` parameter to no keyword argument —
   `gift.ron` writes `keyword_params: []` (or `forwarded_keyword_params`
   learns to forward an `Instruction` as nothing, as it does `Ability`), and
   `Shape::from_params` admits the signature as `Nothing`.
2. **The pass-through label** (decided, F4): a variant declaration names the
   keyword whose label it carries, so `giftACard`'s body is `gift(draw(1))`
   labelled "Gift" and the facts row is gift's.
3. **Write gift's body.** Blocked: the per-class second ability on
   `semantics-v2-keyword-definition-by-card-class` (F1); the permanent form's
   enters trigger on `lean-keyword-definition-regimes` (F2); the chosen
   player as performer on `semantics-v2-linked-choice-readback` (F3).
4. Add the six variants. Each effect is written in the implicit-actor spelling
   (`plugins-v2-implicit-actor-spelling`); add any token or extra-turn helper
   the effects need and list it. Needs steps 2 and 3.
5. Check the variants in Lean (a pin each, or `lean check` over a canon card
   per variant if one is added) and that the English side still reads every
   printed "Gift a …" line in the corpus. Pins cover both classes for "Gift a
   card" and "Gift a tapped Fish".

## Proof

`cargo xtask lean check` passes with a pin per variant; `cargo xtask
expansions` shows `gift` and the six new declarations and nothing else.

## Out of scope

The "gives a gift" trigger [CR#702.174c] beyond what the bare declaration
needs to be nameable.

## Landing record

Measured on change `untszuxmsxvn` (tip of the claim after `kata refresh`,
2026-10-08), over base `qxwvqpox`. No coverage lock is in play (the English
census below is the evidence); construction count unchanged (no grammar
construction added).

### PROVE

- **No silent loss.** `cargo xtask lean check`: canon 134 → 138/138, testing
  7/7 (132 seconds). `cargo xtask lean definitions` 553/553 (73 counters, 461
  subtypes, 19 designations). `lake build --wfail` 88 jobs, every existing pin
  keeps its outcome. `cargo xtask english-v3 --all` before (base) and after:
  32,828 faces, 0 issues, 0 internal failures both times; no face lost or
  changed its readings digest; one gained (below).
- **Structural laws.** `cargo xtask facts check`: up to date. Gift's row now
  declares `definition := [⟨.static, some .atCasting⟩, ⟨.triggered, none⟩,
  ⟨.spell, some .atResolution⟩]` and `argumentSchemas := [[]]` (was
  `[[⟨.subject, none⟩]]`), regime summary `.atCasting` unchanged; the Idris row's
  `SubjectParam` became `NoParam`. `cargo xtask expansions` `diff -r`
  before/after: builtin gains `gift` (was skipped) and the six variants (1509 →
  1515 declarations), nothing else; canon and testing identical.
- **No word-naming.** The class guards read card types; the variant label is a
  declared field; the facts generator skips a declaration by its declared
  `label`, never by name.
- **No new game-rule refusal** (charter, §7): the only checks added are
  load-time shape checks on the new `label` field.

### DISCLOSE

- **Gift** (`keyword_abilities/gift.ron`): `params: [Instruction]`, no
  spelling, grammar kept for the keyword and "the gift was promised". Body:
  `static(addedCost(perform(choose(a(opponent))), true))`; an enters trigger
  `triggeredIf(enters(thisPermanent), matches(this, or([hasType(Artifact), …
  hasType(Planeswalker)])), doIf(keywordCostPaid(gift, this),
  act(chosenAsPaid(ByKeyword(gift), this), Param(0))))`; and
  `spell(doIf(matches(this, instantOrSorcery), <same doIf>))`. The choice cost
  is the existing `Cost.perform(choose(…))` form (Lean bench precedent in
  `Cards/Choice.lean`). The permanent guard is the permanent-type disjunction,
  not `permanentCard`, so it holds of a token copy.
- **Variants**: `giftACard` = `gift(draw(1))`; `giftAnExtraTurn` =
  `gift(extraTurn(1))`; `giftAnOctopus` =
  `gift(createToken(creatureToken(8, 8, [Blue], [octopus])))`;
  `giftATappedFish` = `gift(createObject(1, Token(spec:
  written(creatureToken(1, 1, [Blue], [fish])), riders: [EntersAs(Tapped)])))`;
  `giftAFood` and `giftATreasure` = `gift(createToken(…))` with the Food
  [CR#111.10b] and Treasure [CR#111.10a] characteristics written inline. Each
  writes `label: gift`; spellings "gift a card", "gift a Food", "gift a tapped
  Fish", "gift a Treasure", "gift an extra turn", "gift an Octopus".
- **Cards** (canon): Scrapshooter (creature, Gift a card), Long River's Pull
  (instant, Gift a card), Starforged Sword (artifact, Gift a tapped Fish), Into
  the Flood Maw (instant, Gift a tapped Fish). Lean pins
  `Proofs/GiftVariants.lean` (5): both variants fit the registry gift row; each
  on a permanent card and an instant; twins refused (spell half under the
  permanent guard; an unlinked chosen player).
- **English, newly covered**: Octomancer, no → multiple (17 readings); its
  keyword line reads as `BareKeyword` over the new `giftAnOctopus` lexeme, a
  correct analysis; the multiplicity is the existing second line's.
- **Lexicon**: six new keyword lexemes (`lexeme:keyword_ability/giftACard`,
  `giftAFood`, `giftATappedFish`, `giftATreasure`, `giftAnExtraTurn`,
  `giftAnOctopus`, each `KeywordParameterClass` Nullary), and gift's
  `KeywordParameterClass` Subject → Instruction. The Black Lotus and
  Scrapshooter exports differ from the base by exactly those lines.
- **Findings.** F1, F2, F3: answered by their done tickets and used as decided.
  F4: answered here by the `label` field (meta `KeywordAbility.ron`): a
  variant's term is its one invocation of the labelled keyword, its label macro
  reads `Named("gift")`, and the facts generator and `facts labels` skip it.
  F5: answered here: an `Instruction` parameter forwards as nothing,
  `Shape::from_params(["Instruction"])` is `Nothing`, the Lean schema drops it,
  and construction_core classes it `Unsupported(Instruction)`.
- **Deviations and additions.** `Metadata.spelling` is `Option<String>`; only a
  keyword ability may omit it. `UnsupportedKeywordParameterClass::Instruction`
  is new, and the English v3 `KeywordParameterClass` feature declaration does
  not list it: no construction requires it, so bare gift heads no keyword
  payload (it never covered a face as `Subject`); the census reports 0 issues.
  Spellings take the lowercase keyword head ("gift a card"), not the table's
  capitalized line form. "Sacrifice this token" is written `thisArtifact`. No
  token or extra-turn helper was added. Glossary: **Keyword Ability** amended
  (spelled forms carry the keyword's name). `lean/CONTRACTS.md` and three pin
  file headers updated (comments). ADR §12 count unchanged.
- **STOPs.** None.

### Assurance counts

Restored 0; re-spelled 5 (`nursery_and_graduated_sources_use_the_ordinary_macro_reader`,
`builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`,
`every_keyword_declaration_pins_its_template`,
`every_keyword_template_is_consistent_with_its_params_and_grammar`, and
`a_keyword_declaration_builds_its_wrapper`, whose gift `Subject` forwarding
became casualty's `Amount`); ignored 0; added 8 (3 Rust tests, 5 Lean pins;
three existing tests gained gift assertions); removed 0.

### Tests

`cargo test -p` and `cargo clippy -p <crate> --tests -- -D warnings` for
deckmaste_semantics_v2, deckmaste_construction_core, xtask and
deckmaste_lexical_source: all pass, clippy clean; `cargo test -p
deckmaste_english_v3` passes (it reads the changed lexicon). Cite checks: 0
non-compliant, 0 stale after `bless` registered [CR#702.174m,702.17a], and the
diff from the claim audited (36 sites). `kata kanban check` OK.

### REPORT

`cargo xtask english-v3 --all` (dev profile, 8 workers): 147,611 ms corpus wall at
host load 8.4, 423,723 ns/B, against the 16.26-second ceiling; the base measured
159,159 ms, 447,168 ns/B at load 4.0. `lean check` 132 seconds.
