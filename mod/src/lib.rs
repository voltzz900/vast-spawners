//! Vast Spawners: change spawners with items, mine them with Silk Touch, and shape the mobs they make.
//!
//! A modifier used on a spawner changes its data (`logic.rs`); quartz in the off hand turns the change back. A
//! spawner mined with Silk Touch drops itself with the mod's own record of it (its mob and the changes), and placed
//! again the spawner is made anew from that record, checked; only an operator's item puts raw spawner data. A wired
//! spawner keeps no player range without a redstone signal, an unwatched one the widest; its mobs carry the rest.
//! Built from the MIT code and data of Apothic Spawners (Shadows_of_Fire); the names and words are this mod's own.

mod logic;

use logic::{Carried, MODIFIERS, Modifier, Shown};
use mod_api::{
    self, BlockDropsEvent, BlockPart, Compound, EntityJoinLevelEvent, Flow, Hand, Host, HostSide, Hooks, InteractionResult,
    Identifier, ItemDrop, ItemKey, ItemPart, ItemStack, LevelDimension, LevelHost, LevelRef, LevelTicksHost, LootDropsEvent,
    LevelTickPreEvent, NeighborChangedEvent, OnBlockPlacedByEvent, Registrar, RegistrationContext, ServerHost, Tag, Text, TooltipEvent, UseItemOnBlockEvent, UsePhase,
};
use std::sync::{Mutex, OnceLock};

const SPAWNER: &str = "minecraft:spawner";
/// The item component vanilla keeps a spawner's raw data in (as 0.1 to 0.3.1 wrote it): the game puts it only for
/// operators, and this mod reads it raw only for them.
const BLOCK_ENTITY_DATA: &str = "minecraft:block_entity_data";
/// The item component this mod keeps its own record in (under [`logic::CARRIED`]).
const CUSTOM_DATA: &str = "minecraft:custom_data";
/// The permission level an operator has (vanilla's game master level).
const OPERATOR: u8 = 2;

