# Vast Spawners

A native mod for [Horizon](https://github.com/horizon-rust/horizon), on its mod API: change spawners with items,
mine them with Silk Touch, and shape the mobs they make. It is built from the MIT code and data of
[Apothic Spawners](https://github.com/Shadows-of-Fire/Apothic-Spawners) (by Shadows_of_Fire), with names and words
of its own.

## Changing a spawner

Right click a spawner with one of these items; it is used up, and the chat says the new value. With **quartz in
the other hand** the change goes back the other way (the quartz stays).

| Item | Changes | Range |
| --- | --- | --- |
| Sugar | Shortest Wait: −0.5 s | down to 1 s (quartz: up to 80 s) |
| Clock | Longest Wait: −1 s | down to 1 s (quartz: up to 80 s) |
| Fermented Spider Eye | Batch Size: +2 | up to 16 |
| Ghast Tear | Crowd Limit: +2 | up to 32 |
| Prismarine Crystals | Player Reach: +4 | up to 48 |
| Piston | Spread: +2 | up to 32 |
| Nether Star | **Unwatched**: works with no player near | on/off |
| Conduit | **Any Ground**: ignores the mob's spawn rules and light | on/off |
| Soul Lantern | **Lightproof**: spawns in any light | on/off |
| Comparator | **Wired**: works only with a redstone signal | on/off |
| Chorus Fruit | **Dazed**: its mobs have no AI | on/off |
| Any wool | **Hushed**: its mobs make no sound | on/off |
| Golden Dandelion | **Young**: its mobs are babies | on/off |
| Campfire | **Kindled**: its mobs burn | on/off |
| Echo Shard | **Echoes**: its mobs' loot drops once more per echo | 0 to 3 |
| Pointed Dripstone | **Vigour**: its mobs spawn with less health, −5% | down to 20% |

The shortest wait never goes above the longest, nor the longest below the shortest.

## Silk Touch

A spawner mined with a Silk Touch tool drops itself, with the mod's own record of it: the mob it makes and everything
it was changed to. Placed again, the spawner is made anew from that record, checked first: the mob must be a registered
mob with a spawn egg and not a boss (the Wither, the Ender Dragon, anything tagged `c:bosses`), and every change is
kept within the modifiers' limits. Nothing else of the old spawner's data goes with the item (its mob's other data, or
a list of several mobs: the first one is kept). Items from 0.1 to 0.3.1 are read the same way. Only an operator's
item still puts raw spawner data (vanilla's `block_entity_data`), as the game itself allows only for operators. The
item's tooltip lists the mob and the changes.

## Tooltips

Every modifier item says what one use does to a spawner, its limit and what quartz does.

## How it works

Everything lives in the spawner's own data: the game's six numbers, light rules in its spawn data, and the rest as
tags on the mob it makes (`vastspawners.*`), which the mob keeps. Its mobs also keep their changes in the world
(a dazed mob stays dazed). A wired spawner without a redstone signal stores no player range (no player is ever near
it), an unwatched one the widest; while either is on, its own reach is kept in a tag. Worlds and items from 0.1 and
0.2 read the same.

## Not yet

- Apothic Spawners' Capturing enchantment (mobs dropping their spawn eggs): Horizon reads only the game's own
  enchantments, so it needs a way of its own.
- Unstable spawners (an exploded spawner turning unstable), a delay before spawner mobs despawn, the spawn egg
  blacklist and the advancements.
- Dazed mobs don't fall or get pushed as Apothic Spawners' movable ones do.
- Jade's lines and JEI's Spawner Modifiers category (0.2): they reached into those mods' code inside the game's
  build, which the mod API doesn't allow. They can come back once Jade and JEI publish functions for other mods.

## Install

`vastspawners-<version>.hmod`: one package, the code and its words in English and Brazilian Portuguese. Install it,
then build natives in the launcher. From 0.2, remove `vastspawners_native` (and the old content `vastspawners`) first.

## Building

- `tools/build_content.py` writes the lang files into `mod/assets/vastspawners` and `mod/horizon-mod.toml`.
- In `mod/`: `cargo check` and `cargo test` (the modifiers' logic, `mod/src/logic.rs`), with the SDK linked at
  `mod/.horizon/sdk/api`.
- `tools/package.py` packs `mod/` into `dist/` with `cargo horizon-mod build`.

It is pinned to Horizon `0b14f8a` and uses only the mod API's hooks and Parts.

## License

MIT, see [LICENSE](LICENSE). Apothic Spawners is by Shadows_of_Fire (Stormraven Studios, LLC): its code and data
are MIT ([NOTICE-apothic-spawners.txt](NOTICE-apothic-spawners.txt)); its assets and lang are all rights reserved
and none are used. Not affiliated with or endorsed by Apothic Spawners' authors.
