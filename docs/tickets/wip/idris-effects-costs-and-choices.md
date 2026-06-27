---
needs: []
---
**Idris grammar: effect / cost / choice capability gaps.** `[design]` net-new primitives in
`idris/src/Core.idr`, grouped because they share the cost/choice/static design surface.
From the 2026-06-26 grammar census. Split a line into its own ticket if it gains card
pressure.

**Costs & activation**
1. **Activate from a non-battlefield zone.** Optional `from : List Zone` on the activated
   (and triggered) ability constructor — Cycling/Channel/Forecast (hand), Embalm/Unearth
   (graveyard). Mirrors the `ActivatedAbility.from` already added engine-side.
2. **Activation guard.** `activationGuard : Maybe (Condition b)` on `Activated` — "only if it
   attacked this turn" (Boast), Metalcraft, "if you control 3+ Caves". An `If` in the body
   is wrong (the cost stays payable).
3. **Cost references + paid-object binder.** `ManaCostOf : Reference -> Cost` ("pay its mana
   cost" — Snapcaster, Scavenge, Bestow, Emerge) and a binder exposing the paid/sacrificed
   object as a reference (Emerge "X = its toughness").
4. **Tap/exile-to-contribute `CostChange` variants.** Convoke/Improvise reduce the mana cost
   one pip per tapper; Delve/collect-evidence exile to pay generic. These are cost
   *reductions*, not the `TapTotal` stat threshold the comment once claimed to subsume.
5. **Alt-cost machinery.** `WasCastWith : AltCostTag -> Predicate` (Dash/Evoke/Spectacle),
   `TimesKicked : Count` (Multikicker), and a `{when : Condition}` availability guard
   (Prowl/Emerge/Blitz).

**Choices & players**
6. **Multiplayer / open choices.** `Vote` (Council's Dilemma), `DivideAndChoose` (Fact or
   Fiction — one divides, another picks), `AnyPlayerMayPay : Cost -> … -> …` (Rhystic).
7. **Mid-resolution value choice + modal extensions.** `WithChosenValue : ChooseDomain ->
   OneShotEffect (bindChosen d b) -> OneShotEffect` ("choose a color, add that mana" — Three
   Tree City; `AsEnters` is enters-only). Plus `ChooseSpec` extensions: per-mode cost budget,
   cross-invocation mode exclusion, random chooser, Entwine.
8. **Player statics & life.** `SetLifeTo : Reference b APlayer -> Count -> Action` (Biorhythm
   — setting [CR#118.5] fires no delta trigger), `HandSizeLimit`, `AdditionalLandPlay`
   (Exploration), and a `PlayerStatCmp` *exists*-comparator over players ("an opponent has
   ≤10 life"). Pairs with `PlayerStatOf` in `idris-characteristic-read-unification`.

**Static / replacement / stack**
9. **Ability modification.** `LoseKeyword : KeywordSpec -> Modification` (selective — "loses
   flying"; `LoseAbilities` is all-or-nothing) and `InheritAbilities : Reference b AnObject ->
   Modification` (grant a runtime object's abilities — Conspicuous Snoop, Necrotic Ooze).
10. ~~`CantReplace`~~ **— STRUCK; not a gap.** These aren't replacement-suppression: "can't be
    countered/regenerated" is the `cant` deontic on the `Enact` spine (`cant (Enact Counter …)`
    already works); Split Second is `Continuously (UntilEvent <leaves Stack>) (cant (Enact
    Cast/Activate Anyone …))` (with a mana-ability carve-out); and "damage can't be prevented" is
    PREVENTION ([CR#615]), a separate mechanism from replacement ([CR#614]). The only residual is
    a micro-extension: add `Regenerate` to the `Enact` `Relation` set so "can't be regenerated"
    expresses on the spine. (Prevention machinery, if needed, is its own item — not this.)
11. **`Modify` reaching non-battlefield zones** — graveyard/hand statics (Satoru hand-ninjutsu,
    Riftstone Portal). Add a source-zone field on `Modify`.
12. **Enter attacking** — optional `enteringAttacking : Maybe (Reference b APlayer)` on
    `CreateToken`/`Move` (Ninjutsu, Myriad, Encore, "return tapped and attacking").
13. **`OutsideGame` zone** — paper Wishes & Lessons. Add it to `Zone`.
14. **Stack-object ops** — `CopyAbility` (Rings of Brighthearth), `ChangeTarget`/`Redirect`
    (Bolt Bend), stack-object `ChangeController` (Aethersnatch, Commandeer).
15. **Mutate / Banding** — engine-recognized primitives (Mutate's merged permanent; Banding's
    damage-assignment delegation); no composable form today.

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
