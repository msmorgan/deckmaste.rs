# choice-A: the forward-obligation face law

Sub-round A of [workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md)
(the umbrella — read its "Ruling (user, 2026-08-26)" section and the four
routed items that cite it; those are this round's whole scope).

Owns: implementing the ruling — the choice ability mints a FORWARD OBLIGATION
discharged by a later ability's read; a new face law (sibling of
`CardText`/`CardChapters`/`CardBox`, on `textDefines`' whole-sequence fold
precedent) checks the pairing both directions; `staticChoiceIntro`'s export
through `abIntro` retires and `abIntro` collapses toward constant. Also owns
the cross-check: whether the ruling's shape covers Phyrexian Ingester's and
Drach'Nyen's same-card cross-ability memory — bench them if yes, record the
gap at the umbrella if no.

Pins (banned alternatives are STOP fences):
- NO second index on `Effect`. The obligation lives in one `So`-gate at the
  card boundary where the whole text is visible.
- HYBRID: intra-ability chooser→read stays anaphoric under `bs`;
  `ProofsAnaphora` stays byte-identical. Only cross-ability reads move.
- NO cross-ability discourse threading of any kind (§1.3 of the anaphora
  split's ruling); state stays notes-only per
  done/workbench-named-memory-channels.
- Fragment-level bare chosen-reads become tolerated overgeneration, named at
  the face law where they are refused.

Acceptance: the umbrella's `badTwoChoosersOneSortRead` line still holds;
`idris/scripts/build` PASS; no witness lost; no pin silently passing.
Standard constraints apply.

## As landed (2026-08-27)

**The face law is NOT landed, and the ruling's mechanism is refused by the
code rather than by judgement.** Two independent findings, both checked
against the sources rather than argued:

### 1. The `abIntro` collapse is blocked by the HYBRID pin, measured

`ProofsAnaphora` pins all four chosen-read gates verbatim —
`ofChosenReadsOnlyPrefix` (`countQuality q bs = 1`),
`ofLastChosenColorReadsOnlyPrefix` (`ChoiceStands (countQuality Color bs)`),
`chosenNameReadsOnlyPrefix` and `ofChosenColorReadsOnlyPrefix` — so
byte-identity fixes those gates as reads of `bs`. `staticChoiceIntro`'s
export through `abIntro` is the ONLY source of a `Quality` binding in a
card's ability-sequence context (`qualityB` has exactly two call sites:
`Effect.staticChoiceIntro` and `Phrase.kindValueIntro`, and the second is the
intra-ability distributive the HYBRID keeps). Collapse `abIntro` and every
cross-ability read loses its antecedent.

Probed, not reasoned: `abIntro (Static se) = bs` gives **32 errors over 32
benched cards** — adaptiveAutomaton, arcaneAdaptation, chromaticArmor,
conspiracy, crossroadsVillage, declarationOfNaught, engineeredPlague,
etchingsOfTheChosen, hallOfTriumph, meddlingMage, metallicMimic, mirageMesa,
nevermore, phyrexianRevoker, pithingNeedle, prismRing, rallyTheRanks,
realmwright, sharedTriumph, silverquillSilencer, solGrail, thrivingBluff,
thrivingGrove, thrivingHeath, thrivingIsle, thrivingMoor, unchartedHaven,
urzasIncubator, voiceOfAll, voidstoneGargoyle, wardSliver, xenograft. The
probe was reverted.

The only ways out are both fenced off: relax the read gates (breaks the
byte-identity pin) or declare the face's choices in a `CardFace` field so the
text can be typed at them — a card-level prenex slot list, the device
[oracle-text-is-forward-anaphoric](../../decisions/oracle-text-is-forward-anaphoric.md)
retired ("the prenex `Targeted` slot list"). So the ruling's
`EntersChoice → OfChosen` linkage cannot move off the telescope.

### 2. The discharge direction needs a whole-grammar traversal AND refuses no rule

"Every chooser finds a later reader" cannot be computed the way `textDefines`
computes its answer. `abDefinesPt` is shallow because a characteristic-defining
line is a top-level `Static`; chosen reads are not. Benched counter-examples:
`Named ChosenName` inside an **Activated** ability's target noun (Declaration
of Naught) and inside a conditional in an activated effect (Cursed Scroll),
`OfChosenColor` inside `AddMana` (the Thriving lands), `OfChosen Color` inside
a **keyword parameter** (Voice of All). A sound
detector is a fold over `Predicate` (52 constructors), `Noun` (32), `Effect`
(72), `StaticEffect` (42), `Amount` (29), `GameEvent` (31), `Condition` (14)
and the rest — ~500 cases, a permanent per-constructor tax on every later
round.

And the refusal it would buy is not a rules-impossibility. [CR#607.2d]
describes the linked PAIR; no rule refuses a card that prints "choose a
colour" and never refers to the choice — the choice is simply made and unused
[CR#608.2c]. [CR#607.5a] settles the neighbouring case in the same direction:
an ability referring to a choice with no linked chooser is "undefined" and
"won't do anything" — inert, not illegal. Under
`done/workbench-pins-refuse-rules-impossibility-only.md` that makes the
discharge half a refusal the rules do not license, at ~500 lines of standing
cost. Not bought.

**What the telescope already gives, unchanged:** a cross-ability chosen read
of sort `q` resolves to the unique EARLIER chooser of that sort, refusing a
read with no chooser and refusing an ambiguous pair. That is exactly
[CR#607.2d]'s linkage, running the forward direction the binder contract
runs. Its ProofsF pins (`badReaderBeforeChooser`,
`badTwoChoosersOneSortRead`, `badChosenReadWrongSort`,
`badChosenProtectionBeforeChoice`, `badAscribedQualityBeforeChoice`,
`badNameMatchBeforeChooser`, `badNameMatchWrongSort`) all hold on it.

### 3. Cross-check: Phyrexian Ingester and Drach'Nyen — NOT the ruling's shape, and benched anyway

Their linkage is [CR#607.2a]'s (an ability that exiles, and an ability that
refers to "the exiled cards"), not [CR#607.2d]'s (an ability that chooses a
value, and an ability that refers to "the chosen [value]"). The chosen-value
machinery binds a `Quality` kind (`qualityB`, `countQuality`, `QualitySort`);
"the exiled creature card" is an **Object** mention and no `QualitySort`
denotes it, so the ruling's face law could not have covered them whatever its
shape.

They needed no cross-ability channel either: [CR#607.2a] equates "the exiled
cards" with cards "exiled with [this object]" — one linked pair under either
spelling — and `Predicate.ExiledWith` over `LinkSource.SelfLinked` writes the
second spelling **deictically**, off `This`, reading no mention at all. Both
cards were written on that and elaborated first attempt:
`Experimental.Cards.phyrexianIngester` (the imprint trigger plus the existing
`phyrexianIngesterPump` witness) and `Experimental.Cards.drachNyen` (enters
trigger, the menace/`+X/+0`/`Define X` coordination on `AttachHost`, and
Equip). Drach'Nyen's read is written against `thisArtifact`, not
`thisEquipment`: `LinkSource.SortedSelfLinked` covers `AsType t This Nothing`
only, so the subtype-word linkage cell (the umbrella's "Linkage under a
subtype word") is untouched and still shut.

### Remainders for the coordinator

- The 2026-08-26 ruling needs a re-decision: its mechanism is unimplementable
  under its own pins (finding 1) and its new half refuses no rule (finding 2).
- No grammar, macro, proof or pin file changed this round. `ProofsAnaphora`
  is byte-identical (verified by diff — zero hunks).
