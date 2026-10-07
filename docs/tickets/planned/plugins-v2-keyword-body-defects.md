---
needs: []
---
**Residue of the 2026-10-04 keyword-body survey: what is deferred, what stays
as it is on purpose, and what is still unwritten.** Split on 2026-10-05; the
actionable items moved to their own tickets (table below). What remains here
is either a recorded decision not to change something, or a backlog with no
card asking for it yet. Standard constraints apply.

## Where the other items went

| Item (old wording) | Now in |
|---|---|
| `gift` cannot be invoked | `plugins-v2-gift-variants` |
| The bodyless `exchange` shadows the `Exchange` alias; `auraSwap` keeps a constructor | `plugins-v2-exchange-helpers` |
| Scope: `assemble`, `hiddenAgenda`, `spaceSculptor`, `visit` | `plugins-v2-out-of-scope-keywords` (`assemble` stays, blank) |
| Lean has bodies for fight, scry and surveil | `plugins-v2-scry-surveil-fight-bodies` |
| Helpers proposed, not built (`mayOrElse`, "+X/+X", token copy, Servo token, cast permission, menace's bound) | `plugins-v2-keyword-helper-additions` |
| Named calls forced by parameter order | `plugins-v2-keyword-helper-additions` |
| `returnToBattlefieldWithCounters` omits the return wrapper | `plugins-v2-keyword-helper-additions` (delete it; see below) |
| Storm, split second and surge flatten "another spell" | `plugins-v2-keyword-helper-additions` |
| Bushido, exalted, flanking, melee, prowess and rampage repeat the subject in the toughness half | `semantics-v2-gets-both-deltas` |
| `isCard` written inconsistently | `plugins-v2-keyword-helper-additions` |
| `amass` has no `Enact` wrapper | `semantics-v2-actor-handoff` |
| The `exile` keyword action has no agent | `plugins-v2-implicit-actor-spelling` (`exileBy` retires) |

## Corrections to the old text (2026-10-05)

- "Eight bodies write the raw wrapper" for exile is false now: eight files call
  the `exileBy` helper (ingest, scavenge, myriad, embalm, eternalize, recover,
  unearth, forage), which the handoff turns into `exile(…)` under the actor.
- Old text quoting `aAtRandom` now reads `aRandom`.
- `returnToBattlefieldWithCounters`: "one of the two is wrong" was mistaken.
  The RON helper has no callers, and persist and undying already go through
  `returnToBattlefield`; the helper is deleted.

## Stays as it is, on purpose

- **Sequenced costs stay refused** (`Instruction.costActionOk` in
  `lean/Semantics/Check/Abilities.lean`, pin `badSequentialCost` in
  `Proofs/Deontic.lean`). The reason, to be recorded beside the rule: costs are
  paid "in any order" [CR#601.2h], which a cost written as an ordered
  sequence would contradict, and no Vintage-legal card has "then" inside a
  cost. The rule's other inconsistencies (it
  accepts a sequence inside a fixed repeat, and accepts conditionals and loops)
  wait until costs are next touched.
- **Deferred, unchanged:** `champion`, `enchant`, `companion`, `foretell`
  declare a parameter the keyword term drops and carry `keyword_params: []`.
  Owner: they "may need new mechanisms that allow different parts of the game
  state to be addressed that currently cannot be". `enchant` is marked
  "Settled empty" and still drops its subject.
- **`prototype`** forwards nothing by design: its cost, power and toughness are
  the inset frame's alternative characteristics [CR#718.1,718.2], modelled as
  `Card::Prototype`. A frame fact; listed so nobody "fixes" it.
- **`demonstrate`** stays on the raw allowlist with `Pro(Word(Copy), One,
  top(1))` for "that copy" (the chosen player's copy, not the caster's). No
  `latest` helper is built: one use does not justify a name.

## Backlog, waiting for a card or a pass

- **21 keyword actions have no body** and wait for a card that needs one:
  activate, assemble (blank on purpose), cast, cloak, collectEvidence,
  connive, discover, double, exchange (bodyless on purpose, for parsing),
  fateseal, incubate, learn, manifest, manifestDread, play, regenerate,
  support, theRingTemptsYou, triple, ventureIntoTheDungeon, waterbend.
- **The counter and type declarations still write raw conferrals.** `types/`
  is skipped by `cargo xtask expansions`, so give it coverage before
  re-spelling. (The counter declarations are rewritten by
  `semantics-v2-counter-kind-is-a-name`; do this after it.)
  Unblocked 2026-10-06: `semantics-v2-counter-kind-is-a-name` is done, so the
  counter and type raw-conferral re-spells this item was holding can go
  ahead. They are Sonnet-mechanical.
- **Helper macros' own bodies** were left for a later pass.
