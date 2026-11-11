// The range a spawner looks for a player in: everywhere when unwatched, nowhere when wired and unpowered.
fn vastspawners_spawner_range(
    range: f64,
    game: &mut crate::game::Game<'_>,
    key: (i32, i32, i32),
    spawner: &horizon_client::mobs::spawner::Spawner,
) -> f64 {
    crate::game::vastspawners_client::spawner_range(game, key, spawner, range)
}
