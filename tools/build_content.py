#!/usr/bin/env python3
"""Builds content/vastspawners: Vast Spawners' words in English and Brazilian Portuguese (its own)."""
import json
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "content/vastspawners"
VERSION = "0.2.0"

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
    "quartz": ("With quartz", "Com quartzo"),
    "jei.title": ("Spawner Modifiers", "Modificadores de Spawner"),
}


def main():
    if OUT.exists():
        shutil.rmtree(OUT)
    en, br = {}, {}
    for key, (e, p) in STATS.items():
        en[f"vastspawners.stat.{key}"], br[f"vastspawners.stat.{key}"] = e, p
    for key, (e, p) in WORDS.items():
        en[f"vastspawners.{key}"], br[f"vastspawners.{key}"] = e, p
    lang = OUT / "assets/vastspawners/lang"
    lang.mkdir(parents=True)
    (lang / "en_us.json").write_text(json.dumps(dict(sorted(en.items())), indent=2, ensure_ascii=False) + "\n")
    (lang / "pt_br.json").write_text(json.dumps(dict(sorted(br.items())), indent=2, ensure_ascii=False) + "\n")
    (OUT / "horizon-mod.toml").write_text(
        'id = "vastspawners"\n'
        'name = "Vast Spawners (content)"\n'
        f'version = "{VERSION}"\n'
        'description = "Vast Spawners\' words in English and Brazilian Portuguese. Install it with the native mod."\n'
        'authors = ["voltzz900"]\n'
        'api = "0.2"\n'
        'side = "both"\n'
        'capabilities = ["registries"]\n'
    )
    print(f"{OUT}: {len(en)} lang keys")


if __name__ == "__main__":
    main()
