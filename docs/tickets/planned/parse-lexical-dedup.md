---
needs: []
---
Mechanical lexical dups in the migrations parsers. The English color-word -> `Color`
ident map is written three times (`filter.rs:122` `color_code`, `effect.rs:256`
`color_word`, `keyword_ability.rs:367` `quality_filter`) — fold to one `color_ident()`
in `filter.rs` (distinct from core `Color::from_code`, which maps single-letter codes).
`strip_prefix_ci` is defined twice (`effect.rs:296`, `modify.rs:90`) — keep the
`pub(super)` one and delete the copy. The English type-noun -> filter map + singularizer
is split between `filter.rs:171` (`head_noun`) and `keyword_ability.rs:367` — share the
vocabulary and `singularize`, keeping the divergent `Type(...)`-vs-builtin-macro wrapper.
Pure refactor.
