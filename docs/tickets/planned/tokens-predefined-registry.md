---
needs: []
design: true
---
**Data-fy `PredefinedToken`: one canonical copy in the token registry, enum
dies.** 2026-07-18 deep-dive: `plugins/builtin/tokens/{Treasure,Food,Gold,
Clue,Blood,Vibranium}.ron` already exist as the canonical definitions, and
`PredefinedToken::token()` (`deckmaste_core/src/token.rs`, ~140 lines of
hardcoded `Token` construction) is a second, hand-synchronized copy — its own
doc comment says each match arm must match its `.ron` file. This is the
conferrals precedent violated: conferral-shaped data belongs in RON, never
hardcoded.

Evidence the enum should die: [CR#111.10] is explicitly a lookup table ("use
the definition below"); idris has no predefined-token constructor (blesses
inline `TokenSpec` only — soundness-safe); the 15+ CR predefined tokens not
yet implemented (Walker, Shard, Powerstone, Incubator, the eight Roles, Map,
Junk, Lander, Mutagen) each become "add a `.ron` file" instead of "add a Rust
match arm AND a `.ron` file."

## The design question (why this is [design])

`TokenName::resolve()` is synchronous and dependency-free *because* it
hardcodes — `deckmaste_core` has no handle to the plugin registry. Options:
thread a token-registry handle to the resolve sites (engine + idris_emit), or
embed `builtin/tokens/*.ron` into core at build time. Pick one with the user.
`TokenSpec::Named(TokenName)` stays as the typed hook either way.

## Fold in: the never-crash gap

`resolve/player_action.rs` resolves `Named` with
`.expect("a Named token in a card resolves to a builtin definition")` — a
hand-authored `Named(Bogus)` reaching resolution panics today (the parser
gate only protects the migration path). Land a fizzle/gap path with the
registry change; `idris_emit` already models the graceful form. Sibling
pattern: `cards-counter-ref-validation`.

Relation to `tokens-design`/`tokens-templates` (parked): this ticket is
narrower — core plumbing only, no token *extraction* work resumes here.
