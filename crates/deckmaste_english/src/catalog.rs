#![cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "catalog scans are consumed by the staged chart grammar"
    )
)]

use std::array;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use deckmaste_catalogs::legacy::LegacyCatalogKind;
use deckmaste_catalogs::legacy::LegacyCatalogSet;

use crate::word::Adjective;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounUsage;
use crate::word::VERB_SLOTS;
use crate::word::Verb;
use crate::word::VerbInstance;
use crate::word::VerbSlot;
use crate::word::Vocab;
use crate::word::Vocabulary;
use crate::word::WordMatch;
use crate::word::regular_plural;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[repr(u8)]
pub enum CatalogKind {
    KeywordAbility,
    KeywordAction,
    AbilityWord,
    ArtifactType,
    BattleType,
    CreatureType,
    EnchantmentType,
    LandType,
    PlaneswalkerType,
    SpellType,
    Supertype,
    CardType,
    /// A hand-curated rules-defined word whose membership or meaning the
    /// Comprehensive Rules fix: the collective shorthands (`historic`,
    /// `modified`, `party`, `outlaw`) and the measured value nominal
    /// (`devotion`). Unlike the Scryfall-derived kinds this one is never
    /// populated externally through [`Catalogs::with_catalog`]; its atoms come
    /// from the hand-curated [`RulesNominal`] table.
    RulesBundle,
    /// A Scryfall flavor word: an italicized, rules-inert label a card prints
    /// before an em dash to name an ability for flavor (`Polymorphine`,
    /// `Chaos`, `Sanctified Rules of Combat`). Members are matched byte-exact
    /// ([`CasePolicy::Exact`]) so ordinary sentence-initial vocabulary never
    /// collides. Populated externally from `data/catalogs/flavor-words.json`;
    /// used only to license a header peel (see
    /// [`Catalogs::is_flavor_word`]), never as a noun or adjective.
    FlavorWord,
}

impl CatalogKind {
    const ALL: [Self; 14] = [
        Self::KeywordAbility,
        Self::KeywordAction,
        Self::AbilityWord,
        Self::ArtifactType,
        Self::BattleType,
        Self::CreatureType,
        Self::EnchantmentType,
        Self::LandType,
        Self::PlaneswalkerType,
        Self::SpellType,
        Self::Supertype,
        Self::CardType,
        Self::RulesBundle,
        Self::FlavorWord,
    ];
    const COUNT: usize = Self::ALL.len();

    const fn index(self) -> usize {
        self as usize
    }

    const fn case_policy(self) -> CasePolicy {
        match self {
            Self::ArtifactType
            | Self::BattleType
            | Self::CreatureType
            | Self::EnchantmentType
            | Self::LandType
            | Self::PlaneswalkerType
            | Self::SpellType
            | Self::FlavorWord => CasePolicy::Exact,
            Self::Supertype | Self::CardType | Self::RulesBundle => CasePolicy::Lowercase,
            Self::KeywordAbility | Self::KeywordAction | Self::AbilityWord => {
                CasePolicy::Insensitive
            }
        }
    }

