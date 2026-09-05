# Oracle text is forward-anaphoric

## Decision

Oracle text is strictly anaphoric. A pronoun, a demonstrative, a definite
participle read and a defined letter all follow their antecedent, and nothing
on a card reads a mention introduced later in reading order. There is no
cataphora, and no introduction channel above the clause for a construction to
read from. The semantics grammar encodes this as law rather than convention.

### The forward-only binder contract

Every construction that reads a previously introduced mention meets four
conditions. A new constructor is reviewed against them.

1. **Its gate is a function of the term's context alone.** The obligation
   mentions `bs` and the constructor's own words; there is no second index, no
   slot list, and no channel above the clause.
2. **`bs` at any argument position is the reading-order prefix**: the
   accumulated mints of everything textually earlier in the term, and only
   those. Every threading function either has the shape `delta ++ bs` or
   re-marks earlier material in place (`settleTargets`, which turns an
   announced target into a definite one); none inserts anything minted
   later. Every constructor telescope types each argument in the previous
   arguments' output.
3. **The gate resolves to a binding that is IN `bs`**, by counted uniqueness
   — over every compatible mention, or over the narrower set the consuming
   verb's own rule admits — or, where the text marks the read as existential
   rather than unique, by counted existence.
4. **A binding a construction mints for a sibling argument is derived from
   that construction's own earlier arguments**, never from a later one. This
   is the clause that separates a scoped binder from a cataphor.

`idris/src/Experimental/ProofsAnaphora.idr` carries 1, 3 and 4 per
constructor, as proofs where the gate can be restated as a fold and as
typechecked witnesses where the property is structural; 2 is stated there as
equations over the threading functions.

Widening a gate so that it reads something other than `bs` — or adding a
constructor whose argument reads a later argument's mint — is a change to this
contract, not an implementation detail.

### Counted uniqueness is scoped by the consuming verb's rule

