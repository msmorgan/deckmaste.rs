---
needs: []
---
# Search takes a noun phrase; "and/or" is its own coordinator

Standard constraints apply. This is the search part of the spelling-layer audit: the planned
spelling layer needs one AST encoding for each printed phrase, in both directions.

## Ruling (owner, 2026-10-21)

- `Action.search` (`lean/Semantics/Abilities.lean:1091`, now
  `(scope : Location) (quantity : Quantity) (predicate : Predicate)`) becomes
  `(scope : Location) (for_ : NounPhrase := any)`. Every other object-selecting verb already takes
  a noun phrase. "Search your library for a card" is `search yourLibrary (a isCard)`
  ([CR#701.23a,701.23d]).
- An unqualified "search your library" (no "for …") is `search yourLibrary`. This removes the
  dummy `anyNumber (and [])` quantity and predicate.
- "X and/or Y" is a different coordinator from "X and Y" and from "X or Y", on `NounPhrase`,
  `Location` and `Predicate` alike: no two of the three share an encoding. The preferred design is
  one coordinator enum (`and | or | andOr`) shared by all three types. The implementer settles the
  exact shape: a field on the existing constructors, or new constructors.
- A coordinated `NounPhrase` publishes a plural group referent, so a later `them` resolves.
- This ticket fixes *Agency Outfitter*'s `upTo 2` deviation, recorded in
  [lean-v3-typed-verbed-deeds](../done/lean-v3-typed-verbed-deeds.md).

## Verified state (2026-10-21)

- **No group referent today.** `NounPhrase.and` (`Check/Phrase.lean:2257-2260`) steps each
  conjunct and returns their ids. It never pushes a plural referent, although `NounPhrase.plur` is
  `.many` for `.and` (`Check/Phrase.lean:328`). `J1` matches number exactly
  (`reaches`, `Check/Words.lean:973-978`).
  A probe showed this: `sequentially [tap (.and [target isCreature, target isArtifact]),
  untap them]` is refused with `anaphor .bare .many`, while `tap (target isCreature)` followed by
  `untap it` checks.
- **Search publishes its found referent from the quantity.** The `.search` step
  (`Check/Phrase.lean:3027-3040`) pushes one `foundRef` with `plur := q.plur` and
  `p.seedTy`. Once `for_` is a noun phrase, the step publishes what `for_` introduces, in the
  patient role, with the `search` stamp and the library-search flag. The plural group referent for
  a coordinated `for_` comes from the general fix below.
- **"and" and "and/or" already collide on the zone list.** `Location.or` encodes *Eradicate*'s
  printed "graveyard, hand, and library" (`searchZonesOf`, `Definitions/Actions.lean:178-187`;
  *Eradicate* `Cards/Anaphora.lean:199`, *Memoricide* `Cards/Choice.lean:970`). It also encodes
  "library and/or graveyard" (`searchLibraryOrGraveyard`, `Definitions/Actions.lean:190-194`) and
  *Agency Outfitter*'s "graveyard, hand and/or library" (`Cards/Trigger.lean:687`). *Agency
  Outfitter* cannot be fixed without its zone list, so the coordinator split covers `Location` as
  well as `NounPhrase` (accepted by the owner 2026-10-21).
- **"or" and "and/or" collide on `Predicate`.** `Predicate.or` encodes the printed "or" ("artifact
  or creature") and also the printed "and/or" of flying ("creatures with flying and/or reach",
  `Definitions/Keywords.lean:86-92`), fear (`:674-679`) and intimidate (`:892-897`). By the owner
  ruling, these split too.
- **Two trees for "search your library for …".** `searchLibraryFor` (`Definitions/Actions.lean:173`)
  expands to `.search (libraryOf .actor) …`. The `search` Definition (`Definitions/Actions.lean:717`)
  with `yourLibrary` expands to `.search (.zone .library (some you)) …`. The new signature must
  leave one encoding of the phrase, `search yourLibrary <for_>` by the ruling. Either retire
  `searchLibraryFor` or make it a Template that expands to the same tree.

## Change

1. `Action.search (scope : Location) (for_ : NounPhrase := any)`. Update every match on `.search`:
   `Check/Phrase.lean:939,3027`, `Check/LocalShape.lean:145`, `Check/Events.lean:39,56` and
   `Check/NumberSlots.lean:176`.
2. Re-sign the search Definitions onto the new shape and leave one encoding per printed phrase:
   `searchLibraryFor`, `searchZonesOf`, `searchLibraryOrGraveyard` and `search`
   (`Definitions/Actions.lean:173,178,190,717`). Rewrite the raw `.search yourLibrary (oneThrough 1)`
   in transmute and transfigure (`Definitions/Keywords.lean:1451,1464`). If
   [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §2 lands first, these are
   already Definition calls.
3. Add the "and/or" coordinator to `NounPhrase`, `Location` and `Predicate`, preferably through one
   shared coordinator enum. Each existing `NounPhrase.and`, `Location.or` and `Predicate.or` site
   takes the coordinator its text prints: flying, fear and intimidate take and/or, and the
   printed-"and" zone list of `searchZonesOf` takes "and". Find the printed "and/or" sites by
   checking each site's Oracle text, not only the Lean comments.
4. A coordinated `NounPhrase` ("and" and "and/or") publishes a plural group referent over its
   conjuncts. Each conjunct's own referent stays reachable as it is today.
5. Rewrite these sites:
   - Dummy `anyNumber (and [])`, unqualified "search your library":
     - *Vraska's Scorn* `Cards/Choice.lean:1187` and *Agency Outfitter* `Cards/Trigger.lean:690`:
       `did you (search any …)` becomes `did you (search yourLibrary) thisWay`. The scope is
       `any` (`wherever`) today, which is wrong for "your library".
     - *Tower Winder* `Cards/Keyword.lean:1145`, *Delivery Moogle* `Cards/Mana.lean:880`.
     - `whileSearchingYourLibrary` `Templates.lean:883`.
   - Dummy, "search libraries": *Mindlock Orb* `Cards/Static.lean:213`, *Shadow of Doubt*
     `Cards/Static.lean:227`. These print "Players can't search libraries", so the scope is the
     Location for "libraries", not `any` (`wherever`, "anywhere"), and `for_` defaults.
   - Dummy predicate for a printed "a card": *Demonic Tutor* `Cards/Anaphora.lean:226`
     (`searchLibraryFor (exactly 1) (and [])`) becomes `search yourLibrary (a isCard)`.
   - *Agency Outfitter* `Cards/Trigger.lean:685-691`: the zone list "graveyard, hand and/or
     library" uses the and/or `Location` coordinator, and `for_` is the and/or coordination of
     `a (named "Magnifying Glass")` and `a (named "Thinking Cap")`. `putOntoBattlefield them`
     reads the group referent. Drop `upTo 2`.
   - Every other `searchLibraryFor`, `searchZonesOf`, `searchLibraryOrGraveyard` and `search` call
     in `Cards/` and `Proofs/` (`Proofs/Anaphora.lean:281`, `Proofs/Lookback.lean:105`,
     `Proofs/MovementScopes.lean:180,196`, `Proofs/Zone.lean:294-300`) moves to the new signature.
     The quantity and predicate become the noun phrase they print ("up to three basic land cards",
     "any number of cards with that name").
6. Update `lean/CONTRACTS.md` (the `Location` row and any search shape it states).
   `docs/decisions/semantics-v3.md` §3.6 already writes `search(libraryOf(owner), any)` and needs
   no change.

## Tests

- *Agency Outfitter* checks with `them` resolving to the and/or group referent, and with no
  quantity.
- "Destroy target creature and target artifact" (or `tap … untap them`) followed by `them` checks,
  and `them` resolves to the group. This probe is refused today.
- "X and Y" and "X and/or Y" expand to distinct trees on `NounPhrase` and `Location`, as do
  "X or Y" and "X and/or Y" on `Predicate` (flying's "with flying and/or reach" against a "with
  flying or reach").
- *Demonic Tutor*, *Mindlock Orb*, *Shadow of Doubt*, *Vraska's Scorn*, *Tower Winder* and
  *Delivery Moogle* check in their new form. "If you search your library this way" still matches a
  search of "library and/or graveyard".
- `ok…` theorems for every rewritten site stay green under `./lean/scripts/build` (`--wfail`).
