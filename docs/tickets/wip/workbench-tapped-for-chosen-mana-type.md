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
