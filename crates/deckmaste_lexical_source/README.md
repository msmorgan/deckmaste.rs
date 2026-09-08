# Lexical source adaptation

`deckmaste_lexical_source` assembles the current authored vocabulary into
normalized [`deckmaste_lexical::Lexeme`] declarations. Its public interface is
independent of the corpus-reporting CLI and of any English parser.

The implementation currently contains an explicitly transitional adapter for
the declarations being salvaged from English v2. That adapter is an input
quarry, not an interface v3 consumers must implement or preserve. Its internal
readers can be replaced as the remaining declarations become v3-owned.

Source trees are discovered as inventories. Declaration identity comes from
normalized declaration content; callers and tests do not name an individual
macro file.

V3-owned inputs live in `lexicon/`: `core.ron` holds closed-class words,
applicable auxiliary and pronoun bundles, and selected Complement frames;
`overrides.ron` holds replacing morphology under the original lexical owners.
The transitional readers still salvage the existing core/plugin declarations
and catalogs. None of those readers consults corpus success to license forms.
The keyword-action archive is optional evidence, not a dependency.

Frame literals are reconciled through an explicit declaration table to fixed
marker identities. A missing marker, mismatched spelling, duplicate added frame
or unknown owner is a load error. Source paths for paradigm, form, frame and
morphology changes remain attached to the normalized declaration. Grammar owns
how those signatures combine with constituents; a Frame has no independent
surface apart from its lexical owner and the future grammatical construction.

Pronoun Number is grammatical concord, not the number of real-world referents.
For example, `they` retains plural concord with a singular referent. Possessive
features describe the possessor; they do not require the possessed noun to have
the same Number. Modal and contracted-auxiliary forms keep complete correlated
Person, Number, Tense and Finiteness bundles. The missing agreement and Case dimensions in
the salvaged pronoun inventories are replaced by explicit declarations rather
than interpreted as wildcards.

Source gaps remain visible in the lexical report. In particular, compound v2
construction vocabularies are not automatically assigned a part of speech;
raw flavor heads, abbreviated names, uncommon word forms and unhandled notation
remain named input for the whole-grammar activation report. Flavor words have
no enumerated lexical inventory.
