---
needs: []
---
**Close the facts-table residues the labels worklist left.** Residue of
`workbench-facts-labels-worklist` (2026-09-04), which rowed every keyword
stub and made `cargo xtask facts labels` exit 0:

- **Prototype.** The one stub still unrowed: its `[Cost, Power, Toughness]`
  signature has no `KeywordParamShape` [CR#702.160a]. Since
  `workbench-prototype` models prototype as a layout (`Prototype (inner,
  alt)`), decide whether the stub is scoped out as a layout keyword (recorded
  reason in the check) or gets the shape; do not add a second prototype
  representation.
- **Designations.** `Harnessed`, `Level`, `Sector` and `Solved` want
  `Designation` constructors and `designationFacts` rows, or a recorded
  scope-out where the CR makes the designation a non-object property.
- **Gate columns.** The seven hand-kept columns (`counterEligible`, `regime`,
  `onPermanentCard`, `onSpellCard`, `paidCost`, `bodied`, `wantsModes`) want
  a home in `plugins/builtin_v2/macros/meta/KeywordAbility.ron` so the
  generator can derive them; this is a stub-schema change and may need its
  own ruling.
- **Act roles.** The 47 new `actFacts` rows carry default `agentRole`,
  `patientRole` and destination; author each as its deed gets spelled, or in
  one sweep from the CR entries.
- **`AtCasting` grantability.** Extort and increment took `AtCasting` on the
  prowess precedent (`keywordBodyFits` pairs a `SpellCast` body with it), and
  `grantSubjectFits` treats an `AtCasting` keyword as ungrantable to a
  battlefield permanent — pre-existing with prowess; decide whether the
  regime is the wrong axis for "functions while on the stack" versus "body
  triggers on casting".

Size: M. Done when: each bullet is landed or scoped out with its reason
printed by `cargo xtask facts labels`; the command still exits 0; build at
its module count. Standard constraints apply, including the RON-shaped
constraint.

## As landed

**Prototype: scoped out as a layout keyword, with the reason rewritten.** The
recorded reason no longer says "a new sort, which is design work beyond a row";
it says the prototype ability *is* the inset frame [CR#702.160a,718.1], so the
workbench spells it as the `Card` wrapper `Prototype (inner) (alt)` that
`workbench-prototype` landed, and a `keywordFacts` row carrying the stub's
`[Cost, Power, Toughness]` signature would be a second representation of the
same ability. No `KeywordParamShape` was added and no second prototype
representation exists. `facts labels` prints the new reason.

**Designations: all four stubs rowed; the exemption list is now empty.**
`Words.Designation` gained six constructors and `designationFacts` six rows —
`Harnessed` [CR#701.64b], `Level` [CR#716.2b], `Solved` [CR#719.3b] and the
three sector designations `AlphaSector`, `BetaSector`, `GammaSector`
[CR#702.158b], which that rule names as three ("The sector designations are
alpha sector, beta sector, and gamma sector") and which the `Sector` stub
reaches through the one-stub-to-many mapping the `DayNight → Day, Night`
precedent established. None of the four is a non-object property: each rule
says a *permanent* has the designation, so all six rows are
`HeldBy Object`, effectful, seeded on the battlefield. The seed card type is
`Nothing` for harnessed, level and solved (their rules say "permanent", with no
type), and `Just Creature` for the sectors, because space sculptor gives a
sector designation only to creatures [CR#702.158a,702.158c] — the `Goaded`
idiom [CR#701.15b]. `DESIGNATION_STUBS_EXEMPT` is `&[]`; `facts labels`
designations went 20 stubs / 16 rows with four recorded reasons to 22 / 22 with
none.

**Gate columns: STOP, recorded and not taken.** See the Landing record. The
printed reason in `facts labels` now carries the evidence rather than the bare
"declares no field".

**Act roles: the destination column is authored, the two role columns are
deferred under a printed rule.** Seven of the 47 rows carry the destination
their [CR#701] entry names: `Create` [CR#701.7a], `Manifest` [CR#701.40a],
`Cloak` [CR#701.58a], `Incubate` [CR#701.53a] and `Investigate`
[CR#701.16a,701.7a] to `Battlefield`; `Airbend` [CR#701.65a] and
`Collect Evidence` [CR#701.59a] to `Exile`. The remaining 40 entries name no
single destination (`Discover` exiles, then casts or draws, then bottoms the
rest [CR#701.57a]; `Forage` exiles or sacrifices [CR#701.61a]; most name no
move at all), so `Nothing` is the authored answer there, not a default.
`agentRole` and `patientRole` stay hand-kept per deed, with the rule printed by
`facts labels`; see the Landing record for why.

**`AtCasting` grantability: STOP, recorded and not taken.** `facts labels`
prints the axis question, the two readings, the three keywords that fall
between them and the printed card the conflation makes unspellable.

## Landing record

Numbers before → after: `Words.Designation` constructors 16 → 22;
`designationFacts` rows 16 → 22; `keywordFacts` 197 rows (unchanged);
`actFacts` 87 rows (unchanged), destinations authored 6 → 13; Idris modules
46 → 46; `xtask` unit tests 444 → 445. `facts labels` designations 20 stubs /
16 rows / 4 recorded reasons → 22 / 22 / 0; keyword abilities unchanged at 195
stubs / 197 rows with one recorded reason (Prototype, reworded) and three
row-side reasons. Coverage lock untouched; no construction added or retired;
`cr-citations.lock` unchanged (`cite bless` registered no new rule — every rule
newly cited from `crates/xtask/` sits in a path `cite-config.json` excludes).

Gates (all foreground):

- `cd idris && ./scripts/build` (clean `build/`) — `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 0 `Error`/`Warning` lines, `real 1m14.765s`
  (1m25.015s recorded by the previous round)
- `cargo xtask facts check` — `…/idris/src/Experimental/FactsGen.idr is up to
  date` (the generated module is byte-identical: no overlay row changed)
- `cargo xtask facts labels` — exit 0; four tables, three printed derivation
  rules (gate columns, role columns, regime axis)
- `cargo test -p xtask` — `test result: ok. 445 passed; 0 failed; 1 ignored`
- `cargo fmt --check` — exit 0; `cargo clippy -p xtask --all-targets` — 0
  warnings
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` — `checked 14191 citations against cr.txt
  (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` — `blessed 1420 rules`; the lock file is byte
  unchanged
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` — `audited 6 citation site(s)`; each rule's text read
  against the line citing it

Pin probes (foreground, `Experimental.ProbeTmp` typechecked and removed):

- Positive: `designationScope Solved = HeldBy Object`, `designationSeedZone
  Level = Just Battlefield`, `designationSeedType AlphaSector = Just Creature`,
  `So (designationGiven Harnessed)`, `So (AlphaSector /= BetaSector)`,
  `actDestOf "Create" = Just Battlefield` and `actDestOf "Airbend" = Just
  Exile` all elaborate.
- Mis-stated once each, all five refuse: `designationScope Solved = HeldByGame`
  (`Mismatch between: HeldByGame and HeldBy Object`), `designationSeedZone
  Level = Just Graveyard` (`Graveyard and Battlefield`), `designationSeedType
  AlphaSector = Just Artifact` (`Artifact and Creature`), `So (designationGiven
  CommanderD)` (`True and False`) and `actDestOf "Airbend" = Just Battlefield`
  (`Battlefield and Exile`).
- Every `Proofs*` module re-elaborated in the clean 46/46 build; no pin was
  added, deleted or re-spelled.

Assurance counts: restored 0; re-spelled 0; ignored with blockers 0; added 1
(`facts::tests::keyword_action_destinations_come_from_their_cr_entry`, and
`the_designation_map_reaches_the_idris_constructors` grew the six new
constructors and the `Sector` mapping); removed 0.

Deviations and additions:

- **Three sector constructors, not one.** [CR#702.158b] names three sector
  designations, so the stub maps to three constructors on the `DayNight`
  precedent rather than to one `Sector` marker that would lose which sector a
  creature is in [CR#702.158d,702.158e].
- **`designationIx` renumbered.** The six constructors were placed with the
  other object-held markers rather than appended, so the index function was
  renumbered 0..21. Nothing reads a specific index; `designationIx` feeds only
  the `Eq Designation` implementation, and `So (AlphaSector /= BetaSector)` was
  probed.
- **The destination half of the act-roles bullet landed, the role half did
  not.** Recorded below as the bullet's reason rather than as a STOP: the
  ticket's own bullet offers "author each as its deed gets spelled" as one of
  its two options, and that is the option taken.
- **Two new printed derivation rules** (`ROLE_COLUMNS`, `REGIME_AXIS`) beyond
  the ticket's letter, so that the two bullets left open print their reason
  from `facts labels` as the ticket's "done when" requires.

STOP 1 — **gate columns into the stub schema (bullet 3): not taken.**

- Question: can the seven hand-kept keyword gate columns (`counterEligible`,
  `regime`, `onPermanentCard`, `onSpellCard`, `paidCost`, `bodied`,
  `wantsModes`) move into `plugins/builtin_v2/macros/meta/KeywordAbility.ron`
  so the generator derives them?
- Evidence: that meta-macro emits `metadata: (spelling, grammar)`. Its
  consumer is `deckmaste_construction_core::macro_def::Metadata`, which is
  `#[serde(deny_unknown_fields)]` over exactly `spelling`, `grammar`,
  `noun_class` and `category`; the meta-macros are `include_str!`-embedded in
  that crate, and `Metadata` is what `deckmaste_english_v2::environment`
  reads. A gate field therefore changes the English-v2 metadata seam and, to
  carry any data, all 195 stubs under
  `plugins/builtin_v2/macros/stubs/keyword_abilities/` — both outside this
  ticket's paths and far outside "the facts overlay and its Idris consumers".
- Recommendation: keep the columns in xtask's overlay until a ticket that owns
  the English-v2 metadata seam adds a declaration-class field for them; that
  ticket, not this one, is where the 195 stubs get authored. The printed reason
  now carries this evidence.

STOP 2 — **the `AtCasting` regime axis (bullet 5): not taken.**

- Question: `KeywordFacts.regime` carries two different facts on one axis.
  `Effect.keywordBodyFits` reads `Just AtCasting` as "this keyword's triggered
  body keys on a spell cast" (it pairs the row with a `SpellCast` body event);
  `Effect.grantSubjectFits` reads it as "this keyword functions only while its
  object is on the stack" (`castingOnly` refuses granting the keyword to a
  battlefield permanent). Is `regime` the wrong axis for the first of those?
- Evidence: prowess, extort and increment are each defined as *a triggered
  ability* of a permanent [CR#702.108a,702.101a,702.191a], and an ability of a
  permanent functions while that permanent is on the battlefield [CR#113.6].
  They carry `Just AtCasting` only because `keywordBodyFits` demands it for
  their "whenever you cast" bodies. The consequence is not hypothetical:
  Bria, Riptide Rogue prints "Other creatures you control have prowess", and
  `grantSubjectFits` refuses that grant because the subject noun is on the
  battlefield and the ability's regime is `AtCasting`. Two vintage-legal,
  `supported` cards in `data/derived/cards.jsonl` grant prowess this way
  (Bria, Riptide Rogue; Narset, Enlightened Exile). Of the 49 rows carrying
  `Just AtCasting`, only those three name an ability that functions on the
  battlefield; the other 46 are cost, alternative-cast and cast-trigger
  keywords whose abilities really do function on the stack
  [CR#113.6,113.6d,113.6e].
- Recommendation: split the column — keep `regime` as the *body* axis that
  `keywordBodyFits` reads, and add a separate `functionsOnStack` (or move
  `abRegime` to read it) for the grantability question, so prowess is
  `regime = Just AtCasting, functionsOnStack = False`. That rewrites
  `Effect.abRegime` and `Effect.grantSubjectFits`, which
  `idris/src/Experimental/Effect.idr` owns and this ticket's paths exclude, and
  it wants a bench witness (Bria, Riptide Rogue) and a re-probe of every
  `AtCasting` row. It is one ticket of its own.

Residue (routed as future work, none in scope here):

- The 40 `actFacts` rows whose deed has no authored `agentRole`/`patientRole`;
  each is authored when a bench sentence spells a deontic clause over that
  deed.
- The seven keyword gate columns (STOP 1).
- The `regime` axis split and the prowess-grant witness (STOP 2).
