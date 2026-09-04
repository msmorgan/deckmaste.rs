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
