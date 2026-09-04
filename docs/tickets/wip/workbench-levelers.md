---
needs: []
---
**Give the `Card` sort a leveler wrapper: `Leveler (inner : CardFace) (bands :
List LevelBand)` [CR#711.2a].** Fresh workbench review 2026-09-03, R3,
resolved by ruling.

**Ruling (settled 2026-09-03): layout features are in the workbench's remit,
and the leveler shape is an inner-face wrapper.** `Card.Card` (`Card.idr:365`)
gains a constructor beside `SingleFaced`/`Transforming`/`Adventurer` that
wraps one `CardFace` rather than pairing two, because a leveler has one face
whose printed box and text are replaced band by band. A band is a level range,
a printed box, and the abilities that band grants — `LevelBand (range :
LevelRange) (box : PrintedBox) (text : AbilitySeq …)`. The lowering shape is
`Layout::Leveler { inner, bands }`.

25 supported cards print a level-up box (e.g. Brimstone Mage); today none of
them has a spelling, because `CardFace.box : Maybe PrintedBox` is one box and
`CardFace.text` one sequence.

Design points to settle in the round, not before: whether the range is
`(from, Maybe to)` or a closed pair with an open last band; whether the
level-up activated ability is a band-level ability or an ability on `inner`
(it is printed on the base face, so `inner`); and which face laws
(`Card.FaceLaws`) apply to the bands — a band has no name, cost or type line
of its own.

Every slot is positional and required, and any optional content arrives with
the wrapping macro that spells it, per
`docs/decisions/card-authoring-binds-no-implicits.md`.

Size: M.

Done when: Brimstone Mage is a typechecking bench witness with all its bands,
its level-up ability reading from `inner`; a second leveler with a different
band count is a second witness; a pin refuses a band whose range overlaps
another, or whose box contradicts the CR's band rules, probed non-vacuous; the
build is 44/44 with 0 errors and 0 warnings. Standard constraints apply, plus
the RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

**The three design points, as decided.**

1. **Range shape — a two-constructor sum, one constructor per printed level
   symbol**, not `(from, Maybe to)`: `LevelRange = LevelBetween (from : Nat)
   (to : Nat) | LevelAtLeast (from : Nat)`, mirroring [CR#711.2a]'s
   `{LEVEL N1-N2}` and [CR#711.2b]'s `{LEVEL N3+}`. The open band is a printed
   symbol in its own right, not a closed band with a missing field, and the
   disjointness law then falls out uniformly (two open bands always overlap, so
   a leveler carries at most one).
2. **The level up ability lives on `inner`**, in `inner.text`, never in a band:
   it is printed on the base face and is not preceded by a level symbol
   [CR#711.4]. `inner.box` is likewise the uppermost power/toughness box
   [CR#711.5,711.6]. No law asserts the ability's *presence* — see the
   deviation on the missing `LevelUp` keyword row.
3. **Which `FaceLaws` apply to a band**: exactly the four that read only the
   shared type line and the band's own text and box — `CardText`,
   `CardChapters`, `CardBox Front`, `DoorFrame` — plus the band's own
   `So (levelRangeOk range)`. `CardLine`, `CardSupers`, `CardCost` and
   `JointChoices` do **not** apply: a striation is inside the card's one text
   box [CR#711.3] and has no name, supertypes, type line, mana cost or choice
   list of its own. This is the existing `SharedLineHalfLaws` idiom, and
   `LevelBandLaws` is spelled against it so `workbench-prototype` has a clean
   pattern to follow.

**Shapes** (`idris/src/Experimental/Card.idr`):

- `data LevelRange = LevelBetween (from : Nat) (to : Nat) | LevelAtLeast (from : Nat)`
  with `levelRangeOk`, `rangeLow`, `rangeHigh`, `rangesOverlap`.
- `record LevelBand` — `MkLevelBand (range : LevelRange) (box : PrintedBox)
  (text : AbilitySeq [])`. The box is required, not `Maybe`: a striation always
  prints a power/toughness box [CR#711.1]. The text is at `[]` because a band
  carries no cost letters and no choice bindings of its own.
- `bandDisjointFrom` / `bandsDisjoint` over the band list; `levelerPtBox`,
  `hasBands`, `levelerFrameOk : TypeLine -> Maybe PrintedBox -> List LevelBand
  -> Bool` (a creature line, an uppermost `PtBox`, at least one level symbol
  [CR#711.1]).
- `data LevelBandLaws : (l : TypeLine) -> LevelBand -> Type` and the list form
  `data LevelBandsLaws : (l : TypeLine) -> List LevelBand -> Type`
  (`NoBands` / `AndBand`), both discharged by `auto` search.
- The constructor, beside `SingleFaced`/`Transforming`/`Adventurer`:

      Leveler : (inner : CardFace) -> (bands : List LevelBand) ->
                {auto 0 nf : FaceLaws Front inner} ->
                {auto 0 lv : So (levelerFrameOk inner.line inner.box bands)} ->
                {auto 0 bl : LevelBandsLaws inner.line bands} ->
                {auto 0 dj : So (bandsDisjoint bands)} -> Card

**Macros** (`idris/src/Experimental/Macros.idr`): `levelBand (range) (pow)
(tou) (text)` builds the `PtBox` striation, and `leveler (name) (cost)
(supers) (line) (text) (stats) (bands)` forwards all four obligations as its
own `{auto 0 …}` parameters, so every bench site is brace-free.

**Witnesses** (`idris/src/Experimental/Cards/Faces.idr`, all through
`Macros.leveler`): `brimstoneMage` (LEVEL 1-2 2/3 with a 1-damage tap ability,
LEVEL 3+ 2/4 with a 3-damage one), `studentOfWarfare` (LEVEL 2-6 3/3 first
strike, LEVEL 7+ 4/4 double strike), `karganDragonlord` (LEVEL 4-7 4/4 flying,
LEVEL 8+ 8/8 flying + trample + an activated pump). All three are vintage-legal
and `supported` in `data/derived/cards.jsonl`.

**Pins** (`idris/src/Experimental/ProofsFaces.idr`), each with the positive twin
`okLevelerBands` above them:

- `badEmptyLevelRange` — `{LEVEL 4-2}`: no count is at least 4 and at most 2
  [CR#711.2a]. Refused at `bl`.
- `badOverlappingLevelBands` — `{LEVEL 1-4}` beside `{LEVEL 3+}`: level 3 falls
  in both, each setting base power and toughness [CR#711.2a,711.2b]. Refused at
  `dj`.
- `badLevelBandOffLevelerFrame` — a band printed on a sorcery: no striated text
  box and no power/toughness boxes to level [CR#711.1]. Refused at `lv`.

## Landing record

- `cd idris && ./scripts/build` (clean `build/`): last line
  `44/44: Building Cards (src/Cards.idr)` — **44/44 modules, 0 Error, 0
  Warning**.
- `cargo xtask cite check --list-noncompliant`: 1 non-compliant string,
  **0 in this round's diff** — the hit is pre-existing and untouched
  (`docs/tickets/done/workbench-turn-parts.md:59`, a bare rule number in
  another round's prose).
- `cargo xtask cite check`: `checked 18255 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless`: registered exactly one new rule, [CR#711.1]
  (`blessed 1585 rules`); no prunes, the lock gained one line.
- `jj --no-pager diff --git > /tmp/lv.diff && cargo xtask cite audit --diff <
  /tmp/lv.diff`: `audited 5 citation site(s)`; each rule read against its claim
  and correct. One wording fix made from the read (the overlap pin originally
  also cited [CR#711.3], whose subject is the striations' lack of game
  significance rather than two symbols setting base power and toughness at
  once; dropped, and the claim restated as what the two level-symbol rules
  literally say).
- Pin non-vacuity, probed: a scratch module built each pin's term with only the
  offending value corrected (`LevelBetween 4 2` -> `LevelBetween 2 4`;
  `LevelBetween 1 4` -> `LevelBetween 1 2`; the sorcery/no-box frame -> a 2/2
  creature) and typechecked all three, so each pin refuses exactly its named
  obligation and nothing else. The scratch module was removed; the standing
  evidence is `okLevelerBands`, kept beside the pins.
- Assurance counts: restored 0; re-spelled 0; ignored 0; **added 7** (3 bench
  witnesses, 1 positive twin, 3 pins); **removed 0**. No existing definition
  was deleted or weakened.
- Deviations and additions:
  - **"Level up {cost}" is spelled as the ability it represents, not as a
    keyword.** Level up is a keyword ability [CR#702.87], but
    `Words.keywordFacts` has no `LevelUp` row, and that table is the concurrent
    `workbench-facts-from-ron` round's region. Each witness therefore writes
    `Macros.activatedOnlyDuring (Mana …) (PutCounters (Lit 1) (PrintedKind
    (Named "Level")) thisCreature) AsSorcery` on `inner`, which is the
    reminder text's content. Ledger: once the facts table lands a `LevelUp`
    row, add a `Macros.levelUp` spelling and rewrite the three witnesses
    through it. Consequence: nothing structurally requires a `Leveler` to carry
    a level up ability; the requirement is met by construction at each witness.
  - **All three witnesses have two bands.** The ticket's "Done when" asks for a
    second leveler with a *different band count*, but every printed leveler
    card has exactly two level symbols [CR#711.1], so no such card exists. Not
    taken as a STOP: the round brief had already replaced that bench with
    "Brimstone Mage plus two more vintage-legal levelers", which is what
    landed; the witnesses differ in band *content* (tap abilities, keyword
    grants, an activated pump) and in their thresholds (1-2/3+, 2-6/7+,
    4-7/8+). The band count is deliberately **not** pinned at two: a
    differently-striated leveler is unprinted, not CR-meaningless, and pins
    refuse only the latter. `hasBands` pins only the zero case, which would
    make `Leveler` a redundant spelling of `SingleFaced`.
  - **Helpers beyond the ticket's named sorts**: `LevelBandLaws`,
    `LevelBandsLaws`, `bandDisjointFrom`, `bandsDisjoint`, `levelerPtBox`,
    `hasBands`, `levelerFrameOk`, `rangeLow`, `rangeHigh`, `rangesOverlap`,
    `levelRangeOk`. Justification: the ticket names the two sorts and the
    constructor; these are the predicates the three settled obligations are
    stated over, and all live in `Card.idr` beside the existing
    `flipHalfOk`/`adventureInsetOk`/`SharedLineHalfLaws` family.
  - Nothing outside `idris/`, this ticket, and `cr-citations.lock` was touched.
    No `Words.Kind`/`Payload`/join-word, `Effect.Continuously`/`Conditionally`,
    `Words.KeywordFacts`, or `crates/` edit.
  - **Working-copy divergence, resolved.** Mid-round the working copy went
    divergent (two commits sharing `@`'s change id): `acaeb71b` was a stale
    snapshot still carrying the deleted `idris/src/Scratch.idr` non-vacuity
    probe and lacking the ticket and lock edits; `9131fdad` held the complete
    round. Diffed both by content, kept `9131fdad`, `jj abandon acaeb71b2036`.
    No operation-log command was used.
- No STOP taken.
