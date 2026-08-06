---
needs: [card-crate-split]
---
**Mint the engine-owned base/computed characteristics seam: `trait
BaseCharacteristics` and `CardRef<T>`.** Design:
`docs/decisions/semantics-spelling-lowering.md` (§1). Deferred from
`card-crate-split`, which landed as a pure move and enforced only the
NEGATIVE half of the contract — that `deckmaste_core` carries no
characteristics abstraction. This ticket owns the positive half.

## Current state (verified at mint time, not from the design prose)

The design describes the trait as unifying "the three existing base
sources". Only two are live:

- **`deckmaste_card::CardFace`** — read directly by the layers pipeline:
  `layer::base_values` reads `face.power`/`.toughness`/… and
  `layer::base_colors(face: &deckmaste_card::CardFace)`.
- **`deckmaste_core::Token`** — read by `object.rs` when building an
  object from a token (`power: token.power.clone()`, and kin).
- **`deckmaste_core::FaceDownCharacteristics`** — defined in
  `core/src/status.rs` and exported, but has NO consumer anywhere in the
  workspace. Its base-source role is prospective (`engine-face-down` is
  the ticket that would wire it; `layer.rs`'s `base_values` doc comment
  already anticipates the branch). So the third impl is minting behavior,
  not unifying existing behavior — price it that way.

`layer::Characteristics` is the computed end already; the pipeline reads
base-in / computed-out.

## Scope

- `trait BaseCharacteristics` defined in `deckmaste_engine` and
  implemented THERE for `deckmaste_card`'s types and for core's `Token`
  (a local trait over foreign types — the orphan rule permits it). It must
  NOT live in `deckmaste_core` or `deckmaste_card`; the whole point of the
  crate split's interface contract is that neither grammar crate carries a
  characteristics abstraction.
- `CardRef<T: BaseCharacteristics>` engine-local, kept at the
  object/state boundary (object bases, stack copies, layers input).
- **Grammar enums stay monomorphic**: `CardRef` must not appear inside
  them. The grammar reaches definitions only via `TokenSpec` and registry
  names (the `tokens-predefined-registry` direction).
- Whether to add the `FaceDownCharacteristics` impl here or leave it to
  `engine-face-down` is the claimant's call — but say which, and do not
  implement the trait for a type with no reader without saying why.
- No semantic change.

## Coordination

`characteristics-atoms-crate` (design-gated, `maybe/`) may later become
the trait's home if a shared characteristics-atoms crate below both
grammars ever lands. That does not block this ticket; it is a possible
future relocation, and the design records it as deliberately NOT part of
Stage 1.

## Gates

Standard constraints apply. Zero behavior change: full workspace suites,
`cargo xtask idris-check plugins/canon` no regressions, `cargo xtask
fidelity` PASS. `cargo tree` must still prove `deckmaste_core` does not
depend on `deckmaste_card`.
