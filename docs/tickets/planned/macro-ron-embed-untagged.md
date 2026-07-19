---
needs: []
---
Convert every raw `#[serde(untagged)]` RON-bound core enum to `macro_ron`'s
`#[macro_ron(embed)]` so bare cardtype/subtype macros expand inside them and
there is ONE untagged-replacement idiom for authors and agents to copy.

Background: serde's untagged derive buffers the whole enum value into serde's
private `Content` via `deserialize_any`, then replays it through a
non-macro-aware deserializer — so a bare `Type(Artifact)` written inside an
untagged variant fails to expand ("data did not match any variant of untagged
enum ManaProduction"). `macro_ron`'s `embed` variant is the macro-aware
replacement (name-erased newtype, unknown idents fall through to the embedded
type, stays macro-aware) — already exercised by the `EmbedHost`/`EmbedRef`
fixture in `crates/macro_ron/src/tests.rs`.

Six enums carry raw `#[serde(untagged)]` today:

- `crates/deckmaste_core/src/mana.rs` — `ManaProduction` (untagged `Bare(ManaSpec)`;
  its tagged `WithRiders{riders}` nests `SpendOnly(Predicate)` → `Type`/`Subtype`
  macros — CONFIRMED poison, this is the Vibranium failure), `ManaSpec` (untagged
  `Specific(ColorOrColorless)`; latent poison via `AmongColorsOf(Reference)`),
  `SimpleManaSymbol` (untagged `Specific`), `ManaSymbol` (untagged `Simple`).
- `crates/deckmaste_core/src/color.rs` — `ColorOrColorless` (untagged `Color(Color)`).
- `crates/deckmaste_core/src/card.rs` — `StatValue` (untagged `Number(Int)`).

The two mana enums are correctness (they nest macros); the four leaf enums are
consistency (single prior art). Each conversion: swap the attribute, wire the
`macro_ron` derive participation + kind registration (`.embeds_untagged()` in
`kinds()`) per the `EmbedHost` fixture, and prove the round-trip stays flat —
bare spellings (`White`, `Colorless`, `Generic(2)`, `3`) render byte-identical.

NOT in scope: any gate/CI check forbidding raw `#[serde(untagged)]`. If the
mistake recurs it gets fixed then, better-informed; consistency alone removes
the multiple-prior-art confusion.

Acceptance:
- `plugins/builtin/tokens/Vibranium.ron` authors the rider with bare
  `Type(Artifact)` (not `Type(name: "Artifact", permanent: true)`) and validates.
- `cargo xtask validate plugins/builtin` stays 11/0.
- Full test suite green.
- Render / serialize round-trip byte-identical for every converted enum.
