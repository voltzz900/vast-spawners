// A spawner placed from an item with spawner data gets it back.
fn vastspawners_place_block(game: &mut crate::game::Game<'_>, stack: &ItemStack, pos: [i32; 3]) {
    crate::game::vastspawners_client::place_block(game, stack, pos);
}