/// The items the mod reads, issued at registration.
struct Keys {
    spawner: ItemKey,
    quartz: ItemKey,
    /// Each modifier item and its modifier.
    modifiers: Vec<(ItemKey, &'static Modifier)>,
}

static KEYS: OnceLock<Keys> = OnceLock::new();

/// Spawners a neighbour changed beside, by level and position: a wired one's range follows the signal before its
/// level's next tick (the neighbour notice can't write the world, and only the level in place reads signals).
static NEIGHBOURED: Mutex<Vec<(&'static str, [i32; 3])>> = Mutex::new(Vec::new());

fn modifier_of(item: ItemKey) -> Option<&'static Modifier> {
    KEYS.get()?.modifiers.iter().find(|(k, _)| *k == item).map(|(_, m)| *m)
}

// ---- words -------------------------------------------------------------------------------------------------------

fn tr(key: &str) -> Text {
    Text::translate(format!("vastspawners.{key}"), Vec::new())
}

fn coloured(mut text: Text, color: &str) -> Text {
    text.style.color = Some(color.into());
    text
}

fn shown_text(shown: Shown) -> Text {
    match shown {
        Shown::Word(word) => tr(word),
        Shown::Plain(text) => Text::literal(text),
    }
}

/// `<stat>: <value>`, the stat in `stat_color` and the value in `value_color`.
fn stat_line(m: &Modifier, value: Shown, stat_color: &str, value_color: &str) -> Text {
    coloured(tr(&format!("stat.{}", m.name)), stat_color)
        .append(Text::literal(": "))
        .append(coloured(shown_text(value), value_color))
}

// ---- reading the world ---------------------------------------------------------------------------------------------

fn level_of(host: &dyn Host, dimension: LevelDimension) -> Option<LevelRef> {
    host.level(dimension.key().id())
}

fn is_spawner(host: &dyn Host, level: LevelRef, pos: [i32; 3]) -> bool {
    let spawner = host.block_state_id(SPAWNER);
    spawner.is_some() && level.block_state(host, pos) == spawner
}

/// The level of an enchantment on a stack (`minecraft:enchantments`, either shape).
fn enchantment_level(stack: &ItemStack, id: &str) -> i32 {
    let Some(Tag::Compound(levels)) = stack.components().and_then(|c| c.as_nbt().iter().find(|(k, _)| k.to_string_lossy() == "minecraft:enchantments").map(|(_, v)| v)) else {
        return 0;
    };
    let levels = Tag::Compound(levels.clone());
    let levels = levels.get("levels").cloned().unwrap_or(levels);
    match levels.get(id) {
        Some(Tag::Int(level)) => *level,
        Some(Tag::Short(level)) => i32::from(*level),
        Some(Tag::Byte(level)) => i32::from(*level),
        _ => 0,
    }
}

/// An item component of a stack.
fn component(stack: &ItemStack, key: &str) -> Option<Tag> {
    let patch = stack.components()?.as_nbt();
    patch.iter().find(|(k, _)| k.to_string_lossy() == key).map(|(_, v)| v.clone())
}

/// The raw spawner data an item carries (vanilla's component).
fn stack_spawner_data(stack: &ItemStack) -> Option<Tag> {
    component(stack, BLOCK_ENTITY_DATA)
}

/// The spawner an item carries, as the mod's record: its own, else read from an older item's raw data. Not checked.
fn stack_carried(stack: &ItemStack) -> Option<Carried> {
    let own = component(stack, CUSTOM_DATA).and_then(|c| c.get(logic::CARRIED).and_then(logic::decode));
    own.or_else(|| stack_spawner_data(stack).and_then(|nbt| logic::from_spawner(&nbt)))
}

/// Whether a spawner item may bring back mob `id`: a registered entity type with a spawn egg (a mob, as a creative
/// player may already set a spawner to with an egg), not tagged a boss.
fn mob_allowed(host: &dyn Host, id: &str) -> bool {
    let registered = host.entity_type_keys().into_iter().any(|k| k.id(host) == id);
    let egg = Identifier::parse(&format!("{id}_spawn_egg")).ok().and_then(|egg| host.item_key(&egg)).is_some();
    registered && egg && !host.entity_type_in_tag(id, "c:bosses")
}

/// A spawner's saved data without what the place gives it.
fn portable(mut nbt: Tag) -> Tag {
    for key in ["id", "x", "y", "z", "Delay"] {
        logic::put(&mut nbt, key, None);
    }
    nbt
}

/// Writes a spawner's whole data back (keys the change took out go too). True when it took.
fn write(host: &mut dyn ServerHost, level: LevelRef, pos: [i32; 3], nbt: Tag) -> bool {
    let (Some(entity), Tag::Compound(data)) = (host.block_entity(level, pos), nbt) else { return false };
    entity.replace_data(host, data).unwrap_or(false)
}

// ---- modifiers ---------------------------------------------------------------------------------------------------

/// A modifier used on the spawner at `pos` by `player`: the change, the item spent and the chat told.
fn use_modifier(host: &mut dyn ServerHost, level: LevelRef, pos: [i32; 3], player: u128, m: &Modifier) {
    let (Some(keys), Some(player)) = (KEYS.get(), host.player(player)) else { return };
    let Some(living) = host.living(player.entity()) else { return };
    let back = living.hand_item(&*host, Hand::Off).is_some_and(|s| s.item() == keys.quartz);
    let Some(data) = host.block_entity(level, pos).and_then(|e| e.data(&*host)) else { return };
    let mut nbt = Tag::Compound(data);
    let powered = level.has_neighbor_signal(&*host, pos);
    let Some(value) = logic::apply(&mut nbt, m, back, powered) else {
        let line = coloured(tr(&format!("stat.{}", m.name)), "gray").append(Text::literal(": ")).append(tr("at_limit"));
        let _ = player.tell(host, line);
        return;
    };
    if !write(host, level, pos, nbt) {
        return;
    }
    if !player.is_creative(&*host).unwrap_or(false) {
        let held = living.hand_item(&*host, Hand::Main).and_then(|s| s.shrink(1));
        let _ = living.set_hand_item(host, Hand::Main, held);
    }
    let _ = player.tell(host, stat_line(m, logic::shown(m, value), "gold", "yellow"));
}

/// A modifier item: what one use does to a spawner, in its tooltip.
struct ModifierPart {
    modifier: &'static Modifier,
}

#[mod_api::horizon_part]
impl ItemPart for ModifierPart {
    fn tooltip(&self, _host: &dyn mod_api::ClientHost, event: &mut TooltipEvent<'_>) {
        let m = self.modifier;
        let mut line = coloured(tr("modifier"), "gray")
            .append(Text::literal(": "))
            .append(coloured(tr(&format!("stat.{}", m.name)), "blue"))
            .append(coloured(Text::literal(" "), "blue"))
            .append(coloured(shown_text(logic::change(m, false)), "blue"));
        if !matches!(m.stat, logic::Stat::Flag(_)) {
            line = line.append(
                coloured(Text::literal(" ("), "dark_gray")
                    .append(tr("limit"))
                    .append(Text::literal(" "))
                    .append(shown_text(logic::shown(m, m.limit)))
                    .append(Text::literal(")")),
            );
        }
        event.lines.push((line, u32::MAX));
        let back = coloured(tr("reverse"), "dark_gray").append(Text::literal(": ")).append(shown_text(logic::change(m, true)));
        event.lines.push((back, u32::MAX));
    }
}

// ---- the spawner -------------------------------------------------------------------------------------------------

/// The spawner item: placed with data, the spawner gets it back; its tooltip tells the mob and the changes.
struct SpawnerItemPart;

#[mod_api::horizon_part]
impl ItemPart for SpawnerItemPart {
    fn on_block_placed_by(&self, host: &mut dyn ServerHost, event: &mut OnBlockPlacedByEvent<'_>) {
        let level = match event.placer {
            Some(player) => player.level(&*host),
            None => host.level(host.dimension()),
        };
        let Some(level) = level else { return };
        // Raw spawner data is an operator's to place, as vanilla has it; anyone else's item puts only the mod's
        // record, checked, made into a spawner anew. No placer or no known level counts as no operator.
        let operator = event.placer.and_then(|p| p.permission_level(&*host)).is_some_and(|l| l >= OPERATOR);
        let raw = stack_spawner_data(event.stack).filter(|_| operator);
        let mut nbt = match raw {
            Some(nbt) => portable(nbt),
            None => {
                let checked = stack_carried(event.stack).and_then(|c| logic::sanitize(c, |id| mob_allowed(&*host, id)));
                let Some(carried) = checked else { return };
                logic::build(&carried)
            }
        };
        logic::settle_range(&mut nbt, level.has_neighbor_signal(&*host, event.pos));
        write(host, level, event.pos, nbt);
    }

