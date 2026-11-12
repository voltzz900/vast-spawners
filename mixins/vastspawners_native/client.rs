// Vast Spawners, the game's side: injected into horizon-client's game.rs as `vastspawners_client`. The hooks:
// modifiers used on a spawner, its tick (unwatched and wired spawners), Silk Touch, placing a spawner with data,
// echoing loot, weakened mobs and the spawner item's tooltip.

use horizon_client::items::ItemStack;
use horizon_world::nbt::Tag;
use horizon_world::vastspawners::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

fn text(key: &str, fallback: &str) -> String {
    horizon_client::lang::get_or(&format!("vastspawners.{key}"), fallback)
}

fn shown(m: &Modifier, value: f64) -> String {
    match m.stat {
        Stat::Flag(_) => text(if value > 0.5 { "on" } else { "off" }, if value > 0.5 { "on" } else { "off" }),
        Stat::Health => format!("{}%", (value * 100.0).round()),
        Stat::Number("MinSpawnDelay" | "MaxSpawnDelay") => format!("{:.1}s", value / 20.0),
        _ => format!("{}", value as i64),
    }
}

// ---- hooks ---------------------------------------------------------------------------------------------------

/// At the head of `native_use_item_on`: a modifier used on a spawner changes it, and the item is spent.
pub(super) fn use_item_on(game: &mut super::Game<'_>, stack: &ItemStack, pos: [i32; 3], claimed: &mut bool) {
    // The server's game decides (a client's copy would only change what it shows).
    if *claimed || game.remote.is_some() || game.native_block(pos) != "minecraft:spawner" {
        return;
    }
    let Some(m) = MODIFIERS.iter().find(|m| matches(m.item, &stack.item)) else { return };
    let back = game.offhand.as_ref().is_some_and(|s| &*s.item == "minecraft:quartz");
    let Some(mut nbt) = game.block_entity_nbt(pos) else { return };
    *claimed = true;
    let Some(value) = apply(&mut nbt, m, back) else {
        game.native_feedback(horizon_client::text::Text::legacy(&format!(
            "§7{}: {}",
            text(&format!("stat.{}", m.name), m.name),
            text("at_limit", "already at its limit")
        )));
        return;
    };
    if !game.load_block_entity_nbt(pos, &horizon_world::snbt_text::to_snbt(&nbt)) {
        return;
    }
    if !game.creative {
        let slot = game.selected;
        if let Some(s) = game.inventory[slot].as_mut() {
            s.count = s.count.saturating_sub(1);
            if s.count == 0 {
                game.inventory[slot] = None;
            }
        }
    }
    game.native_feedback(horizon_client::text::Text::legacy(&format!(
        "§6{}: §e{}",
        text(&format!("stat.{}", m.name), m.name),
        shown(m, value)
    )));
}

/// `range`'s value as `server_tick_spawner_of` reads it: a wired spawner does nothing without a redstone signal (an
/// unwatched one has the widest range in its own data).
pub(super) fn spawner_range(game: &mut super::Game<'_>, key: (i32, i32, i32), spawner: &horizon_client::mobs::spawner::Spawner, range: f64) -> f64 {
    let flagged = |name: &str| {
        let wanted = format!("{TAG}{name}");
        let has = |entity: &horizon_world::nbt::Compound| tags(&Tag::Compound(entity.clone())).iter().any(|t| t == &wanted);
        spawner.potentials.first().is_some_and(|(d, _)| has(&d.entity))
            || matches!(&spawner.next, Some(horizon_client::chunk::block_entity::Next::Own(d)) if has(&d.entity))
    };
    if flagged("wired") {
        // The level in place is the spawner's (the native read refuses a game that isn't serving: one player).
        let at = horizon_core::position::BlockPos::new(key.0, key.1, key.2);
        if game.with_level(|b, level, ticks| b.best_neighbor_signal(level, ticks, at)) == 0 {
            // No player is within no blocks (a negative range would mean any distance).
            return 0.0;
        }
    }
    range
}

/// Spawners being mined with Silk Touch, by position: their data, for the drop.
static MINED: Mutex<Option<HashMap<[i32; 3], Tag>>> = Mutex::new(None);

/// At the head of `native_destroy_block`: a spawner mined with Silk Touch keeps its data for its drop.
pub(super) fn destroy_block(game: &mut super::Game<'_>, pos: [i32; 3]) {
    if game.creative || game.remote.is_some() || game.native_block(pos) != "minecraft:spawner" {
        return;
    }
    let silk = game.inventory[game.selected]
        .as_ref()
        .is_some_and(|s| s.enchantments().iter().any(|(id, level)| &**id == "minecraft:silk_touch" && *level > 0));
    if !silk {
        return;
    }
    let Some(mut nbt) = game.block_entity_nbt(pos) else { return };
    for key in ["x", "y", "z", "Delay"] {
        put(&mut nbt, key, None);
    }
    put(&mut nbt, "id", Some(Tag::String("minecraft:mob_spawner".into())));
    if let Ok(mut m) = MINED.lock() {
        m.get_or_insert_with(HashMap::new).insert(pos, nbt);
    }
}

