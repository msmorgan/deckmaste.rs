#!/usr/bin/fish

source (path resolve (status dirname)/../lib/setup.fish)
or return

set -l plugin_dir (check_plugin_dir -n (status filename) $argv[1])
or return

set -l types_dir $plugin_dir/types


create_subtypes artifact_types $types_dir/artifact Artifact
create_subtypes battle_types $types_dir/battle Battle
create_subtypes creature_types $types_dir/creature Creature
create_subtypes enchantment_types $types_dir/enchantment Enchantment
create_subtypes land_types $types_dir/land Land
create_subtypes planeswalker_types $types_dir/planeswalker Planeswalker
create_subtypes spell_types $types_dir/spell Spell
