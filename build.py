#!/usr/bin/env python3
"""brickrust build tool -- Python port of the Makefile.

Each subcommand maps to a make target. See Makefile for the source of truth
this mirrors. Run ``python build.py help`` for usage.
"""

import argparse
import pathlib
import shlex
import shutil
import subprocess
import sys

TRIPLE = "x86_64-pc-windows-gnu"
DEFAULT_MINGW = "/usr/x86_64-w64-mingw32/bin"
GAME_APP_ID = "552100"
REPO_ROOT = pathlib.Path(__file__).resolve().parent
BRMK_PLUGIN = REPO_ROOT / "brmk_plugin"

# Build variant -> cargo features. Both variants compile to TRIPLE; the
# difference is the feature set (standalone vs BRMK plugin), matching the
# Makefile's standalone vs BRMK plugin variants.
TARGETS = {
    "windows": "brickworks_impl/impl",
    "brmk": "brmk,brickworks_impl/impl",
}


def target_dirs(dev: bool, target_base: pathlib.Path):
    """Return (release_dir, current_target_dir) matching the Makefile's
    $(TARGET_RELEASE) and $(TARGET) ($(TARGET) = debug when dev, else release)."""
    base = pathlib.Path(target_base) / TRIPLE
    release = base / "release"
    current = base / "debug" if dev else release
    return release, current


def run(cmd, *, dry_run: bool):
    print("$ " + " ".join(shlex.quote(str(c)) for c in cmd))
    if dry_run:
        return
    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as exc:
        sys.exit(f"error: command failed (exit {exc.returncode}): {' '.join(cmd)}")


def copy(src, dst, *, dry_run: bool):
    src = pathlib.Path(src)
    dst = pathlib.Path(dst)
    print(f"cp {shlex.quote(str(src))} {shlex.quote(str(dst))}")
    if dry_run:
        return
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)


def mkdir_p(path, *, dry_run: bool):
    path = pathlib.Path(path)
    print(f"mkdir -p {shlex.quote(str(path))}")
    if dry_run:
        return
    path.mkdir(parents=True, exist_ok=True)


def build_cmd(features, *, dev: bool, target_base: pathlib.Path, dry_run: bool):
    cmd = ["cargo", "build", "--target", TRIPLE, "--workspace", "--features", features]
    if not dev:
        cmd.append("-r")
    run(cmd, dry_run=dry_run)


def cmd_build(args, *, dry_run: bool):
    build_cmd(
        TARGETS[args.target], dev=args.dev, target_base=args.target_base, dry_run=dry_run
    )


def cmd_doc(args, *, dry_run: bool):
    run(
        ["cargo", "doc", "--target", TRIPLE, "--workspace", "--examples"],
        dry_run=dry_run,
    )


def _install_windows(args, *, dry_run: bool):
    base = pathlib.Path(args.dir)
    mingw = pathlib.Path(args.mingw)
    release, target = target_dirs(args.dev, args.target_base)
    binaries = base / "BrickRigs" / "Binaries" / "Win64"

    # workspace build (covers xinput1_3 implicitly)
    build_cmd(TARGETS["windows"], dev=args.dev, target_base=args.target_base, dry_run=dry_run)

    # copy artifacts (faithful to the Makefile target paths)
    copy(target / "xinput1_3.dll", binaries / "xinput1_3.dll", dry_run=dry_run)
    copy(target / "deps" / "brickworks.dll", binaries / "brickworks.dll", dry_run=dry_run)
    copy(mingw / "libgcc_s_seh-1.dll", base / "libgcc_s_seh-1.dll", dry_run=dry_run)
    copy(mingw / "libwinpthread-1.dll", base / "libwinpthread-1.dll", dry_run=dry_run)
    mkdir_p(base / "brickworks", dry_run=dry_run)


