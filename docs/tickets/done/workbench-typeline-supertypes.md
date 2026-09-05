---
needs: [workbench-attack-agent-role]
---
**Put supertypes on the type line, in printed order.** Ruling 2026-09-04.
`Words.TypeLine` is `MkTypeLine subs tys` and supertypes live apart as
`Characteristics.supers` (with a second separate slot on
`Card.SharedLineSplit`), while the type line by definition carries card
types, supertypes and subtypes [CR#205.1] and the printed order is
supertypes, card types, subtypes [CR#205.4a,205.3a].

Fix: `MkTypeLine (supers : List Supertype) (tys : List CardType) (subs :
List Subtype)`; delete `Characteristics.supers` and `SharedLineSplit`'s
separate `supers`; move the `CardSupers` frame law onto the line; retype
`lineNonEmpty`, `addedFits` and every projection; re-spell every witness
(the `Macros.card` line argument and the type-changing effects that build
a `TypeLine`) and every pin; no supertype-versus-type pin: a supertype is independent of card type
[CR#205.4b], so no such pairing is rules-meaningless. Mechanical after the
record change; the build's face laws catch the rest.

Size: S–M. Done when: no supertype field exists outside `TypeLine`; every
witness spells its printed line in printed order; build at its module
count. Standard constraints apply, including the RON-shaped constraint.

## As landed

- `MkTypeLine (supers : List Supertype) (tys : List CardType) (subs : List Subtype)`:
  landed in `Words.idr`, printed order, one docstring line carrying the cite.
- `Characteristics.supers` deleted; `line` is the only type slot on the record.
- `SharedLineSplit`'s separate `supers` parameter and its `sp` obligation deleted;
  it now reads `SharedLineSplit (line : TypeLine) (box : Maybe PrintedBox) …`.
- `CardSupers` moved onto the line: `MkCardLine` gained `{auto 0 sp : CardSupers l.supers}`;
  `MkCharacteristicsLaws`, `SharedLineSplit` and `Macros.cardOf` dropped theirs.
- `lineNonEmpty`, `addedFits`, `addsSomething`, `retentionOk`, `tokenTyped`,
  `tokenCanonical`, `lossWritesTypes`, `bundleOk` retyped to the three-slot record;
  the `supers` reads now go through `.line.supers`.
- Every witness re-spelled: 1026 `MkTypeLine` sites, and the supers argument dropped
  from `Macros.card` / `cardOf` / `frontFace` / `backFace` / `alternative` / `leveler` /
  `prototype` / `MkCharacteristics` / `MkTokenChars` call sites, its value folded into
  the line's first slot. `Macros.creatureTokOf`, `subtypesOnly`, `typesOnly` and
  `basicLandLine` build three-slot lines.
- Pins re-spelled: `ProofsFaces.badDuplicateSnow`, `ProofsFaces.badDuplicateSupertype`
  (refuted obligation moved from `MkCharacteristicsLaws`'s `sp` into `MkCardLine`, so the
  impossible clause names `{ln = MkCardLine}`), `ProofsTurn.badTokenDuplicateSupertype`
  (`MkSupertypedToken` → `MkToken`, supers in the line). Twins `ProofsFaces.okSingleSnow`
  and `ProofsTurn.okTokenSingleSupertype` unchanged beside them.
- No supertype-versus-type pin added ([CR#205.4b]).

## Landing record

Measured on change `yvnvkvyx` (working copy of this landing), `idris/` only.

Numbers before → after:

- supertype fields outside `TypeLine`: 3 (`Characteristics.supers`,
  `SharedLineSplit.supers`, `TokenChars.supers`) → 0.
- `MkTypeLine` sites: 1026 → 1026, all three-slot.
- type lines carrying a supertype: 0 → 104.
- supertype tokens in the touched files, before = after: Legendary 101, Basic 25,
  Snow 19, Ongoing 2, World 6.
- slot occupancy audit: 0 subtype constructors in the tys slot, 0 card types in the
  subs slot, 0 supertypes outside slot 0.
- `Unspellable` pin declarations: 663 → 663 (0 added, 0 removed).
- modules: 46 → 46.
- diffstat: 28 files, 1883 insertions, 1900 deletions.

Gate lines:

- `cd idris && ./scripts/build` (clean `build/`): `46/46: Building Cards (src/Cards.idr)`;
  0 `Error`/`Warning` lines. Includes the bench implicit-handle lint and `check-pin-twins`.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14439 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff < /tmp/round.diff`:
  `audited 11 citation site(s)` with this landing record in the diff (2 of them in
  `idris/`, the rest in this file); each read against `cr.txt` — [CR#205.1] (the type line
  holds card types, subtypes and supertypes), [CR#205.4a] (supertypes printed directly
  before the card types), [CR#205.3b] (subtypes listed after the long dash), [CR#205.4b]
  (a supertype is independent of card type and subtype).
- `cargo xtask cite bless` not needed: [CR#205.1,205.4a,205.3b] were already in
  `cr-citations.lock`; the lock is unchanged.

Assurance counts: restored 0; re-spelled 3 pins (`badDuplicateSnow`,
`badDuplicateSupertype`, `badTokenDuplicateSupertype`) plus every witness term;
ignored-with-blocker 0; added 0; removed 0. Each re-spelled pin was probed
non-vacuous by deleting the duplicate supertype once — all three then reported
`… is not a valid impossible case`, and the file was restored from the pre-probe copy.

Performance advisory: n/a — this round runs no coverage command; its gate is
`idris/scripts/build`, unchanged in shape and still at 46 modules.

Deviations and additions:

- `TokenChars.supers` was deleted too, folded into `TokenChars.line`. The ticket's Fix
  list names only `Characteristics` and `SharedLineSplit`, but "Done when: no supertype
  field exists outside `TypeLine`" covers it, and a token's line prints its supertype in
  the same order [CR#205.4a]. Consequence: `Macros.MkSupertypedToken` is deleted and its
  two call sites read `Macros.MkToken` — the pair existed only to reach the separate slot,
  so keeping it would have left an arity pair.
- `lineNonEmpty` was retyped to ignore the supers slot
  (`lineNonEmpty (MkTypeLine _ [] []) = False`) rather than to fold supers in. Folding
  would have weakened `MkCardLine`'s `ne` obligation into admitting a card whose entire
  printed line is "Legendary"; no ticket bullet asks for that, and the round adds no new
  frame law. The two disjuncts that read supertypes independently of the line
  (`Effect.lossWritesTypes`, `Effect.bundleOk Adds`) are therefore kept, re-pointed at
  `t.line.supers`, so every predicate means exactly what it meant before.
- The order cite on the record is written [CR#205.4a,205.3b], not the ticket's
  [CR#205.3a]: the latter says only that a card can have subtypes on its type line,
  while [CR#205.3b] is the rule that places them after the long dash.
- One docstring line added on `record TypeLine` (the ticket's cite); no other comment
  added or removed.

STOP taken: none.
