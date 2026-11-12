# Vast Spawners

A native mod and a content mod for [Horizon](https://github.com/horizon-rust/horizon): change spawners with items,
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

A spawner mined with a Silk Touch tool drops itself, with everything it was changed to and the mob it makes. Placed
again, it is the same spawner. The item's tooltip lists the mob and the changes.

## Jade and JEI

- **Jade:** looking at a spawner shows each of its numbers (waits, batch size, crowd limit, player reach, spread)
  and every modifier it carries. The lines can be turned off in Jade's plugin options (Vast Spawners).
- **JEI (Just Enough Items):** a **Spawner Modifiers** category: **U** on a modifier shows what it does to a spawner,
  its limit and what it does with quartz in the off hand; **U** on a spawner lists every modifier. It needs a Just
  Enough Items with plugin categories (`register_category`); with an older one only the tooltips below show.
- **Modifiers' tooltips:** every modifier item says what one use does to a spawner and what quartz does.

Each integration is compiled in only when its mod is in the same build: without Jade or JEI, nothing of it is.

## How it works

Everything lives in the spawner's own data: the game's six numbers, light rules in its spawn data, and the rest as
tags on the mob it makes (`vastspawners.*`), which the mob keeps. Its mobs also keep their changes in the world
(a dazed mob stays dazed).

## Not yet

- Apothic Spawners' Capturing enchantment (mobs dropping their spawn eggs): Horizon reads only the game's own
  enchantments, so it needs a way of its own.
- Unstable spawners (an exploded spawner turning unstable), a delay before spawner mobs despawn, the spawn egg
  blacklist and the advancements.
- Dazed mobs don't fall or get pushed as Apothic Spawners' movable ones do.

## Install

- **The native mod:** `vastspawners_native-<version>.hnmod`. Install it, then build natives in the launcher.
- **The content mod:** `vastspawners-<version>.hmod`, with the words in English and Brazilian Portuguese. Install
  both.

## Building

- `tools/build_content.py` builds `content/vastspawners` (lang).
- `cargo test` runs the tests of `mixins/vastspawners_native/logic.rs` (built on its own by `src/lib.rs`).
- `tools/package.py` packs both files into `dist/`.

It is pinned to Horizon `fdc87ae` (Nightly 2026-11-08): besides the native hook points it reads the range in the
spawner's own tick (`server_tick_spawner_of`) for wired spawners.

## License

MIT, see [LICENSE](LICENSE). Apothic Spawners is by Shadows_of_Fire (Stormraven Studios, LLC): its code and data
are MIT ([NOTICE-apothic-spawners.txt](NOTICE-apothic-spawners.txt)); its assets and lang are all rights reserved
and none are used. Not affiliated with or endorsed by Apothic Spawners' authors.
