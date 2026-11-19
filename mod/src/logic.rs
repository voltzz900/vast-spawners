//! What a spawner's data holds and how the modifiers change it (no game here: the hooks are in `lib.rs`).
//!
//! Built from the MIT code and data of Apothic Spawners (by Shadows_of_Fire). Everything a spawner carries lives in
//! its own data: the game's six numbers, light rules in its spawn data, and the rest as tags on the mob it makes
//! (`vastspawners.*`), which the mob keeps. The tags are the ones 0.1 and 0.2 wrote, so their spawners still read.

use mod_api::Tag;

/// What a modifier changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    /// One of the spawner's own numbers (its NBT key).
    Number(&'static str),
    /// A flag kept as a tag on the spawned mob (unwatched and wired also set the spawner's range).
    Flag(&'static str),
    /// The echo count, 0 to 3: the mob's loot drops that many more times.
    Echo,
    /// The mob's health as it spawns, 20% to 100%.
    Health,
}

/// A modifier: the item held in the main hand, the change, and its bound; quartz in the other hand turns it back.
#[derive(Debug, PartialEq)]
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

/// The prefix of every tag the mod puts on a spawner's mobs.
pub const TAG: &str = "vastspawners.";

/// The widest player range the game stores (an unwatched spawner's).
pub const WIDEST: i16 = i16::MAX;

const WOOLS: [&str; 16] = [
    "white", "light_gray", "gray", "black", "brown", "red", "orange", "yellow", "lime", "green", "cyan", "light_blue",
    "blue", "purple", "magenta", "pink",
];

/// The items a modifier is (every wool for `#wool`).
pub fn modifier_items(m: &Modifier) -> Vec<String> {
    if m.item == "#wool" { WOOLS.iter().map(|c| format!("minecraft:{c}_wool")).collect() } else { vec![m.item.to_owned()] }
}

// ---- the spawner's data ----------------------------------------------------------------------------------------

/// The mob compounds a spawner's data holds: its next spawn's and each potential's.
pub fn entities_mut(nbt: &mut Tag) -> Vec<&mut Tag> {
    spawn_datas_mut(nbt).into_iter().filter_map(|d| d.get_mut("entity")).collect()
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

/// The `Tags` of an entity compound.
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

fn set_tags(entity: &mut Tag, tags: &[String]) {
    let list = (!tags.is_empty()).then(|| Tag::List(tags.iter().map(|t| Tag::String(t.as_str().into())).collect()));
    put(entity, "Tags", list);
}

/// Sets `key` of a compound, or removes it with `None`.
pub fn put(compound: &mut Tag, key: &str, value: Option<Tag>) {
    if let Tag::Compound(map) = compound {
        map.retain(|k, _| !k.0.iter().copied().eq(key.encode_utf16()));
        if let Some(v) = value {
            map.insert(key.into(), v);
        }
    }
}

/// A flag's value on the spawner (its first mob's tags).
pub fn flag(nbt: &Tag, name: &str) -> bool {
    first_entity(nbt).is_some_and(|e| tags(e).iter().any(|t| t == &format!("{TAG}{name}")))
}

/// A counted value in tags (`vastspawners.<name>=<n>`).
pub fn counted_in(tags: &[String], name: &str) -> Option<f64> {
    let prefix = format!("{TAG}{name}=");
    tags.iter().find_map(|t| t.strip_prefix(&prefix).and_then(|v| v.parse().ok()))
}

/// A counted value on the spawner (its first mob's tags).
pub fn counted(nbt: &Tag, name: &str) -> Option<f64> {
    first_entity(nbt).and_then(|e| counted_in(&tags(e), name))
}

pub fn first_entity(nbt: &Tag) -> Option<&Tag> {
    nbt.get("SpawnData").and_then(|d| d.get("entity")).or_else(|| match nbt.get("SpawnPotentials") {
        Some(Tag::List(list)) => list.first().and_then(|p| p.get("data")).and_then(|d| d.get("entity")),
        _ => None,
    })
}

/// One of the spawner's numbers; its player reach is the kept one while the stored range is the flags' own.
pub fn number(nbt: &Tag, key: &str) -> f64 {
    if key == "RequiredPlayerRange"
        && let Some(reach) = counted(nbt, "reach")
    {
        return reach;
    }
    match nbt.get(key) {
        Some(Tag::Short(v)) => f64::from(*v),
        Some(Tag::Int(v)) => f64::from(*v),
        _ => default_number(key),
    }
}

/// The game's value for a number a spawner's data doesn't hold.
pub fn default_number(key: &str) -> f64 {
    match key {
        "MinSpawnDelay" => 200.0,
        "MaxSpawnDelay" => 800.0,
        "SpawnCount" => 4.0,
        "MaxNearbyEntities" => 6.0,
        "RequiredPlayerRange" => 16.0,
        _ => 4.0,
    }
}

/// The player range the spawner stores for `reach`: none when wired and without a redstone signal, the widest
/// when unwatched, else the reach itself.
pub fn stored_range(nbt: &Tag, reach: f64, powered: bool) -> i16 {
    if flag(nbt, "wired") && !powered {
        0
    } else if flag(nbt, "unwatched") {
        WIDEST
    } else {
        reach as i16
    }
}

/// Puts the spawner's stored range where its flags and the redstone signal want it; while either flag is on, the
/// reach is kept in a tag. True when the range changed.
pub fn settle_range(nbt: &mut Tag, powered: bool) -> bool {
    let reach = number(nbt, "RequiredPlayerRange");
    let held = flag(nbt, "wired") || flag(nbt, "unwatched");
    set_counted(nbt, "reach", held.then_some(reach));
    if !held && nbt.get("RequiredPlayerRange").is_none() {
        return false;
    }
    let range = stored_range(nbt, reach, powered);
    if nbt.get("RequiredPlayerRange") == Some(&Tag::Short(range)) {
        return false;
    }
    put(nbt, "RequiredPlayerRange", Some(Tag::Short(range)));
    true
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

/// Sets a counted value on every mob the spawner makes, or removes it with `None`.
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

/// Applies `m` (or its way back) to a spawner's data: the new value, or None when it is at its bound. `powered`:
/// whether the spawner has a redstone signal (for a wired one's range).
pub fn apply(nbt: &mut Tag, m: &Modifier, back: bool, powered: bool) -> Option<f64> {
    let (step, limit) = if back { (m.back_step, m.back_limit) } else { (m.step, m.limit) };
    let bounded = |now: f64| {
        let next = now + step;
        let next = if step > 0.0 { next.min(limit) } else { next.max(limit) };
        ((next - now).abs() > 1e-9).then_some(next)
    };
    let value = match m.stat {
        Stat::Number("RequiredPlayerRange") if counted(nbt, "reach").is_some() => {
            let next = bounded(number(nbt, "RequiredPlayerRange"))?;
            set_counted(nbt, "reach", Some(next));
            next
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
            next
        }
        Stat::Flag(name) => {
            let on = !back;
            if flag(nbt, name) == on {
                return None;
            }
            set_flag(nbt, name, on);
            f64::from(u8::from(on))
        }
        Stat::Echo => {
            let next = bounded(counted(nbt, "echo").unwrap_or(0.0))?;
            set_counted(nbt, "echo", (next > 0.0).then_some(next));
            next
        }
        Stat::Health => {
            let next = bounded(counted(nbt, "vigour").unwrap_or(1.0))?;
            set_counted(nbt, "vigour", (next < 1.0).then_some(next));
            next
        }
    };
    settle_range(nbt, powered);
    Some(value)
}

/// A modifier's value as shown: `on`/`off` (a lang key's tail), `95%`, `9.5s`, `16`.
pub enum Shown {
    Word(&'static str),
    Plain(String),
}

pub fn shown(m: &Modifier, value: f64) -> Shown {
    match m.stat {
        Stat::Flag(_) => Shown::Word(if value > 0.5 { "on" } else { "off" }),
        Stat::Health => Shown::Plain(format!("{}%", (value * 100.0).round())),
        Stat::Number("MinSpawnDelay" | "MaxSpawnDelay") => Shown::Plain(format!("{:.1}s", value / 20.0)),
        _ => Shown::Plain(format!("{}", value as i64)),
    }
}

/// What a modifier does in one use: `-0.5s`, `+2`, `on`.
pub fn change(m: &Modifier, back: bool) -> Shown {
    let step = if back { m.back_step } else { m.step };
    match m.stat {
        Stat::Flag(_) => Shown::Word(if back { "off" } else { "on" }),
        Stat::Health => Shown::Plain(format!("{:+}%", (step * 100.0).round())),
        Stat::Number("MinSpawnDelay" | "MaxSpawnDelay") => Shown::Plain(format!("{:+.1}s", step / 20.0)),
        _ => Shown::Plain(format!("{:+}", step as i64)),
    }
}

/// The changes a spawner's data carries, as (modifier, value): the numbers that differ from the game's and every
/// flag, echo and vigour it has.
pub fn changes(nbt: &Tag) -> Vec<(&'static Modifier, f64)> {
    let mut out = Vec::new();
    for m in MODIFIERS {
        let value = match m.stat {
            Stat::Number(key) => {
                let v = number(nbt, key);
                if (v - default_number(key)).abs() < 1e-9 {
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
        out.push((m, value));
    }
    out
}

// ---- what a spawner item carries -------------------------------------------------------------------------------

/// The key, in a spawner item's `minecraft:custom_data`, of this mod's own record of the spawner it was mined from.
pub const CARRIED: &str = "vastspawners:spawner";

/// Mobs a spawner item never brings back without an operator, whatever else the game says of them.
pub const BOSSES: &[&str] = &["minecraft:ender_dragon", "minecraft:wither"];

/// A spawner as an item carries it: the mob it makes and the modifiers' values that differ from the game's own.
/// Nothing else of the spawner's data (its mob's other NBT, its potentials, its raw numbers) goes with it.
#[derive(Clone, Debug, PartialEq)]
pub struct Carried {
    pub entity: String,
    pub values: Vec<(&'static Modifier, f64)>,
}

/// The lowest and highest value a modifier can give (0 and 1 for a flag).
pub fn bounds(m: &Modifier) -> (f64, f64) {
    (m.limit.min(m.back_limit), m.limit.max(m.back_limit))
}

/// The value of a number tag, any width.
fn numeric(tag: &Tag) -> Option<f64> {
    match tag {
        Tag::Byte(v) => Some(f64::from(*v)),
        Tag::Short(v) => Some(f64::from(*v)),
        Tag::Int(v) => Some(f64::from(*v)),
        Tag::Long(v) => Some(*v as f64),
        Tag::Float(bits) => Some(f64::from(f32::from_bits(*bits))),
        Tag::Double(bits) => Some(f64::from_bits(*bits)),
        _ => None,
    }
}

/// Whether `id` is shaped as a registry name (`namespace:path`, lowercase).
pub fn valid_id(id: &str) -> bool {
    let Some((namespace, path)) = id.split_once(':') else { return false };
    let ok = |s: &str, slash: bool| {
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-.".contains(&b) || (slash && b == b'/'))
    };
    ok(namespace, false) && ok(path, true)
}

/// The record of a spawner's data (as the world holds it, or as a 0.1 to 0.3.1 item carried it): its first mob and
/// the changes it shows. None without a mob. Not checked: see [`sanitize`].
pub fn from_spawner(nbt: &Tag) -> Option<Carried> {
    let Some(Tag::String(entity)) = first_entity(nbt)?.get("id") else { return None };
    Some(Carried { entity: entity.to_string_lossy(), values: changes(nbt) })
}

/// The record as the item keeps it: `{entity: "<id>", <modifier name>: <value>, ...}`.
pub fn encode(carried: &Carried) -> Tag {
    let mut tag = Tag::compound([("entity", Tag::String(carried.entity.as_str().into()))]);
    for (m, value) in &carried.values {
        put(&mut tag, m.name, Some(Tag::Double(value.to_bits())));
    }
    tag
}

/// The record an item keeps (see [`encode`]); keys it doesn't know are left out. Not checked: see [`sanitize`].
pub fn decode(tag: &Tag) -> Option<Carried> {
    let Some(Tag::String(entity)) = tag.get("entity") else { return None };
    let values = MODIFIERS.iter().filter_map(|m| tag.get(m.name).and_then(numeric).map(|v| (m, v))).collect();
    Some(Carried { entity: entity.to_string_lossy(), values })
}

/// A record made safe to place: None unless its mob is a well-formed id, not a boss and `allowed` by the game (a
/// registered mob); each value within its modifier's bounds (whole where the game's are whole, flags on or off,
/// a value that is no number left out), each modifier once, the shortest wait at most the longest, and values that
/// are the game's own left out.
pub fn sanitize(carried: Carried, allowed: impl Fn(&str) -> bool) -> Option<Carried> {
    let entity = carried.entity;
    if !valid_id(&entity) || BOSSES.contains(&entity.as_str()) || !allowed(&entity) {
        return None;
    }
    let mut values: Vec<(&'static Modifier, f64)> = Vec::new();
    for (m, value) in carried.values {
        if !value.is_finite() || values.iter().any(|(seen, _)| std::ptr::eq(*seen, m)) {
            continue;
        }
        let (low, high) = bounds(m);
        let value = match m.stat {
            Stat::Flag(_) => f64::from(u8::from(value >= 0.5)),
            Stat::Number(_) | Stat::Echo => value.round().clamp(low, high),
            Stat::Health => ((value * 100.0).round() / 100.0).clamp(low, high),
        };
        values.push((m, value));
    }
    fn value_of(values: &[(&'static Modifier, f64)], key: &'static str) -> f64 {
        values.iter().find(|(m, _)| m.stat == Stat::Number(key)).map_or(default_number(key), |(_, v)| *v)
    }
    let longest = value_of(&values, "MaxSpawnDelay");
    if value_of(&values, "MinSpawnDelay") > longest {
        values.retain(|(m, _)| m.stat != Stat::Number("MinSpawnDelay"));
        let shortest = MODIFIERS.iter().find(|m| m.stat == Stat::Number("MinSpawnDelay"))?;
        values.push((shortest, longest));
    }
    values.retain(|(m, v)| match m.stat {
        Stat::Number(key) => (v - default_number(key)).abs() > 1e-9,
        Stat::Flag(_) | Stat::Echo => *v > 0.0,
        Stat::Health => *v < 1.0,
    });
    values.sort_by_key(|(m, _)| MODIFIERS.iter().position(|x| std::ptr::eq(x, *m)));
    Some(Carried { entity, values })
}

/// A spawner's data made from a (sanitized) record alone: its mob by id, the game's six numbers, then each flag
/// and count as a modifier would set them. The caller settles its range for the redstone signal.
pub fn build(carried: &Carried) -> Tag {
    let entity = Tag::compound([("id", Tag::String(carried.entity.as_str().into()))]);
    let mut nbt = Tag::compound([("SpawnData", Tag::compound([("entity", entity)]))]);
    let value = |stat: Stat| carried.values.iter().find(|(m, _)| m.stat == stat).map(|(_, v)| *v);
    for key in ["MinSpawnDelay", "MaxSpawnDelay", "SpawnCount", "MaxNearbyEntities", "RequiredPlayerRange", "SpawnRange"] {
        let v = value(Stat::Number(key)).unwrap_or_else(|| default_number(key));
        put(&mut nbt, key, Some(Tag::Short(v as i16)));
    }
    for m in MODIFIERS {
        if let Stat::Flag(name) = m.stat
            && value(m.stat).is_some_and(|v| v > 0.5)
        {
            set_flag(&mut nbt, name, true);
        }
    }
    if let Some(echo) = value(Stat::Echo).filter(|v| *v > 0.0) {
        set_counted(&mut nbt, "echo", Some(echo));
    }
    if let Some(vigour) = value(Stat::Health).filter(|v| *v < 1.0) {
        set_counted(&mut nbt, "vigour", Some(vigour));
    }
    nbt
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawner() -> Tag {
        let zombie = || Tag::compound([("id", Tag::String("minecraft:zombie".into()))]);
        Tag::compound([
            ("SpawnData", Tag::compound([("entity", zombie())])),
            ("SpawnPotentials", Tag::List(vec![Tag::compound([("data", Tag::compound([("entity", zombie())])), ("weight", Tag::Int(1))])])),
            ("MinSpawnDelay", Tag::Short(200)),
            ("MaxSpawnDelay", Tag::Short(800)),
        ])
    }

    fn modifier(item: &str) -> &'static Modifier {
        MODIFIERS.iter().find(|m| modifier_items(m).iter().any(|i| i == item)).unwrap()
    }

    #[test]
    fn numbers_move_within_bounds() {
        let mut nbt = spawner();
        assert_eq!(apply(&mut nbt, modifier("minecraft:sugar"), false, false), Some(190.0));
        assert_eq!(apply(&mut nbt, modifier("minecraft:sugar"), true, false), Some(200.0));
        let piston = modifier("minecraft:piston");
        for _ in 0..20 {
            apply(&mut nbt, piston, false, false);
        }
        assert_eq!(number(&nbt, "SpawnRange"), 32.0);
        assert_eq!(apply(&mut nbt, piston, false, false), None, "at its bound");
    }

    #[test]
    fn flags_reach_every_mob() {
        let mut nbt = spawner();
        let chorus = modifier("minecraft:chorus_fruit");
        assert_eq!(apply(&mut nbt, chorus, false, false), Some(1.0));
        assert!(flag(&nbt, "dazed"));
        for e in entities_mut(&mut nbt) {
            assert_eq!(e.get("NoAI"), Some(&Tag::Byte(1)));
        }
        assert_eq!(apply(&mut nbt, chorus, false, false), None, "already set");
        apply(&mut nbt, chorus, true, false);
        assert!(!flag(&nbt, "dazed"));
        for e in entities_mut(&mut nbt) {
            assert_eq!(e.get("NoAI"), None);
            assert_eq!(e.get("Tags"), None, "no empty list left");
        }
        assert!(std::ptr::eq(modifier("minecraft:red_wool"), modifier("minecraft:white_wool")));
        apply(&mut nbt, modifier("minecraft:soul_lantern"), false, false);
        assert!(nbt.get("SpawnData").unwrap().get("custom_spawn_rules").is_some());
        apply(&mut nbt, modifier("minecraft:soul_lantern"), true, false);
        assert!(nbt.get("SpawnData").unwrap().get("custom_spawn_rules").is_none());
    }

    #[test]
    fn unwatched_keeps_its_reach() {
        let mut nbt = spawner();
        let star = modifier("minecraft:nether_star");
        apply(&mut nbt, star, false, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(WIDEST)));
        assert_eq!(number(&nbt, "RequiredPlayerRange"), 16.0, "its reach, kept");
        apply(&mut nbt, modifier("minecraft:prismarine_crystals"), false, false);
        assert_eq!(number(&nbt, "RequiredPlayerRange"), 20.0);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(WIDEST)));
        apply(&mut nbt, star, true, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(20)));
        assert_eq!(counted(&nbt, "reach"), None);
    }

    #[test]
    fn wired_follows_the_signal() {
        let mut nbt = spawner();
        let comparator = modifier("minecraft:comparator");
        apply(&mut nbt, comparator, false, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(0)), "unpowered: no player is ever near");
        assert_eq!(number(&nbt, "RequiredPlayerRange"), 16.0);
        assert!(settle_range(&mut nbt, true));
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(16)));
        assert!(!settle_range(&mut nbt, true), "nothing to change");
        apply(&mut nbt, modifier("minecraft:nether_star"), false, true);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(WIDEST)));
        settle_range(&mut nbt, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(0)), "wired wins without a signal");
        apply(&mut nbt, modifier("minecraft:nether_star"), true, false);
        apply(&mut nbt, comparator, true, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(16)));
        assert_eq!(counted(&nbt, "reach"), None);
    }

    #[test]
    fn counts_and_health() {
        let mut nbt = spawner();
        let echo = modifier("minecraft:echo_shard");
        for _ in 0..5 {
            apply(&mut nbt, echo, false, false);
        }
        assert_eq!(counted(&nbt, "echo"), Some(3.0));
        let drip = modifier("minecraft:pointed_dripstone");
        apply(&mut nbt, drip, false, false);
        assert_eq!(counted(&nbt, "vigour"), Some(0.95));
        apply(&mut nbt, drip, true, false);
        assert_eq!(counted(&nbt, "vigour"), None, "back to full");
        assert_eq!(changes(&nbt).len(), 1, "only the echoes");
    }

    fn every_change() -> Tag {
        let mut nbt = spawner();
        for m in MODIFIERS {
            apply(&mut nbt, m, false, false);
        }
        nbt
    }

    fn sorted_tags(nbt: &Tag) -> Vec<String> {
        let mut t = tags(first_entity(nbt).unwrap());
        t.sort();
        t
    }

    fn named(name: &str) -> &'static Modifier {
        MODIFIERS.iter().find(|m| m.name == name).unwrap()
    }

    #[test]
    fn a_mined_spawner_comes_back_the_same() {
        let original = every_change();
        let carried = from_spawner(&original).unwrap();
        let read = sanitize(decode(&encode(&carried)).unwrap(), |_| true).unwrap();
        assert_eq!(read, carried, "a legal spawner's record survives the item unchanged");
        let mut placed = build(&read);
        settle_range(&mut placed, false);
        assert_eq!(changes(&placed), changes(&original));
        assert_eq!(sorted_tags(&placed), sorted_tags(&original));
        assert_eq!(placed.get("RequiredPlayerRange"), original.get("RequiredPlayerRange"));
        let (p, o) = (first_entity(&placed).unwrap(), first_entity(&original).unwrap());
        for key in ["id", "NoAI", "Silent", "IsBaby", "Age", "Fire"] {
            assert_eq!(p.get(key), o.get(key), "{key}");
        }
        assert!(placed.get("SpawnData").unwrap().get("custom_spawn_rules").is_some());
        assert!(placed.get("SpawnPotentials").is_none());
    }

    #[test]
    fn an_old_unwatched_item_keeps_its_reach() {
        let mut nbt = spawner();
        apply(&mut nbt, modifier("minecraft:prismarine_crystals"), false, false);
        apply(&mut nbt, modifier("minecraft:nether_star"), false, false);
        assert_eq!(nbt.get("RequiredPlayerRange"), Some(&Tag::Short(WIDEST)));
        let carried = sanitize(from_spawner(&nbt).unwrap(), |_| true).unwrap();
        assert!(carried.values.contains(&(named("reach"), 20.0)), "the kept reach, not the stored widest");
        let mut placed = build(&carried);
        settle_range(&mut placed, false);
        assert_eq!(placed.get("RequiredPlayerRange"), Some(&Tag::Short(WIDEST)));
        assert_eq!(number(&placed, "RequiredPlayerRange"), 20.0);
    }

    #[test]
    fn out_of_range_values_are_clamped() {
        let carried = Carried {
            entity: "minecraft:zombie".into(),
            values: vec![
                (named("batch"), 500.0),
                (named("crowd"), -7.0),
                (named("spread"), 3.4),
                (named("echoes"), 99.0),
                (named("vigour"), 0.01),
                (named("dazed"), 42.0),
                (named("young"), 0.0),
                (named("shortest_wait"), 1600.0),
                (named("longest_wait"), 100.0),
                (named("batch"), 2.0),
            ],
        };
        let read = sanitize(carried, |_| true).unwrap();
        let value = |name: &str| read.values.iter().find(|(m, _)| m.name == name).map(|(_, v)| *v);
        assert_eq!(value("batch"), Some(16.0), "the first of a repeated modifier, clamped");
        assert_eq!(value("crowd"), Some(1.0));
        assert_eq!(value("spread"), Some(3.0), "whole");
        assert_eq!(value("echoes"), Some(3.0));
        assert_eq!(value("vigour"), Some(0.2));
        assert_eq!(value("dazed"), Some(1.0));
        assert_eq!(value("young"), None, "off is the game's own");
        assert_eq!(value("longest_wait"), Some(100.0));
        assert_eq!(value("shortest_wait"), Some(100.0), "never above the longest");
        assert_eq!(read.values.iter().filter(|(m, _)| m.name == "batch").count(), 1);
        let placed = build(&read);
        assert_eq!(placed.get("SpawnCount"), Some(&Tag::Short(16)));
        assert_eq!(placed.get("MinSpawnDelay"), Some(&Tag::Short(100)));
    }

    #[test]
    fn values_that_are_no_numbers_are_left_out() {
        let carried = Carried {
            entity: "minecraft:zombie".into(),
            values: vec![(named("batch"), f64::NAN), (named("crowd"), f64::INFINITY), (named("echoes"), f64::NEG_INFINITY)],
        };
        assert!(sanitize(carried, |_| true).unwrap().values.is_empty());
        let mut tag = Tag::compound([("entity", Tag::String("minecraft:zombie".into()))]);
        put(&mut tag, "batch", Some(Tag::Double(f64::NAN.to_bits())));
        put(&mut tag, "crowd", Some(Tag::String("9".into())));
        assert!(sanitize(decode(&tag).unwrap(), |_| true).unwrap().values.is_empty());
    }

    #[test]
    fn only_allowed_mobs_come_back() {
        let carried = |id: &str| Carried { entity: id.into(), values: Vec::new() };
        let registered = |id: &str| matches!(id, "minecraft:zombie" | "minecraft:wither" | "minecraft:ender_dragon");
        assert!(sanitize(carried("minecraft:zombie"), registered).is_some());
        assert!(sanitize(carried("minecraft:creeper"), registered).is_none(), "unknown to the game");
        assert!(sanitize(carried("minecraft:wither"), registered).is_none(), "a boss");
        assert!(sanitize(carried("minecraft:ender_dragon"), |_| true).is_none(), "a boss, whatever the game says");
        for bad in ["", "zombie", "minecraft:", ":zombie", "Minecraft:Zombie", "minecraft:zom bie", "a:b:c"] {
            assert!(sanitize(carried(bad), |_| true).is_none(), "{bad:?}");
        }
        assert!(valid_id("somemod:mobs/big_slime"));
    }

    #[test]
    fn extra_fields_are_ignored() {
        let mut tag = encode(&Carried { entity: "minecraft:zombie".into(), values: vec![(named("batch"), 6.0)] });
        let nasty = Tag::compound([("entity", Tag::compound([("id", Tag::String("minecraft:wither".into()))]))]);
        put(&mut tag, "SpawnData", Some(nasty.clone()));
        put(&mut tag, "SpawnPotentials", Some(Tag::List(vec![Tag::compound([("data", nasty)])])));
        put(&mut tag, "SpawnCount", Some(Tag::Short(200)));
        put(&mut tag, "Delay", Some(Tag::Short(0)));
        let read = sanitize(decode(&tag).unwrap(), |_| true).unwrap();
        assert_eq!(read, Carried { entity: "minecraft:zombie".into(), values: vec![(named("batch"), 6.0)] });
        let placed = build(&read);
        assert_eq!(placed.get("SpawnCount"), Some(&Tag::Short(6)));
        assert!(placed.get("SpawnPotentials").is_none() && placed.get("Delay").is_none());
        assert_eq!(first_entity(&placed).and_then(|e| e.get("id")), Some(&Tag::String("minecraft:zombie".into())));
        let mut old = spawner();
        put(&mut old, "SpawnCount", Some(Tag::Short(300)));
        if let Some(entity) = old.get_mut("SpawnData").and_then(|d| d.get_mut("entity")) {
            put(entity, "Passengers", Some(Tag::List(vec![Tag::compound([("id", Tag::String("minecraft:wither".into()))])])));
            put(entity, "Tags", Some(Tag::List(vec![Tag::String("vastspawners.echo=50".into())])));
        }
        let read = sanitize(from_spawner(&old).unwrap(), |_| true).unwrap();
        let placed = build(&read);
        assert_eq!(placed.get("SpawnCount"), Some(&Tag::Short(16)), "an old item's count, clamped");
        assert_eq!(counted(&placed, "echo"), Some(3.0));
        assert!(first_entity(&placed).unwrap().get("Passengers").is_none(), "nothing of the mob but its id");
    }
}
