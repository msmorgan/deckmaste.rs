Make the gate's collision/structural metric real (dissolution review F1).
`literal_lexicon_collisions` in the coverage summary is never assigned —
a constant zero that reads as a live gate. The 2026-09-02 ruling asked for
census VISIBILITY of the noun-as-literal class. Replace the dead counter
with measured per-unit structure: per-claim-class byte shares are already
computed; add a structural-depth figure (nonterminal nodes per sentence,
longest single-form literal span) and a literal-vs-lexicon surface
collision count that the tripwire's exact-surface check feeds. Report in
the summary line; no ratchet on it yet — visibility first. Also extend
the compile-time tripwire to `vocab` surfaces (review F6: two live
duplicates of CommonNoun surfaces — BareLocativeNoun{exile,hand},
ControllerNoun{opponent,player} — the residue ticket deletes them; the
tripwire must then keep them out). Standard constraints apply.
