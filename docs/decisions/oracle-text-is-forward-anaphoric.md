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
   or — where the text marks the read as existential rather than unique — by
   counted existence.
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
other's context and both orientations are core. `StaticEffect.Conditionally`
and `StaticEffect.OnlyWhile` are the same pair for the static container.

The `otherwise` arm of both is typed at `otherwiseCtx e` — the phrases the
then-branch announced plus the quantity it wrote — which is a clause written
before the arm. Forward.

### `WhereLetter` is scoped binding, not cataphora

Core writes the definition before the body (`WhereLetter w def body`), where
English postposes it: "…draw X cards, where X is the number of creatures you
control." That departs from [Semantics v2](semantics-v2.md) §2's "constructor
argument order IS textual order", and it is the only place in the grammar that
does. It does **not** depart from this ADR's law, by clause 4: the binding the
body reads is `letterB w`, derived from the constructor's own first argument,
and the definition contributes nothing to it. `def` is typed at `bs`; the body
is typed at `letterB w :: bs`. Neither argument reads a binding the other
introduces.

Both orders are therefore forward, and the choice between them is convention —
binder before scope, the way a `let` is written — not law. The English order is
restored by `Macros.whereLetter` and `Macros.whereLetterStatic`, which take the
same two arguments in the same two contexts with the positions swapped.

The record on the round that minted the pair (`docs/tickets/done/workbench-conditional-and-coordination.md`) justified binder-first
differently: that "a reading-order core would be the cataphoric binder the
workbench rejects". That reasoning is withdrawn here. A body-first core would
type the body at `letterB w :: bs` exactly as the macro does, with no argument
reading a later argument's mint; it would be forward too. The shape stands on
the binder convention alone.

X is a name an ability defines [CR#107.3], not a pronoun resolving to an
antecedent — the where-clause supplies a value for a name already in scope.
That is why the two orders are informationally independent. The letter READ,
`Amount.DefinedLetter`, is a genuine anaphor over the binding the binder minted
and is gated like the pronouns; `Amount.XVal`, the cost variable whose value
the controller announces [CR#107.3a], is neither and is ungated.

### Deixis is not anaphora

`This` [CR#113.7], `You` [CR#109.5], the player-group words, `AttachHost`
("enchanted creature", "equipped creature" [CR#303.4m,301.5f]) and `XVal`
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
context on behalf of a later one, and that is what clause 4 and the
`WhereLetter` analysis are for.

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
`delayedCtx` settle announced targets into definites through
`settleTargets`). Handing it anything minted later is a defect.

The two conditional orientations stay two constructors. A proposal to collapse
them into one is a proposal to reintroduce either the prenex lift or the
rejected endophora binder, and is refused on that ground.

`Macros.whereLetter` and `Macros.whereLetterStatic` are the authoring surface
that spells the English order. Direct core-order use in the bench is legal —
`WhereLetter` binds no implicits, so [Card authoring binds no
implicits](card-authoring-binds-no-implicits.md) does not require the macro —
but it is not the printed order.

## Tracked references

- [Semantics v2](semantics-v2.md)
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md)
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md)
- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
- [Semantics, spelling, lowering](semantics-spelling-lowering.md)
