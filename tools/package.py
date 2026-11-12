#!/usr/bin/env python3
"""Packs dist/vastspawners_native-<v>.hnmod (the native mod) and dist/vastspawners-<v>.hmod (its gems and names).

Deterministic: fixed dates, sorted entries. Prints each file's SHA-256 for the registry.
"""
import hashlib
import json
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BUNDLE = ROOT / "mixins/vastspawners_native"
HOOKS = None  # pinned: it hooks the game's own functions besides the native hook points
MIXINS = "5a64af938ec86b00d53d16a408aabc0038fb3e57"


def add(z, name, data):
    info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
    info.compress_type = zipfile.ZIP_DEFLATED
    z.writestr(info, data)


def main():
    mixin = tomllib.loads((BUNDLE / "horizon-mixin.toml").read_text())
    version = mixin["mod"]["version"]
    meta = {"game": "26.3", "horizon": mixin["compatibility"]["horizon_revision"],
            "id": mixin["mod"]["id"], "mixins": MIXINS, "schema": 1, "version": version}
    if HOOKS:
        meta["hooks"] = HOOKS
    (ROOT / "horizon-native.json").write_text(json.dumps(meta, indent=2, sort_keys=True) + "\n")
    dist = ROOT / "dist"
    dist.mkdir(exist_ok=True)
    native = dist / f"vastspawners_native-{version}.hnmod"
    with zipfile.ZipFile(native, "w") as z:
        add(z, "horizon-native.json", (ROOT / "horizon-native.json").read_bytes())
        for f in sorted(BUNDLE.glob("*.rs")) + [BUNDLE / "horizon-mixin.toml"]:
            add(z, f"mixins/vastspawners_native/{f.name}", f.read_bytes())
        # The (empty) trait mod: some hook points are asked only while one is loaded.
        for f in sorted(p for p in (BUNDLE / "optional").rglob("*") if p.is_file() and "target" not in p.parts and p.name != "Cargo.lock"):
            add(z, f"mixins/vastspawners_native/{f.relative_to(BUNDLE).as_posix()}", f.read_bytes())
        for f in sorted(p for p in (BUNDLE / "api").rglob("*") if p.is_file() and "target" not in p.parts):
            add(z, f"mixins/vastspawners_native/{f.relative_to(BUNDLE).as_posix()}", f.read_bytes())
        for name in ("LICENSE", "NOTICE-apothic-spawners.txt"):
            add(z, f"mixins/vastspawners_native/{name}", (ROOT / name).read_bytes())
    content = ROOT / "content/vastspawners"
    lang_version = tomllib.loads((content / "horizon-mod.toml").read_text())["version"]
    lang = dist / f"vastspawners-{lang_version}.hmod"
    with zipfile.ZipFile(lang, "w") as z:
        for f in sorted(p for p in content.rglob("*") if p.is_file()):
            add(z, f.relative_to(content).as_posix(), f.read_bytes())
        add(z, "LICENSE", (ROOT / "LICENSE").read_bytes())
    for f in (native, lang):
        print(f"{f}  {hashlib.sha256(f.read_bytes()).hexdigest()}")


if __name__ == "__main__":
    main()
