//! Binding a [`Grammar`] to a lexical environment: validation, name
//! resolution, and compilation of Schemas into Productions.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::SourceKind;
use dpsi::Ident;

use crate::LoadError;
use crate::frames::Frame;
use crate::frames::FrameCatalog;
use crate::frames::FrameFamily;
use crate::frames::PrepositionFrame;
use crate::frames::SlotFill;
use crate::frames::SlotKind;
use crate::frames::SlotRelation;
use crate::grammar::Equation;
use crate::grammar::Grammar;
use crate::grammar::HostKind;
use crate::grammar::SchemaDecl;
use crate::grammar::Side;
use crate::grammar::SymbolDecl;
use crate::step::Extension;
use crate::step::Gap;
use crate::step::GapRule;
use crate::step::Production;
use crate::step::Symbol;
use crate::step::Test;
use crate::step::Weight;

/// Feature names that would police attachment rather than describe grammar.
const FORBIDDEN_SUFFIXES: [&str; 5] = ["Use", "Licence", "License", "Present", "Restriction"];

/// Lexical features the engine projects onto leaves, by grammar feature name.
///
/// With [`crate::step::projected_values`] and `LeafFacts` projection
/// ([`CompiledGrammar::leaf_facts`]), this is the lexicon feature-bundle
/// mapping table: the one declared lexicon-to-syntax interface, and the only
/// place the engine names a grammar feature or value
/// (`docs/decisions/english-v4.md` §4.4, item 4).
pub(crate) const PROJECTED: [&str; 8] = [
    "Number",
    "Person",
    "Tense",
    "VerbForm",
    "Case",
    "Inflection",
    "Countability",
    "Sign",
];

/// The index in [`PROJECTED`] of a numeral's or noun's Number.
pub(crate) const NUMBER: usize = 0;
/// The index in [`PROJECTED`] of a word's inflectional Word Form.
pub(crate) const INFLECTION: usize = 5;
/// The index in [`PROJECTED`] of a noun's Countability.
pub(crate) const COUNTABILITY: usize = 6;
/// The index in [`PROJECTED`] of a numeral's Sign.
pub(crate) const SIGN: usize = 7;

/// The lexical facts a bound grammar keeps about one lexeme.
#[derive(Debug, Clone)]
pub(crate) struct LexemeFacts {
    pub countability: Option<Ident>,
    /// A noun both count and mass is count in its plural form.
    pub countable_plural: bool,
    pub features: Vec<(usize, u16)>,
    pub category: LexicalCategory,
    pub binding: Binding,
    pub frames: BTreeSet<usize>,
    /// Whether the lexeme may stand without a Complement where a phrase
    /// makes Complements obligatory: it declares no Frame, or an
    /// Intransitive one.
    pub optional_complement: bool,
    /// Whether every Frame it declares is restricted to active voice, so it
    /// has no passive participle (*cost*, lexical *have*).
    pub active_only: bool,
    pub preposition_frames: BTreeSet<usize>,
    /// The kind of source that declares it.
    pub source: deckmaste_lexical::SourceKind,
}

/// A grammar bound to a lexical environment, ready to parse, admit and realize.
#[derive(Debug)]
pub struct CompiledGrammar {
    pub(crate) categories: Vec<Ident>,
    category_ids: BTreeMap<Ident, usize>,
    pub(crate) category_features: Vec<Vec<usize>>,
    /// Each Category's mask of the Gaps it may contain (`Gap::bit`).
    pub(crate) category_gap: Vec<u16>,
    /// Categories that name a function (`CategoryDecl::function`).
    pub(crate) function_categories: BTreeSet<Ident>,
    pub(crate) features: Vec<Ident>,
    pub(crate) values: Vec<Vec<Ident>>,
    pub(crate) productions: Vec<Production>,
    pub(crate) by_id: BTreeMap<Ident, usize>,
    pub(crate) by_lhs: Vec<Vec<usize>>,
    pub(crate) root: usize,
    pub(crate) lexemes: BTreeMap<LexemeId, LexemeFacts>,
    /// The card-scoped spellings of the lexicon, as (Lexeme, spelling
    /// variant), which analysis admits only within their card's scope.
    pub(crate) card_scoped: BTreeSet<(LexemeId, usize)>,
    pub(crate) frames: Vec<Frame>,
    pub(crate) preposition_frames: Vec<PrepositionFrame>,
    /// Prepositions whose `PP` slot would extend a head's selected frame,
    /// by (lexeme, selected frame), as an index into `extension_sets`.
    pub(crate) extensions: BTreeMap<(LexemeId, Option<usize>), u16>,
    /// Distinct extension lists; index `0` is the empty list.
    pub(crate) extension_sets: Vec<Vec<Extension>>,
    pub(crate) classes: Vec<BTreeSet<LexemeId>>,
    /// Prepositions some Production's `unextended` child may not end in, the
    /// only ones a Summary's `final_preposition` records.
    pub(crate) unextended_prepositions: BTreeSet<LexemeId>,
    /// Projected inflectional feature ids; additional properties are
    /// grammar-declared.
    pub(crate) projected: [Option<usize>; 8],
    /// The grammar's `high_attachment` declarations: `(Category, feature,
    /// value)`.
    pub(crate) high_attachment: Vec<(Ident, Ident, Ident)>,
    pub(crate) prediction: crate::prediction::Prediction,
    report: LoadReport,
}

/// Findings that do not prevent binding.
#[derive(Debug, Clone, Default)]
pub struct LoadReport {
    pub warnings: Vec<String>,
}

impl CompiledGrammar {
    /// Binds `grammar` to the lexicon and frame catalog.
    ///
    /// # Errors
    /// Any unknown name, ill-typed equation, forbidden feature name, grammar
    /// reference to a non-core or open-class lexeme, or frame that cannot be
    /// compiled.
    pub fn bind(
        grammar: &Grammar,
        lexicon: &Lexicon,
        catalog: &FrameCatalog,
    ) -> Result<Self, LoadError> {
        Binder::new(grammar, lexicon, catalog)?.finish()
    }

    #[must_use]
    pub fn report(&self) -> &LoadReport {
        &self.report
    }

    /// Stable ids of every Production, authored and Schema-derived.
    pub fn production_ids(&self) -> impl Iterator<Item = &str> {
        self.productions.iter().map(|p| p.id.as_str())
    }

    pub(crate) fn category_id(&self, name: &str) -> Option<usize> {
        self.category_ids.get(name).copied()
    }
}

fn invalid(message: impl Into<String>) -> LoadError {
    LoadError::Invalid(message.into())
}

struct Binder<'a> {
    grammar: &'a Grammar,
    catalog: &'a FrameCatalog,
    out: CompiledGrammar,
    closed_words: BTreeMap<Ident, LexemeId>,
    class_names: BTreeMap<Ident, usize>,
    /// The Productions that set each (feature, value) on their mother.
    setters: BTreeMap<(usize, u16), BTreeSet<Ident>>,
    /// The Productions that require each (feature, value) of a child.
    valuers: BTreeMap<(usize, u16), BTreeSet<Ident>>,
    /// Pairs of distinct features some equation equates.
    links: BTreeSet<(usize, usize)>,
}

/// One child of a Production under construction.
#[derive(Clone)]
struct Child {
    name: Ident,
    symbol: Symbol,
    gap: GapRule,
}

/// A Production under construction, before equations are resolved.
#[derive(Clone)]
struct Draft {
    id: Ident,
    lhs: usize,
    children: Vec<Child>,
    equations: Vec<(Path, Target)>,
    exclusions: Vec<(usize, usize, u16)>,
    /// Combinations of (child, feature, value) that may not all hold.
    joint_exclusions: Vec<Vec<(usize, usize, u16)>>,
    carried: Vec<(usize, usize, u16)>,
    /// Features a coordination resolves: (feature, value when they differ,
    /// whether the mother is the whole coordination rather than a tail).
    resolutions: Vec<(usize, u16, bool)>,
    resolution_nonlast: Vec<usize>,
    resolution_last: Option<(usize, bool)>,
    weights: Vec<(usize, Weight)>,
    cost: u32,
    introduces: Gap,
    /// A child follows the Gap it introduces, which is therefore not at its
    /// right edge. Right Node Raising shares only a Gap at the right edge of
    /// each conjunct (`Summary::gap_right_edge`): the delayed constituent is
    /// the one that would be the rightmost element of a coordinate (CGEL,
    /// Ch. 15 §4.4, p. 1343).
    gap_followed: bool,
    frame: Option<usize>,
    host_check: Option<(usize, usize, HostKind)>,
    adjunct_slot: Option<SlotKind>,
    predicative: Option<usize>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
    /// The coordinator a correlative coordination's serial tail must end with.
    paired_coordinator: Option<LexemeId>,
    uncoordinated: Vec<usize>,
    scoped: Vec<usize>,
    undelimited: Vec<usize>,
    /// A binary coordination with a delimiter before its coordinator.
    delimits: bool,
    /// Conjuncts that must differ in a feature: (first, last, feature).
    unlike: Option<(usize, usize, usize)>,
    /// A polysyndetic tail that must repeat the coordinator.
    chained: Option<usize>,
    gap_preposition: Option<LexemeId>,
    fills_preposition: Option<usize>,
    /// Coordinators a serial tail admits for correlatives only, which a
    /// plain serial coordination may not end with.
    foreign_coordinators: Vec<LexemeId>,
    /// A derived passive's Internalised Complement.
    internalised: Option<usize>,
    /// A noun's bare phrase, which omits its head's Complements.
    omits_complement: bool,
}

/// What a Valence schema declares its derived phrases carry, beyond their
/// frames.
struct Derivations<'d> {
    /// The phrase's feature naming its first Frame Slot's Slot Kind.
    complement_kind: Option<usize>,
    /// Equations on every active phrase.
    active: &'d [Equation],
    passive: Option<PassiveDecl<'d>>,
    /// The Adjectival Passive's Category and its equations.
    adjectival: Option<(&'d str, &'d [Equation])>,
}

/// A Valence schema's passive derivations.
struct PassiveDecl<'d> {
    /// The Closed Word marking the Internalised Complement.
    by: &'d str,
    /// Equations on every verbal passive.
    verbal: &'d [Equation],
    /// Equations on every prepositional passive.
    prepositional: &'d [Equation],
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Path {
    Mother(usize),
    Child(usize, usize),
}

#[derive(Clone)]
enum Target {
    Path(Path),
    Value(u16),
}

impl Draft {
    fn new(id: impl AsRef<str>, lhs: usize, cost: u32) -> Self {
        Self {
            id: Ident::new(id.as_ref()),
            lhs,
            children: Vec::new(),
            equations: Vec::new(),
            exclusions: Vec::new(),
            joint_exclusions: Vec::new(),
            carried: Vec::new(),
            resolutions: Vec::new(),
            resolution_nonlast: Vec::new(),
            resolution_last: None,
            weights: Vec::new(),
            cost,
            introduces: Gap::None,
            gap_followed: false,
            frame: None,
            host_check: None,
            adjunct_slot: None,
            predicative: None,
            coordinator: None,
            distinct_coordinator: Vec::new(),
            requires_coordination: None,
            paired_coordinator: None,
            uncoordinated: Vec::new(),
            scoped: Vec::new(),
            undelimited: Vec::new(),
            delimits: false,
            chained: None,
            gap_preposition: None,
            fills_preposition: None,
            foreign_coordinators: Vec::new(),
            unlike: None,
            internalised: None,
            omits_complement: false,
        }
    }

    fn child(&mut self, name: &str, symbol: Symbol, gap: GapRule) -> usize {
        self.children.push(Child {
            name: Ident::new(name),
            symbol,
            gap,
        });
        self.children.len() - 1
    }
}

impl<'a> Binder<'a> {
    #[expect(clippy::too_many_lines, reason = "validation in declaration order")]
    fn new(
        grammar: &'a Grammar,
        lexicon: &'a Lexicon,
        catalog: &'a FrameCatalog,
    ) -> Result<Self, LoadError> {
        let mut report = LoadReport::default();
        let features: Vec<Ident> = grammar.features.keys().copied().collect();
        for name in &features {
            if FORBIDDEN_SUFFIXES.iter().any(|s| name.ends_with(s)) {
                return Err(invalid(format!(
                    "feature `{name}` is a policing flag, which the grammar may not declare"
                )));
            }
        }
        if features.len() > crate::step::MAX_FEATURES {
            return Err(invalid(format!(
                "the grammar declares {} features, more than {}",
                features.len(),
                crate::step::MAX_FEATURES
            )));
        }
        let values: Vec<Vec<Ident>> = grammar.features.values().cloned().collect();
        for (name, domain) in features.iter().zip(&values) {
            if domain.len() > usize::from(u8::MAX) {
                return Err(invalid(format!(
                    "feature `{name}` has {} values, more than 255",
                    domain.len()
                )));
            }
        }
        let feature_id = |name: &str| {
            features
                .iter()
                .position(|f| f == name)
                .ok_or_else(|| invalid(format!("unknown feature `{name}`")))
        };
        // Each coordination schema gets an engine-owned serial tail Category.
        let mut declared = grammar.categories.clone();
        for schema in &grammar.schemas {
            if let SchemaDecl::Coordination {
                category,
                coordinators,
                ..
            } = schema
            {
                let declaration = declared
                    .get(category)
                    .cloned()
                    .ok_or_else(|| invalid(format!("unknown category `{category}`")))?;
                let tail = Ident::new(&serial_tail(
                    &grammar.schemas,
                    schema,
                    category,
                    coordinators,
                ));
                declared.insert(Ident::new(&format!("{tail}+")), declaration.clone());
                declared.insert(Ident::new(&format!("{tail}++")), declaration.clone());
                declared.insert(tail, declaration);
            }
        }
        let categories: Vec<Ident> = declared.keys().copied().collect();
        let mut category_features = Vec::new();
        for (name, declaration) in &declared {
            let ids = declaration
                .features
                .iter()
                .map(|f| feature_id(f))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| invalid(format!("category `{name}`: {e}")))?;
            category_features.push(ids);
        }
        let mut category_gap = Vec::new();
        for (name, declaration) in &declared {
            let mut mask = if declaration.gap { !Gap::None.bit() } else { 0 };
            for kind in &declaration.closed_gaps {
                mask &= !gap_named(kind)
                    .ok_or_else(|| invalid(format!("category `{name}`: unknown gap `{kind}`")))?
                    .bit();
            }
            category_gap.push(mask);
        }
        let function_categories = declared
            .iter()
            .filter(|(_, c)| c.function)
            .map(|(name, _)| *name)
            .collect();
        let root = categories
            .iter()
            .position(|c| *c == grammar.root)
            .ok_or_else(|| invalid(format!("unknown root category `{}`", grammar.root)))?;
        for (category, feature, value) in &grammar.high_attachment {
            if !categories.contains(category) {
                return Err(invalid(format!(
                    "high attachment: unknown category `{category}`"
                )));
            }
            let id = features
                .iter()
                .position(|f| f == feature)
                .ok_or_else(|| invalid(format!("high attachment: unknown feature `{feature}`")))?;
            if !values[id].contains(value) {
                return Err(invalid(format!(
                    "high attachment: unknown {feature} value `{value}`"
                )));
            }
        }
        let projected = PROJECTED.map(|name| features.iter().position(|f| f == name));
        for (slot, name) in PROJECTED.iter().zip(projected) {
            let Some(id) = name else { continue };
            for value in crate::step::projected_values(slot) {
                if !values[id].iter().any(|v| v == value) {
                    return Err(invalid(format!(
                        "feature `{slot}` must include lexical value `{value}`"
                    )));
                }
            }
        }

