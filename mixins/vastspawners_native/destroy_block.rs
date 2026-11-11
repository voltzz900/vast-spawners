// A spawner mined with Silk Touch keeps its data for its drop.
fn vastspawners_destroy_block(game: &mut crate::game::Game<'_>, pos: [i32; 3]) {
    crate::game::vastspawners_client::destroy_block(game, pos);
}
