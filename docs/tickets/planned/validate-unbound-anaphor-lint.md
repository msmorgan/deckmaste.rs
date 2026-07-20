---
needs: []
---
**Nothing on the Rust side load-rejects an unbound anaphor.** A bare `It` /
`They` / `That(Sort)` with no loop, binder, or product antecedent reads fine,
loads fine, and fizzles at resolution: `eval_reference` degrades to the null id
(`resolve/query.rs` `unbound_ref`) and `eval_selection_set` to the empty group
(`unbound_group`). Both are deliberate — the engine never crashes on an
authoring mistake ([Invalid authoring fizzles](../../decisions/invalid-authoring-fizzles.md)) — but a
silent no-op is a poor way to learn you wrote a broken card.

Carved out of [[parse-positional-target-reads]], which established that a target
is read only as `Target(n)` / `Targets(n)` and left a bare `It` in a targeted
body genuinely unbound. That ticket asked for the reject as a load error; the
Idris side now delivers it (`tBadItReadsNoTarget` / `tBadThatReadsNoTarget` /
`tBadTheyReadsNoTarget` in `idris/src/Spec.idr` refuse the shape, and
`cargo xtask idris-check` runs it over canon). What is missing is the Rust-side
equivalent, which matters because the gate only covers what the emitter can
emit: `idris-check plugins/canon` reports 65/75, so 10 canon cards are outside
it entirely, and the ~7k `plugins/wizards` corpus is never gated at all.

## Why it isn't a small lint

`validate.rs` has the right seam (`Validation::lint_failures` — "shapes that
read fine but are always authoring mistakes", already home to the
cost-eligibility and subtype-provenance lints), but the check itself is not
local: deciding whether an `It` is bound requires the antecedent-scope walk —
which binders enclose it (`Each`/`Distribute`/`With`/`Where`/`Pick`), which
producing clauses precede it (`Move`/`Create`/`Search` and their sorts/zones),
and which event roles the frame carries. That is `Core.idr`'s `intro` /
`resolveIt` / `resolveStack` reimplemented in Rust, and a second implementation
of the resolution rule is a thing to drift, not a thing to have twice.

## Options (pick before building)

1. **Reuse the model.** Extend `idris-check` coverage rather than duplicating
   its analysis — close the 10 emitter gaps and run the gate over `wizards`.
   Slow (idris2 per batch) but single-source-of-truth.
2. **A conservative Rust lint.** Flag only the shapes provable without a full
   walk — e.g. an `It`/`They` in a `Targeted` body that contains no binder and
   no producing clause anywhere. Sound (no false positives), incomplete, cheap;
   catches the archetype (Lightning Bolt's old `DealDamage(This, 3, It)`).
3. **Share one scope walk.** Extract the antecedent-scope analysis into a real
   Rust pass that BOTH the linter and the engine's resolution consult, so there
   is one implementation to keep honest.

Verify: a fixture card with a bare `It` in a targeted body and no antecedent is
REJECTED at load (not a runtime fizzle); the never-crash degrade stays for cards
already in the wild (`unbound_ref`/`unbound_group` unchanged);
`cargo test --workspace` green.
