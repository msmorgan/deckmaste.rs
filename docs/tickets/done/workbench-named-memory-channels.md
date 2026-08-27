---
needs: []
---
# Rule on the named memory channel, then build it or its replacement

**[design] — NEEDS A DESIGN RULING BEFORE CLAIMING.** Claiming this means
opening the ruling conversation, not writing constructors. The forward-anaphora
ADR's binder contract, as written, forbids the mechanism the crate uses for
every one of the eight surfaces below, and no document says how these cards get
written in v2 or that they are out of scope. Building first picks the answer by
accident.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 18 and summary finding 1 (2026-08-24). Deltas only — this ticket adds the
channel question; it does not restate the binding law, which is
[oracle-text-is-forward-anaphoric.md](../../decisions/oracle-text-is-forward-anaphoric.md).

## From the v1 comparison (2026-08-24)

The crate has an `Ident`-keyed memory channel used eight ways:

> `Reference::Bound(Ident)` (`reference.rs:218`), `Reference::Linked(Ident)`
> ([CR#607.1] linked-ability memory, `reference.rs:221`),
> `OneShotEffect::Noting { key, effect }` (`effect.rs:311`),
> `Count::Noted(Ident)` (`count.rs:167`),
> `Selection::AmongNoted(Ident, Quantity)` (`selection.rs:28`),
> `OneShotEffect::Label { as, effect }` (`effect.rs:401`), and the pile family
> `SeparatePiles { group, into: Arc<[Ident]>, by, note, then }` /
> `ChoosePile { from: PileSource, by, random, then }` (`effect.rs:410,425`) with
> `Selection::PilesOf { note, of }`. `StatePredicate::RelatedBy(Ident,
> Arc<Predicate>)` (`filter.rs:88`) is an eighth.
>
> The workbench has **none of it**. […] The nearest thing is `ExiledWith : (src
> : Noun bs Object) -> {auto 0 ls : LinkSource src}` (`Experimental.idr:214`)
> with `LinkSource` admitting only `This` and `AsType t This`
> (`Experimental.idr:1661`) — a source-anchored link rather than a name-keyed
> one, and arguably the better design for [CR#607.1] (the crate's
> `Linked(Ident)` requires the author to invent and match an identifier), but it
> is one relation, not a channel.

Verified against the workbench: `Noting`, `Label`, `Pile`, `SeparatePiles`,
`AmongNoted`, `RelatedBy` return zero hits across `Experimental.idr` and
`Experimental/*.idr`; `ExiledWith`/`LinkSource` are as described.

## The tension, in the ADR's own words

`oracle-text-is-forward-anaphoric.md:16-18`, binder contract clause 1:

> **Its gate is a function of the term's context alone.** The obligation
> mentions `bs` and the constructor's own words; there is no second index, no
> slot list, and no channel above the clause.

An `Ident`-keyed note is by construction a second index. So the settled law
appears to forbid the mechanism, and there is no recorded replacement. This is
the one place in the comparison where a settled v2 law and real corpus coverage
collide.

## The two readings — decide, do not default

**A. The ADR gains a carve-out: a keyed memory is announced forward, like a
binding.** The argument to test is that the printed English *itself* names the
key — "Note the name of target instant or sorcery card … create a copy of the
card with the **noted name**" (Magar of the Magic Strings), "As long as you
control one or more creatures with **a name you noted for cards named Noble
Banneret**", "**the noted number and kind of counters**". If the key is surface
data introduced by an earlier clause and read by a later one, it is `bs`
threading with a name, not a channel above the clause, and clause 1 needs
wording that says so. Consequence if taken: clause 1 is amended once, the eight
mechanisms get one construction family, and the ADR's "no second index" loses
its current bite — every future gate has to be re-argued against the amended
wording.

**B. The law stands and the eight mechanisms need a different v2 shape.**
`ExiledWith`'s source-anchored link is the existing precedent: the relation is
anchored to the term that introduced it instead of to an invented identifier.
Consequence if taken: each of the eight is redesigned as an anchored relation,
the pile family in particular has to say what a pile is anchored to when two
piles are minted by one clause, and cards whose text writes a *name* as the key
(Noble Banneret, Volo's Journal) need an argument that the name is spelling.

Frame both against the corpus before choosing. No default; the ruling is the
deliverable and the constructors follow it.

## The measured surface

From the grammar corpus (distinct oracle-text lines):

- **29** lines write "note"/"noted" as a memory verb or read. Witnesses:
  **Magar of the Magic Strings** (note a name, read it back at a copy),
  **Sigarda's Splendor** ("As this enchantment enters, note your life total"),
  **Volo's Journal** and **Noble Banneret** (keyed by the *card name* in the
  printed text), and the counters-and-Auras exile-and-return shape.
- **41** lines write "pile"/"piles". Witnesses: **Fact or Fiction**, Boneyard
  Parley, Brilliant Ultimatum, Jace Architect of Thought, Sphinx of Uthuun,
  Sphinx of Clear Skies.

### Two witnesses named upstream that do NOT belong here

Checked against oracle text and corrected, so the list is not re-derived wrong:

- **Sword of the Meek** — "Whenever a 1/1 creature you control enters, you may
  return this card from your graveyard to the battlefield, then attach it to
  **that creature**" is an ordinary forward anaphor over the event subject. No
  key, no channel.
- **Pir, Imaginative Rascal / Toothy, Imaginary Friend** — a partner pair and a
  counters replacement. Not [CR#607.1] linked-ability memory, and not blocked on
  a key.

## Neighbours, so the same thing is not built twice

- **Pile PROCEDURE** is already owned as a protocol bundle by
  [workbench-turn-structure-and-procedures](workbench-turn-structure-and-procedures.md)
  ("Pile partitions"). This ticket owns only whether a pile may be *named*; the
  partition procedure stays there and the two must land compatibly.
- [workbench-amount-comparison-and-quantity](workbench-amount-comparison-and-quantity.md)
  carries "a noted number, 1 — Menacing Ogre" as a fold axis; that line is
  blocked here, not there.
- [workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md)
  records "Chosen/noted this game" as **zero** lines and Sengir's point-in-time
  snapshot as absent from both layers. Neither measurement is disturbed by this
  ruling; the snapshot rider is a plausible payer for whichever shape wins.
- [workbench-copy-family-residues](workbench-copy-family-residues.md) names
  Magar as a one-card rule at the copy family's edge — its blocker is this
  channel.
- [workbench-cost-tags-and-paid-readbacks](workbench-cost-tags-and-paid-readbacks.md)
  is a ninth `Ident` key and depends on this ruling; do not design a second
  keyed channel before this one is settled.

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **Garth One-Eye's not-yet-chosen name memory.** The literal-name LIST needs
  nothing new (`NameOfCard (Or [Named (PrintedName "Disenchant"), …])`
  elaborates today over all six names); what is missing is "a card name that
  hasn't been chosen" — a per-permanent record of which names this ability's
  earlier activations chose, where quality bindings are per-clause rather than
  persistent across activations —
  `docs/tickets/done/workbench-name-match-family.md`.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`ExiledWith`,
`LinkSource`, the `Effect` carriers a note would wrap, the reads),
`idris/src/Experimental/Words.idr` (`Bindings`, `Binding`, `Payload` — a keyed
mint lands here or nowhere), `idris/src/Experimental/Events.idr`, the pin
modules `idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. If the ruling amends
`docs/decisions/oracle-text-is-forward-anaphoric.md`, that edit is part of the
deliverable and the ADR is the only tracked file touched.

## Acceptance

- The ruling is written down where the binder contract is defined, in the ADR's
  own vocabulary, before any constructor lands.
- Both readings are argued against the 29 note lines and the 41 pile lines, not
  against the crate's convenience.
- If clause 1 gains a carve-out, `ProofsAnaphora.idr`'s closure proof is
  re-stated against the amended clause rather than left asserting the old one.
- Fact or Fiction is writable, or its remaining blocker is named exactly.
- Whichever shape wins covers all eight crate mechanisms or says, per mechanism,
  what the v2 spelling is and which cards it costs.
- `ExiledWith`'s source-anchored link is either generalised or explicitly kept as
  the one-off it is; it does not silently become the general answer.
- The pile procedure bundle is not sliced — this round rules on naming and hands
  the partition to its owner.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed: the ruling (user-ratified, 2026-08-26)

**Option B — no carve-out.** No name-keyed memory channel exists anywhere in
the grammar. The eight mechanisms decompose into: same-line notes = ordinary
`bs` threading at a note-marked sort (`OfChosen`'s shape); persistent notes =
STATE on the holder with anchored ungated reads (`GreatestStoredMatch`/
[CR#706.8a] is the landed precedent; counters are the doctrine's model);
piles are mentioned, never named. None of the eight is genuinely name-keyed.
`cost-tags-and-paid-readbacks` therefore takes label-sorted anchored reads on
the spell — no tag namespace.

**Unified with the same-day chosen-value ruling** (recorded on
`workbench-choice-chosen-and-ascription`): NO cross-ability discourse at all —
notes/tags are state on the holder; chosen values are CARD-SCOPE LINKAGE
[CR#607.2d] discharged by a face law over the whole AbilitySeq (cataphora-
capable, order-free), not telescope threading. The forward-anaphora binder
contract governs discourse within a text only, and survives untouched;
ProofsAnaphora is additive-only under this ruling.

**Obligations carried to the implementing rounds** (from the design
consultation, full artifact reviewed this session): (1) spot-check the v1
crate shapes the parent ticket quoted (its crate paths had drifted); (2) the
CR rule defining the "note" keyword action must be cited from data/rules/ at
the first row that lands — it was left uncited rather than invented; (3)
confirm `EntersChoice`'s exact threading extent when the choice-linkage face
law replaces it.
