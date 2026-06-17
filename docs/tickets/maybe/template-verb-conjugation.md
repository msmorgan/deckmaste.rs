---
needs: []
---
**[design]** Teach the macro `template` grammar **verb conjugation** so a verb
agrees with its subject's grammatical person and number — in BOTH directions
(render and parse), since templates are now the bidirectional source of truth
(see the integrated `parse-via-macros` line: sigil, reverse index, slot codec).

Today templates get away with fixed verb forms because the only varying subject,
`~`, is always third-person-singular ("as ~ enters", "~ connives"). But many
mechanics take a subject whose person/number varies, and the verb must follow:

- **Trigger-event templates** — `${0} casts`, `${0} dies`, `${0} attacks`: the
  subject slot can resolve to "a player" (3sg → "casts"), "you" (2sg → "cast"),
  "players"/"they" (3pl → "cast"), "each opponent" (3sg → "casts").
- **Keyword-action / effect verbs** — "~ connives" (3sg) vs a player conniving
  ("connive").

**Worked example — Ledger Shredder**, which needs TWO conjugations on one card:
- trigger: "Whenever a player casts their second spell each turn, …" — the
  cast-event template's `casts` is 3rd-singular (subject "a player"); the same
  template over "you" must read "you **cast**".
- effect: "~ **connives**" — `~` is the creature (3sg); over a player it'd be
  "connive".

So a template fragment like `${0} casts …` or `~ connives` must conjugate its
verb for whatever fills the subject — and the reverse matcher must accept every
agreeing surface form when parsing back.

## Settle first (the design dialogue)

1. **How a template marks a conjugable verb.** A new token distinct from literal
   text, `~`, `${i}`, and single-brace mana — e.g. a verb sigil carrying the
   verb's base form (`\cast`, `{{v:connive}}`), or the verb as its own typed slot
   (a `Verb`-kind macro). The base form is declared once; the renderer inflects
   it for the subject in scope.
2. **Where (person, number) comes from.** The agreeing subject is `~` (3sg) or a
   `${i}` slot resolving to a `Reference`/`Filter` (a player or object, singular
   or plural). Need a small agreement model deriving (person, number) from those:
   `~`→3sg, `You`→2sg, "each/a player"→3sg, "players"/"they"→3pl, etc. Decide how
   a verb token names its governing subject (the nearest `~`/slot, or an explicit
   binding).
3. **The conjugator.** English present tense: 2nd person and 3rd plural take the
   base ("you cast", "they connive"); 3rd singular adds -s/-es/-ies with the
   spelling rules (casts, dies→"dies", goes, flies). A regular rule + a small
   irregular table (is/are, has/have, does/do, was/were). Past/other tenses out
   of scope unless a template needs them.
4. **Bidirectionality — the hard half.** Parsing must invert conjugation: "a
   player casts", "you cast", "they cast" all recover the cast-event verb. The
   reverse index / slot matcher must match a verb token against ANY agreeing form
   and recover the subject's person/number from the parsed subject (which then
   must be consistent — "you casts" is rejected). Likely a verb-aware literal
   matcher that, given the base form + the resolved subject agreement, accepts
   the one correct inflection (or, when the subject isn't yet known, the set).
5. **Pronoun / possessive agreement (related, maybe-in-scope).** "their second
   spell" vs "your second spell" — possessive pronouns agree with the subject
   too ("their"/"your"/"its"). Same agreement model; decide whether this ticket
   covers it or splits it off.
6. **Interaction with the existing template grammar.** The verb token composes
   with the slot codec (the subject is often a `${i}` slot) and must not collide
   with `~` / `${…}` / single-brace literals.

## Shape

Builds directly on the template/codec machinery already integrated
(`deckmaste_cards::template` reverse index + bidirectional slot codec,
`render::template::fill`, `render_arg`). The realistic build after the dialogue:
a small `(person, number)` agreement derivation over `Reference`/`Filter`, a
regular+irregular present-tense conjugator, a verb token in `fill` (render) and a
verb-aware match in the index (parse). Start with the two verbs the example
needs — the cast/dies/attacks **event** family and the **connive** keyword action
— and grow the irregular table as the corpus demands. The irregular tail (odd
phrasings) stays bespoke, per the line's "hybrid that grows" design.
