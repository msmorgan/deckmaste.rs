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
- `Target creature gains trample until end of turn.` — **parse failure**
- `Target land becomes a 3/3 creature until end of turn.` — **parse failure**

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
