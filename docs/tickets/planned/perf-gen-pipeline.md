---
needs: []
---
Speed up the card-generation pipeline (`cargo xtask generate <plugin>` =
stubs → extract → resolve → graduate). Tooling/build perf, not engine runtime.

## Baseline (measured)

Full gen of `plugins/wizards` = **29,872 cards** (24,730 left `.ron.todo` +
5,142 graduated `.ron`); both profiles produce byte-identical output. On a
16-core host:

| Stage    | dev (`cargo xtask` alias) | release | dev→rel |
|----------|---------------------------|---------|---------|
| stubs    | 0.30 s                    | 0.31 s  | ~1×     |
| extract  | 14.09 s (user 9.8/sys 3.9)| 5.89 s  | 2.4×    |
| resolve  | **67.30 s** (user 63.5)   | 12.53 s | 5.4×    |
| graduate | 6.76 s                    | 4.49 s  | 1.5×    |
| **total**| **88.5 s (~2.96 ms/card)**| 23.2 s (~0.78 ms/card) | 3.8× |

The default `cargo xtask` alias uses the dev profile, so ~88 s / ~3.0 ms-per-card
is the time actually experienced. A full release build is 3.8× faster but costs
a ~53 s one-time recompile — **the goal here is to speed up the dev path itself,
not to push people toward release builds.**

`user ≈ real` in every heavy stage → single-threaded, CPU-bound (e.g. dev
resolve: 63.5 user / 67.3 real = 94% one core), with 16 cores idle.

## Fixes

1. **Extend the dev opt-level=3 overrides to cover the regex engine.** The root
   `Cargo.toml` already raises dev `opt-level` for `serde_json`, `ron`,
   `macro_ron`, and `deckmaste_migrations` so the tools stay fast to run without
   a release build — but it misses the `regex` crate, which runs unoptimized in
   dev. That is the dominant dev-only penalty: `resolve` (regex-heavy parser
   registry) is 5.4× slower in dev, and `extract` has a ~7.6 s user-time gap that
   isn't the JSON parse. Add `regex` (and its transitive `regex-automata`,
   `regex-syntax`, `aho-corasick`, `memchr`) to `[profile.dev.package.*]`
   `opt-level = 3`. Cheap, no behavior change; should pull dev extract/resolve
   most of the way to release speeds. **This is the chosen lever for the dev
   path** (in place of recommending `--release`).

2. **Lift the per-call static regex to `LazyLock`.**
   `crates/deckmaste_migrations/src/parsers/filter.rs:152` compiles
   `Regex::new(r"(?i) with (power|toughness) (\d+) or (greater|less)$")` on every
   call — and that function runs per ability-line during `resolve`, so it
   recompiles tens of thousands of times. Every neighboring regex is already a
   module-level `LazyLock`; make this one match. Audit the pipeline for any other
   per-call `Regex::new`. (Note: `extract.rs:135` is a per-card *dynamic* pattern
   built from the card name — inherently data-dependent, not a simple lift; leave
   it unless an easy win appears. Helps both profiles.)

3. **Parallelize the three single-threaded per-card loops with `rayon`** (none of
   the pipeline uses threads today). All three iterate independent per-card work
   with shared read-only state built once before the loop:
   - `resolve` — `resolve.rs:191` (`read_to_string` → `from_str` → run parser
     registry → conditional `write`); plugin + `TemplateIndex` are built once and
     shared `&`. Pure CPU → near-linear scaling. Prime target.
   - `graduate` — `graduate.rs:122` (`read` → `macros.read_str::<Card>` →
     conditional `rename`); plugin built once. The `GraduateReport` aggregation
     must become a `collect` → reduce (per-item result, fold afterward) so the
     loop body stays side-effect-free apart from the independent `rename`.
   - `extract` — `extract.rs:303` write loop (after the one-time ~600 MB JSON
     parse). More sys/I/O-bound than CPU-bound, so a smaller win, but parallel
     writes still help. Note the per-card `eprintln!` ordering will change under
     parallelism.

4. **(Optional / minor)** Add per-stage timing output so regressions are visible,
   and replace the per-card `eprintln!` (30k lines) with a progress counter — the
   per-line stderr to a TTY is itself real wall-time in interactive runs.

## Acceptance

- Output is byte-identical to today's: a from-scratch gen still yields 29,872
  cards (24,730 `.ron.todo` + 5,142 graduated `.ron`); `cargo xtask generate`
  stays deterministic.
- Dev-profile full gen is materially faster than the ~88 s baseline — target dev
  `resolve` approaching its release time, and overall dev gen well under ~88 s
  (fixes 1+2 alone, near-zero risk; fix 3 compounds on both profiles).
- Bench method: generate from scratch into a throwaway plugin dir under
  `plugins/` (sibling `builtin/` supplies the prelude), timing each stage; never
  clobber `plugins/wizards`.
