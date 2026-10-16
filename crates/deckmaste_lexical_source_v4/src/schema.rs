//! The authored shape of every lexicon and catalog data file.
//!
//! Entries state only what the defaults do not: regular inflections come from
//! `deckmaste_lexical`'s morphology, prepositions are named by lemma, and a
//! Frame names its Frame Family and the prepositions it selects.

use std::collections::BTreeMap;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Case;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Number;
use deckmaste_lexical::Onset;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;
use deckmaste_syntax::frames::Hosts;
use serde::Deserialize;

/// One spelling, or several Spelling Variants.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Spellings {
    One(String),
    Many(Vec<String>),
}

impl Spellings {
    #[must_use]
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Self::One(one) => vec![one.clone()],
            Self::Many(many) => many.clone(),
        }
    }
}

/// A rule of the Comprehensive Rules, written as a citation: `[CR#701.8]`.
/// The citation tool checks it like any other.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
#[serde(try_from = "String")]
pub struct Rule(String);

impl Rule {
    /// The rule number alone, without the citation brackets.
    #[must_use]
    pub fn number(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Rule {
    type Error = String;

    fn try_from(citation: String) -> Result<Self, String> {
        citation
            .strip_prefix("[CR#")
            .and_then(|rest| rest.strip_suffix(']'))
            .filter(|number| {
                !number.is_empty()
                    && number
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '.')
            })
            .map(|number| Self(number.to_owned()))
            .ok_or_else(|| {
                format!("`{citation}` is not one rule written as a bracketed CR citation")
            })
    }
}

/// What fills a Frame Slot, with prepositions named by lemma.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum FillSpec {
    NP,
    PP(Vec<String>),
    Pred,
    AdjP,
    LocativeNP,
    FiniteClause,
    VerblessClause,
    ToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    Nominal,
    Amount,
    Cost,
    /// No Complement: only an intransitive preposition (*out* in *phase out*).
    Intransitive,
}

/// A catenative verb's non-finite Complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Catenation {
    ToInf,
    BareInf,
    Gerund,
}

/// A Subcategorization Frame as authored, in active voice only.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum FrameSpec {
    Intransitive,
    Monotransitive,
    Transitive(FillSpec),
    Ditransitive,
    TransitiveWithPP(Vec<String>),
    TransitiveWithComplement(FillSpec),
    Prepositional(Vec<String>),
    Copular,
    Catenative(Catenation),
    /// An object and a predicative complement: *turn it face up*.
    ComplexTransitive,
    /// An object and a non-finite complement: *cause you to sacrifice*.
    ComplexCatenative(Catenation),
    /// A content clause: *decide whether to flip again*.
    ContentClause,
    Amount,
    TransitiveAmount,
    Cost,
    /// The one Complement of a noun, adjective or determinative.
    Complement(FillSpec),
}

/// A replacement for the regular spelling of some inflection slots of a verb.
/// Unnamed person and number match every slot of the form.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormOverride {
    pub form: WordForm,
    #[serde(default)]
    pub person: Option<Person>,
    #[serde(default)]
    pub number: Option<Number>,
    pub surfaces: Spellings,
}

/// A verb with its full paradigm and active Subcategorization Frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerbEntry {
    pub lemma: String,
    /// The third-person singular present.
    #[serde(default)]
    pub third_singular: Option<Spellings>,
    /// Both the preterite and the past participle.
    #[serde(default)]
    pub past: Option<Spellings>,
    #[serde(default)]
    pub preterite: Option<Spellings>,
    #[serde(default)]
    pub past_participle: Option<Spellings>,
    #[serde(default)]
    pub gerund: Option<Spellings>,
    /// The singular irrealis (*were*), for the one verb that has it.
    #[serde(default)]
    pub irrealis: Option<Spellings>,
    /// Person- or number-specific replacements, applied after the fields above.
    #[serde(default)]
    pub forms: Vec<FormOverride>,
    /// A defective verb with only finite forms (the modals and the negative
    /// auxiliaries): no plain, gerund-participle or past-participle slot, and
    /// a preterite only when one is spelled.
    #[serde(default)]
    pub finite_only: bool,
    /// A negative form of an auxiliary (*can't*, *doesn't*).
    #[serde(default)]
    pub negative: bool,
    /// The verb whose forms these are: a contracted auxiliary (CGEL, Ch. 18
    /// §6.2, pp. 1614–1616) or a negative inflectional form (CGEL, Ch. 9 §2.1,
    /// p. 799) is named by that verb's lemma (*'re* and *aren't* are forms of
    /// *be*), while keeping its own Lexeme id and spellings.
    #[serde(default)]
    pub form_of: Option<String>,
    /// A contracted auxiliary bound to its host (*'ve* in *you've*).
    #[serde(default)]
    pub binding: Option<Binding>,
    /// Every verb has a frame, except an auxiliary the grammar names as a
    /// Closed Word or Lexical Class member (perfect *have*, *hasn't*, *'ve*).
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

