---
needs: []
---
`scan_triggers` unconditionally `continue`s past every `StepBegan(_)`
(`trigger.rs:511`), so phase/step-entry triggers never fire — even though the
engine emits `GameEvent::StepBegan(Combat(BeginningOfCombat))` (`step.rs`) and
`event_matches` would match a `BeginningOf` pattern. Goblin Rabblemaster's "At the
beginning of combat on your turn, create a 1/1 red Goblin creature token with
haste" makes no token. Secondary gap: `WhoseTurn::Your` is ignored in the match
(`trigger.rs:127-131`), so "on your turn" scoping isn't enforced. Fix: let
`StepBegan` events reach the matcher and honor the turn-scope. Verify Rabblemaster
mints a Goblin at beginning-of-combat on its controller's turn only. Part of
demo-goblins-elves (bench card).
