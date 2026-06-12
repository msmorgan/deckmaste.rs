# The mtg-rules skill loop

deckmaste treats the **mtg-rules skill** as the authoritative design
baseline for rules semantics (taxonomy, keyword classification, deontics,
the underdetermined registry). The Comprehensive Rules remain the terminal
authority — repo prose and code cite rule numbers in the bracketed `CR#…`
form, never skill docs; the skill is the privileged intermediate that
synthesizes them.

## The pin

`docs/rules-taxonomy.md` §10 records the conformance baseline: skill
version, git commit, CR effective date, and the keywords-classified.json
hash, as reported by the skill's `scripts/version`. **A version bump is the
re-sync trigger**: re-run the alignment pass (conformance audit of changed
docs against core types), update the pin, and read the skill CHANGELOG's
taxonomy-meaning entries first — they call out reclassifications and
notation changes explicitly.

## Downstream (skill → deckmaste)

- Taxonomy positions are adopted, not mirrored: classification truth must
  agree; implementation strategy is the engine's (e.g. the keyword enum
  carries exactly the implemented intrinsics; composite keywords are
  `KeywordAbility`-kind macros).
- Deliberate deviations are documented at the deviation site with their
  exit condition (the `keyword.rs` header is the template).

## Upstream (deckmaste → skill)

- Implementation is the stress test. A taxonomy hole, a wrong record, or a
  stale changelog found during engine work goes upstream as an erratum —
  filed in the skill repo's notes (`docs/superpowers/plans/`), fixed
  skill-first, then re-pinned here. (Precedents: the Enlist `given` bug; the
  v1.7.0 changelog's haste blurb and count errors.)
- Engine choices on CR-underdetermined points are recorded in
  `docs/engine-adrs.md`, keyed by the registry's stable `UD-n` ids. The
  skill's registry stays engine-agnostic; the ADRs are deckmaste's half of
  that contract. Implicit decisions discovered in code get written down.

## Shared machinery

- One CR snapshot: the skill resolves data through `MTG_RULES_DATA` to this
  repo's `data/` (set in `.claude/settings.local.json`), so `cargo xtask
  cite` and the skill's scripts read the same rules text.
- One citation checker: `cargo xtask cite` is a shim over the skill's
  `scripts/cite` (config: `cite-config.json`; JSON lockfile
  `cr-citations.lock` pins normalized-text checksums). The cite CLI surface
  is a cross-repo compatibility contract.