Clause 3's refinement, ruled 2026-08-27. A read's candidate set may be
narrowed to the CARRIER the consuming verb's own rule demands of its object,
and the gate then counts uniqueness over that narrower set. [CR#109.2] gives
the three carriers — a description naming neither a zone nor a carrier word
means a permanent on the battlefield, "card" names a card in a stated zone
[CR#109.2a], "spell" one on the stack [CR#109.2b] — and a verb's own rule
picks among them: [CR#701.21a] lets a player sacrifice a permanent and nothing
else, so "When this creature becomes the target of a spell or ability,
sacrifice it" has one candidate in the sacrifice slot's carrier even though
the header announced the targeting spell beside the creature.

This is a refinement of clause 3 and not a third resolution form. The gate is
still a count, still a `countBy` fold, still over `bs` alone; only the
per-binding test is narrower. It is emphatically NOT a preference: there is no
find-first, no nearest-wins, no determiner tiebreak. Two candidates sharing the
slot's carrier still refuse, and those lines write definite descriptions
instead — the guide's editorial rule, unchanged.

The carrier is the verb's, not a word the card prints, so the core constructor
(`Noun.ItAt`, gated on `Words.countOnesAt`) is written only by a macro, and a
macro that writes one names the rule that gives its slot that carrier. The
split read of a union mention is the same clause-3 shape at a different fact:
`Noun.ThatHalf` gates each arm on the union its arms SHARE
(`Words.countUnionHalf`), because both arms name halves of one mention, not on
each arm's word being unique in the whole prefix.

The residue this leaves is measured, not assumed. Of ~2,541 corpus occurrences
where a bare "it" has two or more singular object antecedents in scope,
carrier-scoping resolves ~1,656 and ~886 (95% CI 710–1,061; ~700 cards) have
both candidates in the same carrier and stay refused — 67 of 69 sampled
same-carrier pairs are battlefield/battlefield. Clause-recency, the rejected
alternative, is recorded as held in reserve against that number; it is not
implemented, because it would replace the count with a find-first and cost the
`countBy` witness lemmas that make clause 3 checkable.

### The retired lifting devices

Old semantics (`idris/src/Semantics.idr`, still carried for the verifier and
for `idris/src/Bridge.idr`'s translation guide) lifted mentions out of their
surface positions. Three devices are retired from the semantics layer; putting
them back is lowering's job, not authoring's.

- **The prenex `Targeted` slot list**, which hoisted every in-situ target
  above the clause that wrote it.
- **The positional read `Target n`**, which read a slot back by collection
  index rather than by the words the card prints.
- **The `With` and `WithChosenValue` prefix binders**, which introduced an
  indefinite or a chosen value above the sentence that names it.

`Bridge.idr`'s T1, T2, T4 and T5 are exactly the transformations that
reconstruct them for the old shape. Nothing above the clause survives in v2:
"any other target" is a modifier carrying a presupposition about the prefix
[CR#601.2c,115.4], and a choice made on resolution is read back off the
binding it left [CR#608.2d].

### The rejected alternative: a full-endophora binder

A binder admitting cataphora as well as anaphora — so that a mention could be
introduced by a later argument and read by an earlier one — was considered and
rejected on 2026-08-21. It is a large refactor in service of nothing the
corpus writes: printed oracle text reads backward only, so the extra
generality would be paid for on every constructor and exercised by no card.
The rejection is what makes clause 4 above checkable at all; without it,
"scoped binder" and "cataphor" are the same shape.

### The two conditional orientations are both forward

`Effect.OnlyIf e c oth` types the condition at `preIntro e` and `Effect.If c e
oth` types the consequent at `condIntro c`. Neither is a macro over the other,
and both are forward: they differ in **where** a mention is introduced, not in
which direction one is read. "Counter target spell if it's red" introduces the
target in the effect and the condition reads it back; "If you control three
artifacts, draw two cards instead" introduces in the condition and the
consequent reads that. Under forward authoring the introduction site is the
binding structure, so one constructor cannot type both arguments in each
other's context and both orientations are core. `StaticSpec.Conditionally`
and `StaticSpec.OnlyWhile` are the same pair for the static container.

The `otherwise` arm of both is typed at `otherwiseCtx e` — the phrases the
then-branch announced plus the quantity it wrote — which is a clause written
before the arm. Forward.

### Obligations: a mention may owe a later step

A letter is introduced by use. "Draw X cards, where X is N" writes X first and
the definition second, and both are forward: the body's X is an introducing
mention (`Amount.LetterVal`, which mints when the prefix holds no such letter),
and "where X is" is a later predication on it (`Effect.Define` and its static
twin `StaticSpec.DefinesLetter`, gated on an open letter in `bs` and
re-marking it definite in place). No argument reads a later argument's mint;
clause 4 holds.

This is the contract's **obligation clause**: a mention may carry an obligation
— here, that the letter be defined — which a later step discharges by
re-marking the binding, and which the ability boundary is where to read.
Whether an undischarged obligation is refused is a rules question per
obligation. For letters the rules admit it: [CR#107.3] gives every X either a
defining ability or its controller's choice, and [CR#107.3j] gives a gained
ability's undefined X the value 0. So an open letter leaving an ability is
recorded, not refused, and the obligation is soft — a `Define` may follow;
nothing demands one. The refusals are the definition's own: no letter to define
[CR#107.3c], and a second definition of a letter the first already settled
[CR#107.3i].

Cost X and text X are one variable [CR#107.3i]: an activation cost's `{X}` or
`-X` mints the letter into the ability's context (`costIntro`) and the text
reads it. It mints the OPEN form, because [CR#107.3c] reads "an {X}, [-X], or X
in its cost and/or its text" together and lets the text define the value of a
letter the cost wrote; only an undefined one falls to its controller's
announcement [CR#107.3a]. No printed line does both, and a count is not a
refusal. A card's own mana cost is threaded the same way: `CardFace.text` is
typed at `costLetters cost`, so a face whose printed cost writes the variable
symbol hands its text that letter already bound and Prosperity's `{X}` and its
text X are one binding rather than two spellings. [CR#107.3a] is where the value
comes from — the caster announces it as the spell is cast — and [CR#107.3i] is
what makes the text's X the same one. The face states the object's scope and
not its exceptions: [CR#107.3k] gives an activated ability's own activation-cost
X a value independent of the object's, and [CR#107.3j] does the same for a
gained ability. Both are the ability's telescope to state, and neither is
enforced today — measured at 3 supported cards that write an `{X}` activation
cost on an `{X}`-cost card (Chamber Sentry, Defenders of Humanity, Wren's Run
Hydra), recorded as tolerated overgeneration.

This supersedes the scoping constructor a definition-first `WhereLetter` used
to be (`docs/tickets/done/workbench-conditional-and-coordination.md`) and the
binder-first convention argued there. The grammar now has no departure from
[Semantics v2](semantics-v2.md) §2's "constructor argument order IS textual
order".

### Deixis is not anaphora

`This` [CR#113.7], `You` [CR#109.5], the player-group words, `AttachHost`
("enchanted creature", "equipped creature" [CR#303.4m,301.5f]) and `LetterVal`
read no context and carry no gate. They are writable in the empty context, which is
the operational difference: an anaphor before its antecedent does not
typecheck, and deixis does.

## Rationale

The claim that all of oracle text's anaphora can be authored forward is what
justifies deleting the lifting devices. Until this ADR it was prose in
[Semantics v2](semantics-v2.md) §2 and §4, restated in `Bridge.idr`'s T-rule
inventory, and nothing would have failed if a constructor quietly broke it —
a new gate reading a second index, or a telescope handing a clause a later
sibling's mints, would have compiled.

Making it falsifiable costs little because Idris's telescope already forbids
the crude failure: an argument's type cannot mention a later argument. What it
does not forbid is a construction minting a binding into an earlier argument's
context on behalf of a later one, and that is what clause 4 and the obligation
clause are for.

Counted uniqueness rather than a nearest-wins tiebreak is the same decision
seen from the read side: the guide's editorial rule — repeat the noun rather
than stack ambiguous pronouns — becomes the type discipline, and an ambiguous
context is a compile error rather than a silent pick.

## Consequences

A new anaphor constructor arrives with its entry in
`Experimental/ProofsAnaphora.idr`: the structural witness that its gate takes
the context and nothing else, and — where the gate can be restated as a
`countBy`/`anyBy` fold — the identity that gives it resolution and the
prefix-split lemma for free. Adding the gate without the entry is an
incomplete landing.

A threading function is reviewed for the shape `delta ++ bs`. Handing a clause
*less* than the prefix is a scope decision and is fine (a conditioned clause
exports nothing); so is re-marking it (`thisWayCtx`, `reflexCtx` and
`delayedCtx` settle announced targets into definites through `settleTargets`;
`effIntro`/`staticIntro` settle open letters into definites through
`defineLetter`). Handing it anything minted later is a defect.

The two conditional orientations stay two constructors. A proposal to collapse
them into one is a proposal to reintroduce either the prenex lift or the
rejected endophora binder, and is refused on that ground.

`Define` and `DefinesLetter` need no authoring macro over them: they are
already written where the printed line writes them, and they bind no implicit a
card would have to supply, so [Card authoring binds no
implicits](card-authoring-binds-no-implicits.md) is satisfied by direct use.

## Tracked references

- [Semantics v2](semantics-v2.md)
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md)
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md)
- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
- [Semantics, spelling, lowering](semantics-spelling-lowering.md)
