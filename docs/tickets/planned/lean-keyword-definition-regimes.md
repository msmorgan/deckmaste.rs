---
needs: []
---
**A keyword definition's triggered parts need not all share the keyword's
stack regime.** Found by `semantics-v2-macro-bodies-keyword-abilities`
round three (2026-09-07): `keywordBodyPartFits` requires every triggered
body part to key on the keyword's own `regime`, but Offspring
[CR#702.174a] and Squad are cast-time keywords (`AtCasting`) whose second
ability is an enters trigger, and Impending's landed body carries the same
mismatch latent (no canon card reads it yet; the first one will refuse).
Decide the law from the CR: a definition's parts may carry different
regimes when the keyword's entry defines abilities that function in
different places; likely the regime becomes a per-part declared fact or the
law checks only the part that carries the keyword's own regime. Re-spell
the pins, mirror if the facts row shape changes, then body Offspring and
Squad and add a canon card that reads Impending. Standard constraints
apply.
