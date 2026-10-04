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
