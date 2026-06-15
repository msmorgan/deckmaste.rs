# deckmaste_tui

An interactive [ratatui](https://ratatui.rs) hotseat client for the deckmaste
rules engine — the engine's first real external consumer. One human drives both
seats of a full game (a **Goblins vs Elves** demo: decklists are committed, the
cards themselves generated locally from MTGJSON) by answering the engine's *own*
enumerated decisions through its public `step()` / `submit_decision()` API.
Nothing is hand-rolled: every offered choice is built from the engine's legal
actions, and no-choice decisions auto-resolve.

## See it work

```sh
# Play it — interactive hotseat in your terminal.
cargo run -p deckmaste_tui

# Watch it play itself to a result (no UI), for a quick smoke check.
cargo run -p deckmaste_tui -- --headless
```

The active perspective **auto-follows whoever the engine is asking to decide**:
the header shows which seat you're controlling (in that seat's color), and a
`◀ YOU` tag marks your battlefield, hand, and graveyard. When priority hands off
to the other player, the color and tag swap sides.

## Keys

Press `?` in-app for the full cheat-sheet. In brief:

| Key | Action |
|-----|--------|
| `Tab` / `Shift-Tab` | cycle zones |
| `↑ ↓ ← →` | move the selection |
| `b` / `o` | your / opponent's battlefield |
| `h` / `g` | hand / graveyard |
| `s` / `e` | stack / exile |
| `Enter` | play or activate the selected card |
| `a` | pass priority |
| `y` | yield (auto-pass until something needs you) |
| `P` | pass until your next turn |
| `Space` | toggle the highlighted object (targets / attackers / blockers) |
| `?` / `q` | toggle help / quit |

Attackers are highlighted red and blockers yellow during combat; while pairing a
blocker, the cursor steers to the live attackers so you can pick which one it
blocks.

## Smoke testing under tmux

`scripts/tui-pilot` drives the real binary headlessly for capture-based checks:

```sh
scripts/tui-pilot start          # build + launch a detached 120x40 session
scripts/tui-pilot keys b h Enter # send keystrokes (tmux send-keys syntax)
scripts/tui-pilot show           # print the current screen as plain text
scripts/tui-pilot stop           # kill the session
```
