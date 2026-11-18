#!/usr/bin/env python3
"""Builds Vast Spawners' content into `mod/` (the mod's package): its words in English and Brazilian Portuguese
(its own) in `mod/assets/vastspawners/lang`, and `mod/horizon-mod.toml`.

Usage: build_content.py
"""
import json
import shutil
from pathlib import Path

NS = "vastspawners"
VERSION = "0.3.1"
# The Horizon build the mod is made for, and the horizon-mixins tool its `[native]` names.
HORIZON = "0b14f8acfd97fa2c750e42795249dcc981178ca4"
MIXINS = "e59da9094f0c2fdda6c627fe9eff22dec6a906f0"
ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "mod"

STATS = {
    "shortest_wait": ("Shortest Wait", "Espera Mínima"),
    "longest_wait": ("Longest Wait", "Espera Máxima"),
    "batch": ("Batch Size", "Tamanho do Lote"),
    "crowd": ("Crowd Limit", "Limite de Multidão"),
    "reach": ("Player Reach", "Alcance do Jogador"),
    "spread": ("Spread", "Espalhamento"),
    "unwatched": ("Unwatched", "Sem Vigia"),
    "anywhere": ("Any Ground", "Qualquer Chão"),
    "lightproof": ("Lightproof", "À Prova de Luz"),
    "wired": ("Wired", "Ligado à Redstone"),
    "dazed": ("Dazed", "Atordoado"),
    "hushed": ("Hushed", "Silenciado"),
    "young": ("Young", "Filhote"),
    "kindled": ("Kindled", "Em Chamas"),
    "echoes": ("Echoes", "Ecos"),
    "vigour": ("Vigour", "Vigor"),
}
WORDS = {
    "on": ("on", "ligado"),
    "off": ("off", "desligado"),
    "at_limit": ("already at its limit", "já está no limite"),
    "mob": ("Mob", "Mob"),
    "modifier": ("Spawner modifier", "Modificador de spawner"),
    "reverse": ("With quartz in the off hand", "Com quartzo na mão secundária"),
    "limit": ("limit", "limite"),
}


def main():
    lang = OUT / "assets" / NS / "lang"
    if (OUT / "assets").exists():
        shutil.rmtree(OUT / "assets")
    lang.mkdir(parents=True)
    en, br = {}, {}
    for key, (e, p) in STATS.items():
        en[f"{NS}.stat.{key}"], br[f"{NS}.stat.{key}"] = e, p
    for key, (e, p) in WORDS.items():
        en[f"{NS}.{key}"], br[f"{NS}.{key}"] = e, p
    (lang / "en_us.json").write_text(json.dumps(dict(sorted(en.items())), indent=2, ensure_ascii=False) + "\n")
    (lang / "pt_br.json").write_text(json.dumps(dict(sorted(br.items())), indent=2, ensure_ascii=False) + "\n")
    (OUT / "horizon-mod.toml").write_text(
        f'id = "{NS}"\nname = "Vast Spawners"\nversion = "{VERSION}"\n'
        'description = "Change spawners with items, mine them with Silk Touch, and shape the mobs they make."\n'
        'authors = ["voltzz900"]\napi = "0.2"\nside = "both"\ncapabilities = ["registries"]\n\n'
        f'[native]\ngame = "26.3"\nhorizon = "{HORIZON}"\nmixins = "{MIXINS}"\n')
    print(f"{NS}: {len(en)} lang keys")


if __name__ == "__main__":
    main()
