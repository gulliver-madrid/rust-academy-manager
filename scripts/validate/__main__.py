# Checks all keys used in the program have their translation in locale files

import sys
from collections import defaultdict
from typing import Final, Sequence
from pathlib import Path

from .translations import TranslationKey, TranslationsExplorer
from .key_finder import KeyFinder
from .types import KeysToPaths, SrcPath, SrcPaths, Pattern
from .patterns import regex_patterns

DEBUG: Final = False



def main() -> None:
    args = sys.argv[1:]
    if len(args) < 2:
        print("Should be called with 2 arguments: src folder and locale folder")
        sys.exit()
    patterns = [Pattern(regex) for regex in regex_patterns]
    src_path_str, locale_path_str = sys.argv[1:]
    src_base_path = Path.cwd() / src_path_str
    locale_dir = Path.cwd() / locale_path_str
    print(f"src dir: {src_base_path}")
    print(f"locale dir: {locale_dir}\n")
    if not locale_dir.exists():
        print("Locale directory introduced does not exist")
        sys.exit()
    key_finder = KeyFinder(src_base_path)
    keys_to_paths = key_finder.get_used_keys(patterns)
    if len(keys_to_paths) == 0:
        print("No keys found in src directory")
        sys.exit()

    translations_explorer = TranslationsExplorer()
    defined_keys = translations_explorer.get_defined_keys(locale_dir)
    all_keys_found = True
    for key, paths in keys_to_paths.items():
        if key not in defined_keys.all_keys:
            all_keys_found = False
            show_msg_found_keys_not_defined(key, paths, src_base_path)
        else:
            key_missing_langs = defined_keys.get_missing_langs_for_key(key)
            if key_missing_langs:
                all_keys_found = False
                show_msg_found_keys_not_defined(key, paths, src_base_path, key_missing_langs)
    if all_keys_found:
        print("All keys were found")

    if missing_in_source_code := defined_keys.all_keys.difference(keys_to_paths.keys()):
        print("\nSome keys defined in translation files were not used in the program:\n  " + "\n  ".join(missing_in_source_code))
        if DEBUG:
            input("Press ENTER to debug found keys")
            debug_found_keys(keys_to_paths, src_base_path)



def show_msg_found_keys_not_defined(key: TranslationKey, paths: SrcPaths, src_path: Path, langs: Sequence[str] = None) -> None:
    key_sources = [path.relative_to(src_path) for path in paths]
    if not langs:
        langs_str = "any language"
    elif len(langs) == 1:
        langs_str = f"language '{langs[0]}'"
    else:
        langs_list = ','.join(langs)
        langs_str = f"languages '{langs_list}'"
    if len(key_sources) == 1:
        src_str = f" from '{key_sources[0]}',"
    else:
        src_str = ""
    print(f"""  Key '{key}',{src_str} not found for {langs_str}.""")
    if len(key_sources) > 1:
        show_locations(key_sources)


def show_locations(key_sources: Sequence[Path]):
    print("It appears in the next src files:")
    for path in key_sources:
        print(f"\t{path}")


def debug_found_keys(keys_to_paths: KeysToPaths, base_dir: Path) -> None:
    paths_to_keys: dict[SrcPath, list[str]] = defaultdict(list)
    for key, paths in keys_to_paths.items():
        for path in paths:
            paths_to_keys[path].append(key)
    for path, keys in paths_to_keys.items():
        relative_path = path.relative_to(base_dir)
        print(f"Discovered in {relative_path}:")
        for key in keys:
            print(f"\t{key}")



if __name__ == '__main__':
    main()
