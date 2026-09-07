---
needs: []
---
**Model gaps behind the keyword-action declarations still bodyless after
`semantics-v2-macro-bodies-keyword-actions`.** Its landing record
(`docs/tickets/done/`) holds the per-declaration STOP text. The crate gaps
are `semantics-v2-macro-param-kinds`; the RON macro-language question is
`semantics-v2-macro-capture-and-plurality`. This ticket is the Lean-first
modelling route: each shape below is added to `lean/Semantics` with its pins,
mirrored, then the declaration gets its body.

- No v2 shape: Cloak, Manifest, Manifest Dread (face-down substrate; engine
  side is `engine-face-down` and `engine-special-actions`), Collect Evidence,
  Discover, Incubate (`tokens-predefined-registry` lists Incubator), Learn,
  The Ring Tempts You (`macro-ring-emblem` holds the engine blockers; its
  "future builtin macro expands to `Action::Composite`" framing is v1 and
  moot), Venture into the Dungeon (`engine-dungeons`, same framing, same
  note; `lean-core-room-declarations` holds the designation question),
  Waterbend.
- Rules procedures, not instructions: Activate, Cast, Play.
- One phrase, several expansions: Double, Triple, Exchange
  (`lean-core-exchange-operands` is this question for Exchange alone; fold it
  in), Support.
- Assemble is Un-set only and stays bodyless permanently; delete the
  declaration if english_v2 does not need its spelling.

`semantics-v1-cutover` deletes `plugins/builtin/macros/action/` wholesale;
any of the 30 still bodyless at parity is a named loss in that record.
`keyword-stub-gate-columns` defers on "once the semantics-v2 keyword macros
are written"; 35 of 65 are, so its ruling reads against this ticket.
Standard constraints apply.
