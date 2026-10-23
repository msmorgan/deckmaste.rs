---
needs: []
---
# "Protection from each …": a protection that is one ability per quality

Standard constraints apply. Lean first (`lean/Semantics/`). Provenance: the Ward/Blessing cycle in
[lean-v3-effect-keeps-existing-attachments](../done/lean-v3-effect-keeps-existing-attachments.md),
which could not spell two of its cards for want of this form.

"Protection from each [characteristic]" and "protection from each [set of characteristics,
qualities, or players]" are shorthand for one protection ability per quality, and behave as
separate protection abilities [CR#702.16h,702.16i]. The bench has no form for it: `protection`
takes one quality (`KeywordParam.quality`), and the only "each color among …" form is the action
loop `Action.doForEachKind`, not a keyword argument. Spelling the five colors out would render
text the card does not print.

Blocked cards (Vintage-legal, `data/derived/cards.jsonl`):

- *Spectra Ward*: "Enchanted creature gets +2/+2 and has protection from each color. This effect
  doesn't remove Auras." Its exemption is `retention (abilityGrant (enchanted creature) <each
  color>) (allOf aura)`, a conjunction member beside `getsPt`; the retention shape is already
  admitted (`Proofs/Retention.lean`, `retentionAdmitsItsShapes`).
- *Pledge of Loyalty*: "Enchanted creature has protection from each color among permanents you
  control. This effect doesn't remove this Aura." The set of colors is read continuously
  [CR#611.3a].

Actionable change: give a granted protection a quality ranging over each value of an axis, with
an optional domain ("among permanents you control"), so that it is one ability per value
[CR#702.16h,702.16i]; then spell both cards. Settle whether an exemption on such a grant reaches
each of its abilities (it should: "this effect" names the grant as a whole).
