---
needs: [english-v2-locative-licence-data]
---
Single ownership for closed-class words (homograph review Class C, 17
rows): `less`, `equal`, `greater`, `more` (three owners), `twice`,
demonstrative `that`, `fewer`, `both`, `any`, `instead` are spelled both
as form literals and as vocab members. One owner each: consume the vocab
member through the existing restricted-slot idiom (`checked by
singular_demonstrative_is_this()`-style, constructions.rs ~:3334) and
delete the duplicate literal. Class A (16 rows: infinitival `to`,
complementizer `that`, `for each`, `at random`, "as you choose") are
genuinely different words — declare their licence on the form atom so
the census separates governed from ungoverned. Split the metric:
`licensed_vocab_lexicon_homographs` pinned EXACTLY at 2 and
`form_literal_vocab_overlaps` as a decreasing ceiling (70 today; this
ticket lowers it by 17 + the 16 licensed), replacing the bumpable exact pin (re-measure its live value at claim — it has moved 72 -> 60 -> 59 across landings; the ceiling belongs in `coverage --check` beside the lock, not in a `#[cfg(test)]` unit test) and the loosened `assert!(… > 1)` environment pin.
Also fold the flavor-word landing's 2x2 label constructions ({AbilityWord, FlavorWord} x {bare, em-dash-prefixed}) through a shared `LabelTerm` sum (flavor review M1; -2 constructions). Zero coverage change; standard constraints apply.
