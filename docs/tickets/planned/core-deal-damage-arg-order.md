---
needs: []
---
**Reorder `Action::DealDamage` to `(source, amount, target)` and make `source`
required.** Today it is `DealDamage(target, amount, source)` — dest-first, which
reads backwards — and `source` is an optional trailing field defaulting to
`This`. That optionality is *why* the order is backwards: the defaulted arg got
shoved to the end, forcing target+amount ahead of it. Make source required and
the fields fall into printed-sentence order — "*source* deals *amount* to
*target*" — `DealDamage(source, amount, target)`, e.g. `DealDamage(This, 3, It)`
for Lightning Bolt.

## Current

`crates/deckmaste_core/src/action.rs`:
`DealDamage(Reference /*target*/, Count /*amount*/, Reference /*source*/)`, with
`source` defaulting to `This` and omitted in the common case (`DealDamage(It, 3)`
= target `It`, amount `3`, source `This`). Convenience ctor
`Action::deal_damage(target, amount)` fills source = `This`.

## Wanted

- Variant becomes `DealDamage(Reference source, Count amount, Reference target)` —
  source first, **required** (no `This` default, always spelled).
- Re-spell every positional `DealDamage` across cards, macros, and tests, plus
  the Idris `DealDamage` constructor and the RON→Idris emitter mapping. The
  common single-source case becomes `DealDamage(This, <amt>, <target>)`.
- Optionally keep a Rust-side `deal_damage(amount, target)` convenience ctor that
  fills source = `This` to cut construction churn — but the RON/enum form spells
  source explicitly.

## Order rationale

`src, amt, dest` was chosen because it reads as the printed oracle sentence
("source deals amount to target"). The alternatives floated — `amt, src, dest`
and `src, dest, amt` — don't track the sentence. Making source required is the
enabler; an optional source could only ever sit last.

## Done

- `DealDamage` is `(source, amount, target)` with `source` required, across
  core + Idris + emitter.
- All usages re-spelled; parse⇄render round-trip holds; idris-check count
  unchanged; `cargo test --workspace` green.
