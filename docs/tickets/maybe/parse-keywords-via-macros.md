---
needs: [macro-parse-index]
---
**[design]** MVP of `parse-via-macros`: route keyword-ability parsing through the
macro registry lookup. Keywords are the most regular case (name ↔ macro, plus a
few parameterized like Protection / Landwalk), the keyword macros already exist,
and there's a bespoke keyword parse path (`parsers/keyword_ability.rs`) to fold
into the index lookup. Recognize a keyword line, emit the registered keyword
macro's invocation instead of hand-building the core node. Proves the round-trip
(authored macro ⇄ rendered text ⇄ parsed text, one source) on the easy cases
before parameterized macros.
