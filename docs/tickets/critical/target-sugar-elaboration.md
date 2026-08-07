---
needs: [macro-author-surface]
---
**Inline target sugar and scope elaboration: the explicit indexed binder
stays the semantic normal form; sugar is accepted input.** Design:
`docs/decisions/semantics-spelling-lowering.md` (§7, §9). Normalization
(desugar + elaboration, semantics → semantic normal form) lives in
`deckmaste_semantics`; `deckmaste_lowering` invokes it. The elaborator is
the ONLY scope introducer; idiom bodies stay scope-free.

## Scope

- `Target(spec)` sugar at exactly-one Reference positions; `Targets(spec)`
  at Selection positions; no type-directed overloading. Recognized by the
  semantics reader/elaborator BEFORE ordinary macro expansion (variant-
  first dispatch would otherwise consume the ident as an index read).
- **Discriminator** (spec §7): at a Reference/Selection position,
  `Target(<numeral>)`/`Targets(<numeral>)` is an index read; any other
  argument shape is the sugar. Sugar is legal only in card-authored
  provenance text — a macro body may FORWARD sugar through a `Param` hole
  but may never introduce it.
- **Occurrence rule** (spec §7 — no primary/secondary distinction): every
  semantic `Target(n)`/`Targets(n)` site counts, including characteristic
  reads; a sugared slot is unreferenceable by construction, so the
  one-site rule is enforced syntactically. Counting is defined over the
  PRE-EXPANSION invocation AST: one caller site is one occurrence no
  matter how many times an idiom body re-reads the `Param` (`Fight` is
  the normative fixture). Implementation commitment: the current reader
  expands during deserialization and re-reads raw argument text per
  `Param`, so this needs a pre-expansion elaboration layer or
  origin-tagged sugar nodes propagated through expansion — price it,
  don't hand-wave it. Traversal order for appended slots is schema order
  (field declaration order, list sequence), so RON named-field reordering
  cannot renumber. Acceptance-time validation and canonical-writer
  contraction are separately labeled directions — this ticket implements
  acceptance only.
