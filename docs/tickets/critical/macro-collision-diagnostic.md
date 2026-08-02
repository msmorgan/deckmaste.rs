---
needs: []
---
**Registration-time collision diagnostic: a macro whose name equals a
variant of any kind it registers under is an ERROR, killing the
silent-dead-macro footgun.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§6). Early-landable: independent of the crate program (lands in the
current machinery and survives the fork mechanically).

## Scope

- Check every kind a def registers under against that kind's COMPLETE
  dispatch set — `SupportsMacros::ALL_VARIANTS`, so flattened compartments
  count (`OneShotEffect`'s set includes every `Action` name). `Kind` gains
  the variant inventory it currently lacks.
- Enforce at ordinary insertion AND plugin-layer replacement.
- Identity-wrapper exemption via unforgeable generated metadata
  (`IdentityOf(kind, variant)`-style), never body-equality. (No identity
  wrappers exist yet; the exemption ships dormant.)
- Per-kind, deliberately: same-name different-kind reuse is intentional
  practice (`Draw` verb vs `Draw` event filter; `AnyTarget` spec vs
  predicate).
- Retire the comment-enforced naming rule on the `DrawCard` variant in
  `action.rs` (rewrite the comment to point at this diagnostic).
- Explicit test for a flattened-name collision.

## Gates

Standard constraints apply. Current corpus registers clean (zero
collisions exist — verified 2026-08-02); negative fixtures for a same-kind
collision and a flattened collision.