    const fn is_noun(self) -> bool {
        matches!(
            self,
            Self::ArtifactType
                | Self::BattleType
                | Self::CreatureType
                | Self::EnchantmentType
                | Self::LandType
                | Self::PlaneswalkerType
                | Self::SpellType
                | Self::CardType
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct CatalogAtom {
    pub kind: CatalogKind,
    canonical: Arc<str>,
    spelling: Arc<str>,
    pub vocab: Option<Vocab>,
}

impl CatalogAtom {
    #[must_use]
    pub(crate) const fn is_keyword_ability(&self) -> bool {
        matches!(self.kind, CatalogKind::KeywordAbility)
    }

    #[must_use]
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    #[must_use]
    pub fn render_noun(&self, plural: bool) -> String {
        let linked = self.vocab.and_then(|vocab| {
            let noun = if plural {
                NounInstance::unchecked_plural(Noun::Word(vocab))
            } else {
                NounInstance::unchecked_singular(Noun::Word(vocab))
            };
            Vocabulary::new().render_noun(&noun)
        });
        let rendered = linked.unwrap_or_else(|| {
            if plural {
                regular_plural(&self.canonical)
            } else {
                self.canonical.to_string()
            }
        });

        match self.kind {
            CatalogKind::KeywordAbility => rendered.to_ascii_lowercase(),
            _ => match self.kind.case_policy() {
                CasePolicy::Lowercase => rendered.to_ascii_lowercase(),
                CasePolicy::Exact | CasePolicy::Insensitive => {
                    apply_initial_case(&rendered, &self.canonical)
                }
            },
        }
    }

    /// Whether [`Self::render_noun`] lowercases this atom's canonical spelling,
    /// so reading a capitalized token as this noun corrupts its case. Mirrors
    /// the lowercasing branches of `render_noun`: keyword abilities always
    /// lowercase, and every [`CasePolicy::Lowercase`] kind (supertypes,
    /// card types) does too.
    #[must_use]
    pub fn renders_lowercase_noun(&self) -> bool {
        matches!(self.kind, CatalogKind::KeywordAbility)
            || matches!(self.kind.case_policy(), CasePolicy::Lowercase)
    }

    /// Whether this atom is a hand-curated rules-defined word (see
    /// [`RulesNominal`]). Such words hyphenate under `non-` as a category — the
    /// render side derives the glyph from this instead of a per-word flag.
    #[must_use]
    pub(crate) fn is_rules_bundle(&self) -> bool {
        matches!(self.kind, CatalogKind::RulesBundle)
    }

    #[must_use]
    pub fn render_adjective(&self) -> String {
        match self.kind.case_policy() {
            CasePolicy::Lowercase => self.canonical.to_ascii_lowercase(),
            CasePolicy::Exact | CasePolicy::Insensitive => self.canonical.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct KeywordAction {
    canonical: Arc<str>,
    head: KeywordActionHead,
    tail: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
enum KeywordActionHead {
    Regular(Arc<str>),
    Irregular(Vocab),
}

impl KeywordAction {
    #[must_use]
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    #[must_use]
    pub fn tail(&self) -> &str {
        &self.tail
    }

    #[must_use]
    pub fn head(&self) -> &str {
        match &self.head {
            KeywordActionHead::Regular(lemma) => lemma,
            KeywordActionHead::Irregular(vocab) => vocab.spelling(),
        }
    }

    #[must_use]
    pub fn irregular_head(&self) -> Option<Vocab> {
        match &self.head {
            KeywordActionHead::Regular(_) => None,
            KeywordActionHead::Irregular(vocab) => Some(*vocab),
        }
    }

    pub(crate) fn render(&self, slot: VerbSlot) -> Option<String> {
        let head = self.render_head(slot)?;
        if self.tail.is_empty() {
            Some(head)
        } else {
            Some(format!("{head} {}", self.tail))
        }
    }

    fn render_head(&self, slot: VerbSlot) -> Option<String> {
        match &self.head {
            KeywordActionHead::Regular(lemma) => Some(Vocabulary::render_regular_verb(lemma, slot)),
            KeywordActionHead::Irregular(vocab) => Vocabulary::new().render_verb(*vocab, slot),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
struct CatalogId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum CatalogSlot {
    AbilityItem,
    KeywordAbilityNoun,
    AbilityWord,
    Noun(NounUsage),
    Adjective,
    Verb(VerbSlot),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct CatalogMatch {
    pub(crate) length: usize,
    pub(crate) value: CatalogValue,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) enum CatalogValue {
    Atom(CatalogAtom),
    Word(WordMatch),
}

impl CatalogValue {
    fn atom(&self) -> Option<&CatalogAtom> {
        match self {
            Self::Atom(atom) => Some(atom),
            Self::Word(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
enum CasePolicy {
    Exact,
    Lowercase,
    Insensitive,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
struct CatalogIndex {
    by_surface: HashMap<String, Vec<CatalogId>>,
    lengths: Vec<usize>,
}

impl CatalogIndex {
    fn insert(&mut self, surface: &str, policy: CasePolicy, id: CatalogId) {
        let key = normalize_key(surface, policy);
        let ids = self.by_surface.entry(key.into_owned()).or_default();
        if !ids.contains(&id) {
            ids.push(id);
        }
        if !self.lengths.contains(&surface.len()) {
            self.lengths.push(surface.len());
        }
    }

    fn finish(&mut self) {
        self.lengths.sort_unstable();
    }

    fn prefix_matches(&self, text: &str, policy: CasePolicy) -> Vec<(usize, CatalogId)> {
        let mut matches = Vec::new();
        for &length in &self.lengths {
            let Some(prefix) = text.get(..length) else {
                continue;
            };
            if !is_word_boundary(text, length)
                || policy == CasePolicy::Lowercase
                    && prefix.bytes().any(|byte| byte.is_ascii_uppercase())
            {
                continue;
            }
            let key = normalize_key(prefix, policy);
            if let Some(ids) = self.by_surface.get(key.as_ref()) {
                matches.extend(ids.iter().map(|&id| (length, id)));
            }
        }
        matches
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Catalogs {
    sources: [Vec<Arc<str>>; CatalogKind::COUNT],
    entries: Vec<CatalogAtom>,
    indexes: [CatalogIndex; CatalogKind::COUNT],
    actions_by_head_surface: HashMap<String, Vec<KeywordAction>>,
}

impl Default for Catalogs {
    fn default() -> Self {
        Self {
            sources: array::from_fn(|_| Vec::new()),
            entries: Vec::new(),
            indexes: array::from_fn(|_| CatalogIndex::default()),
            actions_by_head_surface: HashMap::new(),
        }
    }
}

impl Catalogs {
    /// Adapts the shared legacy inventory to the grammar's catalog roles.
    ///
    /// The shared crate owns only raw catalog data. This explicit mapping
    /// keeps grammar-specific case, morphology, rule-bundle, and indexing
    /// policy in this crate.
    #[must_use]
    pub fn from_legacy(legacy: &LegacyCatalogSet) -> Self {
        Self::default()
            .with_catalog(
                CatalogKind::AbilityWord,
                legacy.get(LegacyCatalogKind::AbilityWords),
            )
            .with_catalog(
                CatalogKind::ArtifactType,
                legacy.get(LegacyCatalogKind::ArtifactTypes),
            )
            .with_catalog(
                CatalogKind::BattleType,
                legacy.get(LegacyCatalogKind::BattleTypes),
            )
            .with_catalog(
                CatalogKind::CardType,
                legacy.get(LegacyCatalogKind::CardTypes),
            )
            .with_catalog(
                CatalogKind::CreatureType,
                legacy.get(LegacyCatalogKind::CreatureTypes),
            )
            .with_catalog(
                CatalogKind::EnchantmentType,
                legacy.get(LegacyCatalogKind::EnchantmentTypes),
            )
            .with_catalog(
                CatalogKind::KeywordAbility,
                legacy.get(LegacyCatalogKind::KeywordAbilities),
            )
            .with_catalog(
                CatalogKind::KeywordAction,
                legacy.get(LegacyCatalogKind::KeywordActions),
            )
            .with_catalog(
                CatalogKind::LandType,
                legacy.get(LegacyCatalogKind::LandTypes),
            )
            .with_catalog(
                CatalogKind::PlaneswalkerType,
                legacy.get(LegacyCatalogKind::PlaneswalkerTypes),
            )
            .with_catalog(
                CatalogKind::SpellType,
                legacy.get(LegacyCatalogKind::SpellTypes),
            )
            .with_catalog(
                CatalogKind::Supertype,
                legacy.get(LegacyCatalogKind::Supertypes),
            )
            .with_catalog(
                CatalogKind::FlavorWord,
                legacy.get(LegacyCatalogKind::FlavorWords),
            )
    }

    #[must_use]
    pub fn new(
        keyword_abilities: impl IntoIterator<Item = impl Into<String>>,
        keyword_actions: impl IntoIterator<Item = impl Into<String>>,
        ability_words: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::default()
            .with_catalog(CatalogKind::KeywordAbility, keyword_abilities)
            .with_catalog(CatalogKind::KeywordAction, keyword_actions)
            .with_catalog(CatalogKind::AbilityWord, ability_words)
    }

    #[must_use]
    pub fn with_catalog(
        mut self,
        kind: CatalogKind,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let policy = kind.case_policy();
        let mut seen = HashSet::new();
        self.sources[kind.index()] = values
            .into_iter()
            .map(Into::into)
            .filter(|value| seen.insert(normalize_key(value, policy).into_owned()))
            .map(Arc::<str>::from)
            .collect();
        self.rebuild();
        self
    }

    pub(crate) fn matches(&self, text: &str, slot: CatalogSlot) -> Vec<CatalogMatch> {
        let mut matches = match slot {
            CatalogSlot::AbilityItem | CatalogSlot::KeywordAbilityNoun => {
                self.atom_matches(text, CatalogKind::KeywordAbility)
            }
            CatalogSlot::AbilityWord => self.atom_matches(text, CatalogKind::AbilityWord),
            CatalogSlot::Noun(NounUsage::Mass) => Vec::new(),
            CatalogSlot::Noun(NounUsage::Count | NounUsage::Either) => self.noun_matches(text),
            CatalogSlot::Adjective => {
                self.grammatical_atom_matches(text, CatalogKind::Supertype, |atom| {
                    WordMatch::Adjective(Adjective::Catalog(atom))
                })
            }
            CatalogSlot::Verb(slot) => self.action_matches(text, slot),
        };
        matches.extend(bundle_matches(text, slot));
        matches.sort_by_key(|catalog_match| catalog_match.length);
        matches
    }

    /// Whether `text` is a byte-exact member of the flavor-word catalog
    /// ([`CatalogKind::FlavorWord`]). Licenses a flavor-word header peel: the
    /// whole label before a spaced em dash must match a member exactly (case
    /// and all), so ordinary sentence-initial vocabulary is never mistaken for
    /// a flavor label. A shorter member that is only a prefix of `text` does
    /// not qualify — the match length must span the entire label.
    pub(crate) fn is_flavor_word(&self, text: &str) -> bool {
        self.atom_matches(text, CatalogKind::FlavorWord)
            .into_iter()
            .any(|catalog_match| catalog_match.length == text.len())
    }

    fn rebuild(&mut self) {
        let mut entries = Vec::new();
        let mut indexes: [CatalogIndex; CatalogKind::COUNT] =
            array::from_fn(|_| CatalogIndex::default());

        for kind in CatalogKind::ALL {
            for canonical in &self.sources[kind.index()] {
                let vocab = kind
                    .is_noun()
                    .then(|| Vocabulary::exceptional_catalog_noun(canonical))
                    .flatten();
                let atom = CatalogAtom {
                    kind,
                    canonical: Arc::clone(canonical),
                    spelling: Arc::clone(canonical),
                    vocab,
                };
                let id = CatalogId(entries.len());
                let index = &mut indexes[kind.index()];
                if kind.is_noun() {
                    index.insert(&atom.render_noun(false), kind.case_policy(), id);
                    index.insert(&atom.render_noun(true), kind.case_policy(), id);
                } else if kind != CatalogKind::KeywordAction {
                    let surface = if kind == CatalogKind::Supertype {
                        atom.render_adjective()
                    } else {
                        atom.canonical().to_owned()
                    };
                    index.insert(&surface, kind.case_policy(), id);
                }
                entries.push(atom);
            }
        }

        // DURABILITY: `Bands with other` is [CR#702.22b]'s own quoted name for
        // a special form of banding; its `[quality]` argument is an ordinary
        // noun phrase [CR#702.22c] (`bands with other legendary creatures`,
        // `bands with other creatures named Wolves of the Hunt`). Neither
        // `CatalogSet::from_cr` nor Scryfall keyword extraction can produce
        // this entry: `from_cr` mines only `702.NN.` CR headings
        // (`Banding` is [CR#702.22]'s heading; "bands with other" is
        // prose inside [CR#702.22b,702.22c], not a heading of its own),
        // and Scryfall extraction mines only the `keywords` tags on
        // real cards — no card, Vintage-legal or otherwise, carries
        // `"Bands with other"` as a raw `keywords` entry (checked
        // directly against the pinned Oracle Cards snapshot: every
        // witness embeds it only inside quoted ability prose), so there
        // is nothing for that mining loop to authorize. Added here,
        // permanently, independent of any catalog refresh — the same
        // shape and rationale as `ABILITY_DERIVED_KEYWORD_ACTION_VERBS`
        // below (`mutate`), just for the keyword-ability catalog
        // instead of keyword-action verbs.
        for canonical in HAND_CURATED_KEYWORD_ABILITY_SURFACES {
            let canonical: Arc<str> = Arc::from(canonical);
            let atom = CatalogAtom {
                kind: CatalogKind::KeywordAbility,
                canonical: Arc::clone(&canonical),
                spelling: canonical,
                vocab: None,
            };
            let id = CatalogId(entries.len());
            indexes[CatalogKind::KeywordAbility.index()].insert(
                atom.canonical(),
                CatalogKind::KeywordAbility.case_policy(),
                id,
            );
            entries.push(atom);
        }

        for index in &mut indexes {
            index.finish();
        }

        let mut actions_by_head_surface: HashMap<String, Vec<KeywordAction>> = HashMap::new();
        let generated_actions = self.sources[CatalogKind::KeywordAction.index()]
            .iter()
            .cloned();
        // DURABILITY: `mutate` (and, pending a fix, `exploit` — see below) is
        // a keyword *ability* [CR#702] whose CR-defined verb usage (`this
        // creature mutates`) Scryfall's `keyword-actions` catalog will never
        // enumerate, because the Comprehensive Rules classify it as an
        // ability, not a keyword action — so it can never appear in the
        // CR-§701-derived generated catalog above. This hand-curated
        // supplement (the same shape as `RulesNominal`'s hand-curated table)
        // is where its ability-derived verb form is added instead. A future
        // regeneration of `keyword-actions` from the CR or from Scryfall
        // must NOT be read as license to drop this entry — it is added
        // here, permanently, independent of any catalog refresh.
        //
        // `exploit` was attempted alongside `mutate` (round `kwverbs`) but
        // reverted: it creates a competing verb reading for the
        // keyword-ability atom `exploit` as a bare object (`have exploit`)
        // on coordinated-subject faces (Henry Wu, InGen Geneticist —
        // `Henry Wu and other Human creatures you control have exploit.`
        // parses `exploit` as a second coordinated intransitive predicate
        // instead of the atom filling `have`'s object slot). See ticket
        // `english-ability-derived-verb-batch` before re-attempting; it
        // needs a fix for that over-fire, not just a re-add.
        let hand_curated_ability_derived_actions = ABILITY_DERIVED_KEYWORD_ACTION_VERBS
            .iter()
            .map(|lemma| Arc::<str>::from(*lemma));
        for canonical in generated_actions.chain(hand_curated_ability_derived_actions) {
            let action = resolve_keyword_action(&canonical);
            for slot in VERB_SLOTS {
                let Some(surface) = action.render_head(slot) else {
                    continue;
                };
                let candidates = actions_by_head_surface
                    .entry(surface.to_ascii_lowercase())
                    .or_default();
                if !candidates.contains(&action) {
                    candidates.push(action.clone());
                }
            }
        }
        for actions in actions_by_head_surface.values_mut() {
            actions.sort_by_key(|action| action.canonical.len());
        }

        self.entries = entries;
        self.indexes = indexes;
        self.actions_by_head_surface = actions_by_head_surface;
    }

    fn atom_matches(&self, text: &str, kind: CatalogKind) -> Vec<CatalogMatch> {
        self.indexes[kind.index()]
            .prefix_matches(text, kind.case_policy())
            .into_iter()
            .map(|(length, id)| {
                let mut atom = self.entries[id.0].clone();
                atom.spelling = Arc::from(&text[..length]);
                CatalogMatch {
                    length,
                    value: CatalogValue::Atom(atom),
                }
            })
            .collect()
    }

    fn grammatical_atom_matches(
        &self,
        text: &str,
        kind: CatalogKind,
        word: impl Fn(CatalogAtom) -> WordMatch,
    ) -> Vec<CatalogMatch> {
        self.indexes[kind.index()]
            .prefix_matches(text, kind.case_policy())
            .into_iter()
            .map(|(length, id)| CatalogMatch {
                length,
                value: CatalogValue::Word(word(self.entries[id.0].clone())),
            })
            .collect()
    }

    fn noun_matches(&self, text: &str) -> Vec<CatalogMatch> {
        const NOUN_KINDS: [CatalogKind; 8] = [
            CatalogKind::ArtifactType,
            CatalogKind::BattleType,
            CatalogKind::CreatureType,
            CatalogKind::EnchantmentType,
            CatalogKind::LandType,
            CatalogKind::PlaneswalkerType,
            CatalogKind::SpellType,
            CatalogKind::CardType,
        ];

        let mut matches = Vec::new();
        for kind in NOUN_KINDS {
            for (length, id) in self.indexes[kind.index()].prefix_matches(text, kind.case_policy())
            {
                let atom = self.entries[id.0].clone();
                let surface = &text[..length];
                if surface_equals(surface, &atom.render_noun(false), kind.case_policy()) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Noun(
                            NounInstance::unchecked_singular(Noun::Catalog(atom.clone())),
                        )),
                    });
                }
                if surface_equals(surface, &atom.render_noun(true), kind.case_policy()) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Noun(NounInstance::unchecked_plural(
                            Noun::Catalog(atom),
                        ))),
                    });
                }
            }
        }
        matches
    }

    fn action_matches(&self, text: &str, slot: VerbSlot) -> Vec<CatalogMatch> {
        let head_surface = text
            .split_once(|character: char| !is_word_character(character))
            .map_or(text, |(head, _)| head);
        let mut matches = Vec::new();

        let key = head_surface.to_ascii_lowercase();
        let Some(actions) = self.actions_by_head_surface.get(&key) else {
            return matches;
        };
        for action in actions {
            let rendered = action
                .render(slot)
                .expect("indexed keyword-action head must render");
            if prefix_equals(text, &rendered, CasePolicy::Exact) {
                let mut action = action.clone();
                let verb_match = &text[..rendered.len()];
                action.tail = Arc::from(
                    verb_match
                        .split_once(' ')
                        .map_or("", |(_, matched_tail)| matched_tail),
                );
                matches.push(CatalogMatch {
                    length: rendered.len(),
                    value: CatalogValue::Word(WordMatch::Verb(VerbInstance {
                        verb: Verb::KeywordAction(action),
                        slot,
                    })),
                });
            }
        }
        matches
    }
}

/// A hand-curated rules-defined word whose membership or meaning the
/// Comprehensive Rules fix. This is the single source for the such words the
/// parser recognizes; each entry fixes the word's spelling, the grammatical
/// slots it fills on the supported corpus, and the CR rule that defines it. The
/// words share [`CatalogKind::RulesBundle`] (lowercase, no external population)
/// and reach the AST through the ordinary [`Adjective::Catalog`] and
/// [`Noun::Catalog`] paths.
///
/// Two kinds live here. The *collective shorthands* (`historic`, `modified`,
/// `party`, `outlaw`) name sets of objects and need no dedicated syntax. The
/// *value nominal* `devotion` is different in kind — a measured value with a
/// mandatory `to <color>` argument — and additionally fills the value-noun slot
/// ([`Self::fills_value_noun`]) the grammar reads to gate its bare-color
/// production.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum RulesNominal {
    /// `historic` — an object with the legendary supertype, the artifact card
    /// type, or the Saga subtype [CR#700.6]. Attributive adjective only
    /// (`historic spell`, `historic permanent`); never a head noun and never
    /// attested with `non-`.
    Historic,
    /// `modified` — a permanent with a counter on it, that is equipped, or
    /// that is enchanted by an Aura its controller controls [CR#700.9].
    /// Adjective,
    /// used attributively (`modified creature`) and predicatively
    /// (`as long as it's modified`); never attested with `non-`.
    Modified,
    /// `party` — up to one each of Cleric, Rogue, Warrior, and Wizard among the
    /// creatures a player controls [CR#700.8]. Count noun (`creatures in your
    /// party`, `a full party`); never attributive and never attested with
    /// `non-`.
    Party,
    /// `outlaw` — an object with the Assassin, Mercenary, Pirate, Rogue, and/or
    /// Warlock creature types [CR#700.12]. Both a count noun (`outlaws you
    /// control`, `an outlaw`) and an attributive adjective (`outlaw creature`,
    /// `outlaw spell`). Its `non-` form is the sole attested witness and
    /// hyphenates (`non-outlaw`, Shoot the Sheriff).
    Outlaw,
    /// `devotion` — a measured value equal to the number of mana symbols of the
    /// named color(s) among the mana costs of permanents a player controls
    /// [CR#700.5]. The value nominal takes a mandatory `to <color>` argument; a
    /// dedicated grammar production supplies the bare-color and color-pair
    /// arguments (`devotion to green`, `devotion to white and black`), while
    /// the generic `devotion to a/each/that color` rides the ordinary
    /// prepositional path. Also fills the count-noun slot for its unrelated
    /// counter-kind sense (`devotion counter`, Pious Kitsune); never
    /// attributive.
    Devotion,
}

impl RulesNominal {
    const ALL: [Self; 5] = [
        Self::Historic,
        Self::Modified,
        Self::Party,
        Self::Outlaw,
        Self::Devotion,
    ];

    const fn spelling(self) -> &'static str {
        match self {
            Self::Historic => "historic",
            Self::Modified => "modified",
            Self::Party => "party",
            Self::Outlaw => "outlaw",
            Self::Devotion => "devotion",
        }
    }

    /// Whether this word fills the attributive/predicative adjective slot.
    const fn fills_adjective(self) -> bool {
        matches!(self, Self::Historic | Self::Modified | Self::Outlaw)
    }

    /// Whether this word fills the count-noun slot.
    const fn fills_count_noun(self) -> bool {
        matches!(self, Self::Party | Self::Outlaw | Self::Devotion)
    }

    /// Whether this word is a rules-defined value nominal taking a mandatory
    /// `to <color>` argument. The grammar reads this to gate its bare-color
    /// devotion production so no other word reaches it.
    const fn fills_value_noun(self) -> bool {
        matches!(self, Self::Devotion)
    }

    pub(crate) fn atom(self) -> CatalogAtom {
        let canonical: Arc<str> = Arc::from(self.spelling());
        CatalogAtom {
            kind: CatalogKind::RulesBundle,
            canonical: Arc::clone(&canonical),
            spelling: canonical,
            vocab: None,
        }
    }
}

/// The value-noun atom (currently `devotion`) beginning at the head of `text`,
/// matched as a lowercase whole word, with the matched length. The grammar's
/// value-noun lexical slot calls this to gate the bare-color devotion
/// production to exactly the rules-defined value nominals.
pub(crate) fn rules_value_noun_prefix(text: &str) -> Option<(usize, CatalogAtom)> {
    RulesNominal::ALL.into_iter().find_map(|word| {
        word.fills_value_noun()
            .then(|| {
                lowercase_word_prefix(text, word.spelling()).map(|length| (length, word.atom()))
            })
            .flatten()
    })
}

/// Scans the fixed [`RulesNominal`] table for matches in the requested slot.
/// These words are lowercase (matched exactly, like every
/// [`CasePolicy::Lowercase`] atom; a sentence-initial capital is handled by the
/// grammar's lowercased retry) and reach the AST as [`Adjective::Catalog`] /
/// [`Noun::Catalog`].
fn bundle_matches(text: &str, slot: CatalogSlot) -> Vec<CatalogMatch> {
    let mut matches = Vec::new();
    for bundle in RulesNominal::ALL {
        match slot {
            CatalogSlot::Adjective if bundle.fills_adjective() => {
                if let Some(length) = lowercase_word_prefix(text, bundle.spelling()) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Adjective(Adjective::Catalog(
                            bundle.atom(),
                        ))),
                    });
                }
            }
            CatalogSlot::Noun(NounUsage::Count | NounUsage::Either)
                if bundle.fills_count_noun() =>
            {
                let atom = bundle.atom();
                if let Some(length) = lowercase_word_prefix(text, &atom.render_noun(false)) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Noun(
                            NounInstance::unchecked_singular(Noun::Catalog(atom.clone())),
                        )),
                    });
                }
                if let Some(length) = lowercase_word_prefix(text, &atom.render_noun(true)) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Noun(NounInstance::unchecked_plural(
                            Noun::Catalog(atom),
                        ))),
                    });
                }
            }
            _ => {}
        }
    }
    matches
}

