---
needs: []
---
**Restore the [CR#615.7] prevention shield as `PreventCut.Shield amt` and
re-spell the fifteen witnesses.** Audit-2 N2 / regression R-C, ruled
2026-09-03. This deliberately reverses part of the `workbench-effect-axes`
fold, which was made on a premise the second audit found wrong.

`Macros.preventNext kind scope amt d = Continuously (Prevents kind
Unattributed scope (CutSome amt) Repeatedly Nothing) d` — 15 `CutSome` bench
sites, 11 `NextTimeOnly`. [CR#615.7]: "Prevent the next 3 damage … These work
like shields … Preventing 1 damage reduces the remaining shield by 1 … Such
effects count only the amount of damage; the number of events or sources
dealing it doesn't matter." `CutSome N` + `Repeatedly` means "prevent N of
each damage event" and `NextTimeOnly` means "one event"; neither is a running
total, so the fifteen sites (Ward of Piety, Carom, Harm's Way, …) currently
read "prevent N of each damage event this turn".

The first review's F9 asserted `Shield.TheNext ⊂ PreventCut.CutSome`; that
premise was wrong, and the effect-axes round recorded the conflation as an
observation rather than stopping on it.

## The ruling

`PreventCut += Shield (amt : Amount bs)` — the cut is a running total across
events, not a per-event cut. The axis is "how much is cut", not "how often the
effect applies", so a `ReplUse` value `UntilSpent` was considered and
rejected. `use` is meaningless for a shield and is gated `Repeatedly` on that
constructor.

Re-spell the 15 `CutSome` sites that print "prevent the next N damage" through
`Shield`; leave sites that genuinely print a per-event cut on `CutSome`, and
name any that are ambiguous in the landing record. `Macros.preventNext` is the
carrier and should expand to `Shield`.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; `PreventCut` has a
`Shield` constructor carrying the amount and gated to `Repeatedly`;
`preventNext` expands through it; each of the 15 witnesses is re-spelled and
typechecks, with any left on `CutSome` named and justified; a pin refutes a
`Shield` with a non-`Repeatedly` `use`, and it is non-vacuous; the [CR#615.7]
citation is registered (`cargo xtask cite bless`) and read against its claim.
Standard constraints apply.