    fn tooltip(&self, _host: &dyn mod_api::ClientHost, event: &mut TooltipEvent<'_>) {
        // What placing it would put (the mob's registration is the server's to check).
        let Some(carried) = stack_carried(event.stack).and_then(|c| logic::sanitize(c, |_| true)) else { return };
        let name = Text::translate(format!("entity.{}", carried.entity.replace(':', ".")), Vec::new());
        let line = coloured(tr("mob"), "gray").append(Text::literal(": ")).append(coloured(name, "white"));
        event.lines.push((line, u32::MAX));
        for (m, value) in carried.values {
            event.lines.push((stat_line(m, logic::shown(m, value), "blue", "blue"), u32::MAX));
        }
    }
}

/// The spawner block: a neighbour's change may change a wired spawner's signal.
struct SpawnerBlockPart;

#[mod_api::horizon_part]
impl BlockPart for SpawnerBlockPart {
    fn neighbor_changed(&self, _level: &mut dyn LevelTicksHost, event: &mut NeighborChangedEvent) {
        let at = (event.dimension.key().id(), [event.pos.x, event.pos.y, event.pos.z]);
        if let Ok(mut pending) = NEIGHBOURED.lock()
            && !pending.contains(&at)
        {
            pending.push(at);
        }
    }
}

/// The wired spawners a neighbour changed beside in the level in place: their range as the signal now wants it.
/// Spawners of other levels wait for their own level's tick; those of unloaded levels go.
fn settle_neighboured(host: &mut dyn ServerHost) {
    let mut here = Vec::new();
    match NEIGHBOURED.lock() {
        Ok(mut pending) if !pending.is_empty() => pending.retain(|&(dimension, pos)| match host.level(dimension) {
            Some(level) if level.in_place(&*host) => {
                here.push((level, pos));
                false
            }
            Some(_) => true,
            None => false,
        }),
        _ => return,
    }
    for (level, pos) in here {
        let Some(data) = host.block_entity(level, pos).and_then(|e| e.data(&*host)) else { continue };
        let mut nbt = Tag::Compound(data);
        if logic::flag(&nbt, "wired") && logic::settle_range(&mut nbt, level.has_neighbor_signal(&*host, pos)) {
            write(host, level, pos, nbt);
        }
    }
}

#[derive(Default)]
struct VastSpawners;

#[mod_api::horizon_mod]
impl Hooks for VastSpawners {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let spawner = r.item_key(SPAWNER);
        let quartz = r.item_key("minecraft:quartz");
        let mut modifiers = Vec::new();
        for m in MODIFIERS {
            for item in logic::modifier_items(m) {
                let key = r.item_key(&item);
                modifiers.push((key, m));
                r.item_part(key, ModifierPart { modifier: m });
            }
        }
        r.item_part(spawner, SpawnerItemPart);
        let block = r.block_key(SPAWNER);
        r.block_part(block, SpawnerBlockPart);
        let _ = KEYS.set(Keys { spawner, quartz, modifiers });
    }

