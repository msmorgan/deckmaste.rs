---
needs: [lean-v3-checker-and-drt-fixes, lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Fix Rules and Oracle meaning defects across Lean v3 Definitions, Templates, and Cards

**Fix the rules-meaning and Oracle-fidelity defects in `lean/Semantics/Templates.lean`,
`lean/Semantics/Definitions/`, and `lean/Semantics/Cards/` identified in
[`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
(§§1.1–1.17, §3.2, §3.4, §4.7) and
[`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
(§1).** Standard constraints apply.

## Scope

### 1. Negative P/T Doubling & Lossy Counter Movement (Opus §§1.1, 1.11; Astra §1)

- **Negative power/toughness doubling/tripling (§1.1):** Under
  `[CR#107.1b,701.10c,701.11c]`, doubling or tripling a negative power or
  toughness makes it more negative rather than clamping to 0. Fix `double*` and
  `triple*` in `lean/Semantics/Definitions/Actions.lean:500-552` and
  `CharacteristicEdit.numberSlots` in `lean/Semantics/Check/NumberSlots.lean:101-103`
  so P/T doubling/tripling is not marked `.clamped`.
- **Counter movement fidelity (Astra §1, Opus §1.11):**
  - Fix `moveCounters` (`lean/Semantics/Templates.lean:1533`) so moving counters
    (`[CR#122.5]`) preserves the all-or-nothing relationship between removing and
    placing counters and identifies the counters actually removed when no counter
    kind is specified (updating `lean/CONTRACTS.md` "Movement destinations").
  - Fix *Star Pupil* and *Denry Klin* (`lean/Semantics/Cards/Counters.lean:429,397`)
    so they move all counters of each kind (`thatMuch thoseCounters` / `moveCounters`)
    rather than `putCounters (.lit 1) (sameKindAs it)`.

### 2. Keyword & Action Definition Fidelity Fixes (Opus §§1.2, 1.7–1.9, 1.15, 1.17, 3.2)

- **`bloodthirst` (`[CR#702.54a]`, §1.2):** Fix `lean/Semantics/Definitions/Keywords.lean:334-338`
  so `anOpponent` is the damage recipient (`dealDamage (to := a isOpponent)`), not
  the `.actor` dealer.
- **`reconfigure` (`[CR#702.151a,701.3d]`, §1.7):** Fix `Keywords.lean:1201-1202`
  to use `unattach` instead of stamping unattach as `enact (.action "attach")`.
- **`regenerate` (`[CR#701.19c]`, §1.8):** Fix `Definitions/Actions.lean:655-675`
  so creating a regeneration replacement shield does not carry the `"regenerate"`
  deed on creation — only the shield's application (`regeneration`) carries the
  deed — and strengthen `regenerationEventDescribesTheApplicationRatherThanTheShield`
  to pin the deed.
- **`buyback`, `rebound`, and `haunt` spell half (`[CR#702.27a,702.55a,702.88a]`, §1.9):**
  Restrict their `stack → graveyard` replacement/trigger in `Keywords.lean:344-352,796,1185-1195`
  to resolution so a countered spell does not return to hand, rebound, or haunt.
- **`endure` (`[CR#701.63a]`, §1.17):** Fix `endure` in `Definitions/Actions.lean`
  to reference `firstMention .actor` (like `explore` and `connive`) rather than
  `that .permanent`.
- **Smaller keyword fidelity fixes (§1.15):**
  - `timeTravel` (`[CR#702.62b]`): include the `"has suspend"` filter on exiled
    cards.
  - `clash` (`[CR#701.30b..701.30d]`): include the opponent and winner condition.
  - `melee` (`[CR#702.121a]`): count attacked opponents rather than attack events.
  - `shadow` (`[CR#702.28b]`): bind `thisCreature` rather than quantifying over
    every shadow creature in the second static ability.
- **Route keyword bodies through keyword-action `Definitions` (§3.2):**
  - Replace raw `.search` / `.shuffle` in `transmute` and `transfigure`
    (`Keywords.lean:1439-1460`), raw `.exchange` in `auraSwap`, raw
    `.expose .reveal` in `ninjutsu`/`splice`/`amplify`, and raw
    `setStatus .untapped` in `doesntUntap` with the corresponding `Definitions`
    so their deeds are preserved.

### 3. Card Encoding Fidelity Fixes (Opus §§1.3–1.6, 1.10, 1.12–1.14, 3.4, 4.7)

- **Celestial Convergence (`Cards/Counters.lean:205-213`, §1.3):** Nest the
  life-tie → `drawGame` branch inside the `"if there are no omen counters on ~"`
  condition.
- **Berserk & Ghor-Clan Rampager (`Cards/Turn.lean:512-516`, `Cards/Cost.lean:731-734`, §1.4):**
  Bind `target isCreature` once and reference `it` across the granted ability and
  P/T modification instead of repeating `target isCreature` three times (and drop
  *Berserk*'s no-op `+0` toughness edit).
- **Inkshield (`Cards/Damage.lean:451-454`, §1.5):** Prevent `some .combat` damage
  (not `none`), and give the created 2/1 white and black token the `inkling`
  subtype and `flying`.
- **Mishra's Factory (`Cards/Mana.lean:141,145`, `[CR#205.3m]`, §1.6):** Use the
  catalog `assemblyWorker` (`"Assembly-Worker"`) subtype instead of the
  misspelled `"AssemblyWorker"`.
- **Performer handoffs on discard, unless-pays, and damage (§§1.10, 4.7):**
  - Wrap `"that player discards"` and `"each player/opponent discards"` in
    `instruct <player> (discard ...)` (`[CR#701.9a]`) on *Hypnotic Specter*
    (`Cards/Anaphora.lean:250`), *Immersturm Skullcairn* (`Cards/Damage.lean:47`),
    *Oona's Blackguard* (`Cards/Counters.lean:728`), *Sonic Shrieker*
    (`Cards/Damage.lean:764`), *Rankle* and *Braids's Frightful Return*
    (`Cards/Choice.lean:59,87`), `Cards/Piles.lean:202`,
    `Cards/Counters.lean:1218`, and `Cards/Keyword.lean:1055`.
  - Fix `"unless its controller pays"` on *Mana Leak* (`Cards/Anaphora.lean:151`),
    *Override*, *Rakshasa's Disdain*, *Rune Snag* (`Cards/Choice.lean:258,877,884`),
    and *Tidal Flats* (`:907-916`) so the spell/ability's controller — not the
    invited payer — performs `counter it`, and generalize `payLife` to `Amount`
    so *Killing Wave*, *Heat Wave*, and *Font of Agonies* use `payLife` instead of
    `perform (loseLife ...)`.
  - Standardize damage-source performer handoffs across `Cards/*.lean` (and record
    the convention in `lean/CONTRACTS.md`).
- **Phyrexian Infiltrator (`Cards/Anaphora.lean:64-67`, `[CR#701.12a,701.12b]`, §1.12):**
  Use `exchange (controlOf thisCreature) (controlOf (target isCreature))` instead
  of `simultaneously [gainControl ..., gainControl ...]`.
- **One-shot resolution `"cast it"` (`[CR#608.2g]`, §1.13):** Encode *Memory
  Plunder* (`Cards/Choice.lean:1005-1015`), *Flawless Forgery*
  (`Cards/Copy.lean:56-61`), and *Arcane Proxy* (`Cards/Faces.lean:601-614`) as
  one-shot optional casting during resolution (`may (cast ...)`) rather than a
  durationless static permission `establish (can you (cast ...))`.
- **`"Return"` vs `"put"` (§3.4):** Re-express *Unsummon*
  (`Cards/Anaphora.lean:616`), *Wind Zendikon* (`Cards/Keyword.lean:345`),
  *Cloudshift* (`Cards/Anaphora.lean:22`), *Flickering Spirit* (`:36`), *Synod
  Sanctum* and *Cold Storage* (`Cards/Cost.lean:76,85`), and
  `returnToBattlefieldWithCounters` (`Templates.lean:1160`) through `returnTo*`
  so they carry `.core .return_`.
- **Restore dropped Oracle clauses (§1.14):** Complete the omitted clauses on
  *Qarsi Deceiver* (`Cards/Mana.lean:300`), *Overgrown Zealot* (`:277`),
  *Candles' Glow* (`Cards/Damage.lean:440`), *Giggling Skitterspike*
  (`Cards/Trigger.lean:223`), *Cyclone* (`Cards/Counters.lean:542`), *Vedalken
  Squirrel-Whacker* (`Cards/Anaphora.lean:264`), and *Purging Scythe*
  (`Cards/Damage.lean:227-234`).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
