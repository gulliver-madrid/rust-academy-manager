from typing import TypedDict

from scripts.validate.types import RegexPattern
from .simple_pattern import simple_pattern
from .macro_menu_option_pattern import macro_menu_option_pattern
from .inside_menu_option_pattern import inside_menu_option_pattern

top_level_regex_patterns: list[RegexPattern] = [
    dict(name="simple_pattern", regex=simple_pattern),
    dict(name="macro_menu_option_pattern", regex=macro_menu_option_pattern)
]

__all__ = ['inside_menu_option_pattern']