        // Frames, deduplicated into global ids.
        let mut frames: Vec<Frame> = Vec::new();
        let mut preposition_frames: Vec<PrepositionFrame> = Vec::new();
        let mut lexemes = BTreeMap::new();
        for (id, lexeme) in lexicon.lexemes() {
            lexemes.insert(
                *id,
                LexemeFacts {
                    features: lexeme
                        .properties
                        .features
                        .iter()
                        .filter_map(|(key, value)| {
                            let feature = features.iter().position(|f| f == key)?;
                            Some(
                                values[feature]
                                    .iter()
                                    .position(|v| v == value)
                                    .ok_or_else(|| {
                                        invalid(format!(
                                            "lexeme `{id}`: unknown {key} value `{value}`"
                                        ))
                                    })
                                    .map(|v| {
                                        (feature, u16::try_from(v).expect("small feature domain"))
                                    }),
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    countability: match lexeme.properties.countability.as_slice() {
                        [value] => Some(Ident::new(&format!("{value:?}"))),
                        _ => lexeme
                            .properties
                            .features
                            .get(PROJECTED[COUNTABILITY])
                            .copied(),
                    },
                    countable_plural: lexeme
                        .properties
                        .countability
                        .contains(&deckmaste_lexical::Countability::Count),
                    category: lexeme.category,
                    binding: lexeme.binding,
                    frames: BTreeSet::new(),
                    optional_complement: true,
                    active_only: false,
                    preposition_frames: BTreeSet::new(),
                    source: lexeme.source.kind,
                },
            );
        }
        for (id, list) in &catalog.frames {
            let facts = lexemes
                .get_mut(id)
                .ok_or_else(|| invalid(format!("frame catalog names unknown lexeme `{id}`")))?;
            if facts.category == LexicalCategory::Preposition {
                return Err(invalid(format!(
                    "preposition `{id}` declares its frames under `prepositions`"
                )));
            }
            for frame in list {
                if !frame.is_well_formed() {
                    return Err(invalid(format!("frame {frame:?} of `{id}` is ill-formed")));
                }
                let index = intern(&mut frames, frame);
                facts.frames.insert(index);
            }
            facts.optional_complement =
                list.is_empty() || list.iter().any(|f| f.family == FrameFamily::Intransitive);
            facts.active_only = !list.is_empty() && list.iter().all(|f| f.active_only);
        }
        for (id, list) in &catalog.prepositions {
            let facts = lexemes
                .get_mut(id)
                .ok_or_else(|| invalid(format!("frame catalog names unknown lexeme `{id}`")))?;
            if facts.category != LexicalCategory::Preposition {
                return Err(invalid(format!("`{id}` is not a preposition")));
            }
            for frame in list {
                let index = intern(&mut preposition_frames, frame);
                facts.preposition_frames.insert(index);
            }
        }
        let mut referenced = Vec::new();
        for frame in frames
            .iter()
            .map(|f| f.slots.iter().map(|s| &s.fill).collect::<Vec<_>>())
            .chain(preposition_frames.iter().map(|f| vec![&f.complement]))
        {
            for fill in frame {
                if let SlotFill::PP(list) = fill {
                    referenced.extend(list.iter().copied());
                }
            }
        }
        for id in referenced {
            if lexemes
                .get(&id)
                .is_none_or(|facts| facts.preposition_frames.is_empty())
            {
                return Err(invalid(format!(
                    "a PP slot names `{id}`, which is not a preposition with frames"
                )));
            }
        }
        let mut extensions = BTreeMap::new();
        let mut extension_sets: Vec<Vec<Extension>> = vec![Vec::new()];
        let mut intern = |list: Vec<Extension>| -> Result<u16, LoadError> {
            let index = extension_sets
                .iter()
                .position(|known| *known == list)
                .unwrap_or_else(|| {
                    extension_sets.push(list);
                    extension_sets.len() - 1
                });
            u16::try_from(index).map_err(|_| invalid("too many distinct extension lists"))
        };
        for (id, facts) in &lexemes {
            if facts.category != LexicalCategory::Verb {
                let mut preps: Vec<_> = facts
                    .frames
                    .iter()
                    .filter_map(|&frame| match frames[frame].slots.as_slice() {
                        [slot] => Some(extensions_of(&slot.fill)),
                        _ => None,
                    })
                    .flatten()
                    .collect();
                preps.sort();
                preps.dedup();
                if !preps.is_empty() {
                    extensions.insert((*id, None), intern(preps)?);
                }
            }
            for &selected in &facts.frames {
                let mut preps = Vec::new();
                for &other in &facts.frames {
                    let (base, longer) = (&frames[selected], &frames[other]);
                    if longer.slots.len() == base.slots.len() + 1
                        && longer.slots[..base.slots.len()] == base.slots[..]
                        && let Some(last) = longer.slots.last()
                    {
                        preps.extend(extensions_of(&last.fill));
                    }
                    // A rival frame with a PP where this one has a predicative.
                    if base.slots.len() == longer.slots.len() {
                        let differing: Vec<_> = base
                            .slots
                            .iter()
                            .zip(&longer.slots)
                            .filter(|(a, b)| a != b)
                            .collect();
                        if let [(a, b)] = differing.as_slice()
                            && a.fill == SlotFill::Pred
                            && let SlotFill::PP(list) = &b.fill
                        {
                            preps.extend(list.iter().map(|p| Extension::Predicative(*p)));
                        }
                    }
                }
                if !preps.is_empty() {
                    preps.sort();
                    preps.dedup();
                    extensions.insert((*id, Some(selected)), intern(preps)?);
                }
            }
        }
        let mut closed_words = BTreeMap::new();
        for (name, id) in &grammar.closed_words {
            let id = *id;
            check_closed(lexicon, &id, &format!("closed word `{name}`"))?;
            closed_words.insert(*name, id);
        }
        let mut classes = Vec::new();
        let mut class_names = BTreeMap::new();
        for (name, members) in &grammar.lexical_classes {
            let mut set = BTreeSet::new();
            for id in members {
                let id = *id;
                check_closed(lexicon, &id, &format!("lexical class `{name}`"))?;
                set.insert(id);
            }
            if set.len() < 2 {
                report.warnings.push(format!(
                    "lexical class `{name}` has {} member(s); a class needs two or more",
                    set.len()
                ));
            }
            class_names.insert(*name, classes.len());
            classes.push(set);
        }

        for name in &grammar.typed_classes {
            if class_names.contains_key(name) {
                return Err(invalid(format!("duplicate lexical class `{name}`")));
            }
            class_names.insert(*name, classes.len());
            let members: BTreeSet<_> = lexicon
                .lexemes()
                .iter()
                .filter(|(_, l)| l.properties.classes.contains(name))
                .map(|(id, _)| *id)
                .collect();
            if members.len() < 2 {
                report.warnings.push(format!(
                    "lexical class `{name}` has {} member(s); a class needs two or more",
                    members.len()
                ));
            }
            classes.push(members);
        }
        let by_lhs = vec![Vec::new(); categories.len()];
        Ok(Self {
            grammar,
            catalog,
            out: CompiledGrammar {
                category_ids: categories
                    .iter()
                    .enumerate()
                    .map(|(i, name)| (*name, i))
                    .collect(),
                categories,
                category_features,
                category_gap,
                function_categories,
                features,
                values,
                productions: Vec::new(),
                by_id: BTreeMap::new(),
                by_lhs,
                root,
                lexemes,
                card_scoped: lexicon
                    .values()
                    .filter(|value| lexicon.is_card_scoped(value))
                    .map(|value| (value.lexeme, value.variant))
                    .collect(),
                frames,
                preposition_frames,
                extensions,
                extension_sets,
                classes,
                unextended_prepositions: BTreeSet::new(),
                projected,
                high_attachment: grammar.high_attachment.clone(),
                prediction: crate::prediction::Prediction::default(),
                report,
            },
            closed_words,
            class_names,
            setters: BTreeMap::new(),
            valuers: BTreeMap::new(),
            links: BTreeSet::new(),
        })
    }

    fn category(&self, name: &str) -> Result<usize, LoadError> {
        self.out
            .category_id(name)
            .ok_or_else(|| invalid(format!("unknown category `{name}`")))
    }

    fn feature(&self, name: &str) -> Result<usize, LoadError> {
        self.out
            .features
            .iter()
            .position(|f| f == name)
            .ok_or_else(|| invalid(format!("unknown feature `{name}`")))
    }

    fn value(&self, feature: usize, name: &str) -> Result<u16, LoadError> {
        let index = self.out.values[feature]
            .iter()
            .position(|v| v == name)
            .ok_or_else(|| {
                invalid(format!(
                    "unknown value `{name}` of feature `{}`",
                    self.out.features[feature]
                ))
            })?;
        Ok(u16::try_from(index).expect("feature domains are small"))
    }

    fn word(&self, name: &str) -> Result<LexemeId, LoadError> {
        self.closed_words
            .get(name)
            .copied()
            .ok_or_else(|| invalid(format!("unknown closed word `{name}`")))
    }

    fn coordinator_test(&self, name: &str) -> Result<Test, LoadError> {
        if let Some(id) = self.closed_words.get(name) {
            return Ok(Test::Word(*id));
        }
        self.class_names
            .get(name)
            .map(|&c| Test::Class(c))
            .ok_or_else(|| invalid(format!("unknown closed word or class `{name}`")))
    }

    /// Whether a coordinator test admits this Closed Word.
    fn admits(&self, test: &Test, word: LexemeId) -> bool {
        match test {
            Test::Word(id) => *id == word,
            Test::Class(class) => self.out.classes[*class].contains(&word),
            Test::Words(words) => words.contains(&word),
            _ => false,
        }
    }

    /// A coordinator test widened by the paired coordinators of the
    /// correlatives, as a serial tail ends with any of them.
    fn with_words(&self, test: Test, words: &[LexemeId]) -> Test {
        let mut set: BTreeSet<LexemeId> = words.iter().copied().collect();
        match &test {
            Test::Word(id) => {
                set.insert(*id);
            }
            Test::Class(class) => set.extend(self.out.classes[*class].iter().copied()),
            Test::Words(more) => set.extend(more.iter().copied()),
            _ => return test,
        }
        if words.iter().all(|w| self.admits(&test, *w)) {
            return test;
        }
        Test::Words(set)
    }

    fn slot_category(&self, kind: SlotKind) -> Result<usize, LoadError> {
        let name = self
            .grammar
            .slot_categories
            .get(&kind)
            .ok_or_else(|| invalid(format!("no slot category declared for {kind:?}")))?;
        self.category(name)
    }

    fn finish(mut self) -> Result<CompiledGrammar, LoadError> {
        for declaration in &self.grammar.productions {
            let draft = self.authored(declaration)?;
            self.add(draft)?;
        }
        for schema in &self.grammar.schemas {
            for draft in self.schema(schema)? {
                self.add(draft)?;
            }
        }
        for declaration in &self.grammar.productions {
            for name in &declaration.unextended {
                self.unextended(declaration.name, name)?;
            }
        }
        self.auxiliaries()?;
        self.report_holders();
        for (name, declaration) in &self.grammar.categories {
            if let Some(category) = &declaration.category {
                self.category(category)?;
                let id = self.category(name)?;
                self.out.categories[id].clone_from(category);
            }
        }
        self.out.prediction = crate::prediction::Prediction::compile(&self.out);
        Ok(self.out)
    }

    /// Marks the auxiliary Verb Phrases, structurally: an uncoordinated
    /// Production whose head precedes its last child, a function Category
    /// aliasing the Production's own Category. That is a head taking a
    /// Complement of its own Category, the participial Verb Phrase Complement
    /// of perfect *have* or of passive or progressive *be*, never a modal's
    /// bare infinitival. It hosts no post-verbal Adjunct
    /// (`english-lexical-analysis.md` §2).
    fn auxiliaries(&mut self) -> Result<(), LoadError> {
        let mut complements = BTreeMap::new();
        for (name, declaration) in &self.grammar.categories {
            if let (true, Some(base)) = (declaration.function, &declaration.category) {
                complements.insert(self.category(name)?, self.category(base)?);
            }
        }
        for production in &mut self.out.productions {
            let Some(Symbol::Cat { category, .. }) = production.rhs.last() else {
                continue;
            };
            let last = production.rhs.len() - 1;
            if production.coordinator.is_none()
                && production.head.is_some_and(|head| head < last)
                && complements.get(category) == Some(&production.lhs)
            {
                production.auxiliary = Some(last);
            }
        }
        Ok(())
    }

    /// Closes child `name` of Production `id` to the prepositions of every
    /// selected `PP` Complement that the Production, or a sibling with the
    /// same children up to and including it, takes right after it.
    fn unextended(&mut self, id: Ident, name: &Ident) -> Result<(), LoadError> {
        let context = |e: &str| invalid(format!("production `{id}`: unextended `{name}` {e}"));
        let index = *self
            .out
            .by_id
            .get(&id)
            .ok_or_else(|| context("names no production"))?;
        let production = &self.out.productions[index];
        let at = production
            .names
            .iter()
            .position(|n| n == name)
            .ok_or_else(|| context("names no child"))?;
        let through = &production.rhs[..=at];
        let mut closed = BTreeSet::new();
        for &sibling in &self.out.by_lhs[production.lhs] {
            let sibling = &self.out.productions[sibling];
            if sibling.rhs.get(..=at) == Some(through)
                && let Some(crate::step::Symbol::Cat {
                    prepositions: Some(list),
                    ..
                }) = sibling.rhs.get(at + 1)
            {
                closed.extend(list.iter().copied());
            }
        }
        if closed.is_empty() {
            return Err(context(
                "is followed by no selected PP Complement, in its Production or a sibling",
            ));
        }
        self.out
            .unextended_prepositions
            .extend(closed.iter().copied());
        self.out.productions[index].unextended = Some((at, closed));
        Ok(())
    }

    fn authored(&self, declaration: &crate::grammar::ProductionDecl) -> Result<Draft, LoadError> {
        let context = |e: LoadError| invalid(format!("production `{}`: {e}", declaration.name));
        if declaration.rhs.is_empty() {
            return Err(context(invalid("empty right-hand side")));
        }
        let lhs = self.category(&declaration.lhs).map_err(context)?;
        let mut draft = Draft::new(declaration.name, lhs, declaration.cost);
        let mut required_gaps = BTreeMap::new();
        for equation in &declaration.equations {
            if let Equation::Require(path, value) = equation
                && let Some(child) = path.strip_suffix(".Gap")
            {
                let gap = gap_named(value.as_str())
                    .ok_or_else(|| context(invalid(format!("unknown gap `{value}`"))))?;
                required_gaps.insert(child.to_owned(), gap);
            }
        }
        // `$.Gap` names the Gap the Production introduces (a subject Gap).
        if let Some(gap) = required_gaps.remove("$") {
            draft.introduces = gap;
        }
        for symbol in &declaration.rhs {
            let (name, symbol) = self.symbol(symbol).map_err(context)?;
            let transparent = declaration.transparent.iter().any(|t| t == name);
            let gap = if let Some(gap) = required_gaps.remove(name) {
                // A transparent child with a required Gap raises it.
                if transparent { GapRule::Raise(gap) } else { GapRule::Require(gap) }
            } else if declaration.transparent.iter().any(|t| t == name) {
                GapRule::Transparent
            } else {
                GapRule::Closed
            };
            draft.child(name, symbol, gap);
        }
        if let Some(name) = required_gaps.keys().next() {
            return Err(context(invalid(format!(
                "gap requirement on unknown child `{name}`"
            ))));
        }
        for name in &declaration.transparent {
            if !draft.children.iter().any(|c| c.name == *name) {
                return Err(context(invalid(format!(
                    "unknown transparent child `{name}`"
                ))));
            }
        }
        for (names, weight) in [
            (&declaration.light, Weight::Light),
            (&declaration.heavy, Weight::Heavy),
        ] {
            for name in names {
                let child = draft
                    .children
                    .iter()
                    .position(|c| c.name == *name)
                    .ok_or_else(|| context(invalid(format!("unknown weighted child `{name}`"))))?;
                draft.weights.push((child, weight));
            }
        }
        for name in &declaration.undelimited {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name)
                .ok_or_else(|| context(invalid(format!("unknown undelimited child `{name}`"))))?;
            draft.undelimited.push(child);
        }
        for name in &declaration.uncoordinated {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name)
                .ok_or_else(|| context(invalid(format!("unknown uncoordinated child `{name}`"))))?;
            draft.uncoordinated.push(child);
        }
        for name in &declaration.scoped {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name && matches!(c.symbol, Symbol::Leaf(_)))
                .ok_or_else(|| {
                    context(invalid(format!(
                        "scoped child `{name}` is not a lexical child"
                    )))
                })?;
            draft.scoped.push(child);
        }
        self.equations(&mut draft, &declaration.equations)
            .map_err(context)?;
        Ok(draft)
    }

    /// Adds a Valence schema's `internalised` equations to a derived passive
    /// with an Internalised Complement, the child they call `internalised`.
    fn internalised_equations(
        &self,
        draft: &mut Draft,
        equations: &[Equation],
    ) -> Result<(), LoadError> {
        let Some(child) = draft.internalised else {
            return Ok(());
        };
        let name = std::mem::replace(&mut draft.children[child].name, Ident::new("internalised"));
        let result = self.equations(draft, equations);
        draft.children[child].name = name;
        result
    }

    /// Adds declared equations over a draft's children and mother. A `Gap`
    /// requirement is a child's Gap Rule, set by the caller. An equation on a
    /// child name that several children share constrains each of them that
    /// carries the feature.
    fn equations(&self, draft: &mut Draft, equations: &[Equation]) -> Result<(), LoadError> {
        for equation in equations {
            match equation {
                Equation::Exclude(path, value) => {
                    for (child, feature) in self.child_paths(draft, path, "Exclude")? {
                        draft
                            .exclusions
                            .push((child, feature, self.value(feature, value)?));
                    }
                }
                Equation::Carry(path, value) => {
                    for (child, feature) in self.child_paths(draft, path, "Carry")? {
                        draft
                            .carried
                            .push((child, feature, self.value(feature, value)?));
                    }
                }
                Equation::ExcludeTogether(conditions) => {
                    if conditions.len() < 2 {
                        return Err(invalid("ExcludeTogether needs two or more conditions"));
                    }
                    let mut joint = Vec::new();
                    for (path, value) in conditions {
                        let [(child, feature)] =
                            self.child_paths(draft, path, "ExcludeTogether")?[..]
                        else {
                            return Err(invalid(
                                "ExcludeTogether requires one child per condition",
                            ));
                        };
                        joint.push((child, feature, self.value(feature, value)?));
                    }
                    draft.joint_exclusions.push(joint);
                }
                Equation::Agree(a, b) => {
                    let a = self.paths(draft, a)?;
                    let b = self.paths(draft, b)?;
                    for &a in &a {
                        for &b in &b {
                            draft.equations.push((a, Target::Path(b)));
                        }
                    }
                }
                Equation::Require(path, _) if path.ends_with(".Gap") => {}
                Equation::Require(path, value) => {
                    for path in self.paths(draft, path)? {
                        let feature = match path {
                            Path::Mother(f) | Path::Child(_, f) => f,
                        };
                        let value = self.value(feature, value)?;
                        draft.equations.push((path, Target::Value(value)));
                    }
                }
            }
        }
        Ok(())
    }

    /// A right-hand-side declaration's child name and Symbol.
    fn symbol<'d>(&self, symbol: &'d SymbolDecl) -> Result<(&'d str, Symbol), LoadError> {
        Ok(match symbol {
            SymbolDecl::Cat(name, category) => (
                name.as_str(),
                Symbol::Cat {
                    binding: None,
                    category: self.category(category)?,
                    prepositions: None,
                    host: None,
                },
            ),
            SymbolDecl::Attached(name, category, binding) => (
                name.as_str(),
                Symbol::Cat {
                    category: self.category(category)?,
                    binding: Some(*binding),
                    prepositions: None,
                    host: None,
                },
            ),
            SymbolDecl::Free(name, category) => (
                name.as_str(),
                Symbol::Cat {
                    category: self.category(category)?,
                    binding: Some(Binding::Free),
                    prepositions: None,
                    host: None,
                },
            ),
            SymbolDecl::Numeral(name, notation) => {
                (name.as_str(), Symbol::Leaf(Test::Numeral(*notation)))
            }
            SymbolDecl::FreeLex(name, category) => {
                (name.as_str(), Symbol::Leaf(Test::FreeLexical(*category)))
            }
            SymbolDecl::Lex(name, category) => {
                (name.as_str(), Symbol::Leaf(Test::Lexical(*category)))
            }
            SymbolDecl::Word(name, word) => {
                (name.as_str(), Symbol::Leaf(Test::Word(self.word(word)?)))
            }
            SymbolDecl::Hosted(name, category, host) => (
                name.as_str(),
                Symbol::Cat {
                    binding: None,
                    category: self.category(category)?,
                    prepositions: None,
                    host: Some(*host),
                },
            ),
            SymbolDecl::Selected(name, category, words) => (
                name.as_str(),
                Symbol::Cat {
                    binding: None,
                    category: self.category(category)?,
                    prepositions: Some(
                        words
                            .iter()
                            .map(|w| self.word(w))
                            .collect::<Result<_, _>>()?,
                    ),
                    host: None,
                },
            ),
            SymbolDecl::Class(name, class) => (
                name.as_str(),
                Symbol::Leaf(Test::Class(
                    *self
                        .class_names
                        .get(class)
                        .ok_or_else(|| invalid(format!("unknown class `{class}`")))?,
                )),
            ),
            SymbolDecl::Mark(c, binding) => ("", Symbol::Punct(*c, *binding)),
            SymbolDecl::Punct(c) => ("", Symbol::Punct(*c, Binding::Suffix)),
        })
    }

    /// A child feature path's `(child, feature)` pairs.
    fn child_paths(
        &self,
        draft: &Draft,
        text: &str,
        equation: &str,
    ) -> Result<Vec<(usize, usize)>, LoadError> {
        self.paths(draft, text)?
            .into_iter()
            .map(|path| match path {
                Path::Child(child, feature) => Ok((child, feature)),
                Path::Mother(_) => Err(invalid(format!("{equation} requires a child feature"))),
            })
            .collect()
    }

    /// The mother's feature, or that feature on every child of the name that
    /// carries it.
    fn paths(&self, draft: &Draft, text: &str) -> Result<Vec<Path>, LoadError> {
        let (owner, feature) = text
            .split_once('.')
            .ok_or_else(|| invalid(format!("path `{text}` is not `owner.Feature`")))?;
        let feature = self.feature(feature)?;
        if owner == "$" {
            if !self.out.category_features[draft.lhs].contains(&feature) {
                return Err(invalid(format!("mother does not carry `{text}`")));
            }
            return Ok(vec![Path::Mother(feature)]);
        }
        let named: Vec<usize> = (0..draft.children.len())
            .filter(|&index| draft.children[index].name == owner)
            .collect();
        if named.is_empty() {
            return Err(invalid(format!("unknown child `{owner}`")));
        }
        let paths: Vec<Path> = named
            .into_iter()
            .filter(|&index| self.carries(&draft.children[index].symbol, feature))
            .map(|index| Path::Child(index, feature))
            .collect();
        if paths.is_empty() {
            return Err(invalid(format!("child `{owner}` does not carry `{text}`")));
        }
        Ok(paths)
    }

    fn carries(&self, symbol: &Symbol, feature: usize) -> bool {
        match symbol {
            Symbol::Cat { category, .. } => {
                self.out.category_features[*category].contains(&feature)
            }
            Symbol::Leaf(_) => true,
            Symbol::Punct(..) => false,
        }
    }

    /// Reports, as findings, a declared feature with fewer than two values
    /// and a feature value with fewer than two holders
    /// (`english-lexical-analysis.md` §3, `english-v4.md` §4.4). A value's
    /// holders are the lexemes whose feature bundles carry it and the
    /// Productions that set it on their mother; a value that has none is a
    /// default, held through the Productions that require it of a child
    /// leaving the feature unvalued. A value the engine projects
    /// from inflection is held by every word form inflected for it, and a
    /// feature equated with another shares that feature's holders of a value
    /// of the same name.
    fn report_holders(&mut self) {
        let out = &mut self.out;
        let mut lexical: BTreeMap<(usize, u16), BTreeSet<String>> = BTreeMap::new();
        for (id, facts) in &out.lexemes {
            for &(feature, value) in &facts.features {
                lexical
                    .entry((feature, value))
                    .or_default()
                    .insert(id.to_string());
            }
        }
        // Features equated with one another, closed transitively.
        let mut linked: Vec<BTreeSet<usize>> = (0..out.features.len())
            .map(|f| BTreeSet::from([f]))
            .collect();
        loop {
            let mut grew = false;
            for &(a, b) in &self.links {
                let union: BTreeSet<usize> = linked[a].union(&linked[b]).copied().collect();
                for &f in &union {
                    if linked[f] != union {
                        linked[f].clone_from(&union);
                        grew = true;
                    }
                }
            }
            if !grew {
                break;
            }
        }
        let projected = |feature: usize| -> &'static [&'static str] {
            if out.projected.contains(&Some(feature)) {
                crate::step::projected_values(&out.features[feature])
            } else {
                &[]
            }
        };
        let mut warnings = Vec::new();
        for (feature, name) in out.features.iter().enumerate() {
            let domain = &out.values[feature];
            if domain.len() < 2 {
                warnings.push(format!(
                    "feature `{name}` has {} value(s); a feature needs two or more",
                    domain.len()
                ));
            }
            for value_name in domain {
                // The same value under every feature equated with this one.
                let keys: Vec<(usize, u16)> = linked[feature]
                    .iter()
                    .filter_map(|&f| {
                        let v = out.values[f].iter().position(|v| v == value_name)?;
                        Some((f, u16::try_from(v).expect("small feature domain")))
                    })
                    .collect();
                if keys
                    .iter()
                    .any(|&(f, _)| projected(f).contains(&value_name.as_str()))
                {
                    continue;
                }
                let mut holders: BTreeSet<String> = keys
                    .iter()
                    .filter_map(|key| lexical.get(key))
                    .flatten()
                    .cloned()
                    .chain(
                        keys.iter()
                            .filter_map(|key| self.setters.get(key))
                            .flatten()
                            .map(ToString::to_string),
                    )
                    .collect();
                if holders.is_empty() {
                    holders = keys
                        .iter()
                        .filter_map(|key| self.valuers.get(key))
                        .flatten()
                        .map(ToString::to_string)
                        .collect();
                }
                if holders.len() < 2 {
                    warnings.push(format!(
                        "feature value `{name}` `{value_name}` has {} holder(s) ({}); a value \
                         needs two or more",
                        holders.len(),
                        holders.into_iter().collect::<Vec<_>>().join(", ")
                    ));
                }
            }
        }
        out.report.warnings.extend(warnings);
    }

    /// Resolves a draft's equations into variables and adds it.
    fn add(&mut self, mut draft: Draft) -> Result<(), LoadError> {
        let feature_of = |path: &Path| match *path {
            Path::Mother(f) | Path::Child(_, f) => f,
        };
        for (path, target) in &draft.equations {
            match target {
                Target::Value(value) => {
                    let valuers = match path {
                        Path::Mother(_) => &mut self.setters,
                        Path::Child(..) => &mut self.valuers,
                    };
                    valuers
                        .entry((feature_of(path), *value))
                        .or_default()
                        .insert(draft.id);
                }
                Target::Path(other) if feature_of(other) != feature_of(path) => {
                    self.links.insert((feature_of(path), feature_of(other)));
                }
                Target::Path(_) => {}
            }
        }
        for &(feature, value, _) in &draft.resolutions {
            self.setters
                .entry((feature, value))
                .or_default()
                .insert(draft.id);
        }
        if let Some(head) = draft.children.iter().position(|c| c.name == "head") {
            for &feature in &self.out.category_features[draft.lhs] {
                if self.carries(&draft.children[head].symbol, feature) {
                    draft.equations.push((
                        Path::Mother(feature),
                        Target::Path(Path::Child(head, feature)),
                    ));
                }
            }
        }
        let mut production = crate::step::resolve(
            &self.out,
            draft.id,
            draft.lhs,
            draft
                .children
                .into_iter()
                .map(|c| (c.name, c.symbol, c.gap))
                .collect(),
            draft
                .equations
                .into_iter()
                .map(|(path, target)| {
                    let key = |p: Path| match p {
                        Path::Mother(f) => (None, f),
                        Path::Child(i, f) => (Some(i), f),
                    };
                    (
                        key(path),
                        match target {
                            Target::Path(p) => Ok(key(p)),
                            Target::Value(v) => Err(v),
                        },
                    )
                })
                .collect(),
            draft.cost,
            draft.introduces,
            draft.gap_followed,
            draft.frame,
            draft.host_check,
            draft.coordinator,
            draft.distinct_coordinator,
            draft.requires_coordination,
        )
        .map_err(|e| invalid(format!("production `{}`: {e}", draft.id)))?;
        production.exclusions = draft.exclusions;
        let conditions: usize = draft.joint_exclusions.iter().map(Vec::len).sum();
        if conditions > crate::step::MAX_JOINT_CONDITIONS {
            return Err(invalid(format!(
                "production `{}`: {conditions} joint-exclusion conditions, more than {}",
                production.id,
                crate::step::MAX_JOINT_CONDITIONS
            )));
        }
        for joint in draft.joint_exclusions {
            production.joint_conditions.extend(joint);
            production
                .joint_exclusions
                .push(production.joint_conditions.len());
        }
        if draft.resolutions.len() > crate::step::MAX_RESOLUTIONS {
            return Err(invalid(format!(
                "production `{}` resolves {} features, more than {}",
                production.id,
                draft.resolutions.len(),
                crate::step::MAX_RESOLUTIONS
            )));
        }
        for (feature, differ, whole) in draft.resolutions {
            let at = self.out.category_features[production.lhs]
                .iter()
                .position(|f| *f == feature)
                .ok_or_else(|| {
                    invalid(format!(
                        "production `{}` resolves a feature its mother does not carry",
                        production.id
                    ))
                })?;
            if whole && production.mother_vars[at].is_some() {
                return Err(invalid(format!(
                    "production `{}` both resolves and binds a mother feature",
                    production.id
                )));
            }
            production
                .resolutions
                .push((feature, differ, whole.then_some(at)));
        }
        production.resolution_nonlast = draft.resolution_nonlast;
        production.resolution_last = draft.resolution_last;
        production.carried = draft.carried;
        production.weights = draft.weights;
        production.adjunct_slot = draft.adjunct_slot;
        production.predicative = draft.predicative;
        production.paired_coordinator = draft.paired_coordinator;
        production.uncoordinated = draft.uncoordinated;
        production.scoped = draft.scoped;
        production.undelimited = draft.undelimited;
        production.delimits = draft.delimits;
        production.chained = draft.chained;
        production.gap_preposition = draft.gap_preposition;
        production.fills_preposition = draft.fills_preposition;
        production
            .foreign_coordinators
            .clone_from(&draft.foreign_coordinators);
        production.unlike = draft.unlike;
        production.omits_complement = draft.omits_complement;
        if self.out.by_id.contains_key(&production.id) {
            return Err(invalid(format!(
                "duplicate production id `{}`",
                production.id
            )));
        }
        let index = self.out.productions.len();
        self.out.by_id.insert(production.id, index);
        self.out.by_lhs[production.lhs].push(index);
        self.out.productions.push(production);
        Ok(())
    }

    /// A Valence schema's declared derivations, validated: a `passive` or
    /// `adjectival` derivation needs its equations, and `complement_kind`
    /// names a feature the phrase carries.
    fn valence_derivations<'d>(
        &self,
        schema: &'d SchemaDecl,
    ) -> Result<Derivations<'d>, LoadError> {
        let SchemaDecl::Valence {
            phrase,
            complement_kind,
            active,
            passive,
            verbal_passive,
            prepositional_passive,
            adjectival,
            adjectival_passive,
            ..
        } = schema
        else {
            return Err(invalid("not a Valence schema"));
        };
        let lhs = self.category(phrase)?;
        let complement_kind = complement_kind
            .as_deref()
            .map(|name| {
                let feature = self.feature(name)?;
                if self.out.category_features[lhs].contains(&feature) {
                    Ok(feature)
                } else {
                    Err(invalid(format!(
                        "Valence complement kind: `{phrase}` does not carry `{name}`"
                    )))
                }
            })
            .transpose()?;
        let passive = passive
            .as_deref()
            .map(|by| {
                if verbal_passive.is_empty() || prepositional_passive.is_empty() {
                    Err(invalid(format!(
                        "passive schema on `{phrase}` needs `verbal_passive` and \
                         `prepositional_passive` equations"
                    )))
                } else {
                    Ok(PassiveDecl {
                        by,
                        verbal: verbal_passive,
                        prepositional: prepositional_passive,
                    })
                }
            })
            .transpose()?;
        let adjectival = adjectival
            .as_deref()
            .map(|category| {
                if adjectival_passive.is_empty() {
                    Err(invalid(format!(
                        "adjectival schema on `{phrase}` needs `adjectival_passive` \
                         equations"
                    )))
                } else {
                    Ok((category, adjectival_passive.as_slice()))
                }
            })
            .transpose()?;
        Ok(Derivations {
            complement_kind,
            active,
            passive,
            adjectival,
        })
    }

    fn schema(&mut self, schema: &SchemaDecl) -> Result<Vec<Draft>, LoadError> {
        match schema {
            SchemaDecl::Premodification {
                head,
                modifier,
                host,
                forms,
                prefix,
                equations,
            } => self.premodification(*head, modifier, host, forms, *prefix, equations),
            SchemaDecl::Parameters {
                head,
                phrase,
                binding,
                coordinators,
            } => self.parameters(*head, phrase, *binding, coordinators.as_deref()),
            SchemaDecl::Valence {
                head,
                phrase,
                cost,
                internalised,
                interleaved,
                heavy_shift,
                postposed,
                obligatory,
                agree,
                clusters,
                ..
            } => {
                let derivations = self.valence_derivations(schema)?;
                let mut drafts = self.valence(
                    *head,
                    phrase,
                    *cost,
                    &derivations,
                    *obligatory,
                    *heavy_shift,
                )?;
                for draft in &mut drafts {
                    self.internalised_equations(draft, internalised)?;
                }
                self.slot_agreement(phrase, &mut drafts, agree)?;
                if let Some(coordinators) = clusters {
                    let coordinator = self.coordinator_test(coordinators)?;
                    let between = interleaved
                        .as_deref()
                        .map(|i| self.category(i))
                        .transpose()?;
                    let clustered: Vec<Draft> = drafts
                        .iter()
                        .filter_map(|d| complement_clusters(d, &coordinator, None, false))
                        .chain(
                            drafts
                                .iter()
                                .filter_map(|d| complement_clusters(d, &coordinator, None, true)),
                        )
                        .chain(drafts.iter().filter_map(|d| {
                            between
                                .and_then(|b| complement_clusters(d, &coordinator, Some(b), false))
                        }))
                        .collect();
                    drafts.extend(clustered);
                }
                let postposed = postposed.as_deref().map(|p| self.category(p)).transpose()?;
                let mut ordered =
                    self.valence_order(drafts.clone(), interleaved.as_deref(), *heavy_shift)?;
                if let Some(modifier) = postposed {
                    ordered.extend(self.postposed_modifiers(&drafts, modifier));
                }
                Ok(ordered)
            }
            SchemaDecl::Adjunct {
                host,
                adjunct,
                side,
                host_kind,
                delimiter,
                cost,
                equations,
            } => {
                let host_id = self.category(host)?;
                let adjunct_id = self.category(adjunct)?;
                let mut draft = Draft::new(
                    format!(
                        "Adjunct:{host}+{adjunct}/{side:?}/{host_kind:?}{}",
                        delimiter.map_or(String::new(), |c| format!("/{c}"))
                    ),
                    host_id,
                    *cost,
                );
                let host_symbol = Symbol::Cat {
                    binding: None,
                    category: host_id,
                    prepositions: None,
                    host: None,
                };
                let adjunct_symbol = Symbol::Cat {
                    binding: None,
                    category: adjunct_id,
                    prepositions: None,
                    host: Some(*host_kind),
                };
                // CGEL: an unselected dependent of a nominal is a Modifier, of
                // a predicate an Adjunct.
                let function = match host_kind {
                    HostKind::Nominal => "modifier",
                    HostKind::Predicate => "adjunct",
                    HostKind::Supplement => "supplement",
                };
                let (h, a) = if *side == Side::After {
                    // A comma-set post-adjunct is outside any relative clause
                    // (CGEL, Ch. 12 §4, p. 1058): its host passes up no Gap.
                    let host_gap =
                        if delimiter.is_some() { GapRule::Closed } else { GapRule::Transparent };
                    let h = draft.child("head", host_symbol, host_gap);
                    if let Some(c) = delimiter {
                        draft.child("", Symbol::Punct(*c, Binding::Suffix), GapRule::Closed);
                    }
                    (h, draft.child(function, adjunct_symbol, GapRule::Closed))
                } else {
                    let a = draft.child(function, adjunct_symbol, GapRule::Closed);
                    if let Some(c) = delimiter {
                        draft.child("", Symbol::Punct(*c, Binding::Suffix), GapRule::Closed);
                    }
                    (draft.child("head", host_symbol, GapRule::Transparent), a)
                };
                draft.host_check = Some((h, a, *host_kind));
                if equations
                    .iter()
                    .any(|e| matches!(e, Equation::Require(p, _) if p.ends_with(".Gap")))
                {
                    return Err(invalid(format!(
                        "adjunct schema `{}`: a Gap requirement",
                        draft.id
                    )));
                }
                self.equations(&mut draft, equations)
                    .map_err(|e| invalid(format!("adjunct schema `{}`: {e}", draft.id)))?;
                draft.adjunct_slot = self
                    .grammar
                    .slot_categories
                    .iter()
                    .find(|(kind, name)| *name == adjunct && **kind != SlotKind::PP)
                    .map(|(kind, _)| *kind);
                Ok(vec![draft])
            }
            SchemaDecl::Coordination { .. } => self.coordination(schema),
        }
    }

    /// The phrases of `drafts` with a Modifier of their Object postposed
    /// after its selected Preposition Phrase Complements
    /// ([`postposed_modifier`]). The Modifier agrees with the Object in each
    /// feature they both carry, as a postposed relative clause does.
    fn postposed_modifiers(&self, drafts: &[Draft], modifier: usize) -> Vec<Draft> {
        let shared = &self.out.category_features[modifier];
        drafts
            .iter()
            .filter_map(|d| postposed_modifier(d, modifier))
            .map(|mut draft| {
                let last = draft.children.len() - 1;
                for &feature in shared {
                    if self.carries(&draft.children[1].symbol, feature) {
                        draft.equations.push((
                            Path::Child(1, feature),
                            Target::Path(Path::Child(last, feature)),
                        ));
                    }
                }
                draft
            })
            .collect()
    }

    /// Adds a Valence Schema's `agree` equations: each `(phrase, slot)`
    /// feature pair makes the phrase's feature agree with the slot feature
    /// of every Frame Slot child that carries it.
    fn slot_agreement(
        &self,
        phrase: &str,
        drafts: &mut [Draft],
        agree: &[(Ident, Ident)],
    ) -> Result<(), LoadError> {
        for (mother, slot) in agree {
            let mother = self.feature(mother)?;
            let slot = self.feature(slot)?;
            let lhs = self.category(phrase)?;
            if !self.out.category_features[lhs].contains(&mother) {
                return Err(invalid(format!(
                    "Valence agreement: `{phrase}` does not carry `{}`",
                    self.out.features[mother]
                )));
            }
            for draft in drafts.iter_mut() {
                for (i, child) in draft.children.iter().enumerate() {
                    if child.name != "head"
                        && matches!(child.symbol, Symbol::Cat { .. })
                        && self.carries(&child.symbol, slot)
                    {
                        draft
                            .equations
                            .push((Path::Mother(mother), Target::Path(Path::Child(i, slot))));
                    }
                }
            }
        }
        Ok(())
    }

    fn premodification(
        &mut self,
        head: LexicalCategory,
        modifier: &str,
        host: &str,
        forms: &[deckmaste_lexical::WordForm],
        prefix: bool,
        equations: &[(deckmaste_lexical::WordForm, Vec<Equation>)],
    ) -> Result<Vec<Draft>, LoadError> {
        let modifier_id = self.category(modifier)?;
        let host_id = self.category(host)?;
        let symbol = |category| Symbol::Cat {
            category,
            binding: None,
            prepositions: None,
            host: None,
        };
        let mut drafts = Vec::new();
        for form in forms {
            let mut leaf = Draft::new(
                format!("Premodification:{host}/{head:?}/{form:?}"),
                modifier_id,
                0,
            );
            // An attributive VP takes no post-head dependent (CGEL, Ch. 5
            // §14.2, p. 444). A passive past participle's Object is the
            // nominal it modifies, but an active gerund-participle realizes no
            // Complement, so only a verb whose Complement is optional
            // premodifies with one (*attacking creature*, never *declaring
            // blockers* as "blockers that declare"). An Active-only verb has
            // no passive participle, so none of its forms premodifies as one
            // (CGEL, Ch. 16 §10.1.2, p. 1432).
            let past = *form == deckmaste_lexical::WordForm::PastParticiple;
            let test = if past { Test::FreeNotActiveOnly(head) } else { Test::FreeOptional(head) };
            let child = leaf.child("head", Symbol::Leaf(test), GapRule::Closed);
            for (_, declared) in equations.iter().filter(|(f, _)| f == form) {
                self.equations(&mut leaf, declared)?;
            }
            // The leaf is of this Word Form: the lexicon feature-bundle
            // mapping table projects it.
            let inflection = self.feature(PROJECTED[INFLECTION])?;
            leaf.equations.push((
                Path::Child(child, inflection),
                Target::Value(self.value(inflection, &crate::step::inflection_name(*form))?),
            ));
            drafts.push(leaf);
        }
        if let Some((form, _)) = equations.iter().find(|(f, _)| !forms.contains(f)) {
            return Err(invalid(format!(
                "Premodification equations name {form:?}, which is not one of its forms"
            )));
        }
        if prefix {
            let mut affixed = Draft::new(
                format!("Premodification:{host}/{head:?}/prefix"),
                modifier_id,
                0,
            );
            affixed.child(
                "prefix",
                Symbol::Leaf(Test::Lexical(LexicalCategory::Affix)),
                GapRule::Closed,
            );
            affixed.child("head", symbol(modifier_id), GapRule::Closed);
            drafts.push(affixed);
        }
        let mut attachment = Draft::new(
            format!("Premodification:{host}/{head:?}/attach"),
            host_id,
            1,
        );
        attachment.child("modifier", symbol(modifier_id), GapRule::Closed);
        // Like an adjective, a participle premodifies a conjunct whose
        // Complement is shared (*the noted number and kind of counters*).
        attachment.child("head", symbol(host_id), GapRule::Transparent);
        drafts.push(attachment);
        Ok(drafts)
    }

    fn private_category(&mut self, target: usize) -> usize {
        let category = self.out.categories.len();
        self.out.categories.push(self.out.categories[target]);
        self.out
            .category_features
            .push(self.out.category_features[target].clone());
        self.out.category_gap.push(self.out.category_gap[target]);
        self.out.by_lhs.push(Vec::new());
        category
    }

    fn parameters(
        &mut self,
        head: LexicalCategory,
        phrase: &str,
        binding: Option<Binding>,
        coordinators: Option<&str>,
    ) -> Result<Vec<Draft>, LoadError> {
        use crate::frames::ParameterPart;
        let coordinator = coordinators.map(|c| self.coordinator_test(c)).transpose()?;
        let lhs = self.category(phrase)?;
        let mut drafts = Vec::new();
        for (id, patterns) in &self.catalog.parameters {
            let facts =
                self.out.lexemes.get(id).ok_or_else(|| {
                    invalid(format!("parameter pattern names unknown lexeme `{id}`"))
                })?;
            if facts.category != head || binding.is_some_and(|b| b != facts.binding) {
                continue;
            }
            for (index, parts) in patterns.iter().enumerate() {
                if parts
                    .iter()
                    .filter(|p| matches!(p, ParameterPart::Head))
                    .count()
                    != 1
                {
                    return Err(invalid(format!(
                        "parameter pattern of `{id}` needs one head"
                    )));
                }
                let mut draft = Draft::new(format!("Parameters:{phrase}/{id}/{index}"), lhs, 0);
                for part in parts {
                    let (name, symbol) = match part {
                        ParameterPart::Feature(feature, value) => {
                            let feature = self.feature(feature)?;
                            if !self.out.category_features[lhs].contains(&feature) {
                                return Err(invalid(format!(
                                    "`{phrase}` does not carry the feature a pattern of `{id}` values"
                                )));
                            }
                            let value = self.value(feature, value)?;
                            draft
                                .equations
                                .push((Path::Mother(feature), Target::Value(value)));
                            continue;
                        }
                        ParameterPart::Head => ("head", Symbol::Leaf(Test::Word(*id))),
                        ParameterPart::Slot(kind) => (
                            "complement",
                            Symbol::Cat {
                                binding: None,
                                category: self.slot_category(*kind)?,
                                prepositions: None,
                                host: None,
                            },
                        ),
                        ParameterPart::Word(word) => {
                            if !self.out.lexemes.contains_key(word) {
                                return Err(invalid(format!("unknown parameter marker `{word}`")));
                            }
                            ("marker", Symbol::Leaf(Test::Word(*word)))
                        }
                        ParameterPart::Words(words) => {
                            if words.is_empty()
                                || words
                                    .iter()
                                    .any(|word| !self.out.lexemes.contains_key(word))
                            {
                                return Err(invalid(format!(
                                    "invalid parameter word class of `{id}`"
                                )));
                            }
                            (
                                "complement",
                                Symbol::Leaf(Test::Words(words.iter().copied().collect())),
                            )
                        }
                        ParameterPart::ModifiedWords(words, modifier, target) => {
                            if words.is_empty()
                                || words
                                    .iter()
                                    .any(|word| !self.out.lexemes.contains_key(word))
                            {
                                return Err(invalid(format!(
                                    "invalid modified word class of `{id}`"
                                )));
                            }
                            let target = self.slot_category(*target)?;
                            let modifier = self.slot_category(*modifier)?;
                            let category = self.private_category(target);
                            let mut base = Draft::new(format!("{}/Stem", draft.id), category, 0);
                            base.child(
                                "head",
                                Symbol::Leaf(Test::Words(words.iter().copied().collect())),
                                GapRule::Closed,
                            );
                            drafts.push(base);
                            let mut recursive =
                                Draft::new(format!("{}/ModifiedStem", draft.id), category, 0);
                            for (name, category) in [("modifier", modifier), ("head", category)] {
                                recursive.child(
                                    name,
                                    Symbol::Cat {
                                        category,
                                        binding: None,
                                        prepositions: None,
                                        host: None,
                                    },
                                    GapRule::Closed,
                                );
                            }
                            drafts.push(recursive);
                            (
                                "complement",
                                Symbol::Cat {
                                    category,
                                    binding: None,
                                    prepositions: None,
                                    host: None,
                                },
                            )
                        }
                        ParameterPart::Mark(c, binding) => ("", Symbol::Punct(*c, *binding)),
                    };
                    draft.child(name, symbol, GapRule::Closed);
                }
                let structural: Vec<&ParameterPart> = parts
                    .iter()
                    .filter(|part| !matches!(part, ParameterPart::Feature(..)))
                    .collect();
                if let (Some(coordinator), [.., ParameterPart::Word(_), ParameterPart::Slot(_)]) =
                    (&coordinator, structural.as_slice())
                {
                    drafts.extend(self.coordinated_parameter(&draft, coordinator));
                }
                drafts.push(draft);
            }
        }
        Ok(drafts)
    }

    /// A pattern's final marker and parameter, coordinated: binary (*from
    /// black and from red*) and serial (*from X, from Y, and from Z*).
    fn coordinated_parameter(&mut self, draft: &Draft, coordinator: &Test) -> Vec<Draft> {
        let marked: Vec<Child> = draft.children[draft.children.len() - 2..].to_vec();
        let push_marked = |d: &mut Draft| {
            for c in &marked {
                d.child(c.name.as_ref(), c.symbol.clone(), GapRule::Closed);
            }
        };
        let mut binary = draft.clone();
        binary.id = Ident::new(&format!("{}/coordinated", draft.id));
        binary.child(
            "coordinator",
            Symbol::Leaf(coordinator.clone()),
            GapRule::Closed,
        );
        push_marked(&mut binary);
        let tail = self.private_category(draft.lhs);
        let tail_symbol = Symbol::Cat {
            category: tail,
            binding: None,
            prepositions: None,
            host: None,
        };
        let mut serial = draft.clone();
        serial.id = Ident::new(&format!("{}/serial", draft.id));
        serial.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        serial.child("rest", tail_symbol.clone(), GapRule::Closed);
        let mut more = Draft::new(format!("{}/serial-more", draft.id), tail, 0);
        push_marked(&mut more);
        more.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        more.child("rest", tail_symbol, GapRule::Closed);
        let mut end = Draft::new(format!("{}/serial-end", draft.id), tail, 0);
        push_marked(&mut end);
        end.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        end.child(
            "coordinator",
            Symbol::Leaf(coordinator.clone()),
            GapRule::Closed,
        );
        push_marked(&mut end);
        vec![binary, serial, more, end]
    }

    fn valence(
        &self,
        head: LexicalCategory,
        phrase: &str,
        cost: u32,
        derivations: &Derivations<'_>,
        obligatory: bool,
        heavy_shift: bool,
    ) -> Result<Vec<Draft>, LoadError> {
        let lhs = self.category(phrase)?;
        let gap_capable = self.out.category_gap[lhs] != 0;
        let mut drafts = Vec::new();
        let complement_kind = derivations.complement_kind;
        if head == LexicalCategory::Preposition {
            return self.preposition_valence(lhs, phrase, cost, gap_capable, complement_kind);
        }
        let used = self.used_frames(head)?;
        if head != LexicalCategory::Verb {
            let mut bare = Draft::new(format!("Valence:{phrase}/Bare"), lhs, cost);
            let test = if obligatory { Test::FreeOptional(head) } else { Test::FreeLexical(head) };
            bare.child("head", Symbol::Leaf(test), GapRule::Closed);
            // A bare noun omits the Complements its frames select: a
            // Preposition Phrase right after it that one of them selects is
            // its Complement, never another host's dependent (CGEL, Ch. 5
            // §14.1, pp. 439–441; §15, p. 454).
            bare.omits_complement = head == LexicalCategory::Noun;
            if let Some(f) = complement_kind {
                let kind = self.value(f, &format!("{:?}", SlotFill::Intransitive.kind()))?;
                bare.equations.push((Path::Mother(f), Target::Value(kind)));
            }
            drafts.push(bare);
        }
        for index in used {
            let frame = &self.out.frames[index];
            // A non-verbal head's Intransitive frame only makes its
            // Complement optional: the bare phrase realizes it.
            if head != LexicalCategory::Verb && frame.family == FrameFamily::Intransitive {
                continue;
            }
            let name = frame_name(phrase, frame);
            let slots: Vec<&SlotFill> = frame.slots.iter().map(|s| &s.fill).collect();
            // A verb's NP-like Complements are extracted; a noun's single
            // *of*-like Preposition Phrase Complement is shared by Right Node
            // Raising (*the top or bottom of their library*).
            let gappable = gap_capable
                && (head == LexicalCategory::Verb || frame.family == FrameFamily::NounComplement);
            // Active, with every NP slot optionally extracted.
            let mut family = self.valence_family(
                lhs,
                &name,
                index,
                cost,
                &slots,
                0,
                false,
                derivations.active,
                gappable,
                heavy_shift,
            )?;
            if let (Some(f), Some(first)) = (complement_kind, slots.first()) {
                let kind = self.value(f, &format!("{:?}", first.kind()))?;
                for draft in &mut family {
                    draft.equations.push((Path::Mother(f), Target::Value(kind)));
                }
            }
            drafts.extend(family);
            // A frame restricted to active voice derives no passive of any
            // kind: verbal, prepositional or adjectival.
            let passive = derivations.passive.as_ref().filter(|_| !frame.active_only);
            if let (Some(passive), true) = (passive, head == LexicalCategory::Verb)
                && matches!(slots.first(), Some(SlotFill::NP))
            {
                // The Internalised Complement is a `by` Preposition Phrase.
                let by_pp = SlotFill::PP(vec![self.word(passive.by)?]);
                for agent in [false, true] {
                    let mut rest: Vec<&SlotFill> = slots[1..].to_vec();
                    if agent {
                        rest.push(&by_pp);
                    }
                    let tag = if agent { "passive+by" } else { "passive" };
                    drafts.extend(self.valence_family(
                        lhs,
                        &format!("{name}/{tag}"),
                        index,
                        cost,
                        &rest,
                        1,
                        agent,
                        passive.verbal,
                        gappable,
                        heavy_shift,
                    )?);
                }
                if let Some((adjectival, equations)) = derivations.adjectival
                    && slots.len() == 1
                {
                    drafts.extend(
                        self.adjectival_passive(adjectival, &name, index, cost, equations, &by_pp)?,
                    );
                }
            }
            if let (Some(passive), true) = (passive, head == LexicalCategory::Verb)
                && frame.family == FrameFamily::Prepositional
                && let [fill @ SlotFill::PP(_)] = slots.as_slice()
            {
                drafts.extend(self.prepositional_passive(lhs, &name, index, cost, fill, passive)?);
            }
        }
        Ok(drafts)
    }

    /// The prepositional passive of a Prepositional frame, as in *each card
    /// looked at*: the preposition's object is the passive's subject, so the
    /// Preposition Phrase is stranded (CGEL, Ch. 4 §6.1.2, pp. 276–277;
    /// Ch. 16 §10.1.2, pp. 1432–1433).
    fn prepositional_passive(
        &self,
        lhs: usize,
        name: &str,
        frame: usize,
        cost: u32,
        fill: &SlotFill,
        passive: &PassiveDecl<'_>,
    ) -> Result<Vec<Draft>, LoadError> {
        let by_pp = SlotFill::PP(vec![self.word(passive.by)?]);
        let relation = self.out.frames[frame].slots[0].relation;
        let mut drafts = Vec::new();
        for agent in [false, true] {
            let tag = if agent { "passive+by" } else { "passive" };
            let mut draft = Draft::new(format!("{name}/{tag}"), lhs, cost);
            draft.frame = Some(frame);
            draft.child("head", Symbol::Leaf(Test::Frame(frame)), GapRule::Closed);
            let child = draft.child(
                relation_name(relation),
                self.slot_symbol(fill)?,
                GapRule::Require(Gap::NP),
            );
            self.slot_equations(&mut draft, child, fill.kind())?;
            if agent {
                draft.internalised =
                    Some(draft.child("complement", self.slot_symbol(&by_pp)?, GapRule::Closed));
            }
            self.equations(&mut draft, passive.prepositional)?;
            drafts.push(draft);
        }
        Ok(drafts)
    }

    /// A phrase per used Preposition Frame and, when `gap_capable`, a
    /// stranded variant of each with an NP Complement.
    fn preposition_valence(
        &self,
        lhs: usize,
        phrase: &str,
        cost: u32,
        gap_capable: bool,
        complement_kind: Option<usize>,
    ) -> Result<Vec<Draft>, LoadError> {
        let mut drafts = Vec::new();
        let used: BTreeSet<usize> = self
            .out
            .lexemes
            .values()
            .flat_map(|f| f.preposition_frames.iter().copied())
            .collect();
        for index in used {
            let frame = &self.out.preposition_frames[index];
            let mut draft = Draft::new(
                format!(
                    "Valence:{phrase}/{:?}@{:?}{}",
                    frame.complement,
                    frame.hosts,
                    if frame.relation == SlotRelation::Complement { "" } else { "/predicative" }
                ),
                lhs,
                cost,
            );
            draft.child(
                "head",
                Symbol::Leaf(Test::PrepositionFrame(index)),
                GapRule::Closed,
            );
            if let Some(f) = complement_kind {
                let kind = self.value(f, &format!("{:?}", frame.complement.kind()))?;
                draft.equations.push((Path::Mother(f), Target::Value(kind)));
            }
            if frame.complement != SlotFill::Intransitive {
                let child = draft.child(
                    relation_name(frame.relation),
                    self.slot_symbol(&frame.complement)?,
                    GapRule::Transparent,
                );
                self.slot_equations(&mut draft, child, frame.complement.kind())?;
            }
            drafts.push(draft);
            if gap_capable && matches!(frame.complement, SlotFill::NP | SlotFill::Amount) {
                let amount = frame.complement == SlotFill::Amount;
                let mut stranded = Draft::new(
                    format!(
                        "Valence:{phrase}/{:?}@{:?}{}/gap0",
                        frame.complement,
                        frame.hosts,
                        if frame.relation == SlotRelation::Complement {
                            ""
                        } else {
                            "/predicative"
                        }
                    ),
                    lhs,
                    cost,
                );
                stranded.child(
                    "head",
                    Symbol::Leaf(Test::PrepositionFrame(index)),
                    GapRule::Closed,
                );
                stranded.introduces = if amount { Gap::Amount } else { Gap::NP };
                // It names the Slot Kind of the Complement it strands.
                if let Some(f) = complement_kind {
                    let kind = self.value(f, &format!("{:?}", frame.complement.kind()))?;
                    stranded
                        .equations
                        .push((Path::Mother(f), Target::Value(kind)));
                }
                drafts.push(stranded);
            }
        }
        Ok(drafts)
    }

    /// The frames of every lexeme of `head`'s Category. A verb without one
    /// must be a Closed Word or a Lexical Class member the grammar names.
    fn used_frames(&self, head: LexicalCategory) -> Result<BTreeSet<usize>, LoadError> {
        let mut used = BTreeSet::new();
        for (id, facts) in &self.out.lexemes {
            if facts.category != head {
                continue;
            }
            if facts.frames.is_empty()
                && head == LexicalCategory::Verb
                && !self.closed_words.values().any(|w| w == id)
                && !self
                    .grammar
                    .lexical_classes
                    .values()
                    .flatten()
                    .any(|m| LexemeId::new(m) == *id)
            {
                return Err(invalid(format!(
                    "verb `{id}` has no Subcategorization Frame"
                )));
            }
            used.extend(facts.frames.iter().copied());
        }
        Ok(used)
    }

    /// The Adjectival Passives of a frame whose passive leaves no Complement:
    /// the participle, with or without its `by` phrase, each with the
    /// schema's `adjectival_passive` equations.
    fn adjectival_passive(
        &self,
        category: &str,
        name: &str,
        frame: usize,
        cost: u32,
        equations: &[Equation],
        by_pp: &SlotFill,
    ) -> Result<Vec<Draft>, LoadError> {
        let lhs = self.category(category)?;
        let mut drafts = Vec::new();
        for agent in [false, true] {
            let tag = if agent { "adjectival+by" } else { "adjectival" };
            let mut draft = Draft::new(format!("{name}/{tag}"), lhs, cost);
            draft.frame = Some(frame);
            draft.child("head", Symbol::Leaf(Test::Frame(frame)), GapRule::Closed);
            if agent {
                draft.internalised =
                    Some(draft.child("complement", self.slot_symbol(by_pp)?, GapRule::Closed));
            }
            self.equations(&mut draft, equations)?;
            drafts.push(draft);
        }
        Ok(drafts)
    }

    /// One frame's phrase and, when `gappable`, a variant per extracted NP
    /// slot. `slots` are the frame's slots from `first_slot` on (a passive
    /// drops the Object), plus a trailing `by` Preposition Phrase, the
    /// Internalised Complement, when `agent` is set. `heavy_shift` is the
    /// schema's (`valence_order`).
    #[expect(clippy::too_many_arguments, reason = "one internal call shape")]
    fn valence_family(
        &self,
        lhs: usize,
        name: &str,
        frame: usize,
        cost: u32,
        slots: &[&SlotFill],
        first_slot: usize,
        agent: bool,
        equations: &[Equation],
        gappable: bool,
        heavy_shift: bool,
    ) -> Result<Vec<Draft>, LoadError> {
        let mut drafts = Vec::new();
        // Only a subjective predicative relativises (CGEL, Ch. 12 §3.3(c),
        // p. 1045): never the objective one of *turn it face up*.
        let copular = self.out.frames[frame].family == FrameFamily::Copular;
        let noun = self.out.frames[frame].family == FrameFamily::NounComplement;
        let catenative = self.out.frames[frame].family == FrameFamily::Catenative;
        let extractable: Vec<Option<usize>> = std::iter::once(None)
            .chain(
                slots
                    .iter()
                    .enumerate()
                    .filter(|(_, fill)| {
                        gappable
                            && if noun {
                                matches!(***fill, SlotFill::PP(ref list) if list.len() == 1)
                                    || ***fill == SlotFill::Amount
                            } else {
                                matches!(
                                    ***fill,
                                    SlotFill::NP | SlotFill::KeywordPhrase | SlotFill::ToInf
                                ) || (copular && ***fill == SlotFill::Pred)
                                    // An auxiliary's bare infinitival, shared
                                    // by Right Node Raising (*may and can
                                    // draw cards*).
                                    || (catenative && ***fill == SlotFill::BareInf)
                            }
                    })
                    .map(|(i, _)| Some(i)),
            )
            .collect();
        let agent_slot = agent.then(|| slots.len() - 1);
        for gap in extractable {
            if gap.is_some() && gap == agent_slot {
                continue;
            }
            let id = match gap {
                None => name.to_owned(),
                Some(i) => format!("{name}/gap{i}"),
            };
            let mut draft = Draft::new(id, lhs, cost);
            draft.frame = Some(frame);
            // A reduced auxiliary clitic (*'s*, *'re*, *'ve*) never stands
            // immediately before a Gap (CGEL, Ch. 18 §6.2.1, p. 1615): an
            // extracting head is a free word.
            let test = if gap.is_some() { Test::FreeFrame(frame) } else { Test::Frame(frame) };
            draft.child("head", Symbol::Leaf(test), GapRule::Closed);
            for (i, fill) in slots.iter().enumerate() {
                if Some(i) == gap {
                    continue;
                }
                // The Internalised Complement strands its preposition too
                // (*the creature that damage was dealt by*).
                if Some(i) == agent_slot {
                    draft.internalised = Some(draft.child(
                        "complement",
                        self.slot_symbol(fill)?,
                        GapRule::Transparent,
                    ));
                    continue;
                }
                let relation = self.out.frames[frame].slots[first_slot + i].relation;
                let child = draft.child(
                    relation_name(relation),
                    self.slot_symbol(fill)?,
                    GapRule::Transparent,
                );
                self.slot_equations(&mut draft, child, fill.kind())?;
                if **fill == SlotFill::Pred {
                    draft.predicative = Some(child);
                }
            }
            if let Some(i) = gap {
                if let SlotFill::PP(list) = slots[i] {
                    draft.gap_preposition = list.first().copied();
                }
                draft.introduces = match slots[i] {
                    SlotFill::PP(_) => Gap::PP,
                    SlotFill::Amount => Gap::Measure,
                    SlotFill::BareInf => Gap::BareInf,
                    SlotFill::Pred => Gap::Pred,
                    SlotFill::KeywordPhrase => Gap::Keyword,
                    SlotFill::ToInf => Gap::ToInf,
                    _ => Gap::NP,
                };
                // The Gap stands in its slot's place: every later slot,
                // the Internalised Complement included, follows it (*named
                // __ Thinking Cap*). An Object Gap before only Preposition
                // Phrase Complements may stand in the shifted Object's place
                // after them instead (`valence_order`): *[put __ into your
                // hand] or [exile __] the card*.
                let later = &slots[i + 1..];
                let shiftable = heavy_shift
                    && self.out.frames[frame].slots[first_slot + i].relation
                        == SlotRelation::Object
                    && later.iter().all(|fill| matches!(**fill, SlotFill::PP(_)));
                draft.gap_followed = !later.is_empty() && !shiftable;
            }
            self.equations(&mut draft, equations)?;
            drafts.push(draft);
        }
        Ok(drafts)
    }

    fn valence_order(
        &mut self,
        drafts: Vec<Draft>,
        adjunct: Option<&str>,
        heavy: bool,
    ) -> Result<Vec<Draft>, LoadError> {
        let adjunct = adjunct.map(|name| self.category(name)).transpose()?;
        let mut ordered = Vec::new();
        for draft in drafts {
            if heavy
                && draft.children.len() > 2
                && draft.children[1].name == "object"
                && draft.children[2..].iter().all(|c| {
                    matches!(
                        c.symbol,
                        Symbol::Cat {
                            prepositions: Some(_),
                            ..
                        }
                    )
                })
            {
                let mut shifted = draft.clone();
                shifted.id = Ident::new(&format!("{}/shifted-object", shifted.id));
                let object = shifted.children.remove(1);
                shifted.children.push(object);
                // The postposed Object follows any Gap.
                shifted.gap_followed = shifted.introduces != Gap::None;
                let last = shifted.children.len() - 1;
                let map = |i| match i {
                    0 => 0,
                    1 => last,
                    _ => i - 1,
                };
                remap_draft(&mut shifted, map);
                ordered.push(shifted);
            }
            ordered.push(draft);
        }
        let Some(adjunct) = adjunct else { return Ok(ordered) };
        let mut out = Vec::new();
        for draft in ordered {
            for split in 1..draft.children.len() {
                if !(matches!(
                    draft.children[split].symbol,
                    Symbol::Cat {
                        prepositions: Some(_),
                        ..
                    }
                ) || (draft.id.ends_with("/shifted-object")
                    && draft.children[split].name == "object"))
                {
                    continue;
                }
                let category = self.out.categories.len();
                self.out.categories.push(self.out.categories[draft.lhs]);
                self.out
                    .category_features
                    .push(self.out.category_features[draft.lhs].clone());
                self.out.category_gap.push(self.out.category_gap[draft.lhs]);
                self.out.by_lhs.push(Vec::new());
                let symbol = |category, host| Symbol::Cat {
                    category,
                    host,
                    binding: None,
                    prepositions: None,
                };
                let mut prefix = draft.clone();
                prefix.id = Ident::new(&format!("{}/prefix{split}", draft.id));
                prefix.lhs = category;
                prefix.introduces = Gap::None;
                prefix.children.truncate(split);
                prefix.equations.retain(|(a, b)| {
                    path_before(*a, split)
                        && match b {
                            Target::Path(p) => path_before(*p, split),
                            Target::Value(_) => true,
                        }
                });
                prefix.exclusions.retain(|(i, _, _)| *i < split);
                prefix
                    .joint_exclusions
                    .retain(|joint| joint.iter().all(|(i, _, _)| *i < split));
                prefix.carried.retain(|(i, _, _)| *i < split);
                prefix.predicative = prefix.predicative.filter(|&i| i < split);
                let a = prefix.child(
                    "adjunct",
                    symbol(adjunct, Some(HostKind::Predicate)),
                    GapRule::Closed,
                );
                prefix.host_check = Some((0, a, HostKind::Predicate));
                prefix.cost += 2;
                out.push(prefix);
                let mut more = Draft::new(format!("{}/prefix{split}/more", draft.id), category, 2);
                more.child("head", symbol(category, None), GapRule::Transparent);
                more.child(
                    "adjunct",
                    symbol(adjunct, Some(HostKind::Predicate)),
                    GapRule::Closed,
                );
                more.host_check = Some((0, 1, HostKind::Predicate));
                out.push(more);
                let mut suffix = draft.clone();
                suffix.id = Ident::new(&format!("{}/interleaved{split}", draft.id));
                suffix.children.drain(..split);
                suffix.children.insert(
                    0,
                    Child {
                        name: "head".into(),
                        symbol: symbol(category, None),
                        gap: GapRule::Transparent,
                    },
                );
                suffix.equations.retain(|(a, b)| {
                    path_after(*a, split)
                        && match b {
                            Target::Path(p) => path_after(*p, split),
                            Target::Value(_) => true,
                        }
                });
                suffix.exclusions.retain(|(i, _, _)| *i >= split);
                suffix
                    .joint_exclusions
                    .retain(|joint| joint.iter().all(|(i, _, _)| *i >= split));
                suffix.carried.retain(|(i, _, _)| *i >= split);
                suffix.predicative = suffix.predicative.filter(|&i| i >= split);
                remap_draft(&mut suffix, |i| i - split + 1);
                out.push(suffix);
            }
            out.push(draft);
        }
        Ok(out)
    }

    /// Adds the grammar's `slot_equations` for a Frame Slot of this kind to
    /// the draft's child that fills it, the child they call `slot`.
    fn slot_equations(
        &self,
        draft: &mut Draft,
        child: usize,
        kind: SlotKind,
    ) -> Result<(), LoadError> {
        let Some(equations) = self.grammar.slot_equations.get(&kind) else {
            return Ok(());
        };
        let name = std::mem::replace(&mut draft.children[child].name, Ident::new("slot"));
        let result = self.equations(draft, equations);
        draft.children[child].name = name;
        result
    }

    fn slot_symbol(&self, fill: &SlotFill) -> Result<Symbol, LoadError> {
        Ok(Symbol::Cat {
            binding: None,
            category: self.slot_category(fill.kind())?,
            prepositions: match fill {
                SlotFill::PP(list) => Some(list.clone()),
                _ => None,
            },
            host: None,
        })
    }

    /// Right Node Raising: a coordination with a shared Gap, filled once
    /// after it.
    fn right_node_raising(&self, conjoin: &Conjoin, category: &str, cost: u32) -> Vec<Draft> {
        let mut drafts = Vec::new();
        // One per shareable Gap: an NP, a keyword phrase, a
        // *to*-infinitival, a predicative (*be and become legendary*).
        for (gap, kind, suffix) in [
            (Gap::NP, SlotKind::NP, ""),
            (Gap::Keyword, SlotKind::KeywordPhrase, "-keyword"),
            (Gap::ToInf, SlotKind::ToInf, "-toinf"),
            (Gap::Pred, SlotKind::Pred, "-pred"),
            (Gap::PP, SlotKind::PP, "-pp"),
            (Gap::Measure, SlotKind::Measure, "-measure"),
            // One Amount for every noun's Amount Complement (*power or
            // toughness 2 or less*).
            (Gap::Measure, SlotKind::Amount, "-measure-amount"),
            (Gap::BareInf, SlotKind::BareInf, "-bareinf"),
            (Gap::Amount, SlotKind::Amount, "-amount"),
        ] {
            let Ok(shared) = self.slot_category(kind) else {
                continue;
            };
            let mut raising = Draft::new(
                format!("Coordination:{category}/shared{suffix}"),
                conjoin.lhs,
                cost,
            );
            let h = raising.child("head", conjoin.conjunct(), GapRule::Require(gap));
            let shared_child = raising.child(
                "shared",
                Symbol::Cat {
                    binding: None,
                    category: shared,
                    prepositions: None,
                    host: None,
                },
                GapRule::Closed,
            );
            raising.requires_coordination = Some(h);
            if gap == Gap::PP {
                raising.fills_preposition = Some(shared_child);
            }
            drafts.push(raising);
        }
        drafts
    }

    /// The drafts of a Coordination schema: the whole coordination (binary,
    /// delimited binary, serial, each after a correlative marker too), its
    /// serial tail, and Right Node Raising, once per Category.
    fn coordination(&self, schema: &SchemaDecl) -> Result<Vec<Draft>, LoadError> {
        let SchemaDecl::Coordination {
            category,
            coordinators,
            cost,
            delimiter,
            delimited,
            serial: serial_shapes,
            ..
        } = schema
        else {
            unreachable!("a coordination schema")
        };
        let conjoin = Conjoin::new(self, schema)?;
        let serials = serial_shapes.unwrap_or(conjoin.unlike.is_none());
        let coordinator = self.coordinator_test(coordinators)?;
        let mut drafts = Vec::new();
        // The whole coordination, plain and after each correlative marker
        // whose paired coordinator the schema's coordinators admit.
        let mut variants = vec![(None, coordinator.clone(), None)];
        for (marker, paired) in &self.grammar.correlatives {
            let paired = self.word(paired)?;
            if self.admits(&coordinator, paired) {
                variants.push((
                    Some(self.coordinator_test(marker)?),
                    Test::Word(paired),
                    Some(paired),
                ));
            }
        }
        for (marker, test, paired) in variants {
            let suffix = paired.map_or(String::new(), |p| format!("-{p}"));
            let id = |shape: &str| format!("Coordination:{}/{shape}{suffix}", conjoin.tail_name);
            // Binary: X Coord X.
            if !*delimited {
                drafts.push(conjoin.binary(
                    self,
                    id("binary"),
                    marker.as_ref(),
                    test.clone(),
                    None,
                    *cost,
                )?);
            }
            // Binary with the delimiter before the coordinator: X , Coord X.
            if let (Some(mark), None) = (delimiter, &marker) {
                drafts.push(conjoin.binary(
                    self,
                    id("binary-delimited"),
                    None,
                    test,
                    Some(*mark),
                    *cost,
                )?);
            }
            // Serial with the Oxford comma: X , X , Coord X — one tree per
            // chain; unlike conjuncts differ first to last.
            if !serials {
                continue;
            }
            let mut serial = Draft::new(id("serial"), conjoin.lhs, *cost);
            if let Some(marker) = &marker {
                serial.child("correlative", Symbol::Leaf(marker.clone()), GapRule::Closed);
            }
            let a = serial.child("first", conjoin.conjunct(), GapRule::Shared);
            serial.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
            let t = serial.child("rest", conjoin.tail(), GapRule::Shared);
            serial.coordinator = Some(t);
            serial.paired_coordinator = paired;
            if marker.is_none() {
                for (_, word) in &self.grammar.correlatives {
                    let word = self.word(word)?;
                    if !self.admits(&coordinator, word) {
                        serial.foreign_coordinators.push(word);
                    }
                }
            }
            serial.distinct_coordinator = vec![a];
            serial.unlike = conjoin.unlike.map(|f| (a, t, f));
            conjoin.close(self, &mut serial, &[a, t], Edge::Whole { last: false })?;
            drafts.push(serial);
        }
        if !serials {
            return Ok(drafts);
        }
        drafts.extend(conjoin.polysyndetic(self, &coordinator, *delimiter, *cost)?);
        let mut more = Draft::new(
            format!("Coordination:{}/serial-more", conjoin.tail_name),
            conjoin.tail_id,
            0,
        );
        let a = more.child("first", conjoin.conjunct(), GapRule::Shared);
        more.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let t = more.child("rest", conjoin.tail(), GapRule::Shared);
        more.coordinator = Some(t);
        more.distinct_coordinator = vec![a];
        conjoin.close(self, &mut more, &[a, t], Edge::Tail { last: false })?;
        conjoin.carry_unlike(&mut more, t);
        drafts.push(more);
        let mut end = Draft::new(
            format!("Coordination:{}/serial-end", conjoin.tail_name),
            conjoin.tail_id,
            0,
        );
        let a = end.child("first", conjoin.conjunct(), GapRule::Shared);
        end.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let mut words = Vec::new();
        for (_, paired) in &self.grammar.correlatives {
            words.push(self.word(paired)?);
        }
        let end_test = self.with_words(coordinator, &words);
        let c = end.child("coordinator", Symbol::Leaf(end_test), GapRule::Closed);
        let b = end.child("last", conjoin.conjunct(), GapRule::Shared);
        end.coordinator = Some(c);
        end.distinct_coordinator = vec![a, b];
        conjoin.close(self, &mut end, &[a, b], Edge::Tail { last: true })?;
        conjoin.carry_unlike(&mut end, b);
        drafts.push(end);
        if conjoin.unlike.is_some() {
            return Ok(drafts);
        }
        // Right Node Raising, derived for the Category's first coordination
        // schema.
        let first =
            self.grammar.schemas.iter().find(
                |s| matches!(s, SchemaDecl::Coordination { category: c, .. } if c == category),
            );
        if conjoin.conjunct == conjoin.lhs
            && first.is_some_and(|first| std::ptr::eq(first, schema))
            && self.out.category_gap[conjoin.lhs] != 0
        {
            drafts.extend(self.right_node_raising(&conjoin, category, *cost));
        }
        Ok(drafts)
    }
}

