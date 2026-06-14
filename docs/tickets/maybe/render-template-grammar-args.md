---
needs: [render-template-first]
---
**[design]** Extend the card renderer's template filler so template-first rendering
(`render-template-first` added the `template::expanded` hook + the nullary wins) reaches
arg-bearing macros, not just nullary / `~`-only ones.

Today `template::fill`'s `render_arg` renders bare integers only; everything else returns
`None` and the caller falls back to structural rendering. That blocks routing macros whose
templates carry a grammar-node arg through the template path, e.g.:
- event clauses: `Dies` / `Attacks` / `Destroyed` carry `"{0} dies"` / `"{0} attacks"` /
  `"{0} is destroyed"`, where `{0}` is the subject *filter* — currently reconstructed
  structurally in `ability::event_clause` / `subject_of` / `state_word`.
- any other `{i}`-with-a-grammar-node template (filters, references, costs).

Design decision to make first: `ExpansionArgs` stores each arg as **raw RON source**, not a
typed node, so filling `{i}` with a rendered filter/reference is either (a) re-parse the
source back into the typed node and render it, or (b) index into the typed `value` for the
matching sub-node and render that. Pick before coding.

Also resolve `AsEnters`' partial template (`"as ~ enters"`, no `{0}`): it names only the
trigger window, not the `also` effect, so it can't replace the whole replacement sentence
as-is — decide whether to complete the template (`"as ~ enters, {0}"`) or keep replacement
framing structural.

Keep structural rendering as the permanent floor (hand-built `CardView`s — tokens, derived
objects, synthesized-filter tests — carry no `Expanded` provenance). Out of scope:
enum→string tables and macro-less composition (Other-/you-control, When/Whenever,
Also/Instead, Must/Cant) stay structural.
