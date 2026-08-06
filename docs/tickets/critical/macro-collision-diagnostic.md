---
needs: [macro-author-surface]
---
**Registration-time collision diagnostic: a macro whose name equals a
variant of any kind it registers under is an ERROR, killing the
silent-dead-macro footgun.** Design:
`docs/decisions/semantics-spelling-lowering.md`
(§6). **Resequenced (2026-08-02):** the early-landing option is
withdrawn — once the fork landed, the dispatch machinery this checks was
moving underneath it (fork duplication, then the `macro-author-surface`
rewrite: suppression, registered kinds, the identity fleet). Land it on
the settled Stage-2 surface.

## Scope

- Check every kind a def registers under against that kind's COMPLETE
  dispatch set — `SupportsMacros::ALL_VARIANTS`, so flattened compartments
  count (`OneShotEffect`'s set includes every `Action` name). `Kind` gains
  the variant inventory it currently lacks.
- Enforce at ordinary insertion AND plugin-layer replacement.
- Identity-wrapper exemption keyed to the compiled identity registry
  (`(kind, variant, signature)` — unforgeable, never serialized RON, never
  body-equality). By claim time `macro-author-surface` has minted the
  fleet, so the exemption is live on day one and must pass every identity
  macro.
- Per-kind, deliberately: same-name different-kind reuse is intentional
  practice (`Draw` verb vs `Draw` event filter; `AnyTarget` spec vs
  predicate).
- Retire the comment-enforced naming rule on the `DrawCard` variant in
  `action.rs` (rewrite the comment to point at this diagnostic — in
  whichever copies survive at claim time; the semantics fork is the live
  dispatch surface).
- Explicit test for a flattened-name collision.

## Gates

Standard constraints apply. Zero NON-identity collisions at claim time
(the pre-fork corpus verified clean 2026-08-02; the identity fleet
collides by design and passes via the registry); negative fixtures for a
same-kind collision and a flattened collision.
