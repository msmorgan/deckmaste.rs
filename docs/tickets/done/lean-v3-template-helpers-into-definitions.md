---
needs: [lean-v3-facts-from-declarations]
---
# Templates name no catalog items: move the CR-composite helpers into `Definitions/`

Standard constraints apply. Lean first (`lean/Semantics/`); the Rust mirror is not this ticket.
Part 1 of 5 of ruling #1 of the owner's batch 4 (2026-10-22). Supersedes step 2 of lean-v3-facts-from-declarations (WIP, owned elsewhere; that ticket's step 1 still lands first).

## Ruling (owner, 2026-10-22, batch 4): the Facts tables are deleted

This replaces step 2 of `lean-v3-facts-from-declarations`' A/B; the owner chose neither.

> Make CardType and all catalog items self-contained `semantic_definition`s whose values carry
> their own fields and whose raw constructors are marked `[semantic_expression, internal_expansion]`.
> `Authoring.inspect` already rejects raw `[internal_expansion]` constructors in cards, so every
> `known*` table check is redundant with the authoring boundary.

The whole ruling is split across five tickets that land one at a time, in this order:
`lean-v3-template-helpers-into-definitions` (B1), `lean-v3-catalog-items-self-contained` (B2),
`lean-v3-keyword-facts-deleted` (B3), `lean-v3-action-facts-deleted` (B4),
`lean-v3-card-type-composite-definition` (B5, which also deletes `Check/Facts.lean` and
`Check/FactTypes.lean`). The order differs from the first suggested split (CardType first): see
B1 "Why first".

## The clause this ticket carries

> Move the CR-composite helpers in `Templates.lean` that name specific card types, subtypes or
> designations (`isOutlaw`, `isHistoric`, `isAnyTarget`, `party`, `army`, `yourCommander`,
> `twoTwoCreature`, `isCreature`, `thisCreature`, ...) into `Definitions/`, giving a strict one-way
> layering: Primitives -> Templates -> Definitions -> Cards.

## Why first

Every later ticket declares catalog items in `Definitions/` and gives them fields (a counter's
holder, a card type's stats). `Templates.lean` imports the four leaf catalogs today
(`Templates.lean:3-6`: `Definitions.Counters`, `.Designations`, `.Relations`, `.Subtypes`), so a
catalog item built over Templates (CardType's `confers`, written over templates such as
`playableFrom yourHand asSorcery`) would make Templates and Definitions import each other. The
layering has to be made one-way before the items move. This is a pure move: no checker change, no
new refusal.

## Verified (2026-10-22)

- Current layering: `Templates.lean` imports `Semantics.Primitives`, `CallerScope` and the four
  leaf catalogs; `Definitions/Actions.lean:1-2` imports `Semantics.Templates` and
  `Semantics.Check.Phrase`; `Definitions/Keywords.lean:1` imports `Definitions.Actions`;
  `Check.lean:13` imports `Definitions.Subtypes`. The four leaf catalogs import only
  `Semantics.Primitives`. So the layering today is Primitives -> leaf Definitions -> Templates ->
  Check -> composite Definitions -> Cards; the ruling's strict form needs the leaf catalogs above
  Templates.
- The helpers exist at: `isCreature` `Templates.lean:399` (and `isArtifact` ... `isKindred`
  `:400-407`), `isOutlaw` `:450`, `isHistoric` `:455`, `isAnyTarget` `:458` (and
  `isAnyOtherTarget` `:461`), `partyRoles` `:612`, `thisCreature` `:705` (and the `this<Type>` /
  `this<Subtype>` family `:706-726`), `yourCommander` `:728`, `partyOf`/`party`/`partySizeOf`/
  `partySize` `:775-781`, the `NounWord` type words `creature` ... `battle` `:822-827`, `army`
  `:1720`.
- **`twoTwoCreature` does not exist.** The nearest helpers are `faceDownCreature` and
  `faceDownWardedCreature` (`Templates.lean:1185-1192`, a 2/2 face-down creature, with and without
  ward), which name `.creature` and the keyword label `ward`. They move with the rest. No other
  part of the ruling depends on the name.
- Sites in `Templates.lean` that name a catalog item (name match against the four catalogs'
  705 declared names, 2026-10-22; false positives `map`, `performer`, `role` excluded): `:452-456`
  (outlaw subtypes, `saga`), `:613` (party subtypes), `:633` (`class_`), `:649` (`foretold`),
  `:705-728` (the `this<Subtype>` family and `commander`), `:795`, `:863` (`phyrexian`), `:985-992`
  (`saga`), `:1059` (`gate`), `:1720-1721` (`army`), `:2483-2485` (`room`), `:2550` (`class_`),
  `:2567` (`vehicle`). Sites that name a card-type constructor: 29 (`:123, 399-413, 456, 459,
  705-717, 822-827, 1187-1192, 1290, 1537`).

## Change

1. Create the Definitions-layer module(s) (name at the implementer's discretion; check the
   glossary before naming a public item) over Templates, and move every helper above, with its
   doc comment, from `Templates.lean`. Move any other template that names a catalog item or card
   type, found by the sites above.
2. Remove the four leaf-catalog imports from `Templates.lean`; the leaf catalogs keep importing
   only `Primitives`, and `Definitions.lean` imports them and the new module(s).
3. Re-point every user. Cards open `Semantics.Templates` and `Semantics.Definitions` both
   (`Proofs/KeywordByCardClass.lean:21`), so Cards are unaffected; `Check/*` files that use a moved
   helper must not import Definitions (it is above Check.Phrase): re-spell such a use with the raw
   primitive, or move that piece of the checker, and say which.
4. Update `lean/CONTRACTS.md:140` ("The checker sits beside this stack, not under it") and the header of
   `Templates.lean:15-17` ("leaf catalogs ... a phrasing helper that needs a composite Definition
   lives after it"), which describe the old layering.

## Tests

- No pin changes meaning. `lean/scripts/build` (`lake build --wfail`) is the test; `cargo xtask
  lean check` only if plugin definitions changed (none expected).
- Add `Proofs/DefinitionsAndTemplates.lean` pins only where a moved helper had one, re-pointed at
  its new home (list them in the landing).

## Other tickets that touch the same files

- `lean-v3-facts-from-declarations` (WIP): Step 1 edits `Check/Facts.lean`, `FactTypes.lean` and
  their docs; no overlap with `Templates.lean`.
- `lean-v3-authoring-surface` (planned umbrella; lists `hasType (t : CardType)` and the type words
  at `:520-529`), `lean-v3-proofs-and-docs-alignment`: read, do not edit, unless they cite a moved
  helper's path.
- Every later ticket of this set (B2-B5).
