---
needs: []
---
**Correct the bench's wrong printed frames, add the dropped printed abilities,
record every remaining omission in place, and re-spell the raw-core sites a
macro already covers.** Cleanroom review 2026-09-03, F3 plus the raw-core
census in F14. The gate from `workbench-corpus-frame-gate` names the frame
half exactly; this ticket drives its count to zero.

## Wrong frames (21)

Seven P/T boxes: Wormfang Manta `(6,6)` vs printed 6/1 (`Cards.idr:6964`),
Wall of Shards `(3,6)` vs 1/8 (`Cards.idr:14475`), Council of the Absolute
(`Cards.idr:6122`), Centaur of Attention (`Cards.idr:10007`), Soldevi Adnate
(`Cards.idr:11998`), Myr Prototype (`Cards.idr:14419`), Keeper of the Flame
(`Cards.idr:15341`).

Fourteen mana costs: Sugar Coat carries a white pip for `{2}{U}`
(`Cards.idr:12486`); Omniscience is one `{U}` short (`Cards.idr:10822`); Mox
Diamond spells `{0}` as `Nothing` (`Cards.idr:12949`) while Dark Sphere spells
it `Just []` (`Cards.idr:4617`) — pick one spelling for `{0}` and apply it
everywhere.

Six short type lines: Wall of Shards lacks `Snow`, Phyrexian Revoker lacks
`Phyrexian`.

## Dropped printed abilities (8)

Flashback ×3 (Daydream `Cards.idr:342`, Cackling Counterpart
`Cards.idr:3861`, Flaring Pain `Cards.idr:4197`), double strike (Char-Rumbler
`Cards.idr:943`), flash (Retro-Mutation `Cards.idr:15183`) — all five
provably spellable (`keywordCosting "Flashback" (Mana …)` typechecks). The
remaining three are blocked elsewhere and are **not** this ticket's to fix:
Wind Zendikon's dies trigger (`Cards.idr:3258`), Grievous Wound's damage
trigger (`Cards.idr:5242`, `workbench-binding-regressions`) and Chromatic
Armor's two sleight-counter lines (`Cards.idr:5531`,
`workbench-counter-kind-open`) — each gets an `Unspellable` twin or a doc line
naming the sentence and the blocking ticket, so it is visible from inside the
file.

`Unspellable` is used 0 times in `Cards.idr`, so today a dropped sentence and
a refused one are indistinguishable. Every omission the bench keeps must be
recorded in place.

## Raw core where a macro exists (102 sites, 71 definitions)

`HasType Land` 15, `KeywordAbility kw Nothing` 14, `HasType Creature` 13,
`ChangeLife w (Up/Down n)` 19, `HasStatus Untapped` 8, `Continuously (Gets …)
d` 7. `openTheVaults` (`Cards.idr:13424`) alone carries four;
`forceOfWill` (`Cards.idr:7763`) writes `Do (ChangeLife You (Down (Lit 1)))`
where `payLife You 1` is definitionally that term. Per the 2026-08-26
refinement of `workbench-mirrors-semantics-v2-structure`, the bench spells
through the macro where one exists — including the one raw `Enact` site.

Size: M.

The corpus-frame gate (`maybe/workbench-corpus-frame-gate`) is parked, so each frame is checked by hand against `data/derived/cards.jsonl` (`jq` by name) and the check is recorded per card in the landing record.

Done when: the build is 23/23 with 0 errors and 0 warnings; the corpus-frame
gate reports 0 mismatches; the five spellable keywords are on their cards; the
three blocked sentences each carry an `Unspellable` twin or a doc line naming
the sentence and its blocking ticket; the 102 raw-core sites read through
their macros with no witness deleted; the landing record gives the before and
after gate counts and the chosen `{0}` spelling. Standard constraints apply.

## As landed

Frames (23 wrong sites over 22 cards, 31 wrong fields; all checked by hand
against `data/derived/cards.jsonl`). Line numbers are post-change.

- Char-Rumbler (`Cards.idr:949`) — type line had no creature subtype; now
  `creatureType "Elemental"`.
- Harsh Sustenance (`:1847`) — was `[Sorcery]`; now `[Instant]`.
- Chainsaw (`:2489`) — cost was `{3}`; now `{1}{R}`.
- Mister Fantastic, Reed Richards (`:2571`) — cost was `{1}{W}{U}` and the
  subtypes were `Human Hero`; now `{3}{U}` and `Human Scientist Hero`.
- Repeated Reverberation (`:3990`) — cost was `{3}{R}{R}` and the type
  `[Sorcery]`; now `{2}{R}{R}` and `[Instant]`.
