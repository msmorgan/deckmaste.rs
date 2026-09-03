---
needs: [workbench-own-read]
---
**Write shared-subject coordination as `AndAlso` over full statics whose
subject is read through `Own`, and delete the reduced VP vocabulary.** Ruling
2026-09-02 on audit item R4.

`Effect.SubjectVP:2921-2927` (`VPGets`, `VPGains`, `VPDeontic`; 8 bench uses)
re-encodes three statics minus their subject — and minus `Deontic`'s patient,
as-though and rider — with `vpOk:2959-2965` re-implementing `Gets`' zone gate,
`Gains`' `grantSubjectFits`, and `Deontic`'s deed fit. It grows one row per
static that can share a subject.

## The ruling

"X gets +1/+1, gains flying and can't block" is a **list of full statics**
(`AndAlso` over statics), not its own vocabulary. The shared subject is read
through the `Own` core noun read that `workbench-own-read` introduces — the
enclosing constructor's own earlier-slot delta, `= 1`-gated, ignoring the outer
stack. This is what v1 writes: `Until(FixedUntil(EndOfTurn),
[Each(SelectAll(..), Modify(It, GainAbility)), Cant(Block(on: ..))])` — see
Glaring Spotlight (`plugins/canon/cards/Glaring Spotlight.ron:48-54`; the card
has no bench witness yet, so authoring one is part of this ticket).

Delete `SubjectVP`, `vpOk:2959`, `vpIntro`, `vpsIntro`, and `OfSubject:460`.
Each of the 8 bench uses becomes a list of the full static it was a reduced
re-encoding of. No gate is re-implemented: `Gets`' zone gate, `Gains`'
`grantSubjectFits` and `Deontic`'s deed fit apply once, on the real statics.

Size: M.

Done when: build is 23/23; `SubjectVP`, `vpOk`, `vpIntro`, `vpsIntro` and
`OfSubject` are gone from `Effect.idr`; the 8 bench witnesses are re-spelled as
`AndAlso` over full statics and still typecheck; a Glaring Spotlight bench
witness exists and typechecks; a pin refutes a shared-subject list whose `Own`
read has no unique referent, and it is non-vacuous. Standard constraints apply.
