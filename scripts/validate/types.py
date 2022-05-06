import re

from dataclasses import dataclass, field
import itertools
from typing import ClassVar, Mapping, NewType
from pathlib import Path

YmlPath = NewType('YmlPath', Path)
SrcPath = NewType('SrcPath', Path)

Paths = list[Path]
SrcPaths = list[SrcPath]
KeysToPaths = dict[str, SrcPaths]
KeysToPathsMapping = Mapping[str, SrcPaths]
RegexPattern = re.Pattern[str]
PathsToLines = dict[SrcPath, list[str]]


@dataclass(frozen=True)
class Pattern:
    """Wrapper useful for debugging"""
    next_id: ClassVar[int] = 0
    regex: RegexPattern = field(repr=False)
    id: int = field(default_factory=itertools.count().__next__)


PatternsToKeysToPaths = dict[Pattern, KeysToPaths]
