---
needs: []
---
**The preterite has no finite realization.** `Whenever a creature you control
attacks, draw a card.` selects; `Whenever a creature you control attacked, draw
a card.` fails at `attacked`. `Attack` already declares its frames and its
present forms already project a finite clause, so this is not a lexeme gap — the
grammar has no path from a preterite verb form to a finite Predicate.

Sizing from the 2026-09-05 census (change `ptoxwkmmrqno`, 19,469 / 32,641
covered, 13,172 parse failures, first-failure byte attribution): **929
preterite-shaped first-failure fragments** — the largest unowned structural
family in the corpus. The bucket mixes true preterites (`entered` 89,
`attacked` 76, `gained` 53, `left` 53, `lost` 51, `triggered` 42, `tapped` 26,
`died` 106) with past participles in reduced relatives that happen to share the
`-ed` shape; the preterite share is the majority but is **not** separately
measured. Measure the split at claim and report it.

Complete by construction, against the Inflectional Form entry in
`docs/contexts/oracle-english/CONTEXT.md`: "A morphological form of a Verb
Lexeme, such as plain, third-person-singular present, preterite,
gerund-participle, or past participle." Of those five, the grammar realizes the
plain form and the third-person-singular present finitely, and the
gerund-participle and past participle non-finitely; the **preterite** has no
realization at all. The enumeration is the glossary's, not the census's, and the
census only sizes it.

Defect sentences, each with the control that isolates the gap:

- Ashen-Skin Zubera — `…target opponent discards a card for each Zubera that
  died this turn.` — fails at `died`. Control: `…that dies this turn` is the
  present form and is already reachable.
- Admiral's Order — `Raid — If you attacked this turn, you may pay {U} rather
  than pay this spell's mana cost.` — fails at `attacked`.
- Probe: `Whenever a creature you control attacked, draw a card.` fails;
  `Whenever a creature you control attacks, draw a card.` **selects**.
- Probe: `Activate only if a creature died this turn.` fails; and
  `Activate only if a creature attacked this turn.` fails identically — two
  different lexemes, one shape, so the gap is the form, not the words.
- Boldwyr Heavyweights — `Then each player who searched their library this way
  shuffles.` — fails at the preterite inside the relative clause.
- Barad-dûr — `Activate only if a creature died this turn.`

What the failure message says: at the preterite the parser offers `third person
auxiliary, other concord class auxiliary, finite copula, bare copula, …` — it
expects an auxiliary because a finite clause can only be built from the present
forms or from an auxiliary. That is the shape of the gap.

Pinned shape. Preterite is an **Inflectional Form value**, and a finite
Predicate selects its verb form by Finiteness plus Agreement — not a new
construction per tense. Give the preterite the same finite projection the
present forms already have, so that every existing finite host
(`finite_subject_gap_relative_clause`, the finite clause predicate, the
subordinate-clause bodies) reaches it with no host-side change. Agreement is
already correct and must not be touched: `Whenever creatures you control attack,
…` selects, so the Other concord class works; the preterite is agreement-neutral
in English and must not acquire a concord split.

Verify constituency link by link before building: for each finite host you
intend the preterite to reach, confirm the host takes a *verb form* slot that is
a node, not a form that spells a tense literal. A host whose form embeds the
present spelling is the wrong granularity and must be reported, not worked
around.

Ruled against: a construction per preterite verb; a `PreteritePredicate`
sibling category duplicating the present one; a closed list of past-tense-taking
verbs; spelling any preterite as a form literal; a per-lexeme `preterite:` field
used as the *licence* to build a finite clause rather than as the realization of
one (the morphology may well be per-lexeme data — the projection must not be).

STOP-and-report: any `require`/`checked by` naming a verb identity, lexeme or
card; any dominance edge or exception entry added to break a rivalry between a
preterite finite clause and a past-participle reduced relative — that rivalry is
real and expected (`creature controlled by`, `damage dealt by`), and resolving it
by fiat is the defect. If the two readings tie, report the tie.

Affected subset. Surface: `\b\w+ed\b` plus `died|left|lost|put|cast|dealt|drew|spent|made|won`.
Touched constructions: every finite predicate and finite-clause host, plus
`reduced_relative_modifier` and `participial_by_complement` (the rivalry above).
Witnesses: the six cards named. Negatives: present-form finite clauses and
existing past-participle reduced relatives must not move.

Acceptance: the standard landing record; the six witnesses select; the
present-form controls still select unchanged; the preterite-versus-participle
rivalry is reported with counts and zero unresolved ties; the measured preterite
share of the 929 is disclosed.

Baseline: change `ptoxwkmmrqno`, 19,469 covered of 32,641, 13,172 parse
failures, 0 ties, 0 internal failures. Re-measure at claim.

Glossary: the Inflectional Form entry already names the preterite; no amendment
is expected. If the landing needs a Tense entry, add it through the
`domain-modeling` skill and disclose it.

Tier: **sol** — the projection of an inflectional form into finite clause
structure is a seam question, and the preterite/participle rivalry is an open
dimension the implementer must measure rather than assume.

Standard constraints apply.
