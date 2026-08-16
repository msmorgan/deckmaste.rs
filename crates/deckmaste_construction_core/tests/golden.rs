use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_construction_core::ItemKey;
use deckmaste_construction_core::NamedKind;
use quote::ToTokens;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum SemanticKey {
    Type(String),
    Trait(String),
    Function(String),
    Constant(String),
    Impl {
        trait_name: Option<String>,
        self_ty: String,
    },
}

impl SemanticKey {
    fn type_named(name: &str) -> Self {
        Self::Type(name.to_owned())
    }

    fn function(name: &str) -> Self {
        Self::Function(name.to_owned())
    }

    fn trait_named(name: &str) -> Self {
        Self::Trait(name.to_owned())
    }

    fn constant(name: &str) -> Self {
        Self::Constant(name.to_owned())
    }

    fn trait_impl(trait_name: &str, self_ty: &str) -> Self {
        Self::Impl {
            trait_name: Some(trait_name.to_owned()),
            self_ty: self_ty.to_owned(),
        }
    }

    fn inherent_impl(self_ty: &str) -> Self {
        Self::Impl {
            trait_name: None,
            self_ty: self_ty.to_owned(),
        }
    }
}

impl std::fmt::Display for SemanticKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(name) => write!(formatter, "type {name}"),
            Self::Trait(name) => write!(formatter, "trait {name}"),
            Self::Function(name) => write!(formatter, "function {name}"),
            Self::Constant(name) => write!(formatter, "constant {name}"),
            Self::Impl {
                trait_name: Some(trait_name),
                self_ty,
            } => write!(formatter, "impl {trait_name} for {self_ty}"),
            Self::Impl {
                trait_name: None,
                self_ty,
            } => write!(formatter, "impl {self_ty}"),
        }
    }
}

#[derive(Debug, Clone)]
struct ExtractedItem {
    key: SemanticKey,
    normalized: String,
    origin: String,
}

fn extract_pinned_items(
    source: &str,
    origin: &str,
    pinned: &BTreeSet<SemanticKey>,
) -> Result<Vec<ExtractedItem>, String> {
    let extracted = extract_pinned_items_permissive(source, origin, pinned)?;
    let mut duplicate_errors = Vec::new();
    append_duplicate_diagnostics("golden", &extracted, &mut duplicate_errors);
    if duplicate_errors.is_empty() {
        Ok(extracted)
    } else {
        Err(duplicate_errors.join("\n"))
    }
}

fn extract_pinned_items_permissive(
    source: &str,
    origin: &str,
    pinned: &BTreeSet<SemanticKey>,
) -> Result<Vec<ExtractedItem>, String> {
    let file = syn::parse_file(source).map_err(|error| format!("parsing {origin}: {error}"))?;
    let mut extracted = Vec::new();

    for item in file.items {
        let Some(key) = semantic_key(&item) else {
            continue;
        };
        if !pinned.contains(&key) {
            continue;
        }
        let normalized = normalize_item(item)?;
        extracted.push(ExtractedItem {
            key: key.clone(),
            normalized,
            origin: origin.to_owned(),
        });
    }
    Ok(extracted)
}

fn inventory_diagnostics(
    pinned: &BTreeSet<SemanticKey>,
    golden: &[ExtractedItem],
    generated: &[ExtractedItem],
) -> String {
    let golden_keys = golden.iter().map(|item| &item.key).collect::<BTreeSet<_>>();
    let generated_keys = generated
        .iter()
        .map(|item| &item.key)
        .collect::<BTreeSet<_>>();
    let mut diagnostics = Vec::new();

    append_duplicate_diagnostics("golden", golden, &mut diagnostics);
    append_duplicate_diagnostics("generated", generated, &mut diagnostics);

    for key in pinned {
        if !golden_keys.contains(key) {
            diagnostics.push(format!("missing golden {key}"));
        }
        if !generated_keys.contains(key) {
            diagnostics.push(format!("missing generated {key}"));
        }
    }
    for item in generated {
        if !pinned.contains(&item.key) {
            diagnostics.push(format!(
                "unexpected generated {} at {}",
                item.key, item.origin
            ));
        }
    }

    diagnostics.join("\n")
}

