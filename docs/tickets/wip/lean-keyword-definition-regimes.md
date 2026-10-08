---
needs: []
---
**A keyword definition's triggered parts need not all share the keyword's
stack regime.** Found by `semantics-v2-macro-bodies-keyword-abilities`
round three (2026-09-07): `keywordBodyPartFits` requires every triggered
body part to key on the keyword's own `regime`, but Offspring
[CR#702.175a] and Squad [CR#702.157a] are cast-time keywords (`AtCasting`) whose second
ability is an enters trigger, and Impending's landed body carries the same
mismatch latent (no canon card reads it yet; the first one will refuse).
Decide the law from the CR: a definition's parts may carry different
regimes when the keyword's entry defines abilities that function in
different places; likely the regime becomes a per-part declared fact or the
law checks only the part that carries the keyword's own regime. Re-spell
the pins, mirror if the facts row shape changes, then body Offspring and
Squad and add a canon card that reads Impending. Standard constraints
apply.

**Gift too** (stopped `plugins-v2-gift-variants` attempt, 2026-10-05). Gift's
row is `regime := some .atCasting` (`lean/Semantics/Check/Facts.lean`) because
its first ability is an additional cost that works on the stack
[CR#702.174a]; its permanent form's second ability is "When this permanent
enters, if its gift cost was paid, [effect]" [CR#702.174b].
`bodyEventRegime` (`lean/Semantics/Check/Abilities.lean` ~L454) gives an
enters event `none`, so `keywordStackRegime "Gift" == bodyEventRegime ev`
fails and `keywordBodyFits "Gift"` refuses the trigger. Same law, same fix;
add a gift-permanent pin with the others. Gift's spell form is a separate
refusal, owned by `semantics-v2-keyword-definition-by-card-class`.

## Decided 2026-10-06

**Shape (a): the regime is a per-part declared fact, and each part is
checked against its own.** Nothing is exempt: no part skips the law. Kicker's
definition is the one-part case (its body is only `static(addedCost(…))`,
`plugins_v2/builtin/macros/keyword_abilities/kicker.ron`) [CR#702.33a]
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords). The rules give each part's regime:

- offspring and squad: the second ability is an enters trigger
  [CR#702.175a,702.157a];
- casualty: the second ability is a cast trigger that functions on the stack
  [CR#702.153a];
- gift: the second ability is an enters trigger on a permanent and a spell
  ability on an instant or sorcery [CR#702.174b] (the spell form waits on
  `semantics-v2-keyword-definition-by-card-class`).

Impending's latent mismatch is caught by the per-part check, not skipped.
