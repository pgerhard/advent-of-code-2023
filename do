#!/Users/pgerhard/.pyenv/shims/python3

from datetime import datetime

import argparse
import os
import subprocess


FORCE=False
LIVE=False
VERBOSE=False
HOME_DIR=os.path.expanduser("~")
SCRIPT_PATH = os.path.dirname(os.path.realpath(__file__))


class ConsoleColors:
    BLACK = '\033[30m'
    RED = '\033[31m'
    GREEN = '\033[32m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    MAGENTA = '\033[35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'
    RESET = '\033[0m'


def _write_log(msg):
    date_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    print(f"{ConsoleColors.BLUE}{date_time} {msg}{ConsoleColors.RESET}")


def _info(msg):
    global LIVE
    _write_log(
        f"{ConsoleColors.MAGENTA}[INFO] "
        f"{ConsoleColors.GREEN}{'[Live]:' if LIVE else '[Dry-Run]:'} "
        f"{msg}"
    )


def _debug(msg):
    global LIVE
    global VERBOSE
    if VERBOSE:
        _write_log(
            f"{ConsoleColors.MAGENTA}[DEBUG] "
            f"{ConsoleColors.GREEN}{'[Live]:' if LIVE else '[Dry-Run]:'} "
            f"{msg}"
        )


def _error(msg):
    global LIVE
    _write_log(
        f"{ConsoleColors.RED}[ERROR] "
        f"{ConsoleColors.GREEN}{'[Live]:' if LIVE else '[Dry-Run]:'} "
        f"{msg}"
    )

def _create_rust_project(day):
    global LIVE

    day_project_path = f"{SCRIPT_PATH}/rust"
    _debug(f"Creating Rust project at: '{day_project_path}/{day}'")
    if LIVE:
        os.makedirs(day_project_path, exist_ok=True)
        print(subprocess.check_output(
                    ['cargo', 'new', day],
                    cwd=f"{SCRIPT_PATH}/rust"
                )
            )
        print(subprocess.check_output(
                    ['rustrover'],
                    cwd=f"{SCRIPT_PATH}/rust/{day}"
                )
            )


def main():
    global LIVE
    global VERBOSE

    parser = argparse.ArgumentParser(description='Script to download model files')
    parser.add_argument('-v', '--verbose', action='store_true', help='Enable verbose mode')
    parser.add_argument('-l', '--live', action='store_true', help='Enable live mode')
    parser.add_argument('language', nargs=1, help='Languge of the project', choices=['rust'])
    parser.add_argument('day', nargs=1, help='Advent of code day')

    args = parser.parse_args()

    _info(f"args: {args}")

    LIVE = True if args.live else False
    VERBOSE = True if args.verbose else False

    _info(f"Live: {LIVE}")
    _info(f"Verbose: {VERBOSE}")

    language = args.language[0]
    day = f"day-{args.day[0]}"

    _info(f"Selected language: '{language}'")
    _info(f"Selected day: '{day}'")
    
    if language == "rust":
        _create_rust_project(day)
    else:
        _error(f"No valid language selected. Got {language}")


if __name__ == "__main__":
    main()