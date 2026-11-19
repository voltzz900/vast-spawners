# Registry page: Vast Spawners

- **File:** `vastspawners-0.3.1.hmod` (release v0.3.1), kind: native mod
- **Icon:** `icons/vastspawners.png`
- **Name:** Vast Spawners
- **Summary:** Change spawners with items, mine them with Silk Touch, and shape the mobs they make.
- **Categories:** game-mechanics, mobs, utility
- **Tags:** spawners, mob-farms, silk-touch, redstone, translation
- **License:** MIT
- **Source:** https://github.com/voltzz900/vast-spawners
- **Authors:** voltzz900

## Description

A native mod for spawners, built from the MIT code and data of [Apothic Spawners](https://github.com/Shadows-of-Fire/Apothic-Spawners) (by Shadows_of_Fire), with names and words of its own, on Horizon's mod API. One package: the code and its words in English and Brazilian Portuguese.

- **Change a spawner** by right clicking it with an item, used up: sugar and a clock shorten its waits, a fermented spider eye, a ghast tear, prismarine crystals and a piston raise its batch size, crowd limit, player reach and spread. With quartz in the other hand the change goes back.
- **Switches**: a nether star makes it work with no player near, a conduit or a soul lantern lets it spawn anywhere or in any light, a comparator makes it work only with a redstone signal, and chorus fruit, wool, a golden dandelion and a campfire make its mobs dazed (no AI), hushed, young or burning.
- **Echoes** (echo shards, up to 3): its mobs' loot drops once more per echo. **Vigour** (pointed dripstone): its mobs spawn with less health.
- **Silk Touch** mines a spawner with all of that and its mob; placed again, it is the same spawner. Its tooltip lists them.
- Every modifier's tooltip says what it does.

**What it changes in the game**: a right click on a spawner with those items (the item is spent, the spawner's data changes), a spawner's drop when mined with Silk Touch and its data when placed, the range a wired spawner looks for a player in, the health of a weakened spawner's mobs, and the loot of an echoing spawner's mobs. It works through the mod API (hooks and Parts on the vanilla spawner; no mixins). Everything it keeps lives in the spawner's and the mob's own data; no files, no network.

0.3.0 replaces 0.2's two files (`vastspawners_native` .hnmod and the `vastspawners` content mod); spawners changed with them keep their changes. Pinned to Horizon 0b14f8a. Not yet: Jade and JEI lines (0.2 compiled them into those mods' builds; the mod API has no way yet), the egg-catching enchantment, unstable spawners and advancements. Not affiliated with or endorsed by Apothic Spawners' authors; none of its assets or lang are used.
