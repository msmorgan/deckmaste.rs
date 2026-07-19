---
needs: []
---
**`cargo xtask idris-check` (batch mode) must fail fast when the shared Idris *dependency*
(`Core.idr` + whatever the probe module imports) does not compile — instead of re-`idris2
--check`-ing every card individually, each failing identically for the same reason.**

**The pathology.** Batch mode (`crates/xtask/src/idris_check.rs`, `run_batch` @99) checks ~100
cards per `idris2 --check` invocation (`render_module` @210 emits `module …\nimport Core\n\n` +
the card defs; `typecheck_module` @225 shells `idris2 --find-ipkg --check`). When a batch fails as
a whole it drops into a **per-card isolation fallback** (@150-173): re-check each of the 100 cards
alone to pin the bad apple, gated by the comment "only on the failure path, so the common 'all
sound' case stays one invocation per ~batch_size." That gate assumes a whole-batch failure means
*some card in the batch is individually unsound*. It never distinguishes that from *the shared
imports don't even compile*.

So when `Core.idr` itself fails to typecheck (e.g. a non-covering function under `%default total`,
Core.idr:14), **every** batch fails, and **every** isolation re-check also fails — all with the
identical `Building Core … Error: … is not covering` cascade, none of it about any card. For the
`plugins/wizards` corpus (7179 finished cards → ~72 batches) that is ~72 × 101 = ~7200 `idris2`
invocations, each rebuilding Core from source, emitting a `batch N failed as a whole; isolating
per-card` block (@153) per batch and thousands of duplicate per-card "failures" — an O(N)
grind that also **buries the actual cause** (one Core build error) under the noise. `run_single`
(@65) has the milder twin of this: a Core build failure is reported as `<card>: failed idris2
--check` (@91-95), blaming the card for a dependency error.

Root of the bug: the isolation fallback was added for the rare single-bad-card case and shipped on
the implicit "whole-batch fail ⇒ bad cards inside it" assumption — the shared-dependency case was
never handled. A broken shared dependency is O(1) to detect up front; today it is *discovered* N
times.

**Fix — a one-shot pre-flight dependency typecheck, before any batch.**

1. Add `fn preflight_deps(idris_dir: &Path) -> anyhow::Result<()>`: render a **deps-only probe
   module** — `render_module("IdrisCheckDeps", &[])` already yields exactly `module
   IdrisCheckDeps\nimport Core\n\n` (no card defs), i.e. precisely the dependency surface every
   batch imports — run it through `typecheck_module` + `cleanup_module`. On `Fail(output)`,
   `bail!` immediately with the idris2 output framed as a **dependency/build failure** (not a card
   failure). On `Pass`, return `Ok(())`.
2. Call `preflight_deps(&idris_dir)?` once at the top of `run` (@47), after `idris_root()`, before
   dispatching to `run_single`/`run_batch` — so it guards **both** modes.
3. Leave the batch loop and the per-card isolation (@142-176) **unchanged**. With deps proven good
   up front, a whole-batch failure now genuinely means a bad apple in that batch — exactly what
   isolation is for — so keep it.

Why this shape:
- **Broken deps:** one `idris2` invocation, a clear "dependencies do not compile" error with the
  Core diagnostics, non-zero exit, and **zero** `isolating per-card` blocks / zero wasted
  subsequent batches.
- **Happy path is net-neutral, arguably faster:** the probe builds `Core.ttc` once — without it,
  batch 0 built Core once anyway. But the probe **warms the cache**, so later batches *and* every
  isolation re-check reuse `Core.ttc` instead of each rebuilding Core from source (today's
  isolation rebuilds Core per card — a real, separate speedup this fix also lands).

Do **not**: (a) rip out per-card isolation — it is correct for real unsound cards; (b) instead
try to string-match idris2 output in the *failure* path to guess "was that a dep error" — the
pre-flight is deterministic and doesn't parse diagnostics.

Optional hardening (not required, pre-flight is the primary fix): as belt-and-suspenders, if a
batch fails and isolation finds **every** card in it failing with byte-identical output, treat it
as systemic and `bail!` rather than finish the sweep.

**Also — scratch-module hygiene (bundled here, same tool).** `typecheck_module` (@225) writes each
scratch module (`IdrisCheckSingle_*` / `IdrisCheckBatch_*` / `IdrisCheckIsolate_*`, and the new
`IdrisCheckDeps`) straight into the **tracked** `idris/src/` tree, and only `cleanup_module` (@252)
removes them — skipped on any panic, `?`-early-return, or kill, so an interrupted run **leaks
tracked-tree litter** (a killed batch run left `idris/src/IdrisCheckIsolate_DimirGuildmage.idr`
showing as an added file). Immediate stopgap **already applied in this ticket's commit**:
`idris/.gitignore` now ignores `/src/IdrisCheck*.idr`, so a leak can never be committed (`.ttc/.ttm`
were already covered by the existing `build/` line). Proper fix for the implementer: make cleanup
survive interruption rather than lean on the ignore — e.g. a `Drop`-guard type owning the scratch
path (removes on `?`/panic too), or a dedicated ignored scratch dir wired in as an ipkg source dir.
(`idris2 --find-ipkg` needs the module under the ipkg `sourcedir = "src"`, so scratch has to live
somewhere idris2 can see — pick whichever keeps it out of the tracked tree AND self-cleans.)

**Verify — both paths, no skipping.** (The skip-the-slow-check reasoning is exactly what shipped
this bug.)

1. **Broken-deps path.** Against a `Core.idr` that fails `idris2 --check`, `cargo xtask idris-check
   plugins/wizards` must exit non-zero after **one** `idris2` invocation, print the Core
   diagnostics, and emit **no** isolation blocks:
   `cargo xtask idris-check plugins/wizards 2>&1 | grep -c 'isolating per-card'` ⇒ `0`.
   Trunk is currently in exactly this broken state and is the ready-made fixture — see NOTE.
2. **Happy path.** Against a *compiling* `Core.idr`: batch mode still prints `N/total cards emit and
   typecheck`, one `idris2` call per clean batch, and a genuinely-unsound card is **still** isolated
   and reported by name. Requires a good Core (trunk's is red right now, see NOTE) — fix Core first
   or use a minimal good-deps fixture; **do not skip** — confirm real bad-apple isolation survives.
3. `cargo fmt` + `cargo clippy` clean; standard constraints apply.

**NOTE (separate issue, not this ticket's scope — but it is the fixture for verify #1).** The gate
is *currently* fully red because `idris/src/Core.idr` does not typecheck: under `%default total`
(Core.idr:14), `eventKindCaps` (@536) and `eventKindObjectSort` are missing clauses for the
keyword-action verb kinds `Mill | Scry | Surveil | Fateseal | Fight` (added to the `EventKind` data
decl @450 per the comment @466-477, no `_` fallback). Those two coverage gaps cascade to 9
dependents (`actionEventCaps → costCaps → costsCaps`, `eventKindHasAmount → kindsHaveAmount →
eventQueryHasAmount`, `eventQueryCaps`, `queryObjectSort`, `queryRoles`). That regression predates
this ticket and needs its own fix (the per-kind caps/sort values are semantic — get intent, don't
guess a total-but-wrong clause). If it is fixed before this ticket is worked, reproduce the
broken-deps state for verify #1 by temporarily reverting one clause.
