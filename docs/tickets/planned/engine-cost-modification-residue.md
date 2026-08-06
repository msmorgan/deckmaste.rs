---
needs: []
---
**Engine: non-mana cost components on a cost-change row are not folded.**

`crates/deckmaste_engine/src/cast.rs` implements the total-cost pipeline
([CR#601.2f] — the player determines the total cost; effects may increase or
reduce it). Three arms stay loud because they only handle mana:

- a cost-**increase** pass reaching a non-mana `CostComponent`;
- a cost-**reduction** pass reaching a non-mana `CostComponent`;
- `reduce_symbol` reaching a symbol kind it can't decompose.

`engine-cost-modification` (done) scoped these as intentional, permanent loud
residue — "fold when a card needs it" — rather than guessing at semantics no
corpus card exercises. Folding a non-mana component into a cost *change* is
genuinely ambiguous in places ("spells cost 1 less to cast" does not reduce an
additional sacrifice), so each new component kind wants its own ruling and its
own test rather than a blanket arm.

This is deliberately **not** `engine-alt-costs`: that ticket owns choosing an
alternative cost at announcement (cascade / suspend / plot and per-keyword
alt-cost selection), a different lane from modifying an already-determined
total.

Close it by widening one component kind at a time as the corpus demands, or by
retiring it if the taxonomy is reshaped so non-mana components can't reach
these passes.

Effort: **S** per component kind.
