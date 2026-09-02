---
needs: [english-v2-stage-5-grammar-buildout-13-10]
---
Two construction-compiler gaps the 13-10 round hit and worked around:

1. The compiler emits `onset_for_<category>` only for categories some
   construction reads onset from, so a symbol run (`{P}`) used as a measure
   unit inside a nominal fails to compile. This blocks the general "N X
   worth of Y" measure construction and with it the whole pawprint modal
   family (`Choose up to five {P} worth of modes.`, 5 faces, [CR#700.2i]).
2. `OracleText` cannot be used as a nested field (a quoted document): the
   generated code fails, so the quoted-ability interior is `DocumentBlock`
   behind a `QuotedBlock` wrapper. Add a compiler test that pins the
   current behaviour as a reproducer, then lift the limitation and switch
   the quoted interior to the nested document.

Fix both in `deckmaste_construction_core`; the grammar-side changes are
one-line follow-ups in english_v2. Standard constraints apply.
