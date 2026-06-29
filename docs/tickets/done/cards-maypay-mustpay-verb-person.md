---
needs: []
---
**Renderer BUG: `MayPay`/`MustPay` print third-person verbs for the default `you`
actor.** Found in the 2026-06-29 code review.

`MayPay.actor`/`MustPay.actor` default to `Reference::You` → renders "you", but
the templates hardcode third-person agreement
(`crates/deckmaste_cards/src/render/effect.rs:71,88,90`): "If {payer} does", "if
{payer} doesn't", "unless {payer} pays". With the (common) `you` actor this is
ungrammatical: "You may pay {1}. If you **does**, …" / "… unless you **pays**
{2}." Uncaught because no corpus card uses these arms yet.

Fix: emit second-person verb forms (do / don't / pay) when the payer renders as
"you", else the third-person forms — e.g. carry a person flag through
`fragment::reference`, or special-case the second-person payer.

Severity: **medium** (render fidelity, common case). Effort: **S**.