/// Noun subclasses with distinct adjunct distributions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum NounKind {
    #[default]
    Ordinary,
    Temporal,
    Manner,
    Locative,
}

/// A noun with its number paradigm and optional Complement frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NounEntry {
    #[serde(default)]
    pub distribution: NounDistribution,
    #[serde(default)]
    pub kind: NounKind,
    /// Ordered notation parameters, for indexed headings.
    #[serde(default)]
    pub parameters: Vec<Vec<deckmaste_syntax::frames::ParameterPart>>,
    pub lemma: String,
    #[serde(default = "count")]
    pub countability: Vec<Countability>,
    #[serde(default)]
    pub plural: Option<Spellings>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

fn count() -> Vec<Countability> {
    vec![Countability::Count]
}

/// A noun written as its lemma alone when every default applies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum NounDecl {
    Lemma(String),
    Entry(NounEntry),
}

impl NounDecl {
    #[must_use]
    pub fn entry(&self) -> NounEntry {
        match self {
            Self::Lemma(lemma) => NounEntry {
                distribution: NounDistribution::Ordinary,
                kind: NounKind::Ordinary,
                parameters: Vec::new(),
                lemma: lemma.clone(),
                countability: count(),
                plural: None,
                frames: Vec::new(),
            },
            Self::Entry(entry) => entry.clone(),
        }
    }
}

/// An adjective. Its inflected grades, when it has them, are declared beside
/// it and become adjectives of their own `Grade`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjectiveEntry {
    #[serde(default)]
    pub position: Option<AdjectivePosition>,
    #[serde(default)]
    pub determination: Option<Determination>,
    #[serde(default)]
    pub fused_head: bool,
    pub lemma: String,
    #[serde(default)]
    pub comparative: Option<String>,
    #[serde(default)]
    pub superlative: Option<String>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

/// The function to which an adjective is restricted (CGEL Ch. 6 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AdjectivePosition {
    Attributive,
    Predicative,
}

/// An adjective written as its lemma alone when it has no grades or frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum AdjectiveDecl {
    Lemma(String),
    Entry(AdjectiveEntry),
}

impl AdjectiveDecl {
    #[must_use]
    pub fn entry(&self) -> AdjectiveEntry {
        match self {
            Self::Lemma(lemma) => AdjectiveEntry {
                position: None,
                determination: None,
                fused_head: false,
                lemma: lemma.clone(),
                comparative: None,
                superlative: None,
                frames: Vec::new(),
            },
            Self::Entry(entry) => entry.clone(),
        }
    }
}

/// Where in a clause an adverb occurs (CGEL Ch. 8 §§17–21).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum AdverbPosition {
    /// Before the subject.
    Front,
    /// Between the subject and the verb, or after the first auxiliary.
    Central,
    /// After the verb and its Complements.
    End,
    /// Inside a phrase, modifying an adjective, adverb, determinative or
    /// numeral rather than a clause.
    Phrasal,
    /// A degree modifier within a scalar phrase.
    Degree,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdverbEntry {
    pub lemma: String,
    pub positions: Vec<AdverbPosition>,
}