/// The length of an exact lowercase `expected` prefix of `text` ending at a
/// word boundary, or `None`. `expected` is already lowercase, so a capitalized
/// surface never matches here.
fn lowercase_word_prefix(text: &str, expected: &str) -> Option<usize> {
    let prefix = text.get(..expected.len())?;
    (prefix == expected && is_word_boundary(text, expected.len())).then_some(expected.len())
}

/// Ability-derived keyword-action verb forms, hand-curated rather than
/// catalog-generated. `mutate` is a keyword *ability* [CR#702], not a
/// keyword action [CR#701] — the Comprehensive Rules define its verb usage
/// (`this creature mutates`) but Scryfall's `keyword-actions` catalog will
/// never list it, because it is not a keyword action. See the durability
/// comment at the merge site in [`Catalogs::rebuild`]. (`exploit` was
/// attempted here too and reverted for an over-fire on `have exploit`; see
/// that comment and ticket `english-ability-derived-verb-batch`.)
const ABILITY_DERIVED_KEYWORD_ACTION_VERBS: [&str; 1] = ["mutate"];

/// Keyword-ability catalog surfaces hand-curated rather than catalog-
/// generated, for the same reason `ABILITY_DERIVED_KEYWORD_ACTION_VERBS`
/// above is: the Comprehensive Rules define the surface, but neither
/// generation path (`CatalogSet::from_cr`'s `702.NN.` heading scan, nor
/// Scryfall `keywords`-field mining) can produce it.
/// See the durability comment at the merge site in [`Catalogs::rebuild`].
const HAND_CURATED_KEYWORD_ABILITY_SURFACES: [&str; 1] = ["Bands with other"];

