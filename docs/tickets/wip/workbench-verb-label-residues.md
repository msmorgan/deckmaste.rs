---
needs: []
---
# The verb-label mechanism's residues: the partition body, the raw carrier, and the labels waiting to join

`docs/tickets/done/workbench-verb-labels-open.md` turned keyword-action verbs
into labels over expanded bodies and opened the vocabulary. Four things it named
and left; they share the `Enact`/`Does` carrier and the two macro layers, so
they are one claimable unit.

## 1. Scry and surveil have no term for the two-pile partition

Quoted from that round's ledger:

> `playerScries`/`playerSurveils` state only the LOOK. Under the old shape the
> label carried the rest; now the body is the meaning, so both under-state their
> rule: [CR#701.22a] and [CR#701.25a] each continue "then put any number of them
> … and the rest on top of your library in any order". The workbench has no term
> for that split, so the expansions are knowingly partial.

The construction is a partition of a looked-at slice into two destinations with
a "any number / the rest" split and an order clause on the remainder. It is the
one thing standing between the scry and surveil labels and honest expansions.

## 2. Raw `Enact`/`Does` overgenerate every refusal the retired pins carried

Same round, "Tolerated overgeneration": `Enact` and `Does` are public and the
bench writes them raw, so eight terms that were refused before are now
type-correct — a subjectless sacrifice against [CR#701.21a], a scry with no one
scrying and a scry off the bottom against [CR#701.22a], a mill from the bottom
against [CR#701.17a], a discard not from hand against [CR#701.9a], a destroy
labelled over an exile body against [CR#701.8b], and a subjectless `Enact "Put"`.
Its own closing sentence is the decision this ticket owes:

> Restating any of these needs a label-generic gate the carrier cannot carry, or
> the macro layer becoming the only way to write a labeled action.

Decide it. "The macro layer is the only way" is a real answer with a real cost
(the bench stops writing raw constructors); so is "these stay overgenerated,
refused at the spelling boundary". Do not split the difference per verb.

## 3. The labels the open vocabulary now makes cheap

Each is one data row plus one macro under the landed mechanism, and each was
named as a blocker by a round that could not afford a core enum arm:

- **proliferate** — `docs/tickets/done/workbench-counter-family-residues.md`
  (Tromell, blocked whole).
- **manifest dread** — same ledger (Curator Beastie, blocked whole).
- **a search verb**, which `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`
  named as consolidated open gap 8: deleting `shuffledAway` admits more than
  [CR#701.24b] licenses, and the fix wanted a search row in the then-closed
  enum. Re-read that gap against the open vocabulary before designing anything.

Take them as the mechanism's proof under load, not as a card-benching exercise —
each carrier has other blockers.

## 4. Two smaller carries

- `Repeated` minted no `ChooseQ`, "nothing in the bench prints its own
  cardinality yet". Mint it against the first witness that does, not before.
- `mkStamp` has arms for move and status only: "Body shapes past move and status
  get a row when a printed line needs one." Same rule — a row per printed
  witness.

## Consumption boundary

`idris/src/Experimental.idr` (`Enact`, `Does`, `Repeated`, `mkStamp`/`stampIntro`,
the partition's new row), `idris/src/Experimental/Words.idr` (`verbFacts`,
`knownVerb`, `VerbFacts`), `idris/src/Experimental/Macros.idr` (the atom and
counted layers, `playerScries`/`playerSurveils`), the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Scry and surveil expand to their full rules body, or the shortfall is named at
  a rule rather than at "no term exists".
- §2 has one written verdict applied uniformly, with the eight terms re-probed
  against it.
- Each label joined in §3 costs one data row plus one macro and nothing else;
  if one costs more, that is the finding.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
