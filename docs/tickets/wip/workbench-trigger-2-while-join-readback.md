# trigger-2: the while clause, the two-header join, and the readback

Sub-round 2 of [workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
(the umbrella — authoritative). Owns its sections: "The header-internal
'while' clause" (60 lines, at least twelve condition shapes, three naming an
action IN PROGRESS — its own design, not a marking variant; [CR#603.4] does
not reach it), "The 'and whenever' two-header join" (58 lines — two trigger
WORDS joining two whole headers over one effect; distinguished from `AltEvent`
by the second word; Autarch Mammoth benches), and "The coordinated header's
readback" (144 of 364 — START at the decidable `Bindings` comparison, not the
carrier: the 46 "blocks or becomes blocked" lines announce the SAME phrase on
both sides and a common-announcement read admits them; Aisling Leprechaun
benches; the remaining 98 stay refused and counted).

Coupling to respect (umbrella "From the v1 comparison"): the readback's
decidable comparison is the same one a general event disjunction would need —
whichever lands first settles it for both, and a general disjunction retires
`AltEvent` rather than sitting beside it. Veiling Oddity: the while clause is
one of its two blockers; record the other (the self's zone evidence) — do not
chase it. Standard constraints apply.

## As landed (2026-08-27)

All counts re-measured over `data/derived/cards.jsonl` through
`jq 'select(.supported)'`, on distinct printed LINES. Gate:
`idris/scripts/build` clean, 23/23, exit 0.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 0 stale over
18,610 citations; `cite bless` registered no new rule (1531 rules, lock
unchanged); `jj diff --git … | cargo xtask cite audit --diff` read 32 citation
sites, each rule read against its claim.

### Built

- **The header's concurrent "while" clause** — `Triggers.Concurrent`, a slot
  of its own on `Triggered` sitting BEFORE the intervening one, and a second
  copy on the joined header. Its rule is [CR#603.1]'s own template: the
  header's slot is "[trigger condition or event]", so a condition named
  inside the header is part of what triggered, and [CR#603.2] triggers on a
  game event "or game state". [CR#603.4] declines it outright ("the word
  'if' has only its normal English meaning anywhere else"; "this rule only
  applies to an 'if' that immediately follows a trigger condition"), and the
  rules content differs, not just the spelling: the intervening condition is
  checked again on resolution and this one is not.
  Re-measured: **61** header-internal lines (umbrella said 60), **21**
  distinct clause shapes (umbrella said "at least twelve"), the saddle family
  **26** of them (umbrella said ~25). **The zero comma-markings hold**: `,
  while ` occurs in 0 supported trigger lines, and the clause always closes
  with the header's own comma. Recorded on the `Concurrent` docstring.
  Two arms, as the corpus writes two things after the word:
  `WhileTrue` takes an ordinary `Condition` (58 of the 61 lines — the shapes
  are ones `Condition` already spells), and `WhileDoing` takes a `GameEvent`
  read as UNDERWAY (the 3 in-progress lines). The act arm is gated by
  `Events.eventUnderwayOk`, whose three open cells are the three rules that
  put ordered steps inside an event: [CR#601.2] casting, [CR#602.2]
  activating, and the keyword actions that answer for themselves through a
  new `VerbFacts.actStepwise` column ([CR#701.22a] scry looks and THEN puts;
  [CR#701.25a] surveil writes the same two steps). Everything else is False
  on [CR#603.2]'s ground and pinned at `badWhileDoingMoment`.
  **Benches: Seasoned Warrenguard whole, Brazen Blademaster whole,
  `veilingOddityLine`, `whileScrying`.**
- **The "and whenever" two-header join** — `Triggers.JoinedHeader` plus a
  `joins` list on `Triggered`, and `joinedCtx` for what the ability's own
  clauses may read. Re-measured **59** supported lines (umbrella said 58).
  **The decision is a second HEADER, not a second event arm or a second
  ability**, and the evidence is what the printed second half carries:
  its own trigger word (11 "and when", 38 "and whenever", 10 "and at",
  against every first word — 41 "When", 11 "Whenever", 7 "At"), its own
  WINDOW (Voice of Resurgence, "Whenever an opponent casts a spell during
  your turn and when this creature dies") and its own CONCURRENT clause
  (Autarch Mammoth's "while saddled", which qualifies the attack alone). An
  `AltEvent` arm is a bare event and cannot carry any of those, since the
  header states word, window and while once for every arm. It is ONE ability
  and not two: [CR#603.1] gives an ability one effect, the printed line
  writes one after both headers, and MACH-1's "This ability triggers only
  once each turn" is singular over a two-headed line — so the limit, the
  intervening clause and the effect stay the ability's.
  A LIST on `AltEvent`'s model rather than a `Maybe`, so the two
  coordination seats read alike; **zero supported lines write a third
  header**, recorded rather than gated.
  **Benches: Up the Beanstalk whole, `autarchMammothLine`** (join and
  concurrent clause in one header).
- **The coordinated header's readback — ALREADY LANDED; nothing to build.**
  The umbrella's premise is stale (deviation, below): the decidable
  `Bindings` comparison it asks for is `Words.sameBindings`/`sameBinding`/
  `samePayload`, `Triggers.armsAgree`/`sharedCtx` is the common-announcement
  read built on it, and `headerCtx` is `sharedCtx eventAfter` — so the
  same-phrase coordinations already write and **Aisling Leprechaun was
  already on the bench**, beside Chub Toad and Inferno Elemental.
  Re-measured with the definition stated: **336** distinct supported header
  lines coordinate EVENTS under one trigger word (umbrella: 364); **134** of
  them read a mention back in the body (umbrella: 144); **44** of those
  headers are "blocks or becomes blocked" (33 with a partner phrase, 11
  bare), of which **34** read a mention back and all write today (umbrella:
  46). **The remaining 100 stay refused and counted**, pinned at
  `badAltHeaderMixedReadback` and `badThreeArmHeaderReadback`.
  The join's readback reuses the same comparison — `joinedCtx` is
  `sharedCtx`'s rule at the second seat — which is the coupling the umbrella
  asked for, paid as the general-purpose piece.

### Pins

- `badWhileDoingMoment` — "while a creature is dying" names a moment inside
  an event that has none: only [CR#601.2] and [CR#602.2] put ordered steps
  inside one, and [CR#700.4] makes a death a zone change.
- `badJoinedHeaderReadback` — the join's readback where the two headers
  announce different things, on `badAltHeaderMixedReadback`'s ground.
- `badChapterWhile`, `badChapterJoin` — [CR#714.2b] writes the chapter
  header whole, so it has nowhere to hang either new slot.
- `badExileCheckOnSortedSelf` still refuses, and `veilingOddityLine` is its
  positive twin: the pin refuses the SORTED self being asked about exile
  ([CR#109.2] seeds it the battlefield), and the card's own spelling is the
  unsorted `This`, which projects no zone.
- Every prior pin still refuses: full 23/23 build with the `Proofs*` modules
  is clean.

### Deviations and remainders

- **The readback item's premise was wrong.** The umbrella says `headerCtx`
  "answers the empty discourse for every coordination" and that the
  decidable `Bindings` comparison "this vocabulary does not have" must be
  built. Both were already true in the tree when this sub-round opened.
  Nothing was built; the numbers were re-measured and the family recorded.
  The umbrella's "coordinated header's readback" section should be struck
  rather than carried forward.
- **Veiling Oddity's second blocker is DISCHARGED, not merely recorded.**
  The umbrella records it as the self's ZONE EVIDENCE (findings 346, 351).
  That is the blocker on the SORTED self (`AsType Creature This`, whose
  `nounZone` is `Just Battlefield`), and a suspended card is not a creature
  on the battlefield: written with the bare `This`, whose `nounZone` is
  `Nothing`, the line writes. The card is still not whole — its other line
  is "Suspend 4—{1}{U}", a keyword row.
- **Four keyword rows routed** to
  `planned/workbench-keyword-parameters-and-attachment.md` (its standing
  scope fence keeps keyword additions there): Saddle [CR#702.171a] —
  Autarch Mammoth, Bridled Bighorn, Seraphic Steed and the 26-line saddle
  family stay ability-only for want of it; Suspend — Veiling Oddity's other
  line; Emerge and Craft — the concurrent clause's two remaining printed act
  shapes.
- **The Temporal Anchor stays off the bench for its own reason**, recorded
  here: its trigger event is "you choose to put one or more cards on the
  bottom of your library", a step INSIDE the scry that no event row names.
  Not a while-clause gap.
- **`AltEvent` was not retired and no general event disjunction was built.**
  The umbrella's coupling asked that whichever round lands the decidable
  comparison settle it for both; the comparison was already general-purpose
  (`sameBindings` is Bindings-level, `sharedCtx` is parameterised by its
  seat's reader), and `joinedCtx` is its third consumer. A general
  disjunction that would retire `AltEvent` did not fall out and is left to
  `workbench-event-algebra`.
- The three-header join is unattested (measured 0) and is tolerated by the
  list rather than gated; no rule refuses one, so there is nothing to pin.
