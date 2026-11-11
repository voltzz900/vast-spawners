// A modifier used on a spawner.
fn vastspawners_use_item_on(game: &mut crate::game::Game<'_>, stack: &ItemStack, pos: [i32; 3], claimed: &mut bool) {
    crate::game::vastspawners_client::use_item_on(game, stack, pos, claimed);
}
