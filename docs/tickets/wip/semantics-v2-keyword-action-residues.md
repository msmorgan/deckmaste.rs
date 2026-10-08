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
  declaration if english_v3 does not need its spelling.
- Connive 0 makes no connive event [CR#701.50e], but `conniveN(0)` still
  enacts the `connive` deed: the loader wraps the body in `Enact` whatever
  the amount, and no body can say "only if N is greater than 0" around its
  own wrapper (`plugins-v2-connive-bare-and-n` landing record). The same
  shape holds for any keyword action whose rule withholds the event at 0.
  Also from that landing: `conniveN`, like every keyword action, is
  registered at the `Deed` position as `Action("conniveN")`, a deed nothing
  enacts and the facts table does not list, so a card writing it loads and
  is refused only at check (`knownAct`).

`semantics-v1-cutover` deletes `plugins/builtin/macros/action/` wholesale;
any of the 30 still bodyless at parity is a named loss in that record.
`keyword-stub-gate-columns` defers on "once the semantics-v2 keyword macros
are written"; 35 of 65 are, so its ruling reads against this ticket.
Standard constraints apply.
