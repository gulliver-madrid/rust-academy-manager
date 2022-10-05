
import re
from typing import Final


def _build_menu_option_pattern() -> re.Pattern[str]:
    """
    Example:
        match: (MenuOption::Teachers, "menu_options.teacher")
        group: "menu_options.teacher"
    """
    no_quot = fr'[^"]'  # no double quotation marks
    text_pattern = fr'''
    \({no_quot}+"({no_quot}+)"{no_quot}*\)
    '''
    return re.compile(text_pattern, re.ASCII | re.DOTALL | re.VERBOSE)


inside_menu_option_pattern: Final = _build_menu_option_pattern()


def test_match_menu_option_inside_pattern() -> None:
    source = """
    (ShowList, "teachers_menu_options.show_list"),
    (AddTeacher, "teachers_menu_options.add_teacher"),
    (RemoveTeacher, "teachers_menu_options.remove_teacher"),
    (GoBack, "teachers_menu_options.go_back")
"""
    matches = inside_menu_option_pattern.findall(source)
    assert len(matches) == 4
    assert matches == [
        "teachers_menu_options.show_list",
        "teachers_menu_options.add_teacher",
        "teachers_menu_options.remove_teacher",
        "teachers_menu_options.go_back"
    ]