- **Naming, owner-settled (2026-08-02)**: the sugar ident is `Announce`
  (`Announce(spec)`, `Announces(spec)`-or-kin for the plural Selection
  form — exact plural spelling is plan latitude). Direction-settled
  family members to shape in the plan: a READ companion
  (`Announced(n)`-style) for previously-announced slots — resolve the
  rename-depth question (canon spells `Target(n)` at ~20 sites: rename
  both grammars with the small canon sweep, or reject the companion and
  keep `Target(n)`; dual spellings are forbidden by decision principle
  2) — and distinctness: `Announce(Distinct([i…], spec))` composes for
  explicit-prefix edges, plus an "other"-flavored form
  (distinct-from-all-earlier announcements, oracle's "any OTHER
  target") for the fully-inline case; explicit indexed `Distinct` stays
  the general mechanism at the semantic surface — and the ULTIMATE core
  encoding (owner-settled 2026-08-02) is predicate-embedded slot
  references: slot i's criteria may reference strictly-earlier slots,
  e.g. `[Target(Creature), Target(And([Not(Ref(Target(0))), Creature]))]`
  — so `TargetSpec::Distinct` becomes DERIVED (the other-form lowers to
  the predicate shape; core's dedicated constructor is deleted as a
  small refactor or as the divergence ledger's first earned divergence).
  Distinguish carefully: this is a STATIC slot-reference constraint —
  order-free, and recheck-correct FOR FREE because [CR#601.2c] and the
  retarget rules recheck targets against the criteria themselves. What
  remains wrong is a dynamic not-already-targeted choose-time filter
  (order-dependent over the in-progress set). Generalized obligations:
  well-formedness becomes "no `Target(j >= i)` anywhere in slot i's
  predicate tree" (deep scan, Rust + Idris — `idris-distinct-position-
  proof` generalizes to this); plural slots need a group-membership
  predicate (`Not(Among(Targets(0)))`-shaped) where set-disjointness was
  uniform. WITHIN-slot uniqueness needs no encoding at all:
  one announce slot = one instance of "target", and the same object
  can't be chosen twice for one instance [CR#601.2c] — that default is
  engine-owned. Cross-slot same-object IS legal by default (once per
  instance), which is why the other-form exists. Fixtures: Arc Trail
  (other-form); Seeds of Strength (cross-slot same-object legal — its
  three announcements may all choose one creature); a
  choose-the-same-mode-more-than-once card (repeated modes mint
  instances, same object legal per instance); a divided-damage card
  (one plural slot, recipients forced-distinct).
- Mixed scopes via the explicit-prefix rule (explicit slots own `0..E`;
  inline sites append in textual order; semantic indices `< E`; validator
  rejects a semantic index landing on a generated slot).
- Normalize to the engine's single top-level `Targeted`; hoisting stops at
  genuine announcement boundaries (modal modes, delayed triggers)
  [CR#601.2c].
- `Distinct` canonical well-formedness: sibling indices strictly earlier,
  sorted, deduplicated, edge stored on the later slot.
- Delete the `TargetedDealDamage` differential-baseline fixture (parked by
  `frames-catalog-merge`) once the compositional path covers its cases.

## Mechanism (surveyed 2026-08-07, not yet implemented)

The scope note's "pre-expansion elaboration layer OR origin-tagged sugar
nodes" is a false pair — neither alone works, because the two constraints
pull opposite ways. Dedup wants PRE-expansion (`Fight`'s body spells
`Param(0)` four times and each hole re-reads the same raw argument text,
so a sugar VALUE node mints four slots for one caller site). Scoping wants
POST-expansion (`Modal` is not natively spellable — canon reaches it
through the `Modal` identity macro — so a textual pass cannot see mode
boundaries at all, and would wrongly hoist a mode's inline site to the
ability root).

Resolution — a **sentinel pre-pass**, splitting the two:

1. **Text rewrite before the read.** Walk the card source at value
   positions (`macro_ron`'s `decompose`, generalized to surface the head
   ident as `leading_invoked_name` already does — the walk itself is the
   one `collect_holes` uses). Replace `Announce(X)` with `Target(S_k)`
   and `Announces(X)` with `Targets(S_k)` for a reserved sentinel index
   `S_k`, stashing `X`'s source text under `k`.
2. **The ordinary restricted read, unchanged.** A sentinel is just a
   `usize` on variants that already exist, so it rides expansion with NO
   new machinery: every `Param` re-read reproduces the same sentinel, and
   the `Fight` multiplication dissolves. No placeholder variant for the
   Idris mirror or the lowering ledger to carry.
3. **`normalize(Card)` on the typed AST**, where every scope boundary is
   visible. Per announcement-scope root, collect distinct sentinels
   reachable without crossing a nested root, in schema order; assign
   `E + position`; append the specs; rewrite sentinels to real indices —
   INCLUDING inside remembered `Expansion::args` text, which is what makes
   the `lower(sugar) == lower(explicit)` byte-for-byte gate hold.

Priced deltas: byte offsets shift under the rewrite, so pass-2 diagnostics
need an offset map back to the author's text; pass 1 is type-free, so only
`Announce` WITH an argument may be rewritten — `LockPoint::Announce` is an
existing bare unit variant (`temporal.rs`), and a test must pin that.

Recommended splits (both left open):

- The ULTIMATE predicate-embedded-slot-reference encoding (deleting
  `TargetSpec::Distinct` from core, the generalized deep-scan proof in
  Rust and Idris, plural group-membership) is its own ticket. The
  "other"-form lands here regardless: it lowers to today's
  `Distinct([every earlier index], spec)`, computable once numbering is
  known.
- Merging SIBLING `Targeted`s in one scope (renumbering two explicit
  lists) has no fixture demanding it; reject that shape with a clear
  error rather than build an unexercised renumberer.

Two open owner calls: whether the `Announced(n)` READ companion is
rejected (survey leans yes — keep `Target(n)`, since `TargetSpec` /
`Targeted` / `targets:` are target-flavored throughout and renaming only
the read is the incoherent middle); and whether `normalize` is invoked at
the loader (recommended, so no sentinel escapes `Plugin::card_from_str`)
with lowering merely ASSERTING the normal form, rather than re-traversing
every card as spec §9 reads literally.

## Gates

Standard constraints apply. For every sugared fixture,
`lower(sugar) == lower(explicit)` byte-for-byte on core; the spec §7 card
list (Bolt, Terminate, Seeds of Strength, Arc Trail, Do or Die, Rabid
Bite — including Rabid Bite's worked verdict: slot 0 explicit, slot 1 may
inline) lands as fixtures; negative fixtures for each eligibility
violation, for a body attempting to introduce sugar, and for a semantic
index colliding with a generated slot.
