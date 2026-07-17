---
needs: []
---
**Every ability frame hand-rolls the same "wrap the body in `Targeted` iff it
declares targets" snippet, and the copies have already diverged.** The divergence
is how the target-read inconsistency [[parse-positional-target-reads]] fixes crept
in: one copy rewrites its target reads, the others don't.

The snippet — `if parsed.targets.is_empty() { <body> } else { Targeted(targets:
[<join>], effect: <body>) }` — is copy-pasted across at least five frame
renderers:

- `spell_ability.rs:23`
- `activated_ability.rs:152`
- `triggered_ability.rs:101`
- `modal.rs:124`
- `loyalty_ability.rs:96`

plus non-identical variants of the same idea at `replacement.rs:102,149` and
`effect.rs:543,870` (branch/then sub-effects).

They are **not** identical: `triggered_ability.rs` runs the body through
`target_slot_reads` (the `\bIt\b → Target(0)` `IT_READ` regex) before wrapping,
while spell/activated/modal/loyalty wrap the raw body with bare `It`. That single
divergence is the entire mechanism behind the five-spellings mess in
[[parse-positional-target-reads]]: the same conceptual step, implemented five
times, drifted in one place and nobody else followed.

## Fix

One target-wrapping seam the frames call — e.g. `ParsedEffect::render_wrapped(pre:
&str, post: &str)` or a free `wrap_targeted(body, &targets) -> String` — so
"targets present → `Targeted(...)`" lives in exactly one place and every frame
renders identically. The frame-specific text (`Spell(effect: …)`,
`Activated(cost: … effect: …)`, `Mode(effect: …)`, `LoyaltyMinus(n: …, effect:
…)`, `Triggered(event: … effect: …)`) becomes the pre/post the frame supplies.

Best landed **before or with** [[parse-positional-target-reads]]: with one wrapper
seam, that ticket's step 3 (emit `Target(n)` at the source, delete `IT_READ`) is a
one-place change instead of five. It is independent enough to do first as pure
dedup — behaviour-preserving, including keeping the `triggered` regex until the
positional-target work retires it.

Related micro-duplication to fold in while here: `modal.rs:100-109` `word_to_count`
re-implements a subset of `effect.rs:2263-2287` `number_word`.

Verify: `cargo xtask generate plugins/wizards` emits a byte-identical corpus
(pure refactor, no output change); `cargo test --workspace` green; the per-frame
render tests (`spell_ability.rs:68+`, `activated_ability.rs:195+`,
`triggered_ability.rs:623+`, `modal.rs:298`, `loyalty_ability.rs:133`) still pass
unchanged.
