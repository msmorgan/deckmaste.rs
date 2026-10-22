---
needs: []
---
# Keyword cast permissions and costs encode the kind of rule the CR gives them

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#31, #32, #35, #36, #38, #39, #60, #61. Verified against head 2026-10-21. All latent in Lean; each
`plugins_v2` RON mirror named below has the same shape and changes with it. Madness's lasting
permission is in [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3 with the
other "cast it now" sites.

## 1. Foretell and plot exile from the hand

`Definitions/Keywords.lean:715-726` (foretell, `exileFaceDown thisCard`) and `:1095-1104` (plot,
`exile thisCard`) move from `.anyLocation` (`Definitions/Actions.lean:73,79`). [CR#702.170a]: plot
"functions while the card with plot is in a player's hand … exile this card from your hand";
[CR#702.143a] says the same for foretell. A move that names its origin functions only in that
zone [CR#113.6m]. Write the moves from `yourHand` (`exileFrom thisCard yourHand`,
`Definitions/Actions.lean:309-313`, and a face-down counterpart), or wrap each in
`.qualified … (from_ := some .hand)` as forecast does. Mirrors: `foretell.ron:21`, `plot.ron:21`.

## 2. Plotted and foretold are states of an exiled card

No plotted or foretold state exists; the Special Action establishes the cast permission
directly. So "When this card becomes plotted", an effect's "it becomes plotted/foretold", "If this
spell was foretold" and "each foretold card you own in exile" have nothing to read. [CR#702.170a]
"It becomes a plotted card"; [CR#702.170d] a plotted card "may be cast this way even if it doesn't
have the plot ability"; [CR#702.143d] and [CR#702.143c] ("a spell that was a foretold card before
it was cast") are the foretell analogues.

Change: model plotted and foretold as states of a card in exile. The DesignationLabel machinery
may carry them, but its doc comment and the `docs/rules-taxonomy.md:463-464` rows must say the CR
never calls them designations; the plotted row cites [CR#702.170c] for the cast rule, which is [CR#702.170d]. Key the
cast and look permissions on the state, keep the Special Action distinct from the "becomes
plotted/foretold" event, and add a was-foretold predicate on spells.

Owner framing (2026-10-22): a decomposable rule is data. Plot's text ends at "It becomes a plotted
card" [CR#702.170a]; the plotted card's cast right [CR#702.170d] is a rule row
(`plottedCardCast`, `Proofs/Rules.lean`), as is the foretold card's look right [CR#702.143d]
(`foretoldCardLook`). The foretold card's cast right "for any foretell cost it has" needs a cost
term naming a keyword's cost, which the grammar lacks; foretell's own text keeps its look and cast
rights [CR#702.143a]. The v2 mirror of `foretoldCardLook` is
`plugins_v2/testing/rules/grant/foretold-card-look.ron`; `plottedCardCast` has none, since v2's
lookback reads a `GameEvent` and no v2 event is an object gaining a designation.

## 3. Megamorph is a morph cost; multikicker is a kicker cost

- Megamorph (`Definitions/Keywords.lean:957-962`) carries its own label `.named "megamorph"` and
  puts its counter after `turnFaceUp`. Multikicker (`:1000-1001`) is a bodiless
  `keywordCosting (.named "multikicker")`. `Check/Facts.lean:98,139` hold separate rows, so a
  multikicked spell is not "kicked" (*Lightkeeper of Emeria*, `Cards/Keyword.lean:991-1000`, reads
  `timesPaid (.byKeyword kicker)`) and a megamorph cost is not a morph cost.
- [CR#702.33c] "A multikicker cost is a kicker cost"; [CR#702.37b] "A megamorph cost is a morph
  cost" and "As this permanent is turned face up, put a +1/+1 counter on it if its megamorph cost
  was paid to turn it face up".
- Follow the `emergeFrom` (label "emerge", `:566`) and `typecycling`
  (`keywordQualityCosting (.named "cycling")`, `:1486`) precedent: multikicker carries the kicker
  label with `.addedCost cost true .anyNumberOfTimes`, megamorph the morph label. Put megamorph's
  counter on as the permanent is turned face up, not after. This also gives multikicker the body
  that [lean-v3-keyword-bodies-and-catalog-sync](lean-v3-keyword-bodies-and-catalog-sync.md) §1
  lists it as lacking.

## 4. Mutate's added target depends on choosing the mutate cost

`Abilities.lean:928-931` documents `addsTarget` as applying "when its alternative cost is chosen",
but the constructor is unconditional; mutate (`Definitions/Keywords.lean:1005-1008`) uses it
unguarded and enchant (`:584`) uses it for Aura spells, which always target [CR#303.4a]. Mutate
targets only "If you do" [CR#702.140a], and a spell "may require some targets only if an
alternative or additional cost … was chosen" [CR#601.2c].

Change: document `addsTarget` as unconditional, and wrap mutate's use in `onlyWhile`
(`Templates.lean:1405`) on a condition that holds once the mutate cost is chosen at [CR#601.2b].
No such condition exists: the only one is `keywordCostPaid` (`Templates.lean:2029`), and payment
comes later [CR#601.2h], so a was-paid test is false when targets are chosen. Add a
cost-chosen condition. Same change in `mutate.ron` and the doc at
`crates/deckmaste_semantics_v2/src/abilities.rs:783-785`. Enchant stays unconditional.

Coordinator decision (2026-10-22, under the owner's boundary), overriding the split above: one "If
you do" is one condition, so the added target and the merge replacement both read
`choiceWasTaken (.byKeyword mutate)` [CR#702.140a,601.2c]; the existing `LinkedFacet.taken` is
the cost-chosen condition, and when it is evaluated is the engine's concern.

## 5. Retrace and jump-start pay their own cost plus an additional cost

Retrace (`Definitions/Keywords.lean:1235-1242`) is `.payingInstead (.compound [.itsManaCost,
.perform (discard …)])`, an alternative cost, which blocks an alternative-cost cast from the
graveyard (*Jodah, Archmage Eternal*, *Fist of Suns*) because only one alternative cost applies
[CR#118.9a]. [CR#702.81a] makes the discard "an additional cost". Jump-start (`:907`) is a bare
marker because `PlayPayment` (`Abilities.lean:950-956`: `itsOwnCost | withoutPaying |
payingInstead | faceDown`) has no "base cost plus an additional cost" form
(`docs/tickets/done/semantics-v2-keyword-ability-census.md:107`); it lacks both of its abilities
[CR#702.133a].

Change: add a `PlayPayment` form such as `.plusAdditional cost`, whose base is chosen at
[CR#601.2b] (the mana cost or one alternative cost) and whose added part is an additional cost
scoped to the permission. Re-encode retrace (Lean and `retrace.ron`). Give jump-start a graveyard
cast permission with a discard as an additional cost, plus the `would thisSpell (leave stack)`
exile replacement gated on a jump-start cast. Update the pin `castFromGraveyardByDiscardingALand`
(`Cards/Deontic.lean:792-800`), which also drops `.in_ yourHand` from the discard.

## 6. Splice adds to the cast; it does not replace it

`Ability.splice` (`Definitions/Keywords.lean:1342-1355`) is `.replace (.would you (cast …))
(instead (may (.expose .reveal (.cards this_)) (some (.establish …))))`; `instead`
(`Templates.lean:1906-1909`) puts the action in the replaced event's place, so accepting replaces
the cast. [CR#702.47a]: splice "functions while a card is in your hand" and the spell "gains the
text"; revealing is a casting step [CR#601.2b]; several splices are revealed "all at once" in a
chosen order [CR#702.47b].

Change: model splice as a cast-time option at [CR#601.2b]: reveal any set of qualifying cards at
once, add each splice cost as an additional cost, and apply the text change in the chosen order
after the main spell's text. `also` (`Templates.lean:1910-1912`) is not an acceptable interim: it
keeps the cast but still fails [CR#702.47b]. Same in `splice.ron`. The routing of splice's raw
`.expose .reveal` through a Definition in the fidelity ticket §2 is compatible and can land in
either order.

The order of several spliced cards' text is a choice made as the spell is cast [CR#702.47b]; it
is deliberately not encoded in the term (`StaticSpec.revealedAsCast` documents it).

## 7. Aftermath: any graveyard; flashback: the resulting spell

- Aftermath (`Definitions/Keywords.lean:206-215`) forbids `cast this_ (from_ := some (not
  yourGraveyard))`. "Your" is the would-be caster [CR#109.5], so another player with a graveyard
  permission cannot cast the half from its owner's graveyard; [CR#702.127a] says only "can't be
  cast from any zone other than a graveyard" (*Cut // Ribbons* ruling, 2017-04-18). Change the
  restriction to `not graveyard`; in `aftermath.ron:14-16` keep the permission on
  `graveyardOf(you)` and make the exclusive zone any graveyard.
  [lean-v3-authoring-surface](lean-v3-authoring-surface.md)'s Aftermath item prescribes the
  defective shape and is corrected in the same change.
- Flashback (`Definitions/Keywords.lean:697-701`) leaves `cast`'s `resulting` unset. [CR#702.34a]
  applies "if the resulting spell is an instant or sorcery spell". Add `(resulting := some
  isInstantOrSorcery)` (the parameter exists, `Definitions/Actions.lean:43`) and the matching rider
  in `flashback.ron:13`.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; `cargo test -p xtask -p deckmaste_semantics_v2`
and clippy for both; the CR citation gates. Pins: a multikicked spell is kicked; megamorph's
counter is on as it turns face up; mutate's target is absent without the mutate cost chosen; a
retrace cast can use an alternative cost; splicing two cards keeps the cast and orders the text;
an opponent's graveyard permission reaches an aftermath half; plot and foretell refuse a card not
in hand.
