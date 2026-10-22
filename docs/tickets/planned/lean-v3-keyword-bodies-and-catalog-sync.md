---
needs: [lean-v3-rules-and-card-fidelity]
---
# Complete bodiless keyword definitions, retire unread registry columns, and cross-pin catalog facts

**Supply expansion bodies for the remaining bodiless keyword definitions in
`lean/Semantics/Definitions/Keywords.lean`, retire unread registry columns in
`lean/Semantics/Rules.lean`, and cross-pin the Lean catalog declarations against
`lean/Semantics/Check/Facts.lean` ([`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
§§3–4; [`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
§§5.2–5.3, 6.9).** Standard constraints apply.

## Scope

### 1. Expand Bodiless Keyword & Action Definitions (Astra §3, Opus §5.2)

- Audit the ~40 bodiless keyword definitions in
  `lean/Semantics/Definitions/Keywords.lean` and supply rules-faithful expansion
  bodies for all keywords whose CR meaning is expressible in the `semantics-v3`
  AST (including `echo` `[CR#702.30a]`, `typecycling` `[CR#702.29e]`,
  `equipQuality` `[CR#702.6c]`, `frenzy` `[CR#702.68a]`, `suspend` `[CR#702.62a]`,
  `multikicker` `[CR#702.33c]`, `miracle` `[CR#702.94a]`, `prototype`
  `[CR#702.160a]` — removing `set_option linter.unusedVariables false` at
  `Keywords.lean:1128` — plus `learn` `[CR#701.48]` and `ventureIntoTheDungeon`
  `[CR#701.49]` in `Definitions/Actions.lean`). `multikicker`'s body and its kicker label (a
  multikicker cost is a kicker cost `[CR#702.33c]`) are written by
  [lean-v3-keyword-cast-permissions-and-costs](lean-v3-keyword-cast-permissions-and-costs.md)
  §3; do not give it a separate label here.
- Note: `lifelink`, `wither`, `infect`, and `toxic` are owned by
  [`semantics-v3-damage-results`](semantics-v3-damage-results.md) (with `toxic`
  gated on Q89); do not duplicate or bypass that ticket's STOP.
- Document in `lean/CONTRACTS.md` the exact set of combat/engine-intrinsic
  keywords that remain intentionally bodiless (`firstStrike`, `doubleStrike`,
  `trample`, `vigilance`, `deathtouch`, etc.).

### 2. Retire Unread Registry Columns & Cross-Pin Catalog Authorities (Astra §4, Opus §§5.3, 6.9)

- **Retire unread columns in `lean/Semantics/Rules.lean` (§5.3):**
  - Remove `effectful` on `Definition.designation` (`Rules.lean:105-116`, ignored
    by `crates/xtask/src/facts/lean.rs:809` and retired per `lean/CONTRACTS.md`
    "GivingWarrant"), `half` / `RoomHalf.designation` / `DesignationLabel.half`
    (since Room doors use `leftHalfUnlocked` / `rightHalfUnlocked` designations),
    and `permanentType` on `Definition.cardType` (`Rules.lean:81`).
- **Cross-pin Lean catalogs against `Check/Facts.lean` (§6.9):**
  - Add missing `Empower Jace` (`[CR#701.71]`) to `keywordActionLabels` and a
    participle row for `"blighted"` (`[CR#701.68c]`).
  - Extend `lean/Semantics/Proofs/Tables.lean` so every declared `CounterKind`,
    `DesignationLabel`, `RelationLabel`, `KeywordActionLabel`, and `KeywordLabel`
    in `lean/Semantics/Definitions/` is cross-pinned against `Check/Facts.lean`
    until `semantics-v3-dsl-and-plugins` switches the facts generator from
    `plugins_v2/builtin` to `plugins_v3/`.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
