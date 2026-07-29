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

## Completion

`uncontrollable-host` in `xtask/src/english/lint.rs` — **431 findings**, the
substantive result of the whole claim.

An object-gap relative attaches its gap to the host noun. When the verb filling
that gap requires an object in the rules sense [CR#109.1] and the host is a mass
noun, the clause cannot belong to that host. Damage is not among the things
[CR#109.1] lists — it is what objects *deal* [CR#120.1] — so `… damage … that player
controls` is misattached whatever the card means. Both rules were already
registered in `cr-citations.lock`; no `bless` was needed.

Verified by hand:

- **Acidic Soil** — `<<damage> … <of <lands>>> <<they> <control>>`: `they
  control` modifies *damage*, not `lands`.
- **Abzan Monument** — `<<toughness> <among <creatures>>> <<you> <control>>`:
  belongs to `creatures`.

This is the general form of the defect `english-ast-grouping` fixed for
`damage … to X and each creature that …`; the general case survived, and every
one of the 431 is invisible to round-trip because the tokens render back
unchanged.

The table is deliberately two verbs (`Control`, `Own`). Widening it is real
follow-up work, but each new entry needs its own CR basis — [CR#109.1] does not
license a claim about `sacrifice` or `exile`. A small table that never lies
beats a broad one that needs triage.
