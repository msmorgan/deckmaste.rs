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
    vocab DamageKind { Ordinary = "damage", Combat = "combat damage", }
    vocab FaceOrientation { FaceUp = "face up", }
    vocab RequirementFrequency { EachCombat = "each combat", }
    vocab FlexibleManaKind { Color = "color", Type = "type", }
    vocab StepModifier { Untap = "untap", }
    vocab MassCommonNoun { Control = "control", Damage = "damage", }
    vocab ObjectOrder { Any = "any", Random = "a random", }
    vocab TemporalUnit { Turn = "turn", Step = "step", }
    vocab TemporalOrder { Next = "next", }
    vocab TemporalBoundary { Beginning = "beginning", End = "end", }
    vocab TemporalRelation { Before = "before", After = "after", }
    vocab ComparativeQuantifier { Fewer = "fewer", More = "more", }
    vocab ScalarDegree { Equal = "equal", Lesser = "lesser", Greater = "greater", }
    vocab AttributiveAdjective {
        feature ModifierLicense = Unrestricted;
        Additional = "additional",
        FaceDown = "face-down",
        Maximum = "maximum",
        Other = "other",
        Target = "target" { feature ModifierLicense = LocalDeterminer; },
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
    vocab BareLocativeNoun { Exile = "exile", Hand = "hand", }
    vocab CostComparisonDirection { More = "more", Less = "less", }
    vocab DistributionReplacement { Instead = "instead", }
    vocab CounterfactualAbility { Flash = "flash", Hexproof = "hexproof", }
    vocab CounterfactualNegativeAuxiliary { Didnt = "didn't", }
    vocab CounterfactualPastPossession { Had = "had", }
    vocab AtBoundary { Beginning = "the beginning of", End = "end of", }
    vocab TriggerMarker { When = "when", Whenever = "whenever", }
    vocab TurnOwnerPostmodifier {
        YourTurn = "on your turn",
        EachOpponentsTurn = "on each opponent's turn",
    }
    vocab TurnPart {
        Turn = "turn",
        BeginningPhase = "beginning phase",
        FirstMainPhase = "first main phase",
        SecondMainPhase = "second main phase",
        PrecombatMainPhase = "precombat main phase",
        PostcombatMainPhase = "postcombat main phase",
        MainPhase = "main phase",
        Combat = "combat",
        CombatPhase = "combat phase",
        EndingPhase = "ending phase",
        UntapStep = "untap step",
        Upkeep = "upkeep",
        DrawStep = "draw step",
        DeclareAttackersStep = "declare attackers step",
        DeclareBlockersStep = "declare blockers step",
        CombatDamageStep = "combat damage step",
        EndStep = "end step",
        CleanupStep = "cleanup step",
    }
    vocab TurnSpecifier {
        Your = "your",
        Each = "each",
        EachPlayer = "each player's",
        EachOpponent = "each opponent's",
        EachOfYour = "each of your",
        The = "the",
        TheNext = "the next",
    }
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
        Blocked = "blocked",
        Blocking = "blocking",
        Enchanted = "enchanted",
        Equipped = "equipped",
        Tapped = "tapped",
        Untapped = "untapped",
    }
    vocab SingularDemonstrative { This = "this", That = "that", }
    vocab ControllerNoun { Opponent = "opponent", Player = "player", }
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
    }
    vocab ModalChooser {
        You = "choose",
        Opponent = "an opponent chooses",
    }
    vocab ModalChoiceBounds {
        ExactlyOne = "one",
        ExactlyTwo = "two",
        OneToTwo = "one or both",
        OneOrMore = "one or more",
        ZeroToOne = "up to one",
    }
    vocab MonocoloredHybridColor {
        White = "W",
        Blue = "U",
        Black = "B",
        Red = "R",
        Green = "G",
    }
    vocab ScalarCharacteristic { Power = "power", Toughness = "toughness", }
    vocab CounterName {
        Charge = "charge",
        Lore = "lore",
        Loyalty = "loyalty",
        Oil = "oil",
        Spore = "spore",
        Stun = "stun",
        Time = "time",
        Verse = "verse",
    }
    vocab DieShape { SixSided = "six-sided", }
    vocab EdgePosition { Top = "top", Bottom = "bottom", }
    vocab NonCommonNoun { Token = "token", }
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
        feature Compoundability = Compoundable;
        Ability = "ability" {
            Plural = "abilities",
        },
        Attacker = "attacker",
        Battlefield = "battlefield",
        Blocker = "blocker",
        Card = "card",
        Choice = "choice",
        Coin = "coin",
        Color = "color",
        Combat = "combat",
        Copy = "copy" {
            Plural = "copies",
        },
        Counter = "counter",
        Damage = "damage",
        Death = "death",
        Draw = "draw",
        Exile = "exile",
        Graveyard = "graveyard",
        Hand = "hand",
        Library = "library" {
            Plural = "libraries",
        },
        Name = "name",
        Number = "number",
        Controller = "controller",
        Opponent = "opponent",
        Owner = "owner",
        Permanent = "permanent",
        Phase = "phase",
        Player = "player",
        Power = "power",
        Rest = "rest",
        Source = "source",
        Size = "size",
        Spell = "spell",
        Stack = "stack",
        Step = "step",
        Target = "target" { feature Compoundability = NonCompoundable; },
        Tax = "tax" {
            Plural = "taxes",
        },
        Token = "token",
        Toughness = "toughness" {
            Plural = "toughnesses",
        },
        Turn = "turn",
        Type = "type",
        Die = "die" {
            Plural = "dice",
        },
    }
    lexeme VerbLexeme using EnglishVerb {
        May = "may" { ThirdPersonSingular = "may", },
        Can = "can" { ThirdPersonSingular = "can", },
        Cant = "can't" { ThirdPersonSingular = "can't", },
        Must = "must" { ThirdPersonSingular = "must", },
        Didnt = "didn't" { ThirdPersonSingular = "didn't", },
        Would = "would" { ThirdPersonSingular = "would", },
        Add = "add",
        Choose = "choose",
        Deal = "deal",
        Draw = "draw",
        Enter = "enter",
        Gain = "gain",
        Get = "get" {
            ThirdPersonSingular = "gets",
        },
        Lose = "lose",
        Pay = "pay",
        Prevent = "prevent",
        Put = "put",
        Remove = "remove",
        Roll = "roll",
        Have = "have" {
            ThirdPersonSingular = "has",
        },
        Look = "look",
        Leave = "leave",
        Control = "control",
        Own = "own",
        Return = "return",
        Cause = "cause",
        Become = "become",
        Cost = "cost",
        Do = "do" {
            ThirdPersonSingular = "does",
        },
        Be = "be" {
            Bare = "are",
            ThirdPersonSingular = "is",
        },
    }
    lexeme CoreIntransitiveVerb using EnglishVerb {
        Attack = "attack",
        Block = "block",
        Cycle = "cycle",
        Die = "die" {
            ThirdPersonSingular = "dies",
        },
        Enter = "enter",
        Leave = "leave",
    }
    lexeme CoreTransitiveVerb using EnglishVerb {
        Attack = "attack",
        Block = "block",
        Choose = "choose",
        Control = "control",
        Copy = "copy" {
            ThirdPersonSingular = "copies",
        },
        Draw = "draw",
        Flip = "flip",
        Have = "have" {
            ThirdPersonSingular = "has",
        },
        Lose = "lose",
        Own = "own",
        Prevent = "prevent",
        Skip = "skip",
        Unattach = "unattach" {
            ThirdPersonSingular = "unattaches",
        },
    }
    lexeme CoreNumerativeVerb using EnglishVerb { Draw = "draw", }
    lexeme DamageParticipleLexeme using EnglishParticiple {
        Deal = "deal" { Participle = "dealt", },
    }
    lexeme MovementParticipleLexeme using EnglishParticiple {
        Put = "put" { Participle = "put", },
    }
    lexeme OrientationParticipleLexeme using EnglishParticiple { Turn = "turn", }
    lexeme CoreTransitiveParticipleLexeme using EnglishParticiple {
        Attack = "attack",
        Choose = "choose" { Participle = "chosen", },
        Exile = "exile" { Participle = "exiled", },
        Prevent = "prevent",
        Draw = "draw" { Participle = "drawn", },
    }
    lexeme CoreObjectOnParticipleLexeme using EnglishParticiple {
        Put = "put" { Participle = "put", },
    }

    codec IntransitiveVerb {
        generate declaration_verb {
            closed = CoreIntransitiveVerb;
            position = Verb;
            tail = [];
            feature = Agreement;
        }
    }
    codec TransitiveVerb {
        generate declaration_verb {
            closed = CoreTransitiveVerb;
            position = Verb;
            tail = [ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec NumerativeVerb {
        generate declaration_verb {
            closed = CoreNumerativeVerb;
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
    codec DealAmountDamageVerb { generate declaration_verb { position = Verb; tail = [Amount, "damage", ToPhrase]; feature = Agreement; } }
    codec DealDistributedDamageVerb { generate declaration_verb { position = Verb; tail = [DistributedDamageAmount, "damage", DamageDistribution, DistributionReplacement?]; feature = Agreement; } }
    codec DealDamageKindVerb { generate declaration_verb { position = Verb; tail = [DamageKind, ToPhrase?]; feature = Agreement; } }
    codec DealDamageEqualToVerb { generate declaration_verb { position = Verb; tail = ["damage", ScalarEquality, ToPhrase]; feature = Agreement; } }
    codec DealDamageToEqualToVerb { generate declaration_verb { position = Verb; tail = ["damage", ToPhrase, ScalarEquality]; feature = Agreement; } }
    codec LifeAmountVerb { generate declaration_verb { position = Verb; tail = [Amount, "life"]; feature = Agreement; } }
    codec GainLifeVerb { generate declaration_verb { position = Verb; tail = ["life"]; feature = Agreement; } }
    codec LifeEqualityVerb { generate declaration_verb { position = Verb; tail = ["life", ScalarEquality]; feature = Agreement; } }
    codec DamageParticipleHead {
        generate declaration_verb {
            closed = DamageParticipleLexeme;
            position = Verb;
            tail = [Amount];
            feature = Participle;
        }
    }
    codec MovementParticipleHead {
        generate declaration_verb {
            closed = MovementParticipleLexeme;
            position = Verb;
            tail = [moved: ObjectNounPhrase, "into", destination: ObjectNounPhrase];
            feature = Participle;
        }
    }
    codec OrientationParticipleHead {
        generate declaration_verb {
            closed = OrientationParticipleLexeme;
            position = Verb;
            tail = ["face", "up"];
            feature = Participle;
        }
    }
    codec DeclaredTransitiveParticipleHead {
        generate declaration_verb {
            closed = CoreTransitiveParticipleLexeme;
            position = Verb;
            tail = [ObjectNounPhrase];
            feature = Participle;
        }
    }
    codec DeclaredToObjectParticipleHead {
        generate declaration_verb {
            closed = DamageParticipleLexeme;
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
            closed = CoreObjectOnParticipleLexeme;
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                "on",
                complement: ObjectNounPhrase,
            ];
            feature = Participle;
        }
    }

    codec TypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Type];
            feature = Number;
        }
    }
    codec ArtifactSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Artifact)];
            feature = Number;
        }
    }
    codec BattleSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Battle)];
            feature = Number;
        }
    }
    codec CreatureSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Creature)];
            feature = Number;
        }
    }
    codec EnchantmentSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Enchantment)];
            feature = Number;
        }
    }
    codec LandSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Land)];
            feature = Number;
        }
    }
    codec PlaneswalkerSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Planeswalker)];
            feature = Number;
        }
    }
    codec SpellSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Spell)];
            feature = Number;
        }
    }
    codec KeywordAbility {
        generate declaration_term {
            position = FixedKeyword;
            kinds = [KeywordAbility];
        }
    }
    codec DeclaredCounterKind {
        generate declaration_term {
            position = FixedTerm;
            kinds = [CounterKind];
        }
    }
    codec Designation {
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
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "the"; }];
                },
                ProximalDemonstrative {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "this"; phrase_number = Singular; }];
                },
                DistalDemonstrative {
                    number_license = Both;
                    nominal_license = CountNominal;
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
                    number_license = PluralOnly;
                    nominal_license = CountNominal;
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
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "no"; }];
                },
                Any {
                    number_license = Both;
                    nominal_license = CountNominal;
                    fused_head_license = FusedHead;
                    realizations = [{ surface = "any"; }];
                },
                Target {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "target"; }];
                },
            ];
            kinds = [KeywordAbility];
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
    abstract sum CounterfactualFiniteClause {
        Status: CounterfactualStatusClause,
        NegativeAbility: CounterfactualNegativeAbilityClause,
        PastAbility: CounterfactualPastAbilityClause,
    }
    abstract sum AsThoughPredicate {
        Intransitive: IntransitiveAsThoughPredicate,
        Transitive: TransitiveAsThoughPredicate,
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
    abstract sum ManaCostReference {
        ThisSpell: ThisSpellManaCost,
        Definite: DefiniteManaCost,
        SingularPronoun: SingularPronounManaCost,
        PluralPronoun: PluralPronounManaCosts,
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
    abstract sum TemporalEndpoint {
        Possessed: PossessedTemporalEndpoint,
        Definite: DefiniteTemporalEndpoint,
        Boundary: BoundaryTemporalEndpoint,
        BareBoundary: BareBoundaryTemporalEndpoint,
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
        ChangeState: ChangeStatePredicate,
        BarePassive: BarePassivePredicate,
        FinitePassive: FinitePassivePredicate,
        Auxiliary: AuxiliaryPredicate,
        ForAdjunct: ForAdjunctPredicate,
        DuringAdjunct: DuringAdjunctPredicate,
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
        RatherThanManaCost: RatherThanManaCostPredicate,
        WithoutPayingManaCost: WithoutPayingManaCostPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum BareCoordinatedPredicate {
        Atomic: VerbPhrase,
        BareCopular: BareCopularPredicate,
        ChangeState: ChangeStatePredicate,
        BarePassive: BarePassivePredicate,
        ForAdjunct: ForAdjunctPredicate,
        DuringAdjunct: DuringAdjunctPredicate,
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
        RatherThanManaCost: RatherThanManaCostPredicate,
        WithoutPayingManaCost: WithoutPayingManaCostPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum BarePredicate {
        Atomic: VerbPhrase,
        Coordination: BarePredicateCoordination,
        ThenSequence: BareThenPredicateSequence,
        BareCopular: BareCopularPredicate,
        ChangeState: ChangeStatePredicate,
        BarePassive: BarePassivePredicate,
        ForAdjunct: ForAdjunctPredicate,
        DuringAdjunct: DuringAdjunctPredicate,
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
        RatherThanManaCost: RatherThanManaCostPredicate,
        WithoutPayingManaCost: WithoutPayingManaCostPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum Predicate {
        Atomic: VerbPhrase,
        Coordination: PredicateCoordination,
        ThenSequence: ThenPredicateSequence,
        BareCopular: BareCopularPredicate,
        FiniteCopular: FiniteCopularPredicate,
        ChangeState: ChangeStatePredicate,
        BarePassive: BarePassivePredicate,
        FinitePassive: FinitePassivePredicate,
        Auxiliary: AuxiliaryPredicate,
        ForAdjunct: ForAdjunctPredicate,
        DuringAdjunct: DuringAdjunctPredicate,
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
        RatherThanManaCost: RatherThanManaCostPredicate,
        WithoutPayingManaCost: WithoutPayingManaCostPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum Clause {
        Finite: FiniteClause,
        Coordination: ClauseCoordination,
        Copular: CopularClause,
        Passive: PassiveFiniteClause,
        PostposedWhile: PostposedWhileClause,
        PostposedForAsLongAs: PostposedForAsLongAsClause,
    }
    abstract sum CoordinatedClause {
        Finite: FiniteClause,
        Copular: CopularClause,
        Passive: PassiveFiniteClause,
    }
    abstract sum PredicativeComplement {
        Adjective: PredicativeAdjectiveComplement,
        Color: PredicativeColorComplement,
        Nominal: PredicativeNominalComplement,
        Status: PredicativeStatus,
        Ability: PredicativeAbilityComplement,
        Orientation: PredicativeFaceOrientation,
        PowerToughness: PredicativePowerToughnessComplement,
        Scalar: PredicativeScalarComplement,
    }
    abstract sum PredicativeStatus {
        Plain: PredicativeStatusComplement,
        BlockedBy: BlockedByStatusComplement,
        BlockedExceptBy: BlockedExceptByStatusComplement,
    }
    abstract sum PassivePredicate {
        Damage: PassiveDamagePredicate,
        Movement: PassiveMovementPredicate,
        Orientation: PassiveOrientationPredicate,
        DeclaredTransitive: DeclaredTransitivePassivePredicate,
        DeclaredTransitiveFrom: DeclaredTransitivePassiveFromPredicate,
        DeclaredToObject: DeclaredToObjectPassivePredicate,
    }
    abstract sum StateDurationBase {
        BarePassive: BarePassivePredicate,
    }
    abstract sum DistributedDamageAmount {
        Scalar: ScalarDistributedDamageAmount,
        TwiceVariable: TwiceVariableAmount,
        VariablePlus: VariablePlusAmount,
    }
    abstract sum DamageDistribution {
        AsYouChoose: ChosenDamageDistribution,
        Evenly: EvenDamageDistribution,
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
        PreposedFor,
        PreposedForPredicate,
        PreposedWhile,
        PreposedWhilePredicate,
        PreposedDuring,
        PreposedDuringPredicate,
        PreposedUntil,
        PreposedUntilPredicate,
        PreposedDuration,
        PreposedDurationPredicate,
        PreposedTemporalRelation,
        ThenSequence,
        AdditionalCost,
    }
    abstract sum ConditionClause { FiniteCondition, }
    construction finite_condition: FiniteCondition {
        element FiniteConditionValue { clause: Clause, }
        form finite_condition = "if" clause ",";
    }
    construction among_phrase: AmongPhrase {
        element AmongPhraseValue { domain: NounPhrase, }
        form among_phrase = "among" domain;
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
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form modal_mode = sentence_initial("• ") sentences;
    }
    construction plain_modal: AbilityBody {
        element PlainModal {
            chooser: lex ModalChooser,
            bounds: lex ModalChoiceBounds,
            modes: seq ModalMode separated by sentence_initial("\n"),
        }
        require len(modes) >= 2;
        form plain_modal =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
    }
    construction finite: TriggerPrefix {
        element Finite {
            marker: lex TriggerMarker,
            clause: Clause,
        }
        form finite = lex(marker) clause;
    }
    construction temporal: TriggerPrefix {
        element Temporal { phrase: AtPhrase, }
        form temporal = "at" phrase;
    }
    construction at_phrase: AtPhrase {
        element AtPhraseValue {
            boundary: lex AtBoundary,
            specifier: opt lex TurnSpecifier,
            part: lex TurnPart,
            postmodifier: opt lex TurnOwnerPostmodifier,
        }
        require any(
            all(
                boundary is End,
                specifier.is_none(),
                part is Combat,
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier is EachOfYour,
                part in [
                    FirstMainPhase,
                    SecondMainPhase,
                    PrecombatMainPhase,
                    PostcombatMainPhase,
                    MainPhase
                ],
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier.is_none(),
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier in [Your, Each, EachPlayer, EachOpponent, The, TheNext],
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier.is_none(),
                part is Combat,
                postmodifier.is_some()
            ),
            all(
                boundary is Beginning,
                specifier in [Your, Each, EachPlayer, EachOpponent, The, TheNext],
                part is Combat,
                postmodifier.is_some()
            )
        );
        form plural_main_phase when specifier is EachOfYour =
            lex(boundary) lex(specifier) suffix(lex(part), "s") lex(postmodifier);
        form singular otherwise =
            lex(boundary) lex(specifier) lex(part) lex(postmodifier);
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
    construction preposed_for: ClauseAttachment {
        element PreposedFor { basis: ForPhrase, body: Clause, }
        form preposed_for = basis "," body;
    }
    construction preposed_for_predicate: ClauseAttachment {
        element PreposedForPredicate { basis: ForPhrase, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_for_predicate = basis "," body;
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
    construction preposed_during: ClauseAttachment {
        element PreposedDuring { timing: DuringPhrase, body: Clause, }
        form preposed_during = timing "," body;
    }
    construction preposed_during_predicate: ClauseAttachment {
        element PreposedDuringPredicate {
            timing: DuringPhrase,
            body: Predicate,
        }
        derive body.agreement = Values::Bare;
        form preposed_during_predicate = timing "," body;
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
    construction temporal_relation_phrase: TemporalRelationPhrase {
        element TemporalRelationPhraseValue {
            relation: lex TemporalRelation,
            complement: Object,
        }
        form temporal_relation_phrase = lex(relation) complement;
    }
    construction preposed_temporal_relation: ClauseAttachment {
        element PreposedTemporalRelation {
            adjunct: TemporalRelationPhrase,
            body: Clause,
        }
        form preposed_temporal_relation = adjunct "," body;
    }
    construction fixed_duration_phrase: FixedDurationPhrase {
        element FixedDurationPhraseValue { unit: lex TemporalUnit, }
        form fixed_duration_phrase = "this" lex(unit);
    }
    construction possessed_temporal_endpoint: PossessedTemporalEndpoint {
        element PossessedTemporalEndpointValue {
            possessor: lex PossessiveDeterminerPronoun,
            order: opt lex TemporalOrder,
            part: lex TurnPart,
        }
        form possessed_temporal_endpoint = lex(possessor) lex(order) lex(part);
    }
    construction definite_temporal_endpoint: DefiniteTemporalEndpoint {
        element DefiniteTemporalEndpointValue {
            order: opt lex TemporalOrder,
            part: lex TurnPart,
        }
        form definite_temporal_endpoint = "the" lex(order) lex(part);
    }
    construction boundary_temporal_endpoint: BoundaryTemporalEndpoint {
        element BoundaryTemporalEndpointValue {
            boundary: lex TemporalBoundary,
            endpoint: PossessedTemporalEndpoint,
        }
        form boundary_temporal_endpoint = "the" lex(boundary) "of" endpoint;
    }
    construction bare_boundary_temporal_endpoint: BareBoundaryTemporalEndpoint {
        element BareBoundaryTemporalEndpointValue {
            boundary: lex TemporalBoundary,
            unit: lex TemporalUnit,
        }
        form bare_boundary_temporal_endpoint = lex(boundary) "of" lex(unit);
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
            action: InfinitiveComplement,
            body: AdditionalCostBody,
        }
        form additional_cost = "as" "an" "additional" "cost" action "," body;
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
    construction predicative_nominal: PredicativeNominalComplement {
        element PredicativeNominalValue { value: NounPhrase, }
        form predicative_nominal = value;
    }
    construction predicative_status: PredicativeStatusComplement {
        element PredicativeStatusValue { status: lex Status, }
        form predicative_status = lex(status);
    }
    construction blocked_by_status: BlockedByStatusComplement {
        element BlockedByStatusValue { agent: Object, }
        form blocked_by_status = "blocked" "by" agent;
    }
    construction blocked_except_by_status: BlockedExceptByStatusComplement {
        element BlockedExceptByStatusValue { agent: Object, }
        form blocked_except_by_status = "blocked" "except" "by" agent;
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
    construction change_state_predicate: ChangeStatePredicate {
        element ChangeStatePredicateValue { complement: PredicativeComplement, }
        derive agreement = verb.agreement;
        form change_state_predicate = verb(VerbLexeme::Become) complement;
    }
    construction passive_damage_predicate: PassiveDamagePredicate {
        element PassiveDamagePredicateValue {
            head: lex DamageParticipleHead,
            kind: lex DamageKind,
        }
        form passive_damage_predicate = verb(head) lex(kind);
    }
    construction passive_movement_predicate: PassiveMovementPredicate {
        element PassiveMovementPredicateValue {
            head: lex MovementParticipleHead,
            destination: IntoPhrase,
            source: FromPhrase,
        }
        form passive_movement_predicate = verb(head) destination source;
    }
    construction passive_orientation_predicate: PassiveOrientationPredicate {
        element PassiveOrientationPredicateValue {
            head: lex OrientationParticipleHead,
            orientation: lex FaceOrientation,
        }
        form passive_orientation_predicate = verb(head) lex(orientation);
    }
    construction declared_transitive_passive_predicate: DeclaredTransitivePassivePredicate {
        element DeclaredTransitivePassivePredicateValue {
            head: lex DeclaredTransitiveParticipleHead,
        }
        form declared_transitive_passive_predicate = verb(head);
    }
    construction declared_transitive_passive_from_predicate: DeclaredTransitivePassiveFromPredicate {
        element DeclaredTransitivePassiveFromPredicateValue {
            head: lex DeclaredTransitiveParticipleHead,
            source: FromPhrase,
        }
        form declared_transitive_passive_from_predicate = verb(head) source;
    }
    construction declared_to_object_passive_predicate: DeclaredToObjectPassivePredicate {
        element DeclaredToObjectPassivePredicateValue {
            head: lex DeclaredToObjectParticipleHead,
            complement: ToPhrase,
        }
        form declared_to_object_passive_predicate = verb(head) complement;
    }
    construction bare_passive_predicate: BarePassivePredicate {
        element BarePassivePredicateValue {
            copula: lex BareCopula,
            predicate: PassivePredicate,
        }
        derive agreement = Values::Bare;
        form bare_passive_predicate = lex(copula) predicate;
    }
    construction may_auxiliary: AuxiliaryHead {
        element MayAuxiliary {}
        derive agreement = verb.agreement;
        form may_auxiliary = verb(VerbLexeme::May);
    }
    construction can_auxiliary: AuxiliaryHead {
        element CanAuxiliary {}
        derive agreement = verb.agreement;
        form can_auxiliary = verb(VerbLexeme::Can);
    }
    construction cant_auxiliary: AuxiliaryHead {
        element CantAuxiliary {}
        derive agreement = verb.agreement;
        form cant_auxiliary = verb(VerbLexeme::Cant);
    }
    construction must_auxiliary: AuxiliaryHead {
        element MustAuxiliary {}
        derive agreement = verb.agreement;
        form must_auxiliary = verb(VerbLexeme::Must);
    }
    construction didnt_auxiliary: AuxiliaryHead {
        element DidntAuxiliary {}
        derive agreement = verb.agreement;
        form didnt_auxiliary = verb(VerbLexeme::Didnt);
    }
    construction would_auxiliary: AuxiliaryHead {
        element WouldAuxiliary {}
        derive agreement = verb.agreement;
        form would_auxiliary = verb(VerbLexeme::Would);
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
            object: Object,
            complement: VerbPhrase,
        }
        derive agreement = verb.agreement;
        derive complement.agreement = Values::Bare;
        form object_infinitive_predicate = verb(VerbLexeme::Cause) object "to" complement;
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
        element ChooseInfinitivePredicate { complement: InfinitiveComplement, }
        derive agreement = verb.agreement;
        form choose_infinitive_predicate = verb(VerbLexeme::Choose) complement;
    }
    construction requirement_predicate: RequirementPredicate {
        element RequirementPredicateValue {
            head: lex IntransitiveVerb,
            frequency: lex RequirementFrequency,
        }
        derive agreement = head.agreement;
        form requirement_predicate = verb(head) lex(frequency) "if" "able";
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
    construction intransitive_as_though_predicate: IntransitiveAsThoughPredicate {
        element IntransitiveAsThoughPredicateValue {
            head: lex IntransitiveVerb,
            condition: CounterfactualFiniteClause,
        }
        derive agreement = head.agreement;
        form intransitive_as_though_predicate = verb(head) "as" "though" condition;
    }
    construction transitive_as_though_predicate: TransitiveAsThoughPredicate {
        element TransitiveAsThoughPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            condition: CounterfactualFiniteClause,
        }
        derive agreement = head.agreement;
        form transitive_as_though_predicate = verb(head) object "as" "though" condition;
    }
    construction ordered_predicate: OrderedPredicate {
        element OrderedPredicateValue {
            object: Object,
            source: opt FromPhrase,
            destination: OnPhrase,
            order: lex ObjectOrder,
        }
        derive agreement = verb.agreement;
        form ordered_predicate =
            verb(VerbLexeme::Put) object source destination "in" lex(order) "order";
    }
    construction counterfactual_status_clause: CounterfactualStatusClause {
        element CounterfactualStatusClauseValue {
            subject: Subject,
            copula: lex FiniteCopula,
            status: lex Status,
        }
        derive copula.agreement = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Bare,
            Arent => Values::Bare,
            Was => Values::ThirdPersonSingular,
            Were => Values::Bare,
        };
        derive subject.agreement = copula.agreement;
        form counterfactual_status_clause = subject lex(copula) lex(status);
    }
    construction counterfactual_negative_ability_clause: CounterfactualNegativeAbilityClause {
        element CounterfactualNegativeAbilityClauseValue {
            subject: Subject,
            auxiliary: lex CounterfactualNegativeAuxiliary,
            ability: lex CounterfactualAbility,
        }
        derive verb.agreement = Values::Bare;
        form counterfactual_negative_ability_clause =
            subject lex(auxiliary) verb(VerbLexeme::Have) lex(ability);
    }
    construction counterfactual_past_ability_clause: CounterfactualPastAbilityClause {
        element CounterfactualPastAbilityClauseValue {
            subject: Subject,
            possession: lex CounterfactualPastPossession,
            ability: lex CounterfactualAbility,
        }
        form counterfactual_past_ability_clause = subject lex(possession) lex(ability);
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
    construction for_phrase: ForPhrase {
        element ForPhraseValue { complement: Object, }
        form for_phrase = "for" complement;
    }
    construction during_phrase: DuringPhrase {
        element DuringPhraseValue { complement: Object, }
        form during_phrase = "during" complement;
    }
    construction for_adjunct_predicate: ForAdjunctPredicate {
        element ForAdjunctPredicateValue {
            predicate: VerbPhrase,
            adjunct: ForPhrase,
        }
        derive agreement = predicate.agreement;
        form for_adjunct_predicate = predicate adjunct;
    }
    construction during_adjunct_predicate: DuringAdjunctPredicate {
        element DuringAdjunctPredicateValue {
            predicate: VerbPhrase,
            adjunct: DuringPhrase,
        }
        derive agreement = predicate.agreement;
        form during_adjunct_predicate = predicate adjunct;
    }
    construction instead_predicate: InsteadPredicate {
        element InsteadPredicateValue { predicate: VerbPhrase, }
        derive agreement = predicate.agreement;
        form instead_predicate = predicate "instead";
    }
    construction manner_predicate: MannerPredicate {
        element MannerPredicateValue {
            predicate: BaseVerbFrame,
            manner: MannerReference,
        }
        derive agreement = predicate.agreement;
        form manner_predicate = predicate manner;
    }
    construction this_spell_mana_cost: ManaCostReference {
        element ThisSpellManaCost {}
        form this_spell_mana_cost = "this" "spell's" "mana" "cost";
    }
    construction definite_mana_cost: ManaCostReference {
        element DefiniteManaCost {}
        form definite_mana_cost = "the" "mana" "cost";
    }
    construction singular_pronoun_mana_cost: ManaCostReference {
        element SingularPronounManaCost {}
        form singular_pronoun_mana_cost = "its" "mana" "cost";
    }
    construction plural_pronoun_mana_costs: ManaCostReference {
        element PluralPronounManaCosts {}
        form plural_pronoun_mana_costs = "their" "mana" "costs";
    }
    construction rather_than_mana_cost_predicate: RatherThanManaCostPredicate {
        element RatherThanManaCostPredicateValue {
            action: VerbPhrase,
            reference: ManaCostReference,
        }
        derive action.agreement = Values::Bare;
        derive agreement = action.agreement;
        derive verb.agreement = Values::Bare;
        form rather_than_mana_cost_predicate =
            action "rather" "than" verb(VerbLexeme::Pay) reference;
    }
    construction without_paying_mana_cost_predicate: WithoutPayingManaCostPredicate {
        element WithoutPayingManaCostPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            reference: ManaCostReference,
        }
        derive agreement = head.agreement;
        form without_paying_mana_cost_predicate =
            verb(head) object "without" "paying" reference;
    }
    construction controlled_cost_action: ControlledCostAction {
        element ControlledCostActionValue { head: lex TransitiveVerb, }
        derive head.agreement = Values::Bare;
        form controlled_cost_action = "to" verb(head);
    }
    construction cost_comparison_predicate: CostComparisonPredicate {
        element CostComparisonPredicateValue {
            amount: ManaAmount,
            direction: lex CostComparisonDirection,
            action: ControlledCostAction,
            basis: opt ForEachCostBasis,
        }
        derive agreement = verb.agreement;
        form cost_comparison_predicate =
            verb(VerbLexeme::Cost) amount lex(direction) action basis;
    }
    construction for_each_cost_basis: ForEachCostBasis {
        element ForEachCostBasisValue { object: Object, }
        form for_each_cost_basis = "for" object;
    }
    construction restriction_turn: RestrictionTurn {
        element RestrictionTurnValue {
            specifier: opt lex TurnSpecifier,
            part: lex TurnPart,
        }
        form restriction_turn = lex(specifier) lex(part);
    }
    construction possessed_next_restriction_turn: RestrictionTurn {
        element PossessedNextRestrictionTurn {
            possessor: Possessive,
            part: lex TurnPart,
        }
        form possessed_next_restriction_turn = possessor "next" lex(part);
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
            relation: lex TemporalRelation,
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
    construction copular_clause: CopularClause {
        element CopularClauseValue {
            subject: Subject,
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
        derive subject.agreement = copula.agreement;
        form copular_clause = subject lex(copula) complement;
    }
    construction passive_finite_clause: PassiveFiniteClause {
        element PassiveFiniteClauseValue {
            subject: Subject,
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
        derive subject.agreement = copula.agreement;
        form passive_finite_clause = subject lex(copula) predicate;
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
            complement: OnPhrase,
        }
        form contracted_perfect_object_on_clause =
            lex(subject) verb(head) object complement;
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
            domain: opt AmongPhrase,
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
        element NominalObject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form object_nominal = value;
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
    construction common_singular_head: SingularHead {
        element CommonSingularHead { noun: lex CommonNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form common_singular_head = noun(noun);
    }
    construction type_singular_head: SingularHead {
        element TypeSingularHead { noun: lex TypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form type_singular_head = noun(noun);
    }
    construction artifact_subtype_singular_head: SingularHead {
        element ArtifactSubtypeSingularHead { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form artifact_subtype_singular_head = noun(noun);
    }
    construction battle_subtype_singular_head: SingularHead {
        element BattleSubtypeSingularHead { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form battle_subtype_singular_head = noun(noun);
    }
    construction creature_subtype_singular_head: SingularHead {
        element CreatureSubtypeSingularHead { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form creature_subtype_singular_head = noun(noun);
    }
    construction enchantment_subtype_singular_head: SingularHead {
        element EnchantmentSubtypeSingularHead { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form enchantment_subtype_singular_head = noun(noun);
    }
    construction land_subtype_singular_head: SingularHead {
        element LandSubtypeSingularHead { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form land_subtype_singular_head = noun(noun);
    }
    construction planeswalker_subtype_singular_head: SingularHead {
        element PlaneswalkerSubtypeSingularHead { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form planeswalker_subtype_singular_head = noun(noun);
    }
    construction spell_subtype_singular_head: SingularHead {
        element SpellSubtypeSingularHead { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form spell_subtype_singular_head = noun(noun);
    }
    construction common_plural_head: PluralHead {
        element CommonPluralHead { noun: lex CommonNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form common_plural_head = noun(noun);
    }
    construction type_plural_head: PluralHead {
        element TypePluralHead { noun: lex TypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form type_plural_head = noun(noun);
    }
    construction artifact_subtype_plural_head: PluralHead {
        element ArtifactSubtypePluralHead { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form artifact_subtype_plural_head = noun(noun);
    }
    construction battle_subtype_plural_head: PluralHead {
        element BattleSubtypePluralHead { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form battle_subtype_plural_head = noun(noun);
    }
    construction creature_subtype_plural_head: PluralHead {
        element CreatureSubtypePluralHead { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form creature_subtype_plural_head = noun(noun);
    }
    construction enchantment_subtype_plural_head: PluralHead {
        element EnchantmentSubtypePluralHead { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form enchantment_subtype_plural_head = noun(noun);
    }
    construction land_subtype_plural_head: PluralHead {
        element LandSubtypePluralHead { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form land_subtype_plural_head = noun(noun);
    }
    construction planeswalker_subtype_plural_head: PluralHead {
        element PlaneswalkerSubtypePluralHead { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form planeswalker_subtype_plural_head = noun(noun);
    }
    construction spell_subtype_plural_head: PluralHead {
        element SpellSubtypePluralHead { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form spell_subtype_plural_head = noun(noun);
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
            FaceDown => Values::Consonant,
            Maximum => Values::Consonant,
            Other => Values::Vowel,
            Target => Values::Consonant,
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
    construction common_noun_modifier: NominalModifier {
        element CommonNounModifier { noun: lex CommonNoun, }
        require noun.compoundability is Compoundable;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form common_noun_modifier = noun(noun);
    }
    construction step_modifier: NominalModifier {
        element StepModifierValue { modifier: lex StepModifier, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = modifier.onset;
        form step_modifier = lex(modifier);
    }
    construction type_modifier: NominalModifier {
        element TypeModifier { noun: lex TypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form type_modifier = noun(noun);
    }
    construction artifact_subtype_modifier: NominalModifier {
        element ArtifactSubtypeModifier { noun: lex ArtifactSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form artifact_subtype_modifier = noun(noun);
    }
    construction battle_subtype_modifier: NominalModifier {
        element BattleSubtypeModifier { noun: lex BattleSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form battle_subtype_modifier = noun(noun);
    }
    construction creature_subtype_modifier: NominalModifier {
        element CreatureSubtypeModifier { noun: lex CreatureSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form creature_subtype_modifier = noun(noun);
    }
    construction enchantment_subtype_modifier: NominalModifier {
        element EnchantmentSubtypeModifier { noun: lex EnchantmentSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form enchantment_subtype_modifier = noun(noun);
    }
    construction land_subtype_modifier: NominalModifier {
        element LandSubtypeModifier { noun: lex LandSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form land_subtype_modifier = noun(noun);
    }
    construction planeswalker_subtype_modifier: NominalModifier {
        element PlaneswalkerSubtypeModifier { noun: lex PlaneswalkerSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form planeswalker_subtype_modifier = noun(noun);
    }
    construction spell_subtype_modifier: NominalModifier {
        element SpellSubtypeModifier { noun: lex SpellSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form spell_subtype_modifier = noun(noun);
    }
    construction non_color_modifier: NominalModifier {
        element NonColorModifier { color: lex Color, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_color_modifier = prefix("non", lex(color));
    }
    construction non_common_noun_modifier: NominalModifier {
        element NonCommonNounModifier { noun: lex NonCommonNoun, }
        derive modifier_license = Values::Unrestricted;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_common_noun_modifier = prefix("non", lex(noun));
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
    construction non_type_modifier: NominalModifier {
        element NonTypeModifier { noun: lex TypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_type_modifier = prefix("non", noun(noun));
    }
    construction non_artifact_subtype_modifier: NominalModifier {
        element NonArtifactSubtypeModifier { noun: lex ArtifactSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_artifact_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_battle_subtype_modifier: NominalModifier {
        element NonBattleSubtypeModifier { noun: lex BattleSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_battle_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_creature_subtype_modifier: NominalModifier {
        element NonCreatureSubtypeModifier { noun: lex CreatureSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_creature_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_enchantment_subtype_modifier: NominalModifier {
        element NonEnchantmentSubtypeModifier { noun: lex EnchantmentSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_enchantment_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_land_subtype_modifier: NominalModifier {
        element NonLandSubtypeModifier { noun: lex LandSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_land_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_planeswalker_subtype_modifier: NominalModifier {
        element NonPlaneswalkerSubtypeModifier { noun: lex PlaneswalkerSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_planeswalker_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_spell_subtype_modifier: NominalModifier {
        element NonSpellSubtypeModifier { noun: lex SpellSubtypeNoun, }
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_spell_subtype_modifier = prefix("non-", noun(noun));
    }
    construction negative_modifier_member: NegativeNominalModifier {
        element NegativeModifierMember { value: NominalModifier, }
        require any(
            value is NonColorModifier,
            value is NonCommonNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier,
            value is NonTypeModifier,
            value is NonArtifactSubtypeModifier,
            value is NonBattleSubtypeModifier,
            value is NonCreatureSubtypeModifier,
            value is NonEnchantmentSubtypeModifier,
            value is NonLandSubtypeModifier,
            value is NonPlaneswalkerSubtypeModifier,
            value is NonSpellSubtypeModifier
        );
        form negative_modifier_member = value;
    }
    construction coordinated_modifier_member: CoordinatedNominalModifier {
        element CoordinatedModifierMember { value: NominalModifier, }
        require value.modifier_license is Unrestricted;
        require any(
            value is AttributiveAdjectiveModifier,
            value is ColorModifier,
            value is CommonNounModifier,
            value is StatusModifier,
            value is SupertypeModifier,
            value is TypeModifier,
            value is ArtifactSubtypeModifier,
            value is BattleSubtypeModifier,
            value is CreatureSubtypeModifier,
            value is EnchantmentSubtypeModifier,
            value is LandSubtypeModifier,
            value is PlaneswalkerSubtypeModifier,
            value is SpellSubtypeModifier,
            value is NonColorModifier,
            value is NonCommonNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier,
            value is NonTypeModifier,
            value is NonArtifactSubtypeModifier,
            value is NonBattleSubtypeModifier,
            value is NonCreatureSubtypeModifier,
            value is NonEnchantmentSubtypeModifier,
            value is NonLandSubtypeModifier,
            value is NonPlaneswalkerSubtypeModifier,
            value is NonSpellSubtypeModifier
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
        form negative_modified_singular_nominal = leading modifiers head;
    }
    construction bare_plural_nominal: PluralNominal {
        element BarePluralNominal { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive nominal_form = Values::BarePluralNoun;
        derive onset = head.onset;
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
        form negative_modified_plural_nominal = leading modifiers head;
    }
    construction bare_singular_coordination_member: SingularCoordinationMember {
        element BareSingularCoordinationMember { head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
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
        form negative_modified_singular_coordination_member = leading modifiers head;
    }
    construction bare_plural_coordination_member: PluralCoordinationMember {
        element BarePluralCoordinationMember { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
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
        form plural_and_or_nominal_coordination = members;
    }
    construction singular_nominal_value: Nominal {
        element SingularNominalValue { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive nominal_form = nominal.nominal_form;
        derive onset = nominal.onset;
        form singular_nominal_value = nominal;
    }
    construction plural_nominal_value: Nominal {
        element PluralNominalValue { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive nominal_form = nominal.nominal_form;
        derive onset = nominal.onset;
        form plural_nominal_value = nominal;
    }
    construction singular_coordination_nominal_value: Nominal {
        element SingularCoordinationNominalValue {
            coordination: SingularNominalCoordination,
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive nominal_form = coordination.nominal_form;
        derive onset = coordination.onset;
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
        form modified_plural_coordination_nominal_value = first rest coordination;
    }
    construction unmarked_singular_selector: SingularSelector {
        element UnmarkedSingularSelector { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form unmarked_singular_selector = nominal;
    }
    construction unmarked_plural_selector: PluralSelector {
        element UnmarkedPluralSelector { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form unmarked_plural_selector = nominal;
    }
    construction unmarked_plural_coordination_selector: PluralSelector {
        element UnmarkedPluralCoordinationSelector {
            coordination: PluralNominalCoordination,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form unmarked_plural_coordination_selector = coordination;
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
    construction one_quantifying_determiner: Determinative {
        element OneQuantifyingDeterminer { count: CardinalQuantity, }
        require count.cardinality is One;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form one_quantifying_determiner = count;
    }
    construction plural_cardinal_quantifying_determiner: Determinative {
        element PluralCardinalQuantifyingDeterminer { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form plural_cardinal_quantifying_determiner = count;
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
    construction up_to_one_quantifying_determiner: Determinative {
        element UpToOneQuantifyingDeterminer { count: CardinalQuantity, }
        require count.cardinality is One;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Vowel;
        form up_to_one_quantifying_determiner = "up" "to" count;
    }
    construction up_to_many_quantifying_determiner: Determinative {
        element UpToManyQuantifyingDeterminer { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Vowel;
        form up_to_many_quantifying_determiner = "up" "to" count;
    }
    construction any_number_quantifying_determiner: Determinative {
        element AnyNumberQuantifyingDeterminer {}
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Vowel;
        form any_number_quantifying_determiner = "any" "number" "of";
    }
    construction one_or_more_quantifying_determiner: Determinative {
        element OneOrMoreQuantifyingDeterminer {}
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form one_or_more_quantifying_determiner = "one" "or" "more";
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
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form count_comparison_quantifying_determiner = count comparison;
    }
    construction named_card_reference: UnqualifiedReference {
        element NamedCardReference { name: identity CardName, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form named_card_reference = "a" "card" "named" identity(name);
    }
    construction mass_common_noun_reference: UnqualifiedReference {
        element MassCommonNounReference { noun: lex MassCommonNoun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form mass_common_noun_reference = lex(noun);
    }
    construction all_mass_common_noun_reference: UnqualifiedReference {
        element AllMassCommonNounReference { noun: lex MassCommonNoun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form all_mass_common_noun_reference = "all" lex(noun);
    }
    construction definite_next_mass_quantity_reference: UnqualifiedReference {
        element DefiniteNextMassQuantityReference {
            quantity: Amount,
            noun: lex MassCommonNoun,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form definite_next_mass_quantity_reference = "the" "next" quantity lex(noun);
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
            possessor: SingularNominal,
            possessed: SingularNominal,
        }
        derive agreement = possessed.agreement;
        derive number = possessed.number;
        derive onset = Values::Consonant;
        form demonstrative_possessive_reference = lex(demonstrative) suffix(possessor, "'s") possessed;
    }
    construction possessed_singular_reference: UnqualifiedReference {
        element PossessedSingularReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
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
        form possessed_plural_reference = lex(possessor) nominal;
    }
    construction genitive_determiner_singular_reference: UnqualifiedReference {
        element GenitiveDeterminerSingularReference {
            possessor: UnqualifiedReference,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form genitive_determiner_singular_reference = suffix(possessor, "'s") nominal;
    }
    construction genitive_determiner_plural_reference: UnqualifiedReference {
        element GenitiveDeterminerPluralReference {
            possessor: UnqualifiedReference,
            nominal: PluralNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form genitive_determiner_plural_reference = suffix(possessor, "'s") nominal;
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
        derive det.number = nominal.number;
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form determined_nominal = det nominal;
    }
    construction full_and_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndNounPhraseCoordination {
            members: seq UnqualifiedReference separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        form full_and_noun_phrase_coordination = members;
    }
    construction full_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullOrNounPhraseCoordination {
            members: seq UnqualifiedReference separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form full_or_noun_phrase_coordination = members;
    }
    construction full_and_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndOrNounPhraseCoordination {
            members: seq UnqualifiedReference separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form full_and_or_noun_phrase_coordination = members;
    }
    construction coordinated_noun_phrase: UnqualifiedReference {
        element CoordinatedNounPhrase {
            coordination: FullNounPhraseCoordination checked by full_coordination_is_independent(),
        }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive onset = Values::Consonant;
        form coordinated_noun_phrase = coordination;
    }
    construction self_reference: UnqualifiedReference {
        element SourceSelfReference { spelling: identity SelfReferenceSpelling, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = spelling.onset;
        form self_reference = identity(spelling);
    }
    construction self_genitive_singular_reference: UnqualifiedReference {
        element SelfGenitiveSingularReference {
            spelling: identity SelfReferenceSpelling,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = spelling.onset;
        form apostrophe when spelling.possessive_ending is EndsInS =
            suffix(identity(spelling), "'") nominal;
        form apostrophe_s otherwise = suffix(identity(spelling), "'s") nominal;
    }
    construction self_genitive_plural_reference: UnqualifiedReference {
        element SelfGenitivePluralReference {
            spelling: identity SelfReferenceSpelling,
            nominal: PluralNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = spelling.onset;
        form apostrophe when spelling.possessive_ending is EndsInS =
            suffix(identity(spelling), "'") nominal;
        form apostrophe_s otherwise = suffix(identity(spelling), "'s") nominal;
    }
    construction self_genitive_coordination_reference: UnqualifiedReference {
        element SelfGenitiveCoordinationReference {
            spelling: identity SelfReferenceSpelling,
            coordination: SingularNominalCoordination,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = spelling.onset;
        form apostrophe when spelling.possessive_ending is EndsInS =
            suffix(identity(spelling), "'") coordination;
        form apostrophe_s otherwise = suffix(identity(spelling), "'s") coordination;
    }
    construction this_way: MannerReference {
        element ThisWay {}
        form this_way = "this" "way";
    }
    construction at_random_manner: MannerReference {
        element AtRandomManner {}
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
    construction singular_owner_possessor: OwnerPossessor {
        element SingularOwnerPossessor { possessor: lex PossessiveDeterminerPronoun, }
        derive onset = possessor.onset;
        form singular_owner_possessor = lex(possessor) "owner's";
    }
    construction plural_owner_possessor: OwnerPossessor {
        element PluralOwnerPossessor { possessor: lex PossessiveDeterminerPronoun, }
        require possessor is Their;
        derive onset = possessor.onset;
        form plural_owner_possessor = lex(possessor) "owners'";
    }
    construction owner_possessed_reference: UnqualifiedReference {
        element OwnerPossessedReference {
            owner: OwnerPossessor,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = owner.onset;
        form owner_possessed_reference = owner nominal;
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
    construction from_phrase: FromPhrase {
        element FromPhraseValue { complement: Object, }
        form from_phrase = "from" complement;
    }
    construction of_phrase: OfPhrase {
        element OfPhraseValue { complement: Object, }
        form of_phrase = "of" complement;
    }
    construction from_bare_locative: FromPhrase {
        element FromBareLocative { complement: lex BareLocativeNoun, }
        form from_bare_locative = "from" lex(complement);
    }
    construction from_anywhere: FromPhrase {
        element FromAnywhere {}
        form from_anywhere = "from" "anywhere";
    }
    construction from_among_phrase: FromPhrase {
        element FromAmongPhrase { complement: AmongPhrase, }
        form from_among_phrase = "from" complement;
    }
    construction into_phrase: IntoPhrase {
        element IntoPhraseValue { complement: Object, }
        form into_phrase = "into" complement;
    }
    construction onto_phrase: OntoPhrase {
        element OntoPhraseValue { complement: Object, }
        form onto_phrase = "onto" complement;
    }
    construction edge_of_phrase: EdgeOfPhrase {
        element EdgeOfPhraseValue {
            position: lex EdgePosition,
            whole: Object,
        }
        form top when position is Top = lex(position) "of" whole;
        form bottom otherwise = "the" lex(position) "of" whole;
    }
    construction on_phrase: OnPhrase {
        element OnPhraseValue { complement: Object, }
        form on_phrase = "on" complement;
    }
    construction on_edge_phrase: OnPhrase {
        element OnEdgePhrase { complement: EdgeOfPhrase, }
        form on_edge_phrase = "on" complement;
    }
    construction to_phrase: ToPhrase {
        element ToPhraseValue { complement: Object, }
        form to_phrase = "to" complement;
    }
    construction direct_control_postmodifier: ControlPostmodifier {
        element DirectControlPostmodifier {
            controller: lex PossessiveDeterminerPronoun,
        }
        form direct_control_postmodifier = "under" lex(controller) "control";
    }
    construction owner_control_postmodifier: ControlPostmodifier {
        element OwnerControlPostmodifier { owner: OwnerPossessor, }
        form owner_control_postmodifier = "under" owner "control";
    }
    construction in_phrase: InPhrase {
        element InPhraseValue { complement: Object, }
        form in_phrase = "in" complement;
    }
    construction in_bare_locative: InPhrase {
        element InBareLocative { complement: lex BareLocativeNoun, }
        form in_bare_locative = "in" lex(complement);
    }
    construction fixed_scalar_threshold: ScalarThreshold {
        element FixedScalarThreshold { value: lex ScalarNumber, }
        form fixed_scalar_threshold = lex(value);
    }
    construction variable_scalar_threshold: ScalarThreshold {
        element VariableScalarThreshold { value: lex Variable, }
        form variable_scalar_threshold = lex(value);
    }
    construction characteristic_scalar: ScalarMeasure {
        element CharacteristicScalar { characteristic: lex ScalarCharacteristic, }
        form characteristic_scalar = lex(characteristic);
    }
    construction mana_value_scalar: ScalarMeasure {
        element ManaValueScalar {}
        form mana_value_scalar = "mana" "value";
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
            possessor: SingularSelector,
            measure: ScalarMeasure,
        }
        form genitive_scalar_value = "the" suffix(possessor, "'s") measure;
    }
    construction number_of_scalar_value: ScalarValue {
        element NumberOfScalarValue { counted: Object, }
        form number_of_scalar_value = "the" "number" "of" counted;
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
            domain: AmongPhrase,
        }
        form greatest_scalar_value = "the" "greatest" measure domain;
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
        form subject_relative_qualified_reference = reference clause;
    }
    construction contracted_copular_relative_reference: ControllerStage {
        element ContractedCopularRelativeReference {
            reference: UnqualifiedReference,
            nominal: SingularNominal,
            complement: OfPhrase,
        }
        require reference.number is Singular;
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form contracted_copular_relative_reference = reference "that's" "a" nominal complement;
    }
    construction reduced_passive_qualified_reference: ControllerStage {
        element ReducedPassiveQualifiedReference {
            reference: UnqualifiedReference,
            clause: PassivePredicate,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
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
        form other_than_qualified_reference = reference "other" "than" excluded;
    }
    construction unqualified_locative_stage: LocativeStage {
        element UnqualifiedLocativeStage { reference: ControllerStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_locative_stage = reference;
    }
    construction from_qualified_reference: LocativeStage {
        element FromQualifiedReference {
            reference: ControllerStage,
            source: FromPhrase,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form from_qualified_reference = reference source;
    }
    construction in_qualified_reference: LocativeStage {
        element InQualifiedReference {
            reference: ControllerStage,
            location: InPhrase,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form in_qualified_reference = reference location;
    }
    construction on_qualified_reference: LocativeStage {
        element OnQualifiedReference {
            reference: ControllerStage,
            location: OnPhrase,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form on_qualified_reference = reference location;
    }
    construction of_qualified_reference: LocativeStage {
        element OfQualifiedReference {
            reference: ControllerStage,
            complement: OfPhrase,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form of_qualified_reference = reference complement;
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
    construction determinative_partitive: NounPhrase {
        element DeterminativePartitive {
            head: Determinative checked by determinative_licenses_partitive_head(
                head.fused_head_license,
                head.determiner_number,
                head.number,
                whole.number
            ),
            whole: Object,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
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
        element SingularCommonNounChoice { noun: lex CommonNoun, }
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form singular_common_noun_choice = noun(noun);
    }
    construction plural_common_noun_choice: CommonNounChoice {
        element PluralCommonNounChoice { noun: lex CommonNoun, }
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
    construction possessive_self_reference: PossessiveOwner {
        element PossessiveSelfReference { spelling: identity SelfReferenceSpelling, }
        derive number = Values::Singular;
        derive possessive_ending = spelling.possessive_ending;
        form possessive_self_reference = identity(spelling);
    }
    construction possessive_plural_noun: PossessiveOwner {
        element PossessiveNoun { head: PluralHead, }
        derive number = Values::Plural;
        derive possessive_ending = head.possessive_ending;
        form possessive_plural_noun = head;
    }
    construction possessive_singular_reference: PossessiveOwner {
        element PossessiveSingularReference { reference: UnqualifiedReference, }
        require reference.number is Singular;
        derive number = Values::Singular;
        derive possessive_ending = Values::Other;
        form possessive_singular_reference = reference;
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
    construction singular_card_quantity: CardQuantity {
        element SingularCardQuantity {}
        form singular_card_quantity = "a" "card";
    }
    construction fixed_card_quantity: CardQuantity {
        element FixedCardQuantity { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        form fixed_card_quantity = count "cards";
    }
    construction variable_card_quantity: CardQuantity {
        element VariableCardQuantity { count: lex Variable, }
        form variable_card_quantity = lex(count) "cards";
    }
    construction anaphoric_card_quantity: CardQuantity {
        element AnaphoricCardQuantity { count: CountReference, }
        form anaphoric_card_quantity = count "cards";
    }
    construction compared_card_quantity: CardQuantity {
        element ComparedCardQuantity {
            count: CardinalQuantity,
            comparison: CountComparison,
        }
        form compared_card_quantity = count comparison "cards";
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
    construction named_counter: CounterKind {
        element NamedCounter { name: lex CounterName, }
        derive onset = name.onset;
        form named_counter = lex(name);
    }
    construction singular_counter_quantity: CounterQuantity {
        element SingularCounterQuantity { kind: CounterKind, }
        form an when kind.onset is Vowel = "an" kind "counter";
        form a otherwise = "a" kind "counter";
    }
    construction fixed_counter_quantity: CounterQuantity {
        element FixedCounterQuantity {
            count: CardinalQuantity,
            kind: CounterKind,
        }
        require count.cardinality is TwoPlus;
        form fixed_counter_quantity = count kind "counters";
    }
    construction variable_counter_quantity: CounterQuantity {
        element VariableCounterQuantity {
            count: lex Variable,
            kind: CounterKind,
        }
        form variable_counter_quantity = lex(count) kind "counters";
    }
    construction anaphoric_counter_quantity: CounterQuantity {
        element AnaphoricCounterQuantity {
            count: CountReference,
            kind: CounterKind,
        }
        form anaphoric_counter_quantity = count kind "counters";
    }
    construction unnamed_singular_counter_quantity: CounterQuantity {
        element UnnamedSingularCounterQuantity {}
        form unnamed_singular_counter_quantity = "a" "counter";
    }
    construction unnamed_fixed_counter_quantity: CounterQuantity {
        element UnnamedFixedCounterQuantity { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        form unnamed_fixed_counter_quantity = count "counters";
    }
    construction unnamed_variable_counter_quantity: CounterQuantity {
        element UnnamedVariableCounterQuantity { count: lex Variable, }
        form unnamed_variable_counter_quantity = lex(count) "counters";
    }
    construction singular_die_object: DieObject {
        element SingularDieObject { shape: opt lex DieShape, }
        form singular_die_object = "a" lex(shape) "die";
    }
    construction fixed_dice_object: DieObject {
        element FixedDiceObject {
            count: CardinalQuantity,
            shape: opt lex DieShape,
        }
        require count.cardinality is TwoPlus;
        form fixed_dice_object = count lex(shape) "dice";
    }
    construction d20_object: DieObject {
        element D20Object {}
        form d20_object = "a" "d20";
    }
    construction base_verb_phrase: VerbPhrase {
        element BaseVerbPhrase { frame: BaseVerbFrame, }
        derive agreement = frame.agreement;
        form base_verb_phrase = frame;
    }
    construction pro_verb_predicate: VerbPhrase {
        element ProVerbPredicate {}
        derive agreement = verb.agreement;
        form pro_verb_predicate = verb(VerbLexeme::Do);
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
            destination: IntoPhrase,
        }
        derive agreement = head.agreement;
        form declared_object_into_object_frame = verb(head) object destination;
    }
    construction scalar_distributed_damage_amount: DistributedDamageAmount {
        element ScalarDistributedDamageAmount { amount: Amount, }
        form scalar_distributed_damage_amount = amount;
    }
    construction object_distribution_recipient: DistributionRecipient {
        element ObjectDistributionRecipient { object: Object, }
        form object_distribution_recipient = object;
    }
    construction chosen_damage_distribution: DamageDistribution {
        element ChosenDamageDistribution { recipient: DistributionRecipient, }
        form chosen_damage_distribution =
            "divided" "as" "you" "choose" "among" recipient;
    }
    construction even_damage_distribution: DamageDistribution {
        element EvenDamageDistribution { recipient: DistributionRecipient, }
        form even_damage_distribution =
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
    construction deal_amount_damage: VerbPhrase {
        element DealAmountDamage { head: lex DealAmountDamageVerb, amount: Amount, recipient: ToPhrase, }
        derive agreement = head.agreement;
        form deal_amount_damage = verb(head) amount "damage" recipient;
    }
    construction deal_distributed_damage: VerbPhrase {
        element DealDistributedDamage {
            head: lex DealDistributedDamageVerb,
            amount: DistributedDamageAmount,
            distribution: DamageDistribution,
            replacement: opt lex DistributionReplacement,
        }
        derive agreement = head.agreement;
        form deal_distributed_damage =
            verb(head) amount "damage" distribution lex(replacement);
    }
    construction deal_unspecified_damage: VerbPhrase {
        element DealDamageKind { head: lex DealDamageKindVerb,
            kind: lex DamageKind,
            recipient: opt ToPhrase,
        }
        derive agreement = head.agreement;
        form deal_unspecified_damage = verb(head) lex(kind) recipient;
    }
    construction life_amount: VerbPhrase {
        element LifeAmount { head: lex LifeAmountVerb, amount: Amount, }
        derive agreement = head.agreement;
        form life_amount = verb(head) amount "life";
    }
    construction gain_unspecified_life: VerbPhrase {
        element GainUnspecifiedLife { head: lex GainLifeVerb, }
        derive agreement = head.agreement;
        form gain_unspecified_life = verb(head) "life";
    }
    construction deal_damage_equal_to: VerbPhrase {
        element DealDamageEqualTo { head: lex DealDamageEqualToVerb,
            equality: ScalarEquality,
            recipient: ToPhrase,
        }
        derive agreement = head.agreement;
        form deal_damage_equal_to = verb(head) "damage" equality recipient;
    }
    construction deal_damage_to_equal_to: VerbPhrase {
        element DealDamageToEqualTo { head: lex DealDamageToEqualToVerb,
            recipient: ToPhrase,
            equality: ScalarEquality,
        }
        derive agreement = head.agreement;
        form deal_damage_to_equal_to = verb(head) "damage" recipient equality;
    }
    construction life_equality: VerbPhrase {
        element LifeEquality { head: lex LifeEqualityVerb, equality: ScalarEquality, }
        derive agreement = head.agreement;
        form life_equality = verb(head) "life" equality;
    }
    construction pay_life: VerbPhrase {
        element PayLife { amount: Amount, }
        derive agreement = verb.agreement;
        form pay_life = verb(VerbLexeme::Pay) amount "life";
    }
    construction pay_mana: VerbPhrase {
        element PayMana { mana: ManaPhrase, }
        derive agreement = verb.agreement;
        form pay_mana = verb(VerbLexeme::Pay) mana;
    }
    construction add_mana: VerbPhrase {
        element AddMana { mana: ManaPhrase, }
        derive agreement = verb.agreement;
        form add_mana = verb(VerbLexeme::Add) mana;
    }
    construction flexible_mana: VerbPhrase {
        element FlexibleMana {
            amount: CardinalQuantity,
            kind: lex FlexibleManaKind,
        }
        derive agreement = verb.agreement;
        form flexible_mana = verb(VerbLexeme::Add) amount "mana" "of" "any" lex(kind);
    }
    construction draw_cards: VerbPhrase {
        element DrawCards { cards: CardQuantity, }
        derive agreement = verb.agreement;
        form draw_cards = verb(VerbLexeme::Draw) cards;
    }
    construction draw_cards_equal_to: VerbPhrase {
        element DrawCardsEqualTo { equality: ScalarEquality, }
        derive agreement = verb.agreement;
        form draw_cards_equal_to = verb(VerbLexeme::Draw) "cards" equality;
    }
    construction roll_dice: VerbPhrase {
        element RollDice { dice: DieObject, }
        derive agreement = verb.agreement;
        form roll_dice = verb(VerbLexeme::Roll) dice;
    }
    construction put_counters: VerbPhrase {
        element PutCounters {
            counters: CounterQuantity,
            recipient: OnPhrase,
        }
        derive agreement = verb.agreement;
        form put_counters = verb(VerbLexeme::Put) counters recipient;
    }
    construction remove_counters: VerbPhrase {
        element RemoveCounters {
            counters: CounterQuantity,
            source: FromPhrase,
        }
        derive agreement = verb.agreement;
        form remove_counters = verb(VerbLexeme::Remove) counters source;
    }
    construction put_into: VerbPhrase {
        element PutInto {
            object: Object,
            source: opt FromPhrase,
            destination: IntoPhrase,
        }
        derive agreement = verb.agreement;
        form put_into = verb(VerbLexeme::Put) object source destination;
    }
    construction put_onto: VerbPhrase {
        element PutOnto {
            object: Object,
            source: opt FromPhrase,
            destination: OntoPhrase,
            result: opt PredicativeComplement,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form put_onto = verb(VerbLexeme::Put) object source destination result control;
    }
    construction put_on: VerbPhrase {
        element PutOn {
            object: Object,
            source: opt FromPhrase,
            destination: OnPhrase,
        }
        derive agreement = verb.agreement;
        form put_on = verb(VerbLexeme::Put) object source destination;
    }
    construction put_to: VerbPhrase {
        element PutTo {
            object: Object,
            destination: ToPhrase,
        }
        derive agreement = verb.agreement;
        form put_to = verb(VerbLexeme::Put) object destination;
    }
    construction return_to: VerbPhrase {
        element ReturnTo {
            object: Object,
            source: opt FromPhrase,
            destination: ToPhrase,
            result: opt PredicativeComplement,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form return_to = verb(VerbLexeme::Return) object source destination result control;
    }
    construction enter_resultative: VerbPhrase {
        element EnterResultative { result: PredicativeComplement, }
        derive agreement = verb.agreement;
        form enter_resultative = verb(VerbLexeme::Enter) result;
    }
    construction enter_with_counters: VerbPhrase {
        element EnterWithCounters {
            counters: CounterQuantity,
            recipient: OnPhrase,
        }
        derive agreement = verb.agreement;
        form enter_with_counters = verb(VerbLexeme::Enter) "with" counters recipient;
    }
    construction enter_location: VerbPhrase {
        element EnterLocation {
            location: Object,
            result: opt PredicativeComplement,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form enter_location = verb(VerbLexeme::Enter) location result control;
    }
    construction enter_control: VerbPhrase {
        element EnterControl { control: ControlPostmodifier, }
        derive agreement = verb.agreement;
        form enter_control = verb(VerbLexeme::Enter) control;
    }
    construction leave_location: VerbPhrase {
        element LeaveLocation { location: Object, }
        derive agreement = verb.agreement;
        form leave_location = verb(VerbLexeme::Leave) location;
    }
    construction look_at: VerbPhrase {
        element LookAt { object: Object, }
        derive agreement = verb.agreement;
        form look_at = verb(VerbLexeme::Look) "at" object;
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
    construction quoted_ability: QuotedAbility {
        element QuotedAbilityValue { ability: Ability, }
        form quoted_ability = sentence_initial(" \"") suffix(ability, "\"");
    }
    construction quoted_ability_predicate: VerbPhrase {
        element QuotedAbilityPredicate { ability: QuotedAbility, }
        derive agreement = verb.agreement;
        form quoted_ability_predicate = verb(VerbLexeme::Have) ability;
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
            adjustment: PowerToughnessAdjustment,
            duration: opt DurationPhrase,
        }
        derive agreement = verb.agreement;
        form get_power_toughness = verb(VerbLexeme::Get) adjustment duration;
    }
    construction have_base_power_toughness: VerbPhrase {
        element HaveBasePowerToughness { value: PredicativePowerToughnessComplement, }
        derive agreement = verb.agreement;
        form have_base_power_toughness = verb(VerbLexeme::Have)
            "base" "power" "and" "toughness" value;
    }
    construction have_life: VerbPhrase {
        element HaveLife { comparison: ScalarComparison, }
        derive agreement = verb.agreement;
        form have_life = verb(VerbLexeme::Have) comparison "life";
    }
    construction have_object_control: VerbPhrase {
        element HaveObjectControl {
            object: Object,
            predicate: VerbPhrase,
        }
        derive agreement = verb.agreement;
        derive predicate.agreement = Values::Bare;
        form have_object_control = verb(VerbLexeme::Have) object predicate;
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
    construction twice_variable_amount: DistributedDamageAmount {
        element TwiceVariableAmount { variable: lex Variable, }
        form twice_variable_amount = "twice" lex(variable);
    }
    construction variable_plus_amount: DistributedDamageAmount {
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
        derive cardinality = number.cardinality;
        derive number = number.number;
        form cardinal = lex(number);
    }

    abstract sum DocumentBlock { Ability, }
    abstract product OracleText {
        blocks: seq DocumentBlock separated by "\n",
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

fn determinative_is_fused(
    _head: &Determinative,
    fused_head_license: FusedHeadLicense,
) -> bool {
    fused_head_license == FusedHeadLicense::FusedHead
}

fn determinative_licenses_partitive_head(
    _head: &Determinative,
    fused_head_license: FusedHeadLicense,
    determiner_number: DeterminerNumber,
    head_number: Number,
    whole_number: Number,
) -> bool {
    fused_head_license == FusedHeadLicense::FusedHead
        && match determiner_number {
            DeterminerNumber::SingularOnly => head_number == Number::Singular,
            DeterminerNumber::PluralOnly => head_number == Number::Plural,
            DeterminerNumber::Both => head_number == whole_number,
        }
}

fn full_coordination_is_independent(coordination: &FullNounPhraseCoordination) -> bool {
    fn independently_realized(reference: &UnqualifiedReference) -> bool {
        !matches!(
            reference,
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
        return number == Number::Plural;
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
    let licenses_form = match nominal_license {
        NominalLicense::CountNominal => true,
        NominalLicense::BareSingularNoun => nominal_form == NominalForm::BareSingularNoun,
    };
    licenses_form
}

#[cfg(test)]
mod feature_recipe_tests;
