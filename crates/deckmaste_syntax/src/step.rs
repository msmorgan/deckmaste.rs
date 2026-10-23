//! Productions as compiled, and the one step function shared by the chart
//! and by Admission: adding a child to a partial Production.

use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::WordForm;
use dpsi::Ident;
use serde::Deserialize;
use serde::Serialize;

use crate::compile::CompiledGrammar;
use crate::compile::LexemeFacts;
use crate::frames::SlotKind;
use crate::grammar::HostKind;

/// The most features one Category may carry, so a Summary is inline.
pub(crate) const MAX_CATEGORY_FEATURES: usize = 12;
/// The most feature variables one Production may unify, so a State is inline.
pub(crate) const MAX_PRODUCTION_VARS: usize = 16;
/// The most features a grammar may declare, so a leaf indexes its values.
pub(crate) const MAX_FEATURES: usize = 48;
/// The most conjuncts one coordination Production may mark distinct.
pub(crate) const MAX_DISTINCT_COORDINATORS: usize = 2;
/// The most features one coordination Production may resolve.
pub(crate) const MAX_RESOLUTIONS: usize = 4;
/// The most conditions the joint exclusions of one Production may list.
pub(crate) const MAX_JOINT_CONDITIONS: usize = 8;

/// One feature value, with the final byte value reserved for an unbound slot.
/// Binding validates the domain before any value reaches a chart State.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct FeatureValue(u8);

impl FeatureValue {
    pub(crate) const UNSET: Self = Self(u8::MAX);

    fn new(value: u16) -> Self {
        let value = u8::try_from(value).expect("validated feature domain");
        assert_ne!(value, u8::MAX, "validated feature domain");
        Self(value)
    }

    pub(crate) fn get(self) -> Option<u16> {
        (self != Self::UNSET).then_some(u16::from(self.0))
    }
}

/// A Complement that another frame of a phrase's head selects. A longer
/// frame's Preposition Phrase headed by the preposition, or phrase of the
/// slot's kind (*to*-infinitival in *choose to attack*), is not also an
/// unselected adjunct of the same head instance; a Preposition Phrase that a
/// frame selects in place of a predicative Complement (locative *on* after
/// *be*) is not also that head's predicative Complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Extension {
    Preposition(LexemeId),
    Slot(SlotKind),
    Predicative(LexemeId),
}

/// The unfilled Gap a Constituent contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Gap {
    None,
    NP,
    /// An extracted adjunct, as in an adverbial or pied-piped relative clause.
    Adjunct,
    /// An extracted predicative Complement (*whose turn it is*).
    Pred,
    /// A keyword-phrase Complement shared by Right Node Raising (*can't have
    /// or gain trample*).
    Keyword,
    /// A *to*-infinitival Complement shared by Right Node Raising (*choose
    /// and choose to attack*).
    ToInf,
    /// A noun's Preposition Phrase Complement shared by Right Node Raising
    /// (*the top or bottom of their library*).
    PP,
    /// Nouns' Amount Complements shared, respectively, by a slash Measure
    /// (*base power and toughness 4/4*).
    Measure,
    /// A bare infinitival Complement shared by coordinated auxiliaries (*may
    /// and can draw cards*).
    BareInf,
    /// A preposition's Amount Complement shared by Right Node Raising (*less
    /// than or equal to X*).
    Amount,
}

impl Gap {
    /// This Gap's bit in a Category's mask of the Gaps it may contain.
    pub(crate) const fn bit(self) -> u16 {
        1 << self as u8
    }
}

/// How a child's Gap relates to its mother's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GapRule {
    /// The child contains no Gap.
    Closed,
    /// The child's Gap, if any, is the mother's.
    Transparent,
    /// Every shared child has the same Gap, which is the mother's (across the
    /// board).
    Shared,
    /// The child has exactly this Gap, filled within the mother.
    Require(Gap),
    /// The child has exactly this Gap, which is the mother's.
    Raise(Gap),
}

/// How many lexical leaves a child must span: one, or more than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Weight {
    Light,
    Heavy,
}

/// A lexical leaf test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Test {
    Lexical(LexicalCategory),
    FreeLexical(LexicalCategory),
    /// A free lexeme of this Category whose Complement is optional: it
    /// declares no Subcategorization Frame, or an Intransitive one.
    FreeOptional(LexicalCategory),
    /// A free lexeme of this Category not every Frame of which is restricted
    /// to active voice: an Active-only verb has no passive participle.
    FreeNotActiveOnly(LexicalCategory),
    Numeral(deckmaste_lexical::Numeral),
    Word(LexemeId),
    Class(usize),
    Words(std::collections::BTreeSet<LexemeId>),
    /// A lexeme that declares this Subcategorization Frame.
    Frame(usize),
    /// A free (unbound) lexeme that declares this Subcategorization Frame.
    FreeFrame(usize),
    /// A preposition that declares this complement frame.
    PrepositionFrame(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Symbol {
    Cat {
        category: usize,
        binding: Option<deckmaste_lexical::Binding>,
        /// A selected `PP` slot: the phrase's preposition must be one of these.
        prepositions: Option<Vec<LexemeId>>,
        /// An unselected adjunct: a phrase with a preposition must admit this
        /// host.
        host: Option<HostKind>,
    },
    Leaf(Test),
    Punct(char, deckmaste_lexical::Binding),
}

/// A compiled Production.
/// The Complements a Production's phrase omits at its right edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Omission {
    None,
    /// A noun's bare phrase omits the Complements its head's frames select
    /// (`Summary::omitted_complement`).
    Selected,
    /// It ends in an ellipsis site (`ProductionDecl::elliptical`).
    Ellipsis,
}

