---
needs: []
---
# The Ability kind's missing heads: bare "ability" and the ability target

Routed from `workbench-event-zone-3-targeting-and-disjunction-arms` (close,
2026-08-26). Two adjacent gaps at the `Ability` kind:

1. **No `AbilityClass` arm denotes a bare "ability"** — the joined head for
   "a spell or ability" (171 supported targeter lines) has no ability half,
   though kind, gate, and `Joined`/`PhJoin` all admit `Object \/ Ability`.
   What a bare "ability" denotes is the design question ([CR#113.3]'s four
   kinds; only activated and triggered target) — answer it from the rules,
   then the cost is one constructor + two `Eq` lines.
2. **`Targetable` has no `Ability` arm**, so "counter target activated
   ability" stays unwritable though [CR#115.2] admits abilities as targets.

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Phrase.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- Both arms land with their rule basis or end in a written verdict; at
  least one "spell or ability" targeter line and one ability-target line
  bench.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** the coordinated anaphor "that spell or ability" (Repeated Reverberation's body — the delayed seat's missing witness): a demonstrative over an `Object \/ Ability` union antecedent, which needs this ticket's bare-ability head. It lands here.

## As-landed

**The bare word "ability" denotes an ability ON THE STACK — the activated
and triggered pair — and nothing wider.** Three rules close the set from
three sides and they agree: only activated and triggered abilities are put
on the stack [CR#113.3b,113.3c]; only they can be countered, static
abilities never using the stack at all [CR#113.9]; and only they are given
the word "target" [CR#115.1c,115.1d]. A spell ability is an instruction
followed while its spell resolves [CR#113.3a] and a static ability is
simply true [CR#113.3d] — neither is ever a thing a card can name. This is
the glossary's SECOND sense of "ability", which [CR#109.1] makes an object
in its own right. So the arm is not "any ability whatever": it is the pair
the stack holds, of which `AnyActivated` is the narrower half.

### Signatures

- `AbilityClass` gains `AnyOnStack` (first arm) plus its two `Eq` rows. No
  other table is total over `AbilityClass`.
- `Targetable` gains `AbilityTgt : Targetable Ability` on [CR#115.2]'s
  clause (b) — "an object that can't exist on the battlefield, such as a
  spell or ability" — with `targetablePhrasal AbilityTgt = PhAbility`.
  WHICH abilities may be targeted stays the head's class, not the row's.
- `NounWord` gains `AbilityJoinW` — the demonstrative that reads an
  `Object \/ Ability` mention back whole. Its six tables gain rows
  (`wordReaches`, `kindOfW`, `verbedWordOk`, `attachHeadOk` ×2,
  `attachHostZone`, `attachHostTy`).
- `wordReaches`'s two union words now part on the ANTECEDENT'S KIND. Both
  mentions are one `JoinP` pair, so the payload cannot tell them apart;
  the binding's `kind` can. Eighteen per-arm rows collapse to two clauses
  over the new `joinedPayload : Payload k -> Bool`. `JoinW` was previously
  reached by ANY `JoinP` binding; it now asks `kindLte Player kd`, which
  is what keeps "that spell or ability" and "that creature or player" from
  reading each other's mention.
- `CounterSpell : {k : Kind} -> (what : Noun bs k) -> {auto 0 ct :
  Counterable what} -> Effect bs`, and `Macros.counterSpell` with it.
- `Counterable` is new, on `CostSubject`/`DamageRecipient`'s model: a
  witness family indexed by the noun, one row per kind. `SpellCountered`
  asks the stack [CR#112.1]; `AbilityCountered` asks no zone, because the
  `Ability` kind places nothing — the same kind writes the abilities a
  permanent HAS — and it is the head's class, not a zone, that says an
  ability reached the stack; `JoinCountered` asks `So (counterKind (ka \/
  kb))`, the new kind-level fold (Object and Ability, joins pointwise).

### What moved, and what it cost elsewhere

Counter LEFT the seven-verb family. `phraseZone`'s "a joined phrase places
nothing" was what refused destroy, exile, tap, untap, return, counter and
sacrifice over a union; [CR#701.6a] writes "a spell or ability", so counter
now has a joined subject of its own and refuses `Object \/ Player` on
`counterKind` — i.e. on [CR#109.1] directly — rather than through a missing
zone. `anyTargetIsPlaceless`'s docstring is re-stated as the SIX-verb fact.

`effEq (CounterSpell a) (CounterSpell b)` is gone: a kind-indexed subject
gives two counter effects no common kind to compare at, exactly as
`Choose`'s row already gave up. The fallthrough `_ = False` stands.

`spellOrAbilityJoin`'s docstring said "no head spells this join yet". It
does now, so the claim is re-stated; the payload witness stays as the
one-half-at-a-time reading.

### Benched

- **Squelch** (whole card) — "Counter target activated ability. Draw a
  card." The ability target: `Macros.target (AbilityHead AnyActivated)`
  through `AbilityCountered`.
- **Diplomatic Escort's line** — "{U}, {T}, Discard a card: Counter target
  spell or ability that targets a creature." The joined targeter head with
  both halves, described by [CR#115.9b]'s own "[spell or ability] that
  targets [something]": `Joined Macros.spell (AbilityHead AnyOnStack)`
  under `Targets`, whose `Targeter (Object \/ Ability)` gate is
  `EitherTargets SpellTargets AbilityTargets`.
- **Shimmering Glasskite** (whole card) — "Whenever this creature becomes
  the target of a spell or ability for the first time each turn, counter
  that spell or ability." The coordinated anaphor, landed: `BecomesTarget`
  announces its targeter as ONE union mention, `That AbilityJoinW` reads
  that mention back whole rather than naming a half, and `JoinCountered`
  counters the pair.

### Pins

- `badCounterPermanent` (existing) re-gated `{zn}` → `{ct = ok}` /
  `SpellCountered impossible`; still fails to elaborate.
- `badCounterJoinedPlayer` — "Counter target creature or player."
  [CR#701.6a] removes from the stack and [CR#109.1]'s list of objects names
  no player. This is the zero of `counterKind`, and it protects exactly
  what the widening changed.
- `badAbilityJoinAnaphorOnPlayerUnion` — "This deals 3 damage to any
  target. Counter that spell or ability." The ability join word does not
  read a creature-or-player mention.
- `badAbilityJoinAnaphorOnAbility` — "Counter target activated ability.
  Counter that spell or ability." A bare ability mention is no pair, so
  the union word misses it; the half's own word reads it back.

### Repeated Reverberation — still blocked, now with both blockers measured

Attempted and refused, twice over:

1. `CopyStack`'s subject is `Noun bs Object` with an `OnStack` gate, so it
   will not take `That AbilityJoinW` at all — the same widening
   `CounterSpell` just received, not done here (nothing else in the corpus
   benches on it alone).
2. With the body swapped for a counter, the elaborator reports
   `countWord AbilityJoinW (delayedCtx [...] (Casts You ...)) = 1`
   unsatisfiable. This is the previous round's own diagnosis, confirmed:
   the three arms' after-discourses disagree, `sharedCtx`'s whole-agreement
   fold hands the body `settleTargets bs` (here `[]`), and there is no
   mention to read. **The bare-ability head was not the blocker.** No arm
   ever makes a union mention; a union mention of the arms would have to be
   a JOIN of disagreeing arm discourses, and "whole agreement, not a meet"
   is what `workbench-event-disjunction-seat` settled. That re-decision is
   the ledger item below, and it is the delayed seat's only remaining gap.

### Ledger

- **A discourse JOIN for disagreeing coordination arms.** When a
  coordination's arms announce mentions of different kinds, the body's
  anaphor names the union, not nothing: "copy that spell or ability twice"
  reads whichever arm fired. Today `sharedCtx` hands the body the outer
  discourse bare. Unblocks Repeated Reverberation and, with it, the delayed
  seat's only whole-card witness. Supersedes the previous round's "A
  coordinated anaphor" line, whose grammar half landed here.
- **`CopyStack` and `ChooseNewTargets` want `Counterable`'s widening.**
  Both carry `(what : Noun bs Object)` with an `OnStack` gate and both are
  written over "that spell or ability" in the corpus ("copy that spell or
  ability", "You may choose new targets for that spell or ability"). The
  gate that fits is the one this round built; it should be renamed for the
  stack rather than for countering when a second consumer arrives.
- **A triggered-ability head.** `AbilityClass` has no `AnyTriggered`, so
  "counter target activated or triggered ability" (Stifle, Disallow, Tale's
  End) stays unwritable — 22 distinct MTGJSON names write "activated or
  triggered ability" and one more writes "target triggered ability". One
  constructor plus `Eq` rows, on the same rules basis as `AnyOnStack`.
- **A join half carries no zone.** `joinHalfPayload` drops the zone, so
  `halfReaches PermanentW` reads the SPELL half of a "spell or ability"
  mention as "that permanent" (its `isNothing ty` test passes). Tolerated
  overgeneration, pre-existing at the payload's shape and newly reachable
  now that the union is writable.

### Family, measured

Counts are distinct card names in MTGJSON `AllPrintings` (all printings,
not the supported corpus): "spell or ability" 248, of which "becomes the
target of a spell or ability" 183 and "target spell or ability" 19; "that
spell or ability" 22; "counter target activated ability" 13.

### Gates

- `idris/scripts/build` on a cleaned `build/` — 23/23, 0 errors, 0 warnings.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,685 citations, 0 stale. [CR#113.9] newly
  registered by `cite bless`.
- `jj diff --git | cargo xtask cite audit --diff` — 39 sites, each read
  against its rule. Three were re-worded on that read: [CR#400.1] was
  carrying a placelessness claim that does not hold of an
  `Object \/ Ability` union (both halves are objects in zones), and two
  `ProofsG` pins leaned on rules that did not say what the sentence
  claimed. The Squelch reminder-text cite moved from [CR#115.2] to
  [CR#605.3b], which is the rule that actually keeps a mana ability off the
  stack.
