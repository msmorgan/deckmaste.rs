---
needs: [core-regions-piles]
---
**Engine: pile registers have no resolution writer or choice path.**

Core now represents each temporary pile as a `Kind::Pile` register
([CR#700.3a] — each affected object goes into exactly one pile; [CR#700.3b] —
a pile is not an object, each card in it stays individual). `SeparatePiles`
declares its `dests`, `ChoosePile.from` reads those registers and writes the
chosen pile to its own `dest`, and a pile-valued `Selection::Reg` iterates the
members. The activation value exists and reads as an object group, but no
engine instruction writes one yet.

The whole family is unbuilt, not just the read:

- `resolve/effect.rs` has no arm for `OneShotEffect::SeparatePiles` or
  `ChoosePile` — both fall into the catch-all choice seam.
- The semantic `PileSource::Noted` / `Selection::PilesOf` surface has no
  register spelling and lowering refuses it; the old label-bearing core
  variants and pile store are gone.
- `Action::Shuffle` over a pile-valued register degrades to a silent no-op in
  `resolve/player_action.rs`.

**This is the engine-resolution half that `core-do-or-die-divide-and-choose`
asked for.** That ticket (planned) scopes the two-pile,
opponent-chooses-and-consequence-applies shape onto a new `DivideAndChoose`
core primitive, and defers here twice over: it carves the general N-pile case
back out for "genuinely N-way cards (Whims of the Fates)", and its blocker 2
recommends "splitting into a `core:` emit/render/RON slice (cleanly doable) vs
a separate engine-resolution ticket". This is that separate ticket. Its
blocker 2 also records the one reuse dead end worth knowing up front:
`ChoiceContinuation::ArrangePiles` is scry-style ordering *within* a known
pile, not partition-and-choose, so it is not a starting point.

Scope: surface a partition decision for `group` into `dests.len()` piles and
write every destination register; surface a pile-pick decision over `from`,
write the chosen pile to `dest`, and run `then`; wire shuffle over a pile
register. The cross-region, per-player noted-pile shape needed by Whims of the
Fates also needs a label-free authored/core design before its engine storage
can be built; do not restore the deleted label-bearing core variants.

No canon card exercises this today — Whims of the Fates is the design target
once authored, so check the corpus at pickup before sizing.

Effort: **M**.
