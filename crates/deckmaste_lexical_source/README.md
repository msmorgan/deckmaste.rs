# Lexical source adaptation

`deckmaste_lexical_source` assembles the current authored vocabulary into
normalized [`deckmaste_lexical::Lexeme`] declarations. Its public interface is
independent of the corpus-reporting CLI and of any English parser.

The source vocabulary and verb inventory live in `lexicon/vocabulary.ron` and
`lexicon/verbs.ron`. The vocabulary file retains named classes, member features
and morphology overrides as RON read by serde. Its `provenance_path` preserves the original
declaration identifiers, including the historical `vocabulary.rs` path.
Plugin declarations use the shared `deckmaste_macro_def` metadata reader.

Source trees are discovered as inventories. Declaration identity comes from
normalized declaration content; callers and tests do not name an individual
macro file.

V3-owned inputs live in `lexicon/`: `core.ron` holds closed-class words,
applicable auxiliary and pronoun bundles, and selected Complement frames;
`overrides.ron` holds replacing morphology under the original lexical owners.
The transitional readers still salvage the existing core/plugin declarations
and catalogs. None of those readers consults corpus success to license forms.
The keyword-action archive is optional evidence, not a dependency.

`core.ron` also declares `feature_additions` under existing lexical owners.
These attach grammatical distribution properties and their source paths;
unknown owners, empty declarations and replacement of an existing property
are load errors. `DeterminerUse` states which nominal Number and Countability
combinations a determinative licenses. This is selection data, separate from
its morphological Feature Bundle: invariant words do not acquire invented
inflectional features. Grammar must apply the declared distribution, including
any further constraints on article realization. Scalar variables, measure
operators, measured nominal positions and comparison markers likewise declare
their distributions as lexical data. Subordinator licenses are independent
properties so one owner can participate in several clause roles.

Type Line distribution uses the same feature mechanism. `TypeLineRole` marks
the supported card-type and supertype catalog owners in `core.ron`; subtype
plugin subtype declarations supply the subtype role on their single noun
Lexemes. Type Lines consume those nouns' singular forms. Generated subtype
catalogs remain CR membership authorities without exporting lexical duplicates. The generated grammar consumes those roles for its
ordered groups rather than comparing names or reclassifying ordinary nouns.

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

Generate the canonical catalogs from the local rules and Oracle snapshots with
`cargo xtask catalogs generate` before loading a fresh checkout's lexicon. This
also writes `data/gen/catalogs/card-name-nicknames.json`, containing attested
shortened card-name spellings and the Oracle corpus's size and SHA-256. Missing
or stale nickname catalogs fail with a regeneration command. `cargo xtask
catalogs check` checks this artifact together with the canonical line catalogs.
Feature provisioning shares `data/` with the default checkout, so generation
belongs to the catalog command rather than each workspace's provisioning hook.
Regenerate after replacing the Oracle snapshot.
