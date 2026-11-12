// Jade's and JEI's plugins, added once (the client may never tick a server).
fn vastspawners_frame(game: &mut crate::game::Game<'_>) {
    let _ = game;
    crate::game::vastspawners_client::register_integrations();
}