- Council of the Absolute (`:6176`) — box was `(2,3)`; now `(2,4)`.
- Wormfang Manta (`:7015`) — box was `(6,6)`; now `(6,1)`.
- Phyrexian Revoker (`:7428`) — subtypes were `Horror`; now
  `Phyrexian Horror`.
- Centaur of Attention (`:10062`) — cost was `{2}{G}{G}`, subtypes
  `Centaur Advisor`, box `(0,0)`; now `{3}{G}{G}`, `Centaur Performer`,
  `(3,3)`.
- Memory Plunder (`:10862`) — cost was `{1}{U}{B}`; now
  `{U/B}{U/B}{U/B}{U/B}`.
- Omniscience (`:10875`) — cost was one `{U}` short; now `{7}{U}{U}{U}`.
- Soldevi Adnate (`:12050`) — box was `(1,1)`; now `(1,2)`.
- Churning Eddy (`:12502`) — cost was `{4}{U}` and the type `[Instant]`; now
  `{3}{U}` and `[Sorcery]`.
- Sugar Coat (`:12538`) — cost carried a white pip; now `{2}{U}`.
- Mox Diamond, both witnesses (`:13001` `testMox1`, `:13028` `moxDiamond`) —
  cost was `Nothing`; now `Just []`.
- Myr Prototype (`:14469`) — box was `(3,3)`; now `(2,2)`.
- Wall of Shards (`:14525`) — supertypes were `[]` and the box `(3,6)`; now
  `[Snow]` and `(1,8)`.
- Upwelling (`:14683`) — cost was `{4}{G}`; now `{3}{G}`.
- Shimmerwilds Growth (`:14705`) — cost was `{G}`; now `{1}{G}`.
- Keeper of the Flame (`:15392`) — cost was `{1}{R}`, subtypes
  `Human Cleric`, box `(1,1)`; now `{R}{R}`, `Human Wizard`, `(1,2)`.
- Death or Glory (`:15608`) — cost was `{3}{W}{W}`; now `{4}{W}`.
- Indestructibility (`:15975`) — cost was `{3}`; now `{3}{W}`.

`{0}` is spelled `Just []`. `Card.cardCostOk` (`Card.idr:216–219`) accepts
`Nothing` for a Land and refuses `Just _` there, so `Nothing` is "no printed
mana cost" and `Just []` is a printed cost of zero symbols. Dark Sphere
(`:4663`) was therefore already right; Mox Diamond was wrong at both sites.
All 762 `Macros.card` sites now agree with the corpus on cost, supertypes,
card types, subtypes and the P/T box.

Dropped printed abilities (6 added, 0 residues among the five the ticket
called spellable):

- Daydream (`:344`) — added `Flashback {2}{W}`.
- Char-Rumbler (`:949`) — added `DoubleStrike`, first, as printed.
- Cackling Counterpart (`:3904`) — added `Flashback {5}{U}{U}`.
- Flaring Pain (`:4242`) — added `Flashback {R}`.
- Retro-Mutation (`:15233`) — added `Flash`, first, as printed.
- Wind Zendikon (`:3299`) — added "When enchanted land dies, return that card
  to its owner's hand" as
  `triggered When (Dies (AttachHost Enchanted (TypeW Land)))
  (move (That CardW) handZ)`. The ticket listed this one as blocked
  elsewhere; it is not, and it typechecks (see Deviations).

Omissions recorded in place:

- Chromatic Armor (`:5585`) carries a doc block naming its two omitted printed
  sentences ("This Aura enters with a sleight counter on it." and the `{X}`
  activated ability) and the blocking ticket. No `Sleight` lives in
  `Words.CounterKind`, so neither term can be written at all and an
  `Unspellable` twin is impossible — a pin needs a well-formed term with one
  open obligation, not an undefined name. Blocked on
  `workbench-counter-kind-open`.
- Grievous Wound (`:5288`) needed nothing: `workbench-binding-regressions`
  restored its damage trigger this morning, so the F3 entry is stale.

Raw core re-spelled through an existing macro: **109 sites**, plus 2 sites
already on a macro tightened to the cost macro. Breakdown —
`HasType Land` 15, `HasType Creature` 13, `HasType Enchantment` 6,
`HasType Artifact` 5, `HasStatus Untapped` 8, `HasStatus FaceDown` 3,
`IsSource` 2, `Not IsToken` 2, `ZoneAt Graveyard Bare` 2,
`ZoneAt Graveyard (PossessedBy (PlayerGroup AllPlayers))` 1,
`KeywordAbility kw Nothing Nothing` 14, `KeywordAbility "Bushido"
(Just (ParamNumber …)) Nothing` 1, `ChangeLife w (Up/Down a)` 21 (4 of them
further to `payLife`), `Continuously (Gets n p t) d` 7, `ThisTurn` in
duration position 5, `And [AnyPlayer, OtherThan You]` 2,
`KeywordCounter "Flying"` 2. The 2 tightenings are `mysticForge`
(`Cards.idr:6667`) and `boseijuMana` (`:14601`), both
`Do (Macros.losesLife You (Lit n))` in a cost, now `Macros.payLife You n`.

