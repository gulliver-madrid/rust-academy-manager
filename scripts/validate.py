# Checks all keys used in the program have their translation in locale files

from typing import Any, Sequence
import yaml

from pathlib import Path
import re
import os
import sys

PathsToLines = dict[Path, list[str]]
KeysToPath = dict[str, Path]


def main() -> None:
    args = sys.argv[1:]
    if len(args) < 2:
        print("Should be called with 2 arguments: src folder and locale folder")
        sys.exit()
    patterns = [
        re.compile(r"""
            [^a-zA-Z]       # Some non alphabetic character (prevent format! false positives)
            t!\("           # t!("
            ([^"]+)         # the translation key as a captured group
            "\)             # ")
    """, re.VERBOSE), 

        # example: MenuOption::Teachers, "menu_options.teacher")
        re.compile(r"""
            MenuOption::
            [A-Z][a-zA-Z0-9_]*       # Some pascal identifier
            ,\s*"           # , "
            ([^"]+)         # the translation key as a captured group
            "             # "
    """, re.VERBOSE), 
    ]
    src_path_str, locale_path_str = sys.argv[1:]
    src_path = Path.cwd() / src_path_str
    used_keys_by_pattern = {pattern: get_used_keys(src_path, ".rs", pattern) for pattern in patterns}
    # used_keys = get_used_keys(src_path, ".rs", pattern)
    locale_dir = Path.cwd() / locale_path_str
    print(f"src dir: {src_path}")
    print(f"locale dir: {locale_dir}")
    if not locale_dir.exists():
        print("Locale directory introduced does not exist")
        sys.exit()
    if all(len(used_keys) == 0 for used_keys in used_keys_by_pattern.values()):
        print("No keys found in src directory")
        sys.exit()
    paths_to_parsed = get_paths_to_parsed(locale_dir, ".yml")
    all_keys_found = True
    for pattern in patterns:
        used_keys = used_keys_by_pattern[pattern]
        for key in used_keys:
            for path, parsed in paths_to_parsed.items():
                found = False
                for translations in parsed.values():
                    if "." in key:
                        key_1, key_2 = key.split(".")
                        if key_1 in translations.keys():
                            subtranslations = translations[key_1]
                            if key_2 in subtranslations.keys():
                                found = True
                                break
                    elif key in translations.keys():
                        found = True
                        break
                if not found:
                    key_source_path = used_keys[key]
                    key_source = os.path.relpath(key_source_path, src_path)
                    print(f"Key '{key}', from '{key_source}', not found in file '{path.name}'.")
                    all_keys_found = False
    if all_keys_found:
        print("All keys were found")


def get_paths_to_parsed(base_dir: Path, ext: str) -> dict[Path, Any]:
    """Builds a dict mapping translations files paths to parsed content"""
    yml_paths = get_paths_with_extension(base_dir, ext)
    files_to_contents = get_path_to_contents(yml_paths)
    paths_to_parsed = {
        path: yaml.load(content, yaml.Loader)
        for path, content in files_to_contents.items()
    }
    return paths_to_parsed


def get_path_to_contents(paths: Sequence[Path]) -> dict[Path, str]:
    files_to_contents = {}
    for path in paths:
        with open(path, "r") as file:
            files_to_contents[path] = file.read()
    return files_to_contents


def get_used_keys(base_dir: Path, ext: str, pattern: re.Pattern[str]) -> KeysToPath:
    paths = get_paths_with_extension(base_dir, ext)
    files_to_contents = get_path_to_contents(paths)
    paths_to_lines: PathsToLines = {
        path: content.split("\n") 
        for path, content in files_to_contents.items()
    }
    used_keys: dict[str, Path] = extract_used_keys(pattern, paths_to_lines)
    return used_keys


def extract_used_keys(pattern: re.Pattern[str], paths_to_lines: PathsToLines) -> KeysToPath:
    keys_used = {}
    for path, lines in paths_to_lines.items():
        for line in lines:
            if m := pattern.search(line):
                groups = m.groups()
                assert len(groups) == 1, groups
                keys_used[groups[0]] = path
                # print(groups[0])
    return keys_used



def get_paths_with_extension(base_dir: Path, ext: str) -> list[Path]:
    assert ext.startswith("."), ext
    all_paths = base_dir.glob('**/*' + ext)
    return [p for p in all_paths if p.is_file()]


if __name__ == '__main__':
    main()
