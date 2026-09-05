---
needs: [english-v2-role-preemption-depth]
---
# Coordination of a Verb Frame's complement cluster

**B7a — minted 2026-09-04 from the `english-v2-attachment-class-declared` (R1)
landing review, H1, by coordinator ruling.** Authority: rewrite ADR "Amendment:
one scope device, principles before packing (2026-09-04)", section D of
`docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`.

Defect. A Verb Frame with two or more Complements has no Coordination for the
Complement cluster, so a sentence that coordinates two full clusters under one
verb has only an incoherent derivation: the Coordinator joins the innermost
Nominal of the first cluster with the first Nominal of the second, and the
second cluster's marked Complement becomes a Postmodifier of that pair. Forge
Devil (`07b0724f…`) is the witness — the tree does not say the second quantity
is dealt to the second recipient — and the shape recurs across the movement
frames. 62 of R1's 219 gains are that bracketing.

This is a Construction, not a scope-device case. The two readings differ in
which strings are Conjuncts, not in the attachment height of one right-peripheral
Constituent, so they are not scope variants and the packing device cannot mint
the missing one. Principle 2 of the scope device (frame-role preemption, reaching
the whole complement the frame governs) eliminates the incoherent reading; this
ticket supplies the coherent one, and the two must land together or those 62
identities lose coverage.

Pin. One general Construction: a Coordination whose Conjuncts are the Complement
cluster of one Verb Frame — the frame's non-head material realized once per
Conjunct, the head realized once before the first. Declared from English
(argument-cluster Coordination), not from the attested frames: the Construction
is one rule over the frame's declared roles, never one arm per frame and never a
list of frames. The three Coordinators get their three arms as every other
family does. No Construction, requirement or checker names a preposition, verb,
noun or card.

Acceptance. The witness sentence selects the cluster Coordination, uniquely or
as the single survivor; the incoherent bracketing has no derivation; R1's 62
identities keep coverage with a correct selected analysis, each listed. Report
the census for every frame the Construction reaches.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Identities routed here by the `english-v2-role-preemption-depth` landing (2026-09-04)

These keep coverage on the parent's bracketing after B6 — the right-periphery
rule does not reach an incoherent Coordination whose final Conjunct carries no
role preposition — so each still needs the coherent cluster Coordination this
ticket mints, and each must be listed in this ticket's acceptance:

- Trygon Prime (`0ac2cee1...`) — `... put a +1/+1 counter on it and a +1/+1
  counter on up to one other target attacking creature.`
- Sumala Sentry (`31b2e614...`) — `... put a +1/+1 counter on it and a +1/+1
  counter on this creature.`
- Evolutionary Escalation; X-23, Deadly Weapon; Ajani, the Greathearted; Stand
  Together; Serrated Biskelion; Filigree Vector; Juniper Order Ranger; Brokers
  Ascendancy; River Heralds' Boon — all
  `put <quantity> on <recipient> and <quantity> on <recipient>`, all selecting
  `put [<quantity> on <recipient> and <quantity>] on [<recipient>]`, whose
  Coordination joins a permanent with a counter.
