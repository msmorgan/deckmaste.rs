---
needs: []
---
# Retire the unlisted `NounKind = Numeric` coordination gate

Implements owner-rulings-12b.md finding B (round 12 re-scan).

`Exclude("first.NounKind","Numeric")` on Coordination(NounPhrase) and
`Exclude("head.NounKind","Numeric")` on NonNumericComplement block *Triumphant
Chomp* (`crates/deckmaste_english_v4/deferred.txt`): *equal to 2 or the
greatest power among Dinosaurs you control, whichever is greater*. Neither is
on the ADR §4.4 kept-gate list.

Change: replace the coordination gate with one that bars only an all-Numeric
coordination (so pure *2 or 3* stays in Amount and never doubles as a
NounPhrase), let a mixed coordination project a non-Numeric NounKind, and
either justify every surviving Numeric Exclude on the §4.4 list with a
citation or retire it.

Acceptance:
- *Triumphant Chomp* reads with the intended analysis; its `deferred.txt`
  entry goes and a positive judgment pins it.
- *2 or 3* has no NounPhrase analysis besides the Amount one.
- Shared-brief hard acceptance (census, faces, judgments, strata `--check`).

1. **Census delta**: pending.
2. **Judgment set**: pending.
