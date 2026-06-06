import "rules" as rules;
import "util" as util;

def from_all_printings_file: [.data[].cards[]];
def from_set_file: .data.cards;
def from_atomic_cards_file: [.data[][]];

# We count non-null, non-"Banned" as legal.
def is_legal($format):
    (.legalities[$format] // "Banned") != "Banned";

def is_supported:
    is_legal("vintage")
    and .layout != "reversible_card";

# Input: colorIndicator (string[] or null)
def join_color_indicator: .
    | . // []
    | join("")
    | if length == 0 then null else . end;

# 200.1. The parts of a card are name, mana cost, illustration, color indicator,
# type line, expansion symbol, text box, power and toughness, loyalty, defense,
# hand modifier, life modifier, illustration credit, legal text, and collector number.
# Some cards may have more than one of any or all of these parts.
def card_parts: .
    | {
        oracleId: .identifiers.scryfallOracleId,
        name: .faceName // .name,
        manaCost,
#        illustrationId: .identifiers.scryfallIllustrationId,
        colorIndicator: .colorIndicator | join_color_indicator,
        type,
        setCode,
        text,
        flavorText,
        power,
        toughness,
        loyalty,
        defense,
#        handModifier,
#        lifeModifier,
        artist,
        number,
    }
;

# 200.2. Some parts of a card are also characteristics of the object that has them.
# See rule 109.3.
#
# 109.3. An object's characteristics are name, mana cost, color, color indicator,
# card type, subtype, supertype, rules text, abilities, power, toughness, loyalty,
# defense, hand modifier, and life modifier. Objects can have some or all of these
# characteristics. Any other information about an object isn't a characteristic.
# For example, characteristics don't include whether a permanent is tapped, a spell's
# target, an object's owner or controller, what an Aura enchants, and so on.
def characteristics: .
    | {
        oracleId: .identifiers.scryfallOracleId,
        name: .faceName // .name,
        manaCost,
        colors,
        colorIndicator: if .colorIndicator | length > 0 then .colorIndicator else null end,
        types,
        subtypes,
        supertypes,
        text,
        power,
        toughness,
        loyalty,
        defense,
#        handModifier,
#        lifeModifier,
    }
;

# Input: oracle text (string)
def strip_reminder_text: util::strip_parentheticals;

# Input: oracle text (string)
def expand_keyword_lines: .
    | split("\n")
    | map(
        . as $line
        | split(", ")
        | map(util::capitalize)
        | if map([ startswith(rules::keyword_abilities[]) ] | any) | all
        then .[]
        else $line
        end
    )
    | join("\n");

def ron_color: .
    | if . == null then null
    else {
            "W": "White",
            "U": "Blue",
            "B": "Black",
            "R": "Red",
            "G": "Green",
            "C": "Colorless",
        }[.]
    end
;

# Input: oracle text (string)
def normalize_text: .
    | strings
    | strip_reminder_text
    | expand_keyword_lines;

def parse_mana_symbol: .
    | capture("^\\{(?:
            (?<variable>X)
            |(?<snow>S)
            |(?:
                (?:
                    (?<generic>[0-9]|[1-9][0-9]+)
                    |(?<color>[WUBRGC])
                )
                (?:
                    /
                    (?<hybrid_color>[WUBRG])
                )?
                (?:
                    /
                    (?<phyrexian>P)
                )?
            )
        )\\}$"; "x")
    | with_entries(select(.value != null))
;

def mana_symbol_ron: .
    | .phyrexian as $phyrexian
    | (.hybrid_color | ron_color) as $hybrid_color
    | if .variable then "Variable"
    elif .snow then "Snow"
    elif .generic then @text "Generic(\(.generic))"
    elif .color then .color | ron_color
    end
    | if $hybrid_color then @text "Hybrid(\(.), \($hybrid_color))" end
    | if $phyrexian then @text "Phyrexian(\(.))" end
;

def parse_mana_cost: .
    | [ scan("\\{[^}]+\\}") ]
    | map(parse_mana_symbol)
;

def mana_cost_ron: .
    | map(mana_symbol_ron)
    | @text "[\(join(", "))]"
;

def color_indicator_ron: .
    | map(ron_color)
    | @text "[\(join(", "))]"
;

def subtype_ron: .
    | . as $subtype
    | {
        "Creature": rules::creature_types,
        "Artifact": rules::artifact_types,
        "Enchantment": rules::enchantment_types,
        "Land": rules::land_types,
        "Battle": rules::battle_types,
        "Planeswalker": rules::planeswalker_types,
        "Spell": rules::spell_types,
    }
    | to_entries
    | map(select([$subtype == .value[]] | any))
    | first.key
    | @text "\(.)(\($subtype))"
;

def subtypes_ron: .
    | map(subtype_ron)
    | @text "[\(join(", "))]"
;

def number_ron: .
    | (tonumber? | @text "Number(\(.))") // @text "NonNumber(\"\(.)\")"
;

def render_fields: .
    | .colorIndicatorRon = (.colorIndicator | util::and_then(color_indicator_ron))
    | .manaCostRon = (.manaCost | util::and_then(parse_mana_cost | mana_cost_ron))
    | .subtypesRon = (.subtypes | util::and_then(subtypes_ron))
    | .powerRon = (.power | util::and_then(number_ron))
    | .toughnessRon = (.toughness | util::and_then(number_ron))
    | .loyaltyRon = (.loyalty | util::and_then(number_ron))
    | .defenseRon = (.defense | util::and_then(number_ron))

;

def process_face: .
    | characteristics
    | .text |= normalize_text
    | render_fields
;

def atomic_card_characteristics: .
    | .data
    | to_entries
    | map({name: .key, faces: .value, layout: .faces[0].layout})
    | map(
        .faces |= map(
            select(is_supported)
            | process_face
        )
        | select(.faces | length > 0)
    )
;
