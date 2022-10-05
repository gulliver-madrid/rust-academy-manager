from ..types import PatternName
from .simple_pattern import simple_pattern
from .menu_option_pattern import macro_menu_option_pattern

regex_patterns = [(PatternName("simple_pattern"), simple_pattern), (PatternName("macro_menu_option_pattern"), macro_menu_option_pattern)]
