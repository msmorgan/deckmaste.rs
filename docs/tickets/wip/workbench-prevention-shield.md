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

## As landed

- `PreventCut.Shield amt` restores the running-total cut; `ShieldUse` gates `Shield` to `Repeatedly` on both `Prevents` and `Redirects`, with the [CR#615.7] rule cited at the gate.
- `Macros.preventNext` expands through `Shield`; Shieldmate's Blessing plus Ward of Piety, Carom, Daughter of Autumn, Test of Faith, Temper, Candles' Glow, Healing Grace, Harm's Way, Divine Deflection, and Shining Shoal are the eleven running-total witnesses.
- The fifteen direct `CutSome` card sites were audited: ten moved to `Shield`; Sphere of Purity, Thunderstaff, Urza's Armor, Sphere of Law, and Plated Pegasus remain `CutSome` because each Oracle text cuts an amount from every matching damage event rather than creating a running-total shield.
- `badShieldSizedByItsOwnPrevention` is re-spelled through `Shield`; new pin `badShieldNextTimeOnly` refutes a shield with `NextTimeOnly`. Its `Repeatedly` positive twin typechecked, and deliberately changing the pin to `Repeatedly` produced `badShieldNextTimeOnly Oh is not a valid impossible case`.
- Undone: nothing.

## Landing record

Measured on change `qmvykktxvrqm` with 16,337 covered lock identities.

- Construction count: `PreventCut` 4 -> 5 constructors; direct card uses `CutSome` 15 -> 5 and `Shield` 0 -> 10, with the `preventNext` macro supplying the eleventh shield witness. The printed-card bench remains 783 card definitions.
- Coverage and lock state: selected and covered English-v2 units remain 16,337 -> 16,337, parse failures remain 16,304, and construction declarations remain 384 -> 384. `english-v2-coverage.lock` is unchanged at 48,987 lines, SHA-256 `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`; `cr-citations.lock` remains 1,571 rules after `cite bless`.
- Selection census: 10,843 unique and 5,494 specificity-resolved selections, unchanged by this Idris-only edit; unresolved ties, internal failures, exception resolutions, and exception uses remain zero.
- Assurance: restored 0; re-spelled 11 card witnesses and 1 existing pin; ignored with blockers 0; added 1 pin; removed 0. All sixteen named printed cards are Vintage-supported in `data/derived/cards.jsonl`.
- Positive artifacts: 23/23 Idris build with no warnings; `cite check --list-noncompliant` 0; `cite check` 0 stale; `cite bless` 1,571 rules; diff citation audit read [CR#615.7] against the new gate claim.
- Deviations and additions: five genuine per-event cuts remain on `CutSome`, named above; no ambiguity remained. No construction, witness, test, or lock was removed, and no scratch file remains.
- STOPs: none.
