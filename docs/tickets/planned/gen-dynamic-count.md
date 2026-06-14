---
needs: [gen-token-effect]
---
Generator task. Parse dynamic counts — "X is the number of <filter>" / "for each
<filter>" — into `CountOf` / `Count::Query` wherever an amount appears (token counts,
mana production, damage, pump). None of this parses today. Upgrades fixed token
makers to Krenko-style ("create X 1/1 red Goblin creature tokens, where X is the
number of Goblins you control"), and enables scaling mana ("Add {G} for each Elf you
control") and scaling team pumps. The engine evaluates these counts already
([[engine-resolve-counts]]); this is the parse side. Sits on gen-token-effect for the
token case. Feeds canon-goblins-elves; part of tui-client.
