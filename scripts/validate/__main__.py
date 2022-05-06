# Checks all keys used in the program have their translation in locale files

from collections import defaultdict
from dataclasses import dataclass
from typing import Any, Final, Iterable, Mapping, NewType, TypeVar, Union, cast
import yaml

from pathlib import Path
import sys

from .helpers import get_path_to_contents, get_paths_with_extension
from .key_finder import KeyFinder
from .types import KeysToPaths, KeysToPathsMapping, PatternsToKeysToPaths, SrcPath, SrcPaths, Pattern, YmlPath, YmlPaths
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
    keys_to_paths = key_finder.get_used_keys(patterns)
    if len(keys_to_paths) == 0:
        print("No keys found in src directory")
        sys.exit()

    paths_to_parsed = get_paths_to_parsed_yml(locale_dir)

    defined_keys = get_defined_keys(paths_to_parsed)
    all_keys_found = True
    for key, paths in keys_to_paths.items():
        if key not in defined_keys.all_keys:
            print(f"{key} is not defined, but used in {paths}")
        else:
            key_missing_langs: list[str] = []
            for lang in defined_keys.by_lang.keys():
                if key not in defined_keys.by_lang[lang]:
                    key_missing_langs.append(lang)
                    print(f"{key} is not defined, but used in {paths}")
                    all_keys_found = False
            if key_missing_langs:
                show_msg_found_keys_not_defined(key, paths, key_missing_langs, src_base_path)

    keys_in_source = set(keys_to_paths.keys())
    if all_keys_found:
        print("All keys were found")
    all_defined_keys = frozenset(defined_keys.all_keys)

    if missing_in_source_code := all_defined_keys.difference(keys_in_source):
        print("\nSome keys defined in translation files were not used in the program:\n  " + "\n  ".join(missing_in_source_code))
        if DEBUG:
            input("Press ENTER to debug found keys")
            debug_found_keys(keys_to_paths, src_base_path)



@dataclass(frozen=True)
class DefinedKeys:
    by_lang: dict[str, set[TranslationKey]]
    all_keys: set[TranslationKey]

def get_defined_keys(paths_to_parsed: dict[YmlPath, Parsed]) -> DefinedKeys:
    """Return a set with all the defined keys in locale files"""
    by_lang = {}
    all_keys = set()
    for _yml_path, parsed in paths_to_parsed.items():
        for lang, translations in parsed.items():
            lang_keys = set()
            for k, v in translations.items():
                if isinstance(v, str):
                    lang_keys.add(k)
                else:
                    assert isinstance(v, dict)
                    key_1 = k
                    for key_2, translation in v.items():
                        assert isinstance(translation, str), translation
                        double_key = ".".join([key_1, key_2])
                        lang_keys.add(double_key)
            by_lang[lang] = lang_keys
            all_keys.update(lang_keys)
    return DefinedKeys(by_lang, all_keys)



def show_msg_found_keys_not_defined(key: TranslationKey, paths: SrcPaths, langs: list[str], src_path: Path) -> None:
    key_sources = [path.relative_to(src_path) for path in paths]
    langs_str = "language " + langs[0] if len(langs) == 1 else "languages " + ",".join(langs)
    if len(key_sources) == 1:
        print(f"""  Key '{key}', from '{key_sources[0]}', not found for '{langs_str}'.""")
    else:
        print(f"  Key '{key}', not found for '{langs_str}'.")
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