#[derive(Debug, Clone)]
pub(crate) struct Production {
    pub exclusions: Vec<(usize, usize, u16)>,
    /// Combinations of (child, feature, value) conditions that may not all
    /// hold: each exclusion is the run of `joint_conditions` up to its end
    /// index.
    pub joint_exclusions: Vec<usize>,
    pub joint_conditions: Vec<(usize, usize, u16)>,
    /// Features a coordination resolves: (feature, value when the conjuncts
    /// but the last and the last differ, the mother feature position a whole
    /// coordination resolves; a serial tail binds that position to its
    /// conjuncts but the last instead).
    pub resolutions: Vec<(usize, u16, Option<usize>)>,
    /// The children that carry the value of the conjuncts but the last.
    pub resolution_nonlast: Vec<usize>,
    /// The child that carries the last conjunct's value, as its own feature
    /// or (`true`) as a serial tail's recorded last value.
    pub resolution_last: Option<(usize, bool)>,
    /// (child, feature, value) a child must carry, not merely admit.
    pub carried: Vec<(usize, usize, u16)>,
    /// Children that must be light or heavy, or end in a dependent.
    pub weights: Vec<(usize, Weight)>,
    /// Children in the head line: the head, or a conjunct of a coordination.
    pub head_line: Vec<bool>,
    /// The `unextended` child, and the prepositions of a Preposition Phrase
    /// it may not end in (`ProductionDecl::unextended`).
    pub unextended: Option<(usize, std::collections::BTreeSet<LexemeId>)>,
    pub id: Ident,
    pub lhs: usize,
    pub rhs: Vec<Symbol>,
    /// The name of each child: its grammatical function in the mother.
    pub names: Vec<Ident>,
    pub gaps: Vec<GapRule>,
    pub cost: u32,
    /// (feature, variable) bindings of each child.
    pub child_vars: Vec<Vec<(usize, usize)>>,
    /// (feature, variable) bindings of the mother, in category feature order.
    pub mother_vars: Vec<Option<usize>>,
    pub introduces: Gap,
    pub frame: Option<usize>,
    pub host_check: Option<(usize, usize, HostKind)>,
    /// The Frame Slot kind of a non-prepositional adjunct, for same-head
    /// complement priority.
    pub adjunct_slot: Option<SlotKind>,
    /// The predicative Complement child of a frame-headed phrase.
    pub predicative: Option<usize>,
    pub coordinator: Option<usize>,
    pub distinct_coordinator: Vec<usize>,
    pub requires_coordination: Option<usize>,
    /// The coordinator a correlative coordination's serial tail must end with.
    pub paired_coordinator: Option<LexemeId>,
    /// Children that may not be a coordination.
    pub uncoordinated: Vec<usize>,
    /// Children that may not be, nor be headed by, a Linked Coordination
    /// (`Summary::link`).
    pub unlinked: Vec<usize>,
    /// Lexical children that cost one more where their leaf was not admitted
    /// through the face's scope (`ProductionDecl::scoped`).
    pub scoped: Vec<usize>,
    /// It is a coordination delimited before its coordinator.
    pub delimits: bool,
    /// It is a serial tail of a coordination (*…, [B, and C]*): its `first`
    /// conjunct is a middle one, after the initial conjunct.
    pub tail: bool,
    /// A polysyndetic tail child that must repeat the mother's coordinator.
    pub chained: Option<usize>,
    /// The preposition of the Preposition Phrase Complement it extracts.
    pub gap_preposition: Option<LexemeId>,
    /// The shared child of Right Node Raising, which must be headed by the
    /// preposition of the Gap it fills.
    pub fills_preposition: Option<usize>,
    /// Coordinators a plain serial coordination's tail may not end with.
    pub foreign_coordinators: Vec<LexemeId>,
    /// Two conjuncts that may not share a feature value: (first, last,
    /// feature).
    pub unlike: Option<(usize, usize, usize)>,
    /// An auxiliary Verb Phrase's Complement: the child of an uncoordinated
    /// Production after its head, last, and of a function Category aliasing
    /// the Production's own Category, the participial Verb Phrase Complement
    /// of perfect *have* or of passive or progressive *be*
    /// (`Summary::auxiliary`).
    pub auxiliary: Option<usize>,
    /// The Complements its phrase omits at its right edge.
    pub omission: Omission,
    /// The identity of the Linked Coordination it builds, which has no
    /// Coordinator leaf: it stands for one as the coordination's own
    /// (`Summary::coordinator`) and keeps each chain to one tree
    /// (`Summary::link`).
    pub link: Option<LexemeId>,
    /// The child whose engine fields (preposition, extensions) the mother
    /// inherits.
    pub head: Option<usize>,
    /// The State before any child, its variables bound to their constants.
    start: State,
}

pub(crate) type Key = (Option<usize>, usize);

/// Builds a Production from resolved equations by unifying paths into
/// variables.
#[expect(clippy::too_many_arguments, reason = "one internal call shape")]
#[expect(clippy::too_many_lines, reason = "one Production literal")]
pub(crate) fn resolve(
    grammar: &CompiledGrammar,
    id: Ident,
    lhs: usize,
    children: Vec<(Ident, Symbol, GapRule)>,
    equations: Vec<(Key, Result<Key, u16>)>,
    cost: u32,
    introduces: Gap,
    gap_followed: bool,
    frame: Option<usize>,
    host_check: Option<(usize, usize, HostKind)>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
) -> Result<Production, String> {
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut keys: Vec<Key> = Vec::new();
    let mut parent: Vec<usize> = Vec::new();
    let mut constant: Vec<Option<u16>> = Vec::new();
    let var = |keys: &mut Vec<Key>,
               parent: &mut Vec<usize>,
               constant: &mut Vec<Option<u16>>,
               key: Key| {
        if let Some(i) = keys.iter().position(|k| *k == key) {
            return i;
        }
        keys.push(key);
        parent.push(parent.len());
        constant.push(None);
        keys.len() - 1
    };
    for (path, target) in equations {
        let a = var(&mut keys, &mut parent, &mut constant, path);
        match target {
            Ok(other) => {
                let b = var(&mut keys, &mut parent, &mut constant, other);
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                if ra != rb {
                    match (constant[ra], constant[rb]) {
                        (Some(x), Some(y)) if x != y => {
                            return Err("contradictory feature equations".to_owned());
                        }
                        (Some(x), _) | (_, Some(x)) => constant[rb] = Some(x),
                        _ => {}
                    }
                    parent[ra] = rb;
                }
            }
            Err(value) => {
                let r = find(&mut parent, a);
                if constant[r].is_some_and(|c| c != value) {
                    return Err("contradictory feature requirements".to_owned());
                }
                constant[r] = Some(value);
            }
        }
    }
    // Renumber roots densely.
    let mut roots = Vec::new();
    let slot_of: Vec<usize> = (0..keys.len())
        .map(|i| {
            let r = find(&mut parent, i);
            roots.iter().position(|&x| x == r).unwrap_or_else(|| {
                roots.push(r);
                roots.len() - 1
            })
        })
        .collect();
    let constants: Vec<Option<u16>> = roots.iter().map(|&r| constant[r]).collect();
    if constants.len() > MAX_PRODUCTION_VARS {
        return Err(format!(
            "unifies {} feature variables, more than {MAX_PRODUCTION_VARS}",
            constants.len()
        ));
    }
    if grammar.category_features[lhs].len() > MAX_CATEGORY_FEATURES {
        return Err(format!(
            "its mother carries {} features, more than {MAX_CATEGORY_FEATURES}",
            grammar.category_features[lhs].len()
        ));
    }
    if distinct_coordinator.len() > MAX_DISTINCT_COORDINATORS {
        return Err(format!(
            "marks {} conjuncts distinct, more than {MAX_DISTINCT_COORDINATORS}",
            distinct_coordinator.len()
        ));
    }
    let mut vars = [FeatureValue::UNSET; MAX_PRODUCTION_VARS];
    for (slot, value) in vars.iter_mut().zip(constants) {
        *slot = value.map_or(FeatureValue::UNSET, FeatureValue::new);
    }
    let mut child_vars = vec![Vec::new(); children.len()];
    let mut mother_vars = vec![None; grammar.category_features[lhs].len()];
    for (i, key) in keys.iter().enumerate() {
        match key {
            (Some(child), feature) => child_vars[*child].push((*feature, slot_of[i])),
            (None, feature) => {
                let position = grammar.category_features[lhs]
                    .iter()
                    .position(|f| f == feature)
                    .expect("validated mother feature");
                mother_vars[position] = Some(slot_of[i]);
            }
        }
    }
    let head = children.iter().position(|(name, ..)| name == "head");
    let head_line = children
        .iter()
        .map(|(name, ..)| matches!(name.as_str(), "head" | "first" | "last" | "rest"))
        .collect();
    let names = children.iter().map(|(name, ..)| *name).collect();
    let (rhs, gaps) = children.into_iter().map(|(_, s, g)| (s, g)).unzip();
    if introduces != Gap::None && grammar.category_gap[lhs] & introduces.bit() == 0 {
        return Err("introduces a gap on a category that cannot contain one".to_owned());
    }
    Ok(Production {
        exclusions: Vec::new(),
        joint_exclusions: Vec::new(),
        joint_conditions: Vec::new(),
        resolutions: Vec::new(),
        resolution_nonlast: Vec::new(),
        resolution_last: None,
        carried: Vec::new(),
        weights: Vec::new(),
        head_line,
        unextended: None,
        adjunct_slot: None,
        predicative: None,
        paired_coordinator: None,
        uncoordinated: Vec::new(),
        unlinked: Vec::new(),
        scoped: Vec::new(),
        delimits: false,
        tail: false,
        chained: None,
        gap_preposition: None,
        fills_preposition: None,
        foreign_coordinators: Vec::new(),
        unlike: None,
        auxiliary: None,
        omission: Omission::None,
        link: None,
        id,
        lhs,
        rhs,
        names,
        gaps,
        cost,
        child_vars,
        mother_vars,
        introduces,
        frame,
        host_check,
        coordinator,
        distinct_coordinator,
        requires_coordination,
        head,
        start: State {
            first_glues: None,
            last_glues: false,
            vars,
            gap: None,
            // A Gap the Production introduces before a later child is not
            // at its right edge.
            gap_right_edge: !gap_followed,
            shared_gap_right_edge: true,
            gap_preposition: None,
            conjoined: false,
            leaves: 0,
            head_final: false,
            final_preposition: None,
            omitted_complement: 0,
            ellipsis_final: false,
            after_unlike_omissions: false,
            conjunct_omitted: None,
            unlike_omissions: false,
            preposition: None,
            extensions: 0,
            coordinator: None,
            conjunct_coordinators: [None; MAX_DISTINCT_COORDINATORS],
            other_preposition: None,
            mixed_prepositions: false,
            conjunct_hosts: u8::MAX,
            unlike_value: None,
            nonlast: [FeatureValue::UNSET; MAX_RESOLUTIONS],
            last: [FeatureValue::UNSET; MAX_RESOLUTIONS],
            joint_hits: 0,
            host_extensions: 0,
            adjunct_extension: None,
            internal_comma: false,
            unwritten_period: false,
            headed_link: None,
        },
    })
}

