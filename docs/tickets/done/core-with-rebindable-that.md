---
needs: [engine-replacements]
design: true
---
## Both residuals landed — the design pass concluded by shipping the `With`/`That` route

The binding model shipped earlier (recap below); this feature closed the two
remaining residuals.

**Residual item 1 (DONE, earlier):** the `Selection::Each` / `Effect::ForEach`
call sites were migrated to the explicit `Each`-over-selection form.

**Residual item 2 (DONE, this feature):** the regeneration-shield `That`-capture
was re-expressed through `With`. `CreateReplacement.subject` — the bespoke
capture field `engine-replacements` had added — is **retired**; `create_shield`
now reads the protected permanent from the enclosing `With`'s `That` binding,
and `Regenerate.ron` is re-spelled through `With`. `floating_watches` needed no
change (it already keys on the frozen subject identity captured at shield
creation). The design questions this ticket raised (how a persistent shield
freezes the frame-bound `That`, how the matching key survives without a
`subject` field) were answered by that shipped shape: the `With` resolution
freezes the captured permanent into the shield at creation time, exactly as the
old `subject` field did, so the matching key is unchanged. No open design
question remains.

## Binding model (recap — LANDED earlier)

**`Effect::With { selection, body }`** resolves the whole `selection` as a
PLURAL anaphor **`Those`** (bound in the resolution frame), runs `body` once,
does NOT distribute. **`Selection::Those`** reads the bound plural group
(order-preserved). **`Each` / `ForEach`** are the explicit per-element
distributors binding the singular **`That`** (`ThatObject`/`ThatPlayer`); they
were retained. **`This` never rebinds** — always the ability's source.
Group-vs-element is explicit at the effect level: `With` → `Those` (whole
ordered group, once); `Each`/`ForEach` → `That` (one element at a time).
