---
needs: []
---
Collapse the duplicate count/comparator evaluators into one. `eval_const_count`
(`condition.rs:93`) is a frame-free subset of `eval_count` (`resolve.rs:748`) that
`todo!`s most `CountOf`, so `Compare(CountOf(...))` PANICS on the condition side
while evaluating fine at resolution. `condition_holds`'s `Compare` arm already holds
a `&Frame`, so route it through `eval_count` and delete `eval_const_count`, folding
in the announce-slot Stack-census adjustment (`condition.rs:102`). Separately,
`legal.rs:547` `arrangement_forbidden_by` re-implements the 5-arm `Cmp` table over
`CountBound` plus a `lit()` that `todo!`s non-literal bounds (a second latent panic);
give `CountBound` a shared `eval()` in core over the unified evaluator. Release-blocker
(panics on valid grammar). Distinct from engine-resolve-count-x (the X feature) and
engine-history-filtered-counts.
