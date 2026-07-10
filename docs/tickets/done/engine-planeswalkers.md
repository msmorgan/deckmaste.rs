---
needs: [core-loyalty-costs, engine-counters-api]
---
Planeswalker loyalty enters/activation/SBA and attacking planeswalkers (attack-target
choice). Needs loyalty-cost grammar and counter apply machinery. Compleated (Phyrexian
loyalty payment) is split out to the sibling `engine-compleated` ticket; rich AI/TUI
attack-target selection and wiring rules-as-data into headless `sim::play` are split to
`planeswalker-attack-target-ui` and `sim-play-wire-rules-as-data`.