fn append_duplicate_diagnostics(
    inventory: &str,
    items: &[ExtractedItem],
    diagnostics: &mut Vec<String>,
) {
    let mut by_key = BTreeMap::<&SemanticKey, Vec<&str>>::new();
    for item in items {
        by_key.entry(&item.key).or_default().push(&item.origin);
    }
    for (key, origins) in by_key {
        if origins.len() > 1 {
            diagnostics.push(format!(
                "duplicate {inventory} {key} at {}",
                origins.join(", ")
            ));
        }
    }
}

fn normalized_item_comparison(left: &str, right: &str) -> Result<(), String> {
    let golden = normalize_item(syn::parse_str(left).map_err(|error| error.to_string())?)?;
    let generated = normalize_item(syn::parse_str(right).map_err(|error| error.to_string())?)?;
    if golden == generated {
        Ok(())
    } else {
        Err(unified_diff(&golden, &generated))
    }
}

fn semantic_key(item: &syn::Item) -> Option<SemanticKey> {
    match item {
        syn::Item::Enum(item) => Some(SemanticKey::Type(item.ident.to_string())),
        syn::Item::Struct(item) => Some(SemanticKey::Type(item.ident.to_string())),
        syn::Item::Trait(item) => Some(SemanticKey::Trait(item.ident.to_string())),
        syn::Item::Fn(item) => Some(SemanticKey::Function(item.sig.ident.to_string())),
        syn::Item::Const(item) => Some(SemanticKey::Constant(item.ident.to_string())),
        syn::Item::Impl(item) => Some(SemanticKey::Impl {
            trait_name: item
                .trait_
                .as_ref()
                .map(|(_, path, _)| compact_tokens(&path.to_token_stream())),
            self_ty: compact_tokens(&item.self_ty.to_token_stream()),
        }),
        _ => None,
    }
}

fn compact_tokens(tokens: &proc_macro2::TokenStream) -> String {
    tokens.to_string().replace(" :: ", "::")
}

fn normalize_item(item: syn::Item) -> Result<String, String> {
    let normalized = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: vec![item],
    });
    let reparsed = syn::parse_file(&normalized).map_err(|error| error.to_string())?;
    if reparsed.items.len() != 1 {
        return Err(format!(
            "normalization produced {} items instead of one",
            reparsed.items.len()
        ));
    }
    Ok(normalized)
}

fn unified_diff(golden: &str, generated: &str) -> String {
    let golden_lines = golden.lines().collect::<Vec<_>>();
    let generated_lines = generated.lines().collect::<Vec<_>>();
    let mut lengths = vec![vec![0_usize; generated_lines.len() + 1]; golden_lines.len() + 1];
    for left in (0..golden_lines.len()).rev() {
        for right in (0..generated_lines.len()).rev() {
            lengths[left][right] = if golden_lines[left] == generated_lines[right] {
                lengths[left + 1][right + 1] + 1
            } else {
                lengths[left + 1][right].max(lengths[left][right + 1])
            };
        }
    }

    let mut output = format!(
        "--- golden\n+++ generated\n@@ -1,{} +1,{} @@\n",
        golden_lines.len(),
        generated_lines.len()
    );
    let (mut left, mut right) = (0, 0);
    while left < golden_lines.len() || right < generated_lines.len() {
        if left < golden_lines.len()
            && right < generated_lines.len()
            && golden_lines[left] == generated_lines[right]
        {
            output.push(' ');
            output.push_str(golden_lines[left]);
            output.push('\n');
            left += 1;
            right += 1;
        } else if right < generated_lines.len()
            && (left == golden_lines.len() || lengths[left][right + 1] >= lengths[left + 1][right])
        {
            output.push('+');
            output.push_str(generated_lines[right]);
            output.push('\n');
            right += 1;
        } else {
            output.push('-');
            output.push_str(golden_lines[left]);
            output.push('\n');
            left += 1;
        }
    }
    output
}

#[derive(Debug, Clone)]
struct PinnedItem {
    key: SemanticKey,
    file: &'static str,
}

