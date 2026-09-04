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
