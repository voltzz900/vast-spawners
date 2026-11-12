// Vast Spawners: what a spawner's data holds and how the modifiers change it (no game here: the hooks are in
// client.rs). Injected into horizon-world as `pub mod vastspawners`.
//
// Built from the MIT code and data of Apothic Spawners (by Shadows_of_Fire). Everything a spawner carries lives in
// its own data: the game's six numbers, light rules in its spawn data, and the rest as tags on the mob it makes
// (`vastspawners.*`), which the mob keeps.

use crate::nbt::{JavaString, Tag};

pub const ID: &str = "vastspawners_native";
pub const NAME: &str = "Vast Spawners";
pub const VERSION: &str = "0.2.1";
pub const DESCRIPTION: &str = "Change spawners with items, mine them with Silk Touch, and shape the mobs they make.";

/// What a modifier changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    /// One of the spawner's own numbers (its NBT key).
    Number(&'static str),
    /// A flag kept as a tag on the spawned mob (and read by the spawner's tick for two of them).
    Flag(&'static str),
    /// The echo count, 0 to 3: the mob's loot rolls that many more times.
    Echo,
    /// The mob's health as it spawns, 20% to 100%.
    Health,
}

/// A modifier: the item held in the main hand, the change, and its bound; quartz in the other hand turns it back.
pub struct Modifier {
    pub item: &'static str,
    pub stat: Stat,
    pub step: f64,
    pub limit: f64,
    pub back_step: f64,
    pub back_limit: f64,
    /// The name shown for what it changes (a lang key's tail).
    pub name: &'static str,
}

/// Apothic Spawners' modifiers (MIT data), with this mod's names.
pub const MODIFIERS: &[Modifier] = &[
    Modifier { item: "minecraft:sugar", stat: Stat::Number("MinSpawnDelay"), step: -10.0, limit: 20.0, back_step: 10.0, back_limit: 1600.0, name: "shortest_wait" },
    Modifier { item: "minecraft:clock", stat: Stat::Number("MaxSpawnDelay"), step: -20.0, limit: 20.0, back_step: 20.0, back_limit: 1600.0, name: "longest_wait" },
    Modifier { item: "minecraft:fermented_spider_eye", stat: Stat::Number("SpawnCount"), step: 2.0, limit: 16.0, back_step: -2.0, back_limit: 1.0, name: "batch" },
    Modifier { item: "minecraft:ghast_tear", stat: Stat::Number("MaxNearbyEntities"), step: 2.0, limit: 32.0, back_step: -2.0, back_limit: 1.0, name: "crowd" },
    Modifier { item: "minecraft:prismarine_crystals", stat: Stat::Number("RequiredPlayerRange"), step: 4.0, limit: 48.0, back_step: -4.0, back_limit: 1.0, name: "reach" },
    Modifier { item: "minecraft:piston", stat: Stat::Number("SpawnRange"), step: 2.0, limit: 32.0, back_step: -2.0, back_limit: 1.0, name: "spread" },
    Modifier { item: "minecraft:nether_star", stat: Stat::Flag("unwatched"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "unwatched" },
    Modifier { item: "minecraft:conduit", stat: Stat::Flag("anywhere"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "anywhere" },
    Modifier { item: "minecraft:soul_lantern", stat: Stat::Flag("lightproof"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "lightproof" },
    Modifier { item: "minecraft:comparator", stat: Stat::Flag("wired"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "wired" },
    Modifier { item: "minecraft:chorus_fruit", stat: Stat::Flag("dazed"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "dazed" },
    Modifier { item: "#wool", stat: Stat::Flag("hushed"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "hushed" },
    Modifier { item: "minecraft:golden_dandelion", stat: Stat::Flag("young"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "young" },
    Modifier { item: "minecraft:campfire", stat: Stat::Flag("kindled"), step: 1.0, limit: 1.0, back_step: -1.0, back_limit: 0.0, name: "kindled" },
    Modifier { item: "minecraft:echo_shard", stat: Stat::Echo, step: 1.0, limit: 3.0, back_step: -1.0, back_limit: 0.0, name: "echoes" },
    Modifier { item: "minecraft:pointed_dripstone", stat: Stat::Health, step: -0.05, limit: 0.2, back_step: 0.05, back_limit: 1.0, name: "vigour" },
];

pub const TAG: &str = "vastspawners.";

pub fn matches(pattern: &str, item: &str) -> bool {
    match pattern.strip_prefix('#') {
        Some("wool") => item.starts_with("minecraft:") && item.ends_with("_wool"),
        Some(_) => false,
        None => pattern == item,
    }
}

// ---- the spawner's data ----------------------------------------------------------------------------------------

/// The mob compounds a spawner's data holds: its next spawn's and each potential's.
pub fn entities_mut(nbt: &mut Tag) -> Vec<&mut Tag> {
    let mut out = Vec::new();
    let Tag::Compound(map) = nbt else { return out };
    for (key, value) in map.iter_mut() {
        match key.to_string_lossy().as_str() {
            "SpawnData" => {
                if let Some(e) = value.get_mut("entity") {
                    out.push(e);
                }
            }
            "SpawnPotentials" => {
                if let Tag::List(list) = value {
                    for p in list.iter_mut() {
                        if let Some(e) = p.get_mut("data").and_then(|d| d.get_mut("entity")) {
                            out.push(e);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// The spawn data compounds (`SpawnData` and each potential's `data`), for their light rules.
pub fn spawn_datas_mut(nbt: &mut Tag) -> Vec<&mut Tag> {
    let mut out = Vec::new();
    let Tag::Compound(map) = nbt else { return out };
    for (key, value) in map.iter_mut() {
        match key.to_string_lossy().as_str() {
            "SpawnData" => out.push(value),
            "SpawnPotentials" => {
                if let Tag::List(list) = value {
                    for p in list.iter_mut() {
                        if let Some(d) = p.get_mut("data") {
                            out.push(d);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

pub fn tags(entity: &Tag) -> Vec<String> {
    match entity.get("Tags") {
        Some(Tag::List(list)) => list
            .iter()
            .filter_map(|t| match t {
                Tag::String(s) => Some(s.to_string_lossy()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub fn set_tags(entity: &mut Tag, tags: &[String]) {
    if let Tag::Compound(map) = entity {
        if tags.is_empty() {
            map.remove(&JavaString::from("Tags"));
        } else {
            map.insert("Tags".into(), Tag::List(tags.iter().map(|t| Tag::String(t.as_str().into())).collect()));
        }
    }
}

pub fn put(entity: &mut Tag, key: &str, value: Option<Tag>) {
    if let Tag::Compound(map) = entity {
        match value {
            Some(v) => {
                map.insert(key.into(), v);
            }
            None => {
                map.remove(&JavaString::from(key));
            }
        }
    }
}

/// A flag's value on the spawner (its first mob's tags).
pub fn flag(nbt: &Tag, name: &str) -> bool {
    first_entity(nbt).is_some_and(|e| tags(e).iter().any(|t| t == &format!("{TAG}{name}")))
}

/// A counted value on the spawner (`vastspawners.<name>=<n>`).
pub fn counted(nbt: &Tag, name: &str) -> Option<f64> {
    let prefix = format!("{TAG}{name}=");
    first_entity(nbt).and_then(|e| tags(e).iter().find_map(|t| t.strip_prefix(&prefix).and_then(|v| v.parse().ok())))
}

pub fn first_entity(nbt: &Tag) -> Option<&Tag> {
    nbt.get("SpawnData").and_then(|d| d.get("entity")).or_else(|| match nbt.get("SpawnPotentials") {
        Some(Tag::List(list)) => list.first().and_then(|p| p.get("data")).and_then(|d| d.get("entity")),
        _ => None,
    })
}

pub fn number(nbt: &Tag, key: &str) -> f64 {
    if key == "RequiredPlayerRange" && let Some(reach) = counted(nbt, "reach") {
        return reach;
    }
    match nbt.get(key) {
        Some(Tag::Short(v)) => f64::from(*v),
        Some(Tag::Int(v)) => f64::from(*v),
        _ => match key {
            "MinSpawnDelay" => 200.0,
            "MaxSpawnDelay" => 800.0,
            "SpawnCount" => 4.0,
            "MaxNearbyEntities" => 6.0,
            "RequiredPlayerRange" => 16.0,
            _ => 4.0,
        },
    }
}

/// Sets flag `name` on every mob the spawner makes, with what it does to the mob.
pub fn set_flag(nbt: &mut Tag, name: &str, on: bool) {
    let key = format!("{TAG}{name}");
    for e in entities_mut(nbt) {
        let mut t = tags(e);
        t.retain(|x| x != &key);
        if on {
            t.push(key.clone());
        }
        set_tags(e, &t);
        match name {
            "dazed" => put(e, "NoAI", on.then_some(Tag::Byte(1))),
            "hushed" => put(e, "Silent", on.then_some(Tag::Byte(1))),
            "young" => {
                put(e, "IsBaby", on.then_some(Tag::Byte(1)));
                put(e, "Age", on.then_some(Tag::Int(-24000)));
            }
            "kindled" => put(e, "Fire", on.then_some(Tag::Short(i16::MAX))),
            _ => {}
        }
    }
    if name == "unwatched" {
        // The spawner looks for a player as far as its range says, twice in its tick: an unwatched one keeps its
        // reach in a tag and the widest range the game stores.
        if on {
            let reach = number(nbt, "RequiredPlayerRange");
            set_counted(nbt, "reach", Some(reach));
            put(nbt, "RequiredPlayerRange", Some(Tag::Short(i16::MAX)));
        } else {
            let reach = counted(nbt, "reach").unwrap_or(16.0);
            set_counted(nbt, "reach", None);
            put(nbt, "RequiredPlayerRange", Some(Tag::Short(reach as i16)));
        }
    }
    if matches!(name, "anywhere" | "lightproof") {
        // Spawn data with light rules skips the mob's own spawn rules: both flags open every light level.
        let open = on || (name == "anywhere" && flag(nbt, "lightproof")) || (name == "lightproof" && flag(nbt, "anywhere"));
        for d in spawn_datas_mut(nbt) {
            let rules = Tag::compound([
                ("block_light_limit", Tag::List(vec![Tag::Int(0), Tag::Int(15)])),
                ("sky_light_limit", Tag::List(vec![Tag::Int(0), Tag::Int(15)])),
            ]);
            put(d, "custom_spawn_rules", open.then_some(rules));
        }
    }
}

pub fn set_counted(nbt: &mut Tag, name: &str, value: Option<f64>) {
    let prefix = format!("{TAG}{name}=");
    for e in entities_mut(nbt) {
        let mut t = tags(e);
        t.retain(|x| !x.starts_with(&prefix));
        if let Some(v) = value {
            t.push(format!("{prefix}{}", (v * 100.0).round() / 100.0));
        }
        set_tags(e, &t);
    }
}

/// Applies `m` (or its way back) to a spawner's data: the new value, or None when it is at its bound.
pub fn apply(nbt: &mut Tag, m: &Modifier, back: bool) -> Option<f64> {
    let (step, limit) = if back { (m.back_step, m.back_limit) } else { (m.step, m.limit) };
    let bounded = |now: f64| {
        let next = now + step;
        let next = if step > 0.0 { next.min(limit) } else { next.max(limit) };
        ((next - now).abs() > 1e-9).then_some(next)
    };
    match m.stat {
        Stat::Number("RequiredPlayerRange") if flag(nbt, "unwatched") => {
            let next = bounded(counted(nbt, "reach").unwrap_or(16.0))?;
            set_counted(nbt, "reach", Some(next));
            Some(next)
        }
        Stat::Number(key) => {
            let mut next = bounded(number(nbt, key))?;
            // The shortest wait stays at most the longest, and the other way round.
            if key == "MinSpawnDelay" {
                next = next.min(number(nbt, "MaxSpawnDelay"));
            }
            if key == "MaxSpawnDelay" {
                next = next.max(number(nbt, "MinSpawnDelay"));
            }
            if (next - number(nbt, key)).abs() < 1e-9 {
                return None;
            }
            put(nbt, key, Some(Tag::Short(next as i16)));
            Some(next)
        }
        Stat::Flag(name) => {
            let on = !back;
            if flag(nbt, name) == on {
                return None;
            }
            set_flag(nbt, name, on);
            Some(f64::from(u8::from(on)))
        }
        Stat::Echo => {
            let next = bounded(counted(nbt, "echo").unwrap_or(0.0))?;
            set_counted(nbt, "echo", (next > 0.0).then_some(next));
            Some(next)
        }
        Stat::Health => {
            let next = bounded(counted(nbt, "vigour").unwrap_or(1.0))?;
            set_counted(nbt, "vigour", (next < 1.0).then_some(next));
            Some(next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawner() -> Tag {
        Tag::compound([
            ("SpawnData", Tag::compound([("entity", Tag::compound([("id", Tag::String("minecraft:zombie".into()))]))])),
            (
                "SpawnPotentials",
                Tag::List(vec![Tag::compound([
                    ("data", Tag::compound([("entity", Tag::compound([("id", Tag::String("minecraft:zombie".into()))]))])),
                    ("weight", Tag::Int(1)),
                ])]),
            ),
            ("MinSpawnDelay", Tag::Short(200)),
            ("MaxSpawnDelay", Tag::Short(800)),
        ])
    }

    fn modifier(item: &str) -> &'static Modifier {
        MODIFIERS.iter().find(|m| matches(m.item, item)).unwrap()
    }

    #[test]
    fn numbers_move_within_bounds() {
        let mut nbt = spawner();
        assert_eq!(apply(&mut nbt, modifier("minecraft:sugar"), false), Some(190.0));
        assert_eq!(apply(&mut nbt, modifier("minecraft:sugar"), true), Some(200.0));
        let piston = modifier("minecraft:piston");
        for _ in 0..20 {
            apply(&mut nbt, piston, false);
        }
        assert_eq!(number(&nbt, "SpawnRange"), 32.0);
        assert_eq!(apply(&mut nbt, piston, false), None, "at its bound");
    }

    #[test]
    fn flags_reach_every_mob() {
        let mut nbt = spawner();
        assert_eq!(apply(&mut nbt, modifier("minecraft:chorus_fruit"), false), Some(1.0));
        assert!(flag(&nbt, "dazed"));
        for e in entities_mut(&mut nbt) {
            assert_eq!(e.get("NoAI"), Some(&Tag::Byte(1)));
        }
        assert_eq!(apply(&mut nbt, modifier("minecraft:chorus_fruit"), false), None, "already set");
        apply(&mut nbt, modifier("minecraft:chorus_fruit"), true);
        assert!(!flag(&nbt, "dazed"));
        assert!(matches("#wool", "minecraft:red_wool"));
        apply(&mut nbt, modifier("minecraft:soul_lantern"), false);
        assert!(nbt.get("SpawnData").unwrap().get("custom_spawn_rules").is_some());
    }

    #[test]
    fn unwatched_keeps_its_reach() {
        let mut nbt = spawner();
        let star = modifier("minecraft:nether_star");
        apply(&mut nbt, star, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(i16::MAX)));
        assert_eq!(number(&nbt, "RequiredPlayerRange"), 16.0, "its reach, kept");
        apply(&mut nbt, modifier("minecraft:prismarine_crystals"), false);
        assert_eq!(number(&nbt, "RequiredPlayerRange"), 20.0);
        apply(&mut nbt, star, true);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(20)));
    }

    #[test]
    fn counts_and_health() {
        let mut nbt = spawner();
        let echo = modifier("minecraft:echo_shard");
        for _ in 0..5 {
            apply(&mut nbt, echo, false);
        }
        assert_eq!(counted(&nbt, "echo"), Some(3.0));
        let drip = modifier("minecraft:pointed_dripstone");
        apply(&mut nbt, drip, false);
        assert_eq!(counted(&nbt, "vigour"), Some(0.95));
        apply(&mut nbt, drip, true);
        assert_eq!(counted(&nbt, "vigour"), None, "back to full");
    }
}