/// Which part of a coordination a draft builds.
#[derive(Clone, Copy)]
enum Edge {
    /// The whole coordination; `last` when its final child is the last
    /// conjunct itself rather than the serial tail.
    Whole { last: bool },
    /// The serial tail; `last` when it ends with the last conjunct.
    Tail { last: bool },
}

/// A Coordination schema's resolved conjunct agreement.
struct Conjoin {
    lhs: usize,
    /// The conjuncts' Category: `lhs` unless the schema names another.
    conjunct: usize,
    unlike: Option<usize>,
    tail_id: usize,
    tail_name: String,
    features: Vec<usize>,
    unshared: BTreeSet<usize>,
    proximate: BTreeSet<usize>,
    resolved: BTreeSet<usize>,
    /// Features resolved over the conjuncts: (feature, value when they
    /// differ).
    resolutions: Vec<(usize, u16)>,
    /// Equations over the whole coordination.
    whole: Vec<Equation>,
    /// Joint exclusions over a `first` conjunct and the last.
    joints: Vec<Equation>,
    /// Equations over every conjunct but the last (`first`) and the last.
    firsts: Vec<Equation>,
    lasts: Vec<Equation>,
}

impl Conjoin {
    fn new(binder: &Binder<'_>, schema: &SchemaDecl) -> Result<Self, LoadError> {
        let SchemaDecl::Coordination {
            category,
            coordinators,
            unshared,
            equations,
            proximate,
            resolve,
            conjunct,
            unlike,
            ..
        } = schema
        else {
            unreachable!("a coordination schema")
        };
        let lhs = binder.category(category)?;
        let conjunct = conjunct
            .as_deref()
            .map_or(Ok(lhs), |c| binder.category(c))?;
        let unlike = unlike.as_deref().map(|f| binder.feature(f)).transpose()?;
        let features = |names: &[Ident]| {
            names
                .iter()
                .map(|name| binder.feature(name))
                .collect::<Result<BTreeSet<_>, _>>()
        };
        let joint = |e: &Equation| matches!(e, Equation::ExcludeTogether(_));
        let select = |prefix: &str| -> Vec<Equation> {
            equations
                .iter()
                .filter(|e| !joint(e) && equation_path(e).starts_with(prefix))
                .cloned()
                .collect()
        };
        let joints: Vec<Equation> = equations.iter().filter(|e| joint(e)).cloned().collect();
        for equation in &joints {
            let Equation::ExcludeTogether(conditions) = equation else {
                unreachable!("a joint exclusion")
            };
            if conditions
                .iter()
                .any(|(p, _)| !p.starts_with("first.") && !p.starts_with("last."))
            {
                return Err(invalid(format!(
                    "coordination of `{category}`: a joint exclusion is over `first` and `last`"
                )));
            }
        }
        let whole = select("$.");
        // A feature the schema's equations value on the whole coordination is
        // resolved: the conjuncts still share it, but the coordination takes
        // the resolved value (two singular nominals coordinated by *and*).
        let mut resolved = BTreeSet::new();
        for equation in &whole {
            if let Equation::Require(path, _) = equation
                && let Some(feature) = path.strip_prefix("$.")
            {
                resolved.insert(binder.feature(feature)?);
            }
        }
        let unshared = features(unshared)?;
        let proximate = features(proximate)?;
        let mut resolutions = Vec::new();
        for (name, value) in resolve {
            let feature = binder.feature(name)?;
            if unshared.contains(&feature)
                || proximate.contains(&feature)
                || resolved.contains(&feature)
                || !binder.out.category_features[lhs].contains(&feature)
            {
                return Err(invalid(format!(
                    "coordination of `{category}` cannot resolve `{name}`"
                )));
            }
            resolutions.push((feature, binder.value(feature, value)?));
        }
        let tail_name = serial_tail(&binder.grammar.schemas, schema, category, coordinators);
        let tail_id = binder.out.category_id(&tail_name).ok_or_else(|| {
            invalid(format!(
                "coordination of `{category}` needs a category `{tail_name}`"
            ))
        })?;
        Ok(Self {
            lhs,
            conjunct,
            unlike,
            tail_id,
            tail_name,
            features: binder.out.category_features[lhs].clone(),
            unshared,
            proximate,
            resolved,
            resolutions,
            whole,
            joints,
            firsts: select("first."),
            lasts: select("last."),
        })
    }

