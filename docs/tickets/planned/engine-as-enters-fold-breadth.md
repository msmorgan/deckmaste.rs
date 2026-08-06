---
needs: []
design: true
---
**Engine: scope `apply_as_enters`'s catch-all (`replace.rs`).**

`apply_as_enters` folds a self `Also(would: Enters(This), also: …)`
replacement into `EnterStatus`. Handled today: `Act(Tap(This))`,
`With(ChooseOne(quality), Attach(This, It))` (enters attached),
`Act(PutCounters(This, kind, n))`, `If` (the taken branch, recursively), and
`Sequentially`/`Simultaneously` (composing any of the above). Everything else
falls into one generic `todo!` seam — a single catch-all cannot honestly name
one owning ticket, because the residual `also` shapes belong to different
lanes.

This is a **scoping ticket, not an implementation ticket**: enumerate which
`also` shapes a self `Enters(This)` replacement can realistically carry, and
route each to its owner, rather than pre-guessing an implementation.

Known destinations already split out:

- **Entry-time player choices** ("as this enters, choose a color/type/
  number/opponent") — a `With(binder, body)` shape distinct from the
  attach-quality `With` already handled; the choice must be stored and read
  back by other abilities. Owner: `core-as-enters-choices`.
- **Face-down entry** (manifest/morph/disguise/cloak self-text, if any card
  actually prints it as an as-enters replacement rather than an alternative
  cast cost). Owner: `engine-face-down`.

Everything else in the `OneShotEffect` grammar (`Continuously`, `Until`,
`Label`, `SeparatePiles`, `ChoosePile`, `May`, `AdditionalCost`, `Each`,
`Distribute`, `Noting`, `Delayed`, `Reflexive`, `Modal`, `Targeted`, `Repeat`,
`Batch`, `RevealUntil`, `Expanded`, and any bare `Act(...)` other than
`Tap`/`PutCounters`) has no confirmed real-card driver reaching this seam
yet. Don't pre-build folds for these; when the `todo!` actually fires against
a real card, route it to the matching existing ticket or split a new one
here.

Done when every `also` shape with a real corpus card behind it is routed to
an owning ticket (existing or newly split), and this list is the reference
for the next one that fires rather than each getting re-derived from
scratch.

Effort: **S**.
