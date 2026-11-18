#!/usr/bin/env python3
"""Builds the mod's assets, then packs `mod/` into dist/vastspawners-<version>.hmod with the official packer
(`cargo horizon-mod build`, which needs the pinned build's SDK)."""
import hashlib
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MOD = ROOT / "mod"


def main():
    subprocess.run([sys.executable, "-I", str(ROOT / "tools/build_content.py")], check=True)
    # The mod has no registry dependencies: a lock from a local `cargo check` is not part of the package.
    (MOD / "Cargo.lock").unlink(missing_ok=True)
    for name in ("LICENSE", "NOTICE-apothic-spawners.txt"):
        shutil.copy(ROOT / name, MOD / name)
    subprocess.run(["cargo", "horizon-mod", "build"], cwd=MOD, check=True)
    version = tomllib.loads((MOD / "horizon-mod.toml").read_text())["version"]
    dist = ROOT / "dist"
    dist.mkdir(exist_ok=True)
    out = dist / f"vastspawners-{version}.hmod"
    shutil.copy(MOD / "target/horizon-mod/vastspawners.hmod", out)
    print(f"{out}  {hashlib.sha256(out.read_bytes()).hexdigest()}")


if __name__ == "__main__":
    main()
