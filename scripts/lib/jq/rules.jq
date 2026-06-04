import "util" as util;

import "rules/cr" as $cr;
import "rules/keywords" as $keywords;

import "catalogs/artifact-types" as $artifact_types;
import "catalogs/battle-types" as $battle_types;
import "catalogs/creature-types" as $creature_types;
import "catalogs/enchantment-types" as $enchantment_types;
import "catalogs/land-types" as $land_types;
import "catalogs/planeswalker-types" as $planeswalker_types;
import "catalogs/spell-types" as $spell_types;


def cr: $cr[0];
def keywords: $keywords[0];

def keyword_abilities: keywords.keywordAbilities;
def keyword_actions: keywords.keywordActions;
def ability_words: keywords.abilityWords;

def artifact_types: $artifact_types[0].data;
def battle_types: $battle_types[0].data;
def creature_types: $creature_types[0].data;
def enchantment_types: $enchantment_types[0].data;
def land_types: $land_types[0].data;
def planeswalker_types: $planeswalker_types[0].data;
def spell_types: $spell_types[0].data;

def section:
    . as $rule
    | [$rule
        | while(. != null and startswith($rule); cr[.].navigation.nextRule)
        | cr[.]
        | select(. != null)
    ];

def format_section:
    map(
        # numbered rules ("100.2") get a trailing dot; lettered subrules
        # ("100.2a") do not -- matching the cr.txt layout.
        (if .ruleNumber | test("[a-z]$")
        then "\(.ruleNumber) "
        else "\(.ruleNumber). "
        end)
        + .ruleText
        + ((.examples // [])
            | map("\nExample: " + .)
            | add // ""
        )
    )
    | join("\n\n");


def get_keyword_cr:
    ascii_downcase as $k
    | (keywords
        | to_entries
        | map(.value |= map(ascii_downcase))
        | map(select(
            .value
            | map(. == $k)
            | any
        ))
        | first.key
    )
    | {
        keywordAbilities: "702",
        keywordActions: "701",
        abilityWords: "207.2c"
    }[.]
    | if . == "207.2c"
    then .
    else . as $rule
        | (cr
            | to_entries
            | map(select(
                 (.key | startswith($rule))
                 and ((.value.ruleText | ascii_downcase) == $k)))
            | first.key
        )
    end
    | section;

def all_keyword_crs:
    keywords
    | to_entries
    | map({
        category: .key,
        name: .value[]
    })
    | map(.rules = (.name | get_keyword_cr));

def get_subtype_cr:
    util::capitalize as $type
    | cr
    | to_entries
    | map(select(.key | startswith("205.3")))
    | map(select(.value.ruleText | startswith($type)))
    | first.value;

def get_subtype_list:
    get_subtype_cr.ruleText
    | . as $rule
    | capture("are called (?<category>\\w+ types)").category as $category
    | capture(@text "The \($category) are (?<list>.*)\\.").list
    | util::strip_parentheticals
    | split(", (?:and)?");
