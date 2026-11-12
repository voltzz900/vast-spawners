// The mod's row in Options > Mods.
fn vastspawners_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::vastspawners_client::register_integrations();
    use horizon_world::vastspawners as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
