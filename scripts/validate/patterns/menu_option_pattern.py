import re
from typing import Final

# Patron para detectar cadenas en las MenuOptions

optional_spaces = r"\s*"
menu_option_variant = r"MenuOption::[A-Z]\w*"
optional_part_of_key_pattern = r"(?:[.]\w+)"
key_pattern = fr"[\w]+{optional_part_of_key_pattern}?"



text_pattern = r"""create_menu_options!\((.+)\);"""



macro_menu_option_pattern: Final[re.Pattern[str]] = re.compile(text_pattern, re.ASCII | re.DOTALL)


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



# def test_match_menu_option_pattern_2() -> None:
#     m = menu_option_pattern.search('''(
#         MenuOption::AssignTeacher,
#         "subjects_menu_options.assign_teacher",
#     ),''')

#     assert m and m.groups()[0] == 'subjects_menu_options.assign_teacher'


# def test_no_match_menu_option_pattern() -> None:
#     assert not menu_option_pattern.search('("subjects_menu_options.assign_teacher"')
