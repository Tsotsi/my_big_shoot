import toml
import argparse
import os
import pathlib
import re

_idle_cfg = {
    "mode": "Once",
}
__idle_name__ = "idle"


def generate_idle_cfg_files(
    file_path: str, name: str, action: str, force: bool = False
):
    """Generate idle cfg files
    """
    # with os.scandir(file_path) as it:
    #     for entry in it:
    #         if not entry.name.startswith(".") and entry.is_file():
    #             print(entry.name)
    glob_str = "{}_{}*.animation.toml".format(name, action)
    full_path = os.path.join(root_dir(), file_path)
    full_path_obj = pathlib.Path(full_path)
    if not full_path_obj.exists():
        print("{} not exists".format(full_path))
        return
    files = full_path_obj.glob(glob_str)
    new_file_path_tpl = "{}_{}{}.animation.toml"
    for file in files:
        with file.open() as f:
            cfg = toml.loads("".join(f.readlines()))
            new_cfg = dict(cfg)
            new_cfg.update(_idle_cfg)
            # only keep one frames
            new_cfg["frames"] = new_cfg["frames"][:1]
            # write to new file
            res = re.search(
                r"{}_{}(?P<suffix>[a-z_]+)?.animation.toml$".format(name, action),
                file.name,
            )
            new_cfg_path = new_file_path_tpl.format(name, __idle_name__, res.group(1))
            full_new_cfg_path = os.path.join(full_path, new_cfg_path)
            p = pathlib.Path(full_new_cfg_path)
            if p.exists() and not force:
                print("already exists: {}".format(full_new_cfg_path))
            else:
                with p.open("w") as fp:
                    fp.write(toml.dumps(new_cfg))
                print("generated file: {}".format(full_new_cfg_path))


def root_dir():
    return os.path.dirname(os.path.dirname(__file__))


if __name__ == "__main__":
    """
    python ./scripts/gen_idle_animation_cfg.py --name chara_1_1 --action walk --path assets/characters/animations
    """
    parser = argparse.ArgumentParser(
        description="gen idle animation cfg files from other files"
    )
    parser.add_argument(
        "--name", required=True, type=str, help="main name of animation"
    )
    parser.add_argument("--action", required=True, type=str, help="action name")
    parser.add_argument(
        "--force", type=bool, default=False, help="force generate files, default: False"
    )
    default_path = "assets/characters/animations"
    parser.add_argument(
        "--path",
        default=default_path,
        type=str,
        help="directory of files, default: {}".format(default_path),
    )
    args = parser.parse_args()
    # generate_idle_cfg_files(os.path.join(root_dir(), args.path), "{}_{}_*.animation.toml".format(args.name, args.action))
    generate_idle_cfg_files(args.path, args.name, args.action, args.force)
