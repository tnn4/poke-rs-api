# see: https://stackoverflow.com/questions/35112929/how-to-specify-the-path-to-a-cargo-toml

import subprocess
import argparse

# initializer argument parser for CLI
def init_parser(): # -> argparse.ArgumentParser
    parser = argparse.ArgumentParser(description="download pokeapi")
    parser.add_argument(
        '-d',
        '--dry-run',
        action="store_true",
        help="show command"
    )
    parser.add_argument(
        '-b',
        '--build-dev',
        action="store_true",
        help="build debug version"
    )
    parser.add_argument(
        '-br',
        '--build-release',
        action="store_true",
        help="build release version"
    )
    parser.add_argument(
        '-r',
        '--run-dev',
        action="store_true",
        help="run debug version"
    )
    parser.add_argument(
        '-rr',
        '--run-release',
        action="store_true",
        help="run release version"
    )
    parser.add_argument(
        '-o',
        '--option',
        action="store",
        help="build option: b=build-debug, br=build-release, r=run-debug, rr=run-release"
    )
    return parser
#fed


cargo_path="poke-rs-api/"
cargo_file="Cargo.toml"
# Debug builds are the default for cargo.
cmd_str_b="cargo build --manifest-path " + cargo_path + cargo_file
cmd_str_br="cargo build --manifest-path " + cargo_path + cargo_file + " --release"
cmd_str_r="cargo run --manifest-path " + cargo_path + cargo_file
cmd_str_rr="cargo run --manifest-path " + cargo_path + cargo_file + " --release"

def run_cargo(option):
    match option:
        case "b":
            subprocess.run(cmd_str_b, shell=True)
        case "br":
            subprocess.run(cmd_str_br, shell=True)
        case "r":
            subprocess.run(cmd_str_r, shell=True)
        case "rr":
            subprocess.run(cmd_str_rr, shell=True)
        case _:
            print("invalid option")
    #match
#fed

if __name__ == "__main__":
    

    parser = init_parser()
    args = parser.parse_args()

    cmd_str="NULL"

    print(cmd_str)

    run_cargo(args.option)
#fi
