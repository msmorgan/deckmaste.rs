---
needs: [english-lint-selectional]
---
**Object-gap relative clauses attach too high — 431 supported faces.** The
general form of the defect `english-ast-grouping` fixed for one family. That
ticket corrected `damage … to X and each creature that …`; the underlying
attachment rule was not generalised, and `uncontrollable-host` now proves the
rest mechanically.

Measured 2026-07-29 by `cargo xtask english lint --check uncontrollable-host`:
431 faces where an object-gap relative whose verb requires an object
[CR#109.1] attaches to a mass-noun host. Damage is not an object — it is what
objects *deal* [CR#120.1] — so these are wrong whatever the card means.

- **Acidic Soil** — `<<damage> … <of <lands>>> <<they> <control>>`; the clause
  belongs to `lands`.
- **Abzan Monument** — `<<toughness> <among <creatures>>> <<you> <control>>`;
  belongs to `creatures`.

The shape is the same each time: the relative attaches to the NP the parser is
currently building rather than the innermost eligible host. A coordination
member or PP complement must stay the open attachment site until a trailing
modifier is consumed or refused.

**Every one is invisible to round-trip** — the tokens render back unchanged and
the recovery census stays at zero — so `uncontrollable-host` is the regression
gate for this work. It must reach 0; run it with `--require-clean`. Note the
lint's table is only `Control`/`Own`, so 431 is a floor, not a count of the
family. Standard constraints apply.