/// The closed selector-label set written by
/// [CR#702.174a,702.174d,702.174e,702.174f,702.174g,702.174h,702.174i].
const NAMED_KEYWORD_ARGUMENT_LABELS: [&str; 6] = [
    "a Food",
    "a card",
    "a tapped Fish",
    "an extra turn",
    "a Treasure",
    "an Octopus",
];

/// Whether `text` is exactly one complete rules-defined selector label.
pub(crate) fn is_named_keyword_argument_label(text: &str) -> bool {
    NAMED_KEYWORD_ARGUMENT_LABELS.contains(&text)
}

fn resolve_keyword_action(canonical: &Arc<str>) -> KeywordAction {
    let (head_surface, tail) = canonical
        .split_once(' ')
        .map_or((canonical.as_ref(), ""), |(head, tail)| (head, tail));
    let head = Vocabulary::irregular_keyword_action_head(head_surface).map_or_else(
        || KeywordActionHead::Regular(Arc::from(head_surface.to_ascii_lowercase())),
        KeywordActionHead::Irregular,
    );
    KeywordAction {
        canonical: Arc::clone(canonical),
        head,
        // The catalog spells canonicals in title case (`Time Travel`,
        // `Venture into the Dungeon`); oracle text spells the action in
        // running-text case. The head is already lowercased through the lemma
        // above; the tail is lowercased for exactly the same reason. Without
        // it the rendered form (`time Travel`) is byte-exact-matched
        // (`action_matches`, catalog.rs:587) against a surface no card prints.
        tail: Arc::from(tail.to_ascii_lowercase()),
    }
}

