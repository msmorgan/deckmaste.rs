#![allow(
    clippy::pub_underscore_fields,
    reason = "the generated restriction product retains its typed Cast identity even though fixed Bare feature projection does not read the local binding"
)]

use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;
use deckmaste_construction::constructions;

use crate::context::ParseContext;
use crate::parser::LexicalMatch;
use crate::parser::Rule;
use crate::parser::RulePosition;
use crate::parser::ScanInput;
use crate::parser::ownership::RawRenderedClaim;
use crate::render::Render;
use crate::render::Writer;

constructions! {
    vocab ThirdPersonAuxiliary { Doesnt = "doesn't", }
    vocab BareAgreementAuxiliary { Dont = "don't", }
    vocab PredicateNegator { Not = "not", }
    vocab FiniteCopula {
        Is = "is",
        Isnt = "isn't",
        Are = "are",
        Arent = "aren't",
        Was = "was",
        Were = "were",
    }
    vocab BareCopula { Be = "be", }
    vocab PredicativeAdjective { Legendary = "legendary", }
    vocab FaceOrientation { FaceUp = "face up", }
    vocab ObjectOrder { Any = "any", Random = "a random", }
    // One preposition class for the single general prepositional phrase.
    // Which phrase a verb selects is a fact of the verb's declared valence,
    // not of the preposition, so this vocabulary is flat.
    vocab Preposition {
        feature PrepositionClass = SelectedOnly;
        After = "after" { feature PrepositionClass = AdjunctCapable; },
        Among = "among",
        At = "at",
        Before = "before" { feature PrepositionClass = AdjunctCapable; },
        During = "during" { feature PrepositionClass = AdjunctCapable; },
        For = "for" { feature PrepositionClass = AdjunctCapable; },
        From = "from" { feature PrepositionClass = PostmodifierBareLocative; },
        In = "in" { feature PrepositionClass = PostmodifierBareLocative; },
        Into = "into",
        Of = "of" { feature PrepositionClass = PostmodifierOnly; },
        On = "on" { feature PrepositionClass = PostmodifierOnly; },
        Onto = "onto",
        To = "to",
        Under = "under",
    }
    vocab LocativeProform { Anywhere = "anywhere", }
    vocab ComparativeQuantifier { Fewer = "fewer", More = "more", }
    vocab FrequencyAdverb { Once = "once", Twice = "twice", }
    vocab ScalarDegree { Equal = "equal", Lesser = "lesser", Greater = "greater", }
    vocab AttributiveAdjective {
        feature ModifierLicense = Unrestricted;
        Additional = "additional",
        Base = "base",
        Declare = "declare",
        FaceDown = "face-down",
        First = "first",
        Main = "main",
        Maximum = "maximum",
        Next = "next",
        Other = "other",
        Postcombat = "postcombat",
        Precombat = "precombat",
        Same = "same",
        Second = "second",
        SixSided = "six-sided",
        Target = "target" { feature ModifierLicense = LocalDeterminer; },
        Untap = "untap",
    }
    vocab ContractedPerfectSubject { Youve = "you've", Theyve = "they've", }
    vocab ContractedCopularSubject {
        Hes = "he's",
        Its = "it's",
        Shes = "she's",
        Theyre = "they're",
        Youre = "you're",
    }
    vocab FloatedQuantifier { All = "all", Both = "both", Each = "each", }
    vocab CostComparisonDirection { More = "more", Less = "less", }
    vocab DistributionReplacement { Instead = "instead", }
    vocab PastPossession { Had = "had", }
    vocab TriggerMarker { When = "when", Whenever = "whenever", }
    vocab SubjectPronoun { He = "he", It = "it", She = "she", They = "they", You = "you", }
    vocab ObjectPronoun { Her = "her", Him = "him", It = "it", Them = "them", You = "you", }
    vocab PossessiveDeterminerPronoun {
        Her = "her",
        His = "his",
        Its = "its",
        Their = "their",
        Your = "your",
    }
    vocab PossessiveAbsolutePronoun {
        Hers = "hers",
        His = "his",
        Theirs = "theirs",
        Yours = "yours",
    }
    vocab ReflexivePronoun {
        Herself = "herself",
        Himself = "himself",
        Itself = "itself",
        Themself = "themself",
        Themselves = "themselves",
        Yourself = "yourself",
        Yourselves = "yourselves",
    }
    vocab Variable { X = "X", Y = "Y", }
    vocab Color {
        Black = "black",
        Blue = "blue",
        Green = "green",
        Monocolored = "monocolored",
        Multicolored = "multicolored",
        Red = "red",
        White = "white",
    }
    vocab Status {
        Attacking = "attacking",
        Blocking = "blocking",
        Tapped = "tapped",
        Untapped = "untapped",
    }
    vocab IndefinitePronoun { Everything = "everything", }
    vocab SingularDemonstrative { This = "this", That = "that", }
    vocab FixedCostSymbol {
        Variable = "X",
        White = "W",
        Blue = "U",
        Black = "B",
        Red = "R",
        Green = "G",
        Colorless = "C",
        Snow = "S",
        HybridWhiteBlue = "W/U",
        HybridWhiteBlack = "W/B",
        HybridBlueBlack = "U/B",
        HybridBlueRed = "U/R",
        HybridBlackRed = "B/R",
        HybridBlackGreen = "B/G",
        HybridRedGreen = "R/G",
        HybridRedWhite = "R/W",
        HybridGreenWhite = "G/W",
        HybridGreenBlue = "G/U",
        ColorlessHybridWhite = "C/W",
        ColorlessHybridBlue = "C/U",
        ColorlessHybridBlack = "C/B",
        ColorlessHybridRed = "C/R",
        ColorlessHybridGreen = "C/G",
        PhyrexianWhite = "W/P",
        PhyrexianBlue = "U/P",
        PhyrexianBlack = "B/P",
        PhyrexianRed = "R/P",
        PhyrexianGreen = "G/P",
        HybridPhyrexianWhiteBlue = "W/U/P",
        HybridPhyrexianWhiteBlack = "W/B/P",
        HybridPhyrexianBlueBlack = "U/B/P",
        HybridPhyrexianBlueRed = "U/R/P",
        HybridPhyrexianBlackRed = "B/R/P",
        HybridPhyrexianBlackGreen = "B/G/P",
        HybridPhyrexianRedGreen = "R/G/P",
        HybridPhyrexianRedWhite = "R/W/P",
        HybridPhyrexianGreenWhite = "G/W/P",
        HybridPhyrexianGreenBlue = "G/U/P",
        Tap = "T",
        Untap = "Q",
        Pawprint = "P",
    }
    vocab ChapterNumeral {
        One = "I",
        Two = "II",
        Three = "III",
        Four = "IV",
        Five = "V",
        Six = "VI",
    }
    vocab MonocoloredHybridColor {
        White = "W",
        Blue = "U",
        Black = "B",
        Red = "R",
        Green = "G",
    }
    vocab EdgePosition { Top = "top", Bottom = "bottom", }
    vocab Supertype {
        Basic = "basic",
        Legendary = "legendary",
        Ongoing = "ongoing",
        Snow = "snow",
        World = "world",
    }

    morphology EnglishVerb {
        feature = Agreement;
        recipe = english_verb;
    }
    morphology EnglishNoun {
        feature = Number;
        recipe = english_noun;
    }
    morphology EnglishParticiple {
        feature = Participle;
        recipe = english_participle;
    }
    lexeme CommonNoun using EnglishNoun {
        feature BareLocativeLicense = QualifiedOnly;
        feature Compoundability = Compoundable;
        feature Countability = Count;
        feature Properness = Common;
        feature Relationality = NonRelational;
        Ability = "ability" {
            Plural = "abilities",
        },
        Attacker = "attacker",
        Battlefield = "battlefield",
        Beginning = "beginning" { feature Relationality = Relational; },
        Blocker = "blocker",
        Card = "card",
        Choice = "choice",
        Coin = "coin",
        Color = "color",
        Control = "control" { feature Countability = Mass; },
        Copy = "copy" {
            Plural = "copies",
        },
        Cost = "cost",
        Counter = "counter",
        Damage = "damage" { feature Countability = Mass; },
        Death = "death",
        Draw = "draw",
        End = "end" {
            feature Compoundability = NonCompoundable;
            feature Relationality = Relational;
        },
        Exile = "exile" { feature BareLocativeLicense = BareAllowed; },
        Graveyard = "graveyard",
        Hand = "hand" { feature BareLocativeLicense = BareAllowed; },
        Library = "library" {
            Plural = "libraries",
        },
        Life = "life" { feature Countability = Mass; },
        Mana = "mana" { feature Countability = Mass; },
        Mode = "mode",
        Name = "name",
        Number = "number",
        Controller = "controller",
        Opponent = "opponent",
        Owner = "owner",
        Permanent = "permanent",
        Phase = "phase" { feature Relationality = Relational; },
        Player = "player",
        Power = "power",
        Rest = "rest",
        Source = "source",
        Size = "size",
        Spell = "spell",
        Stack = "stack",
        Step = "step" { feature Relationality = Relational; },
        Target = "target" {
            feature Compoundability = NonCompoundable;
        },
        Tax = "tax" {
            Plural = "taxes",
        },
        Token = "token",
        Toughness = "toughness" {
            Plural = "toughnesses",
        },
        Turn = "turn" { feature Relationality = Relational; },
        Type = "type",
        Value = "value",
        Way = "way",
        Die = "die" {
            Plural = "dice",
        },
        D20 = "d20",
    }
    codec IntransitiveVerb {
        generate declaration_verb {
            position = Verb;
            tail = [];
            feature = Agreement;
        }
    }
    codec TransitiveVerb {
        generate declaration_verb {
            position = Verb;
            tail = [ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec NumerativeVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Amount];
            feature = Agreement;
        }
    }
    codec ObjectForObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                location: ObjectNounPhrase,
                "for",
                sought: ObjectNounPhrase,
            ];
            feature = Agreement;
        }
    }
    codec ToObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "to",
                complement: ObjectNounPhrase,
            ];
            feature = Agreement;
        }
    }
    codec ForObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["for", object: ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec ObjectAmountVerb {
        generate declaration_verb {
            position = Verb;
            tail = [object: ObjectNounPhrase, amount: Amount];
            feature = Agreement;
        }
    }
    codec ObjectPredicativeComplementVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                complement: PredicativeComplement,
            ];
            feature = Agreement;
        }
    }
    codec WithObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["with", object: ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec AmongObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["among", recipient: Object];
            feature = Agreement;
        }
    }
    codec ObjectWithObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "with",
                complement: ObjectNounPhrase,
            ];
            feature = Agreement;
        }
    }
    codec ObjectIntoObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "into",
                complement: ObjectNounPhrase,
            ];
            feature = Agreement;
        }
    }
    codec DistributedMeasureVerb { generate declaration_verb { position = Verb; tail = [Amount, MassNoun, DistributionPhrase, DistributionReplacement?]; feature = Agreement; } }
    codec ObjectEqualityVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, ScalarEquality]; feature = Agreement; } }
    codec ObjectEqualityToVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, ScalarEquality, "to", recipient: Object]; feature = Agreement; } }
    codec ObjectToEqualityVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, "to", recipient: Object, ScalarEquality]; feature = Agreement; } }
    codec ManaPhraseVerb { generate declaration_verb { position = Verb; tail = [ManaPhrase]; feature = Agreement; } }
    codec ObjectFromVerb {
        generate declaration_verb {
            position = Verb;
            tail = [object: ObjectNounPhrase, "from", source: FrameComplement];
            feature = Agreement;
        }
    }
    codec ObjectFromOntoResultControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                Object,
                SourcePhrase?,
                "onto",
                destination: FrameComplement,
                PredicativeComplement?,
                ControlPhrase?,
            ];
            feature = Agreement;
        }
    }
    codec ObjectFromOnVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, SourcePhrase?, "on", destination: FrameComplement];
            feature = Agreement;
        }
    }
    codec ObjectToVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, "to", destination: FrameComplement];
            feature = Agreement;
        }
    }
    codec ObjectFromToResultControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                Object,
                SourcePhrase?,
                "to",
                destination: FrameComplement,
                PredicativeComplement?,
                ControlPhrase?,
            ];
            feature = Agreement;
        }
    }
    codec PredicativeComplementVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [PredicativeComplement];
            feature = Agreement;
        }
    }
    codec AuxiliaryVerb {
        generate declaration_verb {
            class = Auxiliary;
            position = Verb;
            tail = [];
            feature = Agreement;
        }
    }
    codec ObjectInfinitiveVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [Object, "to", VerbPhrase];
            feature = Agreement;
        }
    }
    codec ChooseInfinitiveVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [InfinitiveComplement];
            feature = Agreement;
        }
    }
    codec OrderedVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [Object, SourcePhrase?, "on", destination: FrameComplement, "in", ObjectOrder, "order"];
            feature = Agreement;
        }
    }
    codec HaveKeywordAbilityVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [KeywordAbility];
            feature = Agreement;
        }
    }
    codec CostComparisonVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [
                ManaAmount,
                CostComparisonDirection,
                ControlledCostAction,
                ForEachCostBasis?,
            ];
            feature = Agreement;
        }
    }
    codec ProVerbHead {
        generate declaration_verb {
            class = ProVerb;
            position = Verb;
            tail = [];
            feature = Agreement;
        }
    }
    codec EnterWithCountersVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["with", object: ObjectNounPhrase, "on", recipient: FrameComplement];
            feature = Agreement;
        }
    }
    codec EnterLocationVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, PredicativeComplement?, ControlPhrase?];
            feature = Agreement;
        }
    }
    codec EnterControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["under", control: Object];
            feature = Agreement;
        }
    }
    codec LookAtVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["at", Object];
            feature = Agreement;
        }
    }
    codec QuotedAbilityVerb {
        generate declaration_verb {
            position = Verb;
            tail = [QuotedAbility];
            feature = Agreement;
        }
    }
    codec HaveObjectControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, VerbPhrase];
            feature = Agreement;
        }
    }
    codec GetPowerToughnessVerb {
        generate declaration_verb {
            position = Verb;
            tail = [PowerToughnessAdjustment, DurationPhrase?];
            feature = Agreement;
        }
    }
    codec MovementParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = [moved: ObjectNounPhrase, "into", destination: ObjectNounPhrase];
            feature = Participle;
        }
    }
    codec OrientationParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = ["face", "up"];
            feature = Participle;
        }
    }
    codec DeclaredTransitiveParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = [ObjectNounPhrase];
            feature = Participle;
        }
    }
    codec DeclaredToObjectParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "to",
                complement: ObjectNounPhrase,
            ];
            feature = Participle;
        }
    }
    codec ObjectOnParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "on",
                complement: ObjectNounPhrase,
            ];
            feature = Participle;
        }
    }

    codec Noun {
        generate declaration_noun {
            closed = CommonNoun;
            position = Noun;
            kinds = [Type, Subtype, TurnPart];
            feature = Number;
        }
    }
    codec KeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = Any;
        }
    }
    codec BareKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
        }
    }
    codec DeclaredKeywordParticiple {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = Any;
            feature = Participle;
        }
    }
    codec LevelBlockLabel {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = Any;
            feature = BlockLabel;
        }
    }
    codec CostedKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Cost];
        }
    }
    codec AmountKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Amount];
        }
    }
    codec AmountCostKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Amount, Cost];
        }
    }
    codec QualityKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Quality];
        }
    }
    codec QualityCostKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Quality, Cost];
        }
    }
    codec SubjectKeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
            params = [Subject];
        }
    }
    codec AbilityWordTerm {
        generate declaration_term {
            position = FixedTerm;
            kinds = [AbilityWord];
        }
    }
    codec DeclaredCounterKind {
        generate declaration_term {
            position = FixedTerm;
            kinds = [CounterKind];
        }
    }
    codec DesignationTerm {
        generate declaration_term {
            position = FixedTerm;
            kinds = [Designation];
        }
    }
    codec DeterminativeHead {
        generate declaration_determinative {
            closed = [
                IndefiniteArticle {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [
                        { surface = "a"; phrase_number = Singular; following_onset = Consonant; },
                        { surface = "an"; phrase_number = Singular; following_onset = Vowel; },
                    ];
                },
                DefiniteArticle {
                    number_license = Both;
                    nominal_license = AnyNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "the"; }];
                },
                ProximalDemonstrative {
                    number_license = Both;
                    nominal_license = AnyNominal;
                    fused_head_license = FusedHead;
                    realizations = [
                        { surface = "this"; phrase_number = Singular; },
                        { surface = "these"; phrase_number = Plural; },
                    ];
                },
                DistalDemonstrative {
                    number_license = Both;
                    nominal_license = AnyNominal;
                    fused_head_license = FusedHead;
                    realizations = [
                        { surface = "that"; phrase_number = Singular; },
                        { surface = "those"; phrase_number = Plural; },
                    ];
                },
                Another {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "another"; }];
                },
                Each {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "each"; }];
                },
                All {
                    number_license = Both;
                    nominal_license = MassOrPluralCount;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "all"; }];
                },
                Both {
                    number_license = PluralOnly;
                    nominal_license = CountNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "both"; }];
                },
                No {
                    number_license = Both;
                    nominal_license = AnyNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "no"; }];
                },
                Any {
                    number_license = Both;
                    nominal_license = AnyNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "any"; }];
                },
                AnyOne {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "any one"; }];
                },
                Target {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "target"; }];
                },
            ];
        }
    }
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Full;
        }
    }
    identity CardName {
        generate catalog_identity {
            provider = CardNames;
        }
    }
    codec CardinalNumber {
        generate english_cardinal {
            magnitude = u32;
        }
    }
    codec ScalarNumber {
        generate unsigned_decimal {
            magnitude = u32;
        }
    }
    codec LoyaltyMagnitude {
        generate unsigned_decimal {
            magnitude = NonZeroU32;
        }
    }
    abstract sum ActivationCostComponent {
        SymbolRun,
        Loyalty,
        Clause: CostClause,
    }
    abstract sum CounterfactualClause {
        Finite: FiniteClause,
        IrrealisCopular: IrrealisCopularClause,
    }
    abstract sum BareLocative {
        Proform: BareLocativeProform,
        Noun: BareLocativeNoun,
    }
    abstract sum PossessiveComplement {
        Object: Object,
        Keyword: KeywordPossessiveComplement,
        Quoted: QuotedAbility,
    }
    abstract sum BaseVerbFrame {
        IntransitiveFrame,
        TransitiveFrame,
        NumerativeFrame,
        ObjectAmountFrame,
        WithObjectFrame,
        ObjectWithObjectFrame,
        ObjectForObjectFrame,
        ObjectIntoObjectFrame,
    }
    abstract sum AdditionalCostBody {
        Predicate: AdditionalCostPredicateBody,
        Finite: AdditionalCostFiniteBody,
    }
    abstract sum ManaPhrase {
        Amount: ManaAmount,
        Coordination: ManaCoordination,
    }
    abstract sum ManaCoordination {
        And: AndManaCoordination,
        Or: OrManaCoordination,
        AndOr: AndOrManaCoordination,
    }
    abstract sum CastingRestriction {
        Conditional: OnlyIfRestriction,
        Timing: OnlyDuringRestriction,
        TemporalClause: OnlyTemporalClauseRestriction,
    }
    abstract sum DurationPhrase {
        Fixed: FixedDurationPhrase,
        Until: UntilDurationPhrase,
    }
    abstract sum ScalarDegreePhrase {
        Single: SingleScalarDegreePhrase,
        Or: OrScalarDegreePhrase,
    }
    abstract sum TemporalEndpoint { Reference: NounPhrase, }
    // A preposition takes an ordinary object, a bare locative, an edge
    // locative, or another prepositional phrase ("from among ...").
    abstract sum PrepositionalComplement {
        Object: Object,
        Edge: EdgeOfPhrase,
        Phrase: PrepositionalPhrase,
    }
    // A frame names its own preposition, so its complement also admits the
    // determiner-less locative that the free phrase licenses by class.
    abstract sum FrameComplement {
        Object: Object,
        Locative: BareLocative,
        Edge: EdgeOfPhrase,
        Phrase: PrepositionalPhrase,
    }
    abstract sum ObjectGapRelativeClause {
        Positive: PositiveObjectGapRelativeClause,
        Auxiliary: AuxiliaryObjectGapRelativeClause,
        ContractedPerfect: ContractedPerfectObjectGapRelativeClause,
        BareNegative: BareNegativeObjectGapRelativeClause,
        ThirdPersonNegative: ThirdPersonNegativeObjectGapRelativeClause,
    }
    abstract sum SubjectGapRelativeClause {
        Finite: FiniteSubjectGapRelativeClause,
        Modal: ModalSubjectGapRelativeClause,
        Copular: CopularSubjectGapRelativeClause,
        ModalPassive: ModalPassiveSubjectGapRelativeClause,
    }
    abstract sum CoordinatedPredicate {
        Atomic: VerbPhrase,
        BareCopular: BareCopularPredicate,
        FiniteCopular: FiniteCopularPredicate,
        BarePassive: BarePassivePredicate,
        FinitePassive: FinitePassivePredicate,
        Auxiliary: AuxiliaryPredicate,
        PrepositionalAdjunct: PrepositionalAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Purpose: PurposePredicate,
        Duration: DurationPredicate,
        StateDuration: StateDurationPredicate,
        Instead: InsteadPredicate,
        Manner: MannerPredicate,
        Frequency: FrequencyPredicate,
        Alternative: AlternativePredicate,
        WithoutGerundObject: WithoutGerundObjectPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum BareCoordinatedPredicate {
        Atomic: VerbPhrase,
        BareCopular: BareCopularPredicate,
        BarePassive: BarePassivePredicate,
        PrepositionalAdjunct: PrepositionalAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Purpose: PurposePredicate,
        Duration: DurationPredicate,
        StateDuration: StateDurationPredicate,
        Instead: InsteadPredicate,
        Manner: MannerPredicate,
        Frequency: FrequencyPredicate,
        Alternative: AlternativePredicate,
        WithoutGerundObject: WithoutGerundObjectPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum BarePredicate {
        Atomic: VerbPhrase,
        Coordination: BarePredicateCoordination,
        ThenSequence: BareThenPredicateSequence,
        BareCopular: BareCopularPredicate,
        BarePassive: BarePassivePredicate,
        PrepositionalAdjunct: PrepositionalAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Purpose: PurposePredicate,
        Duration: DurationPredicate,
        StateDuration: StateDurationPredicate,
        Instead: InsteadPredicate,
        Manner: MannerPredicate,
        Frequency: FrequencyPredicate,
        Alternative: AlternativePredicate,
        WithoutGerundObject: WithoutGerundObjectPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum Predicate {
        Atomic: VerbPhrase,
        Coordination: PredicateCoordination,
        ThenSequence: ThenPredicateSequence,
        BareCopular: BareCopularPredicate,
        FiniteCopular: FiniteCopularPredicate,
        BarePassive: BarePassivePredicate,
        FinitePassive: FinitePassivePredicate,
        Auxiliary: AuxiliaryPredicate,
        PrepositionalAdjunct: PrepositionalAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Purpose: PurposePredicate,
        Duration: DurationPredicate,
        StateDuration: StateDurationPredicate,
        Instead: InsteadPredicate,
        Manner: MannerPredicate,
        Frequency: FrequencyPredicate,
        Alternative: AlternativePredicate,
        WithoutGerundObject: WithoutGerundObjectPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum Clause {
        Finite: FiniteClause,
        Coordination: ClauseCoordination,
        PostposedWhile: PostposedWhileClause,
        PostposedForAsLongAs: PostposedForAsLongAsClause,
    }
    abstract sum CoordinatedClause {
        Finite: FiniteClause,
    }
    abstract sum PredicativeComplement {
        Adjective: PredicativeAdjectiveComplement,
        Color: PredicativeColorComplement,
        Designation: PredicativeDesignationComplement,
        Nominal: PredicativeNominalComplement,
        Status: PredicativeStatus,
        Ability: PredicativeAbilityComplement,
        Orientation: PredicativeFaceOrientation,
        PowerToughness: PredicativePowerToughnessComplement,
        Scalar: PredicativeScalarComplement,
    }
    abstract sum PredicativeStatus {
        Plain: PredicativeStatusComplement,
        ParticipialAdjective: PredicativeParticipialAdjectiveComplement,
        DeclaredParticiple: PredicativeDeclaredParticipleComplement,
        By: ParticipialByComplement,
        ExceptBy: ParticipialExceptByComplement,
    }
    abstract sum PassivePredicate {
        Object: DeclaredObjectPassivePredicate,
        Movement: PassiveMovementPredicate,
        Orientation: PassiveOrientationPredicate,
        DeclaredTransitiveFrom: DeclaredTransitivePassiveFromPredicate,
        DeclaredToObject: DeclaredToObjectPassivePredicate,
    }
    abstract sum StateDurationBase {
        BarePassive: BarePassivePredicate,
    }
    abstract sum DistributionPhrase {
        AsYouChoose: ChosenDistributionPhrase,
        Evenly: EvenDistributionPhrase,
    }
    abstract sum DistributionRecipient {
        Object: ObjectDistributionRecipient,
    }
    abstract sum ClauseAttachment {
        StartingWith: StartingWithAttachment,
        PreposedIf,
        PreposedIfPredicate,
        PostposedIf,
        PostposedIfPredicate,
        PostposedUnless,
        PostposedUnlessPredicate,
        PreposedAs,
        PreposedAsLongAs,
        PreposedAsLongAsPredicate,
        PostposedAsLongAs,
        PostposedAsLongAsPredicate,
        PreposedPrepositionalAdjunct,
        PreposedPrepositionalAdjunctPredicate,
        PreposedWhile,
        PreposedWhilePredicate,
        PreposedUntil,
        PreposedUntilPredicate,
        PreposedDuration,
        PreposedDurationPredicate,
        ThenSequence,
        AdditionalCost,
    }
    abstract sum ConditionClause { FiniteCondition, }
    construction finite_condition: FiniteCondition {
        element FiniteConditionValue { clause: Clause, }
        form finite_condition = "if" clause ",";
    }
    // A class level bar's activated ability sets the Class's level; a level is
    // a declared designation carrying a number [CR#716.2a,716.2b].
    construction class_level_body: AbilityBody {
        element ClassLevelBody {
            designation: lex DesignationTerm,
            value: lex ScalarNumber,
        }
        form class_level_body = lex(designation) lex(value);
    }
    construction plain: Ability {
        element Plain {
            body: AbilityBody,
        }
        form plain = body;
    }
    construction sentences: AbilityBody {
        element Sentences {
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form sentences = sentences;
    }
    construction then_sentences: AbilityBody {
        element ThenSentenceSequence {
            members: seq Sentence separated by continuation(" Then ") terminated by ".",
        }
        require len(members) >= 2;
        form then_sentences = members;
    }
    construction modal_mode: ModalMode {
        element ModalModeValue {
            marker: ModeMarker,
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form modal_mode = marker sentences;
    }
    abstract sum FrequencyReference {
        Plain: PlainFrequency,
        Comparative: ComparativeFrequency,
    }
    abstract sum ModeMarker {
        Bullet: BulletMarker,
        Weighted: WeightedMarker,
    }
    construction bullet_marker: ModeMarker {
        element BulletMarker {}
        form bullet_marker = sentence_initial("• ");
    }
    // Spree's plus sign and the pawprint symbols are the same weighted mode
    // marker: the plus sign carries no rules meaning [CR#702.172a,702.172b]
    // and pawprints weight a mode against the head's allowance [CR#700.2i].
    construction additional_cost_mark: AdditionalCostMark {
        element AdditionalCostMarkValue {}
        form additional_cost_mark = "+";
    }
    construction weighted_marker: ModeMarker {
        element WeightedMarker {
            additional: opt AdditionalCostMark,
            cost: ActivationCostComponent,
        }
        require cost is SymbolRun;
        form weighted_marker = additional cost sentence_initial(" — ");
    }
    abstract sum ModalHead {
        Dash: DashHead,
        Sentence: SentenceHead,
        Keyword: KeywordLine,
    }
    construction dash_head: DashHead {
        element DashHeadValue { clause: Sentence, }
        form dash_head = clause sentence_initial(" —");
    }
    // A sentence-headed modal states its allowance in ordinary sentences,
    // including the licence to repeat a mode [CR#700.2d].
    construction sentence_head: SentenceHead {
        element SentenceHeadValue {
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form sentence_head = sentences;
    }
    construction plain_modal: AbilityBody {
        element PlainModal {
            head: ModalHead,
            modes: seq ModalMode separated by sentence_initial("\n"),
        }
        require len(modes) >= 2;
        form plain_modal = head sentence_initial("\n") modes;
    }
    construction finite: TriggerPrefix {
        element Finite {
            marker: lex TriggerMarker,
            clause: Clause,
        }
        form finite = lex(marker) clause;
    }
    construction temporal: TriggerPrefix {
        element Temporal { phrase: PrepositionalPhrase, }
        form temporal = phrase;
    }
    construction triggered: Ability {
        element Triggered {
            trigger: TriggerPrefix,
            intervening_if: opt ConditionClause,
            body: AbilityBody,
        }
        form triggered = trigger "," intervening_if body;
    }
    construction generic_cost_symbol: CostSymbol {
        element GenericCostSymbol { magnitude: lex ScalarNumber, }
        form generic_cost_symbol = lex(magnitude);
    }
    construction fixed_cost_symbol: CostSymbol {
        element FixedSymbol { symbol: lex FixedCostSymbol, }
        form fixed_cost_symbol = lex(symbol);
    }
    construction monocolored_hybrid_symbol: CostSymbol {
        element MonocoloredHybridSymbol { color: lex MonocoloredHybridColor, }
        form monocolored_hybrid_symbol = prefix("2/", lex(color));
    }
    construction symbol_run: ActivationCostComponent {
        element SymbolRun {
            symbols: seq CostSymbol separated by "}{",
        }
        require len(symbols) >= 1;
        form symbol_run = circumfix("{", symbols, "}");
    }
    construction positive_loyalty: LoyaltyValue {
        element PositiveLoyalty { magnitude: lex LoyaltyMagnitude, }
        form positive_loyalty = prefix("+", lex(magnitude));
    }
    construction zero_loyalty: LoyaltyValue {
        element ZeroLoyalty {}
        form zero_loyalty = "0";
    }
    construction negative_loyalty: LoyaltyValue {
        element NegativeLoyalty { magnitude: lex LoyaltyMagnitude, }
        form negative_loyalty = prefix("−", lex(magnitude));
    }
    construction loyalty: ActivationCostComponent {
        element Loyalty { value: LoyaltyValue, }
        form loyalty = circumfix("[", value, "]");
    }
    construction cost_clause: ActivationCostComponent {
        element CostClause { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form cost_clause = predicate;
    }
    construction activated: Ability {
        element Activated {
            costs: seq ActivationCostComponent separated by position {
                pair = sentence_initial(", ");
                first = sentence_initial(", ");
                middle = sentence_initial(", ");
                last = sentence_initial(", ");
            },
            body: AbilityBody,
        }
        require len(costs) >= 1;
        form activated = costs sentence_initial(": ") body;
    }
    construction imperative: Sentence {
        element Imperative { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form imperative = predicate;
    }
    construction declarative: Sentence {
        element Declarative { clause: Clause, }
        form declarative = clause;
    }
    construction attached: Sentence {
        element Attached { attachment: ClauseAttachment, }
        form attached = attachment;
    }
    construction preposed_if: ClauseAttachment {
        element PreposedIf { condition: Clause, body: Clause, }
        form preposed_if = "if" condition "," body;
    }
    construction starting_with: ClauseAttachment {
        element StartingWithAttachment { starter: Object, body: Clause, }
        form starting_with = "starting" "with" starter "," body;
    }
    construction preposed_if_predicate: ClauseAttachment {
        element PreposedIfPredicate { condition: Clause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_if_predicate = "if" condition "," body;
    }
    construction postposed_if: ClauseAttachment {
        element PostposedIf { body: Clause, condition: FiniteClause, }
        form postposed_if = body "if" condition;
    }
    construction postposed_if_predicate: ClauseAttachment {
        element PostposedIfPredicate { body: Predicate, condition: FiniteClause, }
        derive body.agreement = Values::Bare;
        form postposed_if_predicate = body "if" condition;
    }
    construction postposed_unless: ClauseAttachment {
        element PostposedUnless { body: Clause, condition: FiniteClause, }
        form postposed_unless = body "unless" condition;
    }
    construction postposed_unless_predicate: ClauseAttachment {
        element PostposedUnlessPredicate { body: Predicate, condition: FiniteClause, }
        derive body.agreement = Values::Bare;
        form postposed_unless_predicate = body "unless" condition;
    }
    construction preposed_as: ClauseAttachment {
        element PreposedAs { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_as = "as" condition "," body;
    }
    construction preposed_as_long_as: ClauseAttachment {
        element PreposedAsLongAs { condition: Clause, body: Clause, }
        form preposed_as_long_as = "as" "long" "as" condition "," body;
    }
    construction preposed_as_long_as_predicate: ClauseAttachment {
        element PreposedAsLongAsPredicate { condition: Clause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_as_long_as_predicate = "as" "long" "as" condition "," body;
    }
    construction postposed_as_long_as: ClauseAttachment {
        element PostposedAsLongAs { body: Clause, condition: FiniteClause, }
        form postposed_as_long_as = body "as" "long" "as" condition;
    }
    construction postposed_as_long_as_predicate: ClauseAttachment {
        element PostposedAsLongAsPredicate { body: Predicate, condition: FiniteClause, }
        derive body.agreement = Values::Bare;
        form postposed_as_long_as_predicate = body "as" "long" "as" condition;
    }
    construction postposed_while_clause: PostposedWhileClause {
        element PostposedWhileClauseValue { body: Clause, condition: FiniteClause, }
        form postposed_while_clause = body "while" condition;
    }
    construction postposed_for_as_long_as_clause: PostposedForAsLongAsClause {
        element PostposedForAsLongAsClauseValue {
            body: Clause,
            condition: FiniteClause,
        }
        form postposed_for_as_long_as_clause = body "for" "as" "long" "as" condition;
    }
    construction preposed_prepositional_adjunct: ClauseAttachment {
        element PreposedPrepositionalAdjunct {
            adjunct: PrepositionalPhrase,
            body: Clause,
        }
        require adjunct.preposition_class is AdjunctCapable;
        form preposed_prepositional_adjunct = adjunct "," body;
    }
    construction preposed_prepositional_adjunct_predicate: ClauseAttachment {
        element PreposedPrepositionalAdjunctPredicate {
            adjunct: PrepositionalPhrase,
            body: Predicate,
        }
        require adjunct.preposition_class is AdjunctCapable;
        derive body.agreement = Values::Bare;
        form preposed_prepositional_adjunct_predicate = adjunct "," body;
    }
    construction preposed_while: ClauseAttachment {
        element PreposedWhile { condition: FiniteClause, body: Clause, }
        form preposed_while = "while" condition "," body;
    }
    construction preposed_while_predicate: ClauseAttachment {
        element PreposedWhilePredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_while_predicate = "while" condition "," body;
    }
    construction preposed_until: ClauseAttachment {
        element PreposedUntil { condition: FiniteClause, body: Clause, }
        form preposed_until = "until" condition "," body;
    }
    construction preposed_until_predicate: ClauseAttachment {
        element PreposedUntilPredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_until_predicate = "until" condition "," body;
    }
    construction preposed_duration: ClauseAttachment {
        element PreposedDuration { duration: DurationPhrase, body: Clause, }
        form preposed_duration = duration "," body;
    }
    construction preposed_duration_predicate: ClauseAttachment {
        element PreposedDurationPredicate {
            duration: DurationPhrase,
            body: Predicate,
        }
        derive body.agreement = Values::Bare;
        form preposed_duration_predicate = duration "," body;
    }
    construction fixed_duration_phrase: FixedDurationPhrase {
        element FixedDurationPhraseValue { endpoint: TemporalEndpoint, }
        form fixed_duration_phrase = endpoint;
    }
    construction until_duration_phrase: UntilDurationPhrase {
        element UntilDurationPhraseValue { endpoint: TemporalEndpoint, }
        form until_duration_phrase = "until" endpoint;
    }
    construction then_sequence: ClauseAttachment {
        element ThenSequence {
            members: seq Clause separated by position {
                pair = ", then ";
                first = ", ";
                middle = ", ";
                last = ", then ";
            },
        }
        require len(members) >= 2;
        form then_sequence = members;
    }
    construction then_predicate_sequence: ThenPredicateSequence {
        element ThenPredicateSequenceValue {
            members: seq CoordinatedPredicate separated by position {
                pair = ", then ";
                first = ", ";
                middle = ", ";
                last = ", then ";
            },
        }
        require len(members) >= 2;
        derive members.agreement = Values::Bare;
        derive agreement = members.agreement;
        form then_predicate_sequence = members;
    }
    construction bare_then_predicate_sequence: BareThenPredicateSequence {
        element BareThenPredicateSequenceValue {
            members: seq BareCoordinatedPredicate separated by position {
                pair = ", then ";
                first = ", ";
                middle = ", ";
                last = ", then ";
            },
        }
        require len(members) >= 2;
        derive members.agreement = Values::Bare;
        derive agreement = members.agreement;
        form bare_then_predicate_sequence = members;
    }
    construction additional_cost: ClauseAttachment {
        element AdditionalCost {
            cost: SingularHead,
            action: InfinitiveComplement,
            body: AdditionalCostBody,
        }
        form additional_cost = "as" "an" "additional" cost action "," body;
    }
    construction additional_cost_predicate_body: AdditionalCostBody {
        element AdditionalCostPredicateBody { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form additional_cost_predicate_body = predicate;
    }
    construction additional_cost_finite_body: AdditionalCostBody {
        element AdditionalCostFiniteBody { clause: FiniteClause, }
        form additional_cost_finite_body = clause;
    }
    construction with_where: Sentence {
        element WithWhere { body: Sentence, clause: WhereClauseCategory, }
        form with_where = body "," clause;
    }
    construction and_predicate_coordination: PredicateCoordination {
        element AndPredicateCoordination {
            members: seq CoordinatedPredicate separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form and_predicate_coordination = members;
    }
    construction or_predicate_coordination: PredicateCoordination {
        element OrPredicateCoordination {
            members: seq CoordinatedPredicate separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form or_predicate_coordination = members;
    }
    construction and_or_predicate_coordination: PredicateCoordination {
        element AndOrPredicateCoordination {
            members: seq CoordinatedPredicate separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form and_or_predicate_coordination = members;
    }
    construction bare_and_predicate_coordination: BarePredicateCoordination {
        element BareAndPredicateCoordination {
            members: seq BareCoordinatedPredicate separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form bare_and_predicate_coordination = members;
    }
    construction bare_or_predicate_coordination: BarePredicateCoordination {
        element BareOrPredicateCoordination {
            members: seq BareCoordinatedPredicate separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form bare_or_predicate_coordination = members;
    }
    construction bare_and_or_predicate_coordination: BarePredicateCoordination {
        element BareAndOrPredicateCoordination {
            members: seq BareCoordinatedPredicate separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form bare_and_or_predicate_coordination = members;
    }
    construction predicative_adjective: PredicativeAdjectiveComplement {
        element PredicativeAdjectiveValue { adjective: lex PredicativeAdjective, }
        form predicative_adjective = lex(adjective);
    }
    construction predicative_color: PredicativeColorComplement {
        element PredicativeColorValue { color: lex Color, }
        form predicative_color = lex(color);
    }
    construction predicative_designation: PredicativeDesignationComplement {
        element PredicativeDesignationValue { designation: lex DesignationTerm, }
        form predicative_designation = lex(designation);
    }
    construction predicative_nominal: PredicativeNominalComplement {
        element PredicativeNominalValue { value: NounPhrase, }
        form predicative_nominal = value;
    }
    construction predicative_status: PredicativeStatusComplement {
        element PredicativeStatusValue { status: lex Status, }
        form predicative_status = lex(status);
    }
    construction declared_participial_adjective: ParticipialAdjective {
        element DeclaredParticipialAdjective { head: lex DeclaredKeywordParticiple, }
        derive onset = head.onset;
        form declared_participial_adjective = lex(head);
    }
    construction predicative_participial_adjective: PredicativeParticipialAdjectiveComplement {
        element PredicativeParticipialAdjectiveValue { adjective: ParticipialAdjective, }
        form predicative_participial_adjective = adjective;
    }
    construction predicative_declared_participle: PredicativeDeclaredParticipleComplement {
        element PredicativeDeclaredParticipleValue {
            head: lex DeclaredTransitiveParticipleHead,
        }
        form predicative_declared_participle = verb(head);
    }
    construction participial_by_complement: ParticipialByComplement {
        element ParticipialByComplementValue {
            head: lex DeclaredTransitiveParticipleHead,
            agent: Object,
        }
        form participial_by_complement = verb(head) "by" agent;
    }
    construction participial_except_by_complement: ParticipialExceptByComplement {
        element ParticipialExceptByComplementValue {
            head: lex DeclaredTransitiveParticipleHead,
            agent: Object,
        }
        form participial_except_by_complement = verb(head) "except" "by" agent;
    }
    construction predicative_ability: PredicativeAbilityComplement {
        element PredicativeAbilityValue { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form predicative_ability = "able" "to" predicate;
    }
    construction predicative_power_toughness: PredicativePowerToughnessComplement {
        element PredicativePowerToughnessValue {
            magnitudes: seq Amount separated by "/",
        }
        require len(magnitudes) = 2;
        form predicative_power_toughness = magnitudes;
    }
    construction predicative_scalar: PredicativeScalarComplement {
        element PredicativeScalarValue { value: CardinalQuantity, }
        form predicative_scalar = value;
    }
    construction predicative_face_orientation: PredicativeFaceOrientation {
        element PredicativeFaceOrientationValue { orientation: lex FaceOrientation, }
        form predicative_face_orientation = lex(orientation);
    }
    construction bare_copular_predicate: BareCopularPredicate {
        element BareCopularPredicateValue {
            copula: lex BareCopula,
            complement: PredicativeComplement,
        }
        derive agreement = Values::Bare;
        form bare_copular_predicate = lex(copula) complement;
    }
    construction declared_object_passive_predicate: DeclaredObjectPassivePredicate {
        element DeclaredObjectPassivePredicateValue {
            head: lex DeclaredTransitiveParticipleHead,
            object: Object,
        }
        form declared_object_passive_predicate = verb(head) object;
    }
    construction passive_movement_predicate: PassiveMovementPredicate {
        element PassiveMovementPredicateValue {
            head: lex MovementParticipleHead,
            destination: FrameComplement,
            source: FrameComplement,
        }
        form passive_movement_predicate =
            verb(head) "into" destination "from" source;
    }
    construction passive_orientation_predicate: PassiveOrientationPredicate {
        element PassiveOrientationPredicateValue {
            head: lex OrientationParticipleHead,
            orientation: lex FaceOrientation,
        }
        form passive_orientation_predicate = verb(head) lex(orientation);
    }
    construction declared_transitive_passive_from_predicate: DeclaredTransitivePassiveFromPredicate {
        element DeclaredTransitivePassiveFromPredicateValue {
            head: lex DeclaredTransitiveParticipleHead,
            source: FrameComplement,
        }
        form declared_transitive_passive_from_predicate = verb(head) "from" source;
    }
    construction declared_to_object_passive_predicate: DeclaredToObjectPassivePredicate {
        element DeclaredToObjectPassivePredicateValue {
            head: lex DeclaredToObjectParticipleHead,
            complement: FrameComplement,
        }
        form declared_to_object_passive_predicate = verb(head) "to" complement;
    }
    construction bare_passive_predicate: BarePassivePredicate {
        element BarePassivePredicateValue {
            copula: lex BareCopula,
            predicate: PassivePredicate,
        }
        derive agreement = Values::Bare;
        form bare_passive_predicate = lex(copula) predicate;
    }
    construction inventory_auxiliary: AuxiliaryHead {
        element InventoryAuxiliary { head: lex AuxiliaryVerb, }
        derive agreement = head.agreement;
        form inventory_auxiliary = verb(head);
    }
    construction state_duration_predicate: StateDurationPredicate {
        element StateDurationPredicateValue {
            predicate: StateDurationBase,
            duration: DurationPhrase,
        }
        derive agreement = Values::Bare;
        form state_duration_predicate = predicate duration;
    }
    construction object_infinitive_predicate: ObjectInfinitivePredicate {
        element ObjectInfinitivePredicateValue {
            head: lex ObjectInfinitiveVerb,
            object: Object,
            complement: VerbPhrase,
        }
        derive agreement = head.agreement;
        derive complement.agreement = Values::Bare;
        form object_infinitive_predicate = verb(head) object "to" complement;
    }
    construction infinitive_complement: InfinitiveComplement {
        element InfinitiveComplementValue {
            negator: opt lex PredicateNegator,
            predicate: VerbPhrase,
        }
        derive predicate.agreement = Values::Bare;
        form infinitive_complement = lex(negator) "to" predicate;
    }
    construction choose_infinitive_predicate: VerbPhrase {
        element ChooseInfinitivePredicate {
            head: lex ChooseInfinitiveVerb,
            complement: InfinitiveComplement,
        }
        derive agreement = head.agreement;
        form choose_infinitive_predicate = verb(head) complement;
    }
    construction requirement_predicate: RequirementPredicate {
        element RequirementPredicateValue {
            head: lex IntransitiveVerb,
            frequency: NounPhrase,
        }
        derive agreement = head.agreement;
        form requirement_predicate = verb(head) frequency "if" "able";
    }
    construction transitive_requirement_predicate: TransitiveRequirementPredicate {
        element TransitiveRequirementPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            duration: opt DurationPhrase,
        }
        derive agreement = head.agreement;
        form transitive_requirement_predicate =
            verb(head) object duration "if" "able";
    }
    construction as_though_predicate: AsThoughPredicate {
        element AsThoughPredicateValue {
            predicate: BaseVerbFrame,
            condition: CounterfactualClause,
        }
        derive agreement = predicate.agreement;
        form as_though_predicate = predicate "as" "though" condition;
    }
    construction ordered_predicate: OrderedPredicate {
        element OrderedPredicateValue {
            head: lex OrderedVerb,
            object: Object,
            source: opt SourcePhrase,
            destination: FrameComplement,
            order: lex ObjectOrder,
        }
        derive agreement = head.agreement;
        form ordered_predicate =
            verb(head) object source "on" destination "in" lex(order) "order";
    }
    construction irrealis_copular_clause: IrrealisCopularClause {
        element IrrealisCopularClauseValue {
            subject: Subject,
            copula: lex FiniteCopula,
            complement: PredicativeComplement,
        }
        require copula is Were;
        form irrealis_copular_clause = subject lex(copula) complement;
    }
    construction past_possession_clause: FiniteClause {
        element PastPossessionClause {
            subject: Subject,
            head: lex PastPossession,
            complement: PossessiveComplement,
        }
        form past_possession_clause = subject lex(head) complement;
    }
    construction keyword_possessive_complement: KeywordPossessiveComplement {
        element KeywordPossessiveComplementValue { value: lex KeywordAbility, }
        form keyword_possessive_complement = lex(value);
    }
    construction purpose_predicate: PurposePredicate {
        element PurposePredicateValue {
            predicate: BaseVerbFrame,
            purpose: BaseVerbFrame,
        }
        derive agreement = predicate.agreement;
        derive purpose.agreement = Values::Bare;
        form purpose_predicate = predicate "to" purpose;
    }
    construction duration_predicate: DurationPredicate {
        element DurationPredicateValue {
            predicate: BaseVerbFrame,
            duration: DurationPhrase,
        }
        derive agreement = predicate.agreement;
        form duration_predicate = predicate duration;
    }
    construction prepositional_adjunct_predicate: PrepositionalAdjunctPredicate {
        element PrepositionalAdjunctPredicateValue {
            predicate: VerbPhrase,
            adjunct: PrepositionalPhrase,
        }
        require adjunct.preposition_class is AdjunctCapable;
        derive agreement = predicate.agreement;
        form prepositional_adjunct_predicate = predicate adjunct;
    }
    construction instead_predicate: InsteadPredicate {
        element InsteadPredicateValue { predicate: VerbPhrase, }
        derive agreement = predicate.agreement;
        form instead_predicate = predicate "instead";
    }
    // A frequency adverbial counts occurrences of the predicate; "more than
    // once" is the comparative form.
    construction plain_frequency: FrequencyReference {
        element PlainFrequency { adverb: lex FrequencyAdverb, }
        form plain_frequency = lex(adverb);
    }
    construction comparative_frequency: FrequencyReference {
        element ComparativeFrequency {
            quantifier: lex ComparativeQuantifier,
            adverb: lex FrequencyAdverb,
        }
        form comparative_frequency = lex(quantifier) "than" lex(adverb);
    }
    construction frequency_predicate: FrequencyPredicate {
        element FrequencyPredicateValue {
            predicate: BaseVerbFrame,
            frequency: FrequencyReference,
        }
        derive agreement = predicate.agreement;
        form frequency_predicate = predicate frequency;
    }
    construction manner_predicate: MannerPredicate {
        element MannerPredicateValue {
            predicate: BaseVerbFrame,
            manner: MannerReference,
        }
        derive agreement = predicate.agreement;
        form manner_predicate = predicate manner;
    }
    construction alternative_predicate: AlternativePredicate {
        element AlternativePredicateValue {
            action: VerbPhrase,
            alternative: BaseVerbFrame,
        }
        derive action.agreement = Values::Bare;
        derive agreement = action.agreement;
        derive alternative.agreement = Values::Bare;
        form alternative_predicate = action "rather" "than" alternative;
    }
    construction without_gerund_object_predicate: WithoutGerundObjectPredicate {
        element WithoutGerundObjectPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            complement: Object,
        }
        derive agreement = head.agreement;
        form without_gerund_object_predicate =
            verb(head) object "without" "paying" complement;
    }
    construction controlled_cost_action: ControlledCostAction {
        element ControlledCostActionValue { head: lex TransitiveVerb, }
        derive head.agreement = Values::Bare;
        form controlled_cost_action = "to" verb(head);
    }
    construction cost_comparison_predicate: CostComparisonPredicate {
        element CostComparisonPredicateValue {
            head: lex CostComparisonVerb,
            amount: ManaAmount,
            direction: lex CostComparisonDirection,
            action: ControlledCostAction,
            basis: opt ForEachCostBasis,
        }
        derive agreement = head.agreement;
        form cost_comparison_predicate =
            verb(head) amount lex(direction) action basis;
    }
    construction for_each_cost_basis: ForEachCostBasis {
        element ForEachCostBasisValue { object: Object, }
        form for_each_cost_basis = "for" object;
    }
    construction restriction_turn: RestrictionTurn {
        element RestrictionTurnValue { endpoint: TemporalEndpoint, }
        form restriction_turn = endpoint;
    }
    construction only_if_restriction: CastingRestriction {
        element OnlyIfRestriction { condition: FiniteClause, }
        form only_if_restriction = "only" "if" condition;
    }
    construction only_during_restriction: CastingRestriction {
        element OnlyDuringRestriction { timing: RestrictionTurn, }
        form only_during_restriction = "only" "during" timing;
    }
    construction only_temporal_clause_restriction: CastingRestriction {
        element OnlyTemporalClauseRestriction {
            relation: lex Preposition,
            condition: FiniteClause,
        }
        form only_temporal_clause_restriction = "only" lex(relation) condition;
    }
    construction action_restriction_predicate: ActionRestrictionPredicate {
        element ActionRestrictionPredicateValue {
            _head: lex TransitiveVerb,
            object: Object,
            restrictions: seq CastingRestriction separated by " and ",
        }
        require len(restrictions) >= 1;
        derive _head.agreement = Values::Bare;
        derive agreement = _head.agreement;
        form action_restriction_predicate = verb(_head) object restrictions;
    }
    construction finite_passive_predicate: FinitePassivePredicate {
        element FinitePassivePredicateValue {
            copula: lex FiniteCopula,
            predicate: PassivePredicate,
        }
        derive copula.agreement = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Bare,
            Arent => Values::Bare,
            Was => Values::ThirdPersonSingular,
            Were => Values::Bare,
        };
        derive agreement = copula.agreement;
        form finite_passive_predicate = lex(copula) predicate;
    }
    construction auxiliary_predicate: AuxiliaryPredicate {
        element AuxiliaryPredicateValue {
            auxiliary: AuxiliaryHead,
            predicate: BarePredicate,
        }
        derive predicate.agreement = Values::Bare;
        derive agreement = auxiliary.agreement;
        form auxiliary_predicate = auxiliary predicate;
    }
    construction finite_copular_predicate: FiniteCopularPredicate {
        element FiniteCopularPredicateValue {
            copula: lex FiniteCopula,
            complement: PredicativeComplement,
        }
        derive copula.agreement = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Bare,
            Arent => Values::Bare,
            Was => Values::ThirdPersonSingular,
            Were => Values::Bare,
        };
        derive agreement = copula.agreement;
        form finite_copular_predicate = lex(copula) complement;
    }
    construction plain_finite_clause: FiniteClause {
        element PlainFiniteClause { subject: Subject, predicate: Predicate, }
        derive predicate.agreement = subject.agreement;
        form plain_finite_clause = subject predicate;
    }
    construction contracted_copular_clause: FiniteClause {
        element ContractedCopularClause {
            subject: lex ContractedCopularSubject,
            negator: opt lex PredicateNegator,
            complement: PredicativeComplement,
        }
        form contracted_copular_clause = lex(subject) lex(negator) complement;
    }
    construction contracted_perfect_transitive_clause: FiniteClause {
        element ContractedPerfectTransitiveClause {
            subject: lex ContractedPerfectSubject,
            head: lex DeclaredTransitiveParticipleHead,
            object: Object,
        }
        form contracted_perfect_transitive_clause = lex(subject) verb(head) object;
    }
    construction contracted_perfect_object_on_clause: FiniteClause {
        element ContractedPerfectObjectOnClause {
            subject: lex ContractedPerfectSubject,
            head: lex ObjectOnParticipleHead,
            object: Object,
            complement: FrameComplement,
        }
        form contracted_perfect_object_on_clause =
            lex(subject) verb(head) object "on" complement;
    }
    construction contracted_perfect_passive_clause: FiniteClause {
        element ContractedPerfectPassiveClause {
            subject: lex ContractedPerfectSubject,
            predicate: PassivePredicate,
            duration: opt DurationPhrase,
        }
        form contracted_perfect_passive_clause = lex(subject) "been" predicate duration;
    }
    construction existential_finite_clause: FiniteClause {
        element ExistentialFiniteClause {
            copula: lex FiniteCopula,
            pivot: NounPhrase,
            domain: opt PrepositionalPhrase,
        }
        derive copula.agreement = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Bare,
            Arent => Values::Bare,
            Was => Values::ThirdPersonSingular,
            Were => Values::Bare,
        };
        derive pivot.agreement = copula.agreement;
        form existential_finite_clause = "there" lex(copula) pivot domain;
    }
    construction floated_quantifier_finite_clause: FiniteClause {
        element FloatedQuantifierFiniteClause {
            subject: Subject,
            quantifier: lex FloatedQuantifier,
            predicate: Predicate,
        }
        derive subject.agreement = Values::Bare;
        derive predicate.agreement = Values::Bare;
        form floated_quantifier_finite_clause = subject lex(quantifier) predicate;
    }
    construction third_person_auxiliary_finite_clause: FiniteClause {
        element ThirdPersonAuxiliaryFiniteClause {
            subject: Subject,
            auxiliary: lex ThirdPersonAuxiliary,
            predicate: Predicate,
        }
        derive subject.agreement = Values::ThirdPersonSingular;
        derive predicate.agreement = Values::Bare;
        form third_person_auxiliary_finite_clause = subject lex(auxiliary) predicate;
    }
    construction bare_agreement_auxiliary_finite_clause: FiniteClause {
        element BareAgreementAuxiliaryFiniteClause {
            subject: Subject,
            auxiliary: lex BareAgreementAuxiliary,
            predicate: Predicate,
        }
        derive subject.agreement = Values::Bare;
        derive predicate.agreement = Values::Bare;
        form bare_agreement_auxiliary_finite_clause = subject lex(auxiliary) predicate;
    }
    construction and_clause_coordination: ClauseCoordination {
        element AndClauseCoordination {
            members: seq CoordinatedClause separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        form and_clause_coordination = members;
    }
    construction or_clause_coordination: ClauseCoordination {
        element OrClauseCoordination {
            members: seq CoordinatedClause separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        form or_clause_coordination = members;
    }
    construction and_or_clause_coordination: ClauseCoordination {
        element AndOrClauseCoordination {
            members: seq CoordinatedClause separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        form and_or_clause_coordination = members;
    }
    construction where: WhereClauseCategory {
        element WhereClause { clause: FiniteClause, }
        form where = "where" clause;
    }
    construction subject_nominal: Subject {
        element NominalSubject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form subject_nominal = value;
    }
    construction subject_pronoun: Subject {
        element PersonalSubject { word: lex SubjectPronoun, }
        derive word.agreement = match word {
            He => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            She => Values::ThirdPersonSingular,
            They => Values::Bare,
            You => Values::Bare,
        };
        derive agreement = word.agreement;
        derive number = Values::Singular;
        derive onset = word.onset;
        form subject_pronoun = lex(word);
    }
    construction variable_subject: Subject {
        element VariableSubject { variable: lex Variable, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = variable.onset;
        form variable_subject = lex(variable);
    }
    construction object_nominal: Object {
        element NominalObject {
            value: NounPhrase checked by nominal_object_is_not_fused_all(),
        }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form object_nominal = value;
    }
    construction bare_singular_coordination_object: Object {
        element BareSingularCoordinationObject { value: SingularNominalCoordination, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form bare_singular_coordination_object = value;
    }
    construction object_pronoun: Object {
        element PersonalObject { word: lex ObjectPronoun, }
        derive agreement = match word {
            Her => Values::ThirdPersonSingular,
            Him => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            Them => Values::Bare,
            You => Values::Bare,
        };
        derive number = match word {
            Her => Values::Singular,
            Him => Values::Singular,
            It => Values::Singular,
            Them => Values::Plural,
            You => Values::Singular,
        };
        derive onset = word.onset;
        form object_pronoun = lex(word);
    }
    construction reflexive_object: Object {
        element ReflexiveObject { word: lex ReflexivePronoun, }
        derive agreement = match word {
            Herself => Values::ThirdPersonSingular,
            Himself => Values::ThirdPersonSingular,
            Itself => Values::ThirdPersonSingular,
            Themself => Values::Bare,
            Themselves => Values::Bare,
            Yourself => Values::Bare,
            Yourselves => Values::Bare,
        };
        derive number = match word {
            Herself => Values::Singular,
            Himself => Values::Singular,
            Itself => Values::Singular,
            Themself => Values::Singular,
            Themselves => Values::Plural,
            Yourself => Values::Singular,
            Yourselves => Values::Plural,
        };
        derive onset = word.onset;
        form reflexive_object = lex(word);
    }
    construction noun_singular_head: SingularHead {
        element NounSingularHead { noun: lex Noun, }
        require noun.countability is Count;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form noun_singular_head = noun(noun);
    }
    construction noun_plural_head: PluralHead {
        element NounPluralHead { noun: lex Noun, }
        require noun.countability is Count;
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form noun_plural_head = noun(noun);
    }
    construction bare_locative_proform: BareLocative {
        element BareLocativeProform { word: lex LocativeProform, }
        derive number = Values::Singular;
        form bare_locative_proform = lex(word);
    }
    construction bare_locative_noun: BareLocative {
        element BareLocativeNoun {
            noun: lex Noun,
        }
        require noun.bare_locative_license is BareAllowed;
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form bare_locative_noun = noun(noun);
    }
    construction color_modifier: NominalModifier {
        element ColorModifier { color: lex Color, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = color.onset;
        form color_modifier = lex(color);
    }
    construction attributive_adjective_modifier: NominalModifier {
        element AttributiveAdjectiveModifier { adjective: lex AttributiveAdjective, }
        derive modifier_license = adjective.modifier_license;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = match adjective {
            Additional => Values::Vowel,
            Base => Values::Consonant,
            Declare => Values::Consonant,
            FaceDown => Values::Consonant,
            First => Values::Consonant,
            Main => Values::Consonant,
            Maximum => Values::Consonant,
            Next => Values::Consonant,
            Other => Values::Vowel,
            Postcombat => Values::Consonant,
            Precombat => Values::Consonant,
            Same => Values::Consonant,
            Second => Values::Consonant,
            SixSided => Values::Consonant,
            Target => Values::Consonant,
            Untap => Values::Vowel,
        };
        form attributive_adjective_modifier = lex(adjective);
    }
    construction counter_kind_modifier: NominalModifier {
        element CounterKindModifier { kind: CounterKind, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = kind.onset;
        form counter_kind_modifier = kind;
    }
    construction power_toughness_modifier: NominalModifier {
        element PowerToughnessModifier { value: PredicativePowerToughnessComplement, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form power_toughness_modifier = value;
    }
    construction status_modifier: NominalModifier {
        element StatusModifier { status: lex Status, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = status.onset;
        form status_modifier = lex(status);
    }
    construction participial_adjective_modifier: NominalModifier {
        element ParticipialAdjectiveModifier { adjective: ParticipialAdjective, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = adjective.onset;
        form participial_adjective_modifier = adjective;
    }
    construction reduced_relative_modifier: NominalModifier {
        element ReducedRelativeModifier { head: lex DeclaredTransitiveParticipleHead, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = head.onset;
        form reduced_relative_modifier = verb(head);
    }
    construction supertype_modifier: NominalModifier {
        element SupertypeModifier { supertype: lex Supertype, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = supertype.onset;
        form supertype_modifier = lex(supertype);
    }
    construction noun_modifier: NominalModifier {
        element NounModifier { noun: lex Noun, }
        require noun.compoundability is Compoundable;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form noun_modifier = noun(noun);
    }
    construction non_color_modifier: NominalModifier {
        element NonColorModifier { color: lex Color, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_color_modifier = prefix("non", lex(color));
    }
    construction non_noun_modifier: NominalModifier {
        element NonNounModifier { noun: lex Noun, }
        require noun.properness is Common;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_noun_modifier = prefix("non", noun(noun));
    }
    construction non_proper_noun_modifier: NominalModifier {
        element NonProperNounModifier { noun: lex Noun, }
        require noun.properness is Proper;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_proper_noun_modifier = prefix("non-", noun(noun));
    }
    construction non_status_modifier: NominalModifier {
        element NonStatusModifier { status: lex Status, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_status_modifier = prefix("non", lex(status));
    }
    construction non_supertype_modifier: NominalModifier {
        element NonSupertypeModifier { supertype: lex Supertype, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_supertype_modifier = prefix("non", lex(supertype));
    }
    construction negative_modifier_member: NegativeNominalModifier {
        element NegativeModifierMember { value: NominalModifier, }
        require any(
            value is NonColorModifier,
            value is NonNounModifier,
            value is NonProperNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier
        );
        form negative_modifier_member = value;
    }
    construction coordinated_modifier_member: CoordinatedNominalModifier {
        element CoordinatedModifierMember { value: NominalModifier, }
        require value.modifier_license is Unrestricted;
        require any(
            value is AttributiveAdjectiveModifier,
            value is ParticipialAdjectiveModifier,
            value is ColorModifier,
            value is NounModifier,
            value is StatusModifier,
            value is SupertypeModifier,
            value is NonColorModifier,
            value is NonNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier
        );
        derive onset = value.onset;
        form coordinated_modifier_member = value;
    }
    construction and_shared_head_modifier: NominalModifier {
        element AndSharedHeadModifier {
            first: CoordinatedNominalModifier,
            rest: seq CoordinatedNominalModifier separated by " and ",
        }
        require len(rest) >= 1;
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form and_shared_head_modifier = first "and" rest;
    }
    construction final_serial_and_modifier_tail: SerialAndModifierTail {
        element FinalSerialAndModifierTail {
            penultimate: CoordinatedNominalModifier,
            last: CoordinatedNominalModifier,
        }
        form final_serial_and_modifier_tail = penultimate "," "and" last;
    }
    construction recursive_serial_and_modifier_tail: SerialAndModifierTail {
        element RecursiveSerialAndModifierTail {
            next: CoordinatedNominalModifier,
            tail: SerialAndModifierTail,
        }
        form recursive_serial_and_modifier_tail = next "," tail;
    }
    construction serial_and_shared_head_modifier: NominalModifier {
        element SerialAndSharedHeadModifier {
            first: CoordinatedNominalModifier,
            tail: SerialAndModifierTail,
        }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form serial_and_shared_head_modifier = first "," tail;
    }
    construction or_shared_head_modifier: NominalModifier {
        element OrSharedHeadModifier {
            first: CoordinatedNominalModifier,
            rest: seq CoordinatedNominalModifier separated by " or ",
        }
        require len(rest) >= 1;
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form or_shared_head_modifier = first "or" rest;
    }
    construction and_or_shared_head_modifier: NominalModifier {
        element AndOrSharedHeadModifier {
            first: CoordinatedNominalModifier,
            rest: seq CoordinatedNominalModifier separated by " and/or ",
        }
        require len(rest) >= 1;
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form and_or_shared_head_modifier = first "and/or" rest;
    }
    construction bare_singular_nominal: SingularNominal {
        element BareSingularNominal { head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        form bare_singular_nominal = head;
    }
    construction modified_singular_nominal: SingularNominal {
        element ModifiedSingularNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: SingularHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::ModifiedSingularNoun;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        form modified_singular_nominal = first rest head;
    }
    construction negative_modified_singular_nominal: SingularNominal {
        element NegativeModifiedSingularNominal {
            leading: opt NominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: SingularHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::ModifiedSingularNoun;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        form negative_modified_singular_nominal = leading modifiers head;
    }
    construction bare_relational_reference: UnqualifiedReference {
        element BareRelationalReference { head: lex Noun, }
        require head.relationality is Relational;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        derive head.number = Values::Singular;
        form bare_relational_reference = noun(head);
    }
    construction modified_bare_relational_reference: UnqualifiedReference {
        element ModifiedBareRelationalReference {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: lex Noun,
        }
        require head.relationality is Relational;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        derive head.number = Values::Singular;
        form modified_bare_relational_reference = first rest noun(head);
    }
    construction participial_singular_reference: UnqualifiedReference {
        element ParticipialSingularReference {
            adjective: ParticipialAdjective,
            head: SingularHead,
        }
        derive agreement = head.agreement;
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = adjective.onset;
        derive possessive_ending = head.possessive_ending;
        form participial_singular_reference = adjective head;
    }
    construction premodified_participial_singular_reference: UnqualifiedReference {
        element PremodifiedParticipialSingularReference {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            adjective: ParticipialAdjective,
            head: SingularHead,
        }
        require first.modifier_license is LocalDeterminer;
        derive agreement = head.agreement;
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        form premodified_participial_singular_reference = first rest adjective head;
    }
    construction bare_plural_nominal: PluralNominal {
        element BarePluralNominal { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::BarePluralNoun;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        form bare_plural_nominal = head;
    }
    construction modified_plural_nominal: PluralNominal {
        element ModifiedPluralNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: PluralHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::ModifiedPluralNoun;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        form modified_plural_nominal = first rest head;
    }
    construction negative_modified_plural_nominal: PluralNominal {
        element NegativeModifiedPluralNominal {
            leading: opt NominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: PluralHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::ModifiedPluralNoun;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        form negative_modified_plural_nominal = leading modifiers head;
    }
    construction bare_singular_coordination_member: SingularCoordinationMember {
        element BareSingularCoordinationMember { head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        form bare_singular_coordination_member = head;
    }
    construction modified_singular_coordination_member: SingularCoordinationMember {
        element ModifiedSingularCoordinationMember {
            modifier: CoordinatedNominalModifier,
            head: SingularHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        derive possessive_ending = head.possessive_ending;
        form modified_singular_coordination_member = modifier head;
    }
    construction negative_modified_singular_coordination_member: SingularCoordinationMember {
        element NegativeModifiedSingularCoordinationMember {
            leading: opt CoordinatedNominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: SingularHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        form negative_modified_singular_coordination_member = leading modifiers head;
    }
    construction bare_plural_coordination_member: PluralCoordinationMember {
        element BarePluralCoordinationMember { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        form bare_plural_coordination_member = head;
    }
    construction modified_plural_coordination_member: PluralCoordinationMember {
        element ModifiedPluralCoordinationMember {
            modifier: CoordinatedNominalModifier,
            head: PluralHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        derive possessive_ending = head.possessive_ending;
        form modified_plural_coordination_member = modifier head;
    }
    construction negative_modified_plural_coordination_member: PluralCoordinationMember {
        element NegativeModifiedPluralCoordinationMember {
            leading: opt CoordinatedNominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: PluralHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        form negative_modified_plural_coordination_member = leading modifiers head;
    }
    construction singular_and_nominal_coordination: SingularNominalCoordination {
        element SingularAndNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::SingularCoordination;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form singular_and_nominal_coordination = members;
    }
    construction singular_or_nominal_coordination: SingularNominalCoordination {
        element SingularOrNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::SingularCoordination;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form singular_or_nominal_coordination = members;
    }
    construction singular_and_or_nominal_coordination: SingularNominalCoordination {
        element SingularAndOrNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::SingularCoordination;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form singular_and_or_nominal_coordination = members;
    }
    construction plural_and_nominal_coordination: PluralNominalCoordination {
        element PluralAndNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive nominal_form = Values::PluralCoordination;
        derive onset = Values::Consonant;
        derive possessive_ending = Values::Other;
        form plural_and_nominal_coordination = members;
    }
    construction plural_or_nominal_coordination: PluralNominalCoordination {
        element PluralOrNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive nominal_form = Values::PluralCoordination;
        derive onset = Values::Consonant;
        derive possessive_ending = Values::Other;
        form plural_or_nominal_coordination = members;
    }
    construction plural_and_or_nominal_coordination: PluralNominalCoordination {
        element PluralAndOrNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive nominal_form = Values::PluralCoordination;
        derive onset = Values::Consonant;
        derive possessive_ending = Values::Other;
        form plural_and_or_nominal_coordination = members;
    }
    construction singular_nominal_value: Nominal {
        element SingularNominalValue { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive nominal_form = nominal.nominal_form;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        form singular_nominal_value = nominal;
    }
    construction plural_nominal_value: Nominal {
        element PluralNominalValue { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive nominal_form = nominal.nominal_form;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        form plural_nominal_value = nominal;
    }
    construction mass_noun: MassNoun {
        element MassNounValue { noun: lex Noun, }
        require noun.countability is Mass;
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form mass_noun = noun(noun);
    }
    construction mass_nominal: Nominal {
        element MassNominal { noun: MassNoun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::MassNoun;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form mass_nominal = noun;
    }
    construction modified_mass_nominal: Nominal {
        element ModifiedMassNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            noun: MassNoun,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::MassNoun;
        derive onset = first.onset;
        derive possessive_ending = noun.possessive_ending;
        form modified_mass_nominal = first rest noun;
    }
    construction singular_coordination_nominal_value: Nominal {
        element SingularCoordinationNominalValue {
            coordination: SingularNominalCoordination,
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive nominal_form = coordination.nominal_form;
        derive onset = coordination.onset;
        derive possessive_ending = coordination.possessive_ending;
        form singular_coordination_nominal_value = coordination;
    }
    construction plural_coordination_nominal_value: Nominal {
        element PluralCoordinationNominalValue {
            coordination: PluralNominalCoordination,
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive nominal_form = coordination.nominal_form;
        derive onset = coordination.onset;
        derive possessive_ending = coordination.possessive_ending;
        form plural_coordination_nominal_value = coordination;
    }
    construction modified_singular_coordination_nominal_value: Nominal {
        element ModifiedSingularCoordinationNominalValue {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            coordination: SingularNominalCoordination,
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive nominal_form = coordination.nominal_form;
        derive onset = first.onset;
        derive possessive_ending = coordination.possessive_ending;
        form modified_singular_coordination_nominal_value = first rest coordination;
    }
    construction modified_plural_coordination_nominal_value: Nominal {
        element ModifiedPluralCoordinationNominalValue {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            coordination: PluralNominalCoordination,
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive nominal_form = coordination.nominal_form;
        derive onset = first.onset;
        derive possessive_ending = coordination.possessive_ending;
        form modified_plural_coordination_nominal_value = first rest coordination;
    }
    construction unmarked_singular_selector: SingularSelector {
        element UnmarkedSingularSelector { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form unmarked_singular_selector = nominal;
    }
    construction singular_simple_determinative: Determinative {
        element SingularSimpleDeterminative {
            head: lex DeterminativeHead checked by determinative_licenses_singular(
                head.determiner_number
            ),
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = head.determiner_number;
        derive nominal_license = head.nominal_license;
        derive fused_head_license = head.fused_head_license;
        derive onset = head.onset;
        form singular_simple_determinative = lex(head);
    }
    construction plural_simple_determinative: Determinative {
        element PluralSimpleDeterminative {
            head: lex DeterminativeHead checked by determinative_licenses_plural(
                head.determiner_number
            ),
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = head.determiner_number;
        derive nominal_license = head.nominal_license;
        derive fused_head_license = head.fused_head_license;
        derive onset = head.onset;
        form plural_simple_determinative = lex(head);
    }
    construction cardinal_quantifying_determiner: Determinative {
        element CardinalQuantifyingDeterminer { count: CardinalQuantity, }
        require any(count.cardinality is One, count.cardinality is TwoPlus);
        derive agreement = count.agreement;
        derive number = count.number;
        derive determiner_number = count.determiner_number;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form cardinal_quantifying_determiner = count;
    }
    construction mass_quantity_determiner: Determinative {
        element MassQuantityDeterminer { amount: Amount, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_quantity_determiner = amount;
    }
    construction mass_comparison_determiner: Determinative {
        element MassComparisonDeterminer { comparison: ScalarComparison, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_comparison_determiner = comparison;
    }
    construction mass_cardinal_quantity_determiner: Determinative {
        element MassCardinalQuantityDeterminer { count: CardinalQuantity, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_cardinal_quantity_determiner = count;
    }
    construction variable_quantifying_determiner: Determinative {
        element VariableQuantifyingDeterminer { count: lex Variable, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form variable_quantifying_determiner = lex(count);
    }
    construction up_to_quantifying_determiner: Determinative {
        element UpToQuantifyingDeterminer { count: CardinalQuantity, }
        require any(count.cardinality is One, count.cardinality is TwoPlus);
        derive agreement = count.agreement;
        derive number = count.number;
        derive determiner_number = count.determiner_number;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Vowel;
        form up_to_quantifying_determiner = "up" "to" count;
    }
    construction any_number_quantifying_determiner: Determinative {
        element AnyNumberQuantifyingDeterminer { unit: SingularHead, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Vowel;
        form any_number_quantifying_determiner = "any" unit "of";
    }
    construction no_more_quantifying_determiner: Determinative {
        element NoMoreQuantifyingDeterminer {}
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form no_more_quantifying_determiner = "no" "more";
    }
    construction counted_quantifying_determiner: Determinative {
        element CountedQuantifyingDeterminer { count: CountReference, }
        derive agreement = count.agreement;
        derive number = count.number;
        derive determiner_number = Values::Both;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = count.onset;
        form counted_quantifying_determiner = count;
    }
    construction count_comparison_quantifying_determiner: Determinative {
        element CountComparisonQuantifyingDeterminer {
            count: CardinalQuantity,
            comparison: CountComparison,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form count_comparison_quantifying_determiner = count comparison;
    }
    construction named_card_reference: UnqualifiedReference {
        element NamedCardReference { kind: SingularHead, name: identity CardName, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        derive possessive_ending = name.possessive_ending;
        form named_card_reference = "a" kind "named" identity(name);
    }
    construction definite_next_mass_quantity_reference: UnqualifiedReference {
        element DefiniteNextMassQuantityReference {
            quantity: Amount,
            noun: MassNoun,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        derive possessive_ending = noun.possessive_ending;
        form definite_next_mass_quantity_reference = "the" "next" quantity noun;
    }
    construction that_many: CountReference {
        element ThatMany {}
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form that_many = "that" "many";
    }
    construction demonstrative_possessive_reference: UnqualifiedReference {
        element DemonstrativePossessiveReference {
            demonstrative: lex SingularDemonstrative,
            possessor: Possessive,
            possessed: SingularNominal,
        }
        require possessor.number is Singular;
        derive agreement = possessed.agreement;
        derive number = possessed.number;
        derive onset = Values::Consonant;
        derive possessive_ending = possessed.possessive_ending;
        form demonstrative_possessive_reference = lex(demonstrative) possessor possessed;
    }
    construction possessed_singular_reference: UnqualifiedReference {
        element PossessedSingularReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        form possessed_singular_reference = lex(possessor) nominal;
    }
    construction possessed_plural_reference: UnqualifiedReference {
        element PossessedPluralReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: PluralNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        form possessed_plural_reference = lex(possessor) nominal;
    }
    construction possessed_mass_reference: UnqualifiedReference {
        element PossessedMassReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: Nominal,
        }
        require nominal.nominal_form is MassNoun;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        form possessed_mass_reference = lex(possessor) nominal;
    }
    construction genitive_determiner_singular_reference: UnqualifiedReference {
        element GenitiveDeterminerSingularReference {
            possessor: Possessive,
            nominal: SingularNominal,
        }
        require possessor.number is Singular;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form genitive_determiner_singular_reference = possessor nominal;
    }
    construction genitive_determiner_plural_reference: UnqualifiedReference {
        element GenitiveDeterminerPluralReference {
            possessor: Possessive,
            nominal: PluralNominal,
        }
        require possessor.number is Singular;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form genitive_determiner_plural_reference = possessor nominal;
    }
    construction plural_genitive_determiner_singular_reference: UnqualifiedReference {
        element PluralGenitiveDeterminerSingularReference {
            possessor: Possessive,
            nominal: SingularNominal,
        }
        require possessor.number is Plural;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form plural_genitive_determiner_singular_reference = possessor nominal;
    }
    construction plural_genitive_determiner_plural_reference: UnqualifiedReference {
        element PluralGenitiveDeterminerPluralReference {
            possessor: Possessive,
            nominal: PluralNominal,
        }
        require possessor.number is Plural;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form plural_genitive_determiner_plural_reference = possessor nominal;
    }
    construction genitive_determiner_mass_reference: UnqualifiedReference {
        element GenitiveDeterminerMassReference {
            possessor: Possessive,
            nominal: Nominal,
        }
        require all(
            possessor.number is Singular,
            nominal.nominal_form is MassNoun
        );
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form genitive_determiner_mass_reference = possessor nominal;
    }
    construction plural_genitive_determiner_mass_reference: UnqualifiedReference {
        element PluralGenitiveDeterminerMassReference {
            possessor: Possessive,
            nominal: Nominal,
        }
        require all(
            possessor.number is Plural,
            nominal.nominal_form is MassNoun
        );
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        form plural_genitive_determiner_mass_reference = possessor nominal;
    }
    construction genitive_determiner_coordination_reference: UnqualifiedReference {
        element GenitiveDeterminerCoordinationReference {
            possessor: Possessive,
            coordination: SingularNominalCoordination,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        derive possessive_ending = coordination.possessive_ending;
        form genitive_determiner_coordination_reference = possessor coordination;
    }
    construction possessive_absolute_reference: UnqualifiedReference {
        element PossessiveAbsoluteReference { word: lex PossessiveAbsolutePronoun, }
        derive agreement = match word {
            Hers => Values::ThirdPersonSingular,
            His => Values::ThirdPersonSingular,
            Theirs => Values::Bare,
            Yours => Values::Bare,
        };
        derive number = match word {
            Hers => Values::Singular,
            His => Values::Singular,
            Theirs => Values::Singular,
            Yours => Values::Singular,
        };
        derive onset = word.onset;
        derive possessive_ending = word.possessive_ending;
        form possessive_absolute_reference = lex(word);
    }
    construction determined_nominal: UnqualifiedReference {
        element DeterminedNominal {
            det: zeroable Determiner from Determinative checked by determiner_licenses_nominal(
                det.determiner_number,
                det.nominal_license,
                nominal.number,
                nominal.nominal_form
            ),
            nominal: Nominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        form determined_nominal = det nominal;
    }
    construction all_predetermined_nominal: UnqualifiedReference {
        element AllPredeterminedNominal {
            all: Determinative checked by determinative_is_plural_all(all.number),
            det: Determinative checked by headed_determiner_licenses_nominal(
                det.determiner_number,
                det.nominal_license,
                nominal.number,
                nominal.nominal_form
            ),
            nominal: Nominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        form all_predetermined_nominal = all det nominal;
    }
    construction full_and_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndNounPhraseCoordination {
            members: seq ControllerStage separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive possessive_ending = members.possessive_ending;
        form full_and_noun_phrase_coordination = members;
    }
    construction full_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullOrNounPhraseCoordination {
            members: seq ControllerStage separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive possessive_ending = members.possessive_ending;
        form full_or_noun_phrase_coordination = members;
    }
    construction full_and_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndOrNounPhraseCoordination {
            members: seq ControllerStage separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Plural;
        derive possessive_ending = members.possessive_ending;
        form full_and_or_noun_phrase_coordination = members;
    }
    construction coordinated_noun_phrase: UnqualifiedReference {
        element CoordinatedNounPhrase {
            coordination: FullNounPhraseCoordination checked by full_coordination_is_independent(),
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive onset = Values::Consonant;
        derive possessive_ending = coordination.possessive_ending;
        form coordinated_noun_phrase = coordination;
    }
    construction self_reference: UnqualifiedReference {
        element SourceSelfReference { spelling: identity SelfReferenceSpelling, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = spelling.onset;
        derive possessive_ending = spelling.possessive_ending;
        form self_reference = identity(spelling);
    }
    construction this_way: MannerReference {
        element ThisWay {
            demonstrative: lex SingularDemonstrative checked by singular_demonstrative_is_this(),
            noun: lex Noun checked by noun_is_way(),
        }
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form this_way = lex(demonstrative) noun(noun);
    }
    construction at_random_manner: MannerReference {
        element AtRandomManner {}
        derive number = Values::Singular;
        form at_random_manner = "at" "random";
    }
    construction that_much: ScalarReference {
        element ThatMuch {}
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form that_much = "that" "much";
    }
    construction positive_object_gap_relative: PositiveObjectGapRelativeClause {
        element PositiveObjectGapRelativeClauseValue {
            subject: Subject,
            head: lex TransitiveVerb,
        }
        derive head.agreement = subject.agreement;
        form positive_object_gap_relative = subject verb(head);
    }
    construction auxiliary_object_gap_relative: AuxiliaryObjectGapRelativeClause {
        element AuxiliaryObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: AuxiliaryHead,
            head: lex TransitiveVerb,
        }
        derive auxiliary.agreement = subject.agreement;
        derive head.agreement = Values::Bare;
        form auxiliary_object_gap_relative = subject auxiliary verb(head);
    }
    construction contracted_perfect_object_gap_relative: ContractedPerfectObjectGapRelativeClause {
        element ContractedPerfectObjectGapRelativeClauseValue {
            subject: lex ContractedPerfectSubject,
            head: lex DeclaredTransitiveParticipleHead,
        }
        form contracted_perfect_object_gap_relative = lex(subject) verb(head);
    }
    construction bare_negative_object_gap_relative: BareNegativeObjectGapRelativeClause {
        element BareNegativeObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: lex BareAgreementAuxiliary,
            head: lex TransitiveVerb,
        }
        derive subject.agreement = Values::Bare;
        derive head.agreement = Values::Bare;
        form bare_negative_object_gap_relative = subject lex(auxiliary) verb(head);
    }
    construction third_person_negative_object_gap_relative: ThirdPersonNegativeObjectGapRelativeClause {
        element ThirdPersonNegativeObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: lex ThirdPersonAuxiliary,
            head: lex TransitiveVerb,
        }
        derive subject.agreement = Values::ThirdPersonSingular;
        derive head.agreement = Values::Bare;
        form third_person_negative_object_gap_relative = subject lex(auxiliary) verb(head);
    }
    construction singular_partitive_selection: PartitiveSelection {
        element SingularPartitiveSelection { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form singular_partitive_selection = nominal;
    }
    construction fixed_partitive_selection: PartitiveSelection {
        element FixedPartitiveSelection {
            count: CardinalQuantity,
            nominal: PluralNominal,
        }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        form fixed_partitive_selection = count nominal;
    }
    // The single prepositional phrase. Temporal, locative, source, goal and
    // manner readings are values of the preposition, never categories.
    construction prepositional_phrase: PrepositionalPhrase {
        element PrepositionalPhraseValue {
            preposition: lex Preposition,
            complement: PrepositionalComplement,
        }
        derive preposition_class = preposition.preposition_class;
        form prepositional_phrase = lex(preposition) complement;
    }
    // A determiner-less locative is licensed by the preposition class as well
    // as by the noun's own bare-locative license.
    construction bare_locative_prepositional_phrase: PrepositionalPhrase {
        element BareLocativePrepositionalPhrase {
            preposition: lex Preposition,
            complement: BareLocative,
        }
        require preposition.preposition_class is PostmodifierBareLocative;
        derive preposition_class = preposition.preposition_class;
        form bare_locative_prepositional_phrase = lex(preposition) complement;
    }
    // Two frame roles whose preposition is declared valence data. They exist
    // because a codec tail cannot carry an optional literal.
    construction source_phrase: SourcePhrase {
        element SourcePhraseValue { complement: FrameComplement, }
        form source_phrase = "from" complement;
    }
    construction control_phrase: ControlPhrase {
        element ControlPhraseValue { complement: Object, }
        form control_phrase = "under" complement;
    }
    construction edge_of_phrase: EdgeOfPhrase {
        element EdgeOfPhraseValue {
            position: lex EdgePosition,
            whole: Object,
        }
        form top when position is Top = lex(position) "of" whole;
        form bottom otherwise = "the" lex(position) "of" whole;
    }
    construction fixed_scalar_threshold: ScalarThreshold {
        element FixedScalarThreshold { value: lex ScalarNumber, }
        form fixed_scalar_threshold = lex(value);
    }
    construction variable_scalar_threshold: ScalarThreshold {
        element VariableScalarThreshold { value: lex Variable, }
        form variable_scalar_threshold = lex(value);
    }
    construction nominal_scalar_measure: ScalarMeasure {
        element NominalScalarMeasure { nominal: SingularNominal, }
        form nominal_scalar_measure = nominal;
    }
    construction scalar_or_less: ScalarComparison {
        element ScalarOrLess { threshold: ScalarThreshold, }
        form scalar_or_less = threshold "or" "less";
    }
    construction scalar_or_greater: ScalarComparison {
        element ScalarOrGreater { threshold: ScalarThreshold, }
        form scalar_or_greater = threshold "or" "greater";
    }
    construction scalar_less_than: ScalarComparison {
        element ScalarLessThan { threshold: ScalarThreshold, }
        form scalar_less_than = "less" "than" threshold;
    }
    construction scalar_greater_than: ScalarComparison {
        element ScalarGreaterThan { threshold: ScalarThreshold, }
        form scalar_greater_than = "greater" "than" threshold;
    }
    construction scalar_less_than_or_equal_to: ScalarComparison {
        element ScalarLessThanOrEqualTo { threshold: ScalarThreshold, }
        form scalar_less_than_or_equal_to = "less" "than" "or" "equal" "to" threshold;
    }
    construction count_or_more: CountComparison {
        element CountOrMore {}
        form count_or_more = "or" "more";
    }
    construction count_or_fewer: CountComparison {
        element CountOrFewer {}
        form count_or_fewer = "or" "fewer";
    }
    construction count_or_both: CountComparison {
        element CountOrBoth {}
        form count_or_both = "or" "both";
    }
    construction scalar_qualification: ScalarQualification {
        element ScalarQualificationValue {
            measure: ScalarMeasure,
            comparison: ScalarComparison,
        }
        form scalar_qualification = "with" measure comparison;
    }
    construction single_scalar_degree_phrase: SingleScalarDegreePhrase {
        element SingleScalarDegreePhraseValue { degree: lex ScalarDegree, }
        form single_scalar_degree_phrase = lex(degree);
    }
    construction or_scalar_degree_phrase: OrScalarDegreePhrase {
        element OrScalarDegreePhraseValue {
            first: lex ScalarDegree,
            rest: seq lex ScalarDegree separated by " or ",
        }
        require len(rest) >= 1;
        form or_scalar_degree_phrase = lex(first) "or" lex(rest);
    }
    construction degree_scalar_qualification: ScalarQualification {
        element DegreeScalarQualification {
            degree: ScalarDegreePhrase,
            measure: ScalarMeasure,
        }
        form degree_scalar_qualification = "with" degree measure;
    }
    construction possessed_scalar_value: ScalarValue {
        element PossessedScalarValue {
            possessor: lex PossessiveDeterminerPronoun,
            measure: ScalarMeasure,
        }
        form possessed_scalar_value = lex(possessor) measure;
    }
    construction genitive_scalar_value: ScalarValue {
        element GenitiveScalarValue {
            possessor: Possessive,
            measure: ScalarMeasure,
        }
        form genitive_scalar_value = "the" possessor measure;
    }
    construction number_of_scalar_value: ScalarValue {
        element NumberOfScalarValue { measure: SingularHead, counted: Object, }
        form number_of_scalar_value = "the" measure "of" counted;
    }
    construction twice_scalar_value: ScalarValue {
        element TwiceScalarValue { value: ScalarValue, }
        form twice_scalar_value = "twice" value;
    }
    construction offset_scalar_value: ScalarValue {
        element OffsetScalarValue {
            offset: CardinalQuantity,
            basis: ScalarValue,
        }
        form offset_scalar_value = offset "plus" basis;
    }
    construction greatest_scalar_value: ScalarValue {
        element GreatestScalarValue {
            measure: ScalarMeasure,
            domain: Object,
        }
        form greatest_scalar_value = "the" "greatest" measure "among" domain;
    }
    construction scalar_equality: ScalarEquality {
        element ScalarEqualityValue { value: ScalarValue, }
        form scalar_equality = "equal" "to" value;
    }
    construction unqualified_controller_stage: ControllerStage {
        element UnqualifiedControllerStage { reference: UnqualifiedReference, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        form unqualified_controller_stage = reference;
    }
    construction relative_qualified_reference: ControllerStage {
        element RelativeQualifiedReference {
            reference: UnqualifiedReference,
            clause: ObjectGapRelativeClause,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        form relative_qualified_reference = reference clause;
    }
    construction subject_relative_qualified_reference: ControllerStage {
        element SubjectRelativeQualifiedReference {
            reference: UnqualifiedReference,
            clause: SubjectGapRelativeClause,
        }
        derive agreement = reference.agreement;
        derive clause.agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        form subject_relative_qualified_reference = reference clause;
    }
    construction contracted_copular_relative_reference: ControllerStage {
        element ContractedCopularRelativeReference {
            reference: UnqualifiedReference,
            nominal: SingularNominal,
            complement: Object,
        }
        require reference.number is Singular;
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = Values::Other;
        form contracted_copular_relative_reference =
            reference "that's" "a" nominal "of" complement;
    }
    construction reduced_passive_qualified_reference: ControllerStage {
        element ReducedPassiveQualifiedReference {
            reference: UnqualifiedReference,
            clause: PassivePredicate,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        form reduced_passive_qualified_reference = reference clause;
    }
    construction other_than_qualified_reference: ControllerStage {
        element OtherThanQualifiedReference {
            reference: UnqualifiedReference,
            excluded: UnqualifiedReference,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = excluded.possessive_ending;
        form other_than_qualified_reference = reference "other" "than" excluded;
    }
    construction unqualified_locative_stage: LocativeStage {
        element UnqualifiedLocativeStage { reference: ControllerStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_locative_stage = reference;
    }
    // Low attachment: a prepositional phrase postmodifies the nearest
    // nominal that licenses it, per the derived-attachment ruling.
    construction prepositional_qualified_reference: LocativeStage {
        element PrepositionalQualifiedReference {
            reference: ControllerStage,
            modifier: PrepositionalPhrase,
        }
        require modifier.preposition_class in [AdjunctCapable, PostmodifierOnly, PostmodifierBareLocative];
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form prepositional_qualified_reference = reference modifier;
    }
    construction unqualified_numeric_stage: NumericStage {
        element UnqualifiedNumericStage { reference: LocativeStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_numeric_stage = reference;
    }
    construction scalar_qualified_reference: NumericStage {
        element ScalarQualifiedReference {
            reference: LocativeStage,
            scalar: ScalarQualification,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form scalar_qualified_reference = reference scalar;
    }
    // One nominal `with` postmodifier for granted abilities, whose complement
    // is either a keyword-line item ("with flying", "with ward {2}") or a
    // quoted ability document ("with \"When this creature dies, ...\"").
    abstract sum GrantedAbility {
        Keyword: KeywordLineItem,
        Quoted: QuotedAbility,
    }
    construction granted_ability_qualified_reference: NumericStage {
        element GrantedAbilityQualifiedReference {
            reference: LocativeStage,
            granted: GrantedAbility,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form granted_ability_qualified_reference = reference "with" granted;
    }
    construction qualified_noun_phrase: NounPhrase {
        element QualifiedNounPhrase { reference: NumericStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form qualified_noun_phrase = reference;
    }
    construction comparative_quantified_reference: NounPhrase {
        element ComparativeQuantifiedReference {
            quantifier: lex ComparativeQuantifier,
            nominal: PluralNominal,
            standard: Object,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form comparative_quantified_reference = lex(quantifier) nominal "than" standard;
    }
    construction fused_determinative_reference: NounPhrase {
        element FusedDeterminativeReference {
            head: Determinative checked by determinative_is_fused(head.fused_head_license),
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        form fused_determinative_reference = head;
    }
    construction determinative_partitive: UnqualifiedReference {
        element DeterminativePartitive {
            head: Determinative checked by determinative_is_fused(
                head.fused_head_license
            ),
            whole: Object checked by partitive_whole_is_licensed(
                head.determiner_number,
                head.number,
                whole.number
            ),
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        derive possessive_ending = Values::Other;
        form determinative_partitive = head "of" whole;
    }
    construction positional_partitive: NounPhrase {
        element PositionalPartitive {
            position: lex EdgePosition,
            selection: PartitiveSelection,
            whole: Object,
        }
        derive agreement = selection.agreement;
        derive number = selection.number;
        derive onset = Values::Consonant;
        form positional_partitive = "the" lex(position) selection "of" whole;
    }
    construction singular_common_noun_choice: CommonNounChoice {
        element SingularCommonNounChoice { noun: lex Noun, }
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form singular_common_noun_choice = noun(noun);
    }
    construction plural_common_noun_choice: CommonNounChoice {
        element PluralCommonNounChoice { noun: lex Noun, }
        derive noun.number = Values::Plural;
        derive number = Values::Plural;
        form plural_common_noun_choice = noun(noun);
    }
    construction common_noun_choice_list: NounPhrase {
        element CommonNounChoiceList {
            choices: seq CommonNounChoice separated by " or ",
        }
        require len(choices) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form common_noun_choice_list = choices;
    }
    construction fused_color_nominal: Nominal {
        element FusedColorNominal { color: lex Color, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = color.onset;
        derive possessive_ending = color.possessive_ending;
        form fused_color_nominal = lex(color);
    }
    construction indefinite_pronoun_nominal: Nominal {
        element IndefinitePronounNominal { pronoun: lex IndefinitePronoun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = pronoun.onset;
        derive possessive_ending = pronoun.possessive_ending;
        form indefinite_pronoun_nominal = lex(pronoun);
    }
    construction possessive_plural_noun: PossessiveOwner {
        element PossessiveNoun { head: PluralHead, }
        derive number = Values::Plural;
        derive possessive_ending = head.possessive_ending;
        form possessive_plural_noun = head;
    }
    construction possessive_self_reference: PossessiveOwner {
        element PossessiveSelfReference { spelling: identity SelfReferenceSpelling, }
        derive number = Values::Singular;
        derive possessive_ending = spelling.possessive_ending;
        form possessive_self_reference = identity(spelling);
    }
    construction possessive_singular_nominal: PossessiveOwner {
        element PossessiveSingularNominal { nominal: SingularNominal, }
        derive number = Values::Singular;
        derive possessive_ending = nominal.possessive_ending;
        form possessive_singular_nominal = nominal;
    }
    construction possessive_singular_reference: PossessiveOwner {
        element PossessiveSingularReference { reference: UnqualifiedReference, }
        require reference.number is Singular;
        derive number = Values::Singular;
        derive possessive_ending = reference.possessive_ending;
        form possessive_singular_reference = reference;
    }
    construction possessive_plural_reference: PossessiveOwner {
        element PossessivePluralReference { reference: UnqualifiedReference, }
        require reference.number is Plural;
        derive number = Values::Plural;
        derive possessive_ending = reference.possessive_ending;
        form possessive_plural_reference = reference;
    }
    construction possessive: Possessive {
        element PossessiveValue { owner: PossessiveOwner, }
        derive number = owner.number;
        form singular when number is Singular = suffix(owner, "'s");
        form plural_s when all(
            number is Plural,
            owner.possessive_ending is EndsInS
        ) = suffix(owner, "'");
        form plural_other otherwise = suffix(owner, "'s");
    }
    construction positive_power_toughness_counter: CounterKind {
        element PositivePowerToughnessCounter {
            magnitudes: seq PositiveCounterMagnitude separated by "/",
        }
        require len(magnitudes) = 2;
        derive onset = Values::Consonant;
        form positive_power_toughness_counter = magnitudes;
    }
    construction negative_power_toughness_counter: CounterKind {
        element NegativePowerToughnessCounter {
            magnitudes: seq NegativeCounterMagnitude separated by "/",
        }
        require len(magnitudes) = 2;
        derive onset = Values::Consonant;
        form negative_power_toughness_counter = magnitudes;
    }
    construction positive_counter_magnitude: PositiveCounterMagnitude {
        element PositiveCounterMagnitudeValue { amount: Amount, }
        form positive_counter_magnitude = prefix("+", amount);
    }
    construction negative_counter_magnitude: NegativeCounterMagnitude {
        element NegativeCounterMagnitudeValue { amount: Amount, }
        form negative_counter_magnitude = prefix("-", amount);
    }
    construction positive_power_toughness_magnitude: PowerToughnessAdjustmentMagnitude {
        element PositivePowerToughnessMagnitude { amount: Amount, }
        form positive_power_toughness_magnitude = prefix("+", amount);
    }
    construction negative_power_toughness_magnitude: PowerToughnessAdjustmentMagnitude {
        element NegativePowerToughnessMagnitude { amount: Amount, }
        form negative_power_toughness_magnitude = prefix("-", amount);
    }
    construction power_toughness_adjustment: PowerToughnessAdjustment {
        element PowerToughnessAdjustmentValue {
            magnitudes: seq PowerToughnessAdjustmentMagnitude separated by "/",
        }
        require len(magnitudes) = 2;
        form power_toughness_adjustment = magnitudes;
    }
    construction declared_counter: CounterKind {
        element DeclaredCounter { kind: lex DeclaredCounterKind, }
        derive onset = kind.onset;
        form declared_counter = lex(kind);
    }
    construction base_verb_phrase: VerbPhrase {
        element BaseVerbPhrase { frame: BaseVerbFrame, }
        derive agreement = frame.agreement;
        form base_verb_phrase = frame;
    }
    construction pro_verb_predicate: VerbPhrase {
        element ProVerbPredicate { head: lex ProVerbHead, }
        derive agreement = head.agreement;
        form pro_verb_predicate = verb(head);
    }
    construction declared_object_predicative_verb_phrase: VerbPhrase {
        element DeclaredObjectPredicativeVerbPhrase {
            head: lex ObjectPredicativeComplementVerb,
            object: Object,
            complement: PredicativeComplement,
        }
        derive agreement = head.agreement;
        form declared_object_predicative_verb_phrase = verb(head) object complement;
    }
    construction intransitive_predicate: IntransitiveFrame {
        element IntransitivePredicate { head: lex IntransitiveVerb, }
        derive agreement = head.agreement;
        form intransitive_predicate = verb(head);
    }
    construction transitive_predicate: TransitiveFrame {
        element TransitivePredicate { head: lex TransitiveVerb, object: Object, }
        derive agreement = head.agreement;
        form transitive_predicate = verb(head) object;
    }
    construction numerative_predicate: NumerativeFrame {
        element NumerativePredicate { head: lex NumerativeVerb, amount: Amount, }
        derive agreement = head.agreement;
        form numerative_predicate = verb(head) amount;
    }
    construction declared_object_amount_frame: ObjectAmountFrame {
        element DeclaredObjectAmountFrame {
            head: lex ObjectAmountVerb,
            object: Object,
            amount: Amount,
        }
        derive agreement = head.agreement;
        form declared_object_amount_frame = verb(head) object amount;
    }
    construction declared_with_object_frame: WithObjectFrame {
        element DeclaredWithObjectFrame {
            head: lex WithObjectVerb,
            object: Object,
        }
        derive agreement = head.agreement;
        form declared_with_object_frame = verb(head) "with" object;
    }
    construction declared_object_with_object_frame: ObjectWithObjectFrame {
        element DeclaredObjectWithObjectFrame {
            head: lex ObjectWithObjectVerb,
            object: Object,
            complement: Object,
        }
        derive agreement = head.agreement;
        form declared_object_with_object_frame = verb(head) object "with" complement;
    }
    construction declared_object_for_object_frame: ObjectForObjectFrame {
        element DeclaredObjectForObjectFrame {
            head: lex ObjectForObjectVerb,
            object: Object,
            complement: Object,
        }
        derive agreement = head.agreement;
        form declared_object_for_object_frame = verb(head) object "for" complement;
    }
    construction declared_object_into_object_frame: ObjectIntoObjectFrame {
        element DeclaredObjectIntoObjectFrame {
            head: lex ObjectIntoObjectVerb,
            object: Object,
            destination: Object,
        }
        derive agreement = head.agreement;
        form declared_object_into_object_frame = verb(head) object "into" destination;
    }
    construction object_distribution_recipient: DistributionRecipient {
        element ObjectDistributionRecipient { object: Object, }
        form object_distribution_recipient = object;
    }
    construction chosen_distribution_phrase: DistributionPhrase {
        element ChosenDistributionPhrase {
            head: lex AmongObjectVerb,
            recipient: Object,
        }
        derive head.agreement = Values::Bare;
        form chosen_distribution_phrase =
            "divided" "as" "you" verb(head) "among" recipient;
    }
    construction even_distribution_phrase: DistributionPhrase {
        element EvenDistributionPhrase { recipient: DistributionRecipient, }
        form even_distribution_phrase =
            "divided" "evenly" "," "rounded" "down" "," "among" recipient;
    }
    construction modal_passive_subject_gap_relative_clause: ModalPassiveSubjectGapRelativeClause {
        element ModalPassiveSubjectGapRelativeClauseValue {
            auxiliary: AuxiliaryHead,
            predicate: BarePassivePredicate,
        }
        derive predicate.agreement = Values::Bare;
        derive agreement = auxiliary.agreement;
        form modal_passive_subject_gap_relative_clause = "that" auxiliary predicate;
    }
    construction finite_subject_gap_relative_clause: FiniteSubjectGapRelativeClause {
        element FiniteSubjectGapRelativeClauseValue { head: lex IntransitiveVerb, }
        derive agreement = head.agreement;
        form finite_subject_gap_relative_clause = "that" verb(head);
    }
    construction modal_subject_gap_relative_clause: ModalSubjectGapRelativeClause {
        element ModalSubjectGapRelativeClauseValue {
            auxiliary: AuxiliaryHead,
            head: lex IntransitiveVerb,
        }
        derive agreement = auxiliary.agreement;
        derive head.agreement = Values::Bare;
        form modal_subject_gap_relative_clause = "that" auxiliary verb(head);
    }
    construction copular_subject_gap_relative_clause: CopularSubjectGapRelativeClause {
        element CopularSubjectGapRelativeClauseValue {
            copula: lex FiniteCopula,
            complement: PredicativeAdjectiveComplement,
        }
        derive copula.agreement = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Bare,
            Arent => Values::Bare,
            Was => Values::ThirdPersonSingular,
            Were => Values::Bare,
        };
        derive agreement = copula.agreement;
        form copular_subject_gap_relative_clause = "that" lex(copula) complement;
    }
    construction distributed_measure_predicate: VerbPhrase {
        element DistributedMeasurePredicate {
            head: lex DistributedMeasureVerb,
            amount: Amount,
            measure: MassNoun,
            distribution: DistributionPhrase,
            replacement: opt lex DistributionReplacement,
        }
        derive agreement = head.agreement;
        form distributed_measure_predicate =
            verb(head) amount measure distribution lex(replacement);
    }
    construction declared_object_equality_to_predicate: VerbPhrase {
        element DeclaredObjectEqualityToPredicate {
            head: lex ObjectEqualityToVerb,
            object: Object,
            equality: ScalarEquality,
            recipient: Object,
        }
        derive agreement = head.agreement;
        form declared_object_equality_to_predicate =
            verb(head) object equality "to" recipient;
    }
    construction declared_object_to_equality_predicate: VerbPhrase {
        element DeclaredObjectToEqualityPredicate {
            head: lex ObjectToEqualityVerb,
            object: Object,
            recipient: Object,
            equality: ScalarEquality,
        }
        derive agreement = head.agreement;
        form declared_object_to_equality_predicate =
            verb(head) object "to" recipient equality;
    }
    construction declared_object_equality_predicate: VerbPhrase {
        element DeclaredObjectEqualityPredicate {
            head: lex ObjectEqualityVerb,
            object: Object,
            equality: ScalarEquality,
        }
        derive agreement = head.agreement;
        form declared_object_equality_predicate = verb(head) object equality;
    }
    construction mana_phrase: VerbPhrase {
        element ManaVerbPhrase { head: lex ManaPhraseVerb, mana: ManaPhrase, }
        derive agreement = head.agreement;
        form mana_phrase = verb(head) mana;
    }
    construction declared_object_from_predicate: VerbPhrase {
        element DeclaredObjectFromPredicate {
            head: lex ObjectFromVerb,
            object: Object,
            source: FrameComplement,
        }
        derive agreement = head.agreement;
        form declared_object_from_predicate = verb(head) object "from" source;
    }
    construction put_onto: VerbPhrase {
        element PutOnto {
            head: lex ObjectFromOntoResultControlVerb,
            object: Object,
            source: opt SourcePhrase,
            destination: FrameComplement,
            result: opt PredicativeComplement,
            control: opt ControlPhrase,
        }
        derive agreement = head.agreement;
        form put_onto = verb(head) object source "onto" destination result control;
    }
    construction put_on: VerbPhrase {
        element PutOn {
            head: lex ObjectFromOnVerb,
            object: Object,
            source: opt SourcePhrase,
            destination: FrameComplement,
        }
        derive agreement = head.agreement;
        form put_on = verb(head) object source "on" destination;
    }
    construction put_to: VerbPhrase {
        element PutTo {
            head: lex ObjectToVerb,
            object: Object,
            destination: FrameComplement,
        }
        derive agreement = head.agreement;
        form put_to = verb(head) object "to" destination;
    }
    construction return_to: VerbPhrase {
        element ReturnTo {
            head: lex ObjectFromToResultControlVerb,
            object: Object,
            source: opt SourcePhrase,
            destination: FrameComplement,
            result: opt PredicativeComplement,
            control: opt ControlPhrase,
        }
        derive agreement = head.agreement;
        form return_to = verb(head) object source "to" destination result control;
    }
    construction predicative_complement_predicate: VerbPhrase {
        element PredicativeComplementPredicate { head: lex PredicativeComplementVerb, complement: PredicativeComplement, }
        derive agreement = head.agreement;
        form predicative_complement_predicate = verb(head) complement;
    }
    construction declared_with_object_on_predicate: VerbPhrase {
        element DeclaredWithObjectOnPredicate {
            head: lex EnterWithCountersVerb,
            object: Object,
            recipient: FrameComplement,
        }
        derive agreement = head.agreement;
        form declared_with_object_on_predicate =
            verb(head) "with" object "on" recipient;
    }
    construction enter_location: VerbPhrase {
        element EnterLocation {
            head: lex EnterLocationVerb,
            location: Object,
            result: opt PredicativeComplement,
            control: opt ControlPhrase,
        }
        derive agreement = head.agreement;
        form enter_location = verb(head) location result control;
    }
    construction enter_control: VerbPhrase {
        element EnterControl { head: lex EnterControlVerb, control: Object, }
        derive agreement = head.agreement;
        form enter_control = verb(head) "under" control;
    }
    construction look_at: VerbPhrase {
        element LookAt { head: lex LookAtVerb, object: Object, }
        derive agreement = head.agreement;
        form look_at = verb(head) "at" object;
    }
    construction declared_to_object_predicate: VerbPhrase {
        element DeclaredToObjectPredicate {
            head: lex ToObjectVerb,
            object: Object,
            complement: Object,
        }
        derive agreement = head.agreement;
        form declared_to_object_predicate = verb(head) object "to" complement;
    }
    construction declared_for_object_predicate: VerbPhrase {
        element DeclaredForObjectPredicate {
            head: lex ForObjectVerb,
            object: Object,
        }
        derive agreement = head.agreement;
        form declared_for_object_predicate = verb(head) "for" object;
    }
    // A quoted granted ability is a document in its own right: its interior
    // parses with the same grammar as printed rules text (oracle convention,
    // style guide "Quotation marks"; the CR does not describe the quoting).
    construction quoted_block: QuotedBlock {
        element QuotedBlockValue { block: DocumentBlock, }
        form quoted_block = block;
    }
    construction quoted_ability: QuotedAbility {
        element QuotedAbilityValue { block: QuotedBlock, }
        form quoted_ability = sentence_initial(" \"") suffix(block, "\"");
    }
    construction quoted_ability_predicate: VerbPhrase {
        element QuotedAbilityPredicate { head: lex QuotedAbilityVerb, ability: QuotedAbility, }
        derive agreement = head.agreement;
        form quoted_ability_predicate = verb(head) ability;
    }
    construction have_keyword_ability: VerbPhrase {
        element HaveKeywordAbility { head: lex HaveKeywordAbilityVerb, ability: lex KeywordAbility, }
        derive agreement = head.agreement;
        form have_keyword_ability = verb(head) lex(ability);
    }
    construction quote_terminated_statement: AbilityBody {
        element QuoteTerminatedStatement {
            subject: Subject,
            predicate: VerbPhrase,
        }
        require predicate is QuotedAbilityPredicate;
        derive predicate.agreement = subject.agreement;
        form quote_terminated_statement = subject predicate;
    }
    construction get_power_toughness: VerbPhrase {
        element GetPowerToughness {
            head: lex GetPowerToughnessVerb,
            adjustment: PowerToughnessAdjustment,
            duration: opt DurationPhrase,
        }
        derive agreement = head.agreement;
        form get_power_toughness = verb(head) adjustment duration;
    }
    construction have_object_control: VerbPhrase {
        element HaveObjectControl {
            head: lex HaveObjectControlVerb,
            object: Object,
            predicate: VerbPhrase,
        }
        derive agreement = head.agreement;
        derive predicate.agreement = Values::Bare;
        form have_object_control = verb(head) object predicate;
    }
    construction mana_amount: ManaAmount {
        element ManaAmountValue { run: ActivationCostComponent, }
        require run is SymbolRun;
        form mana_amount = run;
    }
    construction and_mana_coordination: ManaCoordination {
        element AndManaCoordination {
            members: seq ManaAmount separated by " and ",
        }
        require len(members) >= 2;
        form and_mana_coordination = members;
    }
    construction or_mana_coordination: ManaCoordination {
        element OrManaCoordination {
            members: seq ManaAmount separated by " or ",
        }
        require len(members) >= 2;
        form or_mana_coordination = members;
    }
    construction and_or_mana_coordination: ManaCoordination {
        element AndOrManaCoordination {
            members: seq ManaAmount separated by " and/or ",
        }
        require len(members) >= 2;
        form and_or_mana_coordination = members;
    }
    construction number: Amount {
        element NumberAmount { number: lex ScalarNumber, }
        form number = lex(number);
    }
    construction variable: Amount {
        element VariableAmount { variable: lex Variable, }
        form variable = lex(variable);
    }
    construction twice_variable_amount: Amount {
        element TwiceVariableAmount { variable: lex Variable, }
        form twice_variable_amount = "twice" lex(variable);
    }
    construction variable_plus_amount: Amount {
        element VariablePlusAmount {
            variable: lex Variable,
            increment: lex ScalarNumber,
        }
        form variable_plus_amount = lex(variable) "plus" lex(increment);
    }
    construction scalar_reference_amount: Amount {
        element ScalarReferenceAmount { reference: ScalarReference, }
        form scalar_reference_amount = reference;
    }
    construction cardinal: CardinalQuantity {
        element CardinalQuantityValue { number: lex CardinalNumber, }
        derive agreement = number.agreement;
        derive cardinality = number.cardinality;
        derive determiner_number = number.determiner_number;
        derive number = number.number;
        form cardinal = lex(number);
    }

    abstract sum KeywordLineItem {
        Bare: BareKeywordLineItem,
        ManaCosted: ManaCostedKeywordLineItem,
        ClauseCosted: ClauseCostedKeywordLineItem,
        ManaClauseCosted: ManaClauseCostedKeywordLineItem,
        Amounted: AmountKeywordLineItem,
        AmountManaCosted: AmountManaCostKeywordLineItem,
        AmountClauseCosted: AmountClauseCostKeywordLineItem,
        AmountManaClauseCosted: AmountManaClauseCostKeywordLineItem,
        Qualified: QualifiedKeywordLineItem,
        QualityManaCosted: QualityManaCostKeywordLineItem,
        QualityClauseCosted: QualityClauseCostKeywordLineItem,
        QualityManaClauseCosted: QualityManaClauseCostKeywordLineItem,
        Subject: SubjectKeywordLineItem,
    }
    abstract sum KeywordQuality {
        Reference: Nominal,
        Coordination: KeywordQualityCoordination,
    }
    abstract sum KeywordSubject {
        Nominal: SingularNominal,
        Coordination: SingularNominalCoordination,
        Relative: KeywordRelativeSubject,
    }
    construction bare_keyword_line_item: BareKeywordLineItem {
        element BareKeywordLineItemValue { keyword: lex BareKeywordAbility, }
        form bare_keyword_line_item = lex(keyword);
    }
    construction mana_costed_keyword_line_item: ManaCostedKeywordLineItem {
        element ManaCostedKeywordLineItemValue {
            keyword: lex CostedKeywordAbility,
            cost: ActivationCostComponent,
        }
        require cost is SymbolRun;
        form mana_costed_keyword_line_item = lex(keyword) cost;
    }
    construction keyword_cost_predicate: KeywordCostPredicate {
        element KeywordCostPredicateValue { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form keyword_cost_predicate = predicate ".";
    }
    construction clause_costed_keyword_line_item: ClauseCostedKeywordLineItem {
        element ClauseCostedKeywordLineItemValue {
            keyword: lex CostedKeywordAbility,
            cost: KeywordCostPredicate,
        }
        form clause_costed_keyword_line_item =
            lex(keyword) sentence_initial("—") cost;
    }
    construction mana_clause_costed_keyword_line_item: ManaClauseCostedKeywordLineItem {
        element ManaClauseCostedKeywordLineItemValue {
            keyword: lex CostedKeywordAbility,
            mana: ActivationCostComponent,
            cost: KeywordCostPredicate,
        }
        require mana is SymbolRun;
        form mana_clause_costed_keyword_line_item = lex(keyword)
            sentence_initial("—") mana sentence_initial(", ") cost;
    }
    construction amount_keyword_line_item: AmountKeywordLineItem {
        element AmountKeywordLineItemValue {
            keyword: lex AmountKeywordAbility,
            amount: Amount,
        }
        form amount_keyword_line_item = lex(keyword) amount;
    }
    construction amount_mana_cost_keyword_line_item: AmountManaCostKeywordLineItem {
        element AmountManaCostKeywordLineItemValue {
            keyword: lex AmountCostKeywordAbility,
            amount: Amount,
            cost: ActivationCostComponent,
        }
        require cost is SymbolRun;
        form amount_mana_cost_keyword_line_item =
            lex(keyword) amount sentence_initial("—") cost;
    }
    construction amount_clause_cost_keyword_line_item: AmountClauseCostKeywordLineItem {
        element AmountClauseCostKeywordLineItemValue {
            keyword: lex AmountCostKeywordAbility,
            amount: Amount,
            cost: KeywordCostPredicate,
        }
        form amount_clause_cost_keyword_line_item = lex(keyword) amount
            sentence_initial("—") cost;
    }
    construction amount_mana_clause_cost_keyword_line_item: AmountManaClauseCostKeywordLineItem {
        element AmountManaClauseCostKeywordLineItemValue {
            keyword: lex AmountCostKeywordAbility,
            amount: Amount,
            mana: ActivationCostComponent,
            cost: KeywordCostPredicate,
        }
        require mana is SymbolRun;
        form amount_mana_clause_cost_keyword_line_item = lex(keyword) amount
            sentence_initial("—") mana sentence_initial(", ") cost;
    }
    construction keyword_quality_coordination: KeywordQualityCoordination {
        element KeywordQualityCoordinationValue {
            members: seq Nominal separated by position {
                pair = " and from ";
                first = ", from ";
                middle = ", from ";
                last = ", and from ";
            },
        }
        require len(members) >= 2;
        form keyword_quality_coordination = members;
    }
    construction qualified_keyword_line_item: QualifiedKeywordLineItem {
        element QualifiedKeywordLineItemValue {
            keyword: lex QualityKeywordAbility,
            quality: KeywordQuality,
        }
        form qualified_keyword_line_item = lex(keyword) quality;
    }
    construction quality_mana_cost_keyword_line_item: QualityManaCostKeywordLineItem {
        element QualityManaCostKeywordLineItemValue {
            keyword: lex QualityCostKeywordAbility,
            quality: KeywordQuality,
            cost: ActivationCostComponent,
        }
        require cost is SymbolRun;
        form quality_mana_cost_keyword_line_item =
            lex(keyword) quality sentence_initial("—") cost;
    }
    construction quality_clause_cost_keyword_line_item: QualityClauseCostKeywordLineItem {
        element QualityClauseCostKeywordLineItemValue {
            keyword: lex QualityCostKeywordAbility,
            quality: KeywordQuality,
            cost: KeywordCostPredicate,
        }
        form quality_clause_cost_keyword_line_item = lex(keyword) quality
            sentence_initial("—") cost;
    }
    construction quality_mana_clause_cost_keyword_line_item: QualityManaClauseCostKeywordLineItem {
        element QualityManaClauseCostKeywordLineItemValue {
            keyword: lex QualityCostKeywordAbility,
            quality: KeywordQuality,
            mana: ActivationCostComponent,
            cost: KeywordCostPredicate,
        }
        require mana is SymbolRun;
        form quality_mana_clause_cost_keyword_line_item = lex(keyword) quality
            sentence_initial("—") mana sentence_initial(", ") cost;
    }
    construction subject_keyword_line_item: SubjectKeywordLineItem {
        element SubjectKeywordLineItemValue {
            keyword: lex SubjectKeywordAbility,
            subject: KeywordSubject,
        }
        form subject_keyword_line_item = lex(keyword) subject;
    }
    construction keyword_relative_subject: KeywordRelativeSubject {
        element KeywordRelativeSubjectValue {
            nominal: SingularNominal,
            clause: ObjectGapRelativeClause,
        }
        form keyword_relative_subject = nominal clause;
    }
    construction keyword_line: KeywordLine {
        element KeywordLineValue {
            items: seq KeywordLineItem separated by ", ",
        }
        require len(items) >= 1;
        form keyword_line = items;
    }
    // A chapter symbol is a keyword ability whose Roman numeral heads a
    // triggered ability [CR#107.15,714.2b]; a combined label means each
    // numeral individually [CR#714.2c].
    abstract sum BlockLabel {
        AbilityWord: AbilityWordLabel,
        Chapter: ChapterLabel,
    }
    construction ability_word_label: BlockLabel {
        element AbilityWordLabel { term: lex AbilityWordTerm, }
        form ability_word_label = lex(term);
    }
    construction chapter_label: BlockLabel {
        element ChapterLabel {
            numerals: seq lex ChapterNumeral separated by ", ",
        }
        require len(numerals) >= 1;
        form chapter_label = lex(numerals);
    }
    construction labelled_ability: LabelledAbility {
        element LabelledAbilityValue {
            label: BlockLabel,
            ability: Ability,
        }
        form labelled_ability = label sentence_initial(" — ") ability;
    }

    // A level symbol is a keyword ability whose band owns the power/toughness
    // printed in its striation [CR#711.2,711.2a,711.2b].
    abstract sum LevelRange {
        Bounded: BoundedLevelRange,
        Open: OpenLevelRange,
    }
    construction bounded_level_range: LevelRange {
        element BoundedLevelRange {
            bounds: seq lex ScalarNumber separated by "-",
        }
        require len(bounds) = 2;
        form bounded_level_range = lex(bounds);
    }
    construction open_level_range: LevelRange {
        element OpenLevelRange { low: lex ScalarNumber, }
        form open_level_range = suffix(lex(low), "+");
    }
    construction level_band: LevelBand {
        element LevelBandValue {
            label: lex LevelBlockLabel,
            range: LevelRange,
            power_toughness: PredicativePowerToughnessComplement,
        }
        form level_band = lex(label) range sentence_initial("\n") power_toughness;
    }

    abstract sum DocumentBlock {
        Ability,
        Labelled: LabelledAbility,
        KeywordLine,
        Level: LevelBand,
    }
    abstract product OracleText {
        blocks: seq DocumentBlock separated by sentence_initial("\n"),
    }

    root Ability { eoi = true; standalone_render = true; }
    root Sentence { punctuation = "."; eoi = false; standalone_render = true; }
    root Possessive { eoi = true; standalone_render = true; }
    root CardinalQuantity { eoi = true; standalone_render = true; }
    root MannerReference { eoi = true; standalone_render = true; }
    root CountReference { eoi = true; standalone_render = true; }
    root ScalarReference { eoi = true; standalone_render = true; }
    root OracleText { eoi = true; standalone_render = true; }
}

fn determinative_is_fused(head: &Determinative, fused_head_license: FusedHeadLicense) -> bool {
    let _ = head;
    fused_head_license == FusedHeadLicense::FusedHead
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "checked-field callbacks receive borrowed lexical values"
)]
fn singular_demonstrative_is_this(demonstrative: &SingularDemonstrative) -> bool {
    *demonstrative == SingularDemonstrative::This
}

fn noun_is_way(noun: &Noun) -> bool {
    matches!(noun, Noun::Lexeme(CommonNoun::Way))
}

fn determinative_is_all(head: &Determinative) -> bool {
    match head {
        Determinative::SingularSimpleDeterminative(det) => matches!(
            det.head,
            DeterminativeHead::Closed(DeterminativeHeadLemma::All)
        ),
        Determinative::PluralSimpleDeterminative(det) => matches!(
            det.head,
            DeterminativeHead::Closed(DeterminativeHeadLemma::All)
        ),
        _ => false,
    }
}

fn determinative_is_plural_all(head: &Determinative, number: Number) -> bool {
    number == Number::Plural && determinative_is_all(head)
}

fn nominal_object_is_not_fused_all(value: &NounPhrase) -> bool {
    !matches!(
        value,
        NounPhrase::FusedDeterminativeReference(FusedDeterminativeReference { head })
            if determinative_is_all(head)
    )
}

fn partitive_whole_is_licensed(
    whole: &Object,
    determiner_number: DeterminerNumber,
    head_number: Number,
    whole_number: Number,
) -> bool {
    let plural_or_mass = whole_number == Number::Plural || object_is_mass_nominal(whole);
    match determiner_number {
        DeterminerNumber::SingularOnly => head_number == Number::Singular && plural_or_mass,
        DeterminerNumber::PluralOnly => head_number == Number::Plural && plural_or_mass,
        DeterminerNumber::Both => head_number == whole_number,
    }
}

fn object_is_mass_nominal(value: &Object) -> bool {
    let Object::ObjectNominal(object) = value else {
        return false;
    };
    let NounPhrase::QualifiedNounPhrase(qualified) = object.value.as_ref() else {
        return false;
    };
    let NumericStage::UnqualifiedNumericStage(numeric) = qualified.reference.as_ref() else {
        return false;
    };
    let LocativeStage::UnqualifiedLocativeStage(locative) = numeric.reference.as_ref() else {
        return false;
    };
    let ControllerStage::UnqualifiedControllerStage(UnqualifiedControllerStage { reference }) =
        locative.reference.as_ref()
    else {
        return false;
    };
    match reference.as_ref() {
        UnqualifiedReference::DeterminedNominal(determined) => {
            nominal_form_for_nominal(&determined.nominal) == NominalForm::MassNoun
        }
        UnqualifiedReference::PossessedMassReference(_)
        | UnqualifiedReference::GenitiveDeterminerMassReference(_)
        | UnqualifiedReference::PluralGenitiveDeterminerMassReference(_) => true,
        _ => false,
    }
}

fn full_coordination_is_independent(coordination: &FullNounPhraseCoordination) -> bool {
    fn independently_realized(reference: &ControllerStage) -> bool {
        let ControllerStage::UnqualifiedControllerStage(UnqualifiedControllerStage { reference }) =
            reference
        else {
            return true;
        };
        !matches!(
            reference.as_ref(),
            UnqualifiedReference::DeterminedNominal(determined)
                if matches!(determined.det(), Determiner::Zero)
        )
    }

    match coordination {
        FullNounPhraseCoordination::FullAndNounPhraseCoordination(coordination) => {
            coordination.members.iter().all(independently_realized)
        }
        FullNounPhraseCoordination::FullOrNounPhraseCoordination(coordination) => {
            coordination.members.iter().all(independently_realized)
        }
        FullNounPhraseCoordination::FullAndOrNounPhraseCoordination(coordination) => {
            coordination.members.iter().all(independently_realized)
        }
    }
}

fn determinative_licenses_singular(
    _head: &DeterminativeHead,
    determiner_number: DeterminerNumber,
) -> bool {
    matches!(
        determiner_number,
        DeterminerNumber::SingularOnly | DeterminerNumber::Both
    )
}

fn determinative_licenses_plural(
    _head: &DeterminativeHead,
    determiner_number: DeterminerNumber,
) -> bool {
    matches!(
        determiner_number,
        DeterminerNumber::PluralOnly | DeterminerNumber::Both
    )
}

fn determiner_licenses_nominal(
    det: Option<&Determinative>,
    determiner_number: Option<DeterminerNumber>,
    nominal_license: Option<NominalLicense>,
    number: Number,
    nominal_form: NominalForm,
) -> bool {
    let Some(det) = det else {
        return number == Number::Plural || nominal_form == NominalForm::MassNoun;
    };
    let Some(determiner_number) = determiner_number else {
        return false;
    };
    let number_is_licensed = match determiner_number {
        DeterminerNumber::SingularOnly => number == Number::Singular,
        DeterminerNumber::PluralOnly => number == Number::Plural,
        DeterminerNumber::Both => true,
    };
    if !number_is_licensed || number_for_determinative(det) != number {
        return false;
    }
    let Some(nominal_license) = nominal_license else {
        return false;
    };
    match nominal_license {
        NominalLicense::AnyNominal => true,
        NominalLicense::CountNominal => nominal_form != NominalForm::MassNoun,
        NominalLicense::BareSingularNoun => nominal_form == NominalForm::BareSingularNoun,
        NominalLicense::MassOrPluralCount => {
            nominal_form == NominalForm::MassNoun || number == Number::Plural
        }
    }
}

fn headed_determiner_licenses_nominal(
    det: &Determinative,
    determiner_number: DeterminerNumber,
    nominal_license: NominalLicense,
    number: Number,
    nominal_form: NominalForm,
) -> bool {
    determiner_licenses_nominal(
        Some(det),
        Some(determiner_number),
        Some(nominal_license),
        number,
        nominal_form,
    )
}

#[cfg(test)]
mod feature_recipe_tests;