def _install_brmk(args, *, dry_run: bool):
    base = pathlib.Path(args.dir)
    mingw = pathlib.Path(args.mingw)
    _, target = target_dirs(args.dev, args.target_base)
    plugin_dir = base / "BrickRigs" / "Plugins" / "BrickRust"

    # build + install in a single step (mirrors `install_brmk: build_brmk`)
    build_cmd(TARGETS["brmk"], dev=args.dev, target_base=args.target_base, dry_run=dry_run)

    mkdir_p(plugin_dir / "Binaries" / "Win64", dry_run=dry_run)
    copy(
        target / "brmk_plugin.dll",
        plugin_dir / "Binaries" / "Win64" / "BrickRigsModKitSteam-BrickRust.dll",
        dry_run=dry_run,
    )
    copy(
        BRMK_PLUGIN / "BrickRigsModKitSteam.module",
        plugin_dir / "Binaries" / "Win64" / "BrickRigsModKitSteam.module",
        dry_run=dry_run,
    )
    copy(
        BRMK_PLUGIN / "BrickRust.uplugin",
        plugin_dir / "BrickRust.uplugin",
        dry_run=dry_run,
    )
    copy(
        target / "brickworks.dll",
        base / "BrickRigs" / "Binaries" / "Win64" / "brickworks.dll",
        dry_run=dry_run,
    )
    copy(mingw / "libgcc_s_seh-1.dll", base / "BrickRigs" / "Binaries" / "Win64", dry_run=dry_run)
    copy(mingw / "libwinpthread-1.dll", base / "BrickRigs" / "Binaries" / "Win64", dry_run=dry_run)
    mkdir_p(base / "BrickRigs" / "Binaries" / "Win64" / "brickworks", dry_run=dry_run)


def cmd_install(args, *, dry_run: bool):
    if not args.dir:
        sys.exit("error: install requires --dir <path to Brick Rigs>")
    if args.target == "brmk":
        _install_brmk(args, dry_run=dry_run)
    else:
        _install_windows(args, dry_run=dry_run)


def cmd_run(args, *, dry_run: bool):
    cmd_install(args, dry_run=dry_run)
    print(f"steam -applaunch {GAME_APP_ID}")
    if not dry_run:
        subprocess.run(["steam", "-applaunch", GAME_APP_ID], check=True)


HELP = """\
brickrust build tool (Python port of the Makefile)

Usage: python build.py <command> [options]

Options:
  --target {windows,brmk}   build variant (default: windows)
  --dev                     enable development builds
  --mingw <path>            mingw libraries directory (default: /usr/x86_64-w64-mingw32/bin)
  --target-base <dir>       cargo target base dir (default: target)
  --dry-run                 print actions without executing

Commands:
  build                     Build the windows workspace
  doc                       Build cargo docs for examples
  install                   Build + install for the windows variant
  run                       Install then launch the game via Steam

Examples:
  python build.py build --target=windows
  python build.py build --target=brmk --dev
  python build.py install --dir "$HOME/.steam/steam/steamapps/common/Brick Rigs"
  python build.py install --dir ".../BrickRigs" --target=brmk
  python build.py run --dir ".../Brick Rigs"
"""


def cmd_help(args, *, dry_run: bool):
    print(HELP)


def build_parser():
    global_opts = argparse.ArgumentParser(add_help=False)
    global_opts.add_argument(
        "--dry-run",
        action="store_true",
        help="print actions without executing (cargo/cp/mkdir)",
    )
    global_opts.add_argument(
        "--target-base",
        default="target",
        help="base directory for cargo targets (default: target)",
    )

    parser = argparse.ArgumentParser(
        prog="build.py",
        description="Python port of the brickrust Makefile.",
        parents=[global_opts],
    )
    sub = parser.add_subparsers(dest="command", required=True)

    handlers = {
        "help": cmd_help,
        "doc": cmd_doc,
        "build": cmd_build,
        "install": cmd_install,
        "run": cmd_run,
    }

    def reg(name, help_text, opts=(), target_default=None):
        p = sub.add_parser(name, help=help_text, parents=[global_opts])
        for flags, kwargs in opts:
            p.add_argument(*flags, **kwargs)
        if target_default is not None:
            p.add_argument(
                "--target",
                choices=list(TARGETS),
                default=target_default,
                help=f"build variant (default: {target_default})",
            )
        p.set_defaults(func=handlers[name])
        return p

    dev = (["--dev"], {"action": "store_true", "help": "enable development builds"})
    ddir = (["--dir"], {"help": "path to the Brick Rigs common directory"})
    mingw = (["--mingw"], {"default": DEFAULT_MINGW, "help": "mingw libraries directory"})

    reg("help", "show this help")
    reg("doc", "build cargo docs for examples")

    reg("build", "build the windows workspace", [dev], target_default="windows")

    reg(
        "install",
        "build + install for the windows variant",
        [dev, ddir, mingw],
        target_default="windows",
    )
    reg("run", "install then launch the game", [dev, ddir, mingw], target_default="windows")

    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)
    args.func(args, dry_run=args.dry_run)
    return 0


if __name__ == "__main__":
    sys.exit(main())
