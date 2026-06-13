#!/usr/bin/env fish

source (status dirname)/lib/setup.fish
or return

function fetch_mtgjson
    set wanted_files \
        Meta.json \
        CompiledList.json \
        EnumValues.json \
        Keywords.json \
        CardTypes.json \
        SetList.json \
        AllPrintings.json \
        AtomicCards.json \
        AllSetFiles.tar \
        AllPrintings.sqlite

    mtgjson download $wanted_files
end

function fetch_rules
    set wanted_files_map \
        link/cr:cr.txt \
        cr:cr.json \
        cr/keywords:keywords.json \
        cr/glossary:glossary.json \
        cr/unofficial-glossary:unofficial-glossary.json \
        mtr:mtr.json

    academyruins download $wanted_files_map
end

function fetch_scryfall_catalogs
    set wanted_catalogs \
        card-names artist-names word-bank supertypes card-types artifact-types \
        battle-types creature-types enchantment-types land-types planeswalker-types \
        spell-types powers toughnesses loyalties keyword-abilities keyword-actions \
        ability-words flavor-words watermarks

    for catalog in $wanted_catalogs
        set -a wanted_files_map catalog/$catalog:$catalog.json
    end

    scryfall download $wanted_files_map
end

fetch_mtgjson
fetch_rules
fetch_scryfall_catalogs
