import re

# Patron para detectar cadenas en las MenuOptions

optional_spaces = r"\s*"
menu_option_variant = r"MenuOption::[A-Z]\w*"
optional_part_of_key_pattern = r"(?:[.]\w+)"
key_pattern = fr"[\w]+{optional_part_of_key_pattern}?"

text_pattern = fr'\({optional_spaces}{menu_option_variant},{optional_spaces}"({key_pattern})"[,)]'
# example: (MenuOption::Teachers, "menu_options.teacher")
menu_option_pattern: re.Pattern[str] = re.compile(text_pattern, re.ASCII)


def test_match_menu_option_pattern_1() -> None:
    assert menu_option_pattern.search('(MenuOption::ShowList, "subjects_menu_options.show_list"),')


def test_match_menu_option_pattern_2() -> None:
    assert menu_option_pattern.search('''(
        MenuOption::AssignTeacher,
        "subjects_menu_options.assign_teacher",
    ),''')


def test_no_match_menu_option_pattern() -> None:
    assert not menu_option_pattern.search('("subjects_menu_options.assign_teacher"')
