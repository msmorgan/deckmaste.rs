// Authored vocabulary and morphology; read as declaration data, not compiled Rust.
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
    vocab ArbitraryDeterminer { Any = "any", Random = "a random", }
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
    vocab FocusAdverb { Only = "only", }
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
    vocab ComparisonDirection { More = "more", Less = "less", }
    vocab ReplacementMarker { Instead = "instead", }
    vocab PastPossession { Had = "had", }
    vocab TriggerMarker { When = "when", Whenever = "whenever", }
    // Oracle "you" denotes one player: second-person plural is intentionally
    // omitted from you, your, yours and the reflexive paradigm in core.ron.
    vocab SubjectPronoun { He = "he", It = "it", She = "she", They = "they", You = "you", }
    vocab ObjectPronoun { Her = "her", Him = "him", It = "it", Them = "them", You = "you", }
    vocab PossessiveDeterminerPronoun {
        feature DeterminerNumber = Both;
        feature NominalLicense = AnyNominal;
        // Possessive determinatives require an overt temporal marker.
        feature BareDurationLicense = MarkerRequired;
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
    }
    vocab Variable { X = "X", Y = "Y", }
    vocab ColorWord {
        Black = "black",
        Blue = "blue",
        Colorless = "colorless",
        Green = "green",
        Monocolored = "monocolored",
        Multicolored = "multicolored",
        Red = "red",
        White = "white",
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
}
