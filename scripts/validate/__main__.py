# Checks all keys used in the program have their translation in locale files

from collections import defaultdict
import itertools
from typing import Any, Final, Iterable, Mapping, NewType, TypeVar, Union, cast
import yaml

from pathlib import Path
import sys

from .helpers import get_path_to_contents, get_paths_with_extension
from .key_finder import KeyFinder
from .types import KeysToPathsMapping, PatternsToKeysToPaths, SrcPath, SrcPaths, Pattern, YmlPath
from .patterns import regex_patterns

DEBUG: Final = False


LangKey = str  # ex. 'en'
Translation = str
TranslationKey = str
SimpleTranslations = dict[TranslationKey, Translation]
TranslationValue = Union[Translation, SimpleTranslations]  # because there are nested keys
TranslationMapping = dict[TranslationKey, TranslationValue]
Parsed = dict[LangKey, dict[TranslationKey, TranslationValue]]


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
    patterns_to_keys_to_src_paths = key_finder.get_used_keys(patterns)
    if all(len(used_keys) == 0 for used_keys in patterns_to_keys_to_src_paths.values()):
        print("No keys found in src directory")
        sys.exit()

    paths_to_parsed = get_paths_to_parsed_yml(locale_dir)

    keys_to_paths = chain_values_by_key(patterns_to_keys_to_src_paths.values())

    keys_in_source = set(keys_to_paths.keys())
    all_keys_found = True
    for yml_path, parsed in paths_to_parsed.items():
        assert len(parsed) == 1
        translations = list(parsed.values())[0]
        for key in keys_in_source:
            if not is_defined(key, translations):
                show_msg_found_keys_not_defined(keys_to_paths, key, yml_path, src_base_path)
                all_keys_found = False

    if all_keys_found:
        print("All keys were found")
    defined_keys = frozenset(get_defined_keys(paths_to_parsed))

    if missing_in_source_code := defined_keys.difference(keys_in_source):
        print("\nSome keys defined in translation files were not used in the program:\n  " + "\n  ".join(missing_in_source_code))
        if DEBUG:
            input("Press ENTER to debug found keys")
            debug_found_keys(patterns_to_keys_to_src_paths, src_base_path)

K = TypeVar('K')
V = TypeVar('V')
D = dict[K, list[V]]
def chain_values_by_key(dicts: Iterable[D]) -> D:
    keys = {k for d in dicts for k in d.keys()}
    return {k: group_by_key(dicts, k) for k in keys}


def group_by_key(dicts: Iterable[D], key: K) -> list[V]:
    return list(itertools.chain.from_iterable(d[key] for d in dicts))


def is_defined(key: TranslationKey, translations: TranslationMapping) -> bool:
    if "." in key:
        return double_key_is_defined(key, translations)
    return key in translations.keys()



def get_defined_keys(paths_to_parsed: dict[YmlPath, Parsed]) -> set[TranslationKey]:
    """Return a set with all the defined keys in locale files"""
    keys_in_parsed = set()
    for _lang_path, parsed in paths_to_parsed.items():
        for _lang, translations in parsed.items():
            for k, v in translations.items():
                if isinstance(v, str):
                    keys_in_parsed.add(k)
                else:
                    assert isinstance(v, dict)
                    key_1 = k
                    for key_2, translation in v.items():
                        assert isinstance(translation, str), translation
                        double_key = ".".join([key_1, key_2])
                        keys_in_parsed.add(double_key)
    return keys_in_parsed


def double_key_is_defined(key: TranslationKey, translations: TranslationMapping) -> bool:
    """Check a key "part_1.part_2" style. Returns True if it matches"""
    key_1, key_2 = key.split(".")
    if key_1 in translations.keys():
        subtranslations = translations[key_1]
        if not isinstance(subtranslations, dict):
            raise ValueError(f"{key_1} should map to nested translations")
        return key_2 in subtranslations.keys()
    return False


def show_msg_found_keys_not_defined(keys_to_paths: KeysToPathsMapping, key: TranslationKey, yml_path: YmlPath, src_path: Path) -> None:
    key_source_paths = keys_to_paths[key]
    key_sources = [path.relative_to(src_path) for path in key_source_paths]
    if len(key_sources) == 1:
        print(f"  Key '{key}', from '{key_sources[0]}', not found in file '{yml_path.name}'.")
    else:
        print(f"  Key '{key}', not found in file '{yml_path.name}'.")
        print("It appears in the next src files:")
        for path in key_sources:
            print(f"\t{path}")


def debug_found_keys(patterns_to_keys_to_paths: PatternsToKeysToPaths, base_dir: Path) -> None:
    paths_to_keys: dict[SrcPath, list[str]] = defaultdict(list)
    for _pattern, keys_to_paths in patterns_to_keys_to_paths.items():
        for key, paths in keys_to_paths.items():
            for path in paths:
                paths_to_keys[path].append(key)
    for path, keys in paths_to_keys.items():
        relative_path = path.relative_to(base_dir)
        print(f"Discovered in {relative_path}:")
        for key in keys:
            print(f"\t{key}")


def get_paths_to_parsed_yml(base_dir: Path) -> dict[YmlPath, Parsed]:
    """Builds a dict mapping translations files paths to parsed content"""
    yml_paths = cast(list[YmlPath], get_paths_with_extension(base_dir, ".yml"))
    files_to_contents = get_path_to_contents(yml_paths)
    paths_to_parsed = {}
    for path, content in files_to_contents.items():
        try:
            parsed = yaml.load(content, yaml.Loader)
        except yaml.parser.ParserError as err:
            info_line = "line " + str(err.problem_mark.line + 1) if err.problem_mark else "unknown"
            info = f"Error while parsing {path}, {info_line}"
            raise RuntimeError(info) from err
        assert isinstance(parsed, dict)
        paths_to_parsed[path] = parsed
    return paths_to_parsed




if __name__ == '__main__':
    main()
