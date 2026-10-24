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
use crate::chart::QUOTATION_MARK;
use crate::frames::Frame;
use crate::frames::FrameCatalog;
use crate::frames::FrameFamily;
use crate::frames::PrepositionFrame;
use crate::frames::SlotFill;
use crate::frames::SlotKind;
use crate::frames::SlotRelation;
use crate::grammar::ClustersDecl;
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
/// (`docs/decisions/english-v4.md` §4.4, principle 6).
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
    /// The word forms of every Word spelled like a lexical value, ignoring
    /// case, as a mask (`step::form_bit`), where they are more than the
    /// value's own form (`LeafFacts::forms`).
    pub(crate) homograph_forms: BTreeMap<deckmaste_lexical::LexicalValue, u16>,
    /// The card-scoped spellings of the lexicon, as (Lexeme, spelling
    /// variant), which analysis admits only within their card's scope.
    pub(crate) card_scoped: BTreeSet<(LexemeId, usize)>,
    pub(crate) frames: Vec<Frame>,
    /// Each frame's shape for Complement Clusters, as the index of the first
    /// frame with the same: the relation of each Frame Slot, and what fills
    /// it unless it is an Object. Clusters coordinate only with Clusters of
    /// one shape, like sequences of dependents with like functions (CGEL,
    /// Ch. 15 §4.3, p. 1341, [17ii]: *blame X for Y* and *blame Y on X* do
    /// not coordinate).
    pub(crate) cluster_shapes: Vec<u16>,
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
    /// Categories some Production's `unadjoined` child may not end in, the
    /// only ones a Summary's `final_adjunct` records.
    pub(crate) unadjoined_categories: BTreeSet<u16>,
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

    /// The index of a Category, by name.
    #[must_use]
    pub fn category_id(&self, name: &str) -> Option<usize> {
        self.category_ids.get(name).copied()
    }
}