Residues (raw core the bench keeps because no macro spells it today; none is
this ticket's to add):

- `openTheVaults` (`:13476`) keeps its raw `Enact "Return"`. `Macros.returnTo`
  hard-codes an empty rider list and Open the Vaults needs
  `[Under (PossessorsOf OwnerAx Them)]`; a rider-taking `returnTo` is a new
  macro.
- `ZoneAt Library Bare` (`:11322`) — there is a `graveyardZ`, `handZ`,
  `exileZ`, `stackZ` and `commandZ` but no `libraryZ`.
- `ChangeLife w (Set a)` ×4 (`:1670, :1677, :1704, :1763`) — `gainsLife` and
  `losesLife` cover `Up`/`Down` only; there is no set-life macro.
- `Gains` 73, `Draw` 55, `SetStatus` 32, `Create` 27 raw sites are NOT in this
  ticket's census and were left alone. `Macros.draw` is one of the one-token
  forwarders `workbench-macro-discipline-2` is retiring, and
  `Macros.tap`/`untap` wrap `SetStatus` in an `Enact` keyword-action label —
  changing those is a meaning change, not a re-spelling.

## Landing record

Numbers before/after:

- modules 23/23 → 23/23, 0 Error and 0 Warning lines both.
- `Cards.idr` top-level definitions 1568 → 1568; no witness deleted.
- `Macros.card` sites 762 → 762; sites disagreeing with
  `data/derived/cards.jsonl` on cost / supertypes / types / subtypes / P/T:
  23 → 0.
- Bench abilities: +6 printed abilities on 6 cards.
- Raw-core sites with an existing macro: 109 → 0 for the shapes above; the 4
  residual shapes above have no macro.

Per-card frame check (hand-checked, `jq 'select(.name=="…")'` over
`data/derived/cards.jsonl`; every card `.supported == true`):

| Card | printed cost | printed P/T | printed supertypes | printed type line | corrected |
| --- | --- | --- | --- | --- | --- |
| Char-Rumbler | `{2}{R}{R}` | -1/3 | — | Creature — Elemental | subtype |
| Harsh Sustenance | `{1}{W}{B}` | — | — | Instant | card type |
| Chainsaw | `{1}{R}` | — | — | Artifact — Equipment | cost |
| Mister Fantastic, Reed Richards | `{3}{U}` | 2/4 | Legendary | Legendary Creature — Human Scientist Hero | cost, subtypes |
| Repeated Reverberation | `{2}{R}{R}` | — | — | Instant | cost, card type |
| Dark Sphere | `{0}` | — | — | Artifact | none (already `Just []`) |
| Council of the Absolute | `{2}{W}{U}` | 2/4 | — | Creature — Human Advisor | P/T |
| Wormfang Manta | `{5}{U}{U}` | 6/1 | — | Creature — Nightmare Fish Beast | P/T |
| Phyrexian Revoker | `{2}` | 2/1 | — | Artifact Creature — Phyrexian Horror | subtypes |
| Centaur of Attention | `{3}{G}{G}` | 3/3 | — | Creature — Centaur Performer | cost, subtypes, P/T |
| Memory Plunder | `{U/B}{U/B}{U/B}{U/B}` | — | — | Instant | cost |
| Omniscience | `{7}{U}{U}{U}` | — | — | Enchantment | cost |
| Soldevi Adnate | `{1}{B}` | 1/2 | — | Creature — Human Cleric | P/T |
| Churning Eddy | `{3}{U}` | — | — | Sorcery | cost, card type |
| Sugar Coat | `{2}{U}` | — | — | Enchantment — Aura | cost |
| Mox Diamond (×2 witnesses) | `{0}` | — | — | Artifact | cost |
| Myr Prototype | `{5}` | 2/2 | — | Artifact Creature — Myr | P/T |
| Wall of Shards | `{1}{W}` | 1/8 | Snow | Snow Creature — Wall | supertype, P/T |
| Upwelling | `{3}{G}` | — | — | Enchantment | cost |
| Shimmerwilds Growth | `{1}{G}` | — | — | Enchantment — Aura | cost |
| Keeper of the Flame | `{R}{R}` | 1/2 | — | Creature — Human Wizard | cost, subtypes, P/T |
| Death or Glory | `{4}{W}` | — | — | Sorcery | cost |
| Indestructibility | `{3}{W}` | — | — | Enchantment — Aura | cost |
| Daydream | `{W}` | — | — | Sorcery | frame ok; +Flashback {2}{W} |
| Cackling Counterpart | `{1}{U}{U}` | — | — | Instant | frame ok; +Flashback {5}{U}{U} |
| Flaring Pain | `{1}{R}` | — | — | Instant | frame ok; +Flashback {R} |
| Retro-Mutation | `{2}{U}` | — | — | Enchantment — Aura | frame ok; +Flash |
| Wind Zendikon | `{U}` | — | — | Enchantment — Aura | frame ok; +dies trigger |
| Chromatic Armor | `{1}{W}{U}` | — | — | Enchantment — Aura | frame ok; 2 sentences recorded |
| Grievous Wound | `{3}{B}{B}` | — | — | Enchantment — Aura | frame ok; nothing missing |

The whole bench was checked, not only these: a scratch comparator (in the
session scratchpad, not the repo) parsed the `name / cost / supers / line /
stats` arguments out of all 762 `Macros.card` sites and diffed them against
the corpus rows, and every row above was then read by hand against `jq`
output. The comparator reports 0 disagreements after the change.

Gates (foreground):

- `cd idris && rm -rf build && ./scripts/build` →
  `23/23: Building Cards (src/Cards.idr)`, no Error and no Warning lines.
- `cargo xtask cite check --list-noncompliant` →
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` →
  `checked 17752 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/bf.diff && cargo xtask cite audit --diff <
  /tmp/bf.diff` → `audited 0 citation site(s) — nothing selected`. The diff
  makes no CR claim, so `cite bless` was not run and `cr-citations.lock` is
  untouched.

Assurance counts: restored 0, re-spelled 109 (raw core to its macro) plus 31
frame fields, ignored 0, added 6 printed abilities and 1 doc block, removed 0.
No witness, pin or proof was deleted.

Deviations and additions:

- **Wind Zendikon's dies trigger was added, though the ticket listed it as
  blocked elsewhere.** The ticket names three sentences as "blocked elsewhere
  and not this ticket's to fix". Two of the three premises are stale: Grievous
  Wound's damage trigger was restored by `workbench-binding-regressions` this
  morning, and Wind Zendikon's dies trigger typechecks today (probed in a
  scratch module before landing, scratch deleted). Only Chromatic Armor is
  genuinely blocked. Adding a printed sentence that the compiler accepts is
  the ticket's own rule for the spellable half, so it was added rather than
  recorded as a residue.
- **The frame sweep found 23 wrong sites, not 21, and 31 wrong fields.** The
  extra sites are Chainsaw, Mister Fantastic, Repeated Reverberation, Churning
  Eddy, Upwelling, Shimmerwilds Growth, Death or Glory and Indestructibility;
  Dark Sphere, which F3 named, was already correct once `{0}` is settled as
  `Just []`. The count difference is because the whole bench was compared,
  not the 58 cards auditor B read.
- **The raw-core re-spell covers 109 sites, not 102**, and adds six shapes the
  census did not name (`HasType Artifact`/`Enchantment`, `HasStatus FaceDown`,
  `IsSource`, `Not IsToken`, `ZoneAt Graveyard Bare`/`…AllPlayers`, duration
  `ThisTurn`, `And [AnyPlayer, OtherThan You]`, `KeywordCounter "Flying"`,
  and the one parameterised `KeywordAbility "Bushido"`). Each has a macro in
  `Macros.idr` today and is the same defect the census names.
- **The `Enact` site was not re-spelled.** The ticket asks for "the one raw
  `Enact` site"; `Macros.returnTo` cannot carry Open the Vaults' `Under`
  rider, and the brief forbids adding macros. Recorded as a residue above.
- **Chromatic Armor's record is a doc block, not an `Unspellable` twin.** The
  ticket allows either; a pin is not available here because the refused terms
  cannot be written (`Sleight` is not a `CounterKind` constructor, so the term
  is an undefined name, not an unmet obligation). This is the only comment
  added to `Cards.idr`; every other witness keeps its one name line.
- Two sites already spelled `Do (Macros.losesLife You (Lit n))` in a cost were
  tightened to `Macros.payLife You n` — macro-to-macro, the same shape the
  ticket calls out on `forceOfWill`.

STOP taken: none. The two stale "blocked elsewhere" premises were resolved by
the ticket's own spellable/unspellable rule and by re-reading the sibling
ticket that landed this morning, not against a recorded ruling.
