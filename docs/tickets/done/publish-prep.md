---
needs: [tui-client, engine-derived-type-reads, engine-count-eval-unify, engine-trigger-target-restrictions]
---
Meta / tracking ticket: get deckmaste.rs to a state worth publishing as a portfolio /
resume project (the README "see it work" link that tui-polish and demo-goblins-elves
already point at). `needs:` lists the release-blockers:

- `tui-client` — the interactive demo epic; a resume project needs something you can
  SHOW running, so the Goblins-vs-Elves hotseat client (and its decomposed UI / render
  / generated-deck subtasks) gates publication.
- The correctness blockers surfaced by the 2026-06-13 duplication audit — bugs that give
  wrong results or panic on valid input, embarrassing to ship: `engine-derived-type-reads`
  (animated permanents mis-typed), `engine-count-eval-unify` (condition-side count
  comparison panics), `engine-trigger-target-restrictions` (triggers can target hexproof).

The other open `critical/` engine tickets (resolve-effects, x-costs, cost-payment,
replacements, sba-breadth, counter/trigger work, ...) are the broader functional release
gate and should be folded into this `needs:` list as they're triaged. Non-blocking polish
from the same audit lives in separate planned tickets — `engine-filter-walker`,
`engine-ability-walk-dedup`, `engine-keyword-name-lookup`, `engine-announce-scheduling-dedup`,
`engine-cause-constructors`, `parse-lexical-dedup`, `macro-ron-dedup` — code-smell
duplication worth cleaning before a code reviewer reads it, but not broken behavior.