fn pinned_manifest() -> Vec<PinnedItem> {
    let mut manifest = Vec::new();
    append_types(
        &mut manifest,
        "ast.rs",
        &[
            "Ability",
            "Sentence",
            "Clause",
            "NounPhrase",
            "VerbPhrase",
            "Amount",
            "Spell",
            "Triggered",
            "Imperative",
            "Declarative",
            "WithWhere",
            "EventClause",
            "WhereClause",
            "PronounNp",
            "Common",
            "DemonstrativeNp",
            "TargetNp",
            "SelfReferenceNp",
            "CountNp",
            "Destroy",
            "Connive",
            "DealDamage",
            "GainLife",
            "NumberAmount",
            "VariableAmount",
            "TriggerWord",
            "Article",
            "Demonstrative",
            "Pronoun",
            "Variable",
            "NounLexeme",
            "VerbLexeme",
        ],
    );

    manifest.extend([
        PinnedItem {
            key: SemanticKey::trait_impl("Render", "Ability"),
            file: "render.rs",
        },
        PinnedItem {
            key: SemanticKey::trait_impl("Render", "Sentence"),
            file: "render.rs",
        },
    ]);
    for name in [
        "render_sentence_body",
        "render_clause",
        "render_verb_phrase",
        "render_noun_phrase",
        "render_amount",
        "render_trigger_word",
        "render_article",
        "render_demonstrative",
        "render_pronoun",
        "render_variable",
        "agreement_for_noun_phrase",
        "number_for_noun_phrase",
    ] {
        manifest.push(PinnedItem {
            key: SemanticKey::function(name),
            file: "render.rs",
        });
    }

    manifest.push(PinnedItem {
        key: SemanticKey::trait_named("Visitor"),
        file: "visit.rs",
    });
    for name in [
        "walk_ability",
        "walk_sentence",
        "walk_clause",
        "walk_noun_phrase",
        "walk_verb_phrase",
        "walk_amount",
        "walk_noun",
        "walk_spell",
        "walk_triggered",
        "walk_imperative",
        "walk_declarative",
        "walk_with_where",
        "walk_event_clause",
        "walk_where_clause",
        "walk_pronoun_np",
        "walk_common",
        "walk_demonstrative_np",
        "walk_target_np",
        "walk_self_reference_np",
        "walk_count_np",
        "walk_destroy",
        "walk_connive",
        "walk_deal_damage",
        "walk_gain_life",
        "walk_number_amount",
        "walk_variable_amount",
        "walk_trigger_word",
        "walk_article",
        "walk_demonstrative",
        "walk_pronoun",
        "walk_variable",
        "walk_sign",
        "walk_self_reference_spelling",
        "walk_noun_lexeme",
        "walk_verb_lexeme",
        "walk_signed_number",
        "walk_catalog_identity",
    ] {
        manifest.push(PinnedItem {
            key: SemanticKey::function(name),
            file: "visit.rs",
        });
    }

    append_types(
        &mut manifest,
        "parser/rules.rs",
        &["Category", "Construction", "RuleId"],
    );
    manifest.extend([
        PinnedItem {
            key: SemanticKey::inherent_impl("RuleId"),
            file: "parser/rules.rs",
        },
        PinnedItem {
            key: SemanticKey::constant("RULES"),
            file: "parser/rules.rs",
        },
        PinnedItem {
            key: SemanticKey::function("build"),
            file: "parser/build.rs",
        },
    ]);
    manifest
}

fn append_types(manifest: &mut Vec<PinnedItem>, file: &'static str, names: &[&str]) {
    manifest.extend(names.iter().map(|name| PinnedItem {
        key: SemanticKey::type_named(name),
        file,
    }));
}

fn generated_items(
    expansion: &deckmaste_construction_core::Expansion,
) -> Result<Vec<ExtractedItem>, String> {
    expansion
        .items()
        .iter()
        .map(|item| {
            let parsed = syn::parse2::<syn::Item>(item.tokens.clone())
                .map_err(|error| format!("parsing generated {:?}: {error}", item.key))?;
            Ok(ExtractedItem {
                key: generated_key(&item.key),
                normalized: normalize_item(parsed)?,
                origin: item
                    .origins
                    .iter()
                    .map(|origin| format!("{:?} {}", origin.kind(), origin.name()))
                    .collect::<Vec<_>>()
                    .join(", "),
            })
        })
        .collect()
}

fn generated_key(key: &ItemKey) -> SemanticKey {
    match key {
        ItemKey::Named { kind, name } => match kind {
            NamedKind::Type => SemanticKey::Type(name.clone()),
            NamedKind::Trait => SemanticKey::Trait(name.clone()),
            NamedKind::Function => SemanticKey::Function(name.clone()),
            NamedKind::Constant => SemanticKey::Constant(name.clone()),
        },
        ItemKey::Impl {
            trait_name,
            self_ty,
        } => SemanticKey::Impl {
            trait_name: trait_name.clone(),
            self_ty: self_ty.clone(),
        },
    }
}