/// The word forms of every Word spelled like each lexical value, ignoring
/// case, where they are more than the value's own form
/// (`CompiledGrammar::homograph_forms`).
fn homograph_forms(lexicon: &Lexicon) -> BTreeMap<deckmaste_lexical::LexicalValue, u16> {
    let spelled: Vec<(String, &deckmaste_lexical::LexicalValue)> = lexicon
        .values()
        .filter_map(|value| {
            let reading = deckmaste_lexical::LexicalReading::Word(value.clone());
            Some((lexicon.realize(&reading).ok()?.to_lowercase(), value))
        })
        .collect();
    let mut by_spelling: BTreeMap<&str, u16> = BTreeMap::new();
    for (spelling, value) in &spelled {
        *by_spelling.entry(spelling).or_default() |= crate::step::form_bit(value.form);
    }
    spelled
        .iter()
        .filter_map(|(spelling, value)| {
            let forms = by_spelling[spelling.as_str()];
            (forms != crate::step::form_bit(value.form)).then(|| ((*value).clone(), forms))
        })
        .collect()
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
    /// The holders of each (feature, value) that Productions set on their
    /// mother: each such Production, or the lexemes its lexical `head` names
    /// ([`Binder::setter_holders`]).
    setters: BTreeMap<(usize, u16), BTreeSet<String>>,
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
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent facts of one Production"
)]
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
    adjunct_unfit: Vec<(SlotKind, usize, u16)>,
    predicative: Option<usize>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
    /// The coordinator a correlative coordination's serial tail must end with.
    paired_coordinator: Option<LexemeId>,
    uncoordinated: Vec<usize>,
    coordinated: Vec<usize>,
    /// Mother features not shared with the `head` child.
    unshared: Vec<usize>,
    /// Children that are not, nor are headed by, a Linked Coordination.
    unlinked: Vec<usize>,
    scoped: Vec<usize>,
    /// A binary coordination with a delimiter before its coordinator.
    delimits: bool,
    /// A serial tail of a coordination, whose `first` conjunct follows another.
    tail: bool,
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
    /// It ends in an ellipsis site whose omitted Complement takes these word
    /// forms, as a mask (`ProductionDecl::elliptical`).
    elliptical: Option<u16>,
    /// It opens with a dash only its sentence's end closes
    /// (`ProductionDecl::unclosed`).
    unclosed: bool,
    /// A Linked Coordination's identity (`Production::link`).
    link: Option<LexemeId>,
    /// It builds a Complement Cluster of this frame (`Production::cluster`).
    cluster: Option<usize>,
    /// The child that must be a coordination of Complement Clusters of this
    /// frame (`Production::clusters`).
    clusters: Option<(usize, usize)>,
    /// Its part in Complement Cluster remnants (`Production::remnant`).
    remnant: crate::step::RemnantRole,
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
            adjunct_unfit: Vec::new(),
            predicative: None,
            coordinator: None,
            distinct_coordinator: Vec::new(),
            requires_coordination: None,
            paired_coordinator: None,
            uncoordinated: Vec::new(),
            unlinked: Vec::new(),
            coordinated: Vec::new(),
            unshared: Vec::new(),
            scoped: Vec::new(),
            delimits: false,
            tail: false,
            chained: None,
            gap_preposition: None,
            fills_preposition: None,
            foreign_coordinators: Vec::new(),
            unlike: None,
            internalised: None,
            omits_complement: false,
            elliptical: None,
            unclosed: false,
            link: None,
            cluster: None,
            clusters: None,
            remnant: crate::step::RemnantRole::None,
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
                link,
                ..
            } = schema
            {
                if coordinators.is_some() == link.is_some() {
                    return Err(invalid(format!(
                        "coordination of `{category}` names exactly one of `coordinators` and `link`"
                    )));
                }
                let declaration = declared
                    .get(category)
                    .cloned()
                    .ok_or_else(|| invalid(format!("unknown category `{category}`")))?;
                let tail = Ident::new(&serial_tail(&grammar.schemas, schema, category));
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
            // A preposition that selects both an Amount and a Noun Phrase
            // selects their unlike coordination, where the grammar names a
            // Category for it (CGEL, Ch. 15 §3.1, p. 1323, rule [1]).
            if grammar.slot_categories.contains_key(&SlotKind::AmountOrNP) {
                let selected = |fill: SlotFill, relation: SlotRelation| {
                    list.iter()
                        .find(|f| f.complement == fill && f.relation == relation)
                };
                for relation in [
                    SlotRelation::Complement,
                    SlotRelation::PredicativeComplement,
                ] {
                    if let (Some(np), Some(amount)) = (
                        selected(SlotFill::NP, relation),
                        selected(SlotFill::Amount, relation),
                    ) {
                        let derived = PrepositionFrame {
                            relation,
                            complement: SlotFill::AmountOrNP,
                            hosts: np.hosts.meet(amount.hosts),
                        };
                        let index = intern(&mut preposition_frames, &derived);
                        facts.preposition_frames.insert(index);
                    }
                }
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
                homograph_forms: homograph_forms(lexicon),
                card_scoped: lexicon
                    .values()
                    .filter(|value| lexicon.is_card_scoped(value))
                    .map(|value| (value.lexeme, value.variant))
                    .collect(),
                frames,
                cluster_shapes: Vec::new(),
                preposition_frames,
                extensions,
                extension_sets,
                classes,
                unextended_prepositions: BTreeSet::new(),
                unadjoined_categories: BTreeSet::new(),
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
            for (name, categories) in &declaration.unadjoined {
                self.unadjoined(declaration.name, name, categories)?;
            }
        }
        self.auxiliaries()?;
        self.edge_lines()?;
        self.report_holders();
        for (name, declaration) in &self.grammar.categories {
            if let Some(category) = &declaration.category {
                self.category(category)?;
                let id = self.category(name)?;
                self.out.categories[id].clone_from(category);
            }
        }
        self.out.cluster_shapes = cluster_shapes(&self.out.frames);
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

    /// Closes child `name` of Production `id` to a trailing Adjunct of each of
    /// `categories`, through its edge line.
    fn unadjoined(
        &mut self,
        id: Ident,
        name: &Ident,
        categories: &[Ident],
    ) -> Result<(), LoadError> {
        let context = |e: &str| invalid(format!("production `{id}`: unadjoined `{name}` {e}"));
        let index = *self
            .out
            .by_id
            .get(&id)
            .ok_or_else(|| context("names no production"))?;
        let at = self.out.productions[index]
            .names
            .iter()
            .position(|n| n == name)
            .ok_or_else(|| context("names no child"))?;
        if categories.is_empty() {
            return Err(context("names no Category"));
        }
        let mut closed = Vec::new();
        for category in categories {
            let category = u16::try_from(self.category(category)?)
                .map_err(|_| context("names a Category past the index range"))?;
            closed.push(category);
        }
        self.out
            .unadjoined_categories
            .extend(closed.iter().copied());
        self.out.productions[index].unadjoined = Some((at, closed));
        Ok(())
    }

    /// Extends each Production's edge line to a last child of its own
    /// Category or an alias of it, after its head: a Complement that the
    /// phrase ends in as it ends in its head (a modal's bare infinitival).
    /// A coordination has no head, so its conjuncts are never on it.
    fn edge_lines(&mut self) -> Result<(), LoadError> {
        let mut base = (0..self.out.categories.len()).collect::<Vec<_>>();
        for (name, declaration) in &self.grammar.categories {
            if let Some(category) = &declaration.category {
                base[self.category(name)?] = self.category(category)?;
            }
        }
        for production in &mut self.out.productions {
            let last = production.rhs.len() - 1;
            if let Some(Symbol::Cat { category, .. }) = production.rhs.last()
                && production.head.is_some_and(|head| head < last)
                && base[*category] == base[production.lhs]
            {
                production.edge_line[last] = true;
            }
        }
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
        for declared in &declaration.rhs {
            let quoted = matches!(declared, SymbolDecl::Quoted(..));
            if quoted {
                draft.child(
                    "",
                    Symbol::Punct(QUOTATION_MARK, Binding::Prefix),
                    GapRule::Closed,
                );
            }
            let (name, symbol) = self.symbol(declared).map_err(context)?;
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
            if quoted {
                draft.child(
                    "",
                    Symbol::Punct(QUOTATION_MARK, Binding::Suffix),
                    GapRule::Closed,
                );
            }
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
        for name in &declaration.unshared {
            let feature = self.feature(name.as_str()).map_err(context)?;
            if !self.out.category_features[draft.lhs].contains(&feature)
                || !draft.children.iter().any(|c| c.name == "head")
            {
                return Err(context(invalid(format!(
                    "unshared feature `{name}` is not one the mother shares with a head"
                ))));
            }
            draft.unshared.push(feature);
        }
        for name in &declaration.uncoordinated {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name)
                .ok_or_else(|| context(invalid(format!("unknown uncoordinated child `{name}`"))))?;
            draft.uncoordinated.push(child);
        }
        for name in &declaration.unlinked {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name)
                .ok_or_else(|| context(invalid(format!("unknown unlinked child `{name}`"))))?;
            draft.unlinked.push(child);
        }
        for name in &declaration.coordinated {
            let child = draft
                .children
                .iter()
                .position(|c| c.name == *name)
                .ok_or_else(|| context(invalid(format!("unknown coordinated child `{name}`"))))?;
            draft.coordinated.push(child);
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
        draft.unclosed = declaration.unclosed;
        if let Some(forms) = &declaration.elliptical {
            if forms.is_empty() {
                return Err(context(invalid(
                    "an elliptical Production names its omitted Complement's word forms",
                )));
            }
            draft.elliptical = Some(
                forms
                    .iter()
                    .fold(0, |mask, form| mask | crate::step::form_bit(*form)),
            );
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
            // A quoted child's marks are added around it (`authored`).
            SymbolDecl::Cat(name, category) | SymbolDecl::Quoted(name, category) => (
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
            SymbolDecl::Mark(c, _) | SymbolDecl::Punct(c)
                if crate::chart::is_quotation_mark(*c) =>
            {
                return Err(invalid(format!(
                    "quotation mark `{c}`: a quoted child is declared with `Quoted`"
                )));
            }
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

    /// Reports, as findings, a declared feature with no value and a feature
    /// value with fewer than two holders (`english-lexical-analysis.md` §3,
    /// `english-v4.md` §4.4, principle 3). A feature may declare one value, a
    /// privative one, whose unvalued feature is the unmarked default; the
    /// value is still a finding unless it has two or more holders. A
    /// value's holders are the lexemes whose feature bundles carry it and
    /// those that hold it through a Production setting it on its mother
    /// ([`Binder::setter_holders`]); a value that has none is a default,
    /// held through the Productions that require it of a child leaving the
    /// feature unvalued. A value the engine projects from inflection is
    /// held by every word form inflected for it. Features equated with one
    /// another share their holders of a value of the same name, and that
    /// value is reported once, naming them all.
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
        let mut reported: BTreeSet<(usize, &str)> = BTreeSet::new();
        for (feature, name) in out.features.iter().enumerate() {
            let domain = &out.values[feature];
            if domain.is_empty() {
                warnings.push(format!(
                    "feature `{name}` has no value; a feature needs one or more"
                ));
            }
            for value_name in domain {
                // Linked features are reported once, under the first of them.
                let first = *linked[feature]
                    .first()
                    .expect("a feature is linked to itself");
                if !reported.insert((first, value_name.as_str())) {
                    continue;
                }
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
                    .chain(
                        keys.iter()
                            .filter_map(|key| self.setters.get(key))
                            .flatten(),
                    )
                    .cloned()
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
                    let names = keys
                        .iter()
                        .map(|&(f, _)| format!("`{}`", out.features[f]))
                        .collect::<Vec<_>>()
                        .join("/");
                    warnings.push(format!(
                        "feature value {names} `{value_name}` has {} holder(s) ({}); a value \
                         needs two or more",
                        holders.len(),
                        holders.into_iter().collect::<Vec<_>>().join(", ")
                    ));
                }
            }
        }
        out.report.warnings.extend(warnings);
    }

    /// Who holds a value a Production sets on its mother. Where its `head`
    /// is a lexical leaf that names its lexemes (a Closed Word, a Lexical
    /// Class, a keyword's Parameter pattern, or the lexemes that declare a
    /// Subcategorization Frame), the holders are those lexemes: a Production
    /// lifting a class, or derived from a frame or catalog parameter
    /// pattern, gives each of them the value. Otherwise it is the Production.
    fn setter_holders(&self, draft: &Draft) -> BTreeSet<String> {
        let lexemes: BTreeSet<String> = match draft.children.iter().find(|c| c.name == "head") {
            Some(Child {
                symbol: Symbol::Leaf(test),
                ..
            }) => match test {
                Test::Word(id) => BTreeSet::from([id.to_string()]),
                Test::Words(words) => words.iter().map(ToString::to_string).collect(),
                Test::Class(class) => self.out.classes[*class]
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                Test::Frame(frame) | Test::FreeFrame(frame) | Test::PrepositionFrame(frame) => self
                    .out
                    .lexemes
                    .iter()
                    .filter(|(_, facts)| match test {
                        Test::Frame(_) => facts.frames.contains(frame),
                        Test::FreeFrame(_) => {
                            facts.frames.contains(frame) && facts.binding == Binding::Free
                        }
                        _ => facts.preposition_frames.contains(frame),
                    })
                    .map(|(id, _)| id.to_string())
                    .collect(),
                _ => BTreeSet::new(),
            },
            _ => BTreeSet::new(),
        };
        if lexemes.is_empty() {
            BTreeSet::from([draft.id.to_string()])
        } else {
            lexemes
        }
    }

    /// Records who holds each value a draft sets or requires, and the
    /// features its equations link, for the load report.
    fn record_holders(&mut self, draft: &Draft) {
        let feature_of = |path: &Path| match *path {
            Path::Mother(f) | Path::Child(_, f) => f,
        };
        let holders = self.setter_holders(draft);
        for (path, target) in &draft.equations {
            match target {
                Target::Value(value) => {
                    let key = (feature_of(path), *value);
                    match path {
                        Path::Mother(_) => {
                            self.setters
                                .entry(key)
                                .or_default()
                                .extend(holders.iter().cloned());
                        }
                        Path::Child(..) => {
                            self.valuers.entry(key).or_default().insert(draft.id);
                        }
                    }
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
                .extend(holders.iter().cloned());
        }
    }

    /// Resolves a draft's equations into variables and adds it.
    fn add(&mut self, mut draft: Draft) -> Result<(), LoadError> {
        self.record_holders(&draft);
        if let Some(head) = draft.children.iter().position(|c| c.name == "head") {
            for &feature in &self.out.category_features[draft.lhs] {
                if !draft.unshared.contains(&feature)
                    && self.carries(&draft.children[head].symbol, feature)
                {
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
        production.adjunct_unfit = draft.adjunct_unfit;
        production.predicative = draft.predicative;
        production.paired_coordinator = draft.paired_coordinator;
        production.uncoordinated = draft.uncoordinated;
        production.unlinked = draft.unlinked;
        production.coordinated = draft.coordinated;
        production.scoped = draft.scoped;
        production.delimits = draft.delimits;
        production.tail = draft.tail;
        production.chained = draft.chained;
        production.gap_preposition = draft.gap_preposition;
        production.fills_preposition = draft.fills_preposition;
        production
            .foreign_coordinators
            .clone_from(&draft.foreign_coordinators);
        production.unlike = draft.unlike;
        production.omission = if let Some(forms) = draft.elliptical {
            crate::step::Omission::Ellipsis(forms)
        } else if draft.omits_complement {
            crate::step::Omission::Selected
        } else {
            crate::step::Omission::None
        };
        production.link = draft.link;
        production.unclosed = draft.unclosed;
        production.cluster = draft
            .cluster
            .map(|frame| u16::try_from(frame).expect("frame index fits"));
        production.clusters = draft
            .clusters
            .map(|(at, frame)| (at, u16::try_from(frame).expect("frame index fits")));
        production.remnant = draft.remnant;
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
                remnant,
            } => self.parameters(
                *head,
                phrase,
                *binding,
                coordinators.as_deref(),
                remnant.as_deref(),
            ),
            SchemaDecl::Valence {
                head,
                phrase,
                cost,
                internalised,
                interleaved,
                interleaved_internalised,
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
                let mut clustered = Vec::new();
                if let Some(decl) = clusters {
                    let categories = self.cluster_categories(decl)?;
                    for draft in &drafts {
                        clustered.extend(complement_clusters(draft, &categories));
                    }
                }
                let postposed = postposed.as_deref().map(|p| self.category(p)).transpose()?;
                // A verb's interleaved Adjunct is a Predicate's; a noun's
                // interleaved Modifier, before the Complement it selects, a
                // Nominal's (CGEL, Ch. 5 §15, p. 454 [12]–[13]).
                let host = if *head == LexicalCategory::Verb {
                    HostKind::Predicate
                } else {
                    HostKind::Nominal
                };
                let interleaved = Interleaving {
                    adjunct: interleaved.as_deref(),
                    internalised: interleaved_internalised.as_deref(),
                };
                let mut ordered =
                    self.valence_order(drafts.clone(), interleaved, *heavy_shift, host)?;
                if let Some(modifier) = postposed {
                    ordered.extend(self.postposed_modifiers(&drafts, modifier));
                }
                // An Adjunct interleaves in a Complement Cluster as in the
                // phrase (*deals [half X damage, rounded down, to any target],
                // and [half X damage, rounded up, to you]*).
                ordered.extend(self.valence_order(clustered, interleaved, false, host)?);
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
                draft.adjunct_unfit = self.unfit(adjunct_id)?;
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

    /// The Categories a `clusters` declaration names.
    fn cluster_categories(&self, decl: &ClustersDecl) -> Result<ClusterCategories, LoadError> {
        Ok(ClusterCategories {
            cluster: self.category(&decl.cluster)?,
            coordination: self.category(&decl.coordination)?,
            adjunct: decl
                .adjunct
                .as_deref()
                .map(|a| self.category(a))
                .transpose()?,
            remnants: decl
                .remnants
                .iter()
                .map(|(kind, remnant)| Ok((self.slot_category(*kind)?, self.category(remnant)?)))
                .collect::<Result<_, LoadError>>()?,
        })
    }

    fn parameters(
        &mut self,
        head: LexicalCategory,
        phrase: &str,
        binding: Option<Binding>,
        coordinators: Option<&str>,
        remnant: Option<&str>,
    ) -> Result<Vec<Draft>, LoadError> {
        use crate::frames::ParameterPart;
        let coordinator = coordinators.map(|c| self.coordinator_test(c)).transpose()?;
        let lhs = self.category(phrase)?;
        let remnant = remnant.map(|r| self.category(r)).transpose()?;
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
                // A pattern that begins with its head and has more after it
                // also stands without the head, as a Complement Cluster's
                // remnant (`grammar::ClustersDecl::remnants`).
                if let Some(remnant) = remnant
                    && draft.children.len() > 1
                    && draft.children[0].name == "head"
                {
                    let mut elided = Draft::new(format!("{}/remnant", draft.id), remnant, 0);
                    for child in &draft.children[1..] {
                        elided.child(child.name.as_ref(), child.symbol.clone(), GapRule::Closed);
                    }
                    elided.remnant = crate::step::RemnantRole::Elides(*id);
                    draft.remnant = crate::step::RemnantRole::SharesHead;
                    drafts.push(elided);
                }
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

    fn internalised_orders(draft: &Draft) -> Vec<Draft> {
        let mut ordered = Vec::new();
        // The internalised Complement (CGEL, Ch. 16 §10.1.1, pp. 1428–1429)
        // is a PP Complement like the Frame's own, so it can precede a later
        // one (*damage dealt by a source to a player*). Preserve the same
        // Frame and functions in each surface order, keeping any NP Object
        // before the by-phrase.
        if let Some(agent) = draft.internalised {
            for before in 1..agent {
                if !draft.children[before..agent].iter().all(|child| {
                    matches!(
                        child.symbol,
                        Symbol::Cat {
                            prepositions: Some(_),
                            ..
                        }
                    )
                }) {
                    continue;
                }
                let mut reordered = draft.clone();
                reordered.id = Ident::new(&format!("{}/internalised-before{before}", draft.id));
                let child = reordered.children.remove(agent);
                reordered.children.insert(before, child);
                remap_draft(&mut reordered, |i| {
                    if i == agent {
                        before
                    } else if (before..agent).contains(&i) {
                        i + 1
                    } else {
                        i
                    }
                });
                reordered.internalised = Some(before);
                ordered.push(reordered);
            }
        }
        ordered
    }

    /// A phrase's prefix up to an interleaved dependent of the Adjunct
    /// Category `adjunct`, ending in it, and that prefix with another one.
    fn interleaved_dependent(
        &self,
        base: &Draft,
        id: &str,
        category: usize,
        adjunct: usize,
        function: &str,
        host: HostKind,
    ) -> Result<[Draft; 2], LoadError> {
        let symbol = |category, host| Symbol::Cat {
            category,
            host,
            binding: None,
            prepositions: None,
        };
        let mut prefix = base.clone();
        prefix.id = Ident::new(id);
        let a = prefix.child(function, symbol(adjunct, Some(host)), GapRule::Closed);
        prefix.host_check = Some((0, a, host));
        prefix.adjunct_unfit = self.unfit(adjunct)?;
        prefix.cost += 2;
        // The interleaved dependent agrees with the phrase in each feature
        // they both carry, as a postposed Modifier does.
        let shared: Vec<usize> = self.out.category_features[adjunct]
            .iter()
            .copied()
            .filter(|f| self.out.category_features[category].contains(f))
            .collect();
        for &feature in &shared {
            prefix
                .equations
                .push((Path::Mother(feature), Target::Path(Path::Child(a, feature))));
        }
        let mut more = Draft::new(format!("{id}/more"), category, 2);
        more.child("head", symbol(category, None), GapRule::Transparent);
        more.child(function, symbol(adjunct, Some(host)), GapRule::Closed);
        more.host_check = Some((0, 1, host));
        more.adjunct_unfit.clone_from(&prefix.adjunct_unfit);
        for &feature in &shared {
            more.equations
                .push((Path::Mother(feature), Target::Path(Path::Child(1, feature))));
        }
        Ok([prefix, more])
    }

    fn valence_order(
        &mut self,
        drafts: Vec<Draft>,
        interleaving: Interleaving<'_>,
        heavy: bool,
        host: HostKind,
    ) -> Result<Vec<Draft>, LoadError> {
        let adjunct = interleaving
            .adjunct
            .map(|name| self.category(name))
            .transpose()?;
        let internalised = interleaving
            .internalised
            .map(|name| self.category(name))
            .transpose()?;
        let mut ordered = Vec::new();
        for draft in drafts {
            ordered.extend(Self::internalised_orders(&draft));
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
                let mut base = draft.clone();
                base.lhs = category;
                base.introduces = Gap::None;
                base.children.truncate(split);
                base.equations.retain(|(a, b)| {
                    path_before(*a, split)
                        && match b {
                            Target::Path(p) => path_before(*p, split),
                            Target::Value(_) => true,
                        }
                });
                base.exclusions.retain(|(i, _, _)| *i < split);
                base.joint_exclusions
                    .retain(|joint| joint.iter().all(|(i, _, _)| *i < split));
                base.carried.retain(|(i, _, _)| *i < split);
                base.predicative = base.predicative.filter(|&i| i < split);
                // A Nominal's unselected dependent is its Modifier.
                let function = match host {
                    HostKind::Nominal => "modifier",
                    _ => "adjunct",
                };
                // Before the Internalised Complement, its own Adjunct
                // Category interleaves too.
                let before_internalised =
                    internalised.filter(|_| draft.internalised == Some(split));
                for (adjunct, suffix) in std::iter::once((adjunct, ""))
                    .chain(before_internalised.map(|c| (c, "/internalised")))
                {
                    let id = format!("{}/prefix{split}{suffix}", draft.id);
                    out.extend(
                        self.interleaved_dependent(&base, &id, category, adjunct, function, host)?,
                    );
                }
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

    /// The (Frame Slot kind, feature, value) conditions under which a
    /// dependent of `category` could fill no Frame Slot of a kind whose
    /// Category it is: the `Exclude`s of the grammar's `slot_equations` for
    /// that kind (`Production::adjunct_unfit`).
    fn unfit(&self, category: usize) -> Result<Vec<(SlotKind, usize, u16)>, LoadError> {
        let mut unfit = Vec::new();
        for (kind, name) in &self.grammar.slot_categories {
            if self.category(name)? != category {
                continue;
            }
            let mut probe = Draft::new("slot", category, 0);
            let slot = probe.child(
                "slot",
                Symbol::Cat {
                    binding: None,
                    category,
                    prepositions: None,
                    host: None,
                },
                GapRule::Closed,
            );
            self.slot_equations(&mut probe, slot, *kind)?;
            if !(probe.equations.is_empty()
                && probe.carried.is_empty()
                && probe.joint_exclusions.is_empty())
            {
                return Err(invalid(format!(
                    "slot_equations for {kind:?}: only an Exclude can apply to an unselected dependent"
                )));
            }
            unfit.extend(probe.exclusions.iter().map(|&(_, f, v)| (*kind, f, v)));
        }
        Ok(unfit)
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
            link,
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
        let Some(coordinators) = coordinators else {
            let link = link.as_deref().expect("a coordination names its link");
            return self
                .linked_coordination(&conjoin, link, *cost, *delimiter, *delimited, serials);
        };
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
            // chain; unlike conjuncts are not all alike.
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
        conjoin.carry_unlike(&mut more, a, t);
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
        conjoin.carry_unlike(&mut end, a, b);
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

impl Binder<'_> {
    /// The drafts of a Linked Coordination schema, which has no Coordinator:
    /// the Conjunct after each link is of the `link` Category, marked by its
    /// own initial connective (*Draw a card, then discard a card*; CGEL,
    /// Ch. 15 §2.10, pp. 1319–1320). Its shapes are the binary (only with the
    /// delimiter when `delimited`) and the serial, whose middle Conjuncts are
    /// ordinary ones that may begin with the connective as well (*A, B, then
    /// C*; *A, then B, then C*): the serial reads every such chain, so there
    /// is no polysyndetic shape. Its identity is its serial tail's name:
    /// no Conjunct is, or is headed by, a Linked Coordination of the same
    /// identity (`Summary::link`), so each chain has one tree.
    fn linked_coordination(
        &self,
        conjoin: &Conjoin,
        link: &str,
        cost: u32,
        delimiter: Option<char>,
        only_delimited: bool,
        serials: bool,
    ) -> Result<Vec<Draft>, LoadError> {
        let marked = Symbol::Cat {
            binding: None,
            category: self.category(link)?,
            prepositions: None,
            host: None,
        };
        let identity = Ident::new(&conjoin.tail_name);
        let id = |shape: &str| format!("Coordination:{}/{shape}", conjoin.tail_name);
        let mut drafts = Vec::new();
        // Binary: X Y, or X , Y.
        let mut binaries = Vec::new();
        if !only_delimited {
            binaries.push(("binary", None));
        }
        if let Some(mark) = delimiter {
            binaries.push(("binary-delimited", Some(mark)));
        }
        for (shape, mark) in binaries {
            let mut draft = Draft::new(id(shape), conjoin.lhs, cost);
            let a = draft.child("first", conjoin.conjunct(), GapRule::Shared);
            if let Some(mark) = mark {
                draft.child("", Symbol::Punct(mark, Binding::Suffix), GapRule::Closed);
                draft.delimits = true;
            }
            let b = draft.child("last", marked.clone(), GapRule::Shared);
            draft.coordinator = Some(b);
            draft.link = Some(identity);
            draft.distinct_coordinator = vec![a, b];
            draft.unlike = conjoin.unlike.map(|f| (a, b, f));
            conjoin.close(self, &mut draft, &[a, b], Edge::Whole { last: true })?;
            drafts.push(draft);
        }
        if !serials {
            return Ok(drafts);
        }
        // Serial: X , X , Y — the whole, and its tail of one or more links.
        let mut serial = Draft::new(id("serial"), conjoin.lhs, cost);
        let a = serial.child("first", conjoin.conjunct(), GapRule::Shared);
        serial.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let t = serial.child("rest", conjoin.tail(), GapRule::Shared);
        serial.coordinator = Some(t);
        serial.link = Some(identity);
        serial.distinct_coordinator = vec![a];
        serial.unlike = conjoin.unlike.map(|f| (a, t, f));
        conjoin.close(self, &mut serial, &[a, t], Edge::Whole { last: false })?;
        drafts.push(serial);
        let mut more = Draft::new(id("serial-more"), conjoin.tail_id, 0);
        let a = more.child("first", conjoin.conjunct(), GapRule::Shared);
        more.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let t = more.child("rest", conjoin.tail(), GapRule::Shared);
        more.coordinator = Some(t);
        more.link = Some(identity);
        more.distinct_coordinator = vec![a];
        conjoin.close(self, &mut more, &[a, t], Edge::Tail { last: false })?;
        conjoin.carry_unlike(&mut more, a, t);
        drafts.push(more);
        let mut end = Draft::new(id("serial-end"), conjoin.tail_id, 0);
        let a = end.child("first", conjoin.conjunct(), GapRule::Shared);
        end.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let b = end.child("last", marked, GapRule::Shared);
        end.coordinator = Some(b);
        end.link = Some(identity);
        end.distinct_coordinator = vec![a, b];
        conjoin.close(self, &mut end, &[a, b], Edge::Tail { last: true })?;
        conjoin.carry_unlike(&mut end, a, b);
        drafts.push(end);
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
    /// Equations over the initial conjunct alone (`initial`), rewritten over
    /// the whole coordination's `first` child.
    initials: Vec<Equation>,
}

impl Conjoin {
    fn new(binder: &Binder<'_>, schema: &SchemaDecl) -> Result<Self, LoadError> {
        let SchemaDecl::Coordination {
            category,
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
        let tail_name = serial_tail(&binder.grammar.schemas, schema, category);
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
            initials: select("initial.").iter().map(initial_as_first).collect(),
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
        // Unlike conjuncts are not all alike, as in the serial.
        whole.unlike = self.unlike.map(|f| (a, t, f));
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
            self.carry_unlike(&mut link, b, r);
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
        self.carry_unlike(&mut end, b, b);
        drafts.push(end);
        Ok(drafts)
    }

    /// A serial tail of unlike conjuncts carries its last conjunct's value of
    /// the unlike feature, which the whole compares with the first's, and
    /// records whether its own conjuncts, from `first` to `last` (a conjunct
    /// or a further tail), already differ (`Summary::unlike_mixed`).
    fn carry_unlike(&self, draft: &mut Draft, first: usize, last: usize) {
        if let Some(f) = self.unlike {
            if first != last {
                draft.unlike = Some((first, last, f));
            }
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
        draft.tail = !whole;
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
            if whole {
                binder.equations(draft, &self.initials)?;
            }
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

/// Each frame's Complement Cluster shape (`CompiledGrammar::cluster_shapes`).
fn cluster_shapes(frames: &[Frame]) -> Vec<u16> {
    use crate::frames::SlotRelation;
    let shape = |frame: &Frame| -> Vec<(SlotRelation, Option<SlotFill>)> {
        frame
            .slots
            .iter()
            .map(|slot| {
                let fill = (slot.relation != SlotRelation::Object).then(|| slot.fill.clone());
                (slot.relation, fill)
            })
            .collect()
    };
    let shapes: Vec<_> = frames.iter().map(shape).collect();
    shapes
        .iter()
        .map(|s| {
            let first = shapes.iter().position(|t| t == s).expect("its own shape");
            u16::try_from(first).expect("frame index fits")
        })
        .collect()
}

/// The Adjunct Categories a Valence schema interleaves before its later
/// Complements: one before any of them, and one only before an Internalised
/// Complement.
#[derive(Clone, Copy)]
struct Interleaving<'a> {
    adjunct: Option<&'a str>,
    internalised: Option<&'a str>,
}

/// The Categories a Valence schema's Complement Clusters are built in
/// (`grammar::ClustersDecl`).
struct ClusterCategories {
    cluster: usize,
    coordination: usize,
    adjunct: Option<usize>,
    /// (Slot Category, remnant Category) pairs (`ClustersDecl::remnants`).
    remnants: Vec<(usize, usize)>,
}

/// An active phrase's Complement Clusters (right nonce-constituent
/// coordination, CGEL, Ch. 15 §4.3, pp. 1341–1343): its Frame Slots as a
/// Cluster of the frame, with an optional Adjunct after them, and the head
/// with a coordination of such Clusters (*put [one of them into your hand]
/// and [the rest into your graveyard]*; *has [lifelink if you control a
/// Cleric], [deathtouch if you control a Rogue], and [flying if you control
/// a Wizard]*). A single Slot forms a Cluster only with its Adjunct: alone it
/// coordinates as itself. Equations on the Slots move into the Cluster; those
/// on the head and the mother stay with the head.
fn complement_clusters(draft: &Draft, categories: &ClusterCategories) -> Vec<Draft> {
    let derived = ["/gap", "/passive", "/adjectival"];
    let Some(frame) = draft.frame else {
        return Vec::new();
    };
    if draft.children.len() < 2
        || derived.iter().any(|d| draft.id.contains(d))
        || draft.children[0].name != "head"
    {
        return Vec::new();
    }
    let slots = draft.children.len() - 1;
    let on_slot = |p: &Path| matches!(p, Path::Child(i, _) if *i > 0);
    let on_head_or_mother = |p: &Path| !on_slot(p);
    // A Slot's position in the Cluster, one less than in the phrase.
    let into_cluster = |p: Path| match p {
        Path::Child(i, f) => Path::Child(i - 1, f),
        mother @ Path::Mother(_) => mother,
    };
    let symbol = |category| Symbol::Cat {
        category,
        binding: None,
        prepositions: None,
        host: None,
    };
    // A later Cluster's first Slot may be a remnant, its head elided.
    let first_slot = match &draft.children[1].symbol {
        Symbol::Cat { category, .. } => Some(*category),
        _ => None,
    };
    let remnants: Vec<Option<(usize, usize)>> = std::iter::once(None)
        .chain(
            categories
                .remnants
                .iter()
                .enumerate()
                .filter(|(_, (slot, _))| Some(*slot) == first_slot)
                .map(|(n, &(_, remnant))| Some((n, remnant))),
        )
        .collect();
    // An equation, exclusion or weight on a remnant's Slot stays with the
    // phrase it stands for.
    let kept =
        |remnant: Option<usize>, p: &Path| remnant.is_none() || !matches!(p, Path::Child(1, _));
    let mut out = Vec::new();
    let adjuncts = [Some(None), categories.adjunct.map(Some)];
    for (adjunct, remnant) in adjuncts
        .into_iter()
        .flatten()
        .flat_map(|a| remnants.iter().map(move |&r| (a, r)))
    {
        if adjunct.is_none() && slots < 2 {
            continue;
        }
        let tag = if adjunct.is_some() { "cluster+adjunct" } else { "cluster" };
        let id = match remnant {
            Some((0, _)) => format!("{}/{tag}/remnant", draft.id),
            Some((n, _)) => format!("{}/{tag}/remnant{n}", draft.id),
            None => format!("{}/{tag}", draft.id),
        };
        let remnant = remnant.map(|(_, category)| category);
        let mut cluster = Draft::new(id, categories.cluster, 0);
        for (i, child) in draft.children[1..].iter().enumerate() {
            let symbol = match remnant {
                Some(category) if i == 0 => symbol(category),
                _ => child.symbol.clone(),
            };
            cluster.child(child.name.as_ref(), symbol, GapRule::Closed);
        }
        if let Some(category) = adjunct {
            cluster.child("adjunct", symbol(category), GapRule::Closed);
        }
        for (path, target) in &draft.equations {
            let target_on_slot = match target {
                Target::Path(t) => on_slot(t) && kept(remnant, t),
                Target::Value(_) => true,
            };
            if on_slot(path) && kept(remnant, path) && target_on_slot {
                let target = match target {
                    Target::Path(t) => Target::Path(into_cluster(*t)),
                    value @ Target::Value(_) => value.clone(),
                };
                cluster.equations.push((into_cluster(*path), target));
            }
        }
        for (list, original) in [
            (&mut cluster.exclusions, &draft.exclusions),
            (&mut cluster.carried, &draft.carried),
        ] {
            list.extend(
                original
                    .iter()
                    .filter(|(i, ..)| *i > 0 && (remnant.is_none() || *i > 1))
                    .map(|&(i, f, v)| (i - 1, f, v)),
            );
        }
        cluster.weights.extend(
            draft
                .weights
                .iter()
                .filter(|(i, _)| *i > 0 && (remnant.is_none() || *i > 1))
                .map(|&(i, w)| (i - 1, w)),
        );
        cluster.cluster = Some(frame);
        out.push(cluster);
    }
    if out.is_empty() {
        return out;
    }
    let mut whole = Draft::new(format!("{}/clusters", draft.id), draft.lhs, draft.cost);
    whole.child("head", draft.children[0].symbol.clone(), GapRule::Closed);
    let at = whole.child("clusters", symbol(categories.coordination), GapRule::Closed);
    for (path, target) in &draft.equations {
        let target_on_head_or_mother = match target {
            Target::Path(t) => on_head_or_mother(t),
            Target::Value(_) => true,
        };
        if on_head_or_mother(path) && target_on_head_or_mother {
            whole.equations.push((*path, target.clone()));
        }
    }
    for (list, original) in [
        (&mut whole.exclusions, &draft.exclusions),
        (&mut whole.carried, &draft.carried),
    ] {
        list.extend(original.iter().filter(|(i, ..)| *i == 0).copied());
    }
    whole
        .weights
        .extend(draft.weights.iter().filter(|(i, _)| *i == 0).copied());
    whole
        .scoped
        .extend(draft.scoped.iter().filter(|i| **i == 0));
    whole.frame = Some(frame);
    whole.clusters = Some((at, frame));
    out.push(whole);
    out
}

/// An equation over the `initial` conjunct, as one over a whole
/// coordination's `first` child, which is that conjunct.
fn initial_as_first(equation: &Equation) -> Equation {
    let first = |p: &Ident| Ident::new(&p.replacen("initial.", "first.", 1));
    match equation {
        Equation::Require(p, v) => Equation::Require(first(p), *v),
        Equation::Exclude(p, v) => Equation::Exclude(first(p), *v),
        Equation::Carry(p, v) => Equation::Carry(first(p), *v),
        Equation::Agree(p, q) => Equation::Agree(first(p), first(q)),
        Equation::ExcludeTogether(conditions) => {
            Equation::ExcludeTogether(conditions.iter().map(|(p, v)| (first(p), *v)).collect())
        }
    }
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
/// coordinators (or Linked Conjunct Category), numbered after the first
/// schema with the same ones.
fn serial_tail(schemas: &[SchemaDecl], schema: &SchemaDecl, category: &str) -> String {
    let coordinators = schema.joined_by().unwrap_or_default();
    let earlier = schemas
        .iter()
        .take_while(|s| !std::ptr::eq(*s, schema))
        .filter(|s| {
            matches!(s, SchemaDecl::Coordination { category: c, .. }
                if c == category && s.joined_by() == Some(coordinators))
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
