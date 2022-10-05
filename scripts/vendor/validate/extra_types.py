
from typing import ClassVar
from dataclasses import dataclass, field
import itertools

from .types import KeysToPaths
from .patterns import RegexPattern


@dataclass(frozen=True)
class PatternWithDebug:
    """Wrapper useful for debugging"""
    next_id: ClassVar[int] = 0
    regex: RegexPattern = field(repr=False)
    id: int = field(default_factory=itertools.count().__next__)


PatternsToKeysToPaths = dict[PatternWithDebug, KeysToPaths]
