---
needs: []
design: true
---
**Engine: scope `Simultaneously`'s remaining non-verb member shapes.**

`resolve/effect.rs`'s `OneShotEffect::Simultaneously` arm wires two member
kinds: an `Act` verb (lowered to events) and a `Continuously` whose body is a
bare `Modify(_, _)` (scope resolved read-only in phase A, row minted in phase
B). `engine-simultaneous-non-verb-members` built that split for Avarice
Totem's control exchange and deliberately stopped there.

Two seams remain, both loud:

- A `Continuously` member with any other `StaticEffect` body — `Each`,
  `Deontic`, `CostModifier`, `CantHappen`, prevention. The blocker is the
  all-or-nothing void test ([CR#701.12a]): a `Modify` body's `Locked` scope is
  emptiness-testable at mint, whereas a `Floating` scope deliberately is not
  expanded to objects there, so "did this member come up empty" has no answer.
- Any other `OneShotEffect` member — `Sequentially`, `If`, `May`, `With`, a
  nested `Simultaneously`.

This is a **scoping ticket, not an implementation ticket**: no card drives
either shape today. Establish which, if any, a real card needs before
building, and split per shape rather than widening the match arm at once.

Not in scope: a choice-bearing `Act` member, whose verb lowers to a decision
rather than events — `engine-simultaneous-choice-members` (maybe/) owns that.

Effort: **S**.
