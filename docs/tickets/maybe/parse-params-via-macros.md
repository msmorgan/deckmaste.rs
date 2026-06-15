---
needs: [parse-keywords-via-macros]
---
**[design]** Extend `parse-via-macros` past nullary keywords to **parameterized
macros**: bind macro args from the parsed text (Protection(<color>), pump
templates `gets +{0}/+{1}`, count-bearing effects). Where the
bidirectional-template question bites — a template's `{i}` args must be
recoverable *from* English, not just rendered *to* it — so it leans on
`render-template-grammar-args` (the render side of the same arg-bearing templates)
and the `parse-via-macros` decision on how a macro declares its parse side. Grows
macro-driven coverage into the parameterized middle of the corpus; the irregular
tail stays with the bespoke parsers.