/// One complement frame of a preposition with its Host Category.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepositionFrameSpec {
    #[serde(default = "complement_relation")]
    pub relation: deckmaste_syntax::frames::SlotRelation,
    pub complement: FillSpec,
    pub hosts: Hosts,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepositionEntry {
    pub lemma: String,
    pub frames: Vec<PrepositionFrameSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeterminativeEntry {
    #[serde(default)]
    pub noun_kind: Option<NounKind>,
    #[serde(default)]
    pub distribution: DeterminativeDistribution,
    #[serde(default)]
    pub countability: Option<deckmaste_lexical::Countability>,
    #[serde(default)]
    pub number: Option<Number>,
    pub lemma: String,
    /// Spelling Variants, when the lemma is not the only spelling.
    #[serde(default)]
    pub spellings: Option<Vec<String>>,
    /// The Onset each article spelling requires of what follows it.
    #[serde(default)]
    pub article_onsets: BTreeMap<String, Onset>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

/// The distribution of a determinative, including articles and distributive quantifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum DeterminativeDistribution {
    #[default]
    Dependent,
    Independent,
    IndefiniteArticle,
    Distributive,
    /// Selects a singular head or a quantified plural (CGEL, Ch. 5 §3.4, p. 353).
    SingularOrQuantified,
}

/// One cell of a pronoun paradigm.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PronounForm {
    #[serde(default = "invariant")]
    pub form: WordForm,
    pub person: Person,
    pub number: Number,
    #[serde(default)]
    pub case: Option<Case>,
    pub surfaces: Spellings,
}

fn invariant() -> WordForm {
    WordForm::Invariant
}

/// A pronoun Lexeme: a personal, reflexive, relative or interrogative
/// pronoun, with the features Oracle gives it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PronounEntry {
    #[serde(default)]
    pub kind: PronounKind,
    pub lemma: String,
    pub forms: Vec<PronounForm>,
}

/// Personal and existential pronouns have different clause distributions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum PronounKind {
    #[default]
    Personal,
    Existential,
}

/// An affix or clitic, bound to its host.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundEntry {
    /// The identity-bearing name, such as `genitive`.
    pub name: String,
    pub spellings: Vec<String>,
    pub binding: Binding,
}

/// `lexicon/english/closed_class.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedClass {
    pub determinatives: Vec<DeterminativeEntry>,
    pub pronouns: Vec<PronounEntry>,
    pub coordinators: Vec<String>,
    pub subordinators: Vec<String>,
    /// Auxiliaries and modals, including their negative forms.
    pub auxiliaries: Vec<VerbEntry>,
    pub affixes: Vec<BoundEntry>,
    pub clitics: Vec<BoundEntry>,
    /// Braced and other notation symbols, spelled exactly.
    pub symbols: Vec<SymbolEntry>,
}

/// `catalogs/types.ron` [CR#205.2a,205.4a].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Types {
    pub card_types: Vec<NounDecl>,
    pub supertypes: Vec<String>,
}

/// One subtype family of [CR#205.3].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum SubtypeFamily {
    Artifact,
    Battle,
    Creature,
    Dungeon,
    Enchantment,
    Land,
    Planeswalker,
    Spell,
}

/// `catalogs/subtypes.ron` [CR#205.3].
pub type Subtypes = BTreeMap<SubtypeFamily, Vec<NounDecl>>;

/// A CR-enumerated catalog item realized by one or more Lexemes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordAction {
    /// The rule that defines it, such as `[CR#701.8]`.
    pub rule: Rule,
    /// The CR's name for it, such as `Destroy`.
    pub name: String,
    #[serde(default)]
    pub verbs: Vec<VerbEntry>,
    /// Nouns that realize it: the instruction *Monstrosity 3* [CR#701.37a],
    /// or *the Ring* that tempts you [CR#701.54a].
    #[serde(default)]
    pub nouns: Vec<NounDecl>,
    /// Explicit adjective Lexemes associated with the action.
    #[serde(default)]
    pub adjectives: Vec<AdjectiveDecl>,
}

/// One parameter a keyword ability takes on a keyword line.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum KeywordParameter {
    /// A number or `X`: *bushido 2*.
    Amount,
    /// A cost after a space: *equip {2}*.
    Cost,
    /// A cost after an em dash: *ward—Pay 3 life*.
    DashCost,
    /// A verbal or composite cost after an em dash, excluding mana-only costs.
    DashNonManaCost,
    /// A condition after an em dash: *companion—…*.
    DashCondition,
    /// A Keyword Quality, after the preposition named, if any:
    /// *protection from red*, *enchant creature*.
    Quality(Option<String>),
    /// A Keyword Quality written before the keyword: *Goblin offering*.
    QualityBefore,
    /// A card name after the preposition named, if any: *partner with …*.
    Name(Option<String>),
    /// An ability after an em dash, which the keyword labels: *boast — {1}: …*.
    DashAbility,
    /// Power and toughness after an em dash: *prototype {1}{B} — 1/1*.
    DashPowerToughness,
    /// Another keyword after an em dash: *modular—sunburst*.
    DashKeyword,
}

