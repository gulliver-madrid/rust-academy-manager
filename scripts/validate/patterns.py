import re
from .menu_option_pattern import menu_option_pattern

simple_pattern = re.compile(r"""
            [^a-zA-Z]       # Some non alphabetic character (prevent format! false positives)
            t!\("           # t!("
            ([^"]+)         # the translation key as a captured group
            "\)             # ")
""", re.VERBOSE)

regex_patterns = [simple_pattern, menu_option_pattern]
