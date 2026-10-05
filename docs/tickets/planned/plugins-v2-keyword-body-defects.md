---
needs: []
---
**Keyword declarations that are wrong or inconsistent, found while surveying
them for re-spelling (2026-10-04).** None is a re-spelling, so none is fixed by
`plugins-v2-keyword-bodies-over-helpers`. Split this ticket when one is picked
up. Standard constraints apply.

- **`gift` cannot be invoked.** It declares `params: [Subject]` and puts that
  parameter where a predicate is required. Owner's direction: gift is
  non-normative and gets one macro per printed variant (`giftACard`,
  `giftATappedFish`, …). Take the variant list from the CR snapshot.
- **`champion`, `enchant`, `companion`, `foretell` declare a parameter the
  keyword term drops.** Owner: these "may need new mechanisms that allow
  different parts of the game state to be addressed that currently cannot
  be". They carry `keyword_params: []` until then. `enchant` is marked
  "Settled empty" and still drops its subject.
- **`prototype`** forwards nothing by design: its cost, power and toughness are
  the inset frame's alternative characteristics [CR#718.1,718.2], modelled as
  `Card::Prototype`. Its declared params describe the printed line only. Listed
  so nobody "fixes" it.
- **`returnToBattlefieldWithCounters`** omits the return wrapper that persist
  and undying write by hand. One of the two is wrong.
- **`demonstrate`** refers to its copy two different ways in parallel
  branches.
- **`amass`** has no `Enact` wrapper while `bolster` and `adapt` do.
- **Storm, split second and surge** flatten "another spell" into one
  conjunction; **bushido, exalted, flanking, melee, prowess and rampage**
  repeat the subject in the toughness half. Lean's macros write both
  differently. Decide which side is right.
- **The `exile` Keyword Action has no agent** and eight bodies write the raw
  wrapper to get one. Folded into `semantics-v2-action-agents`.
- **The bodyless `exchange` Keyword Action** shadows any alias for the
  `Exchange` instruction, so `auraSwap` keeps a constructor.
- **24 Keyword Actions have no body.** Lean has bodies for fight, scry and
  surveil.
- **Scope.** `assemble` is defined as "a keyword action in the Unstable set"
  [CR#701.45a]. `hiddenAgenda`, `spaceSculptor` and `visit` were flagged as
  possibly out of scope from memory, unverified; check them against the card
  data before removing anything.

## Residue from the keyword-body sweep (2026-10-04)

- **Helpers proposed, not built; the owner has not ruled on names.**
  `mayOrElse(player, body, otherwise)` for "you may …; if you don't, …"
  (fabricate, fading and madness fall to named calls because the "if you do"
  slot comes first); one change applied to both power and toughness for
  "+X/+X" (melee, rampage); a token-copy helper for myriad's tapped-and-attacking
  copy; a token helper that takes extra card types (fabricate's Servo); a
  general cast permission (`airbend`, aftermath); a bound on "can't be blocked
  by" (menace).
- **Named calls forced by parameter order.** `returnToBattlefield` (riders
  before agent and origin), `activated` (limit before guard), `deonticRule`,
  `damage`, `verbedEvent`, `addMana`.
- **The counter and type declarations still write raw conferrals.** `types/`
  is skipped by `cargo xtask expansions`, so give it coverage before
  re-spelling.
- **Helper macros' own bodies** were left for a later pass.
- **The cost-action rule has no recorded rationale and is inconsistent.** It
  refuses a sequence but accepts one inside a fixed repeat, and accepts
  conditionals and loops. `badSequentialCost` was ported from Idris without a
  reason. Costs are paid "in any order" [CR#601.2h], which is a possible
  basis; re-derive the rule from that when costs are next touched.
- **`isCard` is written inconsistently.** The Lean bench writes "a card in
  hand" without it; RON writes `cardIn(hand)`. A token can sit in a hand,
  graveyard or exile until state-based actions are checked [CR#111.7,704.5d],
  so the conjunct is meaningful; bring the bench into line.
