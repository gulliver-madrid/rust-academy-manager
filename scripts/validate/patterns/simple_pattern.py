import re
from typing import Final


def _create_simple_pattern() -> re.Pattern[str]:
    NO_CLOSE_PAREN_CHAR = r'[^)]'
    return re.compile(fr"""
        [^a-zA-Z]       # Some non alphabetic character (prevent format! false positives)
        t!\("           # t!("
        ([^"]+)         # the translation key as a captured group
        "               # "
        (?:,
            {NO_CLOSE_PAREN_CHAR}+
        )?               # optional non-capturing group with a comma and several characters more, excepting close parens.
                \)              # )
    """, re.VERBOSE)


simple_pattern: Final = _create_simple_pattern()


def test_match_simple_pattern_1() -> None:
    KEY = "una_key"
    m = simple_pattern.search(f'let s = t!("{KEY}")')
    assert m
    assert len(m.groups()) == 1
    assert m.groups()[0] == KEY


def test_match_simple_pattern_2() -> None:
    KEY = "una_key_con_arg_${name}_"
    m = simple_pattern.search(f'let s = t!("{KEY}", name=name)')
    assert m
    assert len(m.groups()) == 1
    assert m.groups()[0] == KEY
