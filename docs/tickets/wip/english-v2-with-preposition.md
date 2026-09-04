---
needs: [english-v2-np-postmodifiers, english-v2-closed-class-single-owner, english-v2-adjunct-licence-removal]
---
**Add `with` to `vocab Preposition` and retire the three fused
`with`-postmodifiers.** The inventory has fourteen members (`after` … `under`)
and no `with`, so `"with"` appears as a bare form literal or codec-tail literal
at about eleven sites and cannot head a `PrepositionalPhrase`.
`scalar_qualification` (`"with" measure comparison`),
`degree_scalar_qualification` (`"with" degree measure`) and
`granted_ability_qualified_reference` (`reference "with" granted`) exist only
because of that gap; the general `prepositional_qualified_reference`
postmodifier and its noun-side licence check already carry every other
preposition. This is the unclosed remainder of the Plan 09 taxonomy audit's
HIGH finding that `with/without/by/as/between` cannot form PPs.

Pinned method, per the 2026-09-02 attachment amendment in
`docs/decisions/english-v2-rewrite.md`: `With`'s `PrepositionAttachment` class
is corpus-measured, never asserted, and admissibility is the conjunction of
that class and the complement head's declared licence. `PrepositionalComplement`
needs a measured decision on its non-`Object` complements here (a keyword-line
item, a quoted ability, a scalar measure): add arms only where the census
shows them, and record the census in the landing record.

Fences: declaring `With` adjunct-capable by intuition; keeping a fused
`with` construction "for now" beside the PP; a `checked by` guard naming `with`.
Verb-selected `with` (`WithObjectVerb`, `ObjectWithObjectVerb`,
`EnterWithCountersVerb`) is frame data and stays out of scope; it belongs to
`english-v2-frame-selected-prepositions`.

Acceptance: ~~`form_literal_vocab_overlaps` down by the retired `with` literals~~
(superseded by the 2026-09-04 coordinator ruling: retain the live ceiling of
5 and leave no `with` literal outside frame-selected preposition data),
the three fused constructions deleted with their coverage re-spelled through
the PP postmodifier, selection census before/after, byte-exact laws green.
Standard constraints apply.

## Landing record

STOP — the 2026-09-04 coordinator ruling resolved the claim-time overlap
contradiction, but the live coverage gate then reported a coverage drop. Do not
ship this implementation.

The superseded ticket wording above is the coordinator's correction: `with`
form literals did not contribute to the live overlap count before `With` was a
vocabulary member. The ceiling remains 5; the five unrelated survivors stay
out of scope.

On the refreshed parent-tip implementation, the 8-worker
`cargo xtask english_v2 coverage --check --workers 8` gate reported 22 lost
previously covered identities:

```text
0fffe634e9f0c326390084b6d78bc13494c28f197b22e5473ec7defca0b1dbb0
17cc7d250a98267931037617fe95888534dab820f682b56a0c9cf304878d74ca
19b5b90bccc2207a2f8568d3970bdfe71d6dd30585ad3d41daacd5935106dca5
209c5da23cf0f019eacd1e6cfc964db051dbbbbcd3e94e1ee6efb217974fd78c
220aa154c1a962d2c64974c4ff1243946a3c41eb1a792d22fcc1ff7fbbcd2be5
28acd05e1f85a9e7ad2b850ae520de6767e934f0f71e346488d9b3a6b1267072
4d55242599783e3b6aa4658895593886de8fb471a0d30d5e5a8e52bbdad3ddd8
5669ae9d87590cffe0eb1acc1e004f1295550519ca78b3d5c0d7f545daeed1f7
64178de3d33fc3b6c0586dc5e16df0ce1da48da32a19f3319e9b909fa8744527
71a4a71adbc3ea755f1014d0b6bd3cbd4acb32836bd85f527b9d9cf01f27a4cc
7951d4af00b7ac20c1eee8a4e9dd8ee021f4d11794555a0b498b52aa7fb9db3f
7a9f73046b9931230539af72b5cfada974741b06e215f2591661f5babbb24895
96f57309a5c4e6aaf7c94049171dc21fe7469fa1a240cf21acbf49b2e4e7acf4
ac7e1dd3043728ba1ef0f1fb9c8d3fb0a5a84767e9a23eb6891b8b6393b06c59
ba586bddf6ec5b6885c8ead3cb8b8144293ed6aec1070afb4a63423c2707cc40
c3d09c1e2f3c7cd1acbf0b324af772532605c660f1730751edecadfdc1dae668
c897364894ecf5618d066c810cb1cf35fe70b1d444f0e4896592b06fb174a93f
cc2a19f0c08648c9d7f9c3d87530810bc3cd75f31c727e6e3545063b50309442
d2b78784abc62fb9636f4a628012d4da4ec45bb499bc8de016ddd63d48c1cc50
dbf80c8e57948be653fd0bda229d2ec42f3e77dbea6786714623d1aa07e51621
e7f2827a71686367549d3f6d630a95d97eabec65847c45d17d578f9d095c3aa
f1e707f790fb216b33a9bf4b635530a8b2c701a14886d3c25e2fc01a7b048f24
```

The gate also reported many newly covered identities, but the coverage drop is
terminal under this ticket's stop protocol; they are not blessed and no
coverage lock was changed. A reflinked parent-tip ambiguity baseline completed
resolved with 8 workers in 107.383689678 seconds at 170546 ns/B, under host
load 14.44/20.84/20.98. The feature implementation remains uncommitted in this
workspace for the next executor to diagnose against this record.

- Scope changes: ticket moved from planned to wip by claim; stale overlap
  wording struck under the coordinator ruling.
- Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
- Gates: coverage is red due to the 22-identity drop; no red result is reported
  as passing.
- Glossary gap: none.
