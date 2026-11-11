// Drops: the mined spawner itself, and echoing mobs' loot.
fn vastspawners_loot_drops(
    game: &mut crate::game::Game<'_>,
    context: &horizon_client::mod_api::NativeLootDrops<'_>,
    drops: &mut Vec<ItemStack>,
) {
    crate::game::vastspawners_client::loot_drops(game, context, drops);
}
