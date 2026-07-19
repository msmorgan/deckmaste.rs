---
needs: [idris-mirror-enum-gaps]
---
[design] Idris card types are a bare closed enum `Type_`; Rust card types are the
open, capability-conferring `TypeDef { name, permanent, confers }`
([[composite-types-capability-gating]]) — types confer capabilities as plugin
RON data, keyed on the closed `Type` enum only as a canonical-name substrate.
Idris models *subtype* confers but not *type* confers.

Bring Idris's card-type model in line: adopt an open `TypeDef`-shaped card type
(name + permanent + `confers`) so a card type can confer capabilities the way
Rust's does, rather than a bare `Type_` constructor. Dungeon and the other
nontraditional types then fall out as data, no special-case.

Spun out of the `idris-mirror-enum-gaps` cluster-1 decision (2026-07-19): that
ticket adds a nullary `Dungeon` to `Type_` as the cheap substrate mirror; THIS
ticket is the deeper structural unification, deferred to `maybe/` as a design
item (it's a Core.idr grammar reshape, not a one-constructor add). Weigh against
[[idris-probe-soundness-gate]] — model the conferring shape without
double-representing state the engine already holds.

Standard constraints apply.
