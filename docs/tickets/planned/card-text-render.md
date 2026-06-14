---
needs: []
---
The beginnings of card-data → text: render an object's structured definition and its
derived characteristics into approximate, readable rules text for the TUI's detail
pane (the "runnable in reverse" stretch goal, first cut). Works from the encoding,
not stored oracle text — so it covers tokens and modified/derived objects (pumped,
animated, control-changed) that have no printed text, and stays IP-clean by not
reproducing WotC's verbatim oracle text. Scope v1 to what the Goblins-vs-Elves set
uses (types, P/T, mana cost, keywords, simple triggered/activated/static abilities,
token specs); approximate phrasing is fine. Feeds tui-board-view's detail pane; part
of tui-client.
