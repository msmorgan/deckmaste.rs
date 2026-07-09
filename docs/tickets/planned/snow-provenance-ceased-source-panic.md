---
needs: []
---
Fix a latent never-crash violation in `snow_provenance` (`crates/deckmaste_engine/src/resolve.rs`):
it calls `self.objects.obj(frame.source)` unconditionally, which panics if `frame.source` names an
object that has ceased to exist (a token that left the game, or an LKI-only snapshot id).

## Why

The engine must NEVER panic on any input — an unresolvable/ceased reference must fizzle to a
benign no-op, not crash. `snow_provenance` reads `frame.source` via the panicking `obj(...)`
accessor with no guard; if the source permanent has left the game by the time the effect resolves,
this panics. Surfaced incidentally while writing an `AmongColorsOf` regression test (that test had
to work around it by keeping the source alive). It is pre-existing and unrelated to the value-
language feature that found it.

## Scope

Guard the `frame.source` lookup with the non-panicking `self.objects.get(frame.source)` (Option)
accessor — the same idiom the devotion / `Singleton` / (now) `AmongColorsOf` arms use — and fizzle
to the appropriate benign result (no snow provenance) when the source is gone. Add a regression
test that resolves a snow-provenance path with a ceased `frame.source` and asserts no panic.

## Gate

`cargo test -p deckmaste_engine` green (incl. the new test); `cargo clippy --all-targets -- -D
warnings` clean.
