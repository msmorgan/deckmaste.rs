---
needs: [english-v2-scope-device-distributive-measure]
---
# The scope device: cross-host verification gates (Class C, §A.3-S8)

Two classes of R1's re-measured changed paths are scope variants the collapse
admits on paper but rejects in verification. Design authority: the B7 design
doc `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`,
sections "Class C — modal scope over a Predicate Coordination is Move S, not a
principle (2026-09-05)", "§A.3-S8 — A mobile whose yield changes when a sibling
moves into it (2026-09-05)", and "Instruction to the implementer — R1 and the
collapse's cross-host gates (2026-09-05)" — that instruction is this ticket's
letter, items 1–4 and 6 (item 5 belongs to R1's third measurement).

- **Class C** (95 units at R1's measurement; witness Adventure Awaits): an
  auxiliary realized once left of a Predicate Coordination is a left-edge
  shared Constituent, already declared `auxiliary: mobile(predicate)`. First
  dump which of the three verification gates rejects the pair (entry-for-entry
  mobile inventory, `move_is_licensed` `domain` equality, `verb_frames()`
  equality) and report it before changing anything. If it is the `domain`
  gate, §C.4a is corrected: opacity holds across the edge between a Verb Frame
  and a role *it* declares, not across every frame boundary. No principle (v);
  no elimination may be written for this class.
- **Class D** (93 units; witness Abzan Falconer, `each creature you control
  with a +1/+1 counter on it`): a declared mobile whose subtree strictly
  contains the region where another mobile's host differs is scope-affected —
  span-erased inside that region, identity retained outside (§A.3-S8). Do not
  weaken the entry-for-entry rule anywhere else.
- Witnesses both directions per the instruction; `Search your library for a
  card.` still selects its frame and is not packed; corpus-wide zero site
  paths into a declared frame role; every earlier negative witness unmoved.
- Disclose Class C/D outcomes as selected-analysis changes (they stop
  asserting a wrong reading; semantics chooses), never as corrections.

Runs after phase 4 (`english-v2-scope-device-distributive-measure`) because
both edit `parser/materialize.rs`. R1 `english-v2-attachment-class-declared`
re-measures after this, phase 4, and
`english-v2-frame-preposition-adjunct-preemption` have landed.

Tier: **sol**. Standard constraints apply (closure gate, not `--workspace`).
