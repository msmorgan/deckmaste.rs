---
needs: []
---
**Document `ParseSelection::tied_alternatives` precisely.** The slice contains
every minimum-cost alternative at the selected root, including the chosen
alternative itself. It is therefore nonempty for every successful forest
selection; only `len() > 1` means multiple alternatives tied on cost. The
forest chooses among those deterministically by grammar-rule order and then
alternative index.

Standard constraints apply.

## Completion

- Documented and tested the actual contract; the adversarial instrument's `!is_empty()` tie census was identified as invalid and routed back to its landing ticket.
