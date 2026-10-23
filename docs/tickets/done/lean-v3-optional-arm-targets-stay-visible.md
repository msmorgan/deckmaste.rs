---
needs: []
---
# Targets written in an optional body's arms stay visible too

Standard constraints apply. Follows `lean-v3-optional-body-targets-stay-visible` (done), which made
a target announced in an optional body (`may`, `Action.withContinuation .optional`) an ordinary
referent after it, and left targets written only in its "if you do" / "if you don't" arms as they
were.

## Ruling (owner, 2026-10-23)

Extend the rule to the arms. The arms are part of the same ability, so their targets are also
chosen as the ability is put on the stack [CR#601.2c,603.3d,115.1], whichever arm later runs. A
"when you do" reflexive trigger is a separate ability that chooses its own targets when it is put
on the stack [CR#603.12], and is unaffected.

## Change

- In the `.withContinuation` arm of `stepOps` (`Check/Phrase.lean`), collect the targets the
  arms announce, as the landed change does for the body, and add them to the ordinary referents
  after the arm. Non-target referents stay as they are.
- Keep `ordinaryLocalsAndOptionalTargetsRemainClosed` and the optional-body pins unchanged.
- ADR `docs/decisions/semantics-v3.md`, the Participial Descriptions paragraph: say the arms'
  targets escape too.

## Tests

- A target written in the "if you do" arm is readable after the whole `may`.
- The same for the "if you don't" arm.
- A "when you do" reflexive trigger's target does not escape into the text after it.