/// The finite grammatical summary of a completed Constituent: everything a
/// parent may inspect. Two Constituents with equal summaries are packed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[expect(clippy::struct_excessive_bools, reason = "independent packed facts")]
pub(crate) struct Summary {
    pub first_glues: bool,
    pub last_glues: bool,
    /// Feature values in Category feature order; slots past the Category's
    /// features stay unset.
    pub values: [FeatureValue; MAX_CATEGORY_FEATURES],
    pub gap: Gap,
    /// Its Gap is at its right edge: no child follows it, here or in the
    /// child it comes from. Right Node Raising shares only such a Gap.
    pub gap_right_edge: bool,
    /// The preposition of an extracted Preposition Phrase Complement.
    pub gap_preposition: Option<LexemeId>,
    /// Its lexical leaves, counted to two.
    pub leaves: u8,
    /// It ends in its head (through conjuncts of a coordination), not in a
    /// dependent of it.
    pub head_final: bool,
    /// The preposition of the Preposition Phrase dependent it ends in, through
    /// its head line, when that preposition closes some `unextended` child.
    pub final_preposition: Option<LexemeId>,
    /// The Complements omitted by the bare noun phrase it ends in, through
    /// any of its last children, as an index into
    /// [`CompiledGrammar::extension_sets`] (0: none). A Preposition Phrase
    /// right after it headed by one of their prepositions is that noun's
    /// Complement, never a dependent of another host
    /// (`english-lexical-analysis.md` §2, same-head complement priority).
    pub omitted_complement: u16,
    /// It ends in an ellipsis site, through its last child
    /// (`ProductionDecl::elliptical`): before any punctuation, only a head it
    /// is a dependent of follows it.
    pub ellipsis_final: bool,
    /// It is a coordination whose conjuncts omit unlike Complements, the last
    /// conjunct's `omitted_complement`: a Preposition Phrase right after it
    /// may be a dependent of the whole coordination, which cannot share it as
    /// a Complement (*the type and amount of mana*), but of no other host.
    pub unlike_omissions: bool,
    /// The preposition and complement frame heading a Preposition Phrase.
    pub preposition: Option<(LexemeId, usize)>,
    /// A coordination's second preposition (*into your hand or onto the
    /// battlefield*).
    pub other_preposition: Option<LexemeId>,
    /// A serial tail's last conjunct's value of each resolved feature.
    pub last_resolved: [FeatureValue; MAX_RESOLUTIONS],
    /// Complements that would extend this phrase's selected frame, as an
    /// index into [`CompiledGrammar::extension_sets`].
    pub extensions: u16,
    /// The coordinator of a coordination.
    pub coordinator: Option<LexemeId>,
    /// The Host Categories it may modify unselected: a Preposition Phrase's
    /// frame's, every conjunct's for a coordination, any for the rest.
    pub hosts: u8,
    /// It begins with a fronted dependent of its head, which is its last
    /// child: one set off by a comma (*When …, …*, *Until end of turn, …*)
    /// or a preposed Adjunct without one (*instead put …*). Such a Clause
    /// hosts no comma-set post-dependent, which attaches inside the fronted
    /// one (`english-lexical-analysis.md` §2). Unless it is a
    /// `preposed_predicate`, it is never the initial conjunct of a
    /// coordination undelimited before its coordinator: the fronted
    /// dependent scopes over the whole coordination.
    pub fronted: bool,
    /// Its fronted dependent is a Predicate Adjunct preposed without a comma
    /// (*instead put …*, *instead puts …*). Its scope is ambiguous between
    /// the initial conjunct of a coordination and the whole coordination
    /// (*Yesterday Ed was taken ill and the lecture's been cancelled*; CGEL,
    /// Ch. 15 §1.3.4, pp. 1285–1286), so it may be that initial conjunct.
    pub preposed_predicate: bool,
    /// It contains a delimiting comma that no bracket closes: it is, or has at
    /// any depth, the host of a comma-set post-dependent (*[this creature
    /// dies, if …]*, *[X is half …, rounded down]*). Only a stronger indicator
    /// absorbs such a comma's closing mark (CGEL, Ch. 20 §3.2.5, p. 1745
    /// [25]), so it runs to the end of its sentence, and a dependent set off
    /// before its head by a comma ends at that comma, so it never contains
    /// one: an intervening *if* follows the trigger condition, never inside
    /// it (`docs/oracle-style-guide.md` §3, Commas and the serial comma; §8,
    /// Intervening if; [CR#603.4]). A dependent after its head, which runs to
    /// the end of its sentence, may contain one (*unless [its controller pays
    /// {X}, where X is …]*). The comma of a coordination delimited before its
    /// coordinator separates coordinates rather than delimiting one, and
    /// needs no closing mark (CGEL, Ch. 20 §3.1, pp. 1736–1737 [3]–[4];
    /// §3.2.1, p. 1739), so it sets no bit (*If [A are attacking, and you
    /// both own and control them], …*).
    pub internal_comma: bool,
    /// It is an uncoordinated auxiliary Verb Phrase (`Production::auxiliary`):
    /// it hosts no post-verbal Adjunct, which attaches to the lexical
    /// participial Verb Phrase it ends in (`english-lexical-analysis.md` §2).
    pub auxiliary: bool,
    /// It ends in the final period of a quotation, which is not written
    /// (`chart::quotations`): no other period follows it there.
    pub unwritten_period: bool,
    /// The identity of the Linked Coordination it is, or that its head line
    /// ends in through `head` children (*then [draw a card, then discard a
    /// card]*). No Conjunct of a Linked Coordination carries the
    /// coordination's own: a chain linked by one connective has one tree,
    /// its serial (`Production::link`).
    pub link: Option<LexemeId>,
}

