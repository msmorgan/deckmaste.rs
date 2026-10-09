---
needs: []
---
# Corpus search subcommand: `cargo xtask english-v3 grep`

## Why

Every corpus check in the 2026-10-08 session ran `rg` over the raw
`data/scryfall/oracle-cards.jsonl`, which includes cards outside the
supported corpus and reminder text the grammar never sees; one cited witness
turned out to be reminder text on a Mystery Booster card. There is no
convenient search over the supported corpus's analysed text, so make one
(user, 2026-10-08).

## Goal

One command searches the faces the census analyses, over the text the
grammar sees, and optionally restricts to faces a census left unread or
covered.

## CLI contract

`cargo xtask english-v3 grep PATTERN [-i] [--all-matches] [--count] [--names] [--limit N] [--field text|type-line] [--json OUT] [--census FILE --unread|--covered]`

- Faces: the census's own selection (`raw_corpus` with `all`: Vintage legal
  or restricted, not `set_type: funny`). Text: the census's analysed source
  (reminder text stripped for `text`; the Type Line unchanged), derived by the
  same function the census uses, never a re-implemented stripper.
- PATTERN is a Rust `regex`, case-sensitive; `-i` makes it case-insensitive.
- Default output: one line per matching face,
  `Card Name [face name] | ...40 chars before...MATCH...40 chars after...`
  (first match per face; `--all-matches` prints every match), then
  `N faces, M matches`. `--count` prints only that final line. `--names`
  prints only face identities (card name / face name), one per line, sorted.
  `--limit N` caps face lines (default 200) with "... and K more".
  `--json OUT` writes `[{identity, name, face_name, matches:[{start,end,text}]}]`.
- `--census FILE` with `--unread` (0 Readings in that census) or `--covered`
  restricts to those faces.
- Exit 0 whatever matches; a bad regex or missing file is an error with a
  message.
- Startup costs what a single-card census start costs (about 1 to 2 s), not a
  full census.
- Documented in the english-v3 README beside `diff`; `CLAUDE.md` says corpus
  attestation checks use this command, never raw `rg` over the Scryfall file.

## Method

Standard constraints apply. Unit tests in the xtask `english_v3` module with
small in-code fixtures: selection filter respected, reminder text excluded,
`--unread` filter, count and names modes. Run `cargo test -p xtask` and
`cargo clippy -p xtask --tests -- -D warnings`. Acceptance evidence:
`grep "nonland" --count`; `grep -i "nonlands?\b" --names` (Abundant Harvest,
Abundance, Gollum, Scheming Guide, The Enigma Jewel among them, not Merfolk
Surveyor); `grep "named [A-Z]" --count`; and
`grep "Activate only as a sorcery" --census <latest full census> --unread --count`;
with the wall time of `grep "nonland" --count`.

## Landing record

Measured on change `xkuounvo` (tooling only; no grammar or lexicon change).

1. **Census delta**: none; no corpus input changed. The census's
   analysed-source derivation moved into `report::analyzed_source`, called by
   both `FaceReport::new` and `grep`; behaviour unchanged.
2. **Witnesses** (acceptance, debug build):
   - `grep "nonland" --count`: `758 faces, 805 matches`.
   - `grep -i "nonlands?\b" --names`: 762 faces; includes Abundance, Abundant
     Harvest, `Gollum, Scheming Guide`, `The Enigma Jewel // Locus of
     Enlightenment / The Enigma Jewel`; Merfolk Surveyor absent.
   - `grep "named [A-Z]" --count`: `316 faces, 346 matches`.
   - `grep "Activate only as a sorcery" --census
     review-namescope/full-at-head.json --unread --count` (census of
     `mqrlzszk`): `486 faces, 493 matches`.
3. **Spot checks**: not applicable (no newly covered faces).
4. **Deviations and STOPs**: (a) the match is delimited `«…»` in context
   lines and line breaks print as `⏎`, so a line shows which text matched;
   (b) `--names` and labels print `Card Name / face name` (`Card Name [face
   name]` in context lines) only where a face name differs from the card name;
   (c) faces absent from the `--census` report satisfy neither `--unread` nor
   `--covered`; (d) `diff::load` became `pub(super)` so `grep` reads the
   census with diff's field projection; (e) output and census flags are
   flattened sub-structs (`OutputArgs`, `RestrictionArgs`) to satisfy
   `clippy::struct_excessive_bools`. No STOPs; no glossary gap.
5. **Tests**: `cargo test -p xtask`: lib 388 passed, 1 ignored
   (pre-existing); integration suites 4 and 14 passed. `cargo clippy -p
   xtask --tests -- -D warnings`: clean. Counts: 6 added
   (`english_v3::grep::tests`), 0 restored, 0 re-spelled, 0 ignored, 0
   removed.
6. **Timing**: no census run. `grep "nonland" --count`: 1.17 s wall, debug
   build, single thread, host load 5.2 on 24 cores; the `--census --unread`
   query, loading a full 32,828-face census, 1.27 s.