    fn conjunct(&self) -> Symbol {
        Symbol::Cat {
            binding: None,
            category: self.conjunct,
            prepositions: None,
            host: None,
        }
    }

    fn tail(&self) -> Symbol {
        Symbol::Cat {
            binding: None,
            category: self.tail_id,
            prepositions: None,
            host: None,
        }
    }

    /// A polysyndetic coordination, its coordinator repeated before each
    /// non-initial conjunct, each after the schema's delimiter (*another
    /// creature dies, or a card leaves your graveyard, or ...*; CGEL, Ch. 15
    /// §1.1, p. 1276): the whole (a conjunct and a tail of at least two
    /// links) and the links of the tails `TAIL+` (one or more) and `TAIL++`
    /// (two or more).
    #[expect(clippy::many_single_char_names, reason = "conjunct positions")]
    fn polysyndetic(
        &self,
        binder: &Binder<'_>,
        coordinator: &Test,
        delimiter: Option<char>,
        cost: u32,
    ) -> Result<Vec<Draft>, LoadError> {
        // Oracle sets each repeated coordinator off with the delimiter; an
        // undelimited chain stays binary.
        let Some(mark) = delimiter else {
            return Ok(Vec::new());
        };
        let name = &self.tail_name;
        let tail = |suffix: &str| -> Result<usize, LoadError> {
            binder
                .out
                .category_id(&format!("{name}{suffix}"))
                .ok_or_else(|| invalid(format!("no polysyndetic tail `{name}{suffix}`")))
        };
        let (one, two) = (tail("+")?, tail("++")?);
        let cat = |category| Symbol::Cat {
            binding: None,
            category,
            prepositions: None,
            host: None,
        };
        let mut drafts = Vec::new();
        let mut whole = Draft::new(format!("Coordination:{name}/polysyndetic"), self.lhs, cost);
        let a = whole.child("first", self.conjunct(), GapRule::Shared);
        let t = whole.child("rest", cat(two), GapRule::Shared);
        whole.coordinator = Some(t);
        whole.distinct_coordinator = vec![a];
        self.close(binder, &mut whole, &[a, t], Edge::Whole { last: false })?;
        drafts.push(whole);
        // A link and a further tail: into `TAIL++` and `TAIL+`.
        for (lhs, shape) in [(two, "++"), (one, "+")] {
            let mut link = Draft::new(format!("Coordination:{name}/polysyndetic{shape}"), lhs, 0);
            link.child("", Symbol::Punct(mark, Binding::Suffix), GapRule::Closed);
            let c = link.child(
                "coordinator",
                Symbol::Leaf(coordinator.clone()),
                GapRule::Closed,
            );
            let b = link.child("first", self.conjunct(), GapRule::Shared);
            let r = link.child("rest", cat(one), GapRule::Shared);
            link.coordinator = Some(c);
            link.chained = Some(r);
            link.distinct_coordinator = vec![b];
            self.close(binder, &mut link, &[b, r], Edge::Tail { last: false })?;
            drafts.push(link);
        }
        // The last link: into `TAIL+`.
        let mut end = Draft::new(format!("Coordination:{name}/polysyndetic-end"), one, 0);
        end.child("", Symbol::Punct(mark, Binding::Suffix), GapRule::Closed);
        let c = end.child(
            "coordinator",
            Symbol::Leaf(coordinator.clone()),
            GapRule::Closed,
        );
        let b = end.child("last", self.conjunct(), GapRule::Shared);
        end.coordinator = Some(c);
        end.distinct_coordinator = vec![b];
        self.close(binder, &mut end, &[b], Edge::Tail { last: true })?;
        drafts.push(end);
        Ok(drafts)
    }