/// A lexical leaf as a child sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeafFacts {
    pub binding: deckmaste_lexical::Binding,
    pub integer: Option<i32>,
    pub notation: Option<deckmaste_lexical::Numeral>,
    pub lexeme: Option<LexemeId>,
    pub category: LexicalCategory,
    /// Feature values by feature id.
    pub features: [Option<u16>; MAX_FEATURES],
}

pub(crate) enum ChildView<'a> {
    Node(&'a Summary),
    Leaf(&'a LeafFacts),
    Punct,
}

/// The partial state of a Production whose first children have been found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[expect(clippy::struct_excessive_bools, reason = "independent packed facts")]
pub(crate) struct State {
    pub first_glues: Option<bool>,
    pub last_glues: bool,
    vars: [FeatureValue; MAX_PRODUCTION_VARS],
    gap: Option<Gap>,
    gap_right_edge: bool,
    shared_gap_right_edge: bool,
    gap_preposition: Option<LexemeId>,
    /// A conjunct has been taken into the coordination.
    conjoined: bool,
    /// Lexical leaves so far, counted to two.
    leaves: u8,
    /// The last child so far is in the head line and ends in its head.
    pub head_final: bool,
    /// The preposition of the Preposition Phrase dependent the last child
    /// so far ends in (`Summary::final_preposition`).
    final_preposition: Option<LexemeId>,
    /// The Complements omitted by the bare noun phrase the last child so far
    /// ends in (`Summary::omitted_complement`).
    omitted_complement: u16,
    /// The last child so far ends in an ellipsis site
    /// (`Summary::ellipsis_final`).
    ellipsis_final: bool,
    /// The last child so far is a coordination with unlike omissions
    /// (`Summary::unlike_omissions`).
    after_unlike_omissions: bool,
    /// The Complements the first conjunct so far omits.
    conjunct_omitted: Option<u16>,
    /// Some conjunct so far omits other Complements than the first.
    unlike_omissions: bool,
    preposition: Option<(LexemeId, usize)>,
    extensions: u16,
    coordinator: Option<LexemeId>,
    conjunct_coordinators: [Option<LexemeId>; MAX_DISTINCT_COORDINATORS],
    other_preposition: Option<LexemeId>,
    /// The conjuncts of a coordination have more than two prepositions, or
    /// one has none.
    mixed_prepositions: bool,
    /// The Host Categories every conjunct so far admits, or the head admits.
    conjunct_hosts: u8,
    /// The first conjunct's value of an `unlike` feature.
    unlike_value: Option<u16>,
    /// The value of each resolved feature on the conjuncts but the last, and
    /// on the last.
    nonlast: [FeatureValue; MAX_RESOLUTIONS],
    last: [FeatureValue; MAX_RESOLUTIONS],
    /// The joint-exclusion conditions (as bits) that hold so far.
    joint_hits: u8,
    host_extensions: u16,
    adjunct_extension: Option<Extension>,
    /// A child so far carries `Summary::internal_comma` up.
    internal_comma: bool,
    /// The last child so far ends in a period not written
    /// (`Summary::unwritten_period`).
    pub unwritten_period: bool,
    /// The `head` child's `Summary::link`.
    headed_link: Option<LexemeId>,
}

impl Production {
    pub(crate) fn boundaries(&self, position: usize, child: &ChildView<'_>) -> (bool, bool) {
        use crate::reading::glues_after;
        use crate::reading::glues_before;
        let (mut first, mut last) = match child {
            ChildView::Node(summary) => (summary.first_glues, summary.last_glues),
            ChildView::Leaf(leaf) => (glues_before(leaf.binding), glues_after(leaf.binding)),
            ChildView::Punct => match self.rhs[position] {
                Symbol::Punct(_, binding) => (glues_before(binding), glues_after(binding)),
                _ => (false, false),
            },
        };
        if let Symbol::Cat {
            binding: Some(binding),
            ..
        } = self.rhs[position]
        {
            first |= glues_before(binding);
            last |= glues_after(binding);
        }
        (first, last)
    }

    pub(crate) fn spacing_matches(
        &self,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
        gap: &[char],
    ) -> bool {
        if state.first_glues.is_none() {
            return true;
        }
        let (first, _) = self.boundaries(position, child);
        if state.last_glues || first { gap.is_empty() } else { gap == [' '] }
    }

    pub fn start(&self) -> State {
        self.start
    }

    /// Records the head's and an adjunct's frame facts for same-head
    /// complement priority, or `None` when a predicative Complement is a
    /// Preposition Phrase a rival frame selects.
    fn priority(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        summary: &Summary,
    ) -> Option<()> {
        if self.predicative == Some(position)
            && let Some((p, _)) = summary.preposition
            && grammar
                .extension_set(next.extensions)
                .contains(&Extension::Predicative(p))
        {
            return None;
        }
        if self.head == Some(position) {
            next.preposition = summary.preposition;
            next.other_preposition = summary.other_preposition;
            next.extensions = summary.extensions;
            next.conjunct_hosts = summary.hosts;
        }
        if let Some((host, adjunct, _)) = self.host_check {
            if position == host {
                next.host_extensions = summary.extensions;
            }
            if position == adjunct {
                next.adjunct_extension = summary
                    .preposition
                    .as_ref()
                    .map(|(p, _)| Extension::Preposition(*p))
                    .or(self.adjunct_slot.map(Extension::Slot));
            }
        }
        Some(())
    }

    /// Whether the phrase so far ends in its head: the last child is in the
    /// head line and itself ends in its head (a leaf does). A coordinator
    /// leaves it as its first conjunct left it, so that the last conjunct's
    /// item records whether the first ended in its head (the Ranking Key's
    /// Attachment Class reads it there); the last conjunct then sets it.
    fn track_head_final(&self, next: &mut State, position: usize, child: &ChildView<'_>) {
        match child {
            ChildView::Node(summary) => {
                next.head_final = self.head_line[position] && summary.head_final;
            }
            ChildView::Leaf(_) if self.coordinator == Some(position) => {}
            ChildView::Leaf(_) => next.head_final = self.head_line[position],
            ChildView::Punct => {}
        }
    }

    /// The preposition of the Preposition Phrase dependent the phrase so far
    /// ends in, or `None` if child `position` is an `unextended` child that
    /// ends in one its Production closes.
    fn track_final_preposition(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        match child {
            ChildView::Node(summary) => {
                if let Some((at, closed)) = &self.unextended
                    && *at == position
                    && summary
                        .final_preposition
                        .is_some_and(|p| closed.contains(&p))
                {
                    return None;
                }
                next.final_preposition = if self.head_line[position] {
                    summary.final_preposition
                } else {
                    summary
                        .preposition
                        .map(|(p, _)| p)
                        .filter(|p| grammar.unextended_prepositions.contains(p))
                };
            }
            ChildView::Leaf(_) => next.final_preposition = None,
            ChildView::Punct => {}
        }
        Some(())
    }

    /// Whether the phrase so far ends in an ellipsis site, or `None` if child
    /// `position` comes right after one and is neither the head the phrase
    /// ending in it is a dependent of nor a coordinator
    /// (`ProductionDecl::elliptical`). Punctuation ends the site's reach.
    fn track_ellipsis_final(
        &self,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        match child {
            ChildView::Punct => next.ellipsis_final = false,
            _ if next.ellipsis_final
                && self.head != Some(position)
                && self.coordinator != Some(position) =>
            {
                return None;
            }
            ChildView::Node(summary) => next.ellipsis_final = summary.ellipsis_final,
            ChildView::Leaf(_) => next.ellipsis_final = false,
        }
        Some(())
    }

    /// The Complements omitted by the bare noun phrase the phrase so far ends
    /// in, or `None` if the child is a Preposition Phrase right after one and
    /// headed by a preposition it omits: that phrase is the bare noun's own
    /// Complement, which no other host may take. Only a coordination whose
    /// conjuncts omit unlike Complements may take it as its own dependent.
    fn track_omitted_complement(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        match child {
            ChildView::Node(summary) => {
                if self.coordinator.is_some() && self.head_line[position] {
                    let first = *next
                        .conjunct_omitted
                        .get_or_insert(summary.omitted_complement);
                    next.unlike_omissions |=
                        summary.unlike_omissions || first != summary.omitted_complement;
                }
                if next.omitted_complement != 0 && !next.after_unlike_omissions {
                    let omitted = grammar.extension_set(next.omitted_complement);
                    let heads = [
                        summary.preposition.map(|(p, _)| p),
                        summary.other_preposition,
                    ];
                    if heads
                        .into_iter()
                        .flatten()
                        .any(|p| omitted.contains(&Extension::Preposition(p)))
                    {
                        return None;
                    }
                }
                next.omitted_complement = summary.omitted_complement;
                next.after_unlike_omissions = summary.unlike_omissions;
            }
            ChildView::Leaf(_) | ChildView::Punct => {
                next.omitted_complement = 0;
                next.after_unlike_omissions = false;
            }
        }
        Some(())
    }

    /// The lexical leaves child `position` adds, counted to two, or `None` if
    /// it is not the weight the Production requires.
    fn weigh(&self, position: usize, child: &ChildView<'_>) -> Option<u8> {
        let leaves = match child {
            ChildView::Node(summary) => summary.leaves,
            ChildView::Leaf(_) => 1,
            ChildView::Punct => 0,
        };
        match self.weights.iter().find(|(at, _)| *at == position) {
            Some((_, Weight::Light)) if leaves != 1 => None,
            Some((_, Weight::Heavy)) if leaves < 2 => None,
            _ => Some(leaves),
        }
    }

    /// Takes child `position`'s Gap into `next` under its Gap Rule, or
    /// `None` if the rule rejects it.
    fn take_gap(&self, next: &mut State, position: usize, gap: Gap) -> Option<()> {
        let open = next.gap.is_some_and(|g| g != Gap::None);
        match self.gaps[position] {
            GapRule::Closed => (gap == Gap::None).then_some(()),
            GapRule::Require(rule) => (gap == rule).then_some(()),
            GapRule::Raise(rule) => {
                if gap != rule || open {
                    return None;
                }
                next.gap = Some(rule);
                Some(())
            }
            GapRule::Transparent if gap == Gap::None => Some(()),
            GapRule::Transparent => {
                if open {
                    return None;
                }
                next.gap = Some(gap);
                Some(())
            }
            GapRule::Shared => match next.gap {
                Some(g) if g != gap => None,
                _ => {
                    next.gap = Some(gap);
                    Some(())
                }
            },
        }
    }

    /// Takes a conjunct's Host Categories and prepositions into a
    /// coordination: Preposition Phrases keep up to two prepositions (*from
    /// black and from red*, *into your hand or onto the battlefield*).
    fn conjoin(next: &mut State, summary: &Summary) {
        next.conjunct_hosts &= summary.hosts;
        // Conjuncts that would take the same Complements leave the
        // coordination extendable by them (*the top or bottom of their
        // library* takes its *of*-phrase as a shared Complement, never as an
        // Adjunct).
        if !next.conjoined {
            next.extensions = summary.extensions;
            next.conjoined = true;
        } else if next.extensions != summary.extensions {
            next.extensions = 0;
        }
        let own = [
            summary.preposition.map(|(p, _)| p),
            summary.other_preposition,
        ];
        if own[0].is_none() {
            next.mixed_prepositions = true;
        }
        for q in own.into_iter().flatten() {
            match (next.preposition, next.other_preposition) {
                (None, _) => next.preposition = summary.preposition,
                (Some((p, _)), _) if p == q => {}
                (_, Some(o)) if o == q => {}
                (_, None) => next.other_preposition = Some(q),
                _ => next.mixed_prepositions = true,
            }
        }
    }

    /// Records the first unlike conjunct's value, or `None` if the last one
    /// shares it (or either has none).
    fn unlike_conjuncts(
        &self,
        next: &mut State,
        position: usize,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) -> Option<()> {
        if let Some((first, last, feature)) = self.unlike {
            let value = lookup(feature);
            if position == first {
                next.unlike_value = value;
            } else if position == last
                && (value.is_none() || next.unlike_value.is_none() || value == next.unlike_value)
            {
                return None;
            }
        }
        Some(())
    }

    /// Records the joint-exclusion conditions child `position` meets, or
    /// `None` once every condition of one exclusion holds.
    fn joint_exclusion(
        &self,
        next: &mut State,
        position: usize,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) -> Option<()> {
        if self.joint_conditions.is_empty() {
            return Some(());
        }
        for (k, &(at, feature, value)) in self.joint_conditions.iter().enumerate() {
            if at == position && lookup(feature) == Some(value) {
                next.joint_hits |= 1 << k;
            }
        }
        let mut start = 0;
        for &end in &self.joint_exclusions {
            let mask = u8::try_from((1u16 << end) - (1u16 << start)).expect("validated");
            if next.joint_hits & mask == mask {
                return None;
            }
            start = end;
        }
        Some(())
    }

    /// Takes a child's values of the resolved features into the
    /// coordination: those of the conjuncts but the last (which agree by
    /// equation), or the last conjunct's.
    fn resolve(
        &self,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) {
        let nonlast = self.resolution_nonlast.contains(&position);
        let last = match self.resolution_last {
            Some((at, from_tail)) if at == position => Some(from_tail),
            _ => None,
        };
        for (i, &(feature, ..)) in self.resolutions.iter().enumerate() {
            if nonlast && let Some(value) = lookup(feature) {
                next.nonlast[i] = FeatureValue::new(value);
            }
            match (last, child) {
                (Some(true), ChildView::Node(summary)) => next.last[i] = summary.last_resolved[i],
                (Some(false), _) => {
                    next.last[i] = lookup(feature).map_or(FeatureValue::UNSET, FeatureValue::new);
                }
                _ => {}
            }
        }
    }

    /// The engine facts a lexical leaf child adds.
    fn accept_leaf(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        leaf: &LeafFacts,
    ) {
        if self.coordinator == Some(position) {
            next.coordinator = leaf.lexeme;
        }
        if let Symbol::Leaf(Test::PrepositionFrame(frame)) = &self.rhs[position] {
            next.preposition = leaf.lexeme.map(|l| (l, *frame));
        }
        let key = match (&self.rhs[position], leaf.lexeme) {
            (Symbol::Leaf(Test::Frame(frame) | Test::FreeFrame(frame)), Some(lexeme)) => {
                Some((lexeme, Some(*frame)))
            }
            (
                Symbol::Leaf(
                    Test::FreeLexical(_) | Test::FreeOptional(_) | Test::FreeNotActiveOnly(_),
                ),
                Some(lexeme),
            ) => Some((lexeme, None)),
            _ => None,
        };
        if let Some(key) = key {
            next.extensions = grammar.extensions.get(&key).copied().unwrap_or(0);
        }
        if self.host_check.is_some_and(|(host, _, _)| host == position) {
            next.host_extensions = next.extensions;
        }
    }

    /// Whether the Production begins with a fronted dependent of its head,
    /// which is its last child: one set off by a comma (*When …, …*) or a
    /// preposed Adjunct without one (*instead put …*) (`Summary::fronted`).
    pub(crate) fn fronts(&self) -> bool {
        self.coordinator.is_none()
            && self.head == Some(self.rhs.len() - 1)
            && ((self.rhs.len() > 2
                && matches!(self.rhs[self.rhs.len() - 2], Symbol::Punct(',', _)))
                || self
                    .host_check
                    .is_some_and(|(host, adjunct, _)| adjunct < host))
    }

    /// Whether a fronted dependent's scope rules this child out: a fronted
    /// dependent scopes over a comma-set post-dependent of its Clause, never
    /// inside the post-dependent's host, and, unless it is a Predicate
    /// Adjunct preposed without a comma (`Summary::preposed_predicate`), over
    /// a whole coordination, never its initial conjunct
    /// (`english-lexical-analysis.md` §2). The initial conjunct of a
    /// coordination delimited before its coordinator may have its own (*[If
    /// you do, …], and [if you don't, …]*), and so may a middle conjunct of a
    /// serial one, since a fronted dependent there cannot scope over the
    /// whole (*C1, [if …, C2], and C3*).
    fn comma_scope_excludes(&self, position: usize, summary: &Summary) -> bool {
        let first_conjunct = self.coordinator.is_some()
            && !self.delimits
            && !self.tail
            && self.names[position] == "first";
        summary.fronted
            && ((first_conjunct && !summary.preposed_predicate)
                || (position == 0 && self.hosts_comma_set_dependent()))
    }

    /// Whether the Production hosts a comma-set post-dependent: its head
    /// comes first, followed by a comma.
    pub(crate) fn hosts_comma_set_dependent(&self) -> bool {
        self.head == Some(0) && self.rhs.len() > 2 && matches!(self.rhs[1], Symbol::Punct(',', _))
    }

    /// Whether child `position` is set off before the head by a comma: a
    /// fronted dependent (*When …, …*), which ends at that comma.
    fn set_off_before_head(&self, position: usize) -> bool {
        self.head.is_some_and(|head| {
            position < head
                && self.rhs[position + 1..head]
                    .iter()
                    .any(|symbol| matches!(symbol, Symbol::Punct(',', _)))
        })
    }

    /// Whether child `position` is the head of a Production of its own
    /// Category whose every later child is an Adjunct: a post-verbal Adjunct's
    /// host, which an auxiliary Verb Phrase never is (`Summary::auxiliary`).
    pub(crate) fn post_adjunct_host(&self, position: usize) -> bool {
        position == 0
            && self.head == Some(0)
            && self.coordinator.is_none()
            && self.rhs.len() > 1
            && matches!(self.rhs[0], Symbol::Cat { category, .. } if category == self.lhs)
            && self.names[1..]
                .iter()
                .zip(&self.rhs[1..])
                .all(|(name, symbol)| matches!(symbol, Symbol::Punct(..)) || name == "adjunct")
    }

    /// Whether child `position`'s slot admits the phrase: a selected `PP`
    /// slot its preposition, an unselected adjunct its Host Categories.
    fn slot_admits(&self, position: usize, summary: &Summary) -> bool {
        let Symbol::Cat {
            prepositions, host, ..
        } = &self.rhs[position]
        else {
            return true;
        };
        if let Some(list) = prepositions {
            let Some((prep, _)) = summary.preposition.as_ref() else {
                return false;
            };
            if !list.contains(prep)
                || summary
                    .other_preposition
                    .is_some_and(|o| !list.contains(&o))
            {
                return false;
            }
        }
        !host.is_some_and(|kind| summary.hosts & crate::frames::host_bit(kind) == 0)
    }

    /// Records the Linked Coordination that child `position`, the head, is
    /// or is headed by (`Summary::link`), or `None` if the child may not be
    /// one: an `unlinked` child, or a Conjunct of a Linked Coordination of the
    /// same identity.
    fn track_link(&self, next: &mut State, position: usize, summary: &Summary) -> Option<()> {
        if self.head == Some(position) {
            next.headed_link = summary.link;
        }
        let same_chain = self.link == summary.link && self.distinct_coordinator.contains(&position);
        if summary.link.is_some() && (self.unlinked.contains(&position) || same_chain) {
            return None;
        }
        Some(())
    }

    /// Adds child `position` to `state`, or `None` if it does not fit.
    pub fn accept(
        &self,
        grammar: &CompiledGrammar,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<State> {
        let mut next = *state;
        if state.gap.is_some_and(|gap| gap != Gap::None) {
            next.gap_right_edge = false;
        }
        // A free child does not begin with an atom glued to the one before.
        if let (
            Symbol::Cat {
                binding: Some(deckmaste_lexical::Binding::Free),
                ..
            },
            ChildView::Node(summary),
        ) = (&self.rhs[position], child)
            && summary.first_glues
        {
            return None;
        }
        let (first, last) = self.boundaries(position, child);
        next.first_glues.get_or_insert(first);
        next.last_glues = last;
        next.unwritten_period =
            matches!(child, ChildView::Node(summary) if summary.unwritten_period);
        next.leaves = (next.leaves + self.weigh(position, child)?).min(2);
        self.track_head_final(&mut next, position, child);
        self.track_final_preposition(grammar, &mut next, position, child)?;
        self.track_omitted_complement(grammar, &mut next, position, child)?;
        self.track_ellipsis_final(&mut next, position, child)?;
        let lookup = |feature: usize| -> Option<u16> {
            match child {
                ChildView::Node(summary) => {
                    let category = match &self.rhs[position] {
                        Symbol::Cat { category, .. } => *category,
                        _ => return None,
                    };
                    let index = grammar.category_features[category]
                        .iter()
                        .position(|f| *f == feature)?;
                    summary.values[index].get()
                }
                ChildView::Leaf(leaf) => leaf.features[feature],
                ChildView::Punct => None,
            }
        };
        if self
            .exclusions
            .iter()
            .any(|&(at, feature, value)| at == position && lookup(feature) == Some(value))
            || self
                .carried
                .iter()
                .any(|&(at, feature, value)| at == position && lookup(feature) != Some(value))
        {
            return None;
        }
        self.unlike_conjuncts(&mut next, position, &lookup)?;
        self.joint_exclusion(&mut next, position, &lookup)?;
        self.resolve(&mut next, position, child, &lookup);
        for &(feature, var) in &self.child_vars[position] {
            if let Some(value) = lookup(feature) {
                match next.vars[var].get() {
                    Some(bound) if bound != value => return None,
                    _ => next.vars[var] = FeatureValue::new(value),
                }
            }
        }
        match child {
            ChildView::Node(summary) => {
                if !self.slot_admits(position, summary) {
                    return None;
                }
                if summary.gap != Gap::None {
                    next.gap_right_edge = summary.gap_right_edge;
                    if matches!(self.gaps[position], GapRule::Shared) {
                        next.shared_gap_right_edge &= summary.gap_right_edge;
                    }
                }
                self.take_gap(&mut next, position, summary.gap)?;
                if let Some(p) = summary.gap_preposition {
                    if next.gap_preposition.is_some_and(|q| q != p) {
                        return None;
                    }
                    next.gap_preposition = Some(p);
                }
                if self.fills_preposition == Some(position)
                    && summary.preposition.map(|(p, _)| p) != next.gap_preposition
                {
                    return None;
                }
                if self.uncoordinated.contains(&position) && summary.coordinator.is_some() {
                    return None;
                }
                self.track_link(&mut next, position, summary)?;
                // A comma that no bracket closes stays inside every phrase
                // that contains it, to the end of its sentence.
                if summary.internal_comma {
                    if self.set_off_before_head(position) {
                        return None;
                    }
                    next.internal_comma = true;
                }
                if self.comma_scope_excludes(position, summary) {
                    return None;
                }
                if summary.auxiliary && self.post_adjunct_host(position) {
                    return None;
                }
                if self.requires_coordination == Some(position)
                    && (summary.coordinator.is_none() || !summary.gap_right_edge)
                {
                    return None;
                }
                if self.distinct_coordinator.contains(&position)
                    && let Some(c) = summary.coordinator
                    && let Some(slot) = next.conjunct_coordinators.iter_mut().find(|s| s.is_none())
                {
                    *slot = Some(c);
                }
                if self.coordinator.is_some() && matches!(self.gaps[position], GapRule::Shared) {
                    Self::conjoin(&mut next, summary);
                }
                if self.chained == Some(position) && summary.coordinator != next.coordinator {
                    return None;
                }
                if self.coordinator == Some(position)
                    && summary
                        .coordinator
                        .is_some_and(|c| self.foreign_coordinators.contains(&c))
                {
                    return None;
                }
                if self.coordinator == Some(position) {
                    if self
                        .paired_coordinator
                        .is_some_and(|p| summary.coordinator != Some(p))
                    {
                        return None;
                    }
                    next.coordinator = summary.coordinator;
                }
                self.priority(grammar, &mut next, position, summary)?;
            }
            ChildView::Leaf(leaf) => self.accept_leaf(grammar, &mut next, position, leaf),
            ChildView::Punct => {}
        }
        Some(next)
    }

    /// The mother's summary once every child has been accepted.
    pub fn finish(&self, grammar: &CompiledGrammar, state: &State) -> Option<Summary> {
        if state.coordinator.is_some() && state.conjunct_coordinators.contains(&state.coordinator) {
            return None;
        }
        if self.host_check.is_some()
            && let Some(e) = &state.adjunct_extension
            && grammar.extension_set(state.host_extensions).contains(e)
        {
            return None;
        }
        let gap = if self.introduces == Gap::None {
            state.gap.unwrap_or(Gap::None)
        } else {
            if state.gap.is_some_and(|g| g != Gap::None) {
                return None;
            }
            self.introduces
        };
        if gap != Gap::None && grammar.category_gap[self.lhs] & gap.bit() == 0 {
            return None;
        }
        let coordinated = self.coordinator.is_some();
        let mut values = [FeatureValue::UNSET; MAX_CATEGORY_FEATURES];
        for (value, var) in values.iter_mut().zip(&self.mother_vars) {
            *value = var.map_or(FeatureValue::UNSET, |v| state.vars[v]);
        }
        let mut last_resolved = [FeatureValue::UNSET; MAX_RESOLUTIONS];
        for (i, &(_, differ, at)) in self.resolutions.iter().enumerate() {
            match at {
                // The whole takes the common value, or `differ`.
                Some(at) => {
                    values[at] = match (state.nonlast[i].get(), state.last[i].get()) {
                        (Some(a), Some(b)) if a != b => FeatureValue::new(differ),
                        (None, _) => state.last[i],
                        _ => state.nonlast[i],
                    };
                }
                // A serial tail records its last conjunct's value.
                None => last_resolved[i] = state.last[i],
            }
        }
        Some(Summary {
            first_glues: state.first_glues.unwrap_or(false),
            last_glues: state.last_glues,
            values,
            gap,
            gap_right_edge: state.gap_right_edge && state.shared_gap_right_edge,
            gap_preposition: match gap {
                Gap::PP if self.introduces == Gap::PP => self.gap_preposition,
                Gap::PP => state.gap_preposition,
                _ => None,
            },
            leaves: state.leaves,
            head_final: state.head_final,
            final_preposition: state.final_preposition,
            ellipsis_final: self.omission == Omission::Ellipsis || state.ellipsis_final,
            omitted_complement: if self.omission == Omission::Selected {
                state.extensions
            } else {
                state.omitted_complement
            },
            unlike_omissions: coordinated
                && state.unlike_omissions
                && state.omitted_complement != 0,
            preposition: if coordinated && state.mixed_prepositions {
                None
            } else {
                state.preposition
            },
            other_preposition: if state.mixed_prepositions {
                None
            } else {
                state.other_preposition
            },
            last_resolved,
            extensions: state.extensions,
            coordinator: self.link.or(state.coordinator),
            hosts: match state.preposition {
                Some((_, frame)) if !coordinated => grammar.preposition_frames[frame].hosts.mask(),
                _ => state.conjunct_hosts,
            },
            fronted: self.fronts(),
            preposed_predicate: self.fronts()
                && self.host_check.is_some_and(|(host, adjunct, kind)| {
                    adjunct < host
                        && kind == HostKind::Predicate
                        && !self.set_off_before_head(adjunct)
                }),
            internal_comma: state.internal_comma || self.hosts_comma_set_dependent(),
            auxiliary: self.auxiliary.is_some(),
            unwritten_period: state.unwritten_period,
            link: self.link.or(state.headed_link),
        })
    }
}

impl CompiledGrammar {
    /// The extension list an interned index names.
    pub(crate) fn extension_set(&self, index: u16) -> &[Extension] {
        &self.extension_sets[usize::from(index)]
    }

    /// Whether a lexical leaf satisfies a test.
    pub(crate) fn leaf_matches(&self, test: &Test, leaf: &LeafFacts) -> bool {
        let facts = leaf.lexeme.as_ref().and_then(|l| self.lexemes.get(l));
        self.leaf_matches_with_facts(test, leaf, facts)
    }

    /// The same test when the caller already has the lexeme's bound facts.
    pub(crate) fn leaf_matches_with_facts(
        &self,
        test: &Test,
        leaf: &LeafFacts,
        facts: Option<&LexemeFacts>,
    ) -> bool {
        match test {
            Test::Lexical(category) => leaf.category == *category,
            Test::FreeLexical(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| f.binding == deckmaste_lexical::Binding::Free)
            }
            Test::FreeOptional(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| {
                        f.binding == deckmaste_lexical::Binding::Free && f.optional_complement
                    })
            }
            Test::FreeNotActiveOnly(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| {
                        f.binding == deckmaste_lexical::Binding::Free && !f.active_only
                    })
            }
            Test::Numeral(notation) => leaf.notation == Some(*notation),
            Test::Word(id) => leaf.lexeme.as_ref() == Some(id),
            Test::Words(words) => leaf.lexeme.is_some_and(|id| words.contains(&id)),
            Test::Class(class) => leaf
                .lexeme
                .as_ref()
                .is_some_and(|l| self.classes[*class].contains(l)),
            Test::Frame(frame) => facts.is_some_and(|f| f.frames.contains(frame)),
            Test::FreeFrame(frame) => facts.is_some_and(|f| {
                f.frames.contains(frame) && f.binding == deckmaste_lexical::Binding::Free
            }),
            Test::PrepositionFrame(frame) => {
                facts.is_some_and(|f| f.preposition_frames.contains(frame))
            }
        }
    }

    /// Projects a lexical alternative into the leaf a Production sees.
    pub(crate) fn leaf_facts(&self, reading: &LexicalReading) -> Option<LeafFacts> {
        match reading {
            LexicalReading::Word(value) => {
                let facts = self.lexemes.get(&value.lexeme)?;
                let f = &value.features;
                let verb_form = match value.form {
                    WordForm::Plain => Some("Plain"),
                    // Irrealis *were* is a finite form, like the tensed ones.
                    WordForm::Present | WordForm::Preterite | WordForm::Irrealis => Some("Finite"),
                    WordForm::GerundParticiple => Some("GerundParticiple"),
                    WordForm::PastParticiple => Some("PastParticiple"),
                    _ => None,
                };
                let names = [
                    f.number.map(|n| format!("{n:?}")),
                    f.person.map(|p| format!("{p:?}")),
                    f.tense.map(|t| format!("{t:?}")),
                    verb_form.map(str::to_owned),
                    f.case.map(|c| format!("{c:?}")),
                    Some(inflection_name(value.form)),
                    facts
                        .countability
                        .map(|c| c.as_str().to_owned())
                        .or_else(|| {
                            (facts.countable_plural
                                && f.number == Some(deckmaste_lexical::Number::Plural))
                            .then(|| "Count".to_owned())
                        }),
                    None,
                ];
                let mut features = [None; MAX_FEATURES];
                // A lexeme's own value for a feature outranks a projected one.
                for &(id, value) in &facts.features {
                    features[id].get_or_insert(value);
                }
                for (id, name) in self.projected.iter().zip(names) {
                    if let (Some(id), Some(name)) = (id, name) {
                        let value = self.values[*id].iter().position(|v| *v == name.as_str())?;
                        features[*id].get_or_insert(u16::try_from(value).ok()?);
                    }
                }
                Some(LeafFacts {
                    binding: facts.binding,
                    integer: None,
                    notation: None,
                    lexeme: Some(value.lexeme),
                    category: facts.category,
                    features,
                })
            }
            LexicalReading::Numeral {
                value, notation, ..
            } => Some(LeafFacts {
                binding: deckmaste_lexical::Binding::Free,
                integer: Some(*value),
                notation: Some(*notation),
                lexeme: None,
                category: LexicalCategory::Numeral,
                features: {
                    let number = matches!(
                        notation,
                        deckmaste_lexical::Numeral::Cardinal
                            | deckmaste_lexical::Numeral::Arabic(_)
                    )
                    .then_some(if *value == 1 { "Singular" } else { "Plural" });
                    let sign = match value.cmp(&0) {
                        std::cmp::Ordering::Less => "Negative",
                        std::cmp::Ordering::Equal => "Zero",
                        std::cmp::Ordering::Greater => "Positive",
                    };
                    let mut features = [None; MAX_FEATURES];
                    for (id, name) in [
                        (self.projected[crate::compile::NUMBER], number),
                        (self.projected[crate::compile::SIGN], Some(sign)),
                    ] {
                        let (Some(id), Some(name)) = (id, name) else { continue };
                        if let Some(value) = self.values[id].iter().position(|v| v == name)
                            && let Ok(value) = u16::try_from(value)
                        {
                            features[id].get_or_insert(value);
                        }
                    }
                    features
                },
            }),
            LexicalReading::FlavorWord { .. } => Some(LeafFacts {
                binding: deckmaste_lexical::Binding::Free,
                integer: None,
                notation: None,
                lexeme: None,
                category: LexicalCategory::FlavorWord,
                features: [None; MAX_FEATURES],
            }),
        }
    }
}

/// The `Inflection` value the engine projects from a Word Form: its name.
pub(crate) fn inflection_name(form: WordForm) -> String {
    format!("{form:?}")
}

/// Every lexical value the engine projects onto a grammar feature of this name.
pub(crate) fn projected_values(feature: &str) -> &'static [&'static str] {
    match feature {
        "Number" => &["Singular", "Plural"],
        "Sign" => &["Negative", "Zero", "Positive"],
        "Person" => &["Second", "Third"],
        "Tense" => &["Present", "Past"],
        "VerbForm" => &["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Case" => &["Nominative", "Accusative", "Genitive"],
        "Countability" => &["Count", "Mass"],
        "Inflection" => &[
            "Invariant",
            "IndependentGenitive",
            "Singular",
            "Plural",
            "Plain",
            "Present",
            "Preterite",
            "Irrealis",
            "GerundParticiple",
            "PastParticiple",
        ],
        _ => &[],
    }
}