    /// A modifier used on a spawner, before the item's own use (a block item would be placed). The client takes the
    /// click too, so it doesn't predict the placement; only the server changes the spawner.
    fn use_item_on_block(&self, host: HostSide, event: &mut UseItemOnBlockEvent<'_>) -> Flow {
        let click = event.click;
        if event.phase != UsePhase::ItemBeforeBlock || click.hand != Hand::Main {
            return Flow::Continue;
        }
        let Some(m) = click.stack.and_then(|s| modifier_of(s.item())) else { return Flow::Continue };
        let answer = match host {
            HostSide::Client(host) => {
                let host: &dyn Host = &*host;
                match level_of(host, click.dimension) {
                    Some(level) if is_spawner(host, level, click.pos) => InteractionResult::Success,
                    _ => return Flow::Continue,
                }
            }
            HostSide::Server(host) => {
                let Some(level) = level_of(&*host, click.dimension).filter(|l| is_spawner(&*host, *l, click.pos)) else {
                    return Flow::Continue;
                };
                use_modifier(host, level, click.pos, click.player, m);
                InteractionResult::SuccessServer
            }
        };
        *event.cancellation_result = answer;
        Flow::Cancel
    }

    /// A spawner mined with Silk Touch drops itself, with the mod's record of it (none for a spawner without a mob).
    fn block_drops(&self, level: &dyn LevelHost, event: &mut BlockDropsEvent<'_>) -> Flow {
        if level.state_block(event.state) != Some(SPAWNER) || !event.tool.is_some_and(|t| enchantment_level(t, "minecraft:silk_touch") > 0) {
            return Flow::Continue;
        }
        let Some(keys) = KEYS.get() else { return Flow::Continue };
        let Some(data) = event.block_entity.and_then(|e| level.block_entity_data(e)) else { return Flow::Continue };
        let Some(mut stack) = ItemStack::new(keys.spawner, 1) else { return Flow::Continue };
        if let Some(carried) = logic::from_spawner(&Tag::Compound(data)) {
            let custom = Tag::compound([(logic::CARRIED, logic::encode(&carried))]);
            let patch: Compound = [(CUSTOM_DATA.into(), custom)].into_iter().collect();
            if stack.apply_components(&patch).is_err() {
                return Flow::Continue;
            }
        }
        let p = event.pos;
        let at = [f64::from(p.x) + 0.5, f64::from(p.y) + 0.5, f64::from(p.z) + 0.5];
        event.drops.push(ItemDrop::new(stack, at, [0.0, 0.1, 0.0], 10));
        Flow::Continue
    }

    /// A mob from a spawner with echoes drops its loot that many more times.
    fn loot_drops(&self, host: &mut dyn ServerHost, event: &mut LootDropsEvent<'_>) {
        let context = event.context;
        let Some((id, _)) = context.entity.filter(|_| !context.used && context.damage_source.is_some()) else { return };
        let echoes = host
            .entity(id)
            .and_then(|e| e.tags(&*host))
            .and_then(|tags| logic::counted_in(&tags, "echo"))
            .unwrap_or(0.0) as usize;
        if echoes > 0 {
            let once = event.drops.clone();
            for _ in 0..echoes.min(3) {
                event.drops.extend(once.iter().cloned());
            }
        }
    }

    /// A mob from a weakening spawner comes down to its share of health as it joins the world, once.
    fn entity_join_level(&self, host: &mut dyn ServerHost, event: &mut EntityJoinLevelEvent<'_>) -> Flow {
        if matches!(event.kind, "minecraft:item" | "minecraft:experience_orb") {
            return Flow::Continue;
        }
        let Some(entity) = host.entity(event.entity) else { return Flow::Continue };
        let Some(tags) = entity.tags(&*host) else { return Flow::Continue };
        let prefix = format!("{}vigour=", logic::TAG);
        let Some(tag) = tags.iter().find(|t| t.starts_with(&prefix)) else { return Flow::Continue };
        if let (Some(share), Some(living)) = (logic::counted_in(&tags, "vigour"), host.living(entity))
            && let Some(max) = living.max_health(&*host)
        {
            living.set_health(host, (max * share as f32).max(1.0));
        }
        entity.remove_tag(host, tag);
        Flow::Continue
    }

    fn level_tick_pre(&self, host: HostSide, _event: &mut LevelTickPreEvent) {
        if let HostSide::Server(host) = host {
            settle_neighboured(host);
        }
    }
}