    /// A serial tail of unlike conjuncts carries its last conjunct's value of
    /// the unlike feature, which the whole compares with the first's.
    fn carry_unlike(&self, draft: &mut Draft, last: usize) {
        if let Some(f) = self.unlike {
            draft
                .equations
                .push((Path::Mother(f), Target::Path(Path::Child(last, f))));
        }
    }

    /// A binary coordination, after a correlative marker or with a
    /// delimiter before its coordinator.
    fn binary(
        &self,
        binder: &Binder<'_>,
        id: String,
        marker: Option<&Test>,
        coordinator: Test,
        delimiter: Option<char>,
        cost: u32,
    ) -> Result<Draft, LoadError> {
        let mut draft = Draft::new(id, self.lhs, cost);
        if let Some(marker) = marker {
            draft.child("correlative", Symbol::Leaf(marker.clone()), GapRule::Closed);
        }
        let a = draft.child("first", self.conjunct(), GapRule::Shared);
        if let Some(mark) = delimiter {
            draft.child("", Symbol::Punct(mark, Binding::Suffix), GapRule::Closed);
            draft.delimits = true;
        }
        let c = draft.child("coordinator", Symbol::Leaf(coordinator), GapRule::Closed);
        let b = draft.child("last", self.conjunct(), GapRule::Shared);
        draft.coordinator = Some(c);
        draft.distinct_coordinator = vec![a, b];
        draft.unlike = self.unlike.map(|f| (a, b, f));
        self.close(binder, &mut draft, &[a, b], Edge::Whole { last: true })?;
        Ok(draft)
    }

