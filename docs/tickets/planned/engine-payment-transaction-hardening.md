---
needs: [engine-payment-obligation-window]
design: true
---
Harden the first full-clone payment transaction implementation after the next
engine-shape pass. The initial protocol deliberately keeps the naïve replay
model, but its post-implementation review found several reachable boundaries
that should be fixed together because they share payment-frame reconstruction,
classification, and runner-advisory machinery.

Keep `DeclinePayment` as an unconditional exit. Reconstruction must compute the
dependency closure of forced-retained records, or use a conservative whole-frame
fallback, so decline cannot return `DecisionError` or leave an unpayable
announcement open. Pin the concrete remint case: fulfillment A moves a card
from the battlefield to the graveyard, fulfillment B later moves that reminted
card from the graveyard to a library and is therefore retained under
[CR#733.1]; declining while omitting A must still reconstruct a legal state and
emit the policy-neutral incident.

Complete the lowering-time [CR#605.1a] classifier. An activated ability is not
a mana ability when either its cost or effect moves a card to or from a library.
Inspect nested `Move`, `MoveGroup`, draw, binder, selection, and cast shapes
rather than treating them as neutral. Add positive and negative fixtures for
library movement in both the cost and effect while preserving modal
`ByAnnouncedMode` behavior.

Make runner affordability advice use the same spend subject and rider check as
exact `ManaCoverage` validation, or fail conservatively when it cannot prove a
match. Add a restricted-mana fixture whose rider matches the source permanent
but not the announced ability, plus the inverse, and prove the automated runner
cannot loop between activating a source and declining the same payment.

Replace the `begin_optional_payment` concretization assertion with a checked
path. Hybrid, Phyrexian, `ManaCostOf`, and any future unresolved effect-payment
shape must either expose their required choice before the frame starts or fail
plugin/runtime validation without panicking. Preserve the rule that
affordability is not an announcement-time legality gate.

Replace materialization of every legal mana-reversal subset. The current
`2^n` enumeration repeats full reconstruction for many candidates and its
fixed-width count becomes invalid at 64 actions. Design an incremental or
constraint-based selection protocol that preserves dependency ordering,
[CR#733.1] barriers, independent `ManaActionId` selection, and the runner's
maximal-reversal policy without enumerating all subsets.

Finally, expose fulfillment rescindability only while payment progress is idle;
a suspended fulfillment must not advertise a command the engine will reject.

Acceptance requires focused regressions for every case above, no panic or
softlock on malformed/unpayable input, and the existing Omnath/Cylix/Rancher,
Mox/KCI, Wheel, optional-payment, and nested-replay matrices remaining green.
Coordinate rather than duplicate the broader identity and binder work tracked
by `engine-payment-replay-identity-generalization` and
`engine-payment-general-cost-binder-effects`.