fn load_golden(manifest: &[PinnedItem]) -> (Vec<ExtractedItem>, Vec<String>) {
    let source_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_english_v2/src");
    let files = manifest.iter().map(|pin| pin.file).collect::<BTreeSet<_>>();
    let mut items = Vec::new();
    let mut errors = Vec::new();

    for file in files {
        let pinned = manifest
            .iter()
            .filter(|pin| pin.file == file)
            .map(|pin| pin.key.clone())
            .collect::<BTreeSet<_>>();
        let path = source_root.join(file);
        match std::fs::read_to_string(&path) {
            Ok(source) => {
                match extract_pinned_items_permissive(&source, &path.display().to_string(), &pinned)
                {
                    Ok(mut extracted) => items.append(&mut extracted),
                    Err(error) => errors.push(error),
                }
            }
            Err(error) => errors.push(format!("reading {}: {error}", path.display())),
        }
    }

    (items, errors)
}

fn compare_full_inventory(
    manifest: &[PinnedItem],
    golden: &[ExtractedItem],
    generated: &[ExtractedItem],
    mut diagnostics: Vec<String>,
) -> Result<(), String> {
    let pinned = manifest
        .iter()
        .map(|pin| pin.key.clone())
        .collect::<BTreeSet<_>>();
    diagnostics.extend(
        inventory_diagnostics(&pinned, golden, generated)
            .lines()
            .map(str::to_owned),
    );
    let golden_by_key = golden
        .iter()
        .map(|item| (&item.key, item))
        .collect::<BTreeMap<_, _>>();
    let generated_by_key = generated
        .iter()
        .map(|item| (&item.key, item))
        .collect::<BTreeMap<_, _>>();

    for key in &pinned {
        let (Some(golden), Some(generated)) = (golden_by_key.get(key), generated_by_key.get(key))
        else {
            continue;
        };
        if golden.normalized != generated.normalized {
            diagnostics.push(format!(
                "mismatch {key}\ngolden origin: {}\ngenerated origin: {}\n{}",
                golden.origin,
                generated.origin,
                unified_diff(&golden.normalized, &generated.normalized)
            ));
        }
    }

    if diagnostics.is_empty() { Ok(()) } else { Err(diagnostics.join("\n\n")) }
}

