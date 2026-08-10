---
needs: [engine-cost-payment, engine-activated-abilities, engine-mana-system, core-pip-payment, core-aggregate-stat-cost, engine-crew-battlefield-zone-guard]
---
Replace the one-shot `PayCost`/`PayMana` scheduler with a general,
transactional payment-obligation window. The current path exposes mana
allocation but schedules fixed action costs immediately, opens cost-side
`With` as an ordinary resolution choice, and lets core choose `TapTotal` and
`PayPips` resources. Core must instead expose and validate every payment
choice; deterministic autopayment remains runner policy
(`docs/decisions/engine-runner-boundary.md`).

Route casting, activated abilities, and rule/effect payments such as
`MayPay`/tolls through the same protocol; remove their parallel `PayMana`
schedulers.

## Compile costs into runnable choices

Make payment the first major deliberate divergence between the semantic and
core representations:

```text
semantic Cost -> runnable core Cost -> runtime Iou graph
```

Leave Oracle-facing semantics unchanged. During lowering, reuse the existing
`Binder`/`With` vocabulary and compile cost actions so runnable action subjects
are always already bound. KCI's authored `With(ChooseOne, body)` remains a
nested choose-and-pay cost; Treasure's self-sacrifice is bound directly; and
the choice currently embedded inside discard action composites is lifted into
cost position. The verb itself must not imply whether a choice exists. It
should be structurally impossible for a runnable cost action to retain an
unresolved `Choose` or `Random` selection.

Validate a chosen subject set atomically before enactment, but preserve the
authored event granularity: discard-N then emits N individually replaceable or
preventable discard actions, including per-card madness handling. The existing
nested body remains capable of scoping one binding over several components.

Retain specialized representations where the choice is genuinely different:

- `Mana` locks to individually identified pip obligations; expand generic
  amounts to individual pips.
- `ManaCostOf` resolves to concrete pips when the total cost locks.
- `PayPips` adds alternative fulfillments to applicable pip obligations.
- `Tap` and `Untap` are bound, nullary obligations that validate and enact on
  explicit fulfillment.
- `TapTotal` accepts one complete distinct subset and validates its live stats
  against the locked comparison/count when that IOU is fulfilled.

Give every runtime obligation a stable `IouId`; one IOU has at most one active
fulfillment.

