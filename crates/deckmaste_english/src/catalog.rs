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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl CatalogKind {
    const ALL: [Self; 12] = [
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
            | Self::SpellType => CasePolicy::Exact,
            Self::Supertype | Self::CardType => CasePolicy::Lowercase,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatalogAtom {
    pub kind: CatalogKind,
    canonical: Arc<str>,
    spelling: Arc<str>,
    pub vocab: Option<Vocab>,
}

impl CatalogAtom {
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
                NounInstance::Plural(Noun::Word(vocab))
            } else {
                NounInstance::Singular(Noun::Word(vocab))
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

        match self.kind.case_policy() {
            CasePolicy::Lowercase => rendered.to_ascii_lowercase(),
            CasePolicy::Exact | CasePolicy::Insensitive => {
                apply_initial_case(&rendered, &self.canonical)
            }
        }
    }

    #[must_use]
    pub fn render_adjective(&self) -> String {
        match self.kind.case_policy() {
            CasePolicy::Lowercase => self.canonical.to_ascii_lowercase(),
            CasePolicy::Exact | CasePolicy::Insensitive => self.canonical.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeywordAction {
    canonical: Arc<str>,
    head: KeywordActionHead,
    tail: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CatalogId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CatalogSlot {
    AbilityItem,
    KeywordAbilityNoun,
    AbilityWord,
    Noun(NounUsage),
    Adjective,
    Verb(VerbSlot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogMatch {
    pub(crate) length: usize,
    pub(crate) value: CatalogValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CasePolicy {
    Exact,
    Lowercase,
    Insensitive,
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
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
        matches.sort_by_key(|catalog_match| catalog_match.length);
        matches
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

        for index in &mut indexes {
            index.finish();
        }

        let mut actions_by_head_surface: HashMap<String, Vec<KeywordAction>> = HashMap::new();
        for canonical in &self.sources[CatalogKind::KeywordAction.index()] {
            let action = resolve_keyword_action(canonical);
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
                        value: CatalogValue::Word(WordMatch::Noun(NounInstance::Singular(
                            Noun::Catalog(atom.clone()),
                        ))),
                    });
                }
                if surface_equals(surface, &atom.render_noun(true), kind.case_policy()) {
                    matches.push(CatalogMatch {
                        length,
                        value: CatalogValue::Word(WordMatch::Noun(NounInstance::Plural(
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

fn resolve_keyword_action(canonical: &Arc<str>) -> KeywordAction {
    let (head_surface, tail) = canonical
        .split_once(' ')
        .map_or((canonical.as_ref(), ""), |(head, tail)| (head, tail));
    let lemma = if head_surface.eq_ignore_ascii_case("prepared") {
        "prepare"
    } else {
        head_surface
    };
    let head = Vocabulary::irregular_keyword_action_head(lemma).map_or_else(
        || KeywordActionHead::Regular(Arc::from(lemma.to_ascii_lowercase())),
        KeywordActionHead::Irregular,
    );
    KeywordAction {
        canonical: Arc::clone(canonical),
        head,
        tail: Arc::from(tail),
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
        let WordMatch::Noun(NounInstance::Singular(Noun::Catalog(goblin_atom))) = goblin else {
            panic!("expected a singular catalog noun, got {goblin:?}");
        };
        assert_eq!(goblin_atom.kind, CatalogKind::CreatureType);
        assert_eq!(goblin_atom.vocab, None);
        assert_eq!(goblin_atom.render_noun(false), "Goblin");
        assert_eq!(goblin_atom.render_noun(true), "Goblins");
        assert_eq!(
            crate::word::Vocabulary::new()
                .render_noun(&NounInstance::Plural(Noun::Catalog(goblin_atom.clone()),)),
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
            WordMatch::Noun(NounInstance::Plural(Noun::Catalog(_)))
        ));

        let merfolk_matches = catalogs.matches("Merfolk", CatalogSlot::Noun(NounUsage::Count));
        assert!(merfolk_matches.iter().any(|catalog_match| matches!(
            &catalog_match.value,
            CatalogValue::Word(WordMatch::Noun(NounInstance::Plural(Noun::Catalog(atom))))
                if atom.kind == CatalogKind::CreatureType
                    && atom.vocab == Some(Vocab::Merfolk)
                    && atom.render_noun(true) == "Merfolk"
        )));

        let hero = one_word_match(&catalogs, "Hero", CatalogSlot::Noun(NounUsage::Count));
        let WordMatch::Noun(NounInstance::Singular(Noun::Catalog(hero))) = hero else {
            panic!("expected Hero to be a subtype");
        };
        assert_eq!(hero.kind, CatalogKind::CreatureType);

        let creature = one_word_match(&catalogs, "creature", CatalogSlot::Noun(NounUsage::Count));
        let WordMatch::Noun(NounInstance::Singular(Noun::Catalog(creature))) = creature else {
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
                        CatalogValue::Word(WordMatch::Noun(NounInstance::Plural(
                            Noun::Catalog(candidate)
                        ))) if candidate.canonical() == singular
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
                "Collect evidence",
                "Manifest",
                "Manifest dread",
                "Prepared",
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

        let prepared = one_word_match(
            &catalogs,
            "prepared",
            CatalogSlot::Verb(VerbSlot::PastParticiple),
        );
        assert!(matches!(
            prepared,
            WordMatch::Verb(instance)
                if matches!(&instance.verb, Verb::KeywordAction(action)
                    if action.head() == "prepare" && action.irregular_head().is_none())
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
            WordMatch::Noun(NounInstance::Singular(Noun::Catalog(_)))
        ));
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
