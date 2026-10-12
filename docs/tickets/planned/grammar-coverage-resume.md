---
needs: [grammar-prepositions-from-frames]
---
# Resume coverage under the marginal rate

**Placeholder: not claimable.** The README has no marker for this, so the `needs:` chain is what blocks it, and it must not be claimed or worked until it has been specified. It is specified only after its predecessor lands, following a short design pass the owner approves; no rulings are pre-written here.

Goal: resume growing coverage on the consolidated grammar, vocabulary-only faces first (496 of the 2,034 unread faces were vocabulary-only at the review), with work re-derived from a fresh blockers report; the old coverage tickets are deleted, so nothing is carried forward as input.

Gate decided so far: instead of the ceiling, at most 0.25 grammar lines added per face gained, enforced by the size tool; a landing over the rate is refused, not waived; the owner may reset the number. Coverage is reported as read (a correct analysis is among the readings), read only by a known-wrong analysis (every reading is known-wrong), and unread, with the census check mode refusing a landing that breaks the rate or the baseline. Judgment set plus census gate every change; three-line note.

Open, owner has not decided: how the rate is computed for refactors and for landings that add no faces; whether gains are gross or net of faces lost; whether a repair of a face read only by a known-wrong analysis counts as a gain; how the tracked line-total file hands over to the rate; and what becomes of the two retained un-integrated workspaces, `english-v3-sentence-header-mode-list` and `english-v3-multiplied-variable-quantity`. To be specified after `grammar-prepositions-from-frames` lands.
