from dataclasses import dataclass
from collections import defaultdict
from typing import Iterator
from pathlib import Path


from .helpers import get_path_to_contents, get_paths_with_extension
from .types import KeysToPaths, PathsToLines, PatternsToKeysToPaths, RegexPattern, UsedKeyPattern



RUST_EXTENSION = ".rs"
MULTI_LINE_PATTERNS_MAX_SIZE = 4


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
