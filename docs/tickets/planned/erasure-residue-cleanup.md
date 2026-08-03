---
needs: [runtime-prose-link]
---
**Residue left by the provenance erasure: prose and names that still describe
peeling, plus one swallowed diagnostic.** `runtime-prose-link` erased
invocation provenance at `lower` and deleted the engine's peeling arms, but
some comments, one test name, and one doc still describe the pre-erasure world.
Each was found by review and confirmed against source at the time; none is a
behaviour defect. Filed rather than folded into the erasure so that landing
stayed reviewable.

## Confirmed stale — nothing peels any more

- `crates/deckmaste_engine/src/resolve/mod.rs:400,428` — `spell_ability_effect`
  is documented as "looks through `Ability::Expanded`". It does not; the arm is
  gone.
- `crates/deckmaste_engine/src/resolve/mod.rs:449,560` — `top_targets` is
  documented as peeling `Expanded`, and its test is named
  `top_targets_reads_wrapper_and_peels_expanded` (`:563`) while no longer
  exercising a wrapper. Rename the test to what it now pins, or give it back a
  wrapper fixture built through the authored path if the wrapper case is still
  worth pinning — decide which, don't just retitle.
- `crates/deckmaste_tui/src/ui/detail.rs:175` — `card_id`'s doc says it returns
  `None` "for a token or emblem". Since `ObjectSource` is matched exhaustively
  as `Card`/`Player`, a token returns `Some(CardId)`; the `None` the pane
  actually relies on comes from `CardProvenance::get` reading past the end of
  the companion table. `pins_card_ids_to_deck_order` pins the real behaviour,
  so only the comment is wrong.

## Audit, do not assume — some of these are still true

`crates/deckmaste_engine/src/layer.rs` around `:1730` and `:2371-2381` also
mentions `Expanded`, but at least the `is_innate` / `LoseAllAbilities` material
looks correct as written: core types implement `SupportsMacros` directly, so
`macros.read_str::<CoreType>(…)` still mints wrapped values without going
through `deckmaste_authoring` + `deckmaste_lowering`, and that test does
construct one. Read each against the code before touching it. A comment that
became true again for a different reason is not stale.

## The one non-doc item

`crates/deckmaste_plugin/src/plugin.rs:617-630` — `index_tokens` drops
per-file read and parse failures on the floor (`let Ok(…) else { continue }`).
The consequence is silent: a malformed token file yields no provenance entry,
the pane renders that token's abilities as `[unrendered: …]`, and nothing
anywhere says why. Loading is deliberately lenient here (a token file that
fails to expand should not fail the whole plugin load), so the fix is a
diagnostic, not a hard error — decide where it should surface.

## Non-goals

Not a general comment sweep — `comment-discipline-sweep` owns that. This is
only the residue one landing left behind.

## Gates

Standard constraints apply. No behaviour change: the diff should be comments,
one test name, and one diagnostic. `cargo test --workspace` green.
