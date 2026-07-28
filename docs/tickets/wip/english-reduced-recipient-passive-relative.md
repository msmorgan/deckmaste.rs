---
needs: []
---
**Land the recipient-passive reduced relative — blocked on an Earley
chart-perturbation question.** Round dealtdmg (2026-07-25) landed the finite
recipient passive (`X was dealt damage`, `PredicateFrame::recipient_passive`
on `Deal`) but REVERTED its Edit 2: a postnominal reduced relative for the
same frame (`a creature dealt damage this way`, ~31 S3/S4 rows plus
`by`-agent collateral like Etching of Kumano). The categorical gate was
correct — the reduction fired only on `is_recipient_passive()` frames with a
retained object — but merely REGISTERING the new production
(`N::Nominal = N::Nominal N::VerbPhrase`, appended last, highest RuleId)
regressed roundtrip on two faces containing no `deal` at all (Talruum Piper
rendered `"...able to block this creature do do so."`; You Look Upon the
Tarrasque) and moved the frozen noun-opacity cell 441→443. Isolation proof:
disabling only the `self.add(...)` registration, leaving all other Edit-2
code dead, restored 31685/0 and the opacity baseline — the perturbation is a
chart prediction/tie-break interaction between the new `N::Nominal`
alternative and an unrelated `do so` pro-verb derivation, not a gate false
positive. Design questions for the follow-up: (a) does this grammar's Earley
implementation guarantee a categorically-dead alternative cannot perturb
sibling derivations, and if not is that latent fragility itself worth fixing
(the `do do so` reading presumably existed as a losing alternative already);
(b) is there a narrower attachment point than a bare
`N::Nominal = N::Nominal N::VerbPhrase` production (e.g. the postnominal slot
relative clauses use). The reverted implementation (AST variant, gate,
renderer, walker) is fully described in
`recovery-harness/out/dealtdmg-mechanic-report.md` §4 and the design in
`out/dealtdmg-plan.md` §3 — reuse, do not re-derive.
