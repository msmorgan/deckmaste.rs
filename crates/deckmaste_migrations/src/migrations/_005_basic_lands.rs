use crate::layout::PluginLayout;

pub(super) struct BasicLands;

impl super::Migration for BasicLands {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> { todo!() }
}
