---
needs: []
---
**Renderer BUG: `ControllerOf`/`OwnerOf` references render a debug marker (Mana
Leak).** Found in the 2026-06-29 code review.

`fragment::reference()` has no `ControllerOf`/`OwnerOf` arm, so the canonical Mana
Leak `MustPay(actor: ControllerOf(Target(0)), …)` renders "Counter target spell
unless **[unrendered: ControllerOf(Target(0))]** pays {3}." The totality sweep
(`tests/render.rs`) only counts markers without asserting zero, and the
clean-render anchor list excludes Mana Leak, so the suite is green.

Fix: add `ControllerOf(x) → "its controller"` (or "<x>'s controller") and
`OwnerOf(x) → "its owner"` arms to `reference()`; add Mana Leak (or a synthetic
`MustPay`) to the clean-render anchor set so it can't regress.

Severity: **medium** (a shipped card renders a debug string). Effort: **S**.
