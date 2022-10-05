from dataclasses import dataclass
from collections import defaultdict
import itertools
from typing import Iterable, Iterator, cast
from pathlib import Path

from .patterns import inside_menu_option_pattern
from .helpers import FileReader
from .types import KeysToPaths, MatchedStringsToPaths, Paths, PathsToLines, RegexPattern, SrcPaths, Pattern


RUST_EXTENSION = ".rs"
MULTI_LINE_PATTERNS_MAX_SIZE = 8


@dataclass(frozen=True)
class KeyFinder:
    base_dir: Path

    def get_used_keys(self, patterns: list[Pattern]) -> KeysToPaths:
        by_pattern = [self._get_used_keys_by_pattern(pattern.regex) for pattern in patterns]
        return _chain_values_by_key(by_pattern)

    def _get_used_keys_by_pattern(self, pattern: RegexPattern) -> KeysToPaths:
        rust_paths = FileReader.get_paths_with_extension(self.base_dir, RUST_EXTENSION)
        src_paths = _narrow_to_src_paths(rust_paths)
        files_to_contents = FileReader.get_path_to_contents(src_paths)
        paths_to_lines: PathsToLines = {
            path: content.split("\n")
            for path, content in files_to_contents.items()
        }
        return self._extract_used_keys(pattern, paths_to_lines)


    def _extract_used_keys_macro_menu_option(self, pattern: RegexPattern, paths_to_lines: PathsToLines) -> KeysToPaths:
        assert pattern['name'] == "macro_menu_option_pattern", pattern
        # Same that general one but searching the content of create_menu_options! macro
        # And after that, search the translation keys inside these contents
        contents: MatchedStringsToPaths = _extract_regex_matches(pattern, paths_to_lines)

        keys_used = KeysToPaths(defaultdict(list))
        # Note: different contents could have an identical key
        for content, new_src_paths in contents.items():
            matches = inside_menu_option_pattern.findall(content)
            for key in matches:
                src_paths = keys_used[key]
                src_paths.extend([path for path in new_src_paths if path not in src_paths])
                assert keys_used[key] == src_paths, f'These two lists should be still the same object'

        return keys_used

    def _extract_used_keys(self, pattern: RegexPattern, paths_to_lines: PathsToLines) -> KeysToPaths:
        """Extract the used keys that match with the given pattern"""
        keys_used: KeysToPaths
        if pattern['name'] == 'simple_pattern':
            keys_used = KeysToPaths(_extract_regex_matches(pattern, paths_to_lines))
            return keys_used
        else:
            assert pattern['name'] == "macro_menu_option_pattern", pattern
            keys_used = self._extract_used_keys_macro_menu_option(pattern, paths_to_lines)
            return keys_used


def _extract_regex_matches(pattern: RegexPattern, paths_to_lines: PathsToLines) -> MatchedStringsToPaths:
    # Using blocks of n lines to detect multiline patterns
    # Every string should be detected just once (in order to prevent detecting multiple times a key in the same block)
    all_matches: MatchedStringsToPaths = defaultdict(list)
    n = MULTI_LINE_PATTERNS_MAX_SIZE
    for path, lines in paths_to_lines.items():
        found_matches: set[str] = set()
        for block in _get_blocks(lines, n):
            # There may be multiple keys in the same block (or line)
            matches = pattern['regex'].findall(block)
            # validate there is only one group by pattern
            assert all(isinstance(m, str) for m in matches)
            found_matches.update(matches)
        for key in found_matches:
            all_matches[key].append(path)
    return all_matches


def _get_blocks(lines: list[str], block_size: int) -> Iterator[str]:
    """
    For number_of_lines=10 and block_size=3 it would be range(8),
    so the last block would be lines[7:10]
    """
    number_of_lines = len(lines)
    for i in range(number_of_lines + 1 - block_size):
        block_lines = lines[i: i + block_size]
        block = "\n".join(block_lines)
        yield block


def _chain_values_by_key(keys_to_paths: Iterable[KeysToPaths]) -> KeysToPaths:
    keys = {k for d in keys_to_paths for k in d.keys()}
    return KeysToPaths({k: _group_by_key(keys_to_paths, k) for k in keys})


def _group_by_key(dicts: Iterable[KeysToPaths], key: str) -> SrcPaths:
    return list(itertools.chain.from_iterable(d[key] for d in dicts))


def _narrow_to_src_paths(paths: Paths) -> SrcPaths:
    return cast(SrcPaths, (paths))
