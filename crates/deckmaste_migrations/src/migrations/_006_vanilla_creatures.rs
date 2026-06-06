use crate::layout::PluginLayout;

pub(super) struct VanillaCreatures;

impl super::Migration for VanillaCreatures {
    fn apply(&self, _plugin: &PluginLayout) -> anyhow::Result<()> { todo!() }
}
