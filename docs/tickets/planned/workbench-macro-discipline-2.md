---
needs: []
---
**Retire the duplicate, agreement-paired, unused and forwarding macros; derive
the string-label gates from the facts rows; and delete the facts columns that
duplicate or prove nothing.** Cleanroom review 2026-09-03, F14 + F11 + F12,
and audit-2 N3 + N4 + N5. F14's raw-core-where-a-macro-exists half is
`workbench-bench-fidelity`, not this ticket.

**F14 — macro layer.** `atLeast` (`Macros.idr:85`) and `orHigher` (`:2521`)
are the same function. Twelve agreement pairs where plurality is already a
positional argument: `It`/`Them` (`:9`/`:35`), `That`/`Those` (`:49`/`:44`),
`ItVerbed`/`ThemVerbed` (`:22`/`:39`), `TheVerbed`/`ThoseVerbed`
(`:60`/`:66`), `itVerbed`/`themVerbed` (`:2149`/`:2154`, case-only forwarders
of the previous), `theVerbed(ThisWay)`/`thoseVerbed(ThisWay)` (`:2185–2206`),
`single`/`manyCounterEvent` (`:2096`/`:2089`), `bare`/`manyBareCounterEvent`
(`:2103`/`:2108`), `target`/`targets` (`:93`/`:98`), `scryTheirOne`/`Many`
(`:1528`/`:1548`), `surveilTheirOne`/`Many` (`:1613`/`:1632`) — the last four
`public export` and unused by the bench. Arity families over an argument that
already has a macro: `chooseOne`/`Two`/`OneOrBoth`/`OneOrMore`/`AnyNumber`
(`:1068–1102`) → `chooseModes q`; `oneOf`/`someOf`,
`oneFromAmong`/`fromAmong`, `searchLibraryFor`/`…ForCount`,
`topCard`→`topCards`→`topSlice`, `forEach`/`nForEach`. About 36 one-token
forwarders (`fromZ = Just` `:1967`, `notSo = NotCond` `:1104`, `draw = Draw`
`:1063`, `counterSpell` `:276`, `insteadOf` `:1387`, `shieldingIt` `:1357`, …)
and 19 macros with zero bench uses (auditor A §5 lists them; re-derive before
deleting). Two core modules host macros that belong in `Macros`:
`EntersTapped` (`Effect.idr:866`) and the `MkToken`/`MkSupertypedToken` arity
pair (`Effect.idr:2917–2927`).

**N3 — `mayPlayDeed` lacks the `asThough` cell.** Four bench lines
(`Cards.idr:6425, 6434, 6443, 6462`, the "cast … as though it had flash"
family) are raw eight-slot `Deontic`; `Macros.canDoAsThough` has the
as-though but no patient or rider. Fix: `mayPlayDeed deed who what (asThough :
Maybe (AsThough …)) rider` — positional, per the no-default-slots ruling — and
re-spell the four; a `mayPlayDeedAsThough` sibling is acceptable if the
positional `Nothing` at 20 call sites is judged noise.

**N4 — `scry`/`surveil` coverage-appeasement clauses.** `Macros.scry` has 11
clauses (`Macros.idr:1579–1611`), eight of which match the agent's constructor
and forward to `scryTheirOne`/`Many` exactly as the two catch-alls below them
do; `surveil` mirrors it. `LookReq` (`Macros.idr:1470`) states each gate once
per constructor and `scryTheirOne`/`Many` restate them as erased parameters.
Delete the eight shape clauses; if the checker then loses the `ay = Refl`
refutation, that is a finding to record on `LookReq`, not to paper over per
constructor. Make `scryTheirOne`/`Many` take the `LookReq` constructor's
fields as one record, or take `req` itself.

**F11 — gates matching string labels of an open vocabulary.**
`Card.staticOnSpellCardOk` (`Card.idr:56–61`) matches `Deontic _ Forbid
["Counter"] Patient …`, `["Copy"]` and `["Cast"]` literally, so a coordinated
deed list or any new stack-patient verb fails the card-class check — the fact
it wants ("the patient role's zone is the stack") is already in
`ActFacts.patientRole.roleZone` (`Words.idr:856–865`).
`Card.keywordWantsModes` (`Card.idr:232`) hard-codes `"Entwine"`/`"Escalate"`
instead of a `KeywordFacts` column; `Events.verbForManaOk v = v == "Tap"`
(`Events.idr:339`). (`Triggers.eventName (UnlocksDoor _ _)`,
`Triggers.idr:337`, belongs to `workbench-axis-pairs`.) Derive each from the
facts rows.

**F12 / N5 — facts tables.** `Words.paidCostAgrees` (`Words.idr:2572–2576`)
asserts `paidCostNamed (ByKeyword w) == paidCost f`, but `paidCostNamed` is
*defined* as `paidCost` of the first row matching `w` (`:2562`): it holds by
construction. The law actually missing is distinct keyword names —
`actLabelsDistinct` (`:915`) does that for acts, nothing does it for
`keywordFacts`; add `keywordWordsDistinct`. `Designation` carries five
parallel total tables (`designationScope` `:2965`, `designationChecked`
`:3064`, `designationGiven` `:3083` — row-for-row identical to
`designationChecked` —, `designationSeedZone` `:3118`, `designationSeedType`
`:3137`) where `ActFacts`/`KeywordFacts` use one row record; make it one
`designationFacts`. `ActFacts` duplicates columns: `actPatient : Maybe Kind`
beside `patientRole.roleKinds : List Kind`, and `actDest`/`actLoci` beside
`patientRole.roleZone` (`"Destroy"` carries `actPatient = Just Object` with
`patientRole = MkDeedRole [] [] False (Just Battlefield)`). `actPatient` feeds
`VerbedEvent`'s patient check, `roleKinds` feeds `deedKindOk`; fold
`actPatient` into `roleKinds` (verb-only rows get `[Object]`, which also makes
"can't be destroyed"-shaped deontics admissible — check the `CantBe` pins),
keep `actDest` (a different fact), fold `actLoci` only if a reader needs it.
Rename the `"DrawCard"` label to `"Draw"` to match the effect constructor, as
`"SearchLibrary"` → `"Search"` was, re-spelling the 3 bench deontic sites.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; the duplicate,
agreement-paired, arity-family, forwarding and unused macros are gone or
merged, named in the landing record, with no bench witness deleted;
`EntersTapped` and the `MkToken` pair live in `Macros`; the four as-though
bench lines and the `scry`/`surveil` clauses read through their macros;
`staticOnSpellCardOk`, `keywordWantsModes` and `verbForManaOk` consult facts
rows and no string literal; `paidCostAgrees` is replaced by a real
distinctness law, the five `Designation` tables are one row record, and
`ActFacts` has one patient column; the `CantBe` pins still refute for their
named reasons. Standard constraints apply.