    /// Resolved features: every conjunct but the last shares the value, which
    /// a serial tail carries as its own while recording its last conjunct's;
    /// the whole coordination takes their common value, or the schema's one
    /// when they differ.
    fn resolve(&self, draft: &mut Draft, conjuncts: &[usize], edge: Edge) {
        if self.resolutions.is_empty() {
            return;
        }
        let whole = matches!(edge, Edge::Whole { .. });
        let ends = matches!(edge, Edge::Whole { last: true } | Edge::Tail { last: true });
        let (last, nonlast) = conjuncts.split_last().expect("a conjunct");
        for &(feature, differ) in &self.resolutions {
            draft.resolutions.push((feature, differ, whole));
            // The first conjunct agrees with the rest of the conjuncts but
            // the last: a serial tail's own value.
            if !ends {
                for &c in nonlast {
                    draft.equations.push((
                        Path::Child(c, feature),
                        Target::Path(Path::Child(*last, feature)),
                    ));
                }
            }
            if !whole {
                let carrier = if ends { nonlast } else { conjuncts };
                for &c in carrier {
                    draft
                        .equations
                        .push((Path::Mother(feature), Target::Path(Path::Child(c, feature))));
                }
            }
        }
        draft.resolution_nonlast = if ends { nonlast.to_vec() } else { conjuncts.to_vec() };
        draft.resolution_last = Some((*last, !ends));
    }

