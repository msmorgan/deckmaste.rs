---
needs: [workbench-corpus-frame-gate]
---
**Correct the bench's wrong printed frames, add the dropped printed abilities,
record every remaining omission in place, and re-spell the raw-core sites a
macro already covers.** Cleanroom review 2026-09-03, F3 plus the raw-core
census in F14. The gate from `workbench-corpus-frame-gate` names the frame
half exactly; this ticket drives its count to zero.

## Wrong frames (21)

Seven P/T boxes: Wormfang Manta `(6,6)` vs printed 6/1 (`Cards.idr:6964`),
Wall of Shards `(3,6)` vs 1/8 (`Cards.idr:14475`), Council of the Absolute
(`Cards.idr:6122`), Centaur of Attention (`Cards.idr:10007`), Soldevi Adnate
(`Cards.idr:11998`), Myr Prototype (`Cards.idr:14419`), Keeper of the Flame
(`Cards.idr:15341`).

Fourteen mana costs: Sugar Coat carries a white pip for `{2}{U}`
(`Cards.idr:12486`); Omniscience is one `{U}` short (`Cards.idr:10822`); Mox
Diamond spells `{0}` as `Nothing` (`Cards.idr:12949`) while Dark Sphere spells
it `Just []` (`Cards.idr:4617`) — pick one spelling for `{0}` and apply it
everywhere.

Six short type lines: Wall of Shards lacks `Snow`, Phyrexian Revoker lacks
`Phyrexian`.

## Dropped printed abilities (8)

Flashback ×3 (Daydream `Cards.idr:342`, Cackling Counterpart
`Cards.idr:3861`, Flaring Pain `Cards.idr:4197`), double strike (Char-Rumbler
`Cards.idr:943`), flash (Retro-Mutation `Cards.idr:15183`) — all five
provably spellable (`keywordCosting "Flashback" (Mana …)` typechecks). The
remaining three are blocked elsewhere and are **not** this ticket's to fix:
Wind Zendikon's dies trigger (`Cards.idr:3258`), Grievous Wound's damage
trigger (`Cards.idr:5242`, `workbench-binding-regressions`) and Chromatic
Armor's two sleight-counter lines (`Cards.idr:5531`,
`workbench-counter-kind-open`) — each gets an `Unspellable` twin or a doc line
naming the sentence and the blocking ticket, so it is visible from inside the
file.

`Unspellable` is used 0 times in `Cards.idr`, so today a dropped sentence and
a refused one are indistinguishable. Every omission the bench keeps must be
recorded in place.

## Raw core where a macro exists (102 sites, 71 definitions)

`HasType Land` 15, `KeywordAbility kw Nothing` 14, `HasType Creature` 13,
`ChangeLife w (Up/Down n)` 19, `HasStatus Untapped` 8, `Continuously (Gets …)
d` 7. `openTheVaults` (`Cards.idr:13424`) alone carries four;
`forceOfWill` (`Cards.idr:7763`) writes `Do (ChangeLife You (Down (Lit 1)))`
where `payLife You 1` is definitionally that term. Per the 2026-08-26
refinement of `workbench-mirrors-semantics-v2-structure`, the bench spells
through the macro where one exists — including the one raw `Enact` site.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; the corpus-frame
gate reports 0 mismatches; the five spellable keywords are on their cards; the
three blocked sentences each carry an `Unspellable` twin or a doc line naming
the sentence and its blocking ticket; the 102 raw-core sites read through
their macros with no witness deleted; the landing record gives the before and
after gate counts and the chosen `{0}` spelling. Standard constraints apply.
