---
needs: []
---
**Introduce Oracle text before relying on it.** The README opens by treating
"Oracle text" as a known quantity — the pitch, §Design, and the
authoritative-input claim all assume the reader knows what it is and why a
card's rules text can be treated as a source language. A reviewer who does not
play Magic gets no answer to the obvious first question: why would card text be
uniform enough to parse at all?

Add a short primer — a paragraph or two near the top, ahead of §Design — that
supplies the two properties the rest of the README leans on:

- Oracle text is Wizards of the Coast's authoritative rules text for a card,
  maintained centrally rather than fixed at print time. A card printed decades
  ago carries current Oracle wording, not the wording on the physical card.
- It is written to a house template: the same rules concept gets the same words
  in the same order across the whole card pool, and when the template changes,
  the existing corpus is re-issued to match.

Templated *and* continuously re-normalized is what makes the corpus a language
worth compiling rather than thirty thousand ad-hoc sentences — and the
discipline supplying that regularity is upstream, not this repo's.

Also update the GitHub repository description (the "About" blurb, set on GitHub
— not a tracked file) so the landing page carries the same framing in one line
instead of assuming the vocabulary. There is no GitHub Pages site; the landing
page is the rendered README.

Decisions already made, so the copy does not relitigate them:

- **Do not call Oracle text a regular language.** Formally it isn't — recovery
  is a chart parse over a construction grammar, and the README's existing
  chart-parser framing is already slated to be *softened* by the
  `portfolio-polish` claims audit, not strengthened. "Regular" elsewhere in
  this repo means regular morphology
  (`docs/english-regular-vocabulary-tables-design.md`), an unrelated sense.
  Say "templated", "regimented", or "controlled sublanguage".
- The templating claim's evidence base is `docs/oracle-style-guide.md`, a
  CR/corpus-only reverse-engineering of WotC's manual. It is evidence, never an
  external authority: describe the observed regularity, and link the guide as
  the project's own reconstruction if at all — never as a published standard.
- The primer describes the input corpus, not the parser. Adding implementation
  claims here (grammar coverage, derivation — see
  `docs/decisions/english-grammar-is-derived.md`) just gives the capstone audit
  more to walk back.

Scope is README framing plus the About blurb: no restructuring of §Design, no
new coverage claims.