fn normalize_key(surface: &str, policy: CasePolicy) -> Cow<'_, str> {
    match policy {
        CasePolicy::Lowercase | CasePolicy::Insensitive
            if surface.bytes().any(|byte| byte.is_ascii_uppercase()) =>
        {
            Cow::Owned(surface.to_ascii_lowercase())
        }
        CasePolicy::Exact | CasePolicy::Lowercase | CasePolicy::Insensitive => {
            Cow::Borrowed(surface)
        }
    }
}

fn prefix_equals(text: &str, expected: &str, policy: CasePolicy) -> bool {
    text.get(..expected.len()).is_some_and(|prefix| {
        surface_equals(prefix, expected, policy) && is_word_boundary(text, expected.len())
    })
}

fn surface_equals(left: &str, right: &str, policy: CasePolicy) -> bool {
    match policy {
        CasePolicy::Exact => left == right,
        CasePolicy::Lowercase => {
            !left.bytes().any(|byte| byte.is_ascii_uppercase()) && left.eq_ignore_ascii_case(right)
        }
        CasePolicy::Insensitive => left.eq_ignore_ascii_case(right),
    }
}

fn is_word_boundary(text: &str, end: usize) -> bool {
    text.get(end..)
        .and_then(|suffix| suffix.chars().next())
        .is_none_or(|character| !is_word_character(character))
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '-' | '\'')
}