/// At the head of `native_loot_drops`: a spawner mined with Silk Touch drops itself, with its data; a mob from a
/// spawner with echoes drops its loot that many more times.
pub(super) fn loot_drops(game: &mut super::Game<'_>, context: &horizon_client::mod_api::NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    if let Some((pos, _)) = context.block {
        let nbt = MINED.lock().ok().and_then(|mut m| m.as_mut().and_then(|m| m.remove(&pos)));
        if let Some(nbt) = nbt {
            let mut spawner = ItemStack::new("minecraft:spawner", 1);
            spawner.data_mut().set_other("minecraft:block_entity_data", Some(nbt));
            drops.push(spawner);
        }
        return;
    }
    let Some((id, _)) = context.entity else { return };
    let echoes = game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("Tags"))
        .and_then(|t| match t {
            Tag::List(list) => list.iter().find_map(|t| match t {
                Tag::String(s) => s.to_string_lossy().strip_prefix("vastspawners.echo=").and_then(|v| v.parse::<f64>().ok()),
                _ => None,
            }),
            _ => None,
        })
        .unwrap_or(0.0) as usize;
    if echoes > 0 {
        let once = drops.clone();
        for _ in 0..echoes.min(3) {
            drops.extend(once.iter().cloned());
        }
    }
}

/// At `native_place_block`: a spawner placed from an item that carries spawner data gets that data back (the game
/// lets only operators place spawner data, as vanilla does).
pub(super) fn place_block(game: &mut super::Game<'_>, stack: &ItemStack, pos: [i32; 3]) {
    if &*stack.item != "minecraft:spawner" || game.remote.is_some() {
        return;
    }
    let Some(data) = stack.data().and_then(|d| d.other("minecraft:block_entity_data")).cloned() else { return };
    let Some(mut nbt) = game.block_entity_nbt(pos) else { return };
    if let (Tag::Compound(target), Tag::Compound(source)) = (&mut nbt, &data) {
        for (k, v) in source {
            let key = k.to_string_lossy();
            if !matches!(key.as_str(), "id" | "x" | "y" | "z") {
                target.insert(k.clone(), v.clone());
            }
        }
    }
    game.load_block_entity_nbt(pos, &horizon_world::snbt_text::to_snbt(&nbt));
}

static TICKS: AtomicU64 = AtomicU64::new(0);

/// At the head of `native_server_tick`: a mob a weakening spawner made comes down to its share of health, once.
pub(super) fn server_tick(game: &mut super::Game<'_>) {
    if TICKS.fetch_add(1, Ordering::Relaxed) % 10 != 0 || game.remote.is_some() {
        return;
    }
    let weakened: Vec<(u32, f64, String)> = game
        .mobs
        .iter()
        .filter(|m| !m.removed)
        .filter_map(|m| {
            let raw = game.persist.raw.get(&m.id)?;
            let Some(Tag::List(list)) = raw.get("Tags") else { return None };
            list.iter().find_map(|t| match t {
                Tag::String(s) => {
                    let s = s.to_string_lossy();
                    s.strip_prefix("vastspawners.vigour=").and_then(|v| v.parse::<f64>().ok()).map(|v| (m.id, v, s.clone()))
                }
                _ => None,
            })
        })
        .collect();
    let dimension = game.dim.id().to_owned();
    for (id, share, tag) in weakened {
        let Some(mob) = game.native_mob(&dimension, id) else { continue };
        game.native_set_mob_health(&dimension, id, (mob.max_health * share as f32).max(1.0));
        if let Some(entity) = game.native_entity(id) {
            game.native_entity_remove_tag(entity, &tag);
        }
    }
}

/// The lines that tell a spawner's data: what it spawns and what it was changed to. `all`: every number, changed or
/// not (Jade, on a placed spawner), else only the changed ones (an item's tooltip).
pub(super) fn stat_lines(nbt: &Tag, all: bool) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(Tag::String(kind)) = first_entity(nbt).and_then(|e| e.get("id")) {
        lines.push(format!("§7{}: §f{}", text("mob", "Mob"), kind.to_string_lossy()));
    }
    for m in MODIFIERS {
        let value = match m.stat {
            Stat::Number(key) => {
                let v = number(nbt, key);
                let default = number(&Tag::compound(Vec::<(&str, Tag)>::new()), key);
                if !all && (v - default).abs() < 1e-9 {
                    continue;
                }
                v
            }
            Stat::Flag(name) => {
                if !flag(nbt, name) {
                    continue;
                }
                1.0
            }
            Stat::Echo => match counted(nbt, "echo") {
                Some(v) => v,
                None => continue,
            },
            Stat::Health => match counted(nbt, "vigour") {
                Some(v) => v,
                None => continue,
            },
        };
        lines.push(format!("§9{}: {}", text(&format!("stat.{}", m.name), m.name), shown(m, value)));
    }
    lines
}

