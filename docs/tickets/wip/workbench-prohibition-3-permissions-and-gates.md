# prohibition-3: the play permissions and the outcome gates

Sub-round 3 of [workbench-prohibition-and-permission-acts](workbench-prohibition-and-permission-acts.md)
(the umbrella — authoritative). Runs AFTER sub-round 1; may run parallel with
sub-round 2. Owns the umbrella's sections "The play permission's remaining
subjects, riders and complements" (seven narrow cells: the group possessor —
answered as a LibrarySlice question, not by loosening the riders; `Casts`'
source-zone slot — Melek; the face-down look-at rider — Keeper of the Lens
whole; Xanathar's non-agreeing possessive; the bare window on a permission —
Muldrotha's axis kept distinct from `PlayLimit`; the `ZoneCoherent` refusal —
answered inside the predicate vocabulary the same way or the difference
written down, Danitha whole; Haakon's exclusion clause) and "The game-outcome
gates" (the player-set count reader — 2 lines; the coordination arrives via
sub-round 1's design — bench the 2 gate cards through it; the partial-cause
SBA immunity — 9 lines, naming the causes IS the substance, [CR#104.3b..104.3e]),
plus routed: the ceilinged permissions ("up to N additional lands" 3 +
"block up to N additional" 1) and the play-permission "from among" SOURCE (97
occurrences name a MENTION; `MayPlay`'s `from` is a `ZoneExpr`).

Pins: all four permission pins still refuse afterward; companion blockers
(the alternative-cost "without paying its mana cost" family and the routed
per-card items) stay recorded, not built; counts are a prior session's —
re-measure before building. Standard constraints apply.

## As landed (2026-08-28)

Every count below was re-measured this round, line-level, over
`jq 'select(.supported)'`. Four of the ticket's own premises were wrong
and are corrected here rather than worked around.

### The grammar rows

| row | where | what it pays |
| --- | --- | --- |
| `VisibleThing` moved to `Phrase` and gained `VisibleObjects (n : Noun bs Object)` | `Phrase.idr`, `Effect.Visibility` | the face-down look-at rider |
| `PlayWindow.DuringEachOfYourTurns` + `playWindowOk` | `Events.idr` | the bare window, kept distinct from `PlayLimit` |
| `Casts` gained `from : Maybe (ZoneExpr …)` + a `PlayableFrom` gate | `Triggers.idr` | the cast watch's source phrase |
| `MayPlay` gained `exclusive : Bool` (+ a written-source gate) and a `PlayWindowOk` gate | `Effect.idr` | Haakon's exclusion clause |
| `MayPlayAdditionalLands`' bound moved from `quantLiteral` to `isNil (quantDelta q)` | `Effect.idr` | the one blocked land allowance |
| `StaticEffect.NoLossFrom` + `LoseCause = ZeroOrLessLife` | `Effect.idr`, `Words.idr` | the partial-cause SBA immunity |

### The seven cells

- **(a) The group possessor — FREE, and the premise was wrong.**
  `badSliceOfGroupPossessor` does not exist: it was retired in
  workbench-anaphora-e and `slicePossessorOk (PlayerGroup _)` has been
  `True` since. It was never this cell's blocker anyway — "Players play
  with the top card of their libraries revealed" (**3 lines**: Field of
  Dreams, Lantern of Insight, Wizened Snitches) is the visibility RIDER,
  whose `TopOfLibrary` complement names the position and takes *whose*
  from the subject, so a plural subject pluralises the zone word with
  nothing added. `World` was already a `Supertype` row. **Benched:**
  `fieldOfDreams` (World Enchantment) and `wizenedSnitches` whole,
  `revelation` whole (the hand surface, second World card),
  `lanternOfInsightRider`. The stale note on `playersTopCardSlice` —
  "the static that would consume it has no row" — is corrected in place.

- **(b) `Casts`' source zone — BUILT. 48 supported headers** write a
  source phrase; **29 positive** (hand 18, graveyard 10, library 1) and
  **19** the negative "from anywhere other than [a] hand", which wants a
  negative zone phrase and is NOT paid. **Benched: `melekIzzetParagon`
  whole.**

- **(c) The face-down look-at rider — BUILT. 3 static lines** (Keeper of
  the Lens, Found Footage, Lumbering Laundry) plus Lens of Clarity's
  coordination. The premise was wrong twice: `HasStatus FaceDown`
  already exists (`Words.StatusVal`), so no face-down predicate was
  minted; what was missing was the RIDER's complement, a closed
  two-row tag with no room for an object description. [CR#708.5] is the
  split the arm records — the place arms override a rule that hides by
  PLACE, the object arm one that hides by what the object IS.
  `visibilityOk` opens the reveal cell rather than refusing it on a
  count. **Benched: `keeperOfTheLens` whole, `lensOfClarity` whole**
  (the coordination of both surfaces in one sentence).

- **(d) Xanathar's non-agreeing possessive — RECORDED, 1 line.** The
  possessor slot alone lands nothing, and it does not fall out free:
  `TopOfLibrary` carries no possessor at all, so admitting one is a
  widening of the complement for a single sentence whose card also wants
  a play permission over another player's library and the mana half.

- **(e) The bare window — BUILT. 2 lines** (Muldrotha, Coram).
  `PlayWindow` gains a second arm; the axis stays distinct from
  `PlayLimit`, and `playWindowOk` refuses only the pairing that would
  spell one turn phrase twice ("once during each of your turns" is
  `PlayLimit`'s own word for the composite). **Benched:**
  `muldrothaLandWindow`. Neither card benches whole: Muldrotha's second
  conjunct wants "a permanent spell of each permanent type" (a
  distributive over card types) and Coram's wants a lookback inside a
  partitive.

- **(f) The `ZoneCoherent` refusal — the premise was wrong; nothing to
  build.** `And [spell, Or [HasSubtype Aura, HasSubtype Equipment]]`
  elaborates today and was probed green before any edit.
  `seedZone (HasSubtype _)` is `Nothing` — a subtype word presupposes a
  card TYPE and no zone — so the conjunction's zone question was never
  reached. That is the SAME answer the permission gave one level up
  (`complementLocates`: a word for what playing the object will make it
  locates nothing), which is what the ticket asked for. **Benched:
  `danithaNewBenaliasLight` whole.**

- **(g) Haakon's exclusion clause — BUILT, 1 line.** A `Bool` on the
  permission, gated to require a written source (with no source there is
  no "else"). [CR#601.3] makes casting depend on an allowing rule and
  [CR#302.1] is the default the line revokes. **Benched:
  `haakonStromgaldScourge` whole.**

### Routed items

- **The ceilinged land permissions — 2 of 3 were already writable.**
  "May play … additional land(s)" is **38 lines**; only **2** write a
  ceiling ("up to two", Journey of Discovery; "up to three", Summer
  Bloom) and both are literal `Range`s that always passed —
  `summerBloom` was already on the bench. The literal bound's stated
  reason was never "literals are what is printed" but "an amount bound
  written here would be announced nowhere", so the gate now asks exactly
  that, and the **1** blocked carrier passes: Nahiri's Lithoforming's
  "you may play X additional lands this turn" reads a letter its own
  cost opened [CR#107.3i]. **Benched: `nahiriExtraLands`.**
- **"Block up to N additional creatures" — RECORDED, not built.** The
  ceiling is **1 line** (Yare) inside a cell of **31**. Its carrier is
  sub-round 1's `Deontic`, which has no count slot for the "Block" deed;
  the RULED shape is a label-keyed slot, and adding one to `Deontic`
  while sub-round 2 is editing the same constructor is the wrong move.
  Cost: one slot on the carrier plus a `deedFacts` fact, and the 31
  lines land together.
- **The play-permission "from among" SOURCE — MEASURED ZERO; the
  premise was wrong.** "From among" is **474 supported lines**, of which
  **126** sit under a play or cast permission and **48** are not held by
  the alternative-cost family. None of them wants a mention-valued
  `from`: "you may cast spells from among them" is the COMPLEMENT's own
  partitive `SomeOf` ([CR#109.2a] locates a card-worded description by
  the zone the phrase states, and a partitive states a GROUP in that
  slot), so the mention carries the zone and `MayPlay`'s `from` stays
  unwritten. Probed green before any edit; **benched:
  `apexOfPowerCast`** (Apex of Power's first line). No widening landed
  and none is owed.

### The outcome gates

- **The player-set count — ALREADY PAID; the premise was wrong.** Both
  lines (**2**: Rampant Frogantua, Hot Pursuit) read
  `HappenedTo GameLoss ThisGame` at the player kind, which
  `rampantFrogantuaPump` has benched since an earlier round and which
  [CR#603.10f] states outright ("abilities that trigger when a player
  loses the game look back in time"). A standing-state predicate was
  drafted and then REMOVED: it would have been a second spelling of one
  sentence with no line behind it. **Benched:
  `twoOrMorePlayersHaveLost`** — the second line's `CountOf` over the
  reader that was already there.
- **The partial-cause immunity — BUILT. 7 lines, not 9** (Phyrexian
  Unlife, Lich, Lich's Tomb, Soul Echo, Transcendence, Pact Weapon,
  Marina Vendrell's Grimoire), all writing one cause. Its own
  `StaticEffect` row and NOT the deontic carrier, on the rules: a
  Phyrexian Unlife controller can still be made to lose by an effect
  [CR#104.3e], where `Deontic … Forbid ["LoseGame"]` (Lich's Mastery,
  Platinum Angel) stops every cause. `LoseCause` names [CR#104.3]'s
  three state-based causes as its frame and mints the **one** that is
  printed; [CR#104.3a]'s concession is the one thing a card may never
  override [CR#101.1]. **Benched: `phyrexianUnlifeImmunity`.**

### The `MayPlay` fold — DECLINED, with its cost

Sub-round 1's seam says `MayPlay` folds into the carrier once
`from`/`limit`/`window` become label-keyed slots. It does not land
cleanly with these cells and was not attempted, for two reasons: those
three are label-keyed FACTS about which deeds may carry them, but their
VALUES still need three slots on `Deontic`'s signature — and sub-round 2
is concurrently adding a complement slot to that same constructor, so
two parallel signature rewrites would collide. This round added a fourth
slot (`exclusive`) and two gates to `MayPlay` instead, which the fold
must carry over.

Remaining cost of the fold, measured: three `Maybe` slots plus
`exclusive` on `Deontic`; `PlaySource`/`playSourceOk`/`CastableTy`
re-expressed as deed-keyed gates (they are `Cast`/`Play`-specific
today); `deedFacts` facts for source, limit, window and exclusion; nine
`Macros` wrappers and five pins (`badPlayFromBattlefield`,
`badPlayFromStack`, `badCastALand`, `badPlayFromWrongZone`,
`badFlashPermissionOnPermanent`) re-expressed; `Visibility` and `MayPlayAdditionalLands` untouched.

### Companion blockers, still recorded and not built

"Without paying its mana cost" (the alternative-cost family — it holds
78 of the 126 from-among permission lines by itself), and the per-card
items the umbrella routes elsewhere.

### Gates

`idris/scripts/build` green, 23/23, 0 errors, 0 warnings. The four permission
pins (`badPlayFromBattlefield`, `badPlayFromStack`, `badCastALand`,
`badPlayFromWrongZone`) still refuse over the widened `MayPlay`, as do
`badFlashPermissionOnPermanent` and `badLookAtHandRider` over the
re-indexed `VisibleThing`. **Pin probe run** on the one gate this round
re-typed: flipping `visibilityOk LookAt WholeHand` to `True` makes
`badLookAtHandRider` fail with "not a valid impossible case", so it
still refuses for [CR#402.3] and not by accident of the re-indexing.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 0 stale;
`cite bless` added no rule; `jj diff --git | cargo xtask cite audit
--diff` read over 37 sites, one wrong-topic cite found and fixed
([CR#601.1a] is the "playing a card" terminology rule, not the default
hand permission — replaced with [CR#302.1]).
