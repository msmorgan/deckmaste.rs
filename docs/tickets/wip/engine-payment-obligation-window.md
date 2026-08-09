---
needs: [engine-cost-payment, engine-activated-abilities, core-pip-payment, core-aggregate-stat-cost, engine-crew-battlefield-zone-guard]
---
**Engine: replace the one-shot `PayCost` scheduler with a general payment-
obligation window.**

The announce pipelines exist, but payment is not yet one coherent engine
state. `pay_cost` currently mixes several incompatible policies: `PayMana`
surfaces a player decision, fixed `{T}`/`{Q}` and verb work is scheduled
immediately, cost-side `With` opens its ordinary resolution choice later, and
both `TapTotal` (Crew) and `PayPips` (convoke/delve/improvise) select resources
greedily inside core. The last two are rules bugs: the payer chooses which
qualifying objects pay those costs ([CR#601.2h,602.2b,702.51a,702.66a,
702.122a]). Core must enumerate and validate those decisions; deterministic
autopayment is runner policy (`docs/decisions/engine-runner-boundary.md`).

Introduce a persistent payment session for an announced spell or ability. Once
the total cost is locked, lower every currently-supported way of paying it into
outstanding obligations with a shared payer, subject, and payment identity:

- mana from the pool;
- source tap/untap and cost-eligible `Do` actions;
- choose-then-pay `With` components;
- aggregate-subset `TapTotal` components; and
- per-pip `PayPips` substitutions.

The session, not `pay_cost`, owns progress through those obligations. At
`LockPoint::Payment` it exposes the payer's legal next payment actions and the
data a UI needs to render them. The payer may discharge nonrandom obligations
in any legal order ([CR#601.2h]); choices are validated against the session's
locked candidates/values and no strategic subset or resource is selected by
core. Fixed work may be represented as a forced action, but the cast/activation
does not commit until every obligation is satisfied. Rejected submissions leave
the session open and unchanged. Preserve the existing shared payment id on all
resulting events, and keep the payability gate a feasibility check rather than a
payment selector.

Mana has one additional rule: while mana remains payable, the payer may
activate mana abilities during this session ([CR#601.2g,602.2b,605.3a]). Those
activations use their normal choices/effects and return control to the same
payment session; ordinary priority actions are not available. Existing pool
allocation remains an explicit payment action. This replaces the current need
for a runner to pre-float mana before announcing a spell or ability.

Payment is all-or-nothing ([CR#601.2h,118.3]). The implementation must not
publish a partially paid cast/activation. If incremental mana-ability or cost
actions can invalidate completion, retain enough session state to reject the
step before mutation or reverse the incomplete payment under [CR#733.1]; do not
silently strand a half-paid announcement. This absorbs the corresponding
payment-batch/rewind conformance seam rather than leaving the new window
nontransactional.

As the first end-to-end consumer, fix `TapTotal` by surfacing its candidate
objects with locked stat contributions and accepting any distinct subset whose
sum satisfies its comparison. Move the old highest-stat/fewest-taps behavior to
the headless strategy as an optional shortcut. Finish
`plugins/builtin/macros/keyword/Crew.ron` using that obligation and an
end-of-turn continuous Creature type addition.

Cover mixed-cost sessions, payer-chosen obligation order, nested mana-ability
activation, insufficient/duplicate/stale aggregate submissions, multiple legal
Crew subsets, `PayPips` resource choice, runner autopayment, rollback or
prevention of incomplete payment, and end-to-end Crew activation. The payment
session must be extensible: a future `CostComponent` adds an obligation adapter,
not another parallel scheduling path in `pay_cost`.
