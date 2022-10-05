import re
from typing import Literal, Optional, Protocol, TypedDict

from .simple_pattern import simple_pattern
from ..types import KeysToPaths, PathsToLines

PatternName = str


class ExtractorProtocol(Protocol):
    def execute(self, pattern: re.Pattern[str], paths_to_lines: PathsToLines) -> KeysToPaths:
        ...


RegexPattern = TypedDict('RegexPattern', {'name': PatternName, 'regex': re.Pattern[str], 'extractor': Optional[ExtractorProtocol]})



top_level_regex_patterns: list[RegexPattern] = [
    dict(name="simple_pattern", regex=simple_pattern, extractor=None),
]

__all__ = ['inside_menu_option_pattern']
