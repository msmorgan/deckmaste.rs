---
needs: []
---
**Close the one remaining site where an argument's restriction is dropped:
`deserialize_newtype_struct`'s raw-value capture.** Everywhere else the
restriction bit rides on the argument text itself (`Arg { text, restricted }`),
so a card-written argument stays restricted through any number of expansion
hops. This site discards it.

## The site

`crates/macro_ron/src/expand.rs`, the `RAW_VALUE_TOKEN` branch of
`deserialize_newtype_struct`:

```rust
let (resolved, _) = substitute_into(source, &self.ctx, HoleMode::PassThrough)?;
```

`substitute_into` returns the spliced text *and* whether any of the text it
spliced in was restricted. The bit is discarded.

The comment that used to sit there claimed no restriction decision is taken
because "the body is read as author vocabulary or not when it is USED." That
is wrong for the case that matters: a produced definition's body is read as
definition text, i.e. FREE, so a card-written argument spliced into it is free
from that point on. The comment now says so.

## Why it is not live

The capture only fires for a type that deserializes through ron's private
raw-value newtype. Nothing in the graph the restricted entry reads
(`Card` / `Token` / `Predicate` and their descendants) carries a `RawValue`.
The `RawValue`-bearing types are `MacroDef`, frames, params,
`deckmaste_spelling::lexicon` and `deckmaste_migrations::todo_card` — all read
through free entries.

## What makes it live

A definition-producing macro becoming invocable from card text. At that point
a card could write an argument that lands in a produced body and is read free.
The meta-macro machinery (`Quote`, the `Macro` kind) already produces
definitions; what is missing is a path from a restricted container to one.

## Doing it

Thread the bit the way every other splice site already does: `forward_arg`,
`fill_defaults`, `splice_seq` and the five hole-resolution readers all carry
the argument's own provenance into the re-read. This site should re-read with
that bit rather than with `self.ctx`'s.

The awkward part is that the destination is a *definition body*, which has no
single restriction — it is free text with possibly-restricted holes spliced
into it. Whatever shape the fix takes has to preserve that distinction rather
than marking the whole body restricted.

Acceptance: a fixture where a card-written banned spelling reaches a produced
definition's body and is refused; the comment at the site replaced by a
statement of what now holds.

Related: [[macro-author-surface]], which built the provenance mechanism and
recorded this as its one remaining gap.