Classify mana abilities during lowering under [CR#605.1a,605.1b] and carry the
judgment explicitly in core, including both activated and triggered forms,
for example `Ability::Mana(ManaAbility::{Activated, Triggered})`. An activated
wrapper may carry a lowering-computed `Always`/`ByAnnouncedMode` profile because
[CR#700.2c] can make targetlessness depend on the announced mode; runtime only
instantiates that precomputed result and never reclassifies from game state.
Generic activated/triggered queries must look through the wrapper. Activated
and triggered forms resolve without the stack ([CR#605.3b,605.4a]). Emit the
causal activation, production, `ManaAdded`, and tap-for-mana facts ([CR#106.12])
the triggered form matches; resolve it immediately after the mana ability that
caused it and thread the produced-mana context through the trigger and reversal
trace. A triggered mana ability caused by a bare non-mana-ability `ManaAdded`
event resolves analogously immediately after that addition, before the
containing resolution advances. This ticket owns the previously separate
triggered-mana scope.
Reversibility is not part of classification: runtime replacement results
determine [CR#733.1] barriers.

Replace pool vector-index identity with a stable `FloatingManaId` per
`ManaUnit`, retaining its kind, existing riders, and sufficient production
provenance/source snapshot. Exact units, not counts or positions, satisfy pips
and carry spend restrictions. Omnath-style retention and derived power remain
ordinary live static effects, not mana riders.

## Expose the three-stage protocol

Add `PendingDecision::Payment(PaymentPrompt)` with
`PaymentStage::{PrePayment, Paying, Ready}` and commands equivalent to:

```rust
enum PaymentCommand {
    BeginPayment(ManaCoverage),
    ActivateManaAbility { source: ObjectId, ability: AbilityId },
    Fulfill { iou: IouId, witness: FulfillmentWitness },
    RescindFulfillment(IouId),
    SubmitPayment,
    DeclinePayment,
}
```

Each frame also records `PaymentPurpose::{Announcement, Optional}`. The command
surface stays uniform, but decline semantics depend on that purpose.

Keep `Decision::Act(Action::Concede)` as the universal [CR#104.3a] action; do
not duplicate concession in `PaymentCommand`. Rejected commands leave the same
prompt pending and the active state unchanged.

Keep the three CR-visible stages while allowing an internal
`PaymentProgress::Fulfilling { iou, continuation }` to suspend inside
`Paying` when enactment opens a replacement or other ordinary decision. Mark
the IOU fulfilled only when that continuation completes. Rescind is unavailable
while a fulfillment is incomplete, but transaction-level `DeclinePayment`
remains available from that nested decision just like concession.

`PaymentPrompt` exposes the payer and subject, current stage, outstanding and
fulfilled IOUs, exact floating-mana IDs, locked coverage, legal activated mana
abilities, the currently legal [CR#601.2h] payment tier, and any replay barrier
or rescindability a client needs. It exposes legal mechanics, never a suggested
payment.

`PrePayment` exists only when the locked payment includes a mana payment. For
spells and activations this is the final total cost
([CR#601.2g,602.2b]); rule/effect-requested payments use the same opportunity
under [CR#605.3a]. This includes additional/alternative mana payments and a
`{0}` payment ([CR#118.5,118.7]), not merely a printed mana cost. A pure
nonmana cost starts in `Paying`; an empty cost starts `Ready`.

Keep the existing [CR#601.2b] announcement choices before this protocol:
modes, X, optional costs, and hybrid/Phyrexian readings lock before the total
cost. Phyrexian-as-life becomes a nonmana IOU, so an all-life concrete cost has
no `PrePayment`; `{S}` is a mana IOU whose exact unit must carry snow
provenance. The rules do not explicitly resolve whether the `{0}`
acknowledgment is “a mana payment”; the project deliberately treats it as one
and opens `PrePayment`.

During `PrePayment`, allow legal activated mana abilities to open nested full
payment frames. No cost is fulfilled yet. `BeginPayment` submits a complete
`ManaCoverage` that maps every pip IOU either to an exact spendable
`FloatingManaId` or to a concrete `PayPips` alternative and selected object.
Validate colors, riders, uniqueness, and alternative-payment legality as one
batch. `{0}` uses an explicit empty witness. Reject insufficient coverage
without mutation. Success locks the witness, closes the mana-ability window,
and enters `Paying`.

Do not add mana reservations. Mana is removed only when its pip is fulfilled.
After `BeginPayment`, mana abilities are forbidden, so a nested ability cannot
spend units covered by the parent. A mana ability with its own mana cost—such
as Mana Cylix's `{1}, {T}` ability—gets its own nested `PrePayment` before the
parent crosses its boundary.

In `Paying`, enact each accepted fulfillment immediately in the frame's
isolated working image. The payer chooses the order permitted by
[CR#601.2h]: first pay all costs not involving random elements or moving a
library object to a public zone, in any order, then all remaining costs in any
order. Mana fulfillment spends the exact covered unit. Action and aggregate
witnesses are validated as whole batches before enactment; multi-object action
costs then emit their individual replaceable events. A fulfillment may suspend
on normal replacement/choice machinery and resume to completion. Core performs
no search, source activation, reassignment, ordering repair, or automatic
choice.

After every IOU is fulfilled, enter `Ready`. `SubmitPayment` commits only from
`Ready`. A lone tap cost therefore still exposes `Fulfill`, followed by
`SubmitPayment`; runners may collapse those mechanical steps.

Submitting an activated mana-ability child completes its activation and then
resolves that ability plus any caused triggered mana abilities immediately,
inside the child image, before the completed child is promoted to its parent.
An ordinary spell or activated ability follows its normal post-announcement
continuation only after its root payment submits.

`RescindFulfillment(iou)` reconstructs the frame from its payment base and
replays every other active fulfillment in original order. It makes no new
choices and does not recalculate or repair outcomes. Reject it unchanged when
the selected action crosses a reversal barrier or a retained record cannot
replay identically. Random outcomes and hidden-zone disclosures are observation
barriers that also reject selective rescind, even when they are not separate
[CR#733.1] library barriers. Rescinding every fulfillment may return a
mana-paying frame to `PrePayment`, preserving mana abilities submitted before
its `BeginPayment`, when no barrier prevents the rewind.

`DeclinePayment` is always available, including from `Ready` and during a
suspended fulfillment. For `Announcement`, a root decline unannounces the
spell or ability; a child decline cancels that mana ability and resumes the
parent, with [CR#733.1] reconstruction and legal mana reversals. The engine
intentionally does not solve whether the proposal is payable and permits a
hidden digital proposal to be declined even if it may have been payable. Emit
a policy-neutral `PaymentDeclined` incident containing the player, subject,
all forced-retained transaction records, and whether a reversal or observation
barrier was crossed; tournament-aware runners may interpret it. Do not put an
IPG, REL, or `Grv` judgment in rules-visible state.

For `Optional`, entering the payment frame does not yet choose the [CR#118.12]
pay branch. `SubmitPayment` makes that choice only after legal completion
([CR#608.2d]). Decline keeps mana abilities and other changes in the working
image, promotes it, follows `if_not`, offers no [CR#733.1] reversal, and emits no
incident.

Remove announce-time affordability searches such as the current
`affordable_concretization` rewind and cost-summary `can_pay` gate. Announcement
legality may validate its immediate structural choices, but inability to find a
complete payment is discovered only through explicit coverage/fulfillment and
`DeclinePayment`. Legal enumeration may expose unpayable proposals. The
temporary runner adapter owns advisory affordability and may reuse the existing
autotap/search helper so its current monocolor strategies do not propose
obviously unpayable actions; that helper is not a core legality gate.

## Run payments on isolated full game images

Keep the committed outer state frozen while a payment is pending. The public
engine state owns the committed image, optional payment controller, and
transaction-external metadata. Introduce a complete runnable `GameImage`
containing every field normal engine execution can observe or mutate: object
store and allocator, zones, players and pools, stack/announcement, continuous
effects/layers, replacements, triggers, agenda, pending decisions, `History`,
RNG, and identity sources. The payment controller and checkpoint stack stay
outside the image to avoid recursively cloning snapshots.

Replace the current singular announcement/decision control slots with
image-owned control stacks, including their associated continuations,
replacement state, and trigger-placement state. A nested child clone retains
the parent's suspended announcement and `PaymentPrompt` below the child's top
entries. Ordinary execution addresses only the top entry, so nested full
machinery cannot overwrite its parent.

Start the root transaction before a spell or ability proposal mutates the
game. Run modes, targets, and the rest of announcement in its working image, so
the root checkpoint is the genuine preannouncement state. For a payment
requested during resolution, checkpoint immediately before the pay-or-decline
branch. Start a nested mana-ability frame before that ability's announcement.
`DeclinePayment` can then discard an entire proposal instead of attempting to
invert already-committed announcement changes.

Implement the naïve version with full clones. A root frame starts from a clone
of the committed image. A nested mana ability clones the current parent working
image; the frozen parent's image is the child checkpoint. Only the top frame
runs, using ordinary targets, choices, replacements, triggers, LKI, zone
changes, and nested payment machinery. On child submit, replace the parent's
working image wholesale with the completed child image and incorporate its
transaction records. Complete and pop the child's control entries before
promotion, leaving the cloned parent entries intact; do not merge rules-state
fields. On child decline, discard the child; if [CR#733.1] forces records to
remain, replay those records over a fresh clone of the frozen parent and
promote that result. Root submit promotes its working image and discards replay
metadata.

All engine queries and clients read the top active working image while a
transaction is open; opponent redaction/privacy remains out of scope. The
outer controller intercepts concession at any nesting depth, applies the
terminal loss, and tears down the transaction so it cannot be lost with a
discarded child agenda.

Every frame establishes a payment base on entry to `Paying`.
`BeginPayment` creates it after all already-submitted `PrePayment` mana
abilities; a nonmana frame creates it immediately after the cost locks.
Fulfillments advance from that base. This must make mana actually spent in a
filter-land child unavailable to later derived calculations in the parent.

## Keep a frame-local replay trace

Do not turn ordinary `History` into a mutation ledger and do not add game-wide
Undo. Each payment frame owns a temporary transaction/replay trace alongside
the working image's normal history. The outer controller owns transaction
metadata for policy-neutral incidents and monotone observations, so those
survive discarded images. Successful root submission discards replay data and
merges durable incidents/observations into their out-of-image sinks.

Record concrete enacted outcomes plus the decisions and RNG results needed to
reproduce them, not just semantic ability definitions. Replay must not reroll,
reopen choices, reevaluate replacements, or silently change a computed amount.
A mismatch is an unreplayable dependency and rejects the selective edit.

Each submitted activated mana ability is one stable, player-selectable
`ManaActionId` for [CR#733.1]. Record its actor, concrete costs/effects, caused
triggered mana abilities, produced and spent floating-mana IDs, dependencies,
and disclosure/reversal barriers. Its costs, effect, and caused triggered mana
abilities reverse or remain as one unit. A nested activated mana ability is a
separate unit. Action-cost fulfillment records support deterministic replay but
are not separately selectable [CR#733.1] mana reversals.

Trace concrete ordinary caused events and noted nonmana triggers too. Replaying
a retained record runs them through normal engine machinery with recorded
choices, restoring ordinary triggers for the normal later placement barrier;
a targeting trigger chooses targets there rather than during reconstruction.
Triggered mana abilities remain immediate members of their causal mana-action
unit.

On decline, expose `ChooseManaReversals` when players have legal choices. A
producer cannot be reversed while a retained mana ability spent mana it—or a
triggered mana ability it caused—produced. The current runner chooses the
maximal legal reversal set.

Compute [CR#733.1] barriers from the concrete post-replacement trace. Actions
that moved a card to a library, moved one from a library to a non-stack zone,
shuffled a library, or revealed a library card cannot be reversed. A barrier
never removes `DeclinePayment` or softlocks an unpayable announcement: cancel
the containing announcement while retaining the barred action and all
dependency-forced records. For a submitted KCI activation under Wheel of Sun
and Moon, retain the reveal/bottom move and KCI's mana; if the KCI frame itself
is declined before submission, retain the barred paid cost but do not produce
its unsubmitted mana.

Treat random results and hidden-zone disclosures as observation barriers. They
block selective rescind but never remove `DeclinePayment`; abandonment records
the barrier, reconstructs the legal physical state, and preserves what was
observed.

Treat disclosure as monotone across reconstruction: every player retains all
card/order knowledge or future view grants they obtained, even if the physical
state is rewound. Store observations outside the discarded image under stable
transaction-local logical handles, rebind them through the replay ID map, and
union them into durable view grants. Because SlotMap cannot restore removed
objects at old keys, replay remaps freshly minted `ObjectId`s and
`FloatingManaId`s. A retained record depending on an entity created only by an
omitted record is unreplayable.

## Runner and verification

Adapt the current headless strategy/TUI shim for its monocolor decks so players
see no new interaction: activate the existing preferred sources, construct
exact coverage, fulfill in a deterministic legal order, submit, and select the
maximal reversal set. The full interactive client belongs to
`tui-full-payment-protocol`; clone optimization belongs to
`engine-lightweight-game-image-forks`.

Complete Crew as the first end-to-end `TapTotal` consumer: surface every legal
subset instead of selecting the highest-power/fewest-permanents subset in core,
move that heuristic to runner policy, and finish the Crew macro's end-of-turn
Creature-type continuous effect.

Cover at least:

- lowering Treasure as bound versus KCI's existing `With` as choose-one;
  lifting action-embedded discard into whole-set validation followed by
  per-card replaceable events; nullary tap/untap, aggregate `TapTotal`,
  `ManaCostOf`, `PayPips`, and both activated and triggered mana
  classification, including a modal `ByAnnouncedMode` fixture;
- triggered mana matching from activation/production/`ManaAdded`/
  tapped-for-mana facts, immediate causal resolution, and produced-mana
  context;
- tap-only/empty/`{0}` entry stages, exact coverage rejection without
  mutation, no mana activation after `BeginPayment`, both [CR#601.2h] tiers,
  hybrid/Phyrexian and snow concretization, suspended fulfillment,
  explicit submit/decline/rescind, optional decline that retains produced
  mana, and runner automation that avoids obviously unpayable proposals;
- source-announced-then-sacrificed-to-KCI, including a `{0}` source ability;
- multiple legal Crew subsets, stale/duplicate aggregate rejection, runner
  heuristic autopayment, and end-to-end Crew activation and duration;
- nested-frame freeze, wholesale promotion, child decline, deterministic
  rescind/replay, stacked parent control state, ID remapping, random/disclosure
  observation barriers, concession from a nested frame, and no automatic
  repair;
- selective reversal of independent mana abilities, producer/spender
  dependency, [CR#605.3c] in-flight reactivation exclusion, KCI plus Wheel
  forced retention, ordinary triggers caused by retained actions, monotone
  disclosure, and a `PaymentDeclined` incident without a tournament-policy
  classification;
- Omnath + Mana Cylix + Bighorner Rancher: start with `{G}{G}{G}`, spend one
  green to make black through Cylix, then tap Rancher. Rancher sees Omnath as
  3/3 and adds three green, leaving only `{G}{G}{G}{G}{G}{B}`; exact coverage
  for a `{5}{B}{G}` spell such as Beledros Witherbloom must fail. The payable
  twin taps Rancher first (Omnath is 4/4), then filters one of seven green into
  black and succeeds; Omnath's power drops after each exact green pip is
  fulfilled;
- Mox Amber + KCI + a colored legendary artifact creature such as Breya:
  Mox-before-KCI produces a qualifying color and then KCI may sacrifice Breya;
  KCI-before-Mox removes the only qualifying colored legend, so Mox produces
  no mana. Each submitted child must affect the next live query. Declining the
  parent after Mox then KCI exercises reversal and same-object checkpoint
  restoration; the Wheel variant forces KCI and its mana to remain.

Use synthetic engine fixtures for the verified Oracle behavior of
Omnath/Rancher/Beledros, Mox/Breya, and Wheel; authoring the six absent cards is
not part of this ticket.
