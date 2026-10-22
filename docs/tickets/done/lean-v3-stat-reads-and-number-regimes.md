---
needs: []
---
# Zone-aware loyalty and defense reads; signed letter definitions; name `differ`; `asSorcery` priority

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#20 (Lean half), #52, #68, #70. Verified against head 2026-10-21. All latent in Lean.

## 1. Loyalty and defense read counters on the battlefield, the printed number elsewhere

One Lean term carries two reads. The *Nahiri* witness `nahiriLoyaltyRead` (`Cards/Mana.lean:665-668`, `hasManaValue .less
(statOf (.stat .loyalty) this_)`) needs the live loyalty; the [CR#306.5b] conferral
(`Proofs/Rules.lean:64-69`, `planeswalkerLoyalty`: `entersWithCounters .this (.statOf (.stat
.loyalty) .this) …`) needs the printed number. "The loyalty of a planeswalker on the battlefield
is equal to the number of loyalty counters on it" [CR#306.5c]; off the battlefield it is the
printed number [CR#306.5a]. A battle's defense is the same pair [CR#310.4a,310.4c]. *Nahiri*'s
ruling reads loyalty "as the ability tries to resolve".

Change: make `Stat.loyalty` and `Stat.defense` (`Words.lean:40`) zone-aware: counters on the
battlefield, printed elsewhere, and say so in their docstrings. The entry conferrals
[CR#306.5b,310.4b] must read the printed number.

STOP for an owner ruling before writing the conferrals. The owner decided on 2026-10-13
([lean-defense-projection](lean-defense-projection.md), `semantics-v3.md` §3.7) that the
[CR#310.4b] row is written `statOf(stat(defense), this)`, the shape of the planeswalker row. Under a
zone-aware `Stat` that shape stays correct only if an entering object's read is taken before it is
on the battlefield (the entry replacement applies as it enters [CR#614.1c]). The review proposes an
explicit printed-value projection instead. Bring both options to the owner; do not rewrite the
decided shape in either ticket without that ruling. The Rust half (`resolve/count.rs`, `layer.rs`, `target.rs`, `trigger.rs`, the
`LoyaltyCounter.ron` comment and the count test that pins printed-on-battlefield) is
[engine-loyalty-defense-zone-reads](engine-loyalty-defense-zone-reads.md). Pins: *Nahiri*'s filter
after a loyalty ability's cost was paid reads the reduced count; the conferral reads the printed
number.

Ruled (2026-10-22, owner): option B, an explicit printed-value projection. `statOf loyalty` and
`statOf defense` keep their zone-aware reading [CR#306.5a,306.5c,310.4a,310.4c]; the intrinsic
entry rules read the printed number directly [CR#306.5b,310.4b], through `ProjAxis.printed`.
Every decomposable rule is modelled as data in the DSL, `Proofs/Rules.lean` prototypes those
rows, and their citations (not the checker) carry their correctness, so a row states the rule's
own words with no zone-timing reliance. This supersedes the 2026-10-13 conferral shape
(`semantics-v3.md` §3.7).

## 2. A letter's definition is signed; clamping belongs to the consuming slot

`Check/NumberSlots.lean:132-134` (`-- … never negative [CR#107.1b,107.3]`, `| .letterDefinition _
amount => [(amount, .clamped)]`) and `:184-185` clamp the value X is defined as. [CR#107.1b]
forbids choosing a negative number and says "If a calculation or comparison needs to use a negative
value, it does so"; only a calculation "that would determine the result of an effect" uses zero
instead. [CR#107.3] sets no floor on a defined X. Under the clamp, *Guardian Scalelord*'s "mana value
X or less … where X is this creature's power" with power -1 finds mana-value-0 cards; under the CR
it finds none.

Change: give `letterDefinition` the `.signed` regime (`NumberSlots.lean:40`) and leave clamping to
the slots that consume X. Rewrite `okDefineClamped` and `okDefinesLetterClamped`
(`Proofs/Numbers.lean:75-82`) to assert the signed definition, and add a comparison pin with a
negative X.

## 3. `differ` on names: a nameless object has no different name

`Agreement.holdsOn` (`Words.lean:136-138`) is `| .differ, mine, theirs => !mine.any
theirs.contains`, so `holdsOn .differ [] ["Fire"] = true`, cited to [CR#201.2c]. That rule: "If the
first object has no name, it does not have a different name than any of the other objects"; for a
group, "only if each of them has at least one name and no two objects in that group have a name in
common" [CR#201.2b]. On the name axis only, require `mine` non-empty, and give the group form
(`withDifferentNames`, `Templates.lean:702-703`) the [CR#201.2b] semantics. Pins: `holdsOn .differ
[] ["Fire"] = false` beside `differIsNoValueInCommon` (`Proofs/Sharing.lean:31-35`), and a
face-down group case (*Liliana's Contract*).

## 4. `asSorcery` requires priority

`asSorcery` (`Templates.lean:871-873`) is main phase plus empty stack, with no priority conjunct,
while `asInstant` (`:875`) has one. [CR#307.5]: "the player must have priority, it must be during
the main phase of their turn, and the stack must be empty"; the negation therefore loses
[CR#307.5a]'s "without having priority". Add `.onlyIf (.hasPriority .you)`. Not observable on
printed cards (*Mystic Veil* ruling, 2022-12-08; `Proofs/Turn.lean:571`), so the pin is on the
tree.

## Verification

`./lean/scripts/build`; `cargo xtask lean check` if a Definition changes; the CR citation gates.
