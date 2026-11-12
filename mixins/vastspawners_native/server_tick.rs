// Weakened mobs from a spawner come down to their share of health.
fn vastspawners_server_tick(game: &mut crate::game::Game<'_>) {
    crate::game::vastspawners_client::register_integrations();
    crate::game::vastspawners_client::server_tick(game);
}
