// The spawner item's data in its tooltip.
fn vastspawners_tooltip(stack: &horizon_client::items::ItemStack, lines: &mut Vec<(horizon_client::text::Text, u32)>) {
    crate::game::vastspawners_client::tooltip(stack, lines);
}