/// What a modifier does in one use: `-0.5s`, `+2`, `on`.
fn change(m: &Modifier, back: bool) -> String {
    let step = if back { m.back_step } else { m.step };
    match m.stat {
        Stat::Flag(_) => text(if back { "off" } else { "on" }, if back { "off" } else { "on" }),
        Stat::Health => format!("{:+}%", (step * 100.0).round()),
        Stat::Number("MinSpawnDelay" | "MaxSpawnDelay") => format!("{:+.1}s", step / 20.0),
        _ => format!("{:+}", step as i64),
    }
}

/// The tooltip of a spawner item that carries spawner data (what it spawns and what it was changed to), and of a
/// modifier (what it does to a spawner): the item list of a recipe viewer shows it too.
pub(super) fn tooltip(stack: &ItemStack, lines: &mut Vec<(horizon_client::text::Text, u32)>) {
    let line = |s: String| (horizon_client::text::Text::legacy(&s), u32::MAX);
    if let Some(m) = MODIFIERS.iter().find(|m| matches(m.item, &stack.item)) {
        let limit = match m.stat {
            Stat::Flag(_) => String::new(),
            _ => format!(" §8({} {})", text("limit", "limit"), shown(m, m.limit)),
        };
        lines.push(line(format!(
            "§7{}: §9{} {}{limit}",
            text("modifier", "Spawner modifier"),
            text(&format!("stat.{}", m.name), m.name),
            change(m, false)
        )));
        lines.push(line(format!("§8{}: {}", text("reverse", "With quartz in the off hand"), change(m, true))));
        return;
    }
    if &*stack.item != "minecraft:spawner" {
        return;
    }
    let Some(nbt) = stack.data().and_then(|d| d.other("minecraft:block_entity_data")) else { return };
    lines.extend(stat_lines(nbt, false).into_iter().map(line));
}

// ---- Jade --------------------------------------------------------------------------------------------------------

/// Adds Vast Spawners' lines to Jade (once), when Jade is in the build.
pub(super) fn register_integrations() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        vastspawners_optional::with_mod!("jade", {
            super::jade::register_option("vastspawners_spawner", "Vast Spawners", true);
            super::jade::register_server(jade_spawner);
        });
        vastspawners_optional::with_source!("pub fn register_category(category: PluginCategory)", {
            super::justenoughitems_glue::register_category(super::justenoughitems_glue::PluginCategory {
                id: "vastspawners:modifiers",
                title: |_| text("jei.title", "Spawner Modifiers"),
                width: 150.0,
                height: 52.0,
                catalysts: &["minecraft:spawner"],
                recipes: jei_recipes,
            });
        });
    });
}

/// The items a modifier is (every wool for `#wool`).
#[allow(dead_code)]
fn modifier_items(m: &Modifier) -> Vec<String> {
    const WOOLS: [&str; 16] = [
        "white", "light_gray", "gray", "black", "brown", "red", "orange", "yellow", "lime", "green", "cyan", "light_blue", "blue",
        "purple", "magenta", "pink",
    ];
    if m.item == "#wool" { WOOLS.iter().map(|c| format!("minecraft:{c}_wool")).collect() } else { vec![m.item.to_owned()] }
}

/// The words of a modifier's recipe: what one use does, its limit and what it does with quartz in the off hand.
#[allow(dead_code)]
fn jei_lines(m: &Modifier) -> Vec<String> {
    let mut lines = vec![format!("{}: {}", text(&format!("stat.{}", m.name), m.name), change(m, false))];
    if !matches!(m.stat, Stat::Flag(_)) {
        lines.push(format!("{} {}", text("limit", "limit"), shown(m, m.limit)));
    }
    lines.push(format!("{}: {}", text("quartz", "With quartz"), change(m, true)));
    lines
}

vastspawners_optional::with_source!("pub fn register_category(category: PluginCategory)", {
    /// JEI's "Spawner Modifiers": each modifier used on a spawner, with what it does.
    fn jei_recipes(_: &super::Game<'_>) -> Vec<super::justenoughitems_glue::PluginRecipe> {
        MODIFIERS
            .iter()
            .map(|m| super::justenoughitems_glue::PluginRecipe {
                inputs: vec![modifier_items(m), vec!["minecraft:spawner".to_owned()]],
                outputs: Vec::new(),
                lines: jei_lines(m),
            })
            .collect()
    }
});

vastspawners_optional::with_mod!("jade", {
    /// Jade's lines for a spawner: what it spawns and each of its numbers, with what it was changed to.
    fn jade_spawner(game: &mut super::Game<'_>, pos: [i32; 3], block: &str, lines: &mut Vec<String>) {
        if block != "minecraft:spawner" || !super::jade::plugin_on(game, "vastspawners_spawner") {
            return;
        }
        let Some(nbt) = game.block_entity_nbt(pos) else { return };
        // Jade's own title already names the mob.
        lines.extend(stat_lines(&nbt, true).into_iter().skip_while(|l| l.starts_with("§7")));
    }
});