fn apply_initial_case(surface: &str, canonical: &str) -> String {
    if canonical.chars().next().is_some_and(char::is_uppercase)
        && surface.chars().next().is_some_and(char::is_lowercase)
    {
        let mut characters = surface.chars();
        let first = characters
            .next()
            .expect("checked nonempty catalog-linked surface");
        first.to_uppercase().chain(characters).collect()
    } else {
        surface.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use deckmaste_catalogs::legacy::LegacyCatalogKind;
    use deckmaste_catalogs::legacy::LegacyCatalogSet;

    use super::*;
    use crate::word::Adjective;
    use crate::word::LexicalSlot;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::NounUsage;
    use crate::word::Number;
    use crate::word::Person;
    use crate::word::Verb;
    use crate::word::VerbSlot;
    use crate::word::Vocab;
    use crate::word::WordMatch;

    const THIRD_SINGULAR_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    };

    #[test]
    fn named_keyword_argument_labels_match_exact_whole_strings() {
        for label in NAMED_KEYWORD_ARGUMENT_LABELS {
            assert!(is_named_keyword_argument_label(label), "{label:?}");
        }
        for nonmember in [
            "a creature",
            "each color",
            "the Trolls",
            "a food",
            "a Food.",
            "a Foo",
            "Gift a Food",
            " a Food",
            "a Food ",
        ] {
            assert!(!is_named_keyword_argument_label(nonmember), "{nonmember:?}");
        }
    }

    #[test]
    fn keyword_ability_identity_is_shared_without_a_fake_part_of_speech() {
        let catalogs = Catalogs::new(
            ["Flying", "Deathtouch"],
            std::iter::empty::<&str>(),
            ["Landfall"],
        );

        for surface in ["Flying", "flying"] {
            let ability = catalogs.matches(surface, CatalogSlot::AbilityItem);
            let noun_like = catalogs.matches(surface, CatalogSlot::KeywordAbilityNoun);
            assert_eq!(ability, noun_like);
            assert_eq!(ability.len(), 1);
            assert_eq!(ability[0].length, surface.len());
            assert_eq!(ability[0].value.atom().unwrap().canonical(), "Flying");
            assert_eq!(
                ability[0].value.atom().unwrap().kind,
                CatalogKind::KeywordAbility
            );
        }

        assert!(
            catalogs
                .matches("flying", CatalogSlot::Adjective)
                .is_empty()
        );
        assert_eq!(
            catalogs.matches("deathtouch", CatalogSlot::AbilityItem)[0]
                .value
                .atom()
                .unwrap()
                .canonical(),
            "Deathtouch"
        );
        assert_eq!(
            catalogs.matches("Landfall", CatalogSlot::AbilityWord)[0]
                .value
                .atom()
                .unwrap()
                .kind,
            CatalogKind::AbilityWord
        );
        assert!(
            catalogs
                .matches("Landfall", CatalogSlot::AbilityItem)
                .is_empty()
        );
    }

    #[test]
    fn catalog_nouns_apply_kind_specific_case_and_declension() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Hero", "Merfolk"])
            .with_catalog(CatalogKind::CardType, ["Creature", "Hero"])
            .with_catalog(CatalogKind::Supertype, ["Legendary"]);

        let goblin = one_word_match(&catalogs, "Goblin", CatalogSlot::Noun(NounUsage::Count));
        let WordMatch::Noun(goblin) = goblin else {
            panic!("expected a singular catalog noun, got {goblin:?}");
        };
        let crate::word::NounInstanceKind::Singular(Noun::Catalog(goblin_atom)) = goblin.kind()
        else {
            panic!("expected a singular catalog noun, got {goblin:?}");
        };
        assert_eq!(goblin_atom.kind, CatalogKind::CreatureType);
        assert_eq!(goblin_atom.vocab, None);
        assert_eq!(goblin_atom.render_noun(false), "Goblin");
        assert_eq!(goblin_atom.render_noun(true), "Goblins");
        assert_eq!(
            crate::word::Vocabulary::new().render_noun(&NounInstance::unchecked_plural(
                Noun::Catalog(goblin_atom.clone()),
            )),
            Some("Goblins".to_owned())
        );
        assert!(
            catalogs
                .matches("goblin", CatalogSlot::Noun(NounUsage::Count))
                .is_empty()
        );

        let plural = one_word_match(&catalogs, "Goblins", CatalogSlot::Noun(NounUsage::Count));
        assert!(matches!(
            plural,
            WordMatch::Noun(noun)
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Plural(Noun::Catalog(_))
                )
        ));

        let merfolk_matches = catalogs.matches("Merfolk", CatalogSlot::Noun(NounUsage::Count));
        assert!(merfolk_matches.iter().any(|catalog_match| matches!(
            &catalog_match.value,
            CatalogValue::Word(WordMatch::Noun(noun))
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Plural(Noun::Catalog(atom))
                        if atom.kind == CatalogKind::CreatureType
                    && atom.vocab == Some(Vocab::Merfolk)
                    && atom.render_noun(true) == "Merfolk"
                )
        )));

        let hero = one_word_match(&catalogs, "Hero", CatalogSlot::Noun(NounUsage::Count));
        let WordMatch::Noun(hero) = hero else {
            panic!("expected Hero to be a subtype");
        };
        let crate::word::NounInstanceKind::Singular(Noun::Catalog(hero)) = hero.kind() else {
            panic!("expected Hero to be a subtype");
        };
        assert_eq!(hero.kind, CatalogKind::CreatureType);

        let creature = one_word_match(&catalogs, "creature", CatalogSlot::Noun(NounUsage::Count));
        let WordMatch::Noun(creature) = creature else {
            panic!("expected creature to be a card type");
        };
        let crate::word::NounInstanceKind::Singular(Noun::Catalog(creature)) = creature.kind()
        else {
            panic!("expected creature to be a card type");
        };
        assert_eq!(creature.kind, CatalogKind::CardType);
        assert!(
            catalogs
                .matches("Creature", CatalogSlot::Noun(NounUsage::Count))
                .is_empty()
        );

        let legendary = one_word_match(&catalogs, "legendary", CatalogSlot::Adjective);
        assert!(matches!(
            legendary,
            WordMatch::Adjective(Adjective::Catalog(atom))
                if atom.kind == CatalogKind::Supertype
                    && atom.render_adjective() == "legendary"
        ));
        assert!(
            catalogs
                .matches("Legendary", CatalogSlot::Adjective)
                .is_empty()
        );
    }

    #[test]
    fn creature_subtypes_link_vocab_only_for_exceptional_declensions() {
        let exceptional = [
            ("Aetherborn", "Aetherborn"),
            ("Astartes", "Astartes"),
            ("Aurochs", "Aurochs"),
            ("Bison", "Bison"),
            ("Child", "Children"),
            ("Custodes", "Custodes"),
            ("Cyberman", "Cybermen"),
            ("Drix", "Drix"),
            ("Dwarf", "Dwarves"),
            ("Elf", "Elves"),
            ("Elk", "Elk"),
            ("Fish", "Fish"),
            ("Fungus", "Fungi"),
            ("Graveborn", "Graveborn"),
            ("Hero", "Heroes"),
            ("Jellyfish", "Jellyfish"),
            ("Kithkin", "Kithkin"),
            ("Kor", "Kor"),
            ("Merfolk", "Merfolk"),
            ("Moonfolk", "Moonfolk"),
            ("Mouse", "Mice"),
            ("Myr", "Myr"),
            ("Ox", "Oxen"),
            ("Pegasus", "Pegasi"),
            ("Samurai", "Samurai"),
            ("Squid", "Squid"),
            ("Starfish", "Starfish"),
            ("Thalakos", "Thalakos"),
            ("Treefolk", "Treefolk"),
            ("Vedalken", "Vedalken"),
            ("Werewolf", "Werewolves"),
            ("Wolf", "Wolves"),
            ("Zubera", "Zubera"),
        ];
        let regular = [
            ("Capybara", "Capybaras"),
            ("Echidna", "Echidnas"),
            ("Harpy", "Harpies"),
            ("Kraken", "Krakens"),
            ("Lamia", "Lamias"),
            ("Lammasu", "Lammasus"),
            ("Leech", "Leeches"),
            ("Mercenary", "Mercenaries"),
            ("Naga", "Nagas"),
            ("Nautilus", "Nautiluses"),
            ("Ninja", "Ninjas"),
            ("Octopus", "Octopuses"),
            ("Orgg", "Orggs"),
            ("Phoenix", "Phoenixes"),
            ("Platypus", "Platypuses"),
            ("Rukh", "Rukhs"),
            ("Skrull", "Skrulls"),
            ("Sphinx", "Sphinxes"),
            ("Spy", "Spies"),
            ("Synth", "Synths"),
            ("Teddy", "Teddies"),
            ("Thrull", "Thrulls"),
            ("Volver", "Volvers"),
            ("Walrus", "Walruses"),
        ];
        let catalogs = Catalogs::default().with_catalog(
            CatalogKind::CreatureType,
            exceptional
                .iter()
                .chain(&regular)
                .map(|(singular, _)| *singular),
        );

        for (singular, plural) in exceptional {
            let atom = catalog_atom(&catalogs, singular);
            assert!(atom.vocab.is_some(), "{singular} must link an exception");
            assert_eq!(atom.render_noun(false), singular);
            assert_eq!(atom.render_noun(true), plural);
            for surface in [singular, plural] {
                assert!(
                    Vocabulary::new()
                        .matches(surface, LexicalSlot::Noun(NounUsage::Count))
                        .is_empty(),
                    "{surface} must be recognized through its catalog, not ordinary vocabulary"
                );
            }
            assert!(
                catalogs
                    .matches(plural, CatalogSlot::Noun(NounUsage::Count))
                    .iter()
                    .any(|catalog_match| matches!(
                        &catalog_match.value,
                        CatalogValue::Word(WordMatch::Noun(noun))
                            if matches!(
                                noun.kind(),
                                crate::word::NounInstanceKind::Plural(Noun::Catalog(candidate))
                                    if candidate.canonical() == singular
                            )
                    ))
            );
        }

        for (singular, plural) in regular {
            let atom = catalog_atom(&catalogs, singular);
            assert_eq!(atom.vocab, None, "{singular} is mechanically regular");
            assert_eq!(atom.render_noun(false), singular);
            assert_eq!(atom.render_noun(true), plural);
        }
    }

    #[test]
    fn keyword_actions_delegate_head_inflection_and_keep_every_length() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            [
                "Collect Evidence",
                "Manifest",
                "Manifest Dread",
                "Florbulate",
            ],
            std::iter::empty::<&str>(),
        );

        let matches = catalogs.matches(
            "manifests dread 2",
            CatalogSlot::Verb(THIRD_SINGULAR_PRESENT),
        );
        assert_eq!(
            matches.iter().map(|item| item.length).collect::<Vec<_>>(),
            ["manifests".len(), "manifests dread".len()]
        );
        assert!(matches.iter().all(|catalog_match| matches!(
            &catalog_match.value,
            CatalogValue::Word(WordMatch::Verb(instance))
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.head() == "manifest" && action.irregular_head().is_none())
                    && instance.slot == THIRD_SINGULAR_PRESENT
        )));
        let CatalogValue::Word(WordMatch::Verb(long_action)) = &matches[1].value else {
            panic!("expected a keyword-action verb");
        };
        assert_eq!(
            crate::word::Vocabulary::new().render_verb_instance(long_action),
            Some("manifests dread".to_owned())
        );

        let collected = one_word_match(
            &catalogs,
            "collected evidence",
            CatalogSlot::Verb(VerbSlot::PastParticiple),
        );
        assert!(matches!(
            collected,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.head() == "collect"
                        && action.irregular_head().is_none()
                        && action.tail() == "evidence")
        ));

        let invented = one_word_match(
            &catalogs,
            "florbulate",
            CatalogSlot::Verb(VerbSlot::Imperative),
        );
        assert!(matches!(
            invented,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.head() == "florbulate"
                        && action.irregular_head().is_none())
        ));
    }

    #[test]
    fn multi_word_keyword_action_canonicals_lowercase_the_whole_render() {
        // The 9 real multi-word canonicals from
        // `data/gen/catalogs/keyword-actions.txt` (round `mwcanon`).
        // Each has a capitalized tail in the generated file; the
        // resolver must lowercase both head and tail so the rendered form
        // matches the oracle's running-text spelling.
        const CANONICALS: [&str; 9] = [
            "Collect Evidence",
            "Face a Villainous Choice",
            "Manifest Dread",
            "Open an Attraction",
            "Roll to Visit Your Attractions",
            "Set in Motion",
            "The Ring Tempts You",
            "Time Travel",
            "Venture into the Dungeon",
        ];
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            CANONICALS,
            std::iter::empty::<&str>(),
        );

        for canonical in CANONICALS {
            let lower = canonical.to_ascii_lowercase();
            let (head_lower, tail_lower) = lower.split_once(' ').unwrap();
            let surface = format!("{head_lower}s {tail_lower}"); // third-singular-present head
            let matches = catalogs.matches(&surface, CatalogSlot::Verb(THIRD_SINGULAR_PRESENT));
            let CatalogValue::Word(WordMatch::Verb(instance)) = &matches
                .iter()
                .find(|m| m.length == surface.len())
                .expect("full-span match for the running-text surface")
                .value
            else {
                panic!("expected a keyword-action verb for {canonical}");
            };
            let Verb::KeywordAction(action) = &instance.verb else {
                panic!("expected KeywordAction for {canonical}");
            };
            assert!(
                action.head().bytes().all(|b| !b.is_ascii_uppercase()),
                "{canonical}: head must be fully lowercase, got {:?}",
                action.head()
            );
            assert!(
                action.tail().bytes().all(|b| !b.is_ascii_uppercase()),
                "{canonical}: tail must be fully lowercase, got {:?}",
                action.tail()
            );
        }

        // §2.3's exclusion, pinned as intended behaviour: the oracle
        // capitalizes the proper noun `Ring`, so `The Ring Tempts You`
        // never matches its own oracle surface under
        // `CasePolicy::Exact`, even though the canonical is now fully
        // lowercased internally.
        let oracle_surface_matches = catalogs.matches(
            "the Ring tempts you",
            CatalogSlot::Verb(VerbSlot::Imperative),
        );
        assert!(
            !oracle_surface_matches.iter().any(
                |m| matches!(&m.value, CatalogValue::Word(WordMatch::Verb(instance))
                    if matches!(&instance.verb, Verb::KeywordAction(action)
                        if action.canonical() == "The Ring Tempts You"))
            ),
            "the capitalized `Ring` must not byte-match the lowercased tail"
        );
    }

    #[test]
    fn single_word_keyword_action_canonicals_keep_an_empty_tail() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Scry", "Investigate", "Amass"],
            std::iter::empty::<&str>(),
        );
        for (canonical, surface) in [
            ("Scry", "scry"),
            ("Investigate", "investigate"),
            ("Amass", "amass"),
        ] {
            let matched =
                one_word_match(&catalogs, surface, CatalogSlot::Verb(VerbSlot::Imperative));
            assert!(matches!(
                matched,
                WordMatch::Verb(instance)
                    if matches!(&instance.verb, Verb::KeywordAction(action)
                        if action.canonical() == canonical && action.tail().is_empty())
            ));
        }
    }

    #[test]
    fn narrowed_sentence_initial_retry_does_not_lowercase_a_non_initial_capital() {
        // Edit 2's defining property: only the sentence-initial token's own
        // bytes are lowercased for the retry, never the remainder of
        // the suffix. A capitalized non-initial token inside a
        // multi-word canonical must therefore still fail to match under
        // `CasePolicy::Exact`.
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Venture Into The Dungeon"],
            std::iter::empty::<&str>(),
        );
        let matches = catalogs.matches(
            "Venture Into the dungeon",
            CatalogSlot::Verb(VerbSlot::Imperative),
        );
        assert!(
            matches
                .iter()
                .all(|m| m.length < "venture into the dungeon".len()),
            "a non-initial capitalized token must not be swept into the retry's lowercasing"
        );
    }

    #[test]
    fn manifest_survives_the_arrival_of_manifest_dread() {
        // The single-word canonical must keep resolving on its own once its
        // multi-word sibling is present in the same catalog
        // (anti-regression for Edit 1/Edit 3).
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Manifest", "Manifest Dread"],
            std::iter::empty::<&str>(),
        );
        let matched = one_word_match(
            &catalogs,
            "manifest",
            CatalogSlot::Verb(VerbSlot::Imperative),
        );
        assert!(matches!(
            matched,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.canonical() == "Manifest" && action.tail().is_empty())
        ));
    }

    #[test]
    fn collect_alone_does_not_resolve_to_collect_evidence() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Collect Evidence"],
            std::iter::empty::<&str>(),
        );
        let matches = catalogs.matches("collect", CatalogSlot::Verb(VerbSlot::Imperative));
        assert!(
            matches.iter().all(
                |m| !matches!(&m.value, CatalogValue::Word(WordMatch::Verb(instance))
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.canonical() == "Collect Evidence"))
            ),
            "a bare `collect` must not resolve to the multi-word canonical"
        );
    }

    #[test]
    fn keyword_actions_use_vocab_only_for_irregular_heads() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Airbend", "Waterbend"],
            std::iter::empty::<&str>(),
        );
        let past = VerbSlot::Past {
            person: Person::Third,
            number: Number::Singular,
        };

        let airbent = one_word_match(&catalogs, "airbended", CatalogSlot::Verb(past));
        assert!(matches!(
            airbent,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.head() == "airbend" && action.irregular_head().is_none())
        ));

        let waterbent = one_word_match(&catalogs, "waterbent", CatalogSlot::Verb(past));
        assert!(matches!(
            waterbent,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.irregular_head() == Some(Vocab::Waterbend))
        ));
    }

    #[test]
    fn capitalized_subtypes_do_not_become_mid_sentence_keyword_actions() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Food"],
            std::iter::empty::<&str>(),
        )
        .with_catalog(CatalogKind::ArtifactType, ["Food"]);
        let finite = VerbSlot::Present {
            person: Person::Third,
            number: Number::Plural,
        };

        assert!(
            catalogs
                .matches("Food", CatalogSlot::Verb(finite))
                .is_empty()
        );
        assert!(matches!(
            one_word_match(&catalogs, "food", CatalogSlot::Verb(finite)),
            WordMatch::Verb(_)
        ));
        assert!(
            catalogs
                .matches("Food", CatalogSlot::Verb(VerbSlot::Imperative))
                .is_empty()
        );
        assert!(matches!(
            one_word_match(&catalogs, "food", CatalogSlot::Verb(VerbSlot::Imperative)),
            WordMatch::Verb(_)
        ));
        assert!(matches!(
            one_word_match(&catalogs, "Food", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Singular(Noun::Catalog(_))
                )
        ));
    }

    #[test]
    fn rules_bundle_words_fill_their_declared_slots() {
        // The hand-curated bundle catalog is always present, independent of the
        // externally supplied Scryfall catalogs.
        let catalogs = Catalogs::default();

        // Adjective-only bundles fill the adjective slot and no noun slot.
        for spelling in ["historic", "modified"] {
            let WordMatch::Adjective(Adjective::Catalog(atom)) =
                one_word_match(&catalogs, spelling, CatalogSlot::Adjective)
            else {
                panic!("{spelling} must be a catalog adjective");
            };
            assert!(atom.is_rules_bundle());
            assert_eq!(atom.canonical(), spelling);
            assert!(
                catalogs
                    .matches(spelling, CatalogSlot::Noun(NounUsage::Count))
                    .is_empty(),
                "{spelling} is not a noun"
            );
        }

        // `party` is a count noun only, singular and plural, never attributive.
        let WordMatch::Noun(party) =
            one_word_match(&catalogs, "party", CatalogSlot::Noun(NounUsage::Count))
        else {
            panic!("party must be a singular catalog noun");
        };
        let crate::word::NounInstanceKind::Singular(Noun::Catalog(party)) = party.kind() else {
            panic!("party must be a singular catalog noun");
        };
        assert!(party.is_rules_bundle());
        assert_eq!(party.render_noun(true), "parties");
        assert!(matches!(
            one_word_match(&catalogs, "parties", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Plural(Noun::Catalog(_))
                )
        ));
        assert!(catalogs.matches("party", CatalogSlot::Adjective).is_empty());

        // `outlaw` fills both the count-noun and the adjective slot.
        assert!(matches!(
            one_word_match(&catalogs, "outlaw", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Singular(Noun::Catalog(atom))
                        if atom.is_rules_bundle()
                )
        ));
        assert!(matches!(
            one_word_match(&catalogs, "outlaws", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Plural(Noun::Catalog(_))
                )
        ));
        assert!(matches!(
            one_word_match(&catalogs, "outlaw", CatalogSlot::Adjective),
            WordMatch::Adjective(Adjective::Catalog(atom)) if atom.is_rules_bundle()
        ));

        // A capitalized surface never matches directly (the grammar retries a
        // sentence-initial capital lowercased), and the mass slot never fills.
        assert!(
            catalogs
                .matches("Outlaw", CatalogSlot::Adjective)
                .is_empty()
        );
        assert!(
            catalogs
                .matches("party", CatalogSlot::Noun(NounUsage::Mass))
                .is_empty()
        );
        // A partial prefix does not fire without a word boundary.
        assert!(
            catalogs
                .matches("partying", CatalogSlot::Noun(NounUsage::Count))
                .is_empty()
        );
    }

    #[test]
    fn legacy_catalog_set_populates_the_grammar_coupled_kinds() {
        let set = |value: &str| BTreeSet::from([value.to_owned()]);
        let legacy = LegacyCatalogSet::from_entries(BTreeMap::from([
            (LegacyCatalogKind::AbilityWords, set("Ascendance")),
            (LegacyCatalogKind::ArtifactTypes, set("Gadget")),
            (LegacyCatalogKind::BattleTypes, set("Invasion")),
            (LegacyCatalogKind::CardTypes, set("Widget")),
            (LegacyCatalogKind::CreatureTypes, set("Glimmerkin")),
            (LegacyCatalogKind::EnchantmentTypes, set("Omen")),
            (LegacyCatalogKind::KeywordAbilities, set("Skyguard")),
            (LegacyCatalogKind::KeywordActions, set("Glint")),
            (LegacyCatalogKind::LandTypes, set("Steppe")),
            (LegacyCatalogKind::PlaneswalkerTypes, set("Neriah")),
            (LegacyCatalogKind::SpellTypes, set("Ritual")),
            (LegacyCatalogKind::Supertypes, set("Prime")),
            (LegacyCatalogKind::FlavorWords, set("Asterism")),
        ]))
        .unwrap();

        let catalogs = Catalogs::from_legacy(&legacy);

        for (kind, canonical) in [
            (CatalogKind::AbilityWord, "Ascendance"),
            (CatalogKind::ArtifactType, "Gadget"),
            (CatalogKind::BattleType, "Invasion"),
            (CatalogKind::CardType, "Widget"),
            (CatalogKind::CreatureType, "Glimmerkin"),
            (CatalogKind::EnchantmentType, "Omen"),
            (CatalogKind::KeywordAbility, "Skyguard"),
            (CatalogKind::LandType, "Steppe"),
            (CatalogKind::PlaneswalkerType, "Neriah"),
            (CatalogKind::SpellType, "Ritual"),
            (CatalogKind::Supertype, "Prime"),
            (CatalogKind::FlavorWord, "Asterism"),
        ] {
            assert!(
                catalogs
                    .entries
                    .iter()
                    .any(|atom| atom.kind == kind && atom.canonical() == canonical),
                "missing {kind:?} entry {canonical:?}"
            );
        }

        assert!(matches!(
            catalogs.matches("skyguard", CatalogSlot::AbilityItem).as_slice(),
            [CatalogMatch { value: CatalogValue::Atom(atom), .. }]
                if atom.kind == CatalogKind::KeywordAbility && atom.canonical() == "Skyguard"
        ));
        assert!(matches!(
            one_word_match(&catalogs, "glint", CatalogSlot::Verb(VerbSlot::Imperative)),
            WordMatch::Verb(instance)
                if matches!(instance.verb, Verb::KeywordAction(ref action) if action.canonical() == "Glint")
        ));
        assert!(matches!(
            catalogs.matches("Ascendance", CatalogSlot::AbilityWord).as_slice(),
            [CatalogMatch { value: CatalogValue::Atom(atom), .. }]
                if atom.kind == CatalogKind::AbilityWord && atom.canonical() == "Ascendance"
        ));
        assert!(matches!(
            one_word_match(&catalogs, "Glimmerkin", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(noun.kind(), crate::word::NounInstanceKind::Singular(Noun::Catalog(atom))
                    if atom.kind == CatalogKind::CreatureType && atom.canonical() == "Glimmerkin")
        ));
        assert!(matches!(
            one_word_match(&catalogs, "widget", CatalogSlot::Noun(NounUsage::Count)),
            WordMatch::Noun(noun)
                if matches!(noun.kind(), crate::word::NounInstanceKind::Singular(Noun::Catalog(atom))
                    if atom.kind == CatalogKind::CardType && atom.canonical() == "Widget")
        ));
        assert!(catalogs.is_flavor_word("Asterism"));
    }

    fn one_word_match(catalogs: &Catalogs, surface: &str, slot: CatalogSlot) -> WordMatch {
        let matches = catalogs.matches(surface, slot);
        assert_eq!(matches.len(), 1, "matches for {surface:?}: {matches:#?}");
        let CatalogValue::Word(word) = matches.into_iter().next().unwrap().value else {
            panic!("expected a grammatical word match for {surface:?}");
        };
        word
    }

    fn catalog_atom<'catalogs>(
        catalogs: &'catalogs Catalogs,
        canonical: &str,
    ) -> &'catalogs CatalogAtom {
        catalogs
            .entries
            .iter()
            .find(|atom| atom.kind == CatalogKind::CreatureType && atom.canonical() == canonical)
            .unwrap_or_else(|| panic!("missing creature type {canonical:?}"))
    }
}
