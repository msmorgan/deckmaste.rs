---
needs: [english-shape-rarity]
---
**Lint AST edges that violate a selectional table — which head types each verb
or gap can legally take.** Proves attachment defects that are invisible to
round-trip: a modifier attached to the wrong host renders back byte-identically,
so only a type constraint can catch it.

The model case is the pre-fix Bonfire tree, where a zero-marker object-gap
relative whose verb was `control` attached as a complement of `Mass(Damage)`.
`control` takes a permanent as its object; damage is not a permanent, so the
edge is provably wrong with no card-specific judgment. That is the shape of
every entry here.

- Build the table from `cargo xtask map enums` plus the CR — verb/gap →
  admissible head types. Author it once, check it once, reuse forever; the
  upfront cost is the point of the ticket and the steady-state cost is zero.
- Cite the CR for each entry per the repo's citation rules; rule numbers come
  from the CR, never from memory.
- A violation must be a defect *by the table*, not by plausibility. Where a
  verb genuinely admits an open-ended host set, leave it out of the table
  rather than guessing — recall is explicitly subordinate to soundness here.

Deliberately narrower than "check every edge": entries earn their place by
being provable. Prefer a small table that never lies over a broad one that
needs triage.

Standard constraints apply.
