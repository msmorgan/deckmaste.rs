---
needs: []
---
**Give "tapped for mana" its own `EventName` and delete the `forMana` axis.**
Ruling 2026-09-04 (cleanroom review 3, D-Q5).

- `Triggers.VerbedEvent` carries `forMana : Bool` on every verbed event, and
  `Words.actFacts`'s `actForMana` column is `True` for `Tap` alone — a lexeme
  guard written as data, consulted by `Events.verbForManaOk`. An ability that
  triggers whenever a permanent "is tapped for mana" triggers whenever such a
  mana ability resolves and produces mana [CR#106.12a], which is an event of
  its own rather than a flag on tapping.
- Add `Events.EventName.TappedForMana`, and either carry the specified-mana-type
  variant [CR#106.12a] or record the decision to leave it out. Delete
  `forMana`, `actForMana` and `verbForManaOk`.
- Re-spell the witnesses that carry `forMana = True` as the new event, and pin
  a plain tap read as a mana event.

Size: S. Done when: `grep` finds no `forMana` or `actForMana`; every
tapped-for-mana witness reads the new event; the pin probes non-vacuous; build
at its module count. Standard constraints apply, including the RON-shaped
constraint.

## As landed

- `Triggers.VerbedEvent`'s `forMana : Bool` param and its `{auto 0 fm : So (not
  forMana || verbForManaOk v)}` obligation are gone; `Words.ActFacts`'s
  `actForMana` field and its 88 row values are gone; `Events.verbForManaOk` is
  gone. `grep -rn "forMana\|ForMana" idris/src/Experimental/` now returns only
  the new `TappedForMana` name.
- Added `Events.EventName.TappedForMana` (`eventIx … = 42`) with an
  `eventFactsOf` row `MkEventFacts [Player, Object] [(Player, Object)] [Player]
  False True False True False` — the same facts the retired `VerbedAct "Tap"`
  row computed — carrying the `[CR#106.12a]` cite. Added the core row
  `Triggers.TappedForMana (who : Maybe (Noun bs Player)) (what : Noun
  (agentIntro who) Object)` with `{auto 0 zn : ZoneFits (nounZone what) (Just
  Battlefield)}`, plus its `eventName`/`eventIntro`/`eventAfter`/
  `eventSubjectPlur` cases. `eventAfter` publishes `outcomeB ManaProduced ::
  stampIntro (Just "Tap") what`, the exact bindings the `forMana = True` branch
  published.
  - **Specified-mana-type variant: left out**, deliberately. The only
    VINTAGE-legal printed [CR#106.12a] trigger naming a type is Gauntlet of
    Power ("tapped for mana of the chosen color"); spelling it needs a
    chosen-quality read (`countChoice (QSort Color)` + `ChosenQualityRead
    Color`), not the literal `ManaMatch`/`ColorOrColorless` vocabulary reachable
    at `Triggers` scope, so a literal-type slot would land with no printed
    witness and still would not spell Gauntlet of Power. Nothing pins the
    variant — it is unbuilt, not refused.
- Witnesses re-spelled as the new event: Mana Flare and Shimmerwilds Growth
  (`Cards/Mana.idr`), Darksteel Garrison (`Cards/Cost.idr`). The other 24
  `VerbedEvent` sites lost their trailing `False` only.
- Pins (`ProofsMana`): `okForManaOnTap` → `okTappedForMana` (twin);
  `badForManaOnNontap` → `badTappedForManaOffField` ("Whenever a card in a
  graveyard is tapped for mana" — only a permanent is tapped for mana
  [CR#106.12]); added `badProducedByPlainTap` ("Whenever a player taps a land,
  add one mana of any type that land produced" — a plain tap resolves no mana
  ability [CR#106.12a]) over the new `afterAPlainLandTap` bindings, twinned by
  the kept `okProducedByTapEvent`. Both pins probed non-vacuous.

## Landing record

Numbers before → after:

- `forMana`/`actForMana` occurrences in `idris/src/Experimental/`: 4 defining
  sites (the `VerbedEvent` parameter, its `fm` obligation, `ActFacts.actForMana`,
  `Events.verbForManaOk`) + 88 table values + 30 call-site arguments → 0.
- `EventName` constructors: 42 → 43 (`eventIx` 0..42).
- `ActFacts` fields: 15 → 14. `actFacts` rows: 88 → 88 (unchanged).
- `GameEvent` constructors: +1 (`TappedForMana`); `VerbedEvent` arity 5 → 4
  explicit params, 6 → 5 obligations.
- `VerbedEvent` sites in `Cards/*.idr` + `Proofs*.idr`: 30 → 25 (3 re-spelled
  as `TappedForMana`, 2 folded away with the retired pin/twin, 1 added for the
  new pin's bindings).
- Idris modules: 46 → 46.

Gates:

- `cd idris && rm -rf build && ./scripts/build` → exit 0, `46/46: Building
  Cards (src/Cards.idr)`, no `Warning` and no `Error` line.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14319 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 10 citation site(s)`; each read against its
  claim. No `cite bless` needed — both rules were already registered in
  `cr-citations.lock`.
- Pin probes: mis-stating `badTappedForManaOffField`'s noun to `Macros.a
  Macros.land` → `Error: badTappedForManaOffField Oh is not a valid impossible
  case.`; re-indexing `badProducedByPlainTap` at `afterALandTapForMana` →
  `Error: badProducedByPlainTap Refl is not a valid impossible case.` Both
  reverted.

Assurance counts: restored 0; re-spelled 2 (`okForManaOnTap` →
`okTappedForMana`, `badForManaOnNontap` → `badTappedForManaOffField`) plus 3
bench witnesses re-spelled as the new event; ignored 0; added 1 pin
(`badProducedByPlainTap`) and 1 supporting definition (`afterAPlainLandTap`);
removed 0.

Deviations and additions:

- `ZoneFits (nounZone what) (Just Battlefield)` rather than the stricter
  `ZoneIs`, mirroring `Triggers.Enters`: an unzoned "a land" is admitted while
  an explicitly off-battlefield noun is refused [CR#106.12]. `ZoneIs` would
  have refused every existing bench witness.
- `eventAfter` keeps the `"Tap"` participle stamp (`stampIntro (Just "Tap")`),
  so "the tapped land" reads stay available and the published bindings are
  byte-identical to the retired `forMana = True` branch. Naming the label in a
  core `Triggers` function has precedent in `eventName (UnlocksDoor _ _) =
  VerbedAct "Unlock"`; it gates nothing.
- `afterAPlainLandTap` is a new `Bindings` definition in `ProofsMana`, added
  only to index the new pin.

Ledger: the specified-mana-type variant of [CR#106.12a] and its Gauntlet of
Power witness need their own ticket (a chosen-color mana-type slot on
`TappedForMana`).

STOP taken: none.