    /// Makes the conjuncts agree and adds the schema's equations; the second
    /// of `conjuncts` is the last conjunct or the serial tail.
    fn close(
        &self,
        binder: &Binder<'_>,
        draft: &mut Draft,
        conjuncts: &[usize],
        edge: Edge,
    ) -> Result<(), LoadError> {
        let whole = matches!(edge, Edge::Whole { .. });
        self.resolve(draft, conjuncts, edge);
        for &feature in &self.features {
            if self.unshared.contains(&feature)
                || self.resolutions.iter().any(|&(f, _)| f == feature)
            {
                // Resolved features are bound by `resolve`.
                continue;
            }
            if self.proximate.contains(&feature) {
                // The coordination takes it from its last conjunct (or the
                // serial tail that ends with it).
                let last = conjuncts[conjuncts.len() - 1];
                draft.equations.push((
                    Path::Mother(feature),
                    Target::Path(Path::Child(last, feature)),
                ));
                continue;
            }
            if whole && self.resolved.contains(&feature) {
                for pair in conjuncts.windows(2) {
                    draft.equations.push((
                        Path::Child(pair[0], feature),
                        Target::Path(Path::Child(pair[1], feature)),
                    ));
                }
                continue;
            }
            for &c in conjuncts {
                draft
                    .equations
                    .push((Path::Mother(feature), Target::Path(Path::Child(c, feature))));
            }
        }
        if whole {
            binder.equations(draft, &self.whole)?;
        }
        let first = draft.children.iter().any(|c| c.name == "first");
        if first {
            binder.equations(draft, &self.firsts)?;
        }
        if matches!(edge, Edge::Whole { last: true } | Edge::Tail { last: true }) {
            binder.equations(draft, &self.lasts)?;
            // Joint exclusions relate the last conjunct to the one before it.
            if first {
                binder.equations(draft, &self.joints)?;
            }
        }
        Ok(())
    }
}

