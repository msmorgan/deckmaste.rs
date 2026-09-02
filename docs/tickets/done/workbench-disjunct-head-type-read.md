# workbench-disjunct-head-type-read

The closing sweep's one flagged CORRECTNESS DEFECT (2026-09-02, ledger item
12 in done/workbench-small-residues.md — schedule ahead of everything else):
`nounTy` collapses a coordinated head, so an EXCLUDED disjunct is no longer
refused — a term can read a type off the arm the sentence excluded. Newly
reachable since the deed agent-typing decision (prohibition-tail). Fix the
read so exclusion survives the collapse; re-run the sweep's measurement and
the affected pins.

## As landed (2026-09-02)

### The mechanism, exactly

`DeedParticipant` was indexed by `nounTy`'s `Maybe CardType` and had two
constructors: `Participant` for `Just t`, which asked
`deedTypeOk d r t` of every coordinated deed, and `BareParticipant` for
`Nothing`, which asked `deedBareOk d r` instead. `nounTy` answers ONE
type, so a coordinated head answers `Nothing` — `seedTyJoin` returns
`Nothing` the moment two disjuncts name different types. The two facts
"this phrase writes no type" and "this phrase writes several" therefore
arrived at the gate as the same value, and the gate took the second for
the first.

That was harmless while both `roleBare` flags at [CR#506.3]'s deeds were
`False`. The prohibition-tail round flipped `roleBare` to `True` at
Attack's and Block's AGENT, on the corpus reading that the rule says
which objects can attack or block rather than how a sentence may
describe the one it is said of. From that moment the bare arm was open
at exactly the two deeds whose type list is closed to one type, so
"target creature or land can't block this turn" passed through the arm
meant for "enchanted permanent can't block" — the land disjunct's type
was never asked. `badCantDisjunctSubject` was retired in that round as
the recorded cost, with the honest fix named and not built: "the honest
refusal would ask the head's DISJUNCT types, which `nounTy` does not
expose".

### The fix — a third head reader, and one constructor

**`nounHeadTys` in `Phrase.idr`**, beside `nounTy` and `nounTys`: every
card type the phrase's head WRITES, as a list. It is the third
projection of the same fact and the three differ in what they do with a
disagreement — `nounTy` names the phrase's one description and a
disagreeing pair names none; `nounTys` projects a description per half
of a joined KIND; `nounHeadTys` collects what was written, so a
disagreeing pair wrote two things. It reuses the existing `headTys`
(which already unions an `Or`'s arms and was previously read only by
`OtherAnchored`) at the six determiner rows, recurses through the
transparent wrappers, and unions every coordination's arms. `[]` is the
phrase that writes no type. The machinery was already there; nothing
new was minted at the phrase layer.

**`deedHeadTysOk` in `Events.idr`**: `[]` is `deedBareOk`'s question,
and any non-empty list must be admitted type by type. This is the
"meet/refusal where the consumer cannot know which arm holds" shape —
the gate cannot know, so it demands the rule admit every arm the
sentence may denote.

**`DeedParticipant` collapses to ONE constructor** over a
`List CardType` index. Two constructors over a list would have been
unsound the other way round: `all` over `[]` is `True`, so a typed
constructor left unconstrained would re-open the very hole. One
constructor with the empty case inside `deedHeadTysOk` cannot be
bypassed. The Bool twins at the two seats where the noun is not in hand
— `counterpartFits` (the deed's own complement) and `deedSubjectFits`
(the coordinated-subject seat, threaded through `vpOk`/`vpsOk`) — lost
their duplicated `case ... of Just/Nothing` for the same call.

33 signature call sites changed from `(nounTy n)` to `(nounHeadTys n)`
(26 in `Macros.idr`, 7 in `Effect.idr`), plus the three `vpsOk` sites.
The agent-typing decision itself is untouched: `deedFacts` is unchanged
and its comment now names the disjunct read as the same row's work.

### Measurement — the fix costs zero printed lines and tightens three

Re-measured over `jq 'select(.supported)'` against
`data/derived/cards.jsonl`, reminder text stripped. Two loose passes:
a coordinated subject immediately before a deed modal (17 hits, **all
17 false positives** — every one is "power 2 or less" / "power 4 or
greater", a comparison and not a head), and any card-type disjunction
in a line carrying a deed modal (55 lines / 55 cards).

**No supported line writes a deed participant whose coordinated head
names a type that deed's rule excludes.** The nearest live cases all
name types the rule admits and so are unaffected — Abeyance and
Sphinx's Decree ("can't cast instant or sorcery spells"), Single Combat
("Players can't cast creature or planeswalker spells"), all at the
`Cast` patient, whose type list holds all four. Those three are the
tightening: they used to pass through the bare arm without their types
being looked at at all, and now pass on their arms' actual types.

### Pin — 1, re-minted

**`badCantDisjunctSubject`** in `ProofsB`, beside `badCantAttackLand`:
"Target creature or land can't block this turn", refused at
`DeedParticipant` because [CR#506.3] admits only a creature at Block's
agent and the phrase may denote the land its own arm writes. It is the
defect's exact shape and the pin the prohibition-tail round retired.
**Checked non-vacuous** by mis-stating the second disjunct to
`HasSubtype (creatureType "Zombie")` — whose head type IS `Creature` —
and confirming the build rejects it with
`badCantDisjunctSubject Participant is not a valid impossible case`.
That probe also proves the refusal is the LAND arm's and not
coordination-as-such: a two-armed head both of whose arms the rule
admits still writes.

`badActivatedSpellClass` and `badCastAbilityClass` were respelled from
`BareParticipant` to `Participant` with the constructor collapse and
still refuse, on the KIND half of the same gate.

### Remainder

Other `nounTy` consumers share the collapse but not the hole: the
damage and attack gates (`damageableKind`, `attackableKind`) fail
CLOSED on `Nothing`, which is what `badDamageDisjunctHead` ("target
artifact or enchantment" at a damage recipient) already pins. Nothing
owed there. Whether any fail-closed gate should instead read the
disjuncts and admit a head all of whose arms it admits is a separate
widening question and no supported line asks for it.

### Gate

`idris/scripts/build` from a cleared `build/ttc`: **23/23, 0 errors, 0
warnings**. `cargo xtask cite check --list-noncompliant` empty;
`cite check` 0 stale over 20535 citations; `cite bless` registered no
new rule. The round's 5 citation sites were audited with
`jj diff --git | cargo xtask cite audit --diff` and each rule read
against its claim; none rewritten.
