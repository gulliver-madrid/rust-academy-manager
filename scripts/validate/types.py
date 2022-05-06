import re

from dataclasses import dataclass, field
import itertools
from typing import ClassVar
from pathlib import Path


Paths = list[Path]
KeysToPaths = dict[str, Paths]
RegexPattern = re.Pattern[str]
PathsToLines = dict[Path, list[str]]


@dataclass(frozen=True)
class UsedKeyPattern:
    """Wrapper useful for debugging"""
    next_id: ClassVar[int] = 0
    regex: RegexPattern = field(repr=False)
    id: int = field(default_factory=itertools.count().__next__)


PatternsToKeysToPaths = dict[UsedKeyPattern, KeysToPaths]
