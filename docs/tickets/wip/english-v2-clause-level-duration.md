---
needs: []
---
# Durations attach to the clause, not to one verb's frame

**R3 — Group R.** Authority: rewrite ADR "Ruling: adjunct licences removed;
attachment misselection is a recorded class (2026-09-03)" ("A temporal, manner,
or locative adjunct attaches to any verb clause, matrix or embedded") and
"Amendment: attachment class is a declared linguistic property (2026-09-04)".

Defect. `OptionalRole("DurationPhrase")` is declared on `Get`
(`crates/deckmaste_english_v2/src/core_verbs.ron:79`) and on no other verb.
`Gain`, `Become` and `Lose` do not carry it, so:

- `Target creature gets +1/+1 until end of turn.` — **selects**
- 2026-09-04 coordinator ruling, was: `Target creature gains trample until end
  of turn.` — **parse failure**. Re-measure and route it to
  `english-v2-tail-keyword-ability-grant`; it is not this ticket's acceptance.
- 2026-09-04 coordinator ruling, was: `Target land becomes a 3/3 creature until
  end of turn.` — **parse failure**. Re-measure and route it to
  `english-v2-copular-complement-sum`; it is not this ticket's acceptance.

A duration adverbial is a clause-level adjunct; it is not a complement of *get*.
The whitelist deleted by `english-v2-adjunct-licence-removal` was
`AdjunctLicensed`; this per-frame optional role is the same whitelist under a
different name and survived that deletion.

Pinned shape. Delete `OptionalRole("DurationPhrase")` from `Get`'s frame and let
every duration reach every predicate through `predicate_adjunct_predicate` and
its shared adjunct hosts. `english-v2-tail-keyword-ability-grant` already rules
against the reverse ("the adjunct-class landing ruled per-X slots are replaced,
not inherited; the surviving `get_power_toughness` slot is residue and out of
scope here") — this ticket is where that residue is removed, and the tail ticket
owns the other half of the same failure family (`have_keyword_ability` sitting
outside `abstract sum LexicalVerbPhrase`). Land whichever of the two is claimed
first; the second re-measures.

Sizing, provenance only: the keyword-grant family is 1,522 units and the
"`until` duration/clause on a frame that omits it" failure bucket a further 365
(2026-09-03 census, `docs/tickets/fog.md`; re-measure at claim).

Fences. Adding `OptionalRole("DurationPhrase")` to more verbs instead of
removing it. A `duration: opt DurationPhrase` slot on any construction. A
`checked by` or `require` naming a verb identity. Any census used as a gate.

Glossary: Adjunct, Duration Phrase, Lexical Verb Phrase, Verb Frame,
Complement. Record any gap in `docs/contexts/oracle-english/CONTEXT.md`.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Standard constraints apply.

## Landing record

2026-09-04 coordinator ruling resolved the prior STOP: this ticket owns only
the `Get` residue. The `gains` and `becomes` witnesses are re-coverage owed to
their named successor tickets, not acceptance requirements here.

Safe work completed: removed `OptionalRole("DurationPhrase")` from the `Get`
Verb Frame; moved the power/toughness realization into `LexicalVerbPhrase`; and
removed its per-construction duration field and codec tail. The retained typed
test now proves the `gets` witness selects through
`predicate_adjunct_predicate` with a `DurationPredicateAdjunct`.

Full refreshed-tree gate artifacts and the re-measurements follow below.

Re-measurements: `Target creature gets +1/+1 until end of turn.` selects with a
unique clause-level `PredicateAdjunctPredicate` duration analysis. `Target
creature gains trample until end of turn.` and `Target land becomes a 3/3
creature until end of turn.` both remain parse failures, respectively routed to
`english-v2-tail-keyword-ability-grant` and
`english-v2-copular-complement-sum`.

STOP — coverage check on the refreshed tree selected and covered 17,048 of
32,641 units, with 16 unexplained losses. The named identities are Death Match,
Cultivator of Blades, Primal Boost, Battle-Rattle Shaman, Attack-in-the-Box,
Blightcaster, Undead Executioner, Fourth Bridge Prowler, Akoum Battlesinger,
Dreamspoiler Witches, Thorntooth Witch, Resilient Khenra, Death Pulse, Anointed
Deacon, Caustic Crawler, and Junkyo Bell. They are regressions until each is
traced and either restored or routed by a ruling; do not integrate this change.

The same check gained 12 identities, all selected with the generic replacement
predicate analysis: Pariah; Tekuthal, Inquiry Dominus; Increasing Vengeance;
Vassal's Duty; Empyrial Archangel; Whispers of Emrakul; With Great Power . . .;
Secrets of the Key; Treacherous Link; Razia, Boros Archangel; Protector of the
Crown; and Pariah's Shield. The selected analyses are their byte-exact Oracle
texts printed by coverage, each ending in the ordinary `instead` replacement
predicate; no negative oracle was admitted.

Coverage performance advisory: workers 8; wall 107.181794158 s; 121,650 ns/B;
host load 4.77 / 8.12 / 8.97. The 16.26 s quiet-host ceiling is advisory under
this concurrent load. Permitted licensing checkers: 20; forbidden: 0.
