---
needs: []
---
**The 30 keyword-action declarations `semantics-v2-macro-bodies-keyword-actions`
left bodyless, by cause.** Its landing record (`docs/tickets/done/`) holds the
per-declaration STOP text; this ticket routes them. Three routes, each its
own claim; split this ticket if they are worked separately.

**1. Crate gaps in `deckmaste_semantics_v2` (obvious fixes, terra tier).**

- `ron::param_types()` is a closed list of eight; a body needing a
  `Subtype`, `ZoneExpr`, `Quantity`, `TokenSpec`, or `Instruction` argument
  cannot declare it (Amass, Create, Meld, Vote, Search, Face a Villainous
  Choice). Per `semantics-v2.md` §12 every `SupportsMacros` kind is a
  parameter type; make the list total the way `every_supports_macros_type_is_a_kind`
  keeps the kind set total.
- A declaration named like a native constructor at its own position is
  silently shadowed (`Shuffle` registers but can never be invoked;
  `semantics-spelling-lowering.md` §6's collision diagnostic is the v1
  precedent). The reader refuses the collision at load with both names.

**2. What the RON macro language can say (design; sol or Opus).** Lean's
`semantic_macro` bodies use two devices with no positional-RON spelling:
`capture` parameters that re-read an argument after a binding is introduced
(Fight, Regenerate) and plurality computed from an `Amount` argument
(`Amount.plur (.lit 1) = .one`: Scry, Surveil, Fateseal, Connive). Decide
whether the RON macro language grows the device (`macros-are-declarative.md`
forbids control flow; a typed capture is not control flow) or the Lean macro
is re-spelled without it. `lean-macros-from-ron` needs the same answer from
the other side, so record it in `semantics-v2.md` §12. `target-sugar-elaboration`
names Fight as its fixture for a different mechanism; related, not overlapping.

**3. Model gaps (Lean first, then the body).**

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
