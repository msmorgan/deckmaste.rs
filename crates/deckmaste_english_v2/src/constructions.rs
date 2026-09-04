#![allow(
    clippy::pub_underscore_fields,
    reason = "the generated restriction product retains its typed Cast identity even though fixed Other Concord Class projection does not read the local binding"
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
    vocab OtherConcordClassAuxiliary { Dont = "don't", }
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
    // Attachment and bare-complement licensing are orthogonal declared facts.
    // Every member spells out both values so additions cannot inherit a
    // permissive default accidentally.
    vocab Preposition {
        After = "after" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = TemporalComplement;
        },
        Among = "among" {
            feature PrepositionAttachment = PostmodifierOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = SelectionComplement;
        },
        At = "at" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = TemporalComplement;
        },
        Before = "before" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = TemporalComplement;
        },
        During = "during" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = TemporalComplement;
        },
        For = "for" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
        From = "from" {
            feature PrepositionAttachment = PostmodifierOnly;
            feature BareLocativeComplement = Yes;
            feature PrepositionComplementKind = SourceComplement;
        },
        In = "in" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = Yes;
            feature PrepositionComplementKind = InComplement;
        },
        Into = "into" {
            feature PrepositionAttachment = SelectedOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
        Of = "of" {
            feature PrepositionAttachment = PostmodifierOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = RelationalComplement;
        },
        On = "on" {
            feature PrepositionAttachment = AdjunctCapable;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = OnComplement;
        },
        Onto = "onto" {
            feature PrepositionAttachment = SelectedOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
        To = "to" {
            feature PrepositionAttachment = SelectedOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
        Under = "under" {
            feature PrepositionAttachment = SelectedOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
        With = "with" {
            feature PrepositionAttachment = PostmodifierOnly;
            feature BareLocativeComplement = No;
            feature PrepositionComplementKind = UnrestrictedComplement;
        },
    }
    vocab LocativeProform { Anywhere = "anywhere", }
    vocab ComparativeQuantifier { Fewer = "fewer", More = "more", }
    vocab FrequencyAdverb { Once = "once", Twice = "twice", }
    vocab ScalarDegree { Equal = "equal", Lesser = "lesser", Greater = "greater", }
    vocab AttributiveAdjective {
        feature HomographLicense = Unlicensed;
        feature ModifierLicense = Unrestricted;
        Additional = "additional",
        Base = "base",
        Declare = "declare",
        FaceDown = "face-down",
        First = "first",
        Main = "main",
        Maximum = "maximum",
        New = "new",
        Next = "next",
        Other = "other",
        Postcombat = "postcombat",
        Precombat = "precombat",
        Same = "same",
        Second = "second",
        SixSided = "six-sided",
        Untap = "untap" { feature HomographLicense = Licensed; },
    }
    vocab TargetingMarker {
        feature HomographLicense = Licensed;
        Target = "target",
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
    vocab DefiniteMarker { The = "the", }
    vocab Supertype {
        Basic = "basic",
        Legendary = "legendary",
        Ongoing = "ongoing",
        Snow = "snow",
        World = "world",
    }

    morphology EnglishVerb {
        feature = ConcordClass;
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
        feature MannerAnaphorClass = OtherNoun;
        feature Properness = Common;
        Ability = "ability" {
            Plural = "abilities",
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = DeterminedRelational;
        },
        Attacker = "attacker" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Battlefield = "battlefield" {
            feature LocativeTemporalLicense = OnLicensed;
            feature Relationality = NonRelational;
        },
        Beginning = "beginning" {
            feature LocativeTemporalLicense = OfAndTemporalLicensed;
            feature Relationality = Relational;
        },
        Blocker = "blocker" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Card = "card" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Choice = "choice" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Coin = "coin" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Color = "color" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        CommandZone = "command zone" {
            feature LocativeTemporalLicense = InLicensed;
            feature Relationality = NonRelational;
        },
        Control = "control" {
            feature Countability = Mass;
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Copy = "copy" {
            Plural = "copies",
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Cost = "cost" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = QualifiedRelational;
        },
        Counter = "counter" {
            feature LocativeTemporalLicense = OnLicensed;
            feature Relationality = NonRelational;
        },
        Damage = "damage" {
            feature Countability = Mass;
            feature LocativeTemporalLicense = OfAndOnLicensed;
            feature Relationality = NonRelational;
        },
        Death = "death" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Draw = "draw" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        End = "end" {
            feature Compoundability = NonCompoundable;
            feature LocativeTemporalLicense = OfAndTemporalLicensed;
            feature Relationality = Relational;
        },
        Exile = "exile" {
            feature BareLocativeLicense = BareAllowed;
            feature LocativeTemporalLicense = InLicensed;
            feature Relationality = NonRelational;
        },
        Graveyard = "graveyard" {
            feature LocativeTemporalLicense = InLicensed;
            feature Relationality = NonRelational;
        },
        Hand = "hand" {
            feature BareLocativeLicense = BareAllowed;
            feature LocativeTemporalLicense = InLicensed;
            feature Relationality = NonRelational;
        },
        Library = "library" {
            Plural = "libraries",
            feature LocativeTemporalLicense = InOrOnEdgeLicensed;
            feature Relationality = NonRelational;
        },
        Life = "life" {
            feature Countability = Mass;
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Mana = "mana" {
            feature Countability = Mass;
            feature LocativeTemporalLicense = OfAndOnLicensed;
            feature Relationality = NonRelational;
        },
        Mode = "mode" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Name = "name" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Number = "number" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Controller = "controller" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Opponent = "opponent" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Owner = "owner" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Permanent = "permanent" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Phase = "phase" {
            feature LocativeTemporalLicense = OfAndTemporalLicensed;
            feature Relationality = Relational;
        },
        Player = "player" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Power = "power" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Rest = "rest" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Source = "source" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Size = "size" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Spell = "spell" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Stack = "stack" {
            feature LocativeTemporalLicense = OnLicensed;
            feature Relationality = NonRelational;
        },
        Step = "step" {
            feature LocativeTemporalLicense = OfAndTemporalLicensed;
            feature Relationality = Relational;
        },
        Target = "target" {
            feature Compoundability = NonCompoundable;
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = DeterminedRelational;
        },
        Tax = "tax" {
            Plural = "taxes",
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        Token = "token" {
            feature LocativeTemporalLicense = ObjectAttachmentLicensed;
            feature Relationality = QualifiedRelational;
        },
        Toughness = "toughness" {
            Plural = "toughnesses",
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Turn = "turn" {
            feature LocativeTemporalLicense = OfAndTemporalLicensed;
            feature Relationality = Relational;
        },
        Type = "type" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Value = "value" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = Relational;
        },
        Way = "way" {
            feature LocativeTemporalLicense = Unlicensed;
            feature MannerAnaphorClass = MannerAnaphor;
            feature Relationality = NonRelational;
        },
        Die = "die" {
            Plural = "dice",
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
        D20 = "d20" {
            feature LocativeTemporalLicense = Unlicensed;
            feature Relationality = NonRelational;
        },
    }
    codec IntransitiveVerb {
        generate declaration_verb {
            position = Verb;
            tail = [];
            feature = ConcordClass;
        }
    }
    codec TransitiveVerb {
        generate declaration_verb {
            position = Verb;
            tail = [ObjectNounPhrase];
            feature = ConcordClass;
        }
    }
    codec MeasureComplementVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Amount];
            feature = ConcordClass;
        }
    }
    codec ObjectForObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                location: ObjectNounPhrase,
                lex(Preposition::For),
                sought: ObjectNounPhrase,
            ];
            feature = ConcordClass;
        }
    }
    codec ToObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                lex(Preposition::To),
                complement: ObjectNounPhrase,
            ];
            feature = ConcordClass;
        }
    }
    codec ForObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [lex(Preposition::For), object: ObjectNounPhrase];
            feature = ConcordClass;
        }
    }
    codec ObjectAmountVerb {
        generate declaration_verb {
            position = Verb;
            tail = [object: ObjectNounPhrase, amount: Amount];
            feature = ConcordClass;
        }
    }
    codec ObjectPredicativeComplementVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                complement: PredicativeComplement,
            ];
            feature = ConcordClass;
        }
    }
    codec WithObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [lex(Preposition::With), object: ObjectNounPhrase];
            feature = ConcordClass;
        }
    }
    codec AmongObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = ["among", recipient: Object];
            feature = ConcordClass;
        }
    }
    codec ObjectWithObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                lex(Preposition::With),
                complement: ObjectNounPhrase,
            ];
            feature = ConcordClass;
        }
    }
    codec ObjectIntoObjectVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                object: ObjectNounPhrase,
                lex(Preposition::Into),
                complement: ObjectNounPhrase,
            ];
            feature = ConcordClass;
        }
    }
    codec DistributedMeasureVerb { generate declaration_verb { position = Verb; tail = [Amount, MassNoun, DistributionPhrase, DistributionReplacement?]; feature = ConcordClass; } }
    codec ObjectEqualityVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, ScalarEquality]; feature = ConcordClass; } }
    codec ObjectEqualityToVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, ScalarEquality, lex(Preposition::To), recipient: Object]; feature = ConcordClass; } }
    codec ObjectToEqualityVerb { generate declaration_verb { position = Verb; tail = [object: ObjectNounPhrase, lex(Preposition::To), recipient: Object, ScalarEquality]; feature = ConcordClass; } }
    codec ManaPhraseVerb { generate declaration_verb { position = Verb; tail = [ManaPhrase]; feature = ConcordClass; } }
    codec ObjectFromVerb {
        generate declaration_verb {
            position = Verb;
            tail = [object: ObjectNounPhrase, lex(Preposition::From), source: FrameComplement];
            feature = ConcordClass;
        }
    }
    codec ObjectFromOntoResultControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                Object,
                lex(Preposition::From)?,
                lex(Preposition::Onto),
                destination: FrameComplement,
                PredicativeComplement?,
                control: marked(Preposition::Under, Object)?,
            ];
            feature = ConcordClass;
        }
    }
    codec ObjectFromOnVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, lex(Preposition::From)?, lex(Preposition::On), destination: FrameComplement];
            feature = ConcordClass;
        }
    }
    codec ObjectToVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, lex(Preposition::To), destination: FrameComplement];
            feature = ConcordClass;
        }
    }
    codec ObjectFromToResultControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [
                Object,
                lex(Preposition::From)?,
                lex(Preposition::To),
                destination: FrameComplement,
                PredicativeComplement?,
                control: marked(Preposition::Under, Object)?,
            ];
            feature = ConcordClass;
        }
    }
    codec PredicativeComplementVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [PredicativeComplement];
            feature = ConcordClass;
        }
    }
    codec AuxiliaryVerb {
        generate declaration_verb {
            class = Auxiliary;
            position = Verb;
            tail = [];
            feature = ConcordClass;
        }
    }
    codec ObjectInfinitiveVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [Object, "to", VerbPhrase];
            feature = ConcordClass;
        }
    }
    codec ChooseInfinitiveVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [InfinitiveComplement];
            feature = ConcordClass;
        }
    }
    codec OrderedVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [Object, lex(Preposition::From)?, lex(Preposition::On), destination: FrameComplement, "in", ObjectOrder, "order"];
            feature = ConcordClass;
        }
    }
    codec HaveKeywordAbilityVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [KeywordAbility];
            feature = ConcordClass;
        }
    }
    codec CostComparisonVerb {
        generate declaration_verb {
            class = Predicate;
            position = Verb;
            tail = [ManaAmount, CostComparisonDirection, ControlledCostAction];
            feature = ConcordClass;
        }
    }
    codec ProVerbHead {
        generate declaration_verb {
            class = ProVerb;
            position = Verb;
            tail = [];
            feature = ConcordClass;
        }
    }
    codec EnterWithCountersVerb {
        generate declaration_verb {
            position = Verb;
            tail = [lex(Preposition::With), object: ObjectNounPhrase, lex(Preposition::On), recipient: FrameComplement];
            feature = ConcordClass;
        }
    }
    codec EnterLocationVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, PredicativeComplement?, control: marked(Preposition::Under, Object)?];
            feature = ConcordClass;
        }
    }
    codec EnterControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [control: marked(Preposition::Under, Object)];
            feature = ConcordClass;
        }
    }
    codec LookAtVerb {
        generate declaration_verb {
            position = Verb;
            tail = [lex(Preposition::At), Object];
            feature = ConcordClass;
        }
    }
    codec QuotedAbilityVerb {
        generate declaration_verb {
            position = Verb;
            tail = [QuotedAbility];
            feature = ConcordClass;
        }
    }
    codec HaveObjectControlVerb {
        generate declaration_verb {
            position = Verb;
            tail = [Object, VerbPhrase];
            feature = ConcordClass;
        }
    }
    codec GetPowerToughnessVerb {
        generate declaration_verb {
            position = Verb;
            tail = [PowerToughnessAdjustment];
            feature = ConcordClass;
        }
    }
    codec MovementParticipleHead {
        generate declaration_verb {
            position = Verb;
            tail = [moved: ObjectNounPhrase, lex(Preposition::Into), destination: ObjectNounPhrase];
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
                lex(Preposition::To),
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
                lex(Preposition::On),
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
    codec FlavorWordTerm {
        generate declaration_term {
            position = FixedTerm;
            kinds = [FlavorWord];
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
                    fused_head_license = PluralPredeterminer;
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
                    fused_head_license = PartitiveOnly;
                    realizations = [{ surface = "any"; }];
                },
                AnyOne {
                    number_license = SingularOnly;
                    nominal_license = CountNominal;
                    fused_head_license = NominalOnly;
                    realizations = [{ surface = "any one"; }];
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
    abstract sum LexicalVerbPhrase {
        IntransitiveLexicalVerbPhrase,
        TransitiveLexicalVerbPhrase,
        GetPowerToughnessLexicalVerbPhrase,
        MeasureComplementLexicalVerbPhrase,
        ObjectAmountLexicalVerbPhrase,
        WithObjectLexicalVerbPhrase,
        ObjectWithObjectLexicalVerbPhrase,
        ObjectForObjectLexicalVerbPhrase,
        ObjectIntoObjectLexicalVerbPhrase,
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
    abstract sum PredicateAdjunct {
        Prepositional: PrepositionalPredicateAdjunct,
        Purpose: PurposePredicateAdjunct,
        Duration: DurationPredicateAdjunct,
        Frequency: FrequencyPredicateAdjunct,
        Manner: MannerPredicateAdjunct,
    }
    abstract sum PrepositionalPredicateAdjunctHost {
        Verb: VerbPhrase,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
        Alternative: AlternativePredicate,
    }
    abstract sum ScalarDegreePhrase {
        Single: SingleScalarDegreePhrase,
        Or: OrScalarDegreePhrase,
    }
    abstract sum TemporalEndpoint { Reference: NounPhrase, }
    // A preposition takes an ordinary object or an edge locative. Bare
    // locatives and flat "from among ..." phrases have dedicated routes.
    abstract sum PrepositionalComplement {
        Object: Object,
        Edge: EdgeOfPhrase,
        Keyword: GrantedKeywordLine,
        Quoted: QuotedAbility,
        ScalarMeasure: ScalarMeasureValue,
        DegreeMeasure: DegreeMeasure,
        PowerToughness: PowerToughnessValue,
    }
    abstract sum ScalarMeasureAssignedValue {
        Exact: ScalarThreshold,
        Comparison: ScalarComparison,
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
        ContractedPerfectAdjunct: ContractedPerfectAdjunctObjectGapRelativeClause,
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
        Adjunct: PredicateAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Instead: InsteadPredicate,
        Alternative: AlternativePredicate,
        WithoutGerundObject: WithoutGerundObjectPredicate,
        CostComparison: CostComparisonPredicate,
        ActionRestriction: ActionRestrictionPredicate,
    }
    abstract sum BareCoordinatedPredicate {
        Atomic: VerbPhrase,
        BareCopular: BareCopularPredicate,
        BarePassive: BarePassivePredicate,
        Adjunct: PredicateAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Instead: InsteadPredicate,
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
        Adjunct: PredicateAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Instead: InsteadPredicate,
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
        Adjunct: PredicateAdjunctPredicate,
        Cause: ObjectInfinitivePredicate,
        Requirement: RequirementPredicate,
        TransitiveRequirement: TransitiveRequirementPredicate,
        AsThough: AsThoughPredicate,
        Ordered: OrderedPredicate,
        Instead: InsteadPredicate,
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
        PreposedPredicateAdjunct,
        PreposedPredicateAdjunctPredicate,
        PreposedWhile,
        PreposedWhilePredicate,
        PreposedUntil,
        PreposedUntilPredicate,
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
        FlavorWord: FlavorWordModeMarker,
    }
    construction bullet_marker: ModeMarker {
        element BulletMarker {}
        form bullet_marker = sentence_initial("• ");
    }
    construction flavor_word_mode_marker: ModeMarker {
        element FlavorWordModeMarker { term: lex FlavorWordTerm, }
        form flavor_word_mode_marker = sentence_initial("• ") lex(term) sentence_initial(" — ");
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
        require phrase.preposition_attachment is AdjunctCapable;
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
        derive predicate.concord_class = Values::Other;
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
        derive predicate.concord_class = Values::Other;
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
        form starting_with = "starting" lex(Preposition::With) starter "," body;
    }
    construction preposed_if_predicate: ClauseAttachment {
        element PreposedIfPredicate { condition: Clause, body: Predicate, }
        derive body.concord_class = Values::Other;
        form preposed_if_predicate = "if" condition "," body;
    }
    construction postposed_if: ClauseAttachment {
        element PostposedIf { body: Clause, condition: FiniteClause, }
        form postposed_if = body "if" condition;
    }
    construction postposed_if_predicate: ClauseAttachment {
        element PostposedIfPredicate { body: Predicate, condition: FiniteClause, }
        derive body.concord_class = Values::Other;
        form postposed_if_predicate = body "if" condition;
    }
    construction postposed_unless: ClauseAttachment {
        element PostposedUnless { body: Clause, condition: FiniteClause, }
        form postposed_unless = body "unless" condition;
    }
    construction postposed_unless_predicate: ClauseAttachment {
        element PostposedUnlessPredicate { body: Predicate, condition: FiniteClause, }
        derive body.concord_class = Values::Other;
        form postposed_unless_predicate = body "unless" condition;
    }
    construction preposed_as: ClauseAttachment {
        element PreposedAs { condition: FiniteClause, body: Predicate, }
        derive body.concord_class = Values::Other;
        form preposed_as = "as" condition "," body;
    }
    construction preposed_as_long_as: ClauseAttachment {
        element PreposedAsLongAs { condition: Clause, body: Clause, }
        form preposed_as_long_as = "as" "long" "as" condition "," body;
    }
    construction preposed_as_long_as_predicate: ClauseAttachment {
        element PreposedAsLongAsPredicate { condition: Clause, body: Predicate, }
        derive body.concord_class = Values::Other;
        form preposed_as_long_as_predicate = "as" "long" "as" condition "," body;
    }
    construction postposed_as_long_as: ClauseAttachment {
        element PostposedAsLongAs { body: Clause, condition: FiniteClause, }
        form postposed_as_long_as = body "as" "long" "as" condition;
    }
    construction postposed_as_long_as_predicate: ClauseAttachment {
        element PostposedAsLongAsPredicate { body: Predicate, condition: FiniteClause, }
        derive body.concord_class = Values::Other;
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
        form postposed_for_as_long_as_clause =
            body licensed("for") "as" "long" "as" condition;
    }
    construction preposed_predicate_adjunct: ClauseAttachment {
        element PreposedPredicateAdjunct {
            adjunct: PredicateAdjunct,
            body: Clause,
        }
        form preposed_predicate_adjunct = adjunct "," body;
    }
    construction preposed_predicate_adjunct_predicate: ClauseAttachment {
        element PreposedPredicateAdjunctPredicate {
            adjunct: PredicateAdjunct,
            body: Predicate,
        }
        derive body.concord_class = Values::Other;
        form preposed_predicate_adjunct_predicate = adjunct "," body;
    }
    construction preposed_while: ClauseAttachment {
        element PreposedWhile { condition: FiniteClause, body: Clause, }
        form preposed_while = "while" condition "," body;
    }
    construction preposed_while_predicate: ClauseAttachment {
        element PreposedWhilePredicate { condition: FiniteClause, body: Predicate, }
        derive body.concord_class = Values::Other;
        form preposed_while_predicate = "while" condition "," body;
    }
    construction preposed_until: ClauseAttachment {
        element PreposedUntil { condition: FiniteClause, body: Clause, }
        form preposed_until = "until" condition "," body;
    }
    construction preposed_until_predicate: ClauseAttachment {
        element PreposedUntilPredicate { condition: FiniteClause, body: Predicate, }
        derive body.concord_class = Values::Other;
        form preposed_until_predicate = "until" condition "," body;
    }
    construction fixed_duration_phrase: FixedDurationPhrase {
        element FixedDurationPhraseValue {
            endpoint: TemporalEndpoint checked by temporal_endpoint_denotes_a_time(),
        }
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
        derive members.concord_class = Values::Other;
        derive concord_class = members.concord_class;
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
        derive members.concord_class = Values::Other;
        derive concord_class = members.concord_class;
        form bare_then_predicate_sequence = members;
    }
    construction additional_cost: ClauseAttachment {
        element AdditionalCost {
            cost: Head,
            action: InfinitiveComplement,
            body: AdditionalCostBody,
        }
        require cost.number is Singular;
        form additional_cost = "as" "an" "additional" cost action "," body;
    }
    construction additional_cost_predicate_body: AdditionalCostBody {
        element AdditionalCostPredicateBody { predicate: Predicate, }
        derive predicate.concord_class = Values::Other;
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
        derive concord_class = members.concord_class;
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
        derive concord_class = members.concord_class;
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
        derive concord_class = members.concord_class;
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
        derive concord_class = members.concord_class;
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
        derive concord_class = members.concord_class;
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
        derive concord_class = members.concord_class;
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
        derive predicate.concord_class = Values::Other;
        form predicative_ability = "able" licensed("to") predicate;
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
        derive concord_class = Values::Other;
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
            source: opt FrameComplement,
        }
        form passive_movement_predicate =
            verb(head) lex(Preposition::Into) destination marked(Preposition::From, source);
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
        form declared_transitive_passive_from_predicate =
            verb(head) lex(Preposition::From) source;
    }
    construction declared_to_object_passive_predicate: DeclaredToObjectPassivePredicate {
        element DeclaredToObjectPassivePredicateValue {
            head: lex DeclaredToObjectParticipleHead,
            complement: FrameComplement,
        }
        form declared_to_object_passive_predicate =
            verb(head) lex(Preposition::To) complement;
    }
    construction bare_passive_predicate: BarePassivePredicate {
        element BarePassivePredicateValue {
            copula: lex BareCopula,
            predicate: PassivePredicate,
        }
        derive concord_class = Values::Other;
        form bare_passive_predicate = lex(copula) predicate;
    }
    construction inventory_auxiliary: AuxiliaryHead {
        element InventoryAuxiliary { head: lex AuxiliaryVerb, }
        derive concord_class = head.concord_class;
        form inventory_auxiliary = verb(head);
    }
    construction object_infinitive_predicate: ObjectInfinitivePredicate {
        element ObjectInfinitivePredicateValue {
            head: lex ObjectInfinitiveVerb,
            object: Object,
            complement: VerbPhrase,
        }
        derive concord_class = head.concord_class;
        derive complement.concord_class = Values::Other;
        form object_infinitive_predicate = verb(head) object licensed("to") complement;
    }
    construction infinitive_complement: InfinitiveComplement {
        element InfinitiveComplementValue {
            negator: opt lex PredicateNegator,
            predicate: VerbPhrase,
        }
        derive predicate.concord_class = Values::Other;
        form infinitive_complement = lex(negator) licensed("to") predicate;
    }
    construction choose_infinitive_predicate: VerbPhrase {
        element ChooseInfinitivePredicate {
            head: lex ChooseInfinitiveVerb,
            complement: InfinitiveComplement,
        }
        derive concord_class = head.concord_class;
        form choose_infinitive_predicate = verb(head) complement;
    }
    construction requirement_predicate: RequirementPredicate {
        element RequirementPredicateValue {
            head: lex IntransitiveVerb,
            frequency: NounPhrase,
        }
        derive concord_class = head.concord_class;
        form requirement_predicate = verb(head) frequency "if" "able";
    }
    construction transitive_requirement_predicate: TransitiveRequirementPredicate {
        element TransitiveRequirementPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            duration: opt DurationPhrase,
        }
        derive concord_class = head.concord_class;
        form transitive_requirement_predicate =
            verb(head) object duration "if" "able";
    }
    construction as_though_predicate: AsThoughPredicate {
        element AsThoughPredicateValue {
            predicate: LexicalVerbPhrase,
            condition: CounterfactualClause,
        }
        derive concord_class = predicate.concord_class;
        form as_though_predicate = predicate "as" "though" condition;
    }
    construction ordered_predicate: OrderedPredicate {
        element OrderedPredicateValue {
            head: lex OrderedVerb,
            object: Object checked by object_has_no_selected_source_postmodifier(),
            source: opt PrepositionalPhrase,
            destination: FrameComplement,
            order_relation: lex Preposition,
            order: lex ObjectOrder,
        }
        require source.preposition_complement_kind is SourceComplement;
        require order_relation is In;
        derive concord_class = head.concord_class;
        form ordered_predicate =
            verb(head) object source lex(Preposition::On) destination lex(order_relation) lex(order) "order";
    }
    construction irrealis_copular_clause: IrrealisCopularClause {
        element IrrealisCopularClauseValue {
            subject: Subject,
            complement: PredicativeComplement,
        }
        form irrealis_copular_clause = subject lex(FiniteCopula::Were) complement;
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
    construction purpose_predicate_adjunct: PredicateAdjunct {
        element PurposePredicateAdjunct {
            purpose: LexicalVerbPhrase,
        }
        derive purpose.concord_class = Values::Other;
        form purpose_predicate_adjunct = licensed("to") purpose;
    }
    construction duration_predicate_adjunct: PredicateAdjunct {
        element DurationPredicateAdjunct {
            duration: DurationPhrase,
        }
        form duration_predicate_adjunct = duration;
    }
    construction prepositional_predicate_adjunct: PredicateAdjunct {
        element PrepositionalPredicateAdjunct {
            adjunct: PrepositionalPhrase checked by predicate_preposition_is_licensed(
                adjunct.preposition_complement_kind,
                adjunct.locative_temporal_license
            ),
        }
        require adjunct.preposition_attachment is AdjunctCapable;
        form prepositional_predicate_adjunct = adjunct;
    }
    construction predicate_adjunct_predicate: PredicateAdjunctPredicate {
        element PredicateAdjunctPredicateValue {
            predicate: LexicalVerbPhrase,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_nonprepositional(),
        }
        derive concord_class = predicate.concord_class;
        form predicate_adjunct_predicate = predicate adjunct;
    }
    construction prepositional_predicate_adjunct_predicate: PredicateAdjunctPredicate {
        element PrepositionalPredicateAdjunctPredicateValue {
            predicate: PrepositionalPredicateAdjunctHost,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        derive concord_class = predicate.concord_class;
        form prepositional_predicate_adjunct_predicate = predicate adjunct;
    }
    construction stacked_predicate_adjunct_predicate: PredicateAdjunctPredicate {
        element StackedPredicateAdjunctPredicate {
            predicate: LexicalVerbPhrase,
            leading: PredicateAdjunct checked by predicate_adjunct_is_nonprepositional(),
            trailing: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        derive concord_class = predicate.concord_class;
        form stacked_predicate_adjunct_predicate = predicate leading trailing;
    }
    construction passive_duration_predicate_adjunct: PredicateAdjunctPredicate {
        element PassiveDurationPredicateAdjunct {
            predicate: BarePassivePredicate,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_duration(),
        }
        derive concord_class = Values::Other;
        form passive_duration_predicate_adjunct = predicate adjunct;
    }
    construction instead_predicate: InsteadPredicate {
        element InsteadPredicateValue {
            predicate: Predicate,
            replacement: lex DistributionReplacement,
        }
        require replacement is Instead;
        derive concord_class = predicate.concord_class;
        form instead_predicate = predicate lex(replacement);
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
    construction frequency_predicate_adjunct: PredicateAdjunct {
        element FrequencyPredicateAdjunct {
            frequency: FrequencyReference,
        }
        form frequency_predicate_adjunct = frequency;
    }
    construction manner_predicate_adjunct: PredicateAdjunct {
        element MannerPredicateAdjunct {
            manner: MannerReference,
        }
        form manner_predicate_adjunct = manner;
    }
    construction alternative_predicate: AlternativePredicate {
        element AlternativePredicateValue {
            action: VerbPhrase,
            alternative: LexicalVerbPhrase,
        }
        derive action.concord_class = Values::Other;
        derive concord_class = action.concord_class;
        derive alternative.concord_class = Values::Other;
        form alternative_predicate = action "rather" "than" alternative;
    }
    construction without_gerund_object_predicate: WithoutGerundObjectPredicate {
        element WithoutGerundObjectPredicateValue {
            head: lex TransitiveVerb,
            object: Object,
            complement: Object,
        }
        derive concord_class = head.concord_class;
        form without_gerund_object_predicate =
            verb(head) object "without" "paying" complement;
    }
    construction controlled_cost_action: ControlledCostAction {
        element ControlledCostActionValue { head: lex TransitiveVerb, }
        derive head.concord_class = Values::Other;
        form controlled_cost_action = licensed("to") verb(head);
    }
    construction cost_comparison_predicate: CostComparisonPredicate {
        element CostComparisonPredicateValue {
            head: lex CostComparisonVerb,
            amount: ManaAmount,
            direction: lex CostComparisonDirection,
            action: ControlledCostAction,
        }
        derive concord_class = head.concord_class;
        form cost_comparison_predicate = verb(head) amount lex(direction) action;
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
        element OnlyDuringRestriction {
            relation: lex Preposition,
            timing: RestrictionTurn,
        }
        require relation is During;
        form only_during_restriction = "only" lex(relation) timing;
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
        derive _head.concord_class = Values::Other;
        derive concord_class = _head.concord_class;
        form action_restriction_predicate = verb(_head) object restrictions;
    }
    construction finite_passive_predicate: FinitePassivePredicate {
        element FinitePassivePredicateValue {
            copula: lex FiniteCopula,
            predicate: PassivePredicate,
        }
        derive copula.concord_class = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Other,
            Arent => Values::Other,
            Was => Values::ThirdPersonSingular,
            Were => Values::Other,
        };
        derive concord_class = copula.concord_class;
        form finite_passive_predicate = lex(copula) predicate;
    }
    construction auxiliary_predicate: AuxiliaryPredicate {
        element AuxiliaryPredicateValue {
            auxiliary: AuxiliaryHead,
            predicate: BarePredicate,
        }
        derive predicate.concord_class = Values::Other;
        derive concord_class = auxiliary.concord_class;
        form auxiliary_predicate = auxiliary predicate;
    }
    construction finite_copular_predicate: FiniteCopularPredicate {
        element FiniteCopularPredicateValue {
            copula: lex FiniteCopula,
            complement: PredicativeComplement,
        }
        derive copula.concord_class = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Other,
            Arent => Values::Other,
            Was => Values::ThirdPersonSingular,
            Were => Values::Other,
        };
        derive concord_class = copula.concord_class;
        form finite_copular_predicate = lex(copula) complement;
    }
    construction plain_finite_clause: FiniteClause {
        element PlainFiniteClause { subject: Subject, predicate: Predicate, }
        derive predicate.concord_class = subject.concord_class;
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
            lex(subject) verb(head) object lex(Preposition::On) complement;
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
            adjunct: opt ExistentialPredicateAdjunct,
        }
        derive copula.concord_class = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Other,
            Arent => Values::Other,
            Was => Values::ThirdPersonSingular,
            Were => Values::Other,
        };
        derive pivot.concord_class = copula.concord_class;
        form existential_finite_clause = "there" lex(copula) pivot adjunct;
    }
    construction existential_predicate_adjunct: ExistentialPredicateAdjunct {
        element ExistentialPredicateAdjunctValue {
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        form existential_predicate_adjunct = adjunct;
    }
    construction floated_quantifier_finite_clause: FiniteClause {
        element FloatedQuantifierFiniteClause {
            subject: Subject,
            quantifier: lex FloatedQuantifier,
            predicate: Predicate,
        }
        derive subject.concord_class = Values::Other;
        derive predicate.concord_class = Values::Other;
        form floated_quantifier_finite_clause = subject lex(quantifier) predicate;
    }
    construction third_person_auxiliary_finite_clause: FiniteClause {
        element ThirdPersonAuxiliaryFiniteClause {
            subject: Subject,
            auxiliary: lex ThirdPersonAuxiliary,
            predicate: Predicate,
        }
        derive subject.concord_class = Values::ThirdPersonSingular;
        derive predicate.concord_class = Values::Other;
        form third_person_auxiliary_finite_clause = subject lex(auxiliary) predicate;
    }
    construction other_concord_class_auxiliary_finite_clause: FiniteClause {
        element OtherConcordClassAuxiliaryFiniteClause {
            subject: Subject,
            auxiliary: lex OtherConcordClassAuxiliary,
            predicate: Predicate,
        }
        derive subject.concord_class = Values::Other;
        derive predicate.concord_class = Values::Other;
        form other_concord_class_auxiliary_finite_clause = subject lex(auxiliary) predicate;
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
        derive concord_class = value.concord_class;
        derive number = value.number;
        derive onset = value.onset;
        form subject_nominal = value;
    }
    construction subject_pronoun: Subject {
        element PersonalSubject { word: lex SubjectPronoun, }
        derive word.concord_class = match word {
            He => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            She => Values::ThirdPersonSingular,
            They => Values::Other,
            You => Values::Other,
        };
        derive concord_class = word.concord_class;
        derive number = Values::Singular;
        derive onset = word.onset;
        form subject_pronoun = lex(word);
    }
    construction variable_subject: Subject {
        element VariableSubject { variable: lex Variable, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = variable.onset;
        form variable_subject = lex(variable);
    }
    construction object_nominal: Object {
        element NominalObject { value: NounPhrase, }
        require value.fused_head_license in [NominalOnly, FusedHead];
        derive concord_class = value.concord_class;
        derive number = value.number;
        derive onset = value.onset;
        derive relationality = value.relationality;
        derive locative_temporal_license = value.locative_temporal_license;
        form object_nominal = value;
    }
    construction bare_singular_coordination_object: Object {
        element BareSingularCoordinationObject { value: NominalCoordination, }
        require value.number is Singular;
        derive concord_class = value.concord_class;
        derive number = value.number;
        derive onset = value.onset;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form bare_singular_coordination_object = value;
    }
    construction object_pronoun: Object {
        element PersonalObject { word: lex ObjectPronoun, }
        derive concord_class = match word {
            Her => Values::ThirdPersonSingular,
            Him => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            Them => Values::Other,
            You => Values::Other,
        };
        derive number = match word {
            Her => Values::Singular,
            Him => Values::Singular,
            It => Values::Singular,
            Them => Values::Plural,
            You => Values::Singular,
        };
        derive onset = word.onset;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form object_pronoun = lex(word);
    }
    construction reflexive_object: Object {
        element ReflexiveObject { word: lex ReflexivePronoun, }
        derive concord_class = match word {
            Herself => Values::ThirdPersonSingular,
            Himself => Values::ThirdPersonSingular,
            Itself => Values::ThirdPersonSingular,
            Themself => Values::Other,
            Themselves => Values::Other,
            Yourself => Values::Other,
            Yourselves => Values::Other,
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
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form reflexive_object = lex(word);
    }
    construction noun_singular_head: Head {
        element NounSingularHead { noun: lex Noun, }
        require noun.countability is Count;
        derive noun.number = Values::Singular;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
        form noun_singular_head = noun(noun);
    }
    construction noun_plural_head: Head {
        element NounPluralHead { noun: lex Noun, }
        require noun.countability is Count;
        derive noun.number = Values::Plural;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
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
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = color.onset;
        form color_modifier = lex(color);
    }
    construction attributive_adjective_modifier: NominalModifier {
        element AttributiveAdjectiveModifier { adjective: lex AttributiveAdjective, }
        derive modifier_license = adjective.modifier_license;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = match adjective {
            Additional => Values::Vowel,
            Base => Values::Consonant,
            Declare => Values::Consonant,
            FaceDown => Values::Consonant,
            First => Values::Consonant,
            Main => Values::Consonant,
            Maximum => Values::Consonant,
            New => Values::Consonant,
            Next => Values::Consonant,
            Other => Values::Vowel,
            Postcombat => Values::Consonant,
            Precombat => Values::Consonant,
            Same => Values::Consonant,
            Second => Values::Consonant,
            SixSided => Values::Consonant,
            Untap => Values::Vowel,
        };
        form attributive_adjective_modifier = lex(adjective);
    }
    construction targeting_marker_nominal_modifier: NominalModifier {
        element TargetingMarkerNominalModifier { marker: lex TargetingMarker, }
        derive modifier_license = Values::LocalDeterminer;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form targeting_marker_nominal_modifier = lex(marker);
    }
    construction counter_kind_modifier: NominalModifier {
        element CounterKindModifier { kind: CounterKind, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = kind.onset;
        form counter_kind_modifier = kind;
    }
    construction power_toughness_modifier: NominalModifier {
        element PowerToughnessModifier { value: PredicativePowerToughnessComplement, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form power_toughness_modifier = value;
    }
    construction status_modifier: NominalModifier {
        element StatusModifier { status: lex Status, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = status.onset;
        form status_modifier = lex(status);
    }
    construction participial_adjective_modifier: NominalModifier {
        element ParticipialAdjectiveModifier { adjective: ParticipialAdjective, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = adjective.onset;
        form participial_adjective_modifier = adjective;
    }
    construction reduced_relative_modifier: NominalModifier {
        element ReducedRelativeModifier { head: lex DeclaredTransitiveParticipleHead, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = head.onset;
        form reduced_relative_modifier = verb(head);
    }
    construction supertype_modifier: NominalModifier {
        element SupertypeModifier { supertype: lex Supertype, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = supertype.onset;
        form supertype_modifier = lex(supertype);
    }
    construction noun_modifier: NominalModifier {
        element NounModifier { noun: lex Noun, }
        require noun.compoundability is Compoundable;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form noun_modifier = noun(noun);
    }
    construction non_color_modifier: NominalModifier {
        element NonColorModifier { color: lex Color, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_color_modifier = prefix("non", lex(color));
    }
    construction non_noun_modifier: NominalModifier {
        element NonNounModifier { noun: lex Noun, }
        require noun.properness is Common;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_noun_modifier = prefix("non", noun(noun));
    }
    construction non_proper_noun_modifier: NominalModifier {
        element NonProperNounModifier { noun: lex Noun, }
        require noun.properness is Proper;
        derive modifier_license = Values::Unrestricted;
        derive noun.number = Values::Singular;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_proper_noun_modifier = prefix("non-", noun(noun));
    }
    construction non_status_modifier: NominalModifier {
        element NonStatusModifier { status: lex Status, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_status_modifier = prefix("non", lex(status));
    }
    construction non_supertype_modifier: NominalModifier {
        element NonSupertypeModifier { supertype: lex Supertype, }
        derive modifier_license = Values::Unrestricted;
        derive concord_class = Values::ThirdPersonSingular;
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
        derive concord_class = Values::ThirdPersonSingular;
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
        derive concord_class = Values::ThirdPersonSingular;
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
        derive concord_class = Values::ThirdPersonSingular;
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
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form and_or_shared_head_modifier = first "and/or" rest;
    }
    construction bare_singular_nominal: Nominal {
        element BareSingularNominal { head: Head, }
        derive head.number = Values::Singular;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form bare_singular_nominal = head;
    }
    construction bare_plural_nominal: Nominal {
        element BarePluralNominal { head: Head, }
        derive head.number = Values::Plural;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::BarePluralNoun;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form bare_plural_nominal = head;
    }
    construction modified_singular_nominal: Nominal {
        element ModifiedSingularNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: Head,
        }
        derive head.number = Values::Singular;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::ModifiedSingularNoun;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form modified_singular_nominal = first rest head;
    }
    construction modified_plural_nominal: Nominal {
        element ModifiedPluralNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: Head,
        }
        derive head.number = Values::Plural;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::ModifiedPluralNoun;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form modified_plural_nominal = first rest head;
    }
    construction negative_modified_singular_nominal: Nominal {
        element NegativeModifiedSingularNominal {
            leading: opt NominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: Head,
        }
        require len(modifiers) >= 2;
        derive head.number = Values::Singular;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::ModifiedSingularNoun;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form negative_modified_singular_nominal = leading modifiers head;
    }
    construction negative_modified_plural_nominal: Nominal {
        element NegativeModifiedPluralNominal {
            leading: opt NominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: Head,
        }
        require len(modifiers) >= 2;
        derive head.number = Values::Plural;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive nominal_form = Values::ModifiedPluralNoun;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form negative_modified_plural_nominal = leading modifiers head;
    }
    construction bare_relational_reference: UnqualifiedReference {
        element BareRelationalReference { head: lex Noun, }
        require head.relationality is Relational;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
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
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        derive head.number = Values::Singular;
        form modified_bare_relational_reference = first rest noun(head);
    }
    construction participial_singular_reference: UnqualifiedReference {
        element ParticipialSingularReference {
            adjective: ParticipialAdjective,
            head: Head,
        }
        derive concord_class = head.concord_class;
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = adjective.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form participial_singular_reference = adjective head;
    }
    construction premodified_participial_singular_reference: UnqualifiedReference {
        element PremodifiedParticipialSingularReference {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            adjective: ParticipialAdjective,
            head: Head,
        }
        require first.modifier_license is LocalDeterminer;
        derive concord_class = head.concord_class;
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = first.onset;
        derive possessive_ending = head.possessive_ending;
        derive relationality = head.relationality;
        derive locative_temporal_license = head.locative_temporal_license;
        form premodified_participial_singular_reference = first rest adjective head;
    }
    construction bare_coordination_member: CoordinationMember {
        element BareCoordinationMember { head: Head, }
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive onset = head.onset;
        derive possessive_ending = head.possessive_ending;
        form bare_coordination_member = head;
    }
    construction modified_coordination_member: CoordinationMember {
        element ModifiedCoordinationMember {
            modifier: CoordinatedNominalModifier,
            head: Head,
        }
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive onset = modifier.onset;
        derive possessive_ending = head.possessive_ending;
        form modified_coordination_member = modifier head;
    }
    construction negative_modified_coordination_member: CoordinationMember {
        element NegativeModifiedCoordinationMember {
            leading: opt CoordinatedNominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: Head,
        }
        require len(modifiers) >= 2;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive onset = Values::Consonant;
        derive possessive_ending = head.possessive_ending;
        form negative_modified_coordination_member = leading modifiers head;
    }
    construction and_nominal_coordination: NominalCoordination {
        element AndNominalCoordination {
            members: seq CoordinationMember separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive concord_class = members.concord_class;
        derive number = members.number;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form and_nominal_coordination = members;
    }
    construction or_nominal_coordination: NominalCoordination {
        element OrNominalCoordination {
            members: seq CoordinationMember separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive concord_class = members.concord_class;
        derive number = members.number;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form or_nominal_coordination = members;
    }
    construction and_or_nominal_coordination: NominalCoordination {
        element AndOrNominalCoordination {
            members: seq CoordinationMember separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive concord_class = members.concord_class;
        derive number = members.number;
        derive onset = members.onset;
        derive possessive_ending = Values::Other;
        form and_or_nominal_coordination = members;
    }
    construction mass_noun: MassNoun {
        element MassNounValue { noun: lex Noun, }
        require noun.countability is Mass;
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
        form mass_noun = noun(noun);
    }
    construction mass_nominal: Nominal {
        element MassNominal { noun: MassNoun, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::MassNoun;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
        form mass_nominal = noun;
    }
    construction modified_mass_nominal: Nominal {
        element ModifiedMassNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            noun: MassNoun,
        }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::MassNoun;
        derive onset = first.onset;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
        form modified_mass_nominal = first rest noun;
    }
    construction singular_coordination_nominal_value: Nominal {
        element SingularCoordinationNominalValue {
            coordination: NominalCoordination,
        }
        require coordination.number is Singular;
        derive concord_class = coordination.concord_class;
        derive number = coordination.number;
        derive nominal_form = Values::SingularCoordination;
        derive onset = coordination.onset;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form singular_coordination_nominal_value = coordination;
    }
    construction plural_coordination_nominal_value: Nominal {
        element PluralCoordinationNominalValue {
            coordination: NominalCoordination,
        }
        require coordination.number is Plural;
        derive concord_class = coordination.concord_class;
        derive number = coordination.number;
        derive nominal_form = Values::PluralCoordination;
        derive onset = coordination.onset;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form plural_coordination_nominal_value = coordination;
    }
    construction modified_singular_coordination_nominal_value: Nominal {
        element ModifiedSingularCoordinationNominalValue {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            coordination: NominalCoordination,
        }
        require coordination.number is Singular;
        derive concord_class = coordination.concord_class;
        derive number = coordination.number;
        derive nominal_form = Values::SingularCoordination;
        derive onset = first.onset;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form modified_singular_coordination_nominal_value = first rest coordination;
    }
    construction modified_plural_coordination_nominal_value: Nominal {
        element ModifiedPluralCoordinationNominalValue {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            coordination: NominalCoordination,
        }
        require coordination.number is Plural;
        derive concord_class = coordination.concord_class;
        derive number = coordination.number;
        derive nominal_form = Values::PluralCoordination;
        derive onset = first.onset;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form modified_plural_coordination_nominal_value = first rest coordination;
    }
    construction unmarked_singular_selector: SingularSelector {
        element UnmarkedSingularSelector { nominal: Nominal, }
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        derive concord_class = nominal.concord_class;
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
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = head.determiner_number;
        derive nominal_license = head.nominal_license;
        derive fused_head_license = head.fused_head_license;
        derive onset = head.onset;
        form singular_simple_determinative = lex(head);
    }
    construction targeting_marker_determinative: Determinative {
        element TargetingMarkerDeterminative { marker: lex TargetingMarker, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form targeting_marker_determinative = lex(marker);
    }
    construction plural_simple_determinative: Determinative {
        element PluralSimpleDeterminative {
            head: lex DeterminativeHead checked by determinative_licenses_plural(
                head.determiner_number
            ),
        }
        derive concord_class = Values::Other;
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
        derive concord_class = count.concord_class;
        derive number = count.number;
        derive determiner_number = count.determiner_number;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form cardinal_quantifying_determiner = count;
    }
    construction mass_quantity_determiner: Determinative {
        element MassQuantityDeterminer { amount: Amount, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_quantity_determiner = amount;
    }
    construction mass_comparison_determiner: Determinative {
        element MassComparisonDeterminer { comparison: ScalarComparison, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_comparison_determiner = comparison;
    }
    construction mass_cardinal_quantity_determiner: Determinative {
        element MassCardinalQuantityDeterminer { count: CardinalQuantity, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive determiner_number = Values::SingularOnly;
        derive nominal_license = Values::MassOrPluralCount;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        form mass_cardinal_quantity_determiner = count;
    }
    construction variable_quantifying_determiner: Determinative {
        element VariableQuantifyingDeterminer { count: lex Variable, }
        derive concord_class = Values::Other;
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
        derive concord_class = count.concord_class;
        derive number = count.number;
        derive determiner_number = count.determiner_number;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Vowel;
        form up_to_quantifying_determiner = "up" "to" count;
    }
    construction any_number_quantifying_determiner: Determinative {
        element AnyNumberQuantifyingDeterminer {
            order: lex ObjectOrder,
            unit: Head,
            relation: lex Preposition,
        }
        require unit.number is Singular;
        require order is Any;
        require relation is Of;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Vowel;
        form any_number_quantifying_determiner = lex(order) unit lex(relation);
    }
    construction no_more_quantifying_determiner: Determinative {
        element NoMoreQuantifyingDeterminer { direction: lex CostComparisonDirection, }
        require direction is More;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form no_more_quantifying_determiner = "no" lex(direction);
    }
    construction counted_quantifying_determiner: Determinative {
        element CountedQuantifyingDeterminer { count: CountReference, }
        derive concord_class = count.concord_class;
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
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive determiner_number = Values::PluralOnly;
        derive nominal_license = Values::CountNominal;
        derive fused_head_license = Values::FusedHead;
        derive onset = Values::Consonant;
        form count_comparison_quantifying_determiner = count comparison;
    }
    construction named_card_reference: UnqualifiedReference {
        element NamedCardReference { kind: Head, name: identity CardName, }
        require kind.number is Singular;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        derive possessive_ending = name.possessive_ending;
        derive relationality = kind.relationality;
        derive locative_temporal_license = kind.locative_temporal_license;
        form named_card_reference = "a" kind "named" identity(name);
    }
    construction definite_next_mass_quantity_reference: UnqualifiedReference {
        element DefiniteNextMassQuantityReference {
            quantity: Amount,
            noun: MassNoun,
        }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        derive possessive_ending = noun.possessive_ending;
        derive relationality = noun.relationality;
        derive locative_temporal_license = noun.locative_temporal_license;
        form definite_next_mass_quantity_reference = "the" "next" quantity noun;
    }
    construction that_many: CountReference {
        element ThatMany { demonstrative: lex SingularDemonstrative, }
        require demonstrative is That;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form that_many = lex(demonstrative) "many";
    }
    construction demonstrative_possessive_reference: UnqualifiedReference {
        element DemonstrativePossessiveReference {
            demonstrative: lex SingularDemonstrative,
            possessor: Possessive,
            possessed: Nominal,
        }
        require possessor.number is Singular;
        require possessed.number is Singular;
        require possessed.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        derive concord_class = possessed.concord_class;
        derive number = possessed.number;
        derive onset = Values::Consonant;
        derive possessive_ending = possessed.possessive_ending;
        derive relationality = possessed.relationality;
        derive locative_temporal_license = possessed.locative_temporal_license;
        form demonstrative_possessive_reference = lex(demonstrative) possessor possessed;
    }
    construction possessed_singular_reference: UnqualifiedReference {
        element PossessedSingularReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: Nominal,
        }
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form possessed_singular_reference = lex(possessor) nominal;
    }
    construction possessed_plural_reference: UnqualifiedReference {
        element PossessedPluralReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: Nominal,
        }
        require nominal.number is Plural;
        require nominal.nominal_form in [BarePluralNoun, ModifiedPluralNoun];
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form possessed_plural_reference = lex(possessor) nominal;
    }
    construction possessed_mass_reference: UnqualifiedReference {
        element PossessedMassReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: Nominal,
        }
        require nominal.nominal_form is MassNoun;
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = possessor.onset;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form possessed_mass_reference = lex(possessor) nominal;
    }
    construction genitive_determiner_reference: UnqualifiedReference {
        element GenitiveDeterminerReference {
            possessor: Possessive,
            nominal: Nominal,
        }
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form genitive_determiner_reference = possessor nominal;
    }
    construction genitive_determiner_plural_reference: UnqualifiedReference {
        element GenitiveDeterminerPluralReference {
            possessor: Possessive,
            nominal: Nominal,
        }
        require nominal.number is Plural;
        require nominal.nominal_form in [BarePluralNoun, ModifiedPluralNoun];
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form genitive_determiner_plural_reference = possessor nominal;
    }
    construction genitive_determiner_mass_reference: UnqualifiedReference {
        element GenitiveDeterminerMassReference {
            possessor: Possessive,
            nominal: Nominal,
        }
        require nominal.nominal_form is MassNoun;
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form genitive_determiner_mass_reference = possessor nominal;
    }
    construction genitive_determiner_coordination_reference: UnqualifiedReference {
        element GenitiveDeterminerCoordinationReference {
            possessor: Possessive,
            coordination: NominalCoordination,
        }
        require coordination.number is Singular;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form genitive_determiner_coordination_reference = possessor coordination;
    }
    construction possessive_absolute_reference: UnqualifiedReference {
        element PossessiveAbsoluteReference { word: lex PossessiveAbsolutePronoun, }
        derive concord_class = match word {
            Hers => Values::ThirdPersonSingular,
            His => Values::ThirdPersonSingular,
            Theirs => Values::Other,
            Yours => Values::Other,
        };
        derive number = match word {
            Hers => Values::Singular,
            His => Values::Singular,
            Theirs => Values::Singular,
            Yours => Values::Singular,
        };
        derive onset = word.onset;
        derive possessive_ending = word.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
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
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form determined_nominal = det nominal;
    }
    construction all_predetermined_nominal: UnqualifiedReference {
        element AllPredeterminedNominal {
            predeterminer: Determinative,
            det: Determinative checked by headed_determiner_licenses_nominal(
                det.determiner_number,
                det.nominal_license,
                nominal.number,
                nominal.nominal_form
            ),
            nominal: Nominal,
        }
        require predeterminer.fused_head_license is PluralPredeterminer;
        require predeterminer.number is Plural;
        derive concord_class = nominal.concord_class;
        derive number = nominal.number;
        derive onset = nominal.onset;
        derive possessive_ending = nominal.possessive_ending;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form all_predetermined_nominal = predeterminer det nominal;
    }
    construction full_and_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndNounPhraseCoordination {
            members: seq PostmodifiedReference separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive possessive_ending = members.possessive_ending;
        form full_and_noun_phrase_coordination = members;
    }
    construction full_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullOrNounPhraseCoordination {
            members: seq PostmodifiedReference separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive possessive_ending = members.possessive_ending;
        form full_or_noun_phrase_coordination = members;
    }
    construction full_and_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndOrNounPhraseCoordination {
            members: seq PostmodifiedReference separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Plural;
        derive possessive_ending = members.possessive_ending;
        form full_and_or_noun_phrase_coordination = members;
    }
    construction coordinated_noun_phrase: UnqualifiedReference {
        element CoordinatedNounPhrase {
            coordination: FullNounPhraseCoordination checked by full_coordination_is_independent(),
        }
        derive concord_class = coordination.concord_class;
        derive number = coordination.number;
        derive onset = Values::Consonant;
        derive possessive_ending = coordination.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form coordinated_noun_phrase = coordination;
    }
    construction locative_and_noun_phrase_coordination: LocativeNounPhraseCoordination {
        element LocativeAndNounPhraseCoordination {
            members: seq PostmodifiedReference separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        form locative_and_noun_phrase_coordination = members;
    }
    construction locative_coordinated_noun_phrase: NounPhrase {
        element LocativeCoordinatedNounPhrase {
            coordination: LocativeNounPhraseCoordination checked by
                locative_coordination_has_modifier(),
        }
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form locative_coordinated_noun_phrase = coordination;
    }
    construction self_reference: UnqualifiedReference {
        element SourceSelfReference { spelling: identity SelfReferenceSpelling, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = spelling.onset;
        derive possessive_ending = spelling.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form self_reference = identity(spelling);
    }
    construction this_way: MannerReference {
        element ThisWay {
            demonstrative: lex SingularDemonstrative,
            noun: lex Noun,
        }
        require demonstrative is This;
        require noun.manner_anaphor_class is MannerAnaphor;
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form this_way = lex(demonstrative) noun(noun);
    }
    construction at_random_manner: MannerReference {
        element AtRandomManner {}
        derive number = Values::Singular;
        form at_random_manner = licensed("at") "random";
    }
    construction that_much: ScalarReference {
        element ThatMuch { demonstrative: lex SingularDemonstrative, }
        require demonstrative is That;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form that_much = lex(demonstrative) "much";
    }
    construction positive_object_gap_relative: PositiveObjectGapRelativeClause {
        element PositiveObjectGapRelativeClauseValue {
            subject: Subject,
            head: lex TransitiveVerb,
        }
        derive head.concord_class = subject.concord_class;
        form positive_object_gap_relative = subject verb(head);
    }
    construction positive_object_gap_relative_with_adjunct: PositiveObjectGapRelativeClause {
        element PositiveObjectGapRelativeWithAdjunct {
            subject: Subject,
            head: lex TransitiveVerb,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_nonprepositional(),
        }
        derive head.concord_class = subject.concord_class;
        form positive_object_gap_relative_with_adjunct = subject verb(head) adjunct;
    }
    construction positive_object_gap_relative_with_prepositional_adjunct: PositiveObjectGapRelativeClause {
        element PositiveObjectGapRelativeWithPrepositionalAdjunct {
            subject: Subject,
            head: lex TransitiveVerb,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        derive head.concord_class = subject.concord_class;
        form positive_object_gap_relative_with_prepositional_adjunct = subject verb(head) adjunct;
    }
    construction auxiliary_object_gap_relative: AuxiliaryObjectGapRelativeClause {
        element AuxiliaryObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: AuxiliaryHead,
            head: lex TransitiveVerb,
        }
        derive auxiliary.concord_class = subject.concord_class;
        derive head.concord_class = Values::Other;
        form auxiliary_object_gap_relative = subject auxiliary verb(head);
    }
    construction contracted_perfect_object_gap_relative: ContractedPerfectObjectGapRelativeClause {
        element ContractedPerfectObjectGapRelativeClauseValue {
            subject: lex ContractedPerfectSubject,
            head: lex DeclaredTransitiveParticipleHead,
        }
        form contracted_perfect_object_gap_relative = lex(subject) verb(head);
    }
    construction contracted_perfect_object_gap_relative_with_adjunct: ContractedPerfectAdjunctObjectGapRelativeClause {
        element ContractedPerfectAdjunctObjectGapRelativeClauseValue {
            subject: lex ContractedPerfectSubject,
            head: lex DeclaredTransitiveParticipleHead,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_nonprepositional(),
        }
        form contracted_perfect_object_gap_relative_with_adjunct = lex(subject) verb(head) adjunct;
    }
    construction contracted_perfect_object_gap_relative_with_prepositional_adjunct: ContractedPerfectAdjunctObjectGapRelativeClause {
        element ContractedPerfectPrepositionalAdjunctObjectGapRelativeClauseValue {
            subject: lex ContractedPerfectSubject,
            head: lex DeclaredTransitiveParticipleHead,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        form contracted_perfect_object_gap_relative_with_prepositional_adjunct = lex(subject) verb(head) adjunct;
    }
    construction bare_negative_object_gap_relative: BareNegativeObjectGapRelativeClause {
        element BareNegativeObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: lex OtherConcordClassAuxiliary,
            head: lex TransitiveVerb,
        }
        derive subject.concord_class = Values::Other;
        derive head.concord_class = Values::Other;
        form bare_negative_object_gap_relative = subject lex(auxiliary) verb(head);
    }
    construction third_person_negative_object_gap_relative: ThirdPersonNegativeObjectGapRelativeClause {
        element ThirdPersonNegativeObjectGapRelativeClauseValue {
            subject: Subject,
            auxiliary: lex ThirdPersonAuxiliary,
            head: lex TransitiveVerb,
        }
        derive subject.concord_class = Values::ThirdPersonSingular;
        derive head.concord_class = Values::Other;
        form third_person_negative_object_gap_relative = subject lex(auxiliary) verb(head);
    }
    construction singular_partitive_selection: PartitiveSelection {
        element SingularPartitiveSelection { nominal: Nominal, }
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form singular_partitive_selection = nominal;
    }
    construction fixed_partitive_selection: PartitiveSelection {
        element FixedPartitiveSelection {
            count: CardinalQuantity,
            nominal: Nominal,
        }
        require count.cardinality is TwoPlus;
        require nominal.number is Plural;
        require nominal.nominal_form in [BarePluralNoun, ModifiedPluralNoun];
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        form fixed_partitive_selection = count nominal;
    }
    // The single prepositional phrase. Temporal, locative, source, goal and
    // manner readings are values of the preposition, never categories.
    construction prepositional_phrase: PrepositionalPhrase {
        element PrepositionalPhraseValue {
            preposition: lex Preposition,
            complement: PrepositionalComplement checked by preposition_complement_is_licensed(
                preposition.preposition_complement_kind,
                complement.locative_temporal_license
            ),
        }
        derive bare_locative_complement = preposition.bare_locative_complement;
        derive preposition_attachment = preposition.preposition_attachment;
        derive preposition_complement_kind = preposition.preposition_complement_kind;
        derive relationality = complement.relationality;
        derive locative_temporal_license = complement.locative_temporal_license;
        form prepositional_phrase = lex(preposition) complement;
    }
    // A determiner-less locative is licensed by the preposition class as well
    // as by the noun's own bare-locative license.
    construction bare_locative_prepositional_phrase: PrepositionalPhrase {
        element BareLocativePrepositionalPhrase {
            preposition: lex Preposition,
            complement: BareLocative checked by bare_preposition_complement_is_licensed(
                preposition.preposition_complement_kind
            ),
        }
        require preposition.bare_locative_complement is Yes;
        derive bare_locative_complement = preposition.bare_locative_complement;
        derive preposition_attachment = preposition.preposition_attachment;
        derive preposition_complement_kind = preposition.preposition_complement_kind;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form bare_locative_prepositional_phrase = lex(preposition) complement;
    }
    // H2 forbids a recursively nested PP complement, but Oracle text has a
    // productive flat source-selection relation, "from among <objects>".
    // Keep both prepositions as declared lexical data in one construction.
    construction from_among_prepositional_phrase: PrepositionalPhrase {
        element FromAmongPrepositionalPhrase {
            source_relation: lex Preposition,
            selection_relation: lex Preposition,
            complement: Object,
        }
        require source_relation is From;
        require selection_relation is Among;
        derive bare_locative_complement = source_relation.bare_locative_complement;
        derive preposition_attachment = source_relation.preposition_attachment;
        derive preposition_complement_kind = source_relation.preposition_complement_kind;
        derive relationality = complement.relationality;
        derive locative_temporal_license = complement.locative_temporal_license;
        form from_among_prepositional_phrase =
            lex(source_relation) lex(selection_relation) complement;
    }
    construction edge_of_phrase: EdgeOfPhrase {
        element EdgeOfPhraseValue {
            definiteness: opt lex DefiniteMarker,
            position: lex EdgePosition,
            relation: lex Preposition,
            whole: Object,
        }
        require relation is Of;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = whole.locative_temporal_license;
        form edge_of_phrase = lex(definiteness) lex(position) lex(relation) whole;
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
        element NominalScalarMeasure { nominal: Nominal, }
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        form nominal_scalar_measure = nominal;
    }
    construction scalar_or_less: ScalarComparison {
        element ScalarOrLess {
            threshold: ScalarThreshold,
            direction: lex CostComparisonDirection,
        }
        require direction is Less;
        form scalar_or_less = threshold "or" lex(direction);
    }
    construction scalar_or_greater: ScalarComparison {
        element ScalarOrGreater {
            threshold: ScalarThreshold,
            degree: lex ScalarDegree,
        }
        require degree is Greater;
        form scalar_or_greater = threshold "or" lex(degree);
    }
    construction scalar_less_than: ScalarComparison {
        element ScalarLessThan {
            direction: lex CostComparisonDirection,
            threshold: ScalarThreshold,
        }
        require direction is Less;
        form scalar_less_than = lex(direction) "than" threshold;
    }
    construction scalar_greater_than: ScalarComparison {
        element ScalarGreaterThan {
            degree: lex ScalarDegree,
            threshold: ScalarThreshold,
        }
        require degree is Greater;
        form scalar_greater_than = lex(degree) "than" threshold;
    }
    construction scalar_less_than_or_equal_to: ScalarComparison {
        element ScalarLessThanOrEqualTo {
            direction: lex CostComparisonDirection,
            degree: lex ScalarDegree,
            threshold: ScalarThreshold,
        }
        require direction is Less;
        require degree is Equal;
        form scalar_less_than_or_equal_to =
            lex(direction) "than" "or" lex(degree) "to" threshold;
    }
    construction count_or_more: CountComparison {
        element CountOrMore { quantifier: lex ComparativeQuantifier, }
        require quantifier is More;
        form count_or_more = "or" lex(quantifier);
    }
    construction count_or_fewer: CountComparison {
        element CountOrFewer { quantifier: lex ComparativeQuantifier, }
        require quantifier is Fewer;
        form count_or_fewer = "or" lex(quantifier);
    }
    construction count_or_both: CountComparison {
        element CountOrBoth { quantifier: lex FloatedQuantifier, }
        require quantifier is Both;
        form count_or_both = "or" lex(quantifier);
    }
    construction scalar_measure_value: ScalarMeasureValue {
        element ScalarMeasureValueValue {
            measure: ScalarMeasure,
            value: ScalarMeasureAssignedValue,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form scalar_measure_value = measure value;
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
    construction degree_measure: DegreeMeasure {
        element DegreeMeasureValue {
            degree: ScalarDegreePhrase,
            measure: ScalarMeasure,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form degree_measure = degree measure;
    }
    construction power_toughness_value: PowerToughnessValue {
        element PowerToughnessValueValue {
            characteristics: Nominal,
            fixed: opt PredicativePowerToughnessComplement,
            equality: opt ScalarEquality,
        }
        require any(
            all(fixed.is_some(), equality.is_none()),
            all(fixed.is_none(), equality.is_some())
        );
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form fixed_power_toughness when fixed.is_some() =
            characteristics fixed equality;
        form equal_power_toughness otherwise =
            characteristics lex(FloatedQuantifier::Each) fixed equality;
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
        form genitive_scalar_value = possessor measure;
    }
    construction number_of_scalar_value: ScalarValue {
        element NumberOfScalarValue {
            measure: Head,
            relation: lex Preposition,
            counted: Object,
        }
        require measure.number is Singular;
        require relation is Of;
        form number_of_scalar_value = "the" measure lex(relation) counted;
    }
    construction twice_scalar_value: ScalarValue {
        element TwiceScalarValue {
            adverb: lex FrequencyAdverb,
            value: ScalarValue,
        }
        require adverb is Twice;
        form twice_scalar_value = lex(adverb) value;
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
            relation: lex Preposition,
            domain: Object,
        }
        require relation is Among;
        form greatest_scalar_value = "the" "greatest" measure lex(relation) domain;
    }
    construction scalar_equality: ScalarEquality {
        element ScalarEqualityValue {
            degree: lex ScalarDegree,
            value: ScalarValue,
        }
        require degree is Equal;
        form scalar_equality = lex(degree) lex(Preposition::To) value;
    }
    construction unqualified_postmodified_reference: PostmodifiedReference {
        element UnqualifiedPostmodifiedReference { reference: UnqualifiedReference, }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form unqualified_postmodified_reference = reference;
    }
    construction relative_qualified_reference: PostmodifiedReference {
        element RelativeQualifiedReference {
            reference: PostmodifiedReference,
            clause: ObjectGapRelativeClause,
        }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form relative_qualified_reference = reference clause;
    }
    construction subject_relative_qualified_reference: PostmodifiedReference {
        element SubjectRelativeQualifiedReference {
            reference: PostmodifiedReference,
            clause: SubjectGapRelativeClause,
        }
        derive concord_class = reference.concord_class;
        derive clause.concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form subject_relative_qualified_reference = reference clause;
    }
    construction contracted_copular_relative_reference: PostmodifiedReference {
        element ContractedCopularRelativeReference {
            reference: PostmodifiedReference,
            nominal: Nominal,
            relation: lex Preposition,
            complement: Object,
        }
        require reference.number is Singular;
        require nominal.number is Singular;
        require nominal.nominal_form in [BareSingularNoun, ModifiedSingularNoun];
        require relation is Of;
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = Values::Other;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form contracted_copular_relative_reference =
            reference "that's" "a" nominal lex(relation) complement;
    }
    construction reduced_passive_qualified_reference: PostmodifiedReference {
        element ReducedPassiveQualifiedReference {
            reference: PostmodifiedReference,
            clause: PassivePredicate,
        }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form reduced_passive_qualified_reference = reference clause;
    }
    construction reduced_passive_adjunct_qualified_reference: PostmodifiedReference {
        element ReducedPassiveAdjunctQualifiedReference {
            reference: PostmodifiedReference,
            clause: PassivePredicate,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_nonprepositional(),
        }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form reduced_passive_adjunct_qualified_reference = reference clause adjunct;
    }
    construction reduced_passive_prepositional_adjunct_qualified_reference: PostmodifiedReference {
        element ReducedPassivePrepositionalAdjunctQualifiedReference {
            reference: PostmodifiedReference,
            clause: PassivePredicate,
            adjunct: PredicateAdjunct checked by predicate_adjunct_is_prepositional(),
        }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form reduced_passive_prepositional_adjunct_qualified_reference = reference clause adjunct;
    }
    construction other_than_qualified_reference: PostmodifiedReference {
        element OtherThanQualifiedReference {
            reference: PostmodifiedReference,
            excluded: UnqualifiedReference,
        }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = excluded.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form other_than_qualified_reference = reference "other" "than" excluded;
    }
    // Low attachment: a prepositional phrase postmodifies the nearest
    // nominal that licenses it, per the derived-attachment ruling.
    construction prepositional_qualified_reference: PostmodifiedReference {
        element PrepositionalQualifiedReference {
            reference: PostmodifiedReference,
            modifier: PrepositionalPhrase checked by nominal_nonrelational_preposition_is_licensed(
                reference.relationality,
                reference.locative_temporal_license,
                modifier.preposition_attachment,
                modifier.preposition_complement_kind,
                modifier.relationality,
                modifier.locative_temporal_license
            ),
        }
        require modifier.preposition_attachment in [
            AdjunctCapable,
            PostmodifierOnly
        ];
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form prepositional_qualified_reference = reference modifier;
    }
    construction relational_qualified_reference: PostmodifiedReference {
        element RelationalQualifiedReference {
            reference: PostmodifiedReference,
            modifier: PrepositionalPhrase checked by nominal_relational_preposition_is_licensed(
                reference.relationality,
                reference.locative_temporal_license,
                modifier.preposition_attachment,
                modifier.preposition_complement_kind,
                modifier.relationality,
                modifier.locative_temporal_license
            ),
        }
        require modifier.preposition_attachment in [
            AdjunctCapable,
            PostmodifierOnly
        ];
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive onset = reference.onset;
        derive possessive_ending = reference.possessive_ending;
        derive relationality = Values::SaturatedRelational;
        derive locative_temporal_license = reference.locative_temporal_license;
        form relational_qualified_reference = reference modifier;
    }
    construction qualified_noun_phrase: NounPhrase {
        element QualifiedNounPhrase { reference: PostmodifiedReference, }
        derive concord_class = reference.concord_class;
        derive number = reference.number;
        derive fused_head_license = Values::NominalOnly;
        derive onset = reference.onset;
        derive relationality = reference.relationality;
        derive locative_temporal_license = reference.locative_temporal_license;
        form qualified_noun_phrase = reference;
    }
    construction comparative_quantified_reference: NounPhrase {
        element ComparativeQuantifiedReference {
            quantifier: lex ComparativeQuantifier,
            nominal: Nominal,
            standard: Object,
        }
        require nominal.number is Plural;
        require nominal.nominal_form in [BarePluralNoun, ModifiedPluralNoun];
        derive concord_class = Values::Other;
        derive number = Values::Plural;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        derive relationality = nominal.relationality;
        derive locative_temporal_license = nominal.locative_temporal_license;
        form comparative_quantified_reference = lex(quantifier) nominal "than" standard;
    }
    construction fused_determinative_reference: NounPhrase {
        element FusedDeterminativeReference { head: Determinative, }
        require head.fused_head_license in [FusedHead, PluralPredeterminer];
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive fused_head_license = head.fused_head_license;
        derive onset = head.onset;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form fused_determinative_reference = head;
    }
    construction determinative_partitive: UnqualifiedReference {
        element DeterminativePartitive {
            head: Determinative checked by determinative_is_fused(
                head.fused_head_license
            ),
            relation: lex Preposition,
            whole: Object checked by partitive_whole_is_licensed(
                head.determiner_number,
                head.number,
                whole.number
            ),
        }
        require relation is Of;
        derive concord_class = head.concord_class;
        derive number = head.number;
        derive onset = head.onset;
        derive possessive_ending = Values::Other;
        derive relationality = whole.relationality;
        derive locative_temporal_license = whole.locative_temporal_license;
        form determinative_partitive = head lex(relation) whole;
    }
    construction positional_partitive: NounPhrase {
        element PositionalPartitive {
            position: lex EdgePosition,
            selection: PartitiveSelection,
            relation: lex Preposition,
            whole: Object,
        }
        require relation is Of;
        derive concord_class = selection.concord_class;
        derive number = selection.number;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form positional_partitive = "the" lex(position) selection lex(relation) whole;
    }
    construction singular_common_noun_choice: CommonNounChoice {
        element SingularCommonNounChoice { noun: lex Noun, }
        derive noun.number = Values::Singular;
        derive number = Values::Singular;
        form singular_common_noun_choice = noun(noun);
    }
    construction plural_common_noun_choice: CommonNounChoice {
        element PluralCommonNounChoice {
            noun: lex Noun checked by noun_has_distinct_number_surfaces(),
        }
        derive noun.number = Values::Plural;
        derive number = Values::Plural;
        form plural_common_noun_choice = noun(noun);
    }
    construction common_noun_choice_list: NounPhrase {
        element CommonNounChoiceList {
            choices: seq CommonNounChoice separated by " or ",
        }
        require len(choices) >= 2;
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive fused_head_license = Values::NominalOnly;
        derive onset = Values::Consonant;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form common_noun_choice_list = choices;
    }
    construction fused_color_nominal: Nominal {
        element FusedColorNominal { color: lex Color, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = color.onset;
        derive possessive_ending = color.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form fused_color_nominal = lex(color);
    }
    construction indefinite_pronoun_nominal: Nominal {
        element IndefinitePronounNominal { pronoun: lex IndefinitePronoun, }
        derive concord_class = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive nominal_form = Values::BareSingularNoun;
        derive onset = pronoun.onset;
        derive possessive_ending = pronoun.possessive_ending;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::OfInAndOnLicensed;
        form indefinite_pronoun_nominal = lex(pronoun);
    }
    construction possessive_plural_noun: PossessiveOwner {
        element PossessiveNoun { head: Head, }
        require head.number is Plural;
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
        element PossessiveSingularNominal {
            head: lex Noun,
        }
        require all(
            head.countability is Count,
            head.properness is Proper
        );
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive possessive_ending = head.possessive_ending;
        form possessive_singular_nominal = noun(head);
    }
    construction possessive_modified_singular_nominal: PossessiveOwner {
        element PossessiveModifiedSingularNominal {
            first: NominalModifier,
            rest: seq NominalModifier separated by " ",
            head: lex Noun,
        }
        require all(
            head.countability is Count,
            head.properness is Proper
        );
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive possessive_ending = head.possessive_ending;
        form possessive_modified_singular_nominal = first rest noun(head);
    }
    construction possessive_negative_modified_singular_nominal: PossessiveOwner {
        element PossessiveNegativeModifiedSingularNominal {
            leading: opt NominalModifier,
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: lex Noun,
        }
        require len(modifiers) >= 2;
        require all(
            head.countability is Count,
            head.properness is Proper
        );
        derive head.number = Values::Singular;
        derive number = Values::Singular;
        derive possessive_ending = head.possessive_ending;
        form possessive_negative_modified_singular_nominal = leading modifiers noun(head);
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
        element BaseVerbPhrase { frame: LexicalVerbPhrase, }
        derive concord_class = frame.concord_class;
        form base_verb_phrase = frame;
    }
    construction pro_verb_predicate: VerbPhrase {
        element ProVerbPredicate { head: lex ProVerbHead, }
        derive concord_class = head.concord_class;
        form pro_verb_predicate = verb(head);
    }
    construction declared_object_predicative_verb_phrase: VerbPhrase {
        element DeclaredObjectPredicativeVerbPhrase {
            head: lex ObjectPredicativeComplementVerb,
            object: Object,
            complement: PredicativeComplement,
        }
        derive concord_class = head.concord_class;
        form declared_object_predicative_verb_phrase = verb(head) object complement;
    }
    construction intransitive_predicate: IntransitiveLexicalVerbPhrase {
        element IntransitivePredicate { head: lex IntransitiveVerb, }
        derive concord_class = head.concord_class;
        form intransitive_predicate = verb(head);
    }
    construction transitive_predicate: TransitiveLexicalVerbPhrase {
        element TransitivePredicate { head: lex TransitiveVerb, object: Object, }
        derive concord_class = head.concord_class;
        form transitive_predicate = verb(head) object;
    }
    construction measure_complement_predicate: MeasureComplementLexicalVerbPhrase {
        element MeasureComplementPredicate { head: lex MeasureComplementVerb, amount: Amount, }
        derive concord_class = head.concord_class;
        form measure_complement_predicate = verb(head) amount;
    }
    construction declared_object_amount_lexical_verb_phrase: ObjectAmountLexicalVerbPhrase {
        element DeclaredObjectAmountLexicalVerbPhrase {
            head: lex ObjectAmountVerb,
            object: Object,
            amount: Amount,
        }
        derive concord_class = head.concord_class;
        form declared_object_amount_lexical_verb_phrase = verb(head) object amount;
    }
    construction declared_with_object_lexical_verb_phrase: WithObjectLexicalVerbPhrase {
        element DeclaredWithObjectLexicalVerbPhrase {
            head: lex WithObjectVerb,
            object: Object,
        }
        derive concord_class = head.concord_class;
        form declared_with_object_lexical_verb_phrase = verb(head) lex(Preposition::With) object;
    }
    construction declared_object_with_object_lexical_verb_phrase: ObjectWithObjectLexicalVerbPhrase {
        element DeclaredObjectWithObjectLexicalVerbPhrase {
            head: lex ObjectWithObjectVerb,
            object: Object,
            complement: Object,
        }
        derive concord_class = head.concord_class;
        form declared_object_with_object_lexical_verb_phrase = verb(head) object lex(Preposition::With) complement;
    }
    construction declared_object_for_object_lexical_verb_phrase: ObjectForObjectLexicalVerbPhrase {
        element DeclaredObjectForObjectLexicalVerbPhrase {
            head: lex ObjectForObjectVerb,
            object: Object,
            complement: Object,
        }
        derive concord_class = head.concord_class;
        form declared_object_for_object_lexical_verb_phrase =
            verb(head) object lex(Preposition::For) complement;
    }
    construction declared_object_into_object_lexical_verb_phrase: ObjectIntoObjectLexicalVerbPhrase {
        element DeclaredObjectIntoObjectLexicalVerbPhrase {
            head: lex ObjectIntoObjectVerb,
            object: Object,
            destination: Object,
        }
        derive concord_class = head.concord_class;
        form declared_object_into_object_lexical_verb_phrase =
            verb(head) object lex(Preposition::Into) destination;
    }
    construction object_distribution_recipient: DistributionRecipient {
        element ObjectDistributionRecipient { object: Object, }
        form object_distribution_recipient = object;
    }
    construction chosen_distribution_phrase: DistributionPhrase {
        element ChosenDistributionPhrase {
            head: lex AmongObjectVerb,
            relation: lex Preposition,
            recipient: Object,
        }
        derive head.concord_class = Values::Other;
        require relation is Among;
        form chosen_distribution_phrase =
            "divided" "as" licensed("you") verb(head) lex(relation) recipient;
    }
    construction even_distribution_phrase: DistributionPhrase {
        element EvenDistributionPhrase {
            relation: lex Preposition,
            recipient: DistributionRecipient,
        }
        require relation is Among;
        form even_distribution_phrase =
            "divided" "evenly" "," "rounded" "down" "," lex(relation) recipient;
    }
    construction modal_passive_subject_gap_relative_clause: ModalPassiveSubjectGapRelativeClause {
        element ModalPassiveSubjectGapRelativeClauseValue {
            auxiliary: AuxiliaryHead,
            predicate: BarePassivePredicate,
        }
        derive predicate.concord_class = Values::Other;
        derive concord_class = auxiliary.concord_class;
        form modal_passive_subject_gap_relative_clause =
            licensed("that") auxiliary predicate;
    }
    construction finite_subject_gap_relative_clause: FiniteSubjectGapRelativeClause {
        element FiniteSubjectGapRelativeClauseValue { head: lex IntransitiveVerb, }
        derive concord_class = head.concord_class;
        form finite_subject_gap_relative_clause = licensed("that") verb(head);
    }
    construction finite_transitive_subject_gap_relative_clause: FiniteSubjectGapRelativeClause {
        element FiniteTransitiveSubjectGapRelativeClause {
            head: lex TransitiveVerb,
            object: Object,
        }
        derive concord_class = head.concord_class;
        form finite_transitive_subject_gap_relative_clause =
            licensed("that") verb(head) object;
    }
    construction modal_subject_gap_relative_clause: ModalSubjectGapRelativeClause {
        element ModalSubjectGapRelativeClauseValue {
            auxiliary: AuxiliaryHead,
            head: lex IntransitiveVerb,
        }
        derive concord_class = auxiliary.concord_class;
        derive head.concord_class = Values::Other;
        form modal_subject_gap_relative_clause = licensed("that") auxiliary verb(head);
    }
    construction copular_subject_gap_relative_clause: CopularSubjectGapRelativeClause {
        element CopularSubjectGapRelativeClauseValue {
            copula: lex FiniteCopula,
            complement: PredicativeAdjectiveComplement,
        }
        derive copula.concord_class = match copula {
            Is => Values::ThirdPersonSingular,
            Isnt => Values::ThirdPersonSingular,
            Are => Values::Other,
            Arent => Values::Other,
            Was => Values::ThirdPersonSingular,
            Were => Values::Other,
        };
        derive concord_class = copula.concord_class;
        form copular_subject_gap_relative_clause = licensed("that") lex(copula) complement;
    }
    construction distributed_measure_predicate: VerbPhrase {
        element DistributedMeasurePredicate {
            head: lex DistributedMeasureVerb,
            amount: Amount,
            measure: MassNoun,
            distribution: DistributionPhrase,
            replacement: opt lex DistributionReplacement,
        }
        derive concord_class = head.concord_class;
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
        derive concord_class = head.concord_class;
        form declared_object_equality_to_predicate =
            verb(head) object equality lex(Preposition::To) recipient;
    }
    construction declared_object_to_equality_predicate: VerbPhrase {
        element DeclaredObjectToEqualityPredicate {
            head: lex ObjectToEqualityVerb,
            object: Object,
            recipient: Object,
            equality: ScalarEquality,
        }
        derive concord_class = head.concord_class;
        form declared_object_to_equality_predicate =
            verb(head) object lex(Preposition::To) recipient equality;
    }
    construction declared_object_equality_predicate: VerbPhrase {
        element DeclaredObjectEqualityPredicate {
            head: lex ObjectEqualityVerb,
            object: Object,
            equality: ScalarEquality,
        }
        derive concord_class = head.concord_class;
        form declared_object_equality_predicate = verb(head) object equality;
    }
    construction mana_phrase: VerbPhrase {
        element ManaVerbPhrase { head: lex ManaPhraseVerb, mana: ManaPhrase, }
        derive concord_class = head.concord_class;
        form mana_phrase = verb(head) mana;
    }
    construction declared_object_from_predicate: VerbPhrase {
        element DeclaredObjectFromPredicate {
            head: lex ObjectFromVerb,
            object: Object,
            source: FrameComplement,
        }
        derive concord_class = head.concord_class;
        form declared_object_from_predicate =
            verb(head) object lex(Preposition::From) source;
    }
    construction put_onto: VerbPhrase {
        element PutOnto {
            head: lex ObjectFromOntoResultControlVerb,
            object: Object checked by object_has_no_selected_source_postmodifier(),
            source: opt PrepositionalPhrase,
            destination: FrameComplement,
            result: opt PredicativeComplement,
            control: opt Object,
        }
        require source.preposition_complement_kind is SourceComplement;
        derive concord_class = head.concord_class;
        form put_onto =
            verb(head) object source lex(Preposition::Onto) destination result marked(Preposition::Under, control);
    }
    construction put_onto_source_after: VerbPhrase {
        element PutOntoSourceAfter {
            head: lex ObjectFromOntoResultControlVerb,
            object: Object,
            destination: FrameComplement,
            source: FrameComplement,
            result: opt PredicativeComplement,
            control: opt Object,
        }
        derive concord_class = head.concord_class;
        form put_onto_source_after =
            verb(head) object lex(Preposition::Onto) destination lex(Preposition::From) source result marked(Preposition::Under, control);
    }
    construction put_on: VerbPhrase {
        element PutOn {
            head: lex ObjectFromOnVerb,
            object: Object checked by object_has_no_selected_source_postmodifier(),
            source: opt PrepositionalPhrase,
            destination: FrameComplement,
        }
        require source.preposition_complement_kind is SourceComplement;
        derive concord_class = head.concord_class;
        form put_on = verb(head) object source lex(Preposition::On) destination;
    }
    construction put_to: VerbPhrase {
        element PutTo {
            head: lex ObjectToVerb,
            object: Object,
            destination: FrameComplement,
        }
        derive concord_class = head.concord_class;
        form put_to = verb(head) object lex(Preposition::To) destination;
    }
    construction return_to: VerbPhrase {
        element ReturnTo {
            head: lex ObjectFromToResultControlVerb,
            object: Object checked by object_has_no_selected_source_postmodifier(),
            source: opt PrepositionalPhrase,
            destination: FrameComplement,
            result: opt PredicativeComplement,
            control: opt Object,
        }
        require source.preposition_complement_kind is SourceComplement;
        derive concord_class = head.concord_class;
        form return_to =
            verb(head) object source lex(Preposition::To) destination result marked(Preposition::Under, control);
    }
    construction predicative_complement_predicate: VerbPhrase {
        element PredicativeComplementPredicate { head: lex PredicativeComplementVerb, complement: PredicativeComplement, }
        derive concord_class = head.concord_class;
        form predicative_complement_predicate = verb(head) complement;
    }
    construction declared_with_object_on_predicate: VerbPhrase {
        element DeclaredWithObjectOnPredicate {
            head: lex EnterWithCountersVerb,
            object: Object,
            recipient: FrameComplement,
        }
        derive concord_class = head.concord_class;
        form declared_with_object_on_predicate =
            verb(head) lex(Preposition::With) object lex(Preposition::On) recipient;
    }
    construction enter_location: VerbPhrase {
        element EnterLocation {
            head: lex EnterLocationVerb,
            location: Object,
            result: opt PredicativeComplement,
            control: opt Object,
        }
        derive concord_class = head.concord_class;
        form enter_location = verb(head) location result marked(Preposition::Under, control);
    }
    construction enter_control: VerbPhrase {
        element EnterControl {
            head: lex EnterControlVerb,
            control: Object,
        }
        derive concord_class = head.concord_class;
        form enter_control = verb(head) marked(Preposition::Under, control);
    }
    construction look_at: VerbPhrase {
        element LookAt { head: lex LookAtVerb, object: Object, }
        derive concord_class = head.concord_class;
        form look_at = verb(head) lex(Preposition::At) object;
    }
    construction declared_to_object_predicate: VerbPhrase {
        element DeclaredToObjectPredicate {
            head: lex ToObjectVerb,
            object: Object,
            complement: Object,
        }
        derive concord_class = head.concord_class;
        form declared_to_object_predicate =
            verb(head) object lex(Preposition::To) complement;
    }
    construction declared_for_object_predicate: VerbPhrase {
        element DeclaredForObjectPredicate {
            head: lex ForObjectVerb,
            object: Object,
        }
        derive concord_class = head.concord_class;
        form declared_for_object_predicate = verb(head) lex(Preposition::For) object;
    }
    // A quoted granted ability is a document in its own right: its interior
    // parses with the same grammar as printed rules text (oracle convention,
    // style guide "Quotation marks"; the CR does not describe the quoting).
    construction quoted_block: QuotedBlock {
        element QuotedBlockValue { block: OracleText, }
        form quoted_block = block;
    }
    construction quoted_ability: QuotedAbility {
        element QuotedAbilityValue { block: QuotedBlock, }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form quoted_ability = sentence_initial(" \"") suffix(block, "\"");
    }
    construction quoted_ability_predicate: VerbPhrase {
        element QuotedAbilityPredicate { head: lex QuotedAbilityVerb, ability: QuotedAbility, }
        derive concord_class = head.concord_class;
        form quoted_ability_predicate = verb(head) ability;
    }
    construction have_keyword_ability: VerbPhrase {
        element HaveKeywordAbility { head: lex HaveKeywordAbilityVerb, ability: lex KeywordAbility, }
        derive concord_class = head.concord_class;
        form have_keyword_ability = verb(head) lex(ability);
    }
    construction quote_terminated_statement: AbilityBody {
        element QuoteTerminatedStatement {
            sentence: Sentence checked by rightmost_leaf_is::<QuotedBlock>(),
        }
        form quote_terminated_statement = sentence;
    }
    construction quote_terminated_sentences: AbilityBody {
        element QuoteTerminatedSentences {
            preceding: seq Sentence separated by " " terminated by ".",
            sentence: Sentence checked by rightmost_leaf_is::<QuotedBlock>(),
        }
        require len(preceding) >= 1;
        form quote_terminated_sentences = preceding sentence_initial(" ") sentence;
    }
    construction get_power_toughness: GetPowerToughnessLexicalVerbPhrase {
        element GetPowerToughness {
            head: lex GetPowerToughnessVerb,
            adjustment: PowerToughnessAdjustment,
        }
        derive concord_class = head.concord_class;
        form get_power_toughness = verb(head) adjustment;
    }
    construction have_object_control: VerbPhrase {
        element HaveObjectControl {
            head: lex HaveObjectControlVerb,
            object: Object,
            predicate: BarePredicate,
        }
        derive concord_class = head.concord_class;
        derive predicate.concord_class = Values::Other;
        form have_object_control = verb(head) object predicate;
    }
    construction mana_amount: ManaAmount {
        element ManaAmountValue { run: ActivationCostComponent, }
        require run is SymbolRun;
        derive onset = Values::Consonant;
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
        element TwiceVariableAmount {
            adverb: lex FrequencyAdverb,
            variable: lex Variable,
        }
        require adverb is Twice;
        form twice_variable_amount = lex(adverb) lex(variable);
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
        derive concord_class = number.concord_class;
        derive cardinality = number.cardinality;
        derive determiner_number = number.determiner_number;
        derive number = number.number;
        form cardinal = lex(number);
    }

    abstract sum KeywordLineItem {
        Bare: BareKeywordLineItem,
        Costed: CostedKeywordLineItem,
        Amounted: AmountKeywordLineItem,
        AmountCosted: AmountCostKeywordLineItem,
        Qualified: QualifiedKeywordLineItem,
        QualityCosted: QualityCostKeywordLineItem,
        Subject: SubjectKeywordLineItem,
    }
    abstract sum KeywordCost {
        Mana: KeywordManaCost,
        Clause: KeywordCostPredicate,
        ManaClause: KeywordManaClauseCost,
    }
    abstract sum KeywordCostSeparator {
        Space: SpacedKeywordCost,
        Dash: DashedKeywordCost,
    }
    abstract sum KeywordQuality {
        Reference: Nominal,
        Coordination: KeywordQualityCoordination,
    }
    construction bare_keyword_line_item: BareKeywordLineItem {
        element BareKeywordLineItemValue { keyword: lex BareKeywordAbility, }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form bare_keyword_line_item = lex(keyword);
    }
    construction costed_keyword_line_item: CostedKeywordLineItem {
        element CostedKeywordLineItemValue {
            keyword: lex CostedKeywordAbility,
            cost: KeywordCostSeparator,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form costed_keyword_line_item = lex(keyword) cost;
    }
    construction keyword_mana_cost: KeywordManaCost {
        element KeywordManaCostValue { cost: ActivationCostComponent, }
        require cost is SymbolRun;
        form keyword_mana_cost = cost;
    }
    construction keyword_cost_predicate: KeywordCostPredicate {
        element KeywordCostPredicateValue { predicate: Predicate, }
        derive predicate.concord_class = Values::Other;
        form keyword_cost_predicate = predicate ".";
    }
    construction keyword_mana_clause_cost: KeywordManaClauseCost {
        element KeywordManaClauseCostValue {
            mana: ActivationCostComponent,
            clause: KeywordCostPredicate,
        }
        require mana is SymbolRun;
        form keyword_mana_clause_cost = mana sentence_initial(", ") clause;
    }
    construction spaced_keyword_cost: KeywordCostSeparator {
        element SpacedKeywordCost { cost: KeywordManaCost, }
        form spaced_keyword_cost = cost;
    }
    construction dashed_keyword_cost: KeywordCostSeparator {
        element DashedKeywordCost { cost: KeywordCost, }
        form dashed_keyword_cost = sentence_initial("—") cost;
    }
    construction amount_keyword_line_item: AmountKeywordLineItem {
        element AmountKeywordLineItemValue {
            keyword: lex AmountKeywordAbility,
            amount: Amount,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form amount_keyword_line_item = lex(keyword) amount;
    }
    construction amount_cost_keyword_line_item: AmountCostKeywordLineItem {
        element AmountCostKeywordLineItemValue {
            keyword: lex AmountCostKeywordAbility,
            amount: Amount,
            cost: KeywordCostSeparator,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form amount_cost_keyword_line_item = lex(keyword) amount cost;
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
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form qualified_keyword_line_item = lex(keyword) quality;
    }
    construction quality_cost_keyword_line_item: QualityCostKeywordLineItem {
        element QualityCostKeywordLineItemValue {
            keyword: lex QualityCostKeywordAbility,
            quality: KeywordQuality,
            cost: KeywordCostSeparator,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form quality_cost_keyword_line_item = lex(keyword) quality cost;
    }
    construction subject_keyword_line_item: SubjectKeywordLineItem {
        element SubjectKeywordLineItemValue {
            keyword: lex SubjectKeywordAbility,
            subject: KeywordSubject,
        }
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form subject_keyword_line_item = lex(keyword) subject;
    }
    construction granted_keyword_line: GrantedKeywordLine {
        element GrantedKeywordLineValue {
            items: seq KeywordLineItem separated by " and ",
        }
        require len(items) >= 1;
        derive relationality = Values::NonRelational;
        derive locative_temporal_license = Values::Unlicensed;
        form granted_keyword_line = items;
    }
    construction reference_keyword_subject: KeywordSubject {
        element ReferenceKeywordSubject { subject: NounPhrase, }
        form reference_keyword_subject = subject;
    }
    construction bare_keyword_subject: KeywordSubject {
        element BareKeywordSubject {
            subject: Nominal,
            modifier: opt KeywordSubjectModifier,
        }
        require subject.number is Singular;
        form bare_keyword_subject = subject modifier;
    }
    construction keyword_relative_subject_modifier: KeywordSubjectModifier {
        element KeywordRelativeSubjectModifier { modifier: ObjectGapRelativeClause, }
        form keyword_relative_subject_modifier = modifier;
    }
    construction keyword_prepositional_subject_modifier: KeywordSubjectModifier {
        element KeywordPrepositionalSubjectModifier { modifier: PrepositionalPhrase, }
        form keyword_prepositional_subject_modifier = modifier;
    }
    construction keyword_without_subject_modifier: KeywordSubjectModifier {
        element KeywordWithoutSubjectModifier { ability: lex KeywordAbility, }
        form keyword_without_subject_modifier = "without" lex(ability);
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
        Term: LabelTerm,
        Chapter: ChapterLabel,
    }
    abstract sum LabelTerm {
        AbilityWord: AbilityWordLabelTerm,
        FlavorWord: FlavorWordLabelTerm,
    }
    construction ability_word_label_term: AbilityWordLabelTerm {
        element AbilityWordLabelTermValue { term: lex AbilityWordTerm, }
        form ability_word_label_term = lex(term);
    }
    construction flavor_word_label_term: FlavorWordLabelTerm {
        element FlavorWordLabelTermValue { term: lex FlavorWordTerm, }
        form flavor_word_label_term = lex(term);
    }
    construction chapter_label: ChapterLabel {
        element ChapterLabelValue {
            numerals: seq lex ChapterNumeral separated by ", ",
            secondary: opt LabelTerm,
        }
        require len(numerals) >= 1;
        form chapter_label_secondary when secondary.is_some() =
            lex(numerals) sentence_initial(" — ") secondary;
        form chapter_label_plain otherwise = lex(numerals) secondary;
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

fn preposition_complement_is_licensed(
    complement: &PrepositionalComplement,
    kind: PrepositionComplementKind,
    license: LocativeTemporalLicense,
) -> bool {
    let edge = matches!(complement, PrepositionalComplement::Edge(_));
    match kind {
        PrepositionComplementKind::UnrestrictedComplement
        | PrepositionComplementKind::RelationalComplement
        | PrepositionComplementKind::SelectionComplement
        | PrepositionComplementKind::SourceComplement => true,
        PrepositionComplementKind::InComplement => {
            !edge
                && matches!(
                    license,
                    LocativeTemporalLicense::OfInAndOnLicensed
                        | LocativeTemporalLicense::InLicensed
                        | LocativeTemporalLicense::InOrOnEdgeLicensed
                )
        }
        // `on` may take any ordinary object as its complement. Whether that
        // object is a surface/host for the nominal head is a separate M3
        // attachment decision below. The edge reading remains explicit.
        PrepositionComplementKind::OnComplement => match license {
            LocativeTemporalLicense::InOrOnEdgeLicensed => edge,
            LocativeTemporalLicense::OnLicensed
            | LocativeTemporalLicense::OfAndOnLicensed
            | LocativeTemporalLicense::OfInAndOnLicensed
            | LocativeTemporalLicense::TemporalLicensed
            | LocativeTemporalLicense::OfAndTemporalLicensed
            | LocativeTemporalLicense::Unlicensed
            | LocativeTemporalLicense::OfLicensed
            | LocativeTemporalLicense::InLicensed
            | LocativeTemporalLicense::ObjectAttachmentLicensed => !edge,
        },
        PrepositionComplementKind::TemporalComplement => {
            !edge
                && matches!(
                    license,
                    LocativeTemporalLicense::TemporalLicensed
                        | LocativeTemporalLicense::OfAndTemporalLicensed
                )
        }
    }
}

fn nominal_nonrelational_preposition_is_licensed(
    modifier: &PrepositionalPhrase,
    relationality: Relationality,
    license: LocativeTemporalLicense,
    attachment: PrepositionAttachment,
    kind: PrepositionComplementKind,
    complement_relationality: Relationality,
    complement_license: LocativeTemporalLicense,
) -> bool {
    kind != PrepositionComplementKind::RelationalComplement
        && nominal_preposition_is_licensed(
            modifier,
            relationality,
            license,
            attachment,
            kind,
            complement_relationality,
            complement_license,
        )
}

fn nominal_relational_preposition_is_licensed(
    modifier: &PrepositionalPhrase,
    relationality: Relationality,
    license: LocativeTemporalLicense,
    attachment: PrepositionAttachment,
    kind: PrepositionComplementKind,
    complement_relationality: Relationality,
    complement_license: LocativeTemporalLicense,
) -> bool {
    kind == PrepositionComplementKind::RelationalComplement
        && nominal_preposition_is_licensed(
            modifier,
            relationality,
            license,
            attachment,
            kind,
            complement_relationality,
            complement_license,
        )
}

fn nominal_preposition_is_licensed(
    _modifier: &PrepositionalPhrase,
    relationality: Relationality,
    license: LocativeTemporalLicense,
    attachment: PrepositionAttachment,
    kind: PrepositionComplementKind,
    complement_relationality: Relationality,
    complement_license: LocativeTemporalLicense,
) -> bool {
    let accepts_interior = matches!(
        license,
        LocativeTemporalLicense::InLicensed
            | LocativeTemporalLicense::InOrOnEdgeLicensed
            | LocativeTemporalLicense::OfInAndOnLicensed
            | LocativeTemporalLicense::ObjectAttachmentLicensed
    );
    let accepts_surface = matches!(
        license,
        LocativeTemporalLicense::OnLicensed
            | LocativeTemporalLicense::InOrOnEdgeLicensed
            | LocativeTemporalLicense::OfAndOnLicensed
            | LocativeTemporalLicense::OfInAndOnLicensed
            | LocativeTemporalLicense::TemporalLicensed
            | LocativeTemporalLicense::OfAndTemporalLicensed
    ) || (license == LocativeTemporalLicense::ObjectAttachmentLicensed
        && matches!(
            complement_license,
            LocativeTemporalLicense::OnLicensed | LocativeTemporalLicense::InOrOnEdgeLicensed
        ));
    let accepts_temporal = matches!(
        license,
        LocativeTemporalLicense::TemporalLicensed | LocativeTemporalLicense::OfAndTemporalLicensed
    );
    // A qualified `of` head accepts semantic qualifiers such as choice,
    // color, and type, but not an arbitrary object noun. Declaration object
    // classes are themselves qualified heads, so their object-attachment
    // licence distinguishes "creature of their choice" from the rejected
    // "card of a Goblin".
    let qualified_complement = (matches!(
        complement_relationality,
        Relationality::Relational | Relationality::SaturatedRelational
    ) && complement_license
        != LocativeTemporalLicense::ObjectAttachmentLicensed)
        || matches!(
            complement_license,
            LocativeTemporalLicense::OfAndOnLicensed | LocativeTemporalLicense::OfInAndOnLicensed
        );

    match kind {
        PrepositionComplementKind::RelationalComplement => match relationality {
            Relationality::NonRelational => matches!(
                license,
                LocativeTemporalLicense::OfLicensed
                    | LocativeTemporalLicense::OfAndOnLicensed
                    | LocativeTemporalLicense::OfInAndOnLicensed
                    | LocativeTemporalLicense::OfAndTemporalLicensed
            ),
            Relationality::QualifiedRelational => qualified_complement,
            Relationality::DeterminedRelational | Relationality::Relational => true,
            Relationality::SaturatedRelational => false,
        },
        PrepositionComplementKind::SelectionComplement => true,
        PrepositionComplementKind::SourceComplement => {
            matches!(
                license,
                LocativeTemporalLicense::ObjectAttachmentLicensed
                    | LocativeTemporalLicense::OfAndOnLicensed
                    | LocativeTemporalLicense::OfInAndOnLicensed
            ) || relationality != Relationality::NonRelational
        }
        PrepositionComplementKind::InComplement => accepts_interior,
        PrepositionComplementKind::OnComplement => accepts_surface,
        PrepositionComplementKind::TemporalComplement => accepts_temporal,
        PrepositionComplementKind::UnrestrictedComplement => {
            attachment == PrepositionAttachment::PostmodifierOnly
                && matches!(
                    license,
                    LocativeTemporalLicense::ObjectAttachmentLicensed
                        | LocativeTemporalLicense::OfInAndOnLicensed
                )
        }
    }
}

fn object_has_no_selected_source_postmodifier(object: &Object) -> bool {
    right_edge_nominal_postmodifier_kind(object)
        != Some(PrepositionComplementKind::SourceComplement)
}

fn right_edge_nominal_postmodifier_kind(object: &Object) -> Option<PrepositionComplementKind> {
    let Object::ObjectNominal(NominalObject { value }) = object else {
        return None;
    };
    right_edge_noun_phrase_postmodifier_kind(value)
}

fn right_edge_noun_phrase_postmodifier_kind(
    noun_phrase: &NounPhrase,
) -> Option<PrepositionComplementKind> {
    match noun_phrase {
        NounPhrase::LocativeCoordinatedNounPhrase(value) => match &value.coordination {
            LocativeNounPhraseCoordination::LocativeAndNounPhraseCoordination(coordination) => {
                coordination
                    .members()
                    .last()
                    .and_then(right_edge_postmodified_reference_kind)
            }
        },
        NounPhrase::QualifiedNounPhrase(value) => {
            right_edge_postmodified_reference_kind(&value.reference)
        }
        NounPhrase::ComparativeQuantifiedReference(value) => {
            right_edge_nominal_postmodifier_kind(&value.standard)
        }
        NounPhrase::PositionalPartitive(value) => {
            right_edge_nominal_postmodifier_kind(&value.whole)
        }
        NounPhrase::FusedDeterminativeReference(_) | NounPhrase::CommonNounChoiceList(_) => None,
    }
}

fn right_edge_postmodified_reference_kind(
    reference: &PostmodifiedReference,
) -> Option<PrepositionComplementKind> {
    match reference {
        PostmodifiedReference::PrepositionalQualifiedReference(value) => Some(
            preposition_complement_kind_for_prepositional_phrase(value.modifier()),
        ),
        PostmodifiedReference::RelationalQualifiedReference(value) => Some(
            preposition_complement_kind_for_prepositional_phrase(value.modifier()),
        ),
        PostmodifiedReference::UnqualifiedPostmodifiedReference(_)
        | PostmodifiedReference::RelativeQualifiedReference(_)
        | PostmodifiedReference::SubjectRelativeQualifiedReference(_)
        | PostmodifiedReference::ContractedCopularRelativeReference(_)
        | PostmodifiedReference::ReducedPassiveQualifiedReference(_)
        | PostmodifiedReference::ReducedPassiveAdjunctQualifiedReference(_)
        | PostmodifiedReference::ReducedPassivePrepositionalAdjunctQualifiedReference(_)
        | PostmodifiedReference::OtherThanQualifiedReference(_) => None,
    }
}

fn predicate_preposition_is_licensed(
    _adjunct: &PrepositionalPhrase,
    kind: PrepositionComplementKind,
    complement_license: LocativeTemporalLicense,
) -> bool {
    !matches!(
        kind,
        PrepositionComplementKind::OnComplement | PrepositionComplementKind::TemporalComplement
    ) || matches!(
        complement_license,
        LocativeTemporalLicense::TemporalLicensed | LocativeTemporalLicense::OfAndTemporalLicensed
    )
}

fn predicate_adjunct_is_prepositional(adjunct: &PredicateAdjunct) -> bool {
    matches!(adjunct, PredicateAdjunct::Prepositional(_))
}

fn predicate_adjunct_is_nonprepositional(adjunct: &PredicateAdjunct) -> bool {
    !predicate_adjunct_is_prepositional(adjunct)
}

fn predicate_adjunct_is_duration(adjunct: &PredicateAdjunct) -> bool {
    matches!(adjunct, PredicateAdjunct::Duration(_))
}

fn bare_preposition_complement_is_licensed(
    complement: &BareLocative,
    kind: PrepositionComplementKind,
) -> bool {
    let license = match complement {
        BareLocative::Proform(_) => LocativeTemporalLicense::Unlicensed,
        BareLocative::Noun(value) => locative_temporal_license_for_noun(&value.noun),
    };
    match kind {
        PrepositionComplementKind::UnrestrictedComplement
        | PrepositionComplementKind::SourceComplement => true,
        PrepositionComplementKind::InComplement => matches!(
            license,
            LocativeTemporalLicense::InLicensed | LocativeTemporalLicense::InOrOnEdgeLicensed
        ),
        PrepositionComplementKind::RelationalComplement
        | PrepositionComplementKind::SelectionComplement
        | PrepositionComplementKind::OnComplement
        | PrepositionComplementKind::TemporalComplement => false,
    }
}

fn determinative_is_fused(head: &Determinative, fused_head_license: FusedHeadLicense) -> bool {
    let _ = head;
    matches!(
        fused_head_license,
        FusedHeadLicense::PartitiveOnly
            | FusedHeadLicense::FusedHead
            | FusedHeadLicense::PluralPredeterminer
    )
}

fn noun_has_distinct_number_surfaces(noun: &Noun) -> bool {
    match noun {
        Noun::Lexeme(_) => true,
        Noun::Declaration(noun) => !noun.number_invariant(),
    }
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

// A marker-less duration phrase carries no preposition, so its endpoint is the
// only thing bounding it: any nominal beside the verb can otherwise be absorbed
// as a duration. The endpoint's head must therefore carry the declared temporal
// licence, which is the feature that says the noun denotes a time.
fn temporal_endpoint_denotes_a_time(endpoint: &TemporalEndpoint) -> bool {
    let TemporalEndpoint::Reference(reference) = endpoint;
    matches!(
        locative_temporal_license_for_noun_phrase(reference),
        LocativeTemporalLicense::TemporalLicensed | LocativeTemporalLicense::OfAndTemporalLicensed
    )
}

fn object_is_mass_nominal(value: &Object) -> bool {
    let Object::ObjectNominal(object) = value else {
        return false;
    };
    let NounPhrase::QualifiedNounPhrase(qualified) = object.value.as_ref() else {
        return false;
    };
    match postmodified_base(qualified.reference.as_ref()) {
        UnqualifiedReference::DeterminedNominal(determined) => {
            nominal_form_for_nominal(&determined.nominal) == NominalForm::MassNoun
        }
        UnqualifiedReference::PossessedMassReference(_)
        | UnqualifiedReference::GenitiveDeterminerMassReference(_) => true,
        _ => false,
    }
}

fn postmodified_base(reference: &PostmodifiedReference) -> &UnqualifiedReference {
    match reference {
        PostmodifiedReference::UnqualifiedPostmodifiedReference(value) => &value.reference,
        PostmodifiedReference::RelativeQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::SubjectRelativeQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::ContractedCopularRelativeReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::ReducedPassiveQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::ReducedPassiveAdjunctQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::ReducedPassivePrepositionalAdjunctQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::OtherThanQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::PrepositionalQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
        PostmodifiedReference::RelationalQualifiedReference(value) => {
            postmodified_base(&value.reference)
        }
    }
}

fn full_coordination_is_independent(coordination: &FullNounPhraseCoordination) -> bool {
    fn independently_realized(reference: &PostmodifiedReference) -> bool {
        let PostmodifiedReference::UnqualifiedPostmodifiedReference(reference) = reference else {
            return true;
        };
        !matches!(
            reference.reference.as_ref(),
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

fn locative_coordination_has_modifier(coordination: &LocativeNounPhraseCoordination) -> bool {
    fn has_prepositional_modifier(reference: &PostmodifiedReference) -> bool {
        match reference {
            PostmodifiedReference::PrepositionalQualifiedReference(_)
            | PostmodifiedReference::RelationalQualifiedReference(_) => true,
            PostmodifiedReference::UnqualifiedPostmodifiedReference(_) => false,
            PostmodifiedReference::RelativeQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::SubjectRelativeQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::ContractedCopularRelativeReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::ReducedPassiveQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::ReducedPassiveAdjunctQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::ReducedPassivePrepositionalAdjunctQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
            PostmodifiedReference::OtherThanQualifiedReference(value) => {
                has_prepositional_modifier(&value.reference)
            }
        }
    }

    match coordination {
        LocativeNounPhraseCoordination::LocativeAndNounPhraseCoordination(coordination) => {
            coordination.members.iter().any(has_prepositional_modifier)
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
