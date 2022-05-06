# Checks all keys used in the program have their translation in locale files

from collections import defaultdict
from dataclasses import dataclass, field
import itertools
from typing import ClassVar, Final, Iterator, NewType, Sequence, TypeVar, Union, cast
import yaml

from pathlib import Path
import re
import sys

from .patterns import menu_option_pattern

DEBUG: Final = False

PathsToLines = dict[Path, list[str]]
Paths = list[Path]
KeysToPaths = dict[str, Paths]
RegexPattern = re.Pattern[str]


YmlPath = NewType('YmlPath', Path)
LangKey = str  # ex. 'en'
TranslationKey = str
TranslationValue = Union[str, dict[str, str]]  # because there are nested keys
Parsed = dict[LangKey, dict[str, TranslationValue]]

RUST_EXTENSION = ".rs"
MULTI_LINE_PATTERNS_MAX_SIZE = 4

simple_pattern = re.compile(r"""
            [^a-zA-Z]       # Some non alphabetic character (prevent format! false positives)
            t!\("           # t!("
            ([^"]+)         # the translation key as a captured group
            "\)             # ")
    """, re.VERBOSE)


@dataclass(frozen=True)
class UsedKeyPattern:
    """Wrapper useful for debugging"""
    next_id: ClassVar[int] = 0
    regex: RegexPattern = field(repr=False)
    id: int = field(default_factory=itertools.count().__next__)


PatternsToKeysToPaths = dict[UsedKeyPattern, KeysToPaths]


def main() -> None:
    args = sys.argv[1:]
    if len(args) < 2:
        print("Should be called with 2 arguments: src folder and locale folder")
        sys.exit()
    patterns = [UsedKeyPattern(regex) for regex in (simple_pattern, menu_option_pattern)]
    src_path_str, locale_path_str = sys.argv[1:]
    src_path = Path.cwd() / src_path_str
    locale_dir = Path.cwd() / locale_path_str
    print(f"src dir: {src_path}")
    print(f"locale dir: {locale_dir}\n")
    if not locale_dir.exists():
        print("Locale directory introduced does not exist")
        sys.exit()
    key_finder = KeyFinder(src_path)
    used_keys_by_pattern = key_finder.get_used_keys(patterns)
    if all(len(used_keys) == 0 for used_keys in used_keys_by_pattern.values()):
        print("No keys found in src directory")
        sys.exit()

    paths_to_parsed = get_paths_to_parsed_yml(locale_dir)

    keys_in_parsed = set()
    for _lang_path, parsed in paths_to_parsed.items():
        for _lang, translations in parsed.items():
            for k, v in translations.items():
                if isinstance(v, str):
                    keys_in_parsed.add(k)
                else:
                    assert isinstance(v, dict)
                    for k2, v2 in v.items():
                        assert isinstance(v2, str), v2
                        keys_in_parsed.add(".".join([k, k2]))

    all_keys_found = True
    for pattern in patterns:
        used_keys = used_keys_by_pattern[pattern]
        for key in used_keys.keys():
            if key in keys_in_parsed:
                # This key appears in at least one translation file TODO: REVIEW
                keys_in_parsed.remove(key)
            for yml_path, parsed in paths_to_parsed.items():
                # print(f"Checking {path}")
                found = False
                for _, translations in parsed.items():
                    if "." in key:
                        key_1, key_2 = key.split(".")
                        if key_1 in translations.keys():
                            subtranslations = translations[key_1]
                            assert isinstance(subtranslations, dict)
                            if key_2 in subtranslations.keys():
                                found = True
                                break
                    elif key in translations.keys():
                        found = True
                        break
                if not found:
                    key_source_paths = used_keys[key]
                    key_sources = [path.relative_to(src_path) for path in key_source_paths]
                    if len(key_source_paths) == 1:
                        print(f"  Key '{key}', from '{key_sources[0]}', not found in file '{yml_path.name}'.")
                    else:
                        print(f"  Key '{key}', not found in file '{yml_path.name}'.")
                        print("It appears in the next src files:")
                        for path in key_source_paths:
                            print(f"\t{path}")
                    all_keys_found = False
    if all_keys_found:
        print("All keys were found")
    if keys_in_parsed:
        print("\nSome keys defined in translation files were not used in the program:\n  " + "\n  ".join(keys_in_parsed))
        if DEBUG:
            input("Press ENTER to debug found keys")
            debug_found_keys(used_keys_by_pattern, src_path)


def debug_found_keys(patterns_to_keys_to_paths: PatternsToKeysToPaths, base_dir: Path):
    paths_to_keys: dict[Path, list[str]] = defaultdict(list)
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


GenericPath = TypeVar('GenericPath', bound=Path)


def get_path_to_contents(paths: Sequence[GenericPath]) -> dict[GenericPath, str]:
    files_to_contents = {}
    for path in paths:
        with open(path, "r") as file:
            files_to_contents[path] = file.read()
    return files_to_contents

@dataclass(frozen=True)
class KeyFinder:
    base_dir: Path

    def get_used_keys(self, patterns: list[UsedKeyPattern]) -> PatternsToKeysToPaths:
        return {pattern: self.get_used_keys_by_pattern(pattern.regex) for pattern in patterns}

    def get_used_keys_by_pattern(self, pattern: RegexPattern) -> KeysToPaths:
        paths = get_paths_with_extension(self.base_dir, RUST_EXTENSION)
        files_to_contents = get_path_to_contents(paths)
        paths_to_lines: PathsToLines = {
            path: content.split("\n")
            for path, content in files_to_contents.items()
        }
        return self.extract_used_keys(pattern, paths_to_lines)


    def extract_used_keys(self, pattern: RegexPattern, paths_to_lines: PathsToLines) -> KeysToPaths:
        keys_used = defaultdict(list)
        # Using blocks of n lines to detect multiline patterns
        # Every key should be detected just once (in order to prevent detecting multiple times a key in the same block)
        n = MULTI_LINE_PATTERNS_MAX_SIZE
        for path, lines in paths_to_lines.items():
            keys_found: set[str] = set()
            for block in get_blocks(lines, n):
                # There may be multiple keys in the same block (or line)
                matches = pattern.findall(block)
                # validate there is only one group by pattern
                assert all(isinstance(m, str) for m in matches)
                keys_found.update(matches)
            for key in keys_found:
                keys_used[key].append(path)
        return keys_used


def get_blocks(lines: list[str], block_size: int) -> Iterator[str]:
    """
    For number_of_lines=10 and block_size=3 it would be range(8),
    so the last block would be lines[7:10]
    """
    number_of_lines = len(lines)
    for i in range(number_of_lines + 1 - block_size):
        block_lines = lines[i: i + block_size]
        block = "\n".join(block_lines)
        yield block


def get_paths_with_extension(base_dir: Path, ext: str) -> list[Path]:
    assert ext.startswith("."), ext
    all_paths = base_dir.glob('**/*' + ext)
    return [p for p in all_paths if p.is_file()]


if __name__ == '__main__':
    main()
