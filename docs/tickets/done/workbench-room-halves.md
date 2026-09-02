# workbench-room-halves

The Room half-referent programme, declined by keyword-tail (2026-09-02) with
the fence cited and the reduction disproved: [CR#709.5j] makes a door a HALF,
[CR#709.5b] makes halves part of one object's copiable values, and
[CR#709.5c] keys the unlocked designations on the permanent with the half
named inside — so "unlocked door" is no single-referent predicate and the
door noun does not reduce to existing machinery. Needs: a half-level referent
+ relation, a half-keyed designation read, then the door noun [CR#709.5j],
the unlock trigger header [CR#709.5h] (28 identical "When you unlock this
door" lines), and the fully-unlock header [CR#709.5i] (16 lines, all Eerie).
Counts re-measured at keyword-tail's close. The designations and
`SpecialAction.UnlockDoor` are landed.

## As landed

The reduction stayed disproved and the half referent was built to the shape
the decline had derived.

**The half referent.** `RoomHalf = LeftHalf | RightHalf` is a SORT and not a
`Kind` — [CR#709.5b] makes each half part of one object's copiable values, so
a half is a named part of an object, never a referent beside objects and
players. `Door` (Triggers.idr) is the referent *plus* its relation: `ThisDoor`
is [CR#709.5j]'s deixis on the half an ability is printed on, `DoorOf state
room` an indefinite half of a described battlefield permanent carrying
[CR#709.5c]'s lock adjective (`LockState = Locked | Unlocked`). The half-keyed
designation read is `halfDesignation : RoomHalf -> Designation`, with
`designationHalf` its inverse; `halfDesignationInjective` and
`designationHalfInverse` are what make [CR#709.5c]'s "the *appropriate*
unlocked designation" a definite description. The lock adjective sits on
`Door` beside the permanent, not inside it, exactly because the read takes the
pair.

**The carrier.** `SharedLineSplit` (Card.idr) is [CR#709.5]'s permanent card
with a single shared type line, with `SharedLineHalf` (name, cost, text — no
line, [CR#709.5a] taking it off the half; no box, [CR#200.1] putting it on the
card) and `SharedLineHalfLaws`. Half the old decline was that no card could
carry the header; that is paid.

**The headers.** [CR#709.5h] is `UnlocksDoor` (28 lines, the identical
template), keyed to `VerbedAct "Unlock"` — one new `verbFacts` row whose
`actPatient` is absent, a half being no `Kind`. [CR#709.5i] needed **no door
at all**: "you fully unlock a Room" names a permanent, so it is `VerbedEvent`
at the new "Fully Unlock" label with `actPatient = Just Object`, and the 16
Eerie lines' second trigger word makes it a `JoinedHeader`.

**Also landed:** the `Unlock` effect ([CR#709.5f]), gated on
`DoorNamesHost` — the act chooses among the halves of a *named* permanent, so
the deixis is refused there. And the door frame law `doorFrameOk`, restated at
`FaceLaws`/`AltFaceLaws` and dropped at `SharedLineHalfLaws`: "this door"
prints only on a face that has a half.

**Benches:** `glassworksShatteredYard` (whole, shared line), `balemurkLeech`
(whole, Eerie fully-unlock), `ghostlyKeybearer` (whole, `DoorOf` witness).
The part-bench `glassworksTrigger` retired into the whole card.

**Pins:** `badUnlockThisDoor`, `badDoorHeaderOffSharedLine`,
`badUnlockDoorOffBattlefield`.

**Remainders** (recorded in Cards.idr's Room section, none of them on the half
referent): 2 "Lock or unlock a door of target Room you control" + Ghostly
Dancers' disjunct want an effect-level disjunction of two acts over one door
(and [CR#709.5g]'s Lock label with it); 3 counting reads want a plural door —
halves of a described group, counted, with distinct names among them; the 2
mana-spend restrictions were already spelled by `SpecialAction.UnlockDoor`.
`abilityNamesThisDoor` reads the header and the two wrappers but does not walk
effect bodies, so a deixis buried in a delayed trigger escapes the frame — 0
supported lines write one.
