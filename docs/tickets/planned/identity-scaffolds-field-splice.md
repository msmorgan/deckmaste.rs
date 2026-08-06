---
needs: []
---
**16 already-committed identity-macro scaffolds cannot read the canon
spelling of the variant they claim to cover.** Diagnosed while landing
`macro-author-surface`'s Task 6 (the compiled identity registry and its
coverage gate): the gate now reports these rows as real gaps, hand-listed at
`deckmaste_semantics::authoring::FIELD_SPLICE_HAZARD`, with an explicit
temporary exception in `every_reachable_row_is_covered` — see that ticket's
"Open, to settle before restriction is switched on" section.

## The mechanism

A newtype-tuple variant whose one field is itself a NAMED struct reads FLAT
in RON — field-spliced, leaning on the derive's `unwrap_variant_newtypes`
extension — never as one wrapped value. `deckmaste_semantics::ability.rs`
documents this explicitly for `Ability`'s struct-carrying variants:
`Activated(cost: ..., ...)`, not `Activated((cost: ...))`
(`crates/deckmaste_semantics/src/ability.rs:179,188`).

But an IDENTITY MACRO's scaffold for such a variant is generated from the
derive's per-variant signature alone, which reports a bare newtype tuple as
one opaque positional slot (`Positional([Required])`) — it has no way to see
that the wrapped type is itself a struct with its own field names. The
resulting scaffold (`params: [Any], body: Activated(Param(0))`) is invoked
through the generic macro-call grammar, which for a single positional param
reads exactly ONE self-delimited raw value
(`variant.newtype_variant::<&RawValue>()`,
`crates/macro_ron/src/expand.rs:1813`) — a `key: value, key: value, ...`
argument list is not one raw value, so it fails to parse at all under
restriction. Confirmed directly: `Activated(cost: [Tap], effect:
RestartGame)` fails `read_str_restricted` with `ExpectedRawValue`.

## Why the generator can't fix this today

Automated field-splicing would need each payload struct's own field list
(names, and which carry a constructor default) so a scaffold could be
built the way `plugins/builtin/macros/identity/Normal.ron` was HAND-shaped
around `CardFace`'s fields once `Card::Normal`'s coverage gap surfaced the
same problem. The derive does not emit that for a struct kind — only
`OWN_VARIANTS`/`ALL_VARIANTS` (variant-dispatch names) and
`OWN_SIGNATURES`/`ALL_SIGNATURES` (a variant's own arity), nothing about the
fields of a plain struct sitting behind a single positional slot. Building
that emission is its own capability, the same class of gap
`macro-ron-optional-param-elision` (now `done/`) closed for "forward this
argument if given, drop it if not" — that ticket is the precedent for
sizing and landing this one: grammar support, signature/derive emission,
then the generator and coverage-gate consumers.

## Affected rows (16) and measured exposure

`Ability::Activated`, `Ability::Triggered`, `Ability::Spell`;
`OneShotEffect::Continuously`, `Label`, `SeparatePiles`, `ChoosePile`,
`May`, `If`, `AdditionalCost`, `Each`, `With`, `Distribute`, `Noting`,
`Modal`, `RevealUntil` — every one a newtype-tuple variant wrapping a
dedicated named struct in `ability.rs`/`effect.rs`.

This is not a corner case. `OneShotEffect::May` alone is spelled
`May(who: …)` **273 times** across the real committed corpus
(`plugins/builtin` + `plugins/wizards`, excluding `.ron.todo` stubs —
verified by direct count, not estimated). `Activated`/`Triggered`/`Spell`
back nearly every activated, triggered, and spell ability in the corpus.

## Known-adjacent case

`plugins/builtin/macros/identity/Normal.ron` (`Card::Normal`) hand-mirrors
`CardFace`'s fields directly, with nothing keeping the def's fields in sync
with `CardFace`'s own — the exact shape this capability would generate and
verify automatically instead. Not itself broken today (hand-verified once,
round-trip tested), but the same class of risk: a future field added to
`CardFace` with no forcing function to update `Normal.ron` alongside it.

## Acceptance

Every one of the 16 `FIELD_SPLICE_HAZARD` rows re-scaffolded field-spliced
and round-trip-verified under restriction (`read_str_restricted` producing
the same value as a native read, per row) the way `Card::Normal`/`TwoFaced`
already are. At that point:
`deckmaste_semantics::authoring::FIELD_SPLICE_HAZARD` and its rot-guard test
are DELETED (not emptied — an empty hazard list would itself be dead code),
and the temporary `except` clause in `every_reachable_row_is_covered` is
deleted along with it, so the gate goes back to asserting unconditional
coverage.

Related: [[macro-author-surface]] (the restriction flip this blocks),
[[macro-ron-optional-param-elision]] (closed precedent for the same class of
derive-emission gap).
