---
needs: [english-v2-grammar-migration-design]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

# Declare every preposition's attachment class from English

**R1 — first ticket of Group R (remedies, 2026-09-04 fallout audit).** Authority:
rewrite ADR "Amendment: attachment class is a declared linguistic property
(2026-09-04)", which supersedes the 2026-09-02 "preposition classes are
corpus-measured" amendment.

Defect. `PrepositionAttachment` was reverse-derived from Oracle distribution, so
a word's grammatical class records what cards happened to print rather than what
the word does. `With` landed 2026-09-04 as `PostmodifierOnly` on that method,
and thirteen of that landing's 230 gains are consequently misattached: the
suspend template `Exile … with three time counters on it` (Arc Blade,
Chronomantic Escape, Cyclical Evolution, Doom's Time Platform, Festering March,
Inspiring Refrain, Reality Strobe, Suspended Sentence, Taigam Master
Opportunist) and the attack instrument `… attacks … with one or more creatures`
(Curse of Chaos, Curse of Shallow Graves, Oath of Kaya, Rigo Streetwise Mentor).
Each means a verb-level adjunct; each has no adjunct site, so it attaches low to
the object nominal. Same method, opposite direction: `Under your control, draw a
card.` is refused because the corpus shows `under` only selected.

Pinned shape. Reverse the direction of evidence. Restate all fifteen
`vocab Preposition` rows (`crates/deckmaste_english_v2/src/constructions.rs:38`)
with a one-line **linguistic** justification per row for its
`PrepositionAttachment` value, **defaulting to adjunct-capable and
postmodifier-capable**; narrow a row below that default only on a stated fact
about the word in English. A row that cannot be justified linguistically at its
current value is widened, not kept. The second half of the 2026-09-02 amendment
is untouched: admissibility stays the conjunction of the declared class and the
complement-taking head's declared licence, and no construction or checker names
a preposition.

Reporting obligation this ticket owns. Re-measure the thirteen and say, per
identity, which of two outcomes the widened class produces: **a genuine
selection tie** (two surviving readings for the same bytes — a STOP class under
"Ruling: derived attachment (2026-09-02)"), or **low attachment still winning**
(the recorded attachment-misselection class). Route every survivor of the second
kind to `english-v2-underspecified-adjunct-attachment` by name, together with
the coordination-scope residue already routed there by the with-preposition
landing (Vicious Rivalry). Widening classes will move other identities too;
every changed identity, gained or lost, is named with its selected analysis.

Fences. A census used as a gate or as a justification for a class value ("the
corpus shows no free `under`") — the census is provenance and belongs in the
record beside the row. A `checked by` or `require` naming a preposition, noun,
verb, construction, or card. Keeping a fused per-preposition construction beside
the general PP. Resolving a real two-reading tie with a narrowed form or an
exception entry instead of a STOP.

Glossary (`docs/contexts/oracle-english/CONTEXT.md`): Preposition, Prepositional
Phrase, Adjunct, Postmodifier, Complement, Attachment. Record any term the
landing needs that the context file does not define.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Standard constraints apply.
