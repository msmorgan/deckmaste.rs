---
needs: []
---
**Read "tapped for mana of the chosen color".** Residue of
`workbench-tapped-for-mana-event` (2026-09-04): `Triggers.TappedForMana`
carries no mana-type slot because the only vintage-legal printed trigger
that names one, Gauntlet of Power ("Whenever a basic land is tapped for
mana of the chosen color"), needs a chosen-quality read (the colour chosen
as the permanent entered, `countChoice (QSort Color)` and a
`ChosenQualityRead Color`), not a literal `ManaMatch`. Add the positional
slot as `Maybe` of a mana-type read that admits both a literal type and the
chosen-quality read [CR#106.12a], bench Gauntlet of Power in full, pin a
type no land can produce under the chosen-colour read with the rule named.

Size: S. Done when: Gauntlet of Power is benched; every existing
`TappedForMana` site takes `Nothing`; pin probed; build at its module
count. Standard constraints apply, including the RON-shaped constraint.

## Ruling 2026-09-04

One positional `Maybe` slot holding a mana-type read that admits both a
literal type and the chosen-quality read; `Nothing` at every existing site.

## As landed

- **Positional `Maybe` mana-type slot.** `Triggers.TappedForMana` takes a third
  positional slot `(ty : Maybe (ManaTypeTerm (agentIntro who)))`, indexed at
  the agent's introductions like `VerbedEvent`'s `becomes`. Its obligation set
  is unchanged (`zn : ZoneFits (nounZone what) (Just Battlefield)`), and
  `eventName`/`eventIntro`/`eventAfter`/`eventSubjectPlur` ignore the slot, so
  the published bindings after the event are what they were.
- **The mana-type read.** New `Triggers.ManaTypeTerm : Bindings -> Type` above
  the module's single `mutual` block (it needs no cycle): `ColorlessMana` and
  `ManaOfColor (ColorTerm bs)`. `ColorTerm` already admits both a literal
  colour (`LitColor`) and the chosen-quality read (`ThatColor ref`, gated on
  `choiceRefOk ref (countChoice (QSort Color) bs)` and `ChosenQualityRead
  Color`), so the two rows are exactly the six mana types of [CR#106.1b] with
  no second copy of the chosen-colour read.
- **Gauntlet of Power benched in full** as `Cards/Mana.idr`'s
  `gauntletOfPower` — all three lines: `Macros.entersChoosing
  Macros.thisArtifact Color`; `Macros.getsPt (Macros.allOf (And [Macros.creature,
  Macros.ofChosen Color])) (Up (Lit 1)) (Up (Lit 1))`; and the trigger
  `TappedForMana Nothing (Macros.a (And [Macros.land, HasSupertype Basic]))
  (Just (ManaOfColor Macros.thatColor))` with `AddMana (Macros.controllerOf
  (Macros.That (TypeW Land) OneOf)) (Lit 1) (OfChosenColor Nothing) []`.
- **Every existing site takes `Nothing`**: `Cards/Mana.idr` (Mana Flare,
  Shimmerwilds Growth), `Cards/Cost.idr` (Darksteel Garrison), `ProofsMana`
  (`okTappedForMana`, `badTappedForManaOffField`, `afterALandTapForMana`).
- **Pin**: `ProofsMana.badTapForChosenNonManaType` — "As this artifact enters,
  choose a creature type. Whenever a basic land is tapped for mana of the
  chosen type, draw a card." Refused because `ManaOfColor` reads a chosen
  COLOR alone; the six mana types are the five colours and colorless
  [CR#106.1b], so no land produces mana of a creature type. Twin beside it:
  `ProofsMana.okTapForChosenColorMana`, the same card choosing a colour.
  Probed non-vacuous.
- Nothing left undone.

## Landing record

Numbers before → after:

- `Triggers.TappedForMana` explicit params: 2 → 3; obligations 1 → 1.
- `Triggers` `data` declarations: 15 → 16 (`ManaTypeTerm`, 2 constructors).
- `GameEvent` constructors: 39 → 39 (unchanged).
- `TappedForMana` term sites: 6 → 9 (the 6 existing ones re-spelled with
  `Nothing`; 3 new — Gauntlet of Power, the twin, the pin).
- `Cards/Mana.idr` `: Card` definitions: 85 → 86.
- `ProofsMana` `Unspellable` pins: 43 → 44.
- Idris modules: 46 → 46.

Gates:

- `cd idris && rm -rf build && ./scripts/build` → exit 0, last line `46/46:
  Building Cards (src/Cards.idr)`, 0 lines matching `Warning|Error`. The
  script's bench implicit-handle lint and `check-pin-twins` both passed.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14410 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 3 citation site(s)`; each read against its
  claim. No `cite bless` needed — both cited rules were already registered in
  `cr-citations.lock`.
- Pin probe: re-stating `badTapForChosenNonManaType`'s chooser as
  `Macros.entersChoosing Macros.thisArtifact Color` →
  `Error: badTapForChosenNonManaType Refl is not a valid impossible case.`
  Reverted.

Assurance counts: restored 0; re-spelled 6 (`TappedForMana` sites taking the
new `Nothing`: Mana Flare, Shimmerwilds Growth, Darksteel Garrison,
`okTappedForMana`, `badTappedForManaOffField`, `afterALandTapForMana`);
ignored 0; added 3 (`gauntletOfPower`, `okTapForChosenColorMana`,
`badTapForChosenNonManaType`); removed 0.

Deviations and additions:

- The read is `ManaOfColor (ColorTerm bs)` over the existing `Phrase.ColorTerm`
  rather than a fresh `WrittenType ColorOrColorless | ChosenType ref` pair. A
  second chosen-colour row would have duplicated `ThatColor`'s obligations, and
  the chosen `ColorlessMana | ManaOfColor` split is exactly `ColorOrColorless`
  lifted over bindings, so the type ranges over the six mana types [CR#106.1b]
  with one row per meaning.
- `ColorlessMana` has no printed witness — no printed trigger names colorless
  as the specified type — and is left unpinned rather than refused: [CR#106.12a]
  admits any specified type and colorless is one [CR#106.1b].
- The slot is indexed at `agentIntro who`, not `bs`, so the telescope after
  `who` stays uniform with `VerbedEvent`'s `becomes`.
- The pin and twin use `Draw You (Lit 1)` as the triggered effect rather than
  Gauntlet of Power's `AddMana … (OfChosenColor Nothing)`: the latter carries
  its own standing-colour-choice obligation and would have made the pin refuse
  on two goals at once.

STOP taken: none.
