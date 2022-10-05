import re
from typing import Final

# Patron para detectar el contenido de las llamadas a la macro create_menu_options!


def _create_macro_menu_option_pattern() -> re.Pattern[str]:
    text_pattern = r"""create_menu_options!\((.+)\);"""
    return re.compile(text_pattern, re.ASCII | re.DOTALL)


macro_menu_option_pattern: Final = _create_macro_menu_option_pattern()


def test_match_menu_option_pattern_1() -> None:
    inner = """
    (ShowList, "teachers_menu_options.show_list"),
    (AddTeacher, "teachers_menu_options.add_teacher"),
    (RemoveTeacher, "teachers_menu_options.remove_teacher"),
    (GoBack, "teachers_menu_options.go_back")
"""
    m = macro_menu_option_pattern.search(f'''create_menu_options!({inner});''')
    print(f"{m=}")
    if m:
        print(f"{m.groups()=}")
    assert m and m.groups()[0].strip() == inner.strip()
