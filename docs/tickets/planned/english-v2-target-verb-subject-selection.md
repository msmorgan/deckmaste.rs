---
needs: [english-v2-targeting-marker, english-v2-grammatical-relations]
---
# Target Verb with a selected Subject

Split from `english-v2-targeting-marker` (2026-09-04, second STOP). The
marker landing shipped the Targeting Marker with its two projections and the
count noun; it withheld the Target Verb, so the acceptance sentence
"a spell that targets" still fails and this ticket owns it.

Why it was withheld: a transitive Target Verb row makes finite-clause
coordination outrank the marker nominal — "Two target creatures or
planeswalkers gain 2 life." selected as `Two` (Subject) + `target creatures`
(finite predicate) + `or planeswalkers gain 2 life`. An imperative-head bar on
the verb (a declared licence feature, ruled 2026-09-04) removes only the
imperative reading; the finite reading survives on structural specificity.

The fact to declare: the Target Verb is realized only "in rules text stating
that a Spell or Ability targets" (`docs/contexts/oracle-english/CONTEXT.md`,
Target Verb). That is Subject selection — a grammatical relation the frame
declares, not a word to guard. Design it after
`english-v2-grammatical-relations` gives frames a way to select the Subject
class: the Target Verb's frame selects a Spell/Ability Subject (including the
relativized Subject of "a spell that targets" and anaphoric "it targets"), so
a bare numeral or a creature nominal never satisfies it. Keep the imperative
licence feature only if the Subject selection does not already exclude the
imperative reading.

Fences: no narrowed form, dominance edge, exception entry, or guard naming
`target`. Acceptance: "a spell that targets", "spells that target", "it
targets", "can't target you" select; every marker-subject sentence in the
5,805-identity target audit keeps its analysis; zero ties; standard
constraints apply.

## Re-coverage obligation (2026-09-04)

`english-v2-adjunct-licence-removal` retired **Infectious Curse**
(`5fbcf36b2e6006fefc9c94d13b8c7d165c436d22f6207f5e83dc03956232681c`) from the
coverage lock. The sentence is:

> Spells you cast that target enchanted player cost {1} less to cast.

It had been covered only by a wrong analysis: `fixed_duration_phrase` accepted
any noun phrase as a temporal endpoint, so `that target enchanted player` was
absorbed as a duration adjunct on `cast`. Narrowing the endpoint to a declared
temporal head removed that reading and left the unit with no parse.

The analysis that must select when this ticket lands: `Spells you cast` is a
noun phrase with an object-gap relative, and `that target enchanted player` is
a **subject-gap relative clause on `Spells`** whose head is the Target Verb
with `enchanted player` as its object — the Spell/Ability Subject selection
this ticket owns. The whole nominal is then the subject of
`cost {1} less to cast`.