/// An active phrase whose Object is followed only by selected Preposition
/// Phrase Complements, with a Modifier of that Object postposed after them,
/// a `modifier` of the `postposed` function Category (*put a number of
/// counters [on it] [equal to …]*; CGEL, Ch. 12 §4, p. 1066). The Modifier
/// would otherwise stand inside the Object, before the Complements it
/// crosses; it is never interleaved.
fn postposed_modifier(draft: &Draft, postposed: usize) -> Option<Draft> {
    let crossed = draft.children.get(2..)?;
    if draft.children[1].name != "object"
        || crossed.is_empty()
        || !crossed.iter().all(|c| {
            matches!(
                c.symbol,
                Symbol::Cat {
                    prepositions: Some(_),
                    ..
                }
            )
        })
    {
        return None;
    }
    let mut postposing = draft.clone();
    postposing.id = Ident::new(&format!("{}/postposed", draft.id));
    postposing.child(
        "modifier",
        Symbol::Cat {
            category: postposed,
            host: None,
            binding: None,
            prepositions: None,
        },
        GapRule::Closed,
    );
    Some(postposing)
}

/// The coordination of an active phrase's complement sequence, its head
/// shared on the left (the mirror of Right Node Raising): the head, its
/// Frame Slots, a coordinator, and the Frame Slots again; `serial` makes three
/// sequences with the Oxford comma (*one into your hand, one on top of your
/// library, and one on the bottom*); with `between`, an Adjunct of that
/// Category ends the first of two (*on top of your library in any order and
/// the rest on the bottom*).
fn complement_clusters(
    draft: &Draft,
    coordinator: &Test,
    between: Option<usize>,
    serial: bool,
) -> Option<Draft> {
    let derived = ["/gap", "/passive", "/adjectival"];
    if draft.children.len() < 3
        || draft.frame.is_none()
        || derived.iter().any(|d| draft.id.contains(d))
        || draft.children[0].name != "head"
        || (serial && between.is_some())
    {
        return None;
    }
    let mut clustered = draft.clone();
    let tag = match (serial, between) {
        (true, _) => "clusters-serial",
        (false, Some(_)) => "clusters-interleaved",
        (false, None) => "clusters",
    };
    clustered.id = Ident::new(&format!("{}/{tag}", draft.id));
    for child in &mut clustered.children[1..] {
        child.gap = GapRule::Closed;
    }
    let mut offsets = Vec::new();
    let mut push_copy = |clustered: &mut Draft| {
        offsets.push(clustered.children.len() - 1);
        for child in draft.children[1..].iter().cloned() {
            clustered.child(child.name.as_ref(), child.symbol, GapRule::Closed);
        }
    };
    if serial {
        clustered.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        push_copy(&mut clustered);
        clustered.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
    }
    if let Some(category) = between {
        clustered.child(
            "adjunct",
            Symbol::Cat {
                category,
                binding: None,
                prepositions: None,
                host: None,
            },
            GapRule::Closed,
        );
    }
    clustered.child(
        "coordinator",
        Symbol::Leaf(coordinator.clone()),
        GapRule::Closed,
    );
    push_copy(&mut clustered);
    let on_slot = |p: &Path| matches!(p, Path::Child(i, _) if *i > 0);
    for offset in offsets {
        // A copied sequence's children follow the copy's position.
        let copy = |i: usize| if i == 0 { 0 } else { i + offset };
        let moved = |p: Path| match p {
            Path::Child(i, f) => Path::Child(copy(i), f),
            mother @ Path::Mother(_) => mother,
        };
        for (path, target) in &draft.equations {
            let target_on_slot = matches!(target, Target::Path(t) if on_slot(t));
            if on_slot(path) || target_on_slot {
                let target = match target {
                    Target::Path(t) => Target::Path(moved(*t)),
                    value @ Target::Value(_) => value.clone(),
                };
                clustered.equations.push((moved(*path), target));
            }
        }
        for (list, original) in [
            (&mut clustered.exclusions, &draft.exclusions),
            (&mut clustered.carried, &draft.carried),
        ] {
            list.extend(
                original
                    .iter()
                    .filter(|(i, ..)| *i > 0)
                    .map(|&(i, f, v)| (copy(i), f, v)),
            );
        }
        clustered.weights.extend(
            draft
                .weights
                .iter()
                .filter(|(i, _)| *i > 0)
                .map(|&(i, w)| (copy(i), w)),
        );
    }
    clustered.predicative = None;
    Some(clustered)
}

fn equation_path(equation: &Equation) -> &str {
    match equation {
        Equation::Require(p, _)
        | Equation::Exclude(p, _)
        | Equation::Carry(p, _)
        | Equation::Agree(p, _) => p,
        Equation::ExcludeTogether(conditions) => conditions.first().map_or("", |(p, _)| p),
    }
}

/// The engine-owned serial tail Category of a coordination schema: by its
/// coordinators, numbered after the first schema with the same ones.
fn serial_tail(
    schemas: &[SchemaDecl],
    schema: &SchemaDecl,
    category: &str,
    coordinators: &str,
) -> String {
    let earlier = schemas
        .iter()
        .take_while(|s| !std::ptr::eq(*s, schema))
        .filter(|s| {
            matches!(s, SchemaDecl::Coordination { category: c, coordinators: k, .. }
                if c == category && k == coordinators)
        })
        .count();
    if earlier == 0 {
        format!("{category}#{coordinators}")
    } else {
        format!("{category}#{coordinators}{}", earlier + 1)
    }
}

pub(crate) fn frame_name(phrase: &str, frame: &Frame) -> String {
    let slots: Vec<String> = frame
        .slots
        .iter()
        .map(|slot| match &slot.fill {
            SlotFill::PP(list) => format!(
                "PP[{}]",
                list.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            fill => format!("{:?}", fill.kind()),
        })
        .collect();
    let family = match frame.family {
        FrameFamily::Intransitive => "Intransitive".to_owned(),
        family => format!("{family:?}"),
    };
    let voice = if frame.active_only { "[ActiveOnly]" } else { "" };
    format!("Valence:{phrase}/{family}({}){voice}", slots.join(","))
}

/// What a Frame Slot selects, as same-head complement priority sees it.
fn extensions_of(fill: &SlotFill) -> Vec<Extension> {
    match fill {
        SlotFill::PP(list) => list.iter().map(|p| Extension::Preposition(*p)).collect(),
        other => vec![Extension::Slot(other.kind())],
    }
}

fn intern<T: PartialEq + Clone>(list: &mut Vec<T>, item: &T) -> usize {
    if let Some(index) = list.iter().position(|x| x == item) {
        return index;
    }
    list.push(item.clone());
    list.len() - 1
}

/// The CR-catalog ban: the grammar names only core closed-class lexemes.
fn check_closed(lexicon: &Lexicon, id: &LexemeId, context: &str) -> Result<(), LoadError> {
    let lexeme = lexicon
        .lexemes()
        .get(id)
        .ok_or_else(|| invalid(format!("{context}: unknown lexeme `{id}`")))?;
    if lexeme.source.kind != SourceKind::Core {
        return Err(invalid(format!(
            "{context}: `{id}` comes from {:?}, not the core closed class; catalog vocabulary never appears in the grammar",
            lexeme.source.kind
        )));
    }
    if !matches!(
        lexeme.category,
        LexicalCategory::Determinative
            | LexicalCategory::Pronoun
            | LexicalCategory::Preposition
            | LexicalCategory::Coordinator
            | LexicalCategory::Subordinator
            | LexicalCategory::Adverb
            | LexicalCategory::Clitic
            | LexicalCategory::Verb
    ) {
        return Err(invalid(format!(
            "{context}: `{id}` is a {:?}, not a closed-class word",
            lexeme.category
        )));
    }
    Ok(())
}

/// The child name of a Frame Slot: the grammatical function it bears.
pub(crate) fn relation_name(relation: SlotRelation) -> &'static str {
    match relation {
        SlotRelation::Object => "object",
        SlotRelation::IndirectObject => "indirect_object",
        SlotRelation::Complement => "complement",
        SlotRelation::PredicativeComplement => "predicative_complement",
    }
}

fn path_before(path: Path, split: usize) -> bool {
    match path {
        Path::Mother(_) => true,
        Path::Child(i, _) => i < split,
    }
}
fn path_after(path: Path, split: usize) -> bool {
    match path {
        Path::Mother(_) => true,
        Path::Child(i, _) => i >= split,
    }
}
fn remap_draft(draft: &mut Draft, map: impl Fn(usize) -> usize) {
    let path = |p: &mut Path| {
        if let Path::Child(i, _) = p {
            *i = map(*i);
        }
    };
    for (a, b) in &mut draft.equations {
        path(a);
        if let Target::Path(p) = b {
            path(p);
        }
    }
    for (i, _, _) in draft
        .exclusions
        .iter_mut()
        .chain(&mut draft.carried)
        .chain(draft.joint_exclusions.iter_mut().flatten())
    {
        *i = map(*i);
    }
    draft.predicative = draft.predicative.map(&map);
}

/// The Gap a grammar names.
fn gap_named(name: &str) -> Option<Gap> {
    Some(match name {
        "None" => Gap::None,
        "NP" => Gap::NP,
        "Adjunct" => Gap::Adjunct,
        "Pred" => Gap::Pred,
        "Keyword" => Gap::Keyword,
        "ToInf" => Gap::ToInf,
        "PP" => Gap::PP,
        "Measure" => Gap::Measure,
        "BareInf" => Gap::BareInf,
        "Amount" => Gap::Amount,
        _ => return None,
    })
}