const DECLARATION_SOURCE: &str = r#"
constructions! {
    vocab TriggerWord { Whenever = "whenever", }
    vocab Article { A = "a", An = "an", }
    vocab Demonstrative { That = "that", Those = "those", }
    vocab Pronoun { It = "it", You = "you", }
    vocab Variable { X = "X", }

    lexeme NounLexeme { Player, }
    lexeme VerbLexeme { Destroy, Connive, Deal, Gain, Control, Be, }

    codec Noun {
        atom = noun;
        value_type = crate::ast::Noun;
        lexical = Lexical::Noun;
        render = render_noun;
        build { pattern = BuildValue::Noun(noun); construct = noun; }
        traversal {
            callback = borrowed;
            argument = noun;
            variant Lexeme;
            variant Catalog;
            match noun {
                Noun::Lexeme(noun_lexeme: NounLexeme) => walk_noun_lexeme(copy(noun_lexeme)),
                Noun::Catalog(catalog_identity: CatalogIdentity) => walk_catalog_identity(borrowed(catalog_identity)),
            }
        }
    }
    codec Sign {
        value_type = crate::ast::Sign;
        traversal {
            callback = copy;
            argument = sign;
            variant Positive;
            variant Negative;
        }
    }
    identity SelfReferenceSpelling {
        value_type = crate::ast::SelfReferenceSpelling;
        lexical = Lexical::SelfReference;
        render context_identity {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
        }
        build { pattern = BuildValue::SelfReference(spelling); construct = spelling; }
        traversal {
            callback = copy;
            argument = spelling;
            variant Full;
            variant Abbreviated;
        }
    }
    codec SignedNumber {
        atom = lex;
        value_type = crate::ast::SignedNumber;
        lexical = Lexical::SignedNumber;
        render = render_signed_number;
        build { pattern = BuildValue::SignedNumber(number); construct = number; }
        traversal {
            callback = borrowed;
            argument = number;
            field sign: Sign;
            field magnitude: u32;
            call walk_sign(copy(sign));
            call visitor::visit_signed_number(borrowed(number));
        }
    }
    identity CatalogIdentity {
        value_type = crate::ast::CatalogIdentity;
        traversal {
            callback = borrowed;
            argument = identity;
            leaf visit_catalog_spelling: str = borrowed;
            field kind: CatalogKind;
            field spelling: str;
            call visitor::visit_catalog_identity(borrowed(identity));
            call visitor::visit_catalog_spelling(borrowed(spelling));
        }
    }

    construction spell: Ability {
        element Spell { effect: Sentence, }
        form spell = effect;
    }
    construction triggered: Ability {
        element Triggered {
            trigger: lex TriggerWord,
            event: Clause,
            effect: Sentence,
        }
        checked {
            visibility trigger = pub(crate);
            visibility event = pub(crate);
            visibility effect = pub(crate);
            constructor = Triggered::new(trigger, event, vec![effect]);
        }
        require event is Event;
        form triggered = lex(trigger) event "," effect;
    }
    construction imperative: Sentence {
        element Imperative { predicate: VerbPhrase, }
        derive predicate.agreement = Values::Bare;
        form imperative = predicate;
    }
    construction declarative: Sentence {
        element Declarative { subject: NounPhrase, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form declarative = subject predicate;
    }
    construction with_where: Sentence {
        element WithWhere { body: Sentence, clause: Clause, }
        require clause is Where;
        form with_where = body "," clause;
    }
    construction event: Clause {
        element EventClause { subject: NounPhrase, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form event = subject predicate;
    }
    construction where: Clause {
        element WhereClause { variable: lex Variable, value: NounPhrase, }
        derive verb.agreement = Values::ThirdPersonSingular;
        form where = "where" lex(variable) verb(VerbLexeme::Be)
            "the" "number" "of" value;
    }
    construction pronoun: NounPhrase {
        element PronounNp { word: lex Pronoun, }
        derive word.agreement = match word {
            It => Values::ThirdPersonSingular,
            You => Values::Bare,
        };
        derive agreement = word.agreement;
        derive number = Values::Singular;
        form pronoun = lex(word);
    }
    construction common: NounPhrase {
        element Common { article: lex Article, head: lex Noun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form common = lex(article) noun(head);
    }
    construction demonstrative: NounPhrase {
        element DemonstrativeNp { word: lex Demonstrative, head: lex Noun, }
        derive agreement = match word {
            That => Values::ThirdPersonSingular,
            Those => Values::Bare,
        };
        derive number = match word {
            That => Values::Singular,
            Those => Values::Plural,
        };
        form demonstrative = lex(word) noun(head);
    }
    construction target: NounPhrase {
        element TargetNp { head: lex Noun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form target = "target" noun(head);
    }
    construction self_reference: NounPhrase {
        element SelfReferenceNp { spelling: identity SelfReferenceSpelling, }
        checked {
            visibility spelling = private;
            access spelling = spelling;
            constructor = SelfReferenceNp::new(spelling, context);
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form self_reference = identity(spelling);
    }
    construction count: NounPhrase {
        element CountNp {
            head: lex Noun,
            controller: lex Pronoun,
            threshold: lex SignedNumber,
        }
        require controller is You;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive controller.agreement = match controller {
            It => Values::ThirdPersonSingular,
            You => Values::Bare,
        };
        derive verb.agreement = controller.agreement;
        form count = noun(head) lex(controller) verb(VerbLexeme::Control)
            "with" "power" lex(threshold) "or" "less";
    }
    construction destroy: VerbPhrase {
        element Destroy { object: NounPhrase, }
        derive agreement = verb.agreement;
        form destroy = verb(VerbLexeme::Destroy) object;
    }
    construction connive: VerbPhrase {
        element Connive {}
        derive agreement = verb.agreement;
        form connive = verb(VerbLexeme::Connive);
    }
    construction deal_damage: VerbPhrase {
        element DealDamage { amount: Amount, to: NounPhrase, }
        derive agreement = verb.agreement;
        form deal_damage = verb(VerbLexeme::Deal) amount "damage" "to" to;
    }
    construction gain_life: VerbPhrase {
        element GainLife { amount: Amount, }
        derive agreement = verb.agreement;
        form gain_life = verb(VerbLexeme::Gain) amount "life";
    }
    construction number: Amount {
        element NumberAmount { number: lex SignedNumber, }
        form number = lex(number);
    }
    construction variable: Amount {
        element VariableAmount { variable: lex Variable, }
        form variable = lex(variable);
    }

    root Ability { punctuation = "."; eoi = true; standalone_render = true; }
    root Sentence { punctuation = "."; eoi = false; standalone_render = true; }
}
"#;

#[test]
fn all_pinned_emission_items_match_the_normalized_english_v2_golden() {
    let manifest = pinned_manifest();
    assert_eq!(manifest.len(), 90, "literal pinned manifest cardinality");
    assert_eq!(
        manifest
            .iter()
            .map(|pin| &pin.key)
            .collect::<BTreeSet<_>>()
            .len(),
        90,
        "literal pinned manifest semantic keys are unique"
    );

    let invocation = deckmaste_construction_core::invocation_from_source(DECLARATION_SOURCE)
        .expect("the proof contains exactly one direct declaration invocation");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("the complete declaration validates and emits");
    let (golden, extraction_diagnostics) = load_golden(&manifest);
    let generated = generated_items(&expansion).expect("each generated item parses alone");

    compare_full_inventory(&manifest, &golden, &generated, extraction_diagnostics)
        .unwrap_or_else(|diagnostics| panic!("frozen 90-item proof failed:\n{diagnostics}"));
}

#[test]
fn golden_extractor_rejects_duplicate_item_keys() {
    let pinned = BTreeSet::from([SemanticKey::type_named("Repeated")]);
    let error = extract_pinned_items(
        "pub struct Repeated; pub enum Repeated { Unit }",
        "duplicate.rs",
        &pinned,
    )
    .expect_err("two pinned items with one semantic key must be rejected");

    assert!(error.contains("duplicate"), "{error}");
    assert!(error.contains("type Repeated"), "{error}");
    assert!(error.contains("duplicate.rs"), "{error}");
}

#[test]
fn golden_extractor_reports_every_missing_pinned_item() {
    let pinned = BTreeSet::from([
        SemanticKey::type_named("First"),
        SemanticKey::function("second"),
    ]);
    let diagnostics = inventory_diagnostics(&pinned, &[], &[]);

    assert!(
        diagnostics.contains("missing golden type First"),
        "{diagnostics}"
    );
    assert!(
        diagnostics.contains("missing generated function second"),
        "{diagnostics}"
    );
    assert_eq!(
        diagnostics.matches("missing golden").count(),
        2,
        "{diagnostics}"
    );
    assert_eq!(
        diagnostics.matches("missing generated").count(),
        2,
        "{diagnostics}"
    );
}

#[test]
fn golden_extractor_reports_every_unexpected_generated_item() {
    let pinned = BTreeSet::from([SemanticKey::type_named("Expected")]);
    let generated = [
        ExtractedItem {
            key: SemanticKey::type_named("Expected"),
            normalized: String::new(),
            origin: "generated: expected".to_owned(),
        },
        ExtractedItem {
            key: SemanticKey::function("surprise"),
            normalized: String::new(),
            origin: "generated: surprise".to_owned(),
        },
        ExtractedItem {
            key: SemanticKey::type_named("Extra"),
            normalized: String::new(),
            origin: "generated: extra".to_owned(),
        },
    ];
    let diagnostics = inventory_diagnostics(&pinned, &[], &generated);

    assert!(
        diagnostics.contains("unexpected generated function surprise"),
        "{diagnostics}"
    );
    assert!(
        diagnostics.contains("unexpected generated type Extra"),
        "{diagnostics}"
    );
    assert_eq!(diagnostics.matches("unexpected generated").count(), 2);
}

#[test]
fn normalized_item_comparison_ignores_only_formatting() {
    normalized_item_comparison(
        "pub struct Example { pub value: u32 }",
        "pub struct Example{pub value:u32}",
    )
    .expect("formatting differences normalize away");

    let mismatch = normalized_item_comparison(
        "pub struct Example { pub value: u32 }",
        "pub struct Example { pub value: u64 }",
    )
    .expect_err("a semantic type change must remain visible");
    assert!(mismatch.contains("--- golden"), "{mismatch}");
    assert!(mismatch.contains("+++ generated"), "{mismatch}");
    assert!(mismatch.contains("u32"), "{mismatch}");
    assert!(mismatch.contains("u64"), "{mismatch}");
}
