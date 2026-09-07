---
needs: []
---
**Remove the conferrer bookkeeping around designations.** Ruling (user,
2026-09-07): which keyword confers a designation is not a fact worth
tracking; a keyword's definition body grants the designation, and that is
the whole fact ([CR#701.37a] for monstrosity). Today the same fact is kept
three times and checked for agreement: `Instruction.gainDesignation` carries
a `Conferral` tag (`instructed` / `byKeyword k` / `byDeed d`,
`lean/Semantics/Words.lean`), the keyword and deed facts rows carry a
`confers` column (`Check/FactTypes.lean`, `Facts.lean`), and each
`DesignationDecl` under `plugins_v2/builtin/macros/stubs/designations/`
declares `conferrers`, which xtask inverts into the `confers` columns
through v1's `deckmaste_semantics::DesignationConferrer`
(`crates/xtask/src/facts/lean.rs`). `conferralOk` (`Check/Keywords.lean`)
compares them.

Remove all of it, standard constraints applying:

- `Conferral` and the argument on `gainDesignation` go; every macro and
  card that passes one is re-spelled (`Macros.lean` keyword expansions,
  `Cards/`, `Proofs/`); the Rust mirror follows (drift test).
- `confers` leaves the keyword-ability, keyword-action, and deed facts
  (`FactTypes.lean`, the overlays, `coreDeedConferrals`); `conferrers`
  leaves `DesignationDecl` and every declaration file; xtask's facts
  generator drops the `deckmaste_semantics` import (it must not depend on
  a deletion-bound crate for v2 data). Regenerate; `facts check` clean. The
  Idris reference table's column is emitted as before from nothing, or the
  reference emitter is told the column is gone; say which.
- `conferralOk` reduces to the surviving rules-meaningful read: an
  instructed grant needs a `checked` (effectful) designation, per the
  designation's own row. Pins that asserted a conferrer mismatch are
  re-spelled against that law; positives keep proving.
- `docs/decisions/conferrals-come-from-registries.md` gets a dated note:
  registries carry what a designation is, not who confers it.