/// The noun stems a Bound Keyword Surface binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum StemClass {
    LandType,
    CreatureType,
    /// Any subtype.
    Subtype,
    CardType,
    Supertype,
}

/// A Bound Keyword Surface and the stems its Keyword Quality draws on.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundSurface {
    pub surface: String,
    pub stems: Vec<StemClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordAbility {
    /// Determinative Lexemes explicitly associated with this catalog item.
    #[serde(default)]
    pub determinatives: Vec<DeterminativeEntry>,
    pub rule: Rule,
    pub name: String,
    pub lemma: String,
    /// Alternative parameter lists; empty means the keyword stands alone.
    #[serde(default)]
    pub parameters: Vec<Vec<KeywordParameter>>,
    #[serde(default)]
    pub bound: Option<BoundSurface>,
    /// Keywords the rule defines as variants, each its own Keyword Lexeme:
    /// *megamorph* [CR#702.37b], *partner—Friends forever* [CR#702.124i].
    #[serde(default)]
    pub variants: Vec<KeywordVariant>,
    /// Verbs the rule defines for acting with the keyword: *cycle*, *crew*.
    #[serde(default)]
    pub verbs: Vec<VerbEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordVariant {
    pub lemma: String,
    #[serde(default)]
    pub parameters: Vec<Vec<KeywordParameter>>,
}

/// Where an ability word is listed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AbilityWordListing {
    /// In [CR#207.2c].
    Rules,
    /// Printed on Vintage-legal cards but omitted by the CR list.
    Printed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbilityWord {
    pub lemma: String,
    pub listed: AbilityWordListing,
}

/// A named kind of counter; its Lexeme is the compound noun `<kind> counter`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterKind {
    pub kind: String,
    /// The rule that names it, when the CR does.
    #[serde(default)]
    pub rule: Option<Rule>,
}

/// A designation and the nouns and adjectives that name it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Designation {
    pub rule: Rule,
    pub name: String,
    #[serde(default)]
    pub nouns: Vec<NounDecl>,
    #[serde(default)]
    pub adjectives: Vec<String>,
}

/// A phase or step [CR#500.1].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TurnPart {
    pub rule: Rule,
    pub noun: NounDecl,
}

/// A predefined token [CR#111.10]: the subtype that names it and any token
/// names it introduces.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub rule: Rule,
    pub name: String,
    pub subtype: String,
    #[serde(default)]
    pub names: Vec<String>,
}

/// A name a card's own text gives an object that is not a card name, with the
/// cards whose text uses it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntroducedName {
    pub name: String,
    pub cards: Vec<String>,
}

/// An Anchor Word [CR#614.12c] with the cards that print it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnchorWord {
    pub word: String,
    pub cards: Vec<String>,
}

/// A notation symbol with catalog-declared lexical classes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolEntry {
    pub surface: String,
    pub classes: Vec<String>,
}

fn complement_relation() -> deckmaste_syntax::frames::SlotRelation {
    deckmaste_syntax::frames::SlotRelation::Complement
}

/// Whether a nominal has an overt determiner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Determination {
    Bare,
    Overt,
}

/// Lexical realizations of the permanent status values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub rule: Rule,
    #[serde(default)]
    pub adjectives: Vec<AdjectiveDecl>,
    #[serde(default)]
    pub prepositions: Vec<PrepositionEntry>,
}

/// Lexically declared nominal distributions in the Oracle register.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum NounDistribution {
    #[default]
    Ordinary,
    /// An atomic count-noun term also used as a bare singular NP.
    BareSingular,
    /// A mass noun accepting a direct cardinal measure without a unit noun.
    DirectMeasure,
    /// Used as a nominal head, with no attributive noun use.
    HeadOnly,
}
