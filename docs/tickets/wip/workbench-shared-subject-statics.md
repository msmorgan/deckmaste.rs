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

## As landed

- `SubjectVP`, `SubjectVPs`, `vpIntro`, `vpsIntro`, `vpOk`, `vpsOk`, `vpSpanOk`, `deedSubjectFits` (its only caller was `vpOk`) and `OfSubject` are gone from `Effect`.
- The coordinating node is `SharedSubject n parts` (`parts : StaticParts k (selfSubjIntro n)`, `IsSucc k`, kind `Coordination`, intro `partsIntro parts`): the subject is introduced once by the node, and each body static reads it through `Macros.ownSubject n` = `Own (nounPlur n) (selfSubjDelta n ++ nounDelta n) bs` — the node's own delta, `= 1`-gated, the outer stack ignored. `Macros.sharedSubject n parts d` wraps it in `Continuously` with one span for the list, as v1's `Until` does.
- `Own` gained a positional plurality (`Own pl own outer`, gate `countReach Bare pl own = 1`) so a plural described subject ("creatures you control") can be shared; `dealsDamageOwnPower` fixes `OneOf`. The body static must repeat the subject inside `ownSubject` — a `let`-bound subject leaves the auto gates stuck.
- Deictic subject: a subject that mints nothing (bare `This`, any pronoun such as `ItVerbed "Untap"`) cannot head the node — `ownSubject` refuses at count 0 — and the spelling is `Continuously (AndAlso [...])` with the read repeated (Aim High). `AsType t This` ("this creature") mints a `SelfD` binding through `selfSubjDelta`, so it may head the node, but the repeated read stays the plainer spelling.
- No gate is re-implemented: `Gets`' zone gate, `Gains`' `GrantSubject` and `Deontic`'s deed fit apply on the real statics; the node adds only `IsSucc k`.
- Bench re-spellings: Ghor-Clan Rampager, Tattoo Ward and Distortion Strike through `SharedSubject`/`ownSubject`; Aim High through `AndAlso` (pronoun subject). Distortion Strike's two per-clause spans collapse to the list's one `untilEndOfTurn`.
- `glaringSpotlight` is a new bench witness (supported, vintage-legal; both abilities, the grant list as `sharedSubject (AllOf creatureYouControl) [Gains …, Deontic … Forbid ["Block"] Patient …]`).
- Proofs: `ofSubjectReadsNoPrefix` → `sharedSubjectReadsNoPrefix`; `ownReadsOnlyPrefix`/`ownResolvesInPrefix` take the plurality; new witness `sharedSubjectSurvivesSecondSingular`; new pins `badSharedSubjectEmptyDelta` (pronoun subject) and `badSharedSubjectTwoInDelta` (`BothOf` subject read `OneOf`), both probed non-vacuous.
- Undone: nothing in the ticket's letter. Observation: a `Gets +X/+X` that mints a letter before a later `ownSubject` in the same list would need the raw `Own` split (no bench case has the shape).

## Landing record

- Construction count: `StaticEffect` −1/+1 (`OfSubject` → `SharedSubject`), `SubjectVP` −3 and `SubjectVPs` −2 (types deleted), `Noun.Own` +1 positional param.
- Coverage and lock state: printed-card bench coverage +1 (Glaring Spotlight), no witnesses removed, `cr-citations.lock` unchanged.
- Assurance: restored 0; re-spelled 6 (Ghor-Clan Rampager, Aim High, Tattoo Ward, Distortion Strike, `sharedSubjectReadsNoPrefix`, `ownReadsOnlyPrefix`+`ownResolvesInPrefix`); ignored 0; added 4 (`glaringSpotlight`, `sharedSubjectSurvivesSecondSingular`, two pins); removed 3 — `subjectVPsThreadPrefix`, `vpIntroIsDeltaThenPrefix`, `vpGainsMintsNothing`, each a proof about the deleted `SubjectVP` telescope whose surviving shape (`StaticParts`) `staticPartsThreadPrefix` already covers.
- Positive artifacts: 23/23 Idris build; `cite check` 0 stale of 17884; diff audit selected 0 sites; `--list-noncompliant` reports one pre-existing string in `docs/tickets/critical/engine-cost-preflight-atomicity.md:22`, outside this round's edit scope.
- Deviations and additions: `Own` plurality param (needed for a plural shared subject); `Macros.ownSubject` added; `staticChoiceDelta`/`clauseStaticOk` gained `SharedSubject` clauses mirroring `AndAlso`.
- STOPs: none.
